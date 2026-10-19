# Result roster: "Bounded unique parent kernel roster required" and "System image plist required"

Executor `roster`, 2026-10-19. Paths relative to the repo; `S` = `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`.

## Root cause

Both errors come from `hdiutil info -plist`, the kernel image inventory every persistent native image re-reads on each `recheck` (about 4 to 9 calls per second per `bun repo cargo` process, 900 calls in 100 s for one `check --tests` run). The inventory is not an atomic snapshot while ANY process (any image, any tool, including Docker, simulators and the test suites) attaches or detaches an image. Measured with a stable attached image, 4 churning attach/detach loops and 16 parallel readers, 14,173 samples through the repo's own capture path:

| Observation | Share | Effect in repo code |
|---|---|---|
| exit 0, valid plist, NO stderr, the stable image is missing (truncated listing, 6 KB to 38 KB instead of 39 KB) | 1.9 % | `matched !== 1` in `verifyPersistentManagedChildrenV1` (`S/🧹️cleanup/🎛️command/🟦️.ts`): "Bounded unique parent kernel roster required" |
| the same mount point listed twice in one document (old and new entity during churn) | seen repeatedly | `new Set(rows.map(root)).size !== rows.length`: same error ("unique") |
| exit 0, valid plist on stdout PLUS `hdiutil: info: cannot find IO media` / `failed getting IO media entries` on stderr | 0.4 % | `captureOwnedCommandV1` returns the merged stdout+stderr tail, so the text starts with the stderr line: `nativeImageInformationV1` throws "System image plist required" |
| entities without `mount-point`/`dev-entry` while an image is mid-transition | frequent | other roster checks |

So the invariant itself is sound; the single-shot read of a torn kernel snapshot was the bug. It is not the 64 KiB tail (the capture with `output` limits keeps the full text and is already covered by the existing information-capture law), not a quota on the number of images (23 listed, limit 4,096), and not the leaked test images (they sit outside every repo root; they only enlarge the listing and widen the tear window). It surfaces under fleet load because every process polls the inventory continuously while other processes (the `dependencies sync` publication cycle, test suites with real images, peer cargo runs) attach and detach.

A second, separate cause of the "590 s" waits: `dependencies sync --all` (deps4 loop, 25 to 40 minutes per run, restarted back to back) holds `cargo:registry:<rust cargo home>` EXCLUSIVE; every `bun repo cargo` first takes that lease exclusively for `cargo fetch --locked`, so it queues in FIFO order with no visible reason. The queue itself is by design (`🏃️process/🦀️cargo/📥️dependencies/🟦️.ts:62`); it was invisible because the child control dropped the wait details.

## Changes

1. `S/🍎️image/📥️information/🟦️.ts`
   - `nativeImageInventoryV1(text)`: accepts only a noise-free document (rejects stderr text before or after the plist: "Torn system image inventory: ...").
   - `observeNativeImageInventoryV1(capture, accept, policy)`: re-reads until a snapshot is whole and `accept` (the caller's invariant) holds; at most `attempts` times with linear backoff plus a pid-derived jitter; retries only tool exit failures, noisy/invalid documents and invariant violations; abort and deadline errors propagate; a lasting violation survives every attempt and is rethrown with the original message plus "(N consistent inventory attempts exhausted)" and the cause.
   - `nativeImageInformationV1` now names the offending first 120 characters in "System image plist required: ...".
2. `S/🧬️schema/🔣️.json` (`ImageProtocol.const`): `informationAttempts: 8`, `informationBackoffMilliseconds: 75` (schema-first).
3. `S/🧹️cleanup/🎛️command/🟦️.ts`: new `inventoryPolicy(control)` and the retrying observer wired into
   - `verifyPersistentManagedChildrenV1` (the cargo path): each snapshot is retried until the parent matches once and mount points are unique; `before` is a double-collect (two equal consecutive snapshots); the redundant third read is skipped when the roster is empty (same number of reads as before in the common case);
   - `inspectPersistentReceiver` (open/retire): oracle check, a snapshot that omits the image while the file system says it is mounted (or lists it twice, or without its mount) is retried;
   - the protected-parent `verify`: the mapping/descendant comparison now runs inside the retried acceptor, with a unique-descendants check;
   - the successful-child retirement check and the activation inspection (noise removal).
   Not changed: the accepted-cleanup and publication channels (`captureNativeImageBodyCommandV1` "information"/`capturePublicationBodyCommand`, `S/📤️publication/🎛️owner/🟦️.ts`) already read stdout only and run under protocol-bound command counts; they still take one snapshot (open item).
4. `S/🍎️image/🟦️.ts`: `readNativeImageInformationV1` uses the same observer.
5. Visible queue waits: `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts` (`ScriptProgress.activity?`) and `.../🏃️process/🟦️.ts` (child control forwards `{resource, mode, elapsedMs}` into every `[script]` frame; after 15 s a human line `[script] <command> still waits for <mode> access to <resource> (<n> s)` is printed).
6. Tests (new): `S/🍎️image/📥️information/🧪️tests/🟦️.ts` + `🔣️.json`: 10 fixture-driven cases (whole snapshot, stderr noise before/after, truncated inventory, duplicate mount, empty output, tool failure, lasting violations exhaust every attempt), the `plutil -convert json` third-party cross-check, the refusal text, and a real darwin law: a stable image plus 3 churning attach/detach loops, 120 inventory reads through `captureOwnedCommandV1`, all must succeed (with `attempts: 1` the same law fails on the first torn snapshot, which I verified, then restored the fixture).
7. Ticket tool (not repo code): `C/roster-tools/reap-leaked-test-images.ts` (lists first; `--apply` detaches). Rule: mount matches `/private/tmp/claude-*/<session>/scratchpad/**/i/m`, another session than the current one, `lsof +f -- <mount>` empty, no process command line mentions the mount, backing file older than 30 minutes or already deleted. Image owner PIDs are not recorded by `hdiutil info` (`hdid-pid` is the live `diskimages-helper`), so the evidence is "no open file and no referencing process". Listed and detached the 9 leaked images of session `214979e9` (`ta2/i-H7Sl5U`, `ta3c/i-nzwVDT`, `i-ELdMcA`, `i-T0HQgl`, `r947rjvsh/i-QuXHch`, `ruqcxaae9/i-b8YfL0`, `r1ndb4qll/i-y0VQMe`, `r1rt6wgqi/i-ZX9lAx`, `rdo4rq0cj/i-K2BQnT`, 2 to 3.4 h old, all 0 open files); all repo partitions, Docker, simulator and Cursor images untouched.

## Verification

- `bun test ./S/🍎️image/📥️information/🧪️tests/🟦️.ts`: 13 pass, 0 fail (14.9 s). Real-churn law with `attempts: 1`: fails ("Bounded unique parent roster required"); with 8: passes.
- `bun test` of `S/🧪️tests/🟦️.ts` (SEMIO_TEST_ARTIFACT_DIR set): 9 pass, 1 fail, unrelated: `ReferenceError: nativeImageUsedBytesV1 is not defined` in `📉️rebudget` (peer edit in `S/🟦️.ts`). The information-capture law there passes with the new code.
- `bun test` of routing `🧪️tests/🟦️.ts` and `📥️invocation/🧪️tests/⏱️finite/🟦️.ts`: 60 pass, 0 fail.
- `tsc --noEmit` (strict, bundler resolution) over the four edited files: no errors in them (only peer errors in `🧱️fixed/🟦️.ts`).
- `S/🧹️cleanup/🧪️tests/🛑️failed-birth/🟦️.ts` NOT run meaningfully: it needs the script-process envelope (`receiveScriptProcessInvocation`), plain `bun test` leaves `born` undefined. `🍎️image/👤️owners` and `🍎️image/🧪️tests/🛑️cleanup` not run.
- `perl -e 'alarm 590; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-hash --lib --manifest-path 🧰️framework/Cargo.toml`: rc 0 on every run that was not queued behind the registry lease: 3m42s, 1m24s, then three consecutive runs H1/H2/H3 at 62 s, 51 s, 53 s (H1 started with no `dependencies` process; `dependencies sync --all` of deps4 started during H1 and ran through H2 and H3, so those two ran under that load). Zero occurrences of "roster required" or "plist required" in any of the 19 logs written after the fix; before the fix the same command failed with them in nine logs of this ticket (`rg-core-c1..3`, `rg-core-fw2`, `split-fw/cargo1,3,4,5`, `split-s-cargo-5d`). A clean three-run window without any sync process never occurred (deps4 relaunches within about a minute); runs that started while the sync held the registry waited and hit my alarm (rc 142, not roster related).
- Peer breakage met meanwhile (all transient, not mine): `nativeFixedWindowsCurrentV1` export missing, missing `🧱️fixed/🗂️system/🟦️.ts` module, `cyclic package dependency semio-framework-value-intrinsic`, `cannot update the lock file`.

## Open items

- Volume: 450 physical rechecks per 100 s in one cargo process = 2 `hdiutil info` per recheck (`relay/🟦️.ts:26`, `🎟️admission/🟦️.ts:308`, `S/🟦️.ts:86-100`). Candidate: single-flight per process (arrivals during a read share the NEXT read, never an older one) or recheck only at commit points.
- Accepted-cleanup/publication information reads (`S/📤️publication/🎛️owner/🟦️.ts:70,73`, command `🟦️.ts:72,159`) still take one snapshot; a torn snapshot there could misread `mapping`. Needs the same observer inside their command-count protocol.
- Registry lease: `dependencies sync --all` holds `cargo:registry` exclusive for the whole run; every other `bun repo cargo` waits (now visible). deps4 could take it per step or run `cargo fetch` for a locked workspace under a shared lease.
- Leaked law images: tests that crash/time out leave `scratchpad/**/i/m` mounted; the ticket tool cleans them, a permanent reaper at the start of `withLawStorageV1` would prevent accumulation.
- `S/🧹️cleanup/🧪️tests/🛑️failed-birth` and the rebudget law need the repo harness; `nativeImageUsedBytesV1` reference error is a peer's.
