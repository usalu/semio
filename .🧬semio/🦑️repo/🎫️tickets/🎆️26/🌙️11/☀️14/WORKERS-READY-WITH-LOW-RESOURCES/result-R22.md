# Result R22: zero-touch commands law, native-repo-script law, door startup budget

Script: `T/🧰️scripts/R22/land.py` (175 literal idempotent edits, imports in `imports.json`), landed under the lock; N/LAND-OK written and removed; recorded in `T/landings.md` (the file holds six "R22" blocks, five of them from re-runs of the same land while I iterated to the fixed point; the last block is the final state).

## (a)/(b) Bare imports
Cause: 159 bare `@semio-tech/*` specifiers in 59 files on the closures of the commands test, bench, dev, components, publish, release, schema, verify and of the roster `native-repo-script`. All were workspace packages (no genuine third-party runtime library was found, so no lazy-with-message loader was needed; the law's `external-package` kind has zero findings).
- Wave 1 (126, with file:line and command): `T/🧰️scripts/R22/offenders-wave1.tsv` (columns file:line, specifier, kind, commands).
- Wave 2 (33): files that only became reachable after wave 1 rewrote their parents; `offenders-wave2.tsv`.
- Root edges (why the closures reach so much): `commands/🧪️test` -> `⚡️caching/📜️script.ts` -> `🧪️tests/🔬️script-commands` (dynamic, `repo test cache`, 100+ findings through test files); `commands/🛠️dev` -> `os/dev/🧪️tests/🧪️time-travel` (dynamic; this is also the whole native-repo-script roster); `io/⏱️failure-owner` via `backbone/opening-relay` -> `io/schema/refusal` (components, publish, release); `discovery/.../🕸️wasm` via `📇️inventory` (schema, verify).
- Fix by principle: each specifier became the relative path to the package source entry that `workspaceSpecifierTargetV1` computes (Bun dedups by real path, so there is still one implementation). Nothing else changed in those files.
- Result: `zero-touch` test 37/37 pass, including "the real checkout has no import that leaves a zero-touch closure", "no command of the root registry reaches an import its declared requirement forbids" and R15's "front door and schema closure load from a copy that has no node_modules" (BUN_RUNTIME_TRANSPILER_CACHE_PATH=0). Parse proof: Bun transpiler scan of 59 edited files, 0 failures, 0 missing relative targets. These laws stay the regression guard (they walk the live tree).

## (c) fleet-entry 110 > 100
- `startup` row `fleet-entry` imports `fleet-status/⌨️command`; its static closure was 109 modules. Exclusive static weight by readers import: cache-gc runner 26, host-capacity 13, remote-execution status 8 (the newest landing; `readers` gained that import after 83b2345), texts 6, scheduler+store 3.
- Made lazy (dynamic import at use inside the async readers, same behaviour): cache-gc runner (storage reader), texts (`refuseBuildV1` on the two refusal paths), scheduler + store (scheduler reader), remote-execution status (fleet reader); in the status module the sidecar index/closure are no longer loaded (the shared leaf is imported) and the remote-cache fleet module loads after the endpoint check; doctor `dependencies` probe no longer builds the package-manager catalog at import (9 modules, loaded only when pointers exist).
- Result: fleet-entry 110 -> 67 modules (budget lowered 100 -> 72 as guard; three new groups `remote-execution`, `scheduler`, `cache-gc` are forbidden for that row, so a static import of them fails the law); doctor-entry 107 -> 98 (budget 102).
- Start-up, cold process, 3 runs each (`bun` on the door script, BUN_RUNTIME_TRANSPILER_CACHE_PATH=0, machine load about 20): fleet-status entry import 0.095/0.088/0.081 s before, 0.068/0.064/0.064 s after (about 25 percent). `door repo --help`: 4.35/2.35/2.21 s before, 3.67/2.11/2.16 s after (cold first run is the OS cache; unchanged by this package since the help path does not load fleet-status). The installed launcher `repo --help` is 0.06 to 0.10 s and does not run this code.

## NOT green: startup test still red (4 rows, none caused by the fleet-status readers)
Measured with a scratch copy of the test (identical load function), env without the SEMIO_FLEET_* and SEMIO_REMOTE_CACHE_* variables:
- `fleet-door` 113 modules > 110: this row runs `repo fleet status` in an empty workspace, so every reader's dependencies are loaded at run time whether lazy or not (the set equals the old eager set plus door overhead); the lazy loading cannot reduce it without skipping readers, which changes the report (the scheduler fact prints even when empty). It was hidden behind the first failing row before. Needs a decision: skip readers whose store is absent, or re-baseline this row.
- `caller-entry` bytes 396033 > 391000 and `watchdog-leaf` bytes 21557 > 21000: source growth of `process/.../invocation/process` and the `watchdog` leaf (557 bytes for the leaf), not import growth; needs a slimming of those files by their owner.
- Not touched: I did not raise any number.

## Other tests run under the lock (all pass)
fleet-status (all and guard), remote-execution sidecar tests, doctor tests, zero-touch.

## Unverified
- No real `repo` command ran on a machine without node_modules other than R15's copy proof; the 59 rewritten test/support files were parse-proved and law-proved, not executed (their suites are large integration suites).
- vitest/vite lanes importing those files by relative path were not run.
- The 4 rows above.
