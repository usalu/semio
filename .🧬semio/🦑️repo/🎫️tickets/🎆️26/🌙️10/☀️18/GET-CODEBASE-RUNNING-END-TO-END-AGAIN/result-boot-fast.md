# result-boot-fast: a `repo` command that does no work answers in 50-150 ms and loads 8 modules

`C` = this ticket. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. Tools, staged set, raw results: `C/boot-fast-tools/` (`staged/`, `tools/`, `results/`). Wave 1021a, executor `boot-fast`. Stage-only: nothing outside `C/` was written; every number below comes from read-only light runs of the live checkout (max 3 per command, `nice -n 10`, load guard) or from copies under `/private/tmp/boot-fast/` (clone-on-write overlays, never deleted by me, safe to remove).

READY (stage only, after round 6). `python3 C/boot-fast-tools/staged/apply.py` (dry run) against the LIVE tree today: groups `door`, `caller`, `taxonomy` resolve all their anchors; groups `launcher` and `slo` refuse with "edit target missing (apply its set first)" until repo-launcher and perf-slo are applied, which is the intended order.

## 1. Numbers

Host: macOS arm64, 10 cores, load average 31-48 during every run (3-5x oversubscribed by the fleet, so absolute times are inflated; ratios and counts are reliable). The `bun` floor on this host is 28-65 ms for `bun -e 1`.

### 1.1 Live checkout, today (read-only, before any of my changes)
| command | result | note |
|---|---|---|
| installed Go `repo` binary (built 10-20 12:49) `--help` | 20-60 ms | answers natively; it is the stale launcher, not the staged one |
| same binary `help run`, `leases status`, `tools status` | 29-43 ms, `unknown command ... for "repo"` | stale binary: no `help`, no `leases` verb |
| `bun <build script> repo --help` | 236 / 188 / 185 ms | 3 runs |
| `... repo run --help` | 287 / 324 ms (2 clean runs) | the first three runs (16:20-16:22) hit the 30 s kill: progress lines `Inspecting source metadata: 49,344 operations`; not reproduced in the next five runs, a peer had edited `package.json` at 16:17 (dependency generation stale), see 4.7 |
| `... repo leases status` | 204 / 269 ms (514 ms profiled) | three earlier 30 s kills in the same 16:20-16:22 window |
| `... repo tools status` | refused after 3,504 ms, exit 1 | `Expected repo tools setup [nx|bazel|all]` is thrown inside the dependencies partition, i.e. after capacity, leases, diagnostics and dependencies custody |
| `... repo cache status` | cancelled at 25 s | still walks (`--deep` behaviour); the staged cache-gc set removes the walk |
| `... repo run --help --lang=de` | refused after 997 ms, exit 1 | `Undeclared original Repo build selection`: a live defect, fixed in group `caller`, see 4.3 |

### 1.2 Overlay, round-6 state (ckpt-hot-cache of the integrate2 simulation + perf-slo) before vs with the set, interleaved, n = 13, load 45
Time to first stdout byte (exit time in brackets), p50 / min, ms. Command lines are `repo <args>` through the build script (before) and through the front door (after).
| command | before | after the set | after / before | modules loaded |
|---|---|---|---|---|
| `--help` | 580 (583) / 379 | 85 (144) / 52 | 6.8x faster | 157 -> 8 |
| `run --help` (the "help run" of the brief; there is no `help` verb) | 538 (540) / 265 | 89 (131) / 46 | 6.0x | 157 -> 8 |
| `leases status` | 279 (286) / 130 | 211 (223) / 101 | 1.3x | 157 -> 23 (169 KiB) |
| `capacity status` | 249 (271) / 133 | 133 (136) / 80 | 1.9x | 157 -> 19 (124 KiB) |
| `tools status` (refusal, exit 2) | live: 3,504 | (114) / 55 | 31x | 157 -> 8 |
| `bun -e 1` (floor) | | (63) / 37 | | |
Earlier, quieter runs (load 31, `bench-door1.json`): `--help` 340 -> 112, `run --help` 330 -> 101, `leases status` 279 -> 164, `capacity status` 275 -> 158, refusal 95 (p50 ms). The help-only floor (5 modules, no door) measures 64 ms, so the door spends about 40 ms over the process floor for 3 more modules, the catalogue (87-108 KB JSON) and the profile hooks.

### 1.3 Anatomy of one run (`SEMIO_REPO_PROFILE=1`, single runs, `results/profiles.txt`)
| path | to `boot.main` | in the command | exit |
|---|---|---|---|
| front door `--help` | 61 ms | `door.plan` 1.0 ms, then prints | 65 ms |
| front door `leases status` | 44 ms | `door.plan` 0.8, `door.light-load` 34.3 (import of the command's 15 further modules) | 97 ms |
| build script `--help`, before | 176 ms (97 static modules) | `door.load` 15 + `door.capacity` 139 of which `door.capabilities` **121** + `door.script` 14 | 372 ms |
| build script `--help`, with `caller` group | 142 ms | `door.capabilities` 69, `door.capacity` 100 | 517 ms (load spike) |
`door.capabilities` is `createBuildBootstrapCapabilitiesV1`: it resolves the Nx bootstrap launches for a command whose help is all it will print. It is 50-120 ms on every command that takes the full path (open item 1).

## 2. Where the time goes (import graph, top-level work, redundant reads)

### 2.1 Import graph cost table (`results/before-script.md`, `after-script.md`, `door.md`; tools `import-trace.ts` + `import-isolate.ts` + `import-table.py`)
`self ms` is the wall time of `await import(m)` in a warm process with all of m's dependencies already loaded (read + transpile cache + top-level evaluation), median of the best of two warm runs, instrumentation included, so read it as relative cost; `total` adds the subtree of the module that first imported it.
| route | modules (eager static / lazy) | source | sum of own import time |
|---|---|---|---|
| build script `--help`, before | 157 (96 / 61) | 1,476 KiB | 159 ms |
| build script `--help`, `caller` group | 97 (53 / 44) | 937 KiB | 106 ms |
| front door `--help`, `run --help`, refusal | 8 (8 / 0) | 37 KiB | 9 ms |
| front door, `leases status` / `capacity status` route (door 8 + the command's own closure) | 23 / 19 | 169 / 124 KiB | |
| the entry closures alone: `leases` / `capacity` / `doctor` | 17 / 13 / 57 | 138 / 93 / 730 KiB | |
Heavy groups loaded by the build script `--help` before (modules, KiB, own ms): native build storage 34 / 332 / 29; Cargo test harness and sandbox 18 / 111 / 11; leases 6 / 123 / 17; graph 5 / 85 / 9; caller 5 / 57 / 10; routing 5 / 69 / 4; resident service client (hot-cache) 6 / 16 / 3; host capacity 6 / 57 / 5; store receipt 1 / 12 / 2; invocation parser 1 / 24 / 1.
Top modules by own import time before: `♻️retirement/🧭️continuation` 10.0, `⚙️runtime/🧭️selection` 6.8, `📥️caller/📤️orchestration` 6.8, `📤️arguments` 6.3, `🕸️graph/🕰️generations` 5.2, `🧾️observation/📁️descriptor` 4.5 (167 KiB), `📖️syntax/🦀️rust` 4.3 (45 KiB). Subtree leaders: the `caller` module (83 static modules, 808 KiB; 35 / 362 KiB after).

### 2.2 Eager imports that no light command needs
1. **`📥️caller/🟦️.ts` imported the Cargo execution contract statically** (`🏃️process/🦀️cargo/🏃️execution`), which imports the Cargo test harness, the test sandbox and the native build storage (about 60 modules, 540 KiB, 53 ms of own import). The caller loads it for every command through `readOwnedBuildInvocationV1`, although only a native owner command (`issueBuildTaskEnvironmentV1` with `execution.nativeOwnerCommand`) calls it. Fixed (group `caller`); module import alone: 133 -> 87 ms min, 83 -> 35 modules.
2. The 30-module closure of `🧭️invocation` (313 KiB) is loaded only to learn the command name after `--tool`, `--workers` and `--graph-policy`. The front door reads that name with the 1-module options reader.
3. `🏡️resident` (hot-cache, 6 modules) loads in the build script before any command is known; the front door never loads it.
4. `📜️script.ts` itself has 96 static modules (895 KiB) before `main` runs; its source laws (`birth`, `phase`) parse the file, so it was not restructured. The door is a new entry in front of it instead.

### 2.3 Top-level work and redundant re-reads
| finding | measured | decision |
|---|---|---|
| ancestry `lstat` of the source revision capture (`📥️inputs/🔏️revision`, one chain per captured module) | 550 `lstat` of 563 fs calls on the script `--help`, each ancestor directory 36 times, 8 ms; door: 12 fs calls | not touched (custody module); single capture batch memo is open item 4 |
| `captureBoundedSourceRevisionV1` | 8 calls, 7-13 ms | as above |
| schema compile and validate (`compileJsonSchemaSubset` 21 calls, `validateJsonSchemaSubset` 13, host capacity policy 15.7 KiB compiled 13 times) | 2.6-3.5 ms + 3-4 ms per process; warm 13-65 us per validate, cold first call 2.7 ms | a content-addressed memo was built and **measured slower** (key by `JSON.stringify`: 36 vs 28 us on the 2 KiB caller schema, 158 vs 65 us on the 15.7 KiB policy) and was dropped; a cross-process cache of compiled validators cannot beat JavaScript parse + JIT of the same closure. No precompiled-validator cache staged, on evidence |
| taxonomy `🔣️taxonomy.json` (1.33 MB) | parsed with `JSON.parse` in `resolveBuildSelectionV1` for a `workspace:test` selection, 3.4-157 ms per parse under load, at least four selections per invocation; graph inference reads it through the observation cache (`observedSourceAnalysisV1`), not measured here | memoized by file identity (group `taxonomy`) |
| OS language lookup (`defaults read -g AppleLanguages`) when no language variable is set | within noise of the other runs (<= 15 ms min) | none |
| a timeline note: the `cpuprof` of Bun (8 ms samples, 51 samples) put 36 % on revision capture and 15 % on schema snapshots; direct function timers say 7-13 ms and 6 ms. Do not trust `--cpu-prof` for a 200 ms process | | |

## 3. The staged set (`C/boot-fast-tools/staged/`, `apply.py`, dry run by default, drift refusal)

| group | change | verification |
|---|---|---|
| `door` | new module `B/🚪️door/` (`🟦️.ts` planner and runner, `📜️script.ts` entry, `🔣️.json` policy, closed schema, README EN/DE, 7 tests, startup law) and a `repo-door` row in the zero-touch closure list | 7 door tests, 5 law tests, third-party oracles (Ajv strict, yargs-parser, esbuild), 21 of 21 mutations killed |
| `launcher` | the launcher registry `implementation.entries.build.script` names the door; the Go entry-contract test follows | JSON-structural edit with layout and value check; not run: Go suite, a rebuilt launcher |
| `caller` | lazy Cargo execution contract; declared-selection check consumes `--lang`, `--service`, `--graph-policy` like the parser; new test `🌐️globals` | the test is red on the reference overlay and green with the group; help-offline and one operations test turn green |
| `taxonomy` | test phase lists parsed once per file identity (`statSync` identity); test | 2 tests; 2 mutations killed |
| `slo` | objective `cli-help-p50` (goal 100 ms, gate 250 ms, blocking, source `boot-fast`) on the existing `cli-help` probe, `boot-fast` in both source lists of the schema, perf-slo's selected-rows expectation | perf-slo suite 37 pass, 1 environment failure, identical to the reference |

### 3.1 The front door (`B/🚪️door/📜️script.ts`)
It is the entry the launcher starts for build verbs. It resolves the workspace root without loading the routing module, plans from the catalogue alone (`planBuildDoorV1`: normalize options, resolve the language with the shared rule, consume `--tool`/`--workers`/`--graph-policy`, read the command), and returns one of four routes: `help` (the catalogue renderer, any command, any language), `refused` (bilingual, exit code 2, stderr, before any storage: unknown command with the list, unknown subcommand of a closed set, `--tool`/`--workers` or their variables outside the catalogue's `nx|bazel`, `on|off`), `light` (`leases`, `capacity`, `doctor`: only the command's own closure) or `full` (hands over to `B/📜️script.ts`, unchanged, including `build`/`test` script routes and the dashboard target). Commands, subcommands, options, languages and help texts are never listed by hand: they come from `⚙️operations/🔣️catalog.json`; the door's own data is `🔣️.json` (closed by `🧬️schema/🔣️.json`): the light operations, `closedSubcommands` (tools, show, incremental, concurrency, dependencies, cache, remote-cache, capacity, leases, remote-execution), `fullTargetPrefixes`, EN/DE refusal texts. A light command is one literal loader in `buildDoorLightLoadersV1` (bundlers and closure laws see it). No default language is assumed.
Compatibility kept: `📜️script.ts` is untouched (its source laws, the `repo-cli` closure, `bun repo ...` from `package.json` and Nx bootstrap spawns keep working); the three diagnostic branches of its `runBuildCliV1` stay for direct callers. Not edited: root `package.json` (stales the dependency generation; the `repo` script should name the door in the lock wave).

### 3.2 Law: a light command that loads a heavy module group fails (`B/🚪️door/🧪️tests/🚦️startup/`)
Language-agnostic fixture `🔣️.json`: 15 module groups by path substring (graph, native build, test harness, Cargo launcher, caller, invocation, selection, resident, bootstrap, store receipt, tools, revision, routing, leases, host capacity) and rows: help, command help, refusal and unknown command (forbid all groups, at most 10 modules and 48 KB), `leases`/`capacity`/`doctor` entry closures (forbid what they do not need, at most 22/17/70 modules), `caller-entry` (forbids native build, test harness and Cargo launcher: guards the lazy Cargo edit) and a negative control (the Cargo execution module must reach exactly the three groups, so the trace has teeth). The test spawns the real door under a Bun load plugin and, independently, bundles the door core with esbuild: the traced module list equals the bundler's static closure (6 files plus entry and the runtime main check). A fifth test requires the performance table to state the startup objective (skipped when perf-slo is not applied).

### 3.3 SLO row (language-agnostic JSON, `🏎️bench/🎯️slo/🔣️.json`)
`cli-help-p50`: probe `cli-help`, `wallMs`, p50, goal 100, gate 250, tools `["neutral"]`, **blocking**, tolerance 20 % + 25 ms, source `boot-fast`. It is reachable today (door `--help` p50 85-112 ms at load 31-45, min 52). The existing `cli-help-p95` (200 / 400) and `cli-help-p99` (gate 1000) rows keep their advisory status. Not staged: probes with their own arguments (`run --help`, `leases status`): perf-slo's `cli-help` probe hard-codes `["--help"]` and its test pins `probes * 2 - 1` neutral pairs, so a second probe needs a coordinated change in its runner, model and test.

### 3.4 Verification
Overlay chain: base = integrate2's `ckpt-hot-cache` (the `i6/ov` tree moves while its simulation runs), perf-slo applied by its own `apply.py`, then this set.
- `staged/apply.py` on a fresh overlay: 19 changes, second run 0 (`already`); the generated caller and invocation files equal the hand-edited ones. `tools/apply-check.py`: dry run writes nothing, each group alone, `launcher` refused without the door, 8 kinds of drift refused with exit 2 and nothing written, a create target with other content refused.
- New tests (`bun test`, one file at a time): door 7, startup law 5, taxonomy 2, caller globals 1: all pass. `tools/mutation-check.py`: 21 seeded defects (planner rules, exit code, placeholders, language, root finder, three heavy imports into the door, the Cargo import back into the caller, two caller selection defects, two taxonomy defects), all killed.
- Existing suites, reference overlay vs overlay with the set (`tools/suite.py`): perf-slo bench 37/1 vs 37/1; operations 10/5 vs 11/4 (one red test fixed); `help-offline` 0/1 vs 1/0 (fixed); invocation 7/1 vs 7/1; caller door 3/0 vs 3/0; caller service 116/1 vs 116/1; operations requires 2/1 vs 2/1; zero-touch cannot run in an overlay (8/26 in both). The remaining reds are the overlay's missing files or `testRunnerOnly` guards (birth, phase, build contract suites need `repo test`), identical on both sides.
- Strict typing: not run (`tsc` over the overlay was not set up); the files use only constructs of their neighbours, `bun` transpiles them, and the tests exercise every export.
- NOT run: the live tree (frozen), a rebuilt Go launcher and its entry-contract test, `verify zero-touch` as a command, `repo test` runners, Windows and Linux (code uses `node:path`, `node:fs`, no new platform call; absolute destinations are at most 126 characters, ticket paths at most 152).

## 4. Findings the coordinator should route

1. **`door.capabilities` costs 50-121 ms on every full command**, open item 1 below.
2. **`repo help <cmd>` does not exist** (the Go binary and the catalogue refuse the verb). `repo <cmd> --help` is the documented form; adding `help` needs a catalogue operation (a launcher verb is derived from it).
3. **LIVE DEFECT: `--lang=<l>`, `--service=<v>` and `--graph-policy=<v>` end every full-path command in `Undeclared original Repo build selection`** (proved live with `repo run --help --lang=de`, exit 1 after 997 ms; proved in the round-6 overlay for all three). Root cause: `readOwnedBuildInvocationV1` consumes `--tool` and `--workers` by hand, while the declared launches were derived through `parseBuildInvocationV1` and `buildOperationArgumentsV1`, which also consume `--lang` and `--service` (hot-cache's one-shot worker runs `repo run <id> --service=off`, so the resident tier 2 fails without this) and `--graph-policy`. It also keeps `help-offline` and one operations test red. Group `caller` fixes it with the shared reader. The door answers help and the light commands without reaching that check.
4. A failing refusal is slow: `tools status` pays 3.5 s of custody before the subcommand is refused (fixed for the closed sets by the door).
5. hot-cache's resident client modules are in the build script's static path (6 modules) even for help.
6. The round-6 chain makes the script `--help` slower than before (269 ms at the partial chain, 378 ms at `ckpt-hot-cache`, similar load): 10 more modules (6 of them the resident client) and a heavier `door.capabilities`.
7. Hypothesis, not proven: a peer's `package.json` edit at 16:17 staled the dependency generation and made light live commands wait on the source scope walk of the dependency bootstrap for about two minutes (`run --help`, `leases status` timed out at 30 s with progress lines of 36k-49k operations; a profiled rerun six minutes later did not enter `ensureBuildEnvironmentV1` at all). Not reproduced; if the `ensure` gate sits behind any help or diagnostic route it is a defect: the door makes both of them independent of it.

## 5. Open items, ranked by gain for "well under 1 s"
1. **Trim `door.capabilities` for the full path** (50-121 ms per command): compute the Nx launches only when the selected operation runs Nx; skip for help, `leases`, `capacity`, `cache` and every neutral operation (the catalogue already says `implementations.nx = neutral`). Needs the caller authority tests, so not staged.
2. **Static closure of `📜️script.ts`** (96 modules, 895 KiB before `main`): move the envelope, router and provider imports behind the command that needs them; blocked by the source laws that parse `runBuildCliV1` and the phase test, so it needs those laws moved to the door first.
3. **Go-native help**: the launcher can render the catalogue itself (the language-agnostic help snapshot fixture exists: 100 snapshots EN+DE) and answer in about 5 ms instead of the 46-60 ms Bun floor; hook is `Host.Forward`.
4. **One ancestry memo per capture batch** in `📥️inputs/🔏️revision` (550 `lstat`, 8 ms, in a custody module).
5. A `help` verb and `--version` for build verbs in the catalogue; probes with arguments in perf-slo; a `repo cache status` light route once the ledger read needs no diagnostics store.
6. `package.json` `repo` script and the dashboard authority fixtures name the build script path; switch them to the door in the lock wave.
7. If the leases and doctor commands should be fully custody-free, their closures (17 and 57 modules; doctor loads native build storage and routing for its probes) are the next group split.

## 6. Apply
```
cd /Users/ueli/Documents/semio; C=.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
python3 $C/boot-fast-tools/staged/apply.py                 # dry run, all groups (door caller taxonomy launcher slo)
python3 $C/boot-fast-tools/staged/apply.py --write         # after round 6 and perf-slo; --group <name> for one
bun test ./B/🚪️door/🧪️tests/🟦️.ts ; bun test ./B/🚪️door/🧪️tests/🚦️startup/🟦️.ts ; bun test ./B/🧭️invocation/🧪️tests/🧊️taxonomy/🟦️.ts ; bun test ./B/📥️caller/🧪️tests/🌐️globals/🟦️.ts   # one file at a time
repo --help ; repo run --help --lang=de ; repo tools status    # launcher binary rebuilt from the repo-launcher set
```
Order: after repo-launcher, hot-cache and perf-slo; the `caller` and `taxonomy` groups are independent of them and apply to the live tree today (dry run clean) but the live tree is frozen for chain17. Drift refusal re-run: `python3 C/boot-fast-tools/tools/apply-check.py` (needs `/private/tmp/boot-fast/ovref`, built by `tools/restage.sh`).

## 7. Tools (`C/boot-fast-tools/tools/`)
`import-trace.ts` (Bun load plugin: loaded modules with bytes, static and dynamic import lists, fs call census through `fs-shim.ts`), `import-isolate.ts` (own import time per module, in end order), `import-table.py` (markdown tables with groups, self and subtree time), `closure.ts` + `scan-imports.ts` (static closure by regex, because `Bun.Transpiler.scanImports` drops unused imports and rejects a `#!/usr/bin/env repo` first line), `fn-time.ts` (wraps named exports with timers), `bench.py` and `ttfo.py` (interleaved wall and first-byte timing, load guard, SIGINT then SIGKILL), `mkov.py` and `grow.py` (grow an overlay by `ENOENT` from a pristine tree), `suite.py` (a test file in two overlays, differing failures), `loaded-modules.ts`, `esb.ts`, `mutation-check.py`, `apply-check.py`, `restage.sh`. The preload plugins must live at an ASCII path (Bun cannot preload a path with emoji): copy them to `/private/tmp/boot-fast/tools/` as `restage.sh` and the scripts expect.

## 8. Pitfalls recorded
- `i6/ov` is rewritten while integrate2's simulation runs (my first copy was a mid-chain mix: partial catalogue, hot-cache half applied); use the `ckpt-*` directories as a stable base.
- macOS has no `timeout`: a loop `timeout 60 bun ...` printed nothing; use SIGINT-then-SIGKILL wrappers (`tools/run.py`). zsh does not word-split `$L`: use arrays for test lists.
- `bun test` default timeout is 5 s: tests that spawn the build script need an explicit third argument.
- A Bun `onLoad` plugin must return `{contents, loader}` for every module its filter matches, never `undefined`.
- Bun's cpuprofile (8 ms sampling) misattributes a 200 ms process; time functions directly.
- The repo's own birth, phase and build-contract suites refuse plain `bun test` (`testRunnerOnly`); only `repo test` runs them.

## 9. Round 7 follow-up (coordinator GO)

### 9.1 Applied live: group `caller` only
Drift check (dry run) clean, then `python3 C/boot-fast-tools/staged/apply.py --group caller --write`: 2 changes (`B/📥️caller/🟦️.ts` edited atomically by one `os.replace`, new test `B/📥️caller/🧪️tests/🌐️globals/🟦️.ts`), `bun build --no-bundle` of the caller parses. Live checks (read-only, load 33):
- `bun repo run --help --lang=de`: exit 0 in 581 ms, German help (`Führt eine Task eines Projekts aus.`); before: exit 1 `Undeclared original Repo build selection`.
- `--service=off` and `--graph-policy=current` on `run --help`: exit 0 (252 / 191 ms).
- `bun repo --graph-policy=current show projects --json`: NOT completed; admitted by the capacity gate (`show: 1 job, 5120 MB reserved`) and then walked `Inspecting source metadata` (up to 16k+ operations) for the full 60 s, killed by `run.py` (SIGINT, process gone). No `Undeclared original Repo build selection`: the parser and the declared-selection check accepted the flag. The walk is the dependency-source scope walk of the `show` path (the same progress lines as the 16:20 transient), not this fix.
The rest of the set stays staged. Not applied live: `door`, `launcher`, `validator`, `taxonomy`, `slo`.

### 9.2 `door.capabilities`: the cost is not the capabilities, it is the owner registry validation (new group `validator`)
Measured by wrapping the dynamic imports and the functions (`tools/import-timer.ts`, `fn-time.ts`): of the 50-120 ms for `--help`, about 40-70 ms is the import of the caller module (35 modules, loaded later by every command anyway, so moving it saves nothing) and 12-35 ms the Nx operations, watch and Nx bootstrap modules. Making those three imports lazy changes the capabilities output by nothing and the time by 6 ms of noise (`--help` 46 vs 52 ms min, `cache status` 46 vs 38): **not staged**, and no content-addressed memo of the capabilities either: the launch list depends on the environment and the issued authority, and what it spends is module load, not computation (`resolveNxBootstrapLaunchesV1` 0.5 ms).
The real cost sits behind `repo run <project:target>`: `resolveBuildInvocationV1` -> `readSelectedBuildTargetModesV1` -> `readBuildOwnerRegistryV1` validates the 1.3 MB owner registry (3,736 owners, 7,474 paths) with the schema validator on the first call of every process: **620-960 ms** at load 35 (`run a:b` capabilities 484-746 ms min, 611-1,834 ms p50). 95 % of it is one regular expression: the shared relative-path pattern `^(?!/)(?!.*(?:^|/)\.\.(?:/|$))(?!.*\\)(?!.*\u0000).+$` in unicode mode costs 130 us per emoji path (three look-aheads that each scan the path again).
Group `validator` (3 changes): the validator compiles that one pattern through an equivalent single-pass expression (`jsonSchemaSubsetLinearPatterns`, a map from pattern text to its equivalent; every other pattern compiles as before, still in unicode mode). Registry validation 619 -> 25-42 ms; `run a:b` capabilities 484/611 -> 104/142 ms (min/p50). Equivalence evidence: 207,475 generated strings (astral, lone surrogates, line terminators, NUL, `..` segments) with zero differences against the original expression; all 797 unicode-compilable distinct `pattern` values of the repository's schemas against 700 strings each with zero differences (these ran against the dropped non-unicode variant first and found a real difference on `^(\S)$` with an emoji, which is why only the exact-text map is staged); Ajv (independent validator) agrees on 120 strings over 300 patterns; a ratio test (at least twice as fast as the unicode expression); 2 seeded defects killed. A broader `u` -> no-`u` flag switch was tried and refused: it changes `\S`/`.` on astral characters.
Open: 82 other schema patterns carry the same `(?!.*(?:^|/)\.\.(?:/|$))` look-ahead (graph inputs, cargo, vite, caches); add their equivalents to the map as they show up in a profile.

### 9.3 `bun repo` and the door: package.json
tool-direct-manifests (a2f8e1766a8c6cb94) does not touch root package.json, so the row is staged here as group `manifest` (anchored on the post manifest-neutral line, refuses unless the door exists or is applied together; lands in the lock/sync wave). Runtime-neutral form: `"repo": "repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts repo"`; bun form: same with `bun` instead of the first `repo`. The door file must exist first (group `door`); the launcher registry entry is already covered by group `launcher`.

### 9.4 Go launcher against the door (launcher built from the overlay with `repo-launcher-tools/flatten-build.sh`, pinned-bun slot seeded with a local bun in a scratch runtime cache, registry of the overlay naming the door; `tools/launcher-bench.py`, n = 11, load 36, p50 / min ms)
| command | launcher + door | launcher + old script | bun + door, no launcher |
|---|---|---|---|
| `--help` | 47 / 22 (answered by the Go launcher itself) | | |
| `run --help` | 149 / 85 | 516 / 367 | 155 / 75 |
| `leases status` | 210 / 136 | 320 / 204 | |
| `tools status` (refusal, exit 2) | 154 / 75 | | |
| `bun -e 1` floor | | | 63 / 47 |
The launcher adds 0-10 ms over starting the door with bun directly; it ends in the door for every build verb, so with the registry edit `repo run --help` is 3.5x and `repo leases status` 1.5x faster than through the old entry. The launcher itself prints a short help for bare `--help` in 22-47 ms; Go-native catalogue help (open item 3) would make `<cmd> --help` as fast.
Not run: the Go test suites of the launcher (the entry-contract edit is staged only), a launcher with the real bun 1.4.2 (a 1.3.14 binary sat in the pinned slot).

Verification after round 7: `staged/apply.py` on a fresh overlay applies 23 changes (door 10, validator 3, caller 2, launcher 2, manifest 1, taxonomy 2, slo 3), second run 0; `tools/apply-check.py` ALL OK (includes the manifest group and its drift refusal); `tools/mutation-check.py` 23 of 23 seeded defects killed.
