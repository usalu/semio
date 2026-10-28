# review2-1026g (rev2-g, wave 1026g): what landed after the first review

Reviewer: rev2-g, own reading of the code (graph-free; only single-file `bun test ./<file>` runs, one via `C/chain-g-tools/run-test.sh`). `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `G` = `B/🕸️graph/🕰️generations`, `A` = `B/../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers`.

Verdict: no lost update, no second publisher and no stale-serve path found in hold-g, chain-g, nest-f, the pin ledger or the stable Bazel path. One certain small defect fixed (F1, noop-g memo). Two real hold-g liveness findings and a handful of LOW items below.

## Ranking

| # | sev | lane | one line |
|---|---|---|---|
| 1 | MED | hold-g | a `failed` claim covers its owners for the whole base: one churn ENOENT disables prefetch for 128 owners until the next publication |
| 2 | MED | hold-g | the post-build sweep takes the shard writer exclusively without waiting; overlapping cohort holders can starve it, the observation-block inventory then only grows |
| 3 | LOW-MED | hold-g | a `done` claim is not tied to the owner's content: a re-edit after the prefetch is invisible to claims, the exclusive assembly computes it |
| 4 | LOW | hold-g | the prefetch wait happens before the queue clock: no `waiting` notice, no yield ticket; claim-table truncation; admission section probes every running claim |
| 5 | FIXED | noop-g | the source memo's implementation digest ignored the two JSON policies the capture reads |
| 6 | LOW | keys-g | `ownerViews` premise ("a framework module cannot depend on a product") is false for 131 framework files; sound only because of two other mechanisms; no law pins them |
| 7 | LOW | chain-g | adopted snapshot is entry-agnostic; per-module `loaderIdentity()` still hashes 1455 times; `text()` pins a 17 MB string |
| 8 | LOW | rust-g | thread bound is bypassed by a Windows argfile; oracle binary has no `.exe` and its temp dir is never removed; `nativeKeyed` strip of non-Rust importers unproven |
| 9 | OK | ts-g, nest-f, noop-g pin ledger / stable Bazel path / plan-child skip | checked, notes below |

## 1. MED: a failed claim is permanent for its base (hold-g)

`G/🧩️claims/🟨️.mjs:47-56` (`chunkOfV1`): `covered` collects the owners of every claim of the same base and implementation that is not a dead running claim, `done` only of `done` claims. A `failed` claim (`:154`, any non-abort error of a chunk) therefore counts as covered: `open` is empty, `admit` answers `complete`, the command goes on to the assembly, which computes those owners itself under the exclusive publication writer, which is exactly the long hold this lane removed.

Failing scenario: churn is the normal state of the shared workspace (brief: "ENOENT on a source renamed mid-run"). A peer renames a source while command P computes chunk 1 (128 owners); `discoverBuildProjectsV1` throws; the claim is `failed`; every later command with that base sees the 128 owners as covered and prefetches nothing for them; the assembler pays their inference (about 0.4 s warm, 6.5 s cold per project) alone and exclusively. The base only advances with a publication, so with 20 agents the failed chunk is shared by every command until then.

Fix: a failed claim covers only for `failedRetryMilliseconds` (policy, e.g. 60 s, schema + Python oracle + corpus rows "failed 10 s ago covers", "failed 90 s ago is open again") or at most once per owner (count the failures in the claim). Law: in-process scenario "failed chunk is retaken by the next command after the retry time, and by none before".

## 2. MED: the sweep cannot get the shard exclusively while cohort holders overlap (hold-g)

`B/🕸️graph/🟦️.ts` `sweepSupersededBlocksV1` (after `withBuilder`) calls `supersedeBuildGraphObservationBlocksV1` whose lease port is the non-queued `acquireResourceLease(..., mode:"exclusive", timeoutMs:0)` (`B/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🗂️members/🔎️scan/🟦️.ts:165-166`: `if(!lease){busy++;continue;}`). Since 08:44 every computer, every assembler and every prefetcher holds the shard writer as a SHARED COHORT ticket. The sweep runs right after the assembler released its own ticket, when the peers of the same wave (a second assembler queued behind the publication writer, joiners, prefetchers) still hold theirs, so the no-wait exclusive attempt fails; the next attempt is the next build, which again overlaps. The sweep exists because "a graph store whose inventory is exhausted refuses every command" (docstring), so a starved sweep ends in a store that refuses everything. The `graceMilliseconds` of the sweep protects staged blocks, not the inventory.

Failing scenario: three agents edit different files every few minutes; each load becomes prefetch -> assembly with overlapping shared tickets; `busy` is incremented on every build, `removed` stays 0, `ob/` grows past the inventory limit.

Fix: when the group is busy and the block count is above the limit (not for the opportunistic case), acquire QUEUED (`acquireQueuedResourceLease`, a ticket) with a bounded wait, because a queued exclusive ticket makes later cohort arrivals wait behind it (queue order) and the sweep holds nothing while it waits, so hold-and-wait cannot occur. Law (the real-lease test the lane already has): "a sweep with a queued bounded wait is served after the current cohort holders release, and later cohort tickets wait for it".

## 3. LOW-MED: a done claim does not know the owner's content (hold-g)

`G/🧩️claims/🟨️.mjs:52`: a `done` claim stores owner names only. Edit X, command A prefetches X (claim done), edit X again, command B (same base, nothing published yet) sees X covered, prefetches nothing, and the assembly infers X again exclusively (the stale record no longer holds). Correctness is untouched (records are verified when read), the cost is the exclusive hold again; with repeated peer edits of one project this is the common case. Fix: store `{name, stamp}` per owner (the owner's frontier digest the planner already has in `pending`), cover only on equal stamps. Same corpus/oracle change as finding 1.

## 4. LOW (hold-g)

- `G/🔌️load/🟨️.mjs:297-300`: `queued=Date.now()` and the `notice`/yield interval start AFTER `host.prefetch`. While the prefetch waits (behind an old-code exclusive holder for the shared shard ticket, for slots, for a peer's claim) the user sees only throttled `joined`/`slotsBusy` lines and a front door's yield ticket (`graphQueueYieldsV1`) is never offered. Start the clock before the prefetch, pass it in.
- `claims/🟨️.mjs:82`: `rows.length>=P.maximumClaims` skips further claim files silently; the owners of the skipped claims look open and are computed twice. Refuse (answer `complete` and let the assembly decide) instead of dropping.
- `claims/🟨️.mjs:103-121`: the admission lease is held while every running claim's lease is probed (`annotate`), each probe a queued-lease acquisition with a ticket file. "Microsecond" is true for 5 claims, not for 1024. Probe outside the admission section and re-validate the chosen chunk inside it.
- `sweep()` of the registry is never called: done/failed documents of a base that never advances (build never published) stay. Bounded by `maximumClaims`; call it from `prefetchStaleGenerationV1` when `served`.

What I checked and found sound: exactly-once under `kill -9` (a running claim whose lease is free is removed only if still `running` on re-read, `done` is written before the lease is released, `abandon` before release on cancel); lock order is shard (shared) -> admission / slot / claim for prefetchers and shard (shared) -> publication (exclusive) for the assembler, never the reverse, and waiters on a claim wait with nothing exclusive held, so there is no cycle; publication stays one CAS under the exclusive publication writer (two publishers impossible, a computer never publishes); the record store compaction (`⚡️caching/🗃️records/🟦️.ts:161-166`) of a concurrent writer only removes packs that writer itself loaded and a vanished record reads as a miss, so concurrent compaction costs recomputation, not corruption; the sweep and the headroom recovery are kept away from staged blocks by the shared shard ticket and by the sweep grace.

## 5. FIXED: the source memo could outlive a policy edit (noop-g)

`A/🧊️memo/🟦️.ts` `nxAuthorizerMemoImplementationV1` hashed four files (capture, import-site reader, workspace resolver, memo) and the Bun version, but the capture also reads `A/🧬️schema/🔣️.json` (discovery limits) and `A/🧭️sites/🧬️schema/🔣️.json` (site policy and texts). Editing either changed what a capture contains or refuses while a memo written under the old values was still served (memo hit: 74 ms, no re-scan). Fixed: both JSON files are in the list (now the exported `NX_AUTHORIZER_MEMO_IMPLEMENTATION_FILES_V1`), law `🧪️tests/🧊️memo/🟦️.ts` "the memo implementation digest covers every local module and JSON policy the capture modules import" (failed first with exactly those two paths; the memo suite is 3 pass through `run-test.sh`).

The rest of the memo reads as sound: a hit re-reads and re-hashes every captured file (no stat trust), checks ancestor directory identity, remembered absences and data-directory listings, refuses files over 4 MiB, hard links, symlinks and changing stamps, and the nine mutation kinds of its law plus the independent TypeScript `preProcessFile` oracle cover add/replace/rename/hard-link. Remaining LOW: `TranspilerV1` is covered by the Bun version only (a thin wrapper today); a pure `Bun.Transpiler` option change in `🔨️modules/⚙️runtime/🔬️source` would not invalidate.

## 6. LOW: the owner-view premise is false, the result is sound for other reasons (keys-g)

`B/📥️inputs/🌳️owner-view/🟨️.mjs:9` states "a framework module cannot depend on a product". There are 131 files under `🧰️framework/🔨️modules` that reference `🛍️products` by relative path: Cargo path dependencies (`semio-framework-os-kernel`, three manifests), `#[path = "…🛍️products/💻️os/…"] mod` in production Rust (`🌱️value/📸️snapshot/👁️read/🦀️.rs`, `🎒️pack/🔤️json/📥️retained/🧬️schema/🦀️.rs`), `include_str!` of product fixtures in test Rust, and TS tests that resolve product paths. The split stays sound because (a) Cargo path deps are inferred project edges, (b) `cargoSourceInputs` follows `#[path]` and embedded assets per crate (`🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs:70`) so Rust sources never relied on the owner glob (`ownerExtensions` of a cargo project has no `rs`), (c) the TS reads are tests, i.e. OWN tasks of the module, which add `ownerPrivate`; and I found no non-test TS import of a product from the framework tree. The only residue is JSON read through `include_str!` by a crate that a dependent's TEST consumes (e.g. `🌱️value/🧪️testing/🧾️wire/🦀️.rs:21`).

Fix: replace the docstring by the real argument and add the law that pins (b)+(c): "no file in the shared view reads a private directory except through a Cargo path dependency or a source the crate closure follows", with this grep census as the corpus (an exception list is data, not code). Related: `🧰️framework/🛒️products/💻️os` (cart emoji, not bag) is tracked and stays in the shared view (conservative, not unsound; taxonomy drift, FOR taxonomy owner).

Other keys-g points checked: `ownerPrivate` is added to the project's own `default`/`production`/`nativeTestSources` consumers, so every own task keeps the old key set (the table shows 0 % change for the umbrellas); `.cache/command` is under the ignored `.cache` (never observed) and `⚙️configuration/🏃️process/🔣️.json` resolves to the repository root; the old path stays negated for leftovers. `.cache/command` is not a ledger area of the storage budget (the old location was not either); the process module has its own capture retention (`🔒️leases/🟦️.ts:395`), but the disk-budget law ("steady-state `.cache` under a declared budget") does not see it: FOR disk-g. The process-launch domain is narrower than proposed and fail-closed; `🧭️invocation/🕸️graph/🟦️.ts` also holds the plan-file serializer, a content producer, but its output is transient and byte-checked against `JSON.stringify` by its law, so un-keying it re-keys nothing it should.

## 7. LOW (chain-g)

- `B/📥️source/🔏️closure/🚪️command/🟦️.ts` `runAdoptedCapturedBuildSourceV1`: the adopted proof carries no entry; the loader runs the snapshot's entry whatever `args` route to. Today's only caller passes the parent's own args (same entry), so a planner cannot run another implementation or entry: it receives the executor's snapshot bytes (digest-checked against `proof.sha256`) and the loader text of the same module. A future second caller with different args would silently run the parent's entry. Store `entry` in the proof (loader already has `value.entry`) and refuse a mismatch.
- `B/📥️source/🔏️closure/🍞️bun/🟦️.ts` (`onLoad`): `loaderIdentity()` is still evaluated per module (rev1 FOR (a)); for the evaluated variant it can be memoised exactly (the text is a constant). Not done here because the loader text is keyed: every edit restarts every running door/executor pair across the fleet; batch it with the next planned loader edit.
- `proof.text()` returns `encoded.toString('utf8')`: a 17 MB string is created per planner start and kept by the V8/JSC string table until GC; acceptable, noted because the lane's goal is RSS.

## 8. LOW (rust-g, package-body-policy test)

- `🎟️admission/🦀️.rs:317` rewrites `-Z threads=` in the argument vector only; Cargo hands very long rustc command lines over as an `@argsfile` (Windows), where the authored `threads=0` stays unbounded. Expand `@file` arguments in `bounded_threads` (read, rewrite, pass a new argfile) or set the bound through `RUSTFLAGS`-independent `-Zthreads` in the config of the lane; law row with an argfile.
- The in-process oracle build copies the binary to `copy = join(output, "semio-package-body-oracle")` with no `.exe` and never removes `rust-oracle-*` under `SEMIO_TEST_ARTIFACT_DIR`/tmp (a leak per run, cross-platform break on win32).
- `buildSchedulingNativePrefixesV1` keeps `native-build` keyed for the cargo script closure only; `targetScriptSources` still strips it from the own closure of any other target (`🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs:420`). Whether a non-Rust target imports `native-build` and thereby loses a real input is not proven by a law (rust-g OPEN b); add: "the set of targets whose own closure contains a `nativeKeyed` file is empty or each is covered by the fallback".
- Deterministic output under `-Z threads=N` is not asserted: the bound is sound only if rustc's artifact does not depend on the thread count. Add one law comparing the rlib/rmeta digest of the scratch crate compiled with 1 and 4 threads (the lane has the scratch crate and the oracle harness).

## 9. Checked, no defect

- ts-g: `testWorkersV1` default 4 is applied as `--maxWorkers=N` after the config, so the CLI wins over a config; no vitest config in the framework tree sets `maxWorkers`, `singleFork`, `poolOptions` or `fileParallelism`, so no suite loses an isolation setting. The incremental tsc law has three oracles per step.
- nest-f: the pin is bound by owner pid in the invocation/chain environment, alive, younger than 4 h, not self; a recycled pid inside 4 h that also appears in the chain variables is the only hole (pid start time is not compared, LOW). Nested runs never queue and never build; refusal is loud and bilingual.
- noop-g pin ledger: all ledger writers are under the exclusive `build-graph-store` lease, drift is one-sided (removals make the ledger too high, so it sweeps earlier), `at` is not refreshed by incremental additions, so the one-hour bound holds. Stable Bazel package path: a slot is claimed for the whole invocation including the restore of outputs, selections in one slot are serialised, and a different selection only changes file contents that Bazel keys by digest. Plan-child dependency skip: the marker variable is set only in the planner child's own environment and the same marker already turns the process into a plan-only child.

## FOR lines

- FOR gram-g / native-manifests owner: `B/🕸️graph/📥️inference/🦀️native-manifests/🟨️.gram-old.mjs` (18 KB, 08:59) is a leftover copy of the module next to it; it exports a second `cargoSourceInputs` and violates "no leftovers"; delete when the edit lands.
- FOR cutoff-g: unchanged from rev1 (the constant tests/policy re-discovery is the steady-state assembler hold; finding 3 above makes the exclusive part larger under repeated edits).
- FOR disk-g: `.cache/command` is outside the storage ledger.
