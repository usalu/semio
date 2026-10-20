# result-cli-latency: fixed overhead of a repo command, measured per phase

`C` = this ticket. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `B` = `L/🏗️build`. Tools and staged set: `C/cli-latency-tools/`.

## Status
READY for GO, nothing applied by me beyond two inert live leftovers (see "Live state"). Measured live with LIGHT commands only (`--help`, `leases status`, `cache status` for at most 9 s), host load average 21-51 the whole time (kernel is saturated by the fleet), so absolute numbers are 2-3x inflated; ratios and counts are reliable. The `run`/`test` warm path itself was NOT run (chain16/17 own the graph); its phases are in the profiler hooks and in section 5.

## Live state (needs a coordinator decision)
- New, inert module `B/⏱️profile/` (profiler `🟨️.mjs`, `🟨️.d.mts`, schema, 8 tests passing) was created live BEFORE the chain17 notice. Nothing imports it. Copy staged at `C/cli-latency-tools/staged/profile-module/`.
- Stray temp file `B/.profiled-script.ts` (hooked copy of the CLI script, dotfile). Delete it whenever an edit is allowed (`rm`); I did not touch it after the notice.
- No other file under `L/` or `🧰️framework/🔨️modules/🏃️process/` was edited. The hooks into `📜️script.ts` are staged only.

## 1. The profiler
`SEMIO_REPO_PROFILE=1|json|off` (anything else is refused in EN and DE). Output on stderr at process exit, language neutral: tab separated rows `seq depth phase start duration self detail` (ms since process start; self = duration minus the union of nested children), a `#total/covered/uncovered` row, one block per process; column legend in EN/DE in the module and schema. `json` prints one schema-valid document. Phases nest per asynchronous context; child processes (`Bun.spawn`, `node:child_process`) appear as `spawn.<program>` events under the surrounding phase. Disabled cost: one function call.
Tests (`B/⏱️profile/🧪️tests`): fixture-driven self-time rows checked against an independent half-millisecond grid, Ajv on the JSON schema, concurrent phases, failing bodies, name validation, table format, a real child process printing a valid JSON profile on exit and counting its own spawn; strict `tsc` clean.
Staged hooks: `C/cli-latency-tools/apply-profile-hooks.py` (22 anchored edits in `B/📜️script.ts`, dry run default, `--write`, `--target <copy>`; verified on a hooked copy for `--help`, `leases status`, `cache status`). Phases: `boot.loaded/main`, `door.load/capacity/admitted/capabilities/script`, `invocation.gate/admitted/load/read/recovery-plan/body`, `leases.load/open/close`, `diagnostics.opened/closing` (marks), `dependencies.opening/opened/ensure`, `tools.storage`, `graph.load/rust-ready/pinning/pinned`, `receipts.tasks`, `tool.nx|bazel`.
Without any edit, the same phases plus image internals come from the preload `C/cli-latency-tools/profile-preload.ts` + `profile-targets.json` (wraps named functions of the live sources in profile phases at load time): `SEMIO_PROFILE_ROOT=$PWD SEMIO_PROFILE_TARGETS=<targets> SEMIO_REPO_PROFILE=json bun --preload <copy in an ASCII path> ...` (bun cannot preload a path with emoji). Readers: `profile-agg.py` (median table over runs), `profile-tree.py` (tree with child-process counts per phase).

## 2. Measurements (live, light commands)
| Command | wall | notes |
|---|---|---|
| `bun repo --help` | 160-210 ms at load 21 (goal 200 ms: met only at moderate load; 334-1188 ms at load 50 under the profiler) | process age at `boot.main` 90-160 ms: static import closure (131 modules) |
| `repo leases status` | 160-180 ms (load 21), 254-324 ms with profiler (load 48) | 3 sysctl + 1 hdiutil spawns for the host sample |
| `repo capacity status` | 118-144 ms | |
| `repo cache status` | not finishing: walks `tools/nx`, `tools/sdk` (hundreds of progress lines, >3 min); I stopped it with SIGINT | `cache status` is O(files); needs a stat cache (open item 6) |

Phase anatomy of one command that reaches its body (`cache status`, load 40, profiler + preload, partition internals), ms:
| phase | ms | what |
|---|---|---|
| boot to `boot.main` | 160 | bun start + static closure |
| `door.load` + `door.capacity` self | 66 + 117 | import capacity module, parse, ask (no sample taken for a light command) |
| `door.capabilities` | 546 (120-160 at idle) | `createBuildBootstrapCapabilitiesV1`: computation is 5 ms, the rest is module loading of `invocation` (35-75), `caller` (55-72), `bootstrap/cli`, nx operations, nx bootstrap caller |
| `leases.load` | 226 | import of the image-opening module (~60 modules) |
| `leases.open` | 383 | 2 persistent image opens (open, sweep, close, reopen) = 10 `hdiutil` rounds (237 ms of children), each open = `image.inspect` (3 rounds) + `image.verify-children` x2 |
| diagnostics store open (`store.with` to `diagnostics.opened`) | 840 | nested `leases` open again (114), diagnostics image open (279, 3 rounds), 7 further `verify-children` rechecks (18-43 ms each, 1 hdiutil each), 252 ms self (lease acquisition) |
| total before the command body | 2363 | of which: module loading ~1100, image/lease custody ~1220 |
Earlier run with FIFO contention: the same diagnostics open took 4.2 s, of which 2.85 s was a gap with no child process (shared image lease queued behind an exclusive waiter) and 22 `hdiutil` spawns in total (1247 ms).
Module loading alone (fresh process, only imports, no execution): door closure 60-395 ms, `run`+nx closure 350-440 ms CPU 410-480 ms (wall up to 1 s under load), `run`+bazel 351 ms. `lstatSync` is 175 of 700 profiled ms of the closure load: the source-revision self-capture (`inputs/🔏️revision`) is 181 `capture` calls (112 ms) + 120 `current` self-checks (61 ms). A one-file bundle was tried and does NOT work: modules capture their own physical source revision and refuse a bundle ("Bounded physical Source revision required"); do not pursue bundling.
Bun's transpiler cache is active (cache disabled: +13 % CPU).

## 3. Findings, ranked
1. **Every partition open re-opens the lease image** (`openRepositoryCachePartitionAttemptV1` opens `leases` again), although the CLI door already holds one for the whole command: >= 1 extra full image open (110-260 ms at load 40, 3-5 `hdiutil` rounds) per partition; a `run` opens diagnostics, dependencies, tools, graph, ... partitions. Fix: group `command-custody`.
2. **Roster/recheck traffic**: 22-24 `hdiutil info` spawns before the body of a bare command; every `image.verify-children` recheck is a spawn (18-43 ms at load 40). The recheck window exists (`withNativeRecheckWindowV1`, 500 ms) but only `launchRepositoryCargoV1` enters it. Fix: same group (the CLI command runs inside the window).
3. **Lease sweep stamp is 15 s**: any command started later than 15 s after the previous one pays open + sweep + close + reopen of the lease image (about 230 ms and 8 rounds at load 40). Fix: 5 minutes (`sweep-interval`).
4. **Module loading is ~45 % of the fixed cost** (350-480 ms CPU for the `run` closure, 1142 files). Fixes staged: lazy Bazel-test contract compile (measured: no difference, 51 vs 51 ms import, the schema compile is already cheap; kept as hygiene, drop it if the coordinator prefers), `revision-stamp` (about 60 ms of the 175 ms lstat). The big lever is not importing what a warm path does not need: `graph/🦀️rust` (63-142 ms), `graph` (48-65), `caller` (53-75), opening (27-40), nx bootstrap script (35-46) are all loaded before the first tool process starts; with a valid graph generation the warm path should need a generation reader only (open item 1).
5. **Unknown commands pay the whole chain and queue behind exclusive locks**: a typo (`repo "leases status"` passed as one word, also `repo test x`: `test` is not a repo command) went through capacity, leases, diagnostics and the dependencies/graph locks and then waited behind chain16's exclusive graph holder for over 100 s before being refused (I interrupted two such runs with SIGINT; no lease of mine remains). Fix: `door-validate` refuses at `door.capabilities` (about 150 ms) with an EN+DE message naming the allowed commands.
6. **Capacity gate for a warm no-op**: `run`, `run-many`, `affected`, `exec`, `watch` ask for 1536 MB + kinds and wait while load > 2 per core (here: 3 `run`/`cargo` commands queued 670-750 s with "the load is above its ceiling"). A cache-hit no-op consumes nothing; admission belongs after the "is there work" decision (open item 2).
7. `repo cache status` walks every cache home on each call (open item 6).

## 4. Staged fixes (`C/cli-latency-tools/staged/apply.py`, dry run default, all anchors resolve on the live tree today: 21 operations, 0 refused)
| Group | Change | Test |
|---|---|---|
| `command-custody` | `withRepositoryCommandCustodyV1(workspace, leasesImage, body)` in the opening module: AsyncLocalStorage-shared lease image (partition owners never close it) + `withNativeRecheckWindowV1`; the CLI door wraps the diagnostics store call. Expected: -1 lease image open per partition, -5..9 rechecks per command | NOT unit-tested (needs images). Acceptance: `SEMIO_REPO_PROFILE=json` before/after on `cache status`: `image.open-platform` count 5 -> <= 2 and `spawn.hdiutil` 22 -> < 10 |
| `sweep-interval` | `leaseRetirementIntervalMilliseconds` 15000 -> 300000 + stamp fixture ages | existing stamp suite (7) passes with the new fixture in an overlay |
| `door-validate` | catalogue check in `createBuildBootstrapCapabilitiesV1` for the `repo` entry only | new `📥️caller/🧪️tests/🚪️door` (3 tests, fails without the change, `yargs-parser` as independent reader); EN+DE message |
| `door-validate` (light commands, added on the coordinator's follow-up) | `buildDoorCommandV1` names the command after `--tool`/`--workers`/`--graph-policy` (via `parseBuildInvocationV1`, no storage); `runBuildCliV1` routes `leases`/`capacity` on that name, so `repo --tool=bazel leases status` takes the light path instead of capacity, images and graph discovery | new `🧪️tests/🚪️light` (2 tests: 8 fixture rows with `yargs-parser` as independent reader; a real process prints `--tool=bazel leases|capacity --help`), passes in the overlay |
| `revision-stamp` | `current()` self-check trusts an unchanged, settled stamp (taken before and after the load capture) | new `🧊️own-stamp` (2 tests: digests vs Node and Python hashlib, fewer lstat than after a touch, appended/flipped bytes still refused) passes in an overlay |
| `lazy-contract` | compile the Bazel test contract on first use | import + behaviour identical |
Verification: overlay built from the 1141-file static closure under `/private/tmp/cli-latency/ov2` (deleted), `bun test` for the three suites above, `tsc --strict` for the profiler. NOT run: the opening/store suites (need the repo harness), any live apply, Windows/Linux (all paths use `node:path`, `AsyncLocalStorage`, no new platform calls).
Order after GO (chain17 green): copy `staged/profile-module` (or keep the live copy), `apply-profile-hooks.py --write`, run the light baseline (`bench`: `SEMIO_REPO_PROFILE=json` x5 for `--help`, `leases status`, `cache status` stopped at 9 s with `run.py`), then `staged/apply.py --write`, rerun, compare with `profile-agg.py`.

## 5. Open items (not staged, ranked by gain for the 1 s goal)
1. **Warm path without the cold stack**: `repo run <t>` after a valid graph generation should import only the generation reader, take the tool storage SHARED once and exec the tool; defer `graph/rust`, `selection/closure` (`storeReceiptTasksV1` walks `selectedBuildPrerequisitesV1` over the whole prerequisite closure before every Nx run, up to 65536 tasks) behind "the tool says there is work". Needs the chain17 generation for measurement (`--graph-policy=current`).
2. **Admission after the no-op decision** (finding 6): ask the tool for a dry plan (Bazel: no-op build is ~0.2-0.4 s with a warm server; Nx cannot do a no-op below its own ~1.3 s startup, so the 1 s goal for `bun repo run` holds only on the Bazel lane or with a repo-level result memo keyed by the Nx task hash).
3. **Resident CLI worker**: thin client over a unix socket / named pipe (`node:net`, cross-platform) to one resident process holding the module graph, the leases image and the custody session from `createRepositoryCustodyPoolV1` (already live, idle/age limits so it never starves `dependencies sync`). Per request it would still run `recheck`s (~20-40 ms each) but no module loading (-0.45 s CPU) and no image opens (-1.2 s at load 40). Constraints found: the CLI mutates `process.env`/`process.exitCode`/cwd and writes `console.log`, so each request needs its own Worker or child; the shared SHARED guards must be released when idle. Not started: it is a multi-day change with a protocol schema (cancellation, progress, EN/DE).
4. Single process-wide revision memo: `captureBoundedSourceRevisionV1` re-lstats the whole ancestry about 12 times per call; memoizing ancestry per capture batch would remove another ~100 ms of the closure load but touches a custody-relevant module (`🏗️build/📥️inputs/🔏️revision`).
5. Trim `door.capabilities`: `createBuildBootstrapCapabilitiesV1` computes nx launches for `--help`; skip for help/unknown (partly done by `door-validate`).
6. `repo cache status` stat cache / size ledger instead of walking `tools/nx`, `tools/sdk`.
7. `repo --help` margin: the `compileJsonSchemaSubset` prepare (`ready` 14 ms) and the 131-module static closure of `📜️script.ts` (move `🎟️capacity`, `bazelTest` contracts, providers behind the commands that need them).

## 6. Pitfalls recorded
- zsh does not word-split `$c` (`for c in "leases status"` ran `repo "leases status"`): use arrays or explicit arguments for CLI loops.
- `bun --preload` cannot take a path containing emoji; copy the preload to an ASCII path. Patching `require("node:fs")` works for modules linked after the patch only.
- Copying `🧬️schema` directories recursively filled 10 GB (nested schema dirs); the overlay builder (`/private/tmp/cli-latency/mkov.py`, deleted) copies only the paths named by `ENOENT` errors with a 50 MB cap.
- No `timeout` on macOS; `C/cli-latency-tools` uses `run.py`-style SIGINT-then-kill wrappers (copy in `/private/tmp/cli-latency/run.py`).
