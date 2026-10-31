# 1030 review: infra (reviewinfra)

Read-only. Scope: staged diff vs HEAD of `🦑️repo/**`, `🏃️process/**`, `📁️filesystem/**`. Nothing built or run. Paths abbreviated; G = `🏃️process/🏗️build/🌡️host-capacity`, GROW = `🦑️repo/.../📦️budget/📏️grow`, DEP = `🦑️repo/.../🚀️bootstrap/📦️dependencies/💾️storage`.

## Findings (most severe first)

### 1. Wait-marker edge forks `ps` synchronously on every poll (owner: actinfra) - confidence high
- `G/🎫️gate/🟦️.ts:93-96` (`processStartV1`, `spawnSync`, 5 s timeout) is called from the edge listener at `:105` on EVERY "waiting" edge. `waitingDelayV1` (`🐕️watchdog/🟦️.ts:45`) enters/leaves a wait around every single delay, so each lease-queue poll (20-250 ms, `🧭️order/🔣️policy.json` gate) and each headroom poll (<= 2 s) is an edge: one `ps` fork + one `writeFileSync` per poll per waiting cargo launcher.
- A process's own start time is constant; it is recomputed anyway. With dozens of waiters in a fleet this is hundreds of forks/s and blocks the event loop of every waiter (heartbeats, cancellation).
- Holder side `:112-119,:121` (`readHostCapacityWaitMarkersV1` forks `ps` per marker, `processChildrenTableV1` forks `ps -axo` with 10 s timeout) runs every `pollMilliseconds` (1000) inside the lender interval, blocking the holder's heartbeat loop for as long as `ps` takes under load.
- Fix: compute own start once (memo), write the marker only when absent or `since` changed, read child table at most once per heartbeat, make both async.

### 2. Marker write is not atomic and one bad file aborts the whole read (owner: actinfra) - confidence medium
- `:105` `writeFileSync(path,...)` truncates then writes (and, per finding 1, runs every poll). `:113-117` parses inside one `try` around the loop: a reader that hits an empty/torn file throws, the `catch{}` ends the loop, and every marker after it is dropped for that poll. A torn marker of a SIGKILLed writer (never rewritten) hides all later markers permanently.
- Effect: lending flaps or never starts (under-lend, safe but defeats the feature). Fix: write via temp+rename, per-file try/catch.

### 3. Wait markers are never swept (owner: actinfra) - confidence medium
- `:281` `cleanup()` removes grant, attach, phases and upgrade files but not `<id>.wait.<pid>`. A child killed without running its `exit` handler (SIGKILL, OOM, the fleet's pkill habit) leaves the file forever; also `:105` only checks that `grants/` exists, not the grant, so an orphan that keeps waiting after the grant ended recreates the marker. `grants/` grows unbounded; pid+start identity prevents wrong lending but not the leak.
- Fix: unlink `<id>.wait.*` in `cleanup()`, and require the grant file in the writer.

### 4. Forced-unmount audit trail is lost and journal type drifts from schema (owner: actinfra) - confidence high
- `GROW/🟦️.ts:111` `GrowJournalV1` has no `forcedUnmount`, while the schema `Journal` (`🧬️schema/🔣️.json:57`) and the code at `:171` (`journal.forcedUnmount`) use it (a TS error, hidden because Bun does not type-check). `GrowOutcomeV1.forcedUnmount` (`:108`) is declared but the returned object at `:209` never sets it, and the journal is deleted at `:207`, so after success nothing records that a forced unmount happened. The schema caps the array at 16 but the code appends without bound across resumes.
- Same drift class: schema `collector.stopGraceMilliseconds` (`🧬️schema/🔣️.json:21`) is read by nothing; `GrowRequestV1.stopCollectors` and the CLI `stopCollectors=true` constant are not consulted (`:158` only tests that the tool exists).

### 5. Grow tools run synchronously without timeout while holding the exclusive image lease (owner: actinfra) - confidence medium
- `GROW/🟦️.ts:214` `run()` uses `spawnSync` with no `timeout` for `lsof`, `hdiutil`, `diskutil`, `plutil`; `signal.throwIfAborted()` (`at()`) is only reached between calls. A hung `hdiutil detach` or `lsof` on a wedged volume blocks the event loop (cancel cannot be processed) while the exclusive `native-fixed-image:` lease stalls every command in the fleet. Use async spawn with timeout and abort.
- `:229` `holders` ignores the exit status: an `lsof` failure (killed, permission) reads as "no holders" and then enables the forced-unmount path at `:171` (which requires `open.length===0`). Treat non-0/1 status as unknown.

### 6. Forced unmount allowlist trusts the process name only (owner: actinfra) - confidence low-medium (security)
- `:171,:239` allow `diskutil unmount force` when every dissenter basename is in `forcedUnmountDissenters` (`syspolicyd, mds, mds_stores, fseventsd`), parsed from `dissented by PID n (name)`. The name is `comm`, which any process can choose (copy a binary to `mds`). Mitigated by the preceding `lsof` check (same-user holders show up) but lsof cannot see other users' processes and finding 5 shows it can fail open. Fix: also verify the PID's executable path (`ps -o comm=` full path under `/System/` or `/usr/libexec/`, `/usr/sbin/`) and uid 0 before forcing; the non-forced probe (`unmountProbe`) is a real unmount attempt, which is fine, but its success path then reports "detach failed" (`:171` pushes a failure) although the volume is already unmounted; harmless because the next resume detaches.

### 7. 6 h holder-pass ceiling converts starvation into a real deadlock (owner: actinfra) - confidence medium
- `🔒️leases/⚖️fairness/🟦️.ts:20-25`: after `reentrantPassCeilingMilliseconds` (21 600 000, `🧭️order/🔣️policy.json`) the holder tree that already holds the image shared queues behind the waiting writer. The writer cannot get the lease while that same tree holds it, so both wait for ever; only the wait-for graph reports it. A legitimate long build (> 6 h incl. the writer's wait) that opens a nested shared request after the ceiling deadlocks with the grow/reset writer. Consider failing the nested request fast with a diagnostic instead of queueing, or aging relative to the holder's own start.
- `writerPreferenceByPrefix` `native-fixed-image:` = 1 h (`:policy.json`): after an hour the waiting grow/reset stops deferring new shared requests (`queueTicketDefersV1` only defers while `now-arrived < preference`), so under a steady fleet the writer is starved again, exactly the case the prefix rule was added for. The preference needs to be unbounded (or the writer must latch a "draining" state) rather than a longer timeout.

### 8. Headroom learns the whole image's free-space fall (owner: actinfra) - confidence medium-low
- `🚪️launcher/🟦️.ts:98` `commandWriteBytesV1 = max(lane growth, free before - free after)`. Concurrent builds, GC and unrelated writers all land in one command's sample, so reservations inflate with concurrency (self-limiting, but it serializes builds and the 3-sample quantile then sticks the inflated value). Failed commands (including ENOSPC kills, the case that most needs a bigger reservation) are never learned (`completed` gate). Use a lane-only sample plus a separate shared-cache sample, or subtract concurrent running rows.
- `decideHeadroomV1` stranded grace (`🏦️headroom/🟦️.ts`): `active` filters rows with `now-admitted > ramp+120000`, but a legitimately long compile whose ramp (10-15 min) ended keeps running; a second command is then admitted as `alone` while the first still writes, relying only on the post-hoc out-of-space retry. Bound by `reclaimed`, so low severity.

### 9. Ramp-idle exemption counts a grant that has not heartbeated yet as idle (owner: actinfra) - confidence low
- `G/📏️decision/🟦️.ts:131,254` `idleAtGate`: `(observed??0)*10000 < reserved*2500` is true for a grant with no observation yet, and for an analysis/metadata phase that is quiet for 30 s (`rampIdleAfterMilliseconds`, `🏗️build/🔣️policy.json:205`). Such a grant stops occupying one of `maximumRamping` (2) slots and a third and fourth run ramp concurrently. Memory is still reserved (committed), so this is slot oversubscription, not memory oversubscription. Require `observedMegabytes!==undefined`.

### 10. Cross-platform gaps of the new tree-wait lending (owner: actinfra) - confidence high, low impact
- `processChildrenTableV1` returns `undefined` on win32 (`:121`), so the feature is silently off on Windows; busybox `ps` (Alpine devcontainers) has neither `-o lstart` nor `-axo`, so markers are never written. Fails safe (no lending) but not disclosed by the doctor. `lstart` compares formatted local-time strings: a reader and writer with different `TZ` env see different strings, so valid markers are rejected.

### 11. Cargo.lock missing-path marker is POSIX-English only (owner: actinfra) - confidence medium
- `🦀️cargo-workspace/🔒️lock/🧬️schema/🔣️.json:78`: the regex needs `No such file or directory`. On Windows cargo prints `The system cannot find the file/path specified. (os error 2/3)` (localized by the OS locale), so a missing path dependency still goes online and fails minutes later, the exact failure the change removes. Match `os error (2|3)` instead.

### 12. Quiet-wait uses wall clock vs mtime and blocks synchronously (owner: actinfra) - confidence low
- `📁️filesystem/🧾️observation/🔁️churn/🟦️.ts:99` `Date.now()-mtimeMs`: a file with a future mtime (clock skew on network/bind mounts, container clock) reads age 0 and waits the full 30 s ceiling per retry call; `defaultSleepSync` blocks the thread for the whole wait in the sync variant. Bounded (30 s), no loop risk (`attempt--` only while `quietSpent` < ceiling).

### 13. Singleton publication: remove-then-link is not crash-atomic and nested copies are untouched (owner: scopereact) - confidence low-medium
- `DEP/🟦️.ts:485` removes the physical `node_modules/<pkg>` (`removeJavascriptDependencyTreeV1`/`unlinkSync`) and only then publishes the link. A crash between the two leaves a generation without the package; a re-run observes `missing` (`:476` `observe`), plans no alias for it (packages() skips missing) and nothing asserts presence afterwards, while the ready `singletons` rows would claim it bound. Safe only if the produced directory is always fresh per attempt; the adopted-donor path (the code comment mentions it) reuses trees. Add a post-condition check that every declared singleton name resolves to the owner copy for every scope that had one, and publish via link-to-temp + rename.
- Only top-level `node_modules/<name>` per scope root and `w/<slot>` is replaced; nested copies (`node_modules/x/node_modules/react` from a peer-range conflict) still give two Reacts. The real-tree oracle confirmed one copy today; add the same oracle as a ready-time guard.
- `readProducerBindingLinkV1(path)===location(ownerStage,name)` compares raw readlink text; on Windows junction readlink adds `\\?\`/trailing `\`, so every run classes links as `stale` and relinks (idempotent, but `version:"stale"` then fails `One singleton version required` when the owner has no copy). Links are absolute; moving the checkout or restoring a cache on another path breaks them (same as borrowed providers).
- The planner turns any scope with a different react/react-dom version into a hard `sync` failure for ALL scopes (`planJavascriptDependencySingletonAliasesV1` "One singleton version required"). Intended, but it should name the scope in the error (currently it does not).

### 14. Artifact scratch lease per invocation: lease files never reused (owner: actinfra) - confidence low (unverified)
- `🦑️repo/.../📦️artifacts/💾️storage/🟦️.ts:40` keys the lease by sha256 of a random directory, so every invocation creates a new `.queue` directory and lease file under `.cache/b/l/m/data`. Unless the lease cleanup (`🧹️cleanup`) sweeps `artifact-scratch-private:*`, they accumulate. Also no stale-scratch-directory sweep exists for crashed invocations (`tmp/<rand>`); the previous shared lease did not provide one either. I did not find the sweep; please confirm.

### 15. wasm-build ordering (owner: actinfra) - confidence low
- `🕸️wasm-build/🟦️.ts:110-111,156-157`: `acquireCargoBuildLeaseV1` is now taken INSIDE `withCargoCompilerInputsV1` (inputs/registry reader first, cargo-build lease second; previously the reverse), and `consumeProducedWasmPackageV1` runs after `withWriter` ended but before the cargo-build lease is released. The order change must match the lease-order policy (not verified; lock-order enforcement would report it at runtime). The consume step now reads `files` after the writer lease is gone; it is still under the cargo-build lease on the same target dir, so no TOCTOU with cargo, but another `withWriter` user of `pkgDir` is not excluded. The progress-renewed deadline wiring (`renewableCompilerAdmissionControlV1`) was not traced to a producer of progress (wasm-pack output).

### 16. Receipt-driven capacity can break budget resolution (owner: actinfra) - confidence low
- `🦑️repo/.../📦️budget/🟦️.ts` `repositoryCacheBudgetV1` subtracts `declared.capacityBytes-capacity` from `maximumLogicalBytes`; a valid but tiny receipt (>= 32 MiB, multiple of MiB, e.g. a corrupted or foreign `image.json`) drives `maximumLogicalBytes` negative and the schema read throws for every command, instead of refusing that one partition.

## Checked, no defect found
- Elastic priority ranking (`📏️decision` `rank`): interactive-grow/elastic = 2, batch = 1, batch-grow = 3; aged batch is promoted before ordering; no inversion found.
- Cache grow journal: every phase re-checks real state (file size, ino, roster, device), `writeAtomic` fsyncs file, resumes from every `at()` checkpoint were traced; the restamp is idempotent. `growReceiptTextV1` accepts both pre- and post-grow receipts.
- Image receipt parsing is now one function (`readImageReceiptBytesV1`); `maximumPhysicalBytes = filesystemBytes + 1 MiB` matches policy capacity `103080263680` (rule maximum), so elastic clamping does not shrink a grown image.
- Funding replacement model (`💰️funding/🔁️replacement`): reserve/settle/abort preserve predecessor + delta accounting; a `recover` path exists for a dead owner.
- Registry/provider memo keys (`physicalMemo` by resolved path, `candidateMemo` by dir+name+range, manifest by path+stat stamp) are complete; errors are not cached.
- Lock-order `held-prefix` edge applies only to leases whose resource starts with `artifact-scratch-private:`, other artifact leases still refuse the toolchain edge; schema `oneOf` matches the TS union.
- JSON import-attribute guard: removals only where the specifier resolves to `.ts`; the new `with {type:'json'}` in `📂️paths/🪟️windows/🟦️.ts` targets a real JSON.
