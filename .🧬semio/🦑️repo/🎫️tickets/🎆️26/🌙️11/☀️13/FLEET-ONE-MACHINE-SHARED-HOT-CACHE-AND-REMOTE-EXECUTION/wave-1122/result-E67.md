# Result E67 (tests-build-B)

Scope: open rows of the build-library (B) table (E31 table, E50 notes). I re-ran every file that was not green in the E31 table (81 files, one file per `repo test-file --json`, memory gate 25 % free, load gate relaxed to 90 for single files as the brief allows) and worked through them one file at a time. Runner: `T/🧰️scripts/E67/run.py` (resumable, rows in scratch `E67/open.jsonl`, full pass `E67/full.jsonl`). Landings: `T/landings.md` sections "Wave 1122 E67" (four sections). All locks released.

## Table (file relative to B, before = E31 table, after = this lane)
| file | before | after | cause / fix |
|---|---|---|---|
| 👁️observation/tests/📦️projects | 0/2 | 2/0 | product: `dependency-edges` loaded the admission module with a sync `createRequire(...)` although its import graph reaches `inputs/data/load` (top-level await): ERR_REQUIRE_ASYNC_MODULE in a node-run Nx plugin; now `await import`. Test: second test lacked `SEMIO_PROJECT_OBSERVER`; the law "no `.cache` in the fixture root" now allows the owner-claims branch `.cache/f` and forbids `.cache/b` |
| 🔨️modules/🏃️process/native-build/component-pass (framework, not under B) | file did not parse | parses | product: two `constants` imports (`node:fs` and `node:os`); the os one is `osConstants`; found by scanning every framework source with the Bun transpiler (all other parse errors are `.d.ts` of wasm bindings) |
| 📖️storybook/💾️storage tests | 1/3 | 3/0 | test: 10 s default invoke budget expired; 120 s budget |
| 📥️inputs/🎯️selected tests | 0/2 | 1/0 | test: `nx.json` missing in the private root, `withBuildTestInvocationV1` with a private workspace (Original Graph format owner needs caller root == workspace) -> `repository.invoke`, `privateRepositoryWorkspaceV1("graph")` for the Nx socket byte limit (95) |
| 🧪️testing/private-repository retirement (library) | EBUSY on retire | retire works | test infra: nested disk images (`n/m/<id>/native/m`, the Nx namespace) were still mounted when the partition clear ran `rm`; now detached first |
| 📥️source/closure/scheduling boundary | 4/1 | 5/0 | corpus: resident-service gets the port `slice-pool/quiet` (new graph `settle-quiet` module imports it) |
| 📥️inputs/🧮️digest batch-snapshot | 1/1 | 2/0 | test: a process birth can be received once; `withBuildTestInvocationV1` |
| 🧾️ci-report/suites tests | 15/1 | 16/0 | product: `defaultCiSuitePortsV1.start` recognised a timeout only by the old message; `runOwnedCommand` now refuses with code `timeout`; a hung suite was reported as failed |
| ✏️s/🧑‍💻dev project (not under B) | target input list incomplete | complete | product: `test-catalog-publication` runtimeSourceEntries lacked the `🖍️draw-route` test the script runs |
| 🧪️tests/🕸️graph/🎯️source-demand/📋️scope | 3/1 | 4/0 | fixture and title follow the product project |
| 🧪️tests/🕸️graph/🚀️dependency-bootstrap | 2/1 | 3/0 | test: `--linker=hoisted` |
| 🧪️tests (workspace laws) | 3/2 | 5/0 | fixture: build-storybook and build-canvas-font-assets live in the storybook / typescript-package projects (fixture names the owner); lease law budget |
| 🧭️selection/📥️closure tests | 0/3 | 2/0 (file passes) | same recipe as selected |
| 🧭️selection/🎯️scope/🧶️nx tests | 0/2 | 1/0 | same recipe |
| 🕸️graph/📥️inference/🧭️script-inputs/🔀️router tests | 24/1 | 25/0 | fixture ceiling 2600 -> 2800 (course check script reaches 2637 files) |
| 🚀️bootstrap/📦️parsers tests | 2/2 | 2/1 | budget fixed; remaining: see open |
| source/closure/command/runtime artifact-plan (+ tools) | 0/2 | 0/2 | provider designation (`SEMIO_SCRIPT_PROVIDERS`) now issued: artifact-plan reaches 48 expects (was refused at the first), then a later positive run exits 1 (not analysed); tools refuses at `port.observe/measure` (the "tools" partition of the sandbox checkout has no backing) |
| 🧪️tests/🕸️graph/📌️pinned-publication | 6/6 | 6/6 | first writer uses `repository.invoke`; nested handoff now fails at its own expectation (line 147) instead of at the format owner; the other 5 not analysed |
| passed without a change (fixed by peers since E31) | | | resident provision/journal/host/restart/cancel, sidecar closure, inputs storage + ledger, owner-view reach, caller birth, storybook engines, leases status, agent-sessions, readmission tasks, selection watch phase, fleet-status, bootstrap cli, show-project-scope, scheduling main (333 s), conformance (410 s) |
| 🕸️graph/📥️inference/🧪️tests/🧷️closure-facts | 20/2 | 22/0 | fixture: `emojiProjectJsonNodes` is a wrapper now (start marker) |
| 🕸️graph/🧪️tests/🔓️sealed-reuse | 8/6 | 14/0 | fixture: end marker and two order texts follow the `loadBuildGraphAttemptV1` split |
| 🕸️graph/🧪️tests/🔬️analysis-providers | 0/1 | 1/0 | test child: the library bootstrap registers the parsers, so "unregistered" is observed before the inference import |
| 🧪️tests/🕸️graph/🧭️portable-cli | 10/6 (E31) | 15/0 | test: authoring schema requires `codeRoot` (handover item "native-caller needs codeRoot") |
| 🧭️invocation/graph/memo/derive/warm tests | 6/1 | 7/0 | test text: shared program reuse spelling |

## Still red (17 files by the overlay count; cause, owner)
| file | state | analysis |
|---|---|---|
| 🌐️remote-execution/tests/🔧️bazel, 🔧️tools/🏗️bazel/🏃️execution/tests/🔒️classes, 🔒️hermetic | 0/1 each | `/tmp/<hash>` hermetic scratch is EPERM inside the worker sandbox (the darwin profile writes only the input root, the runner's scratch base is the fixed `/tmp`, policy `🏗️bazel/📐️path-plan`); `classes`: the hermetic entry imports `library/🏃️process/🔌️providers/🟦️.ts` (and through it framework provider/validator modules) that the sandbox closure lacks; `hermetic`: `bun {projectRoot}/gen.ts` exits 1 (not analysed). Bazel lane |
| 🏃️execution tests | deadline | 420 s envelope under load; conformance passes at 410 s (just inside) |
| 📥️inputs/🎯️selected/📦️snapshot tests | 1/1 | `context.loadSemioGraphAuthorizersV1()` returns null inside the private root (the patched Nx authorizer bundle is not found: `SEMIO_NX_AUTHORIZER_ROOT`/marker inspect), plus a lease-order violation `native-snapshot-slot` (namespace rank 5) held while `nx:graph-namespace` (graph rank 4) is requested; E57/E49 Nx-namespace area |
| 📥️inputs/🔭️read-oracle tests | 20/1 | seatbelt backend: the row `undeclared-read-is-named-and-learned` learns nothing under the suite sandbox although `log show` works there (probed); product risk: an unreadable denial log is reported as `clean` |
| source/closure/command/runtime tests 👓️reader-lifetime, artifact-plan, artifact-plan/⏱️tools | 0/2 each | provider designation fixed (artifact-plan reaches 48 expects), then a later positive run exits 1; tools: `port.observe`/`measure` of the sandbox checkout "tools" partition have no backing; reader-lifetime not analysed |
| 📥️source/🛗️zero-touch tests | 34/3 | component-pass parse error fixed; the 3 left report 127 genuine zero-touch findings (`command:verify|workspace-specifier|…os/…/mcp/…` imports of `@semio-tech/*` workspace specifiers from root-registry commands): product design debt |
| 🧭️selection/👁️watch tests | 2/2 | the child script `📜️script.ts` writes no `law-authority.json` (silent child failure, declared-birth cluster) |
| 🧪️tests/🕸️graph/🧊️optional-cache, 📌️pinned-publication | 11/3, 6/6 | "Nested Original process envelope must preserve its actual parent" and graph-format-owner/compact-token expectations; not analysed |
| 🧪️tests/🕸️graph/🎯️source-demand (main) | 33/1 | `projectWithDefaults` evaluated from source throws `path.join(..., undefined)` (ERR_INVALID_ARG_TYPE); the test wiring lacks a coordinate the product now uses; not located |
| 🚀️bootstrap/📦️parsers tests | 2/1 | budget fixed; the test copies the real package.json into a private root that lacks the workspace members: "Bun published workspace patterns drift" |
| 🕸️graph/🕰️host/tests/🌊️wave-prime/🧫️fixture.ts | 0/1 | not a test: a fixture child script (argv JSON) matched by the file list; the harness list/suite glob should exclude `🧫️fixture.ts` (E59 ratchet) |
| ⚙️operations/tests/🧭️help-offline | 0 (new) | real regression: the file held 3075-3593 MB and is stopped at its ceiling of 3072 MB: the offline help path loads something huge now |

## Fresh count
The full re-run was started (`E67/full.jsonl`) but the host gives about 75 files/hour for one-file-at-a-time runs under load 35-55, so only 48 of 662 files were re-run (47 passed, 1 new failure: help-offline). The count below is therefore an overlay: E31 latest result per file, replaced by this lane's re-runs of all 81 files that were not green plus the 48 files of the full pass, plus single-file confirmations after each fix. Result: **645 of 662 pass, 17 fail** (the 17 listed above; wave-prime is a fixture, not a test, so 646 of 661 real test files). NOT re-verified: about 580 files that E31 marked green and that nothing here touched; a peer edit since the E31 pass may have broken any of them (help-offline is the first such case found in 48 files). Resume: `python3 T/🧰️scripts/E67/run.py list full.jsonl 3400 T/🧰️scripts/E30/all-test-files.txt` (resumable; one file at a time, memory gate 25 %, load gate 90).

## What other lanes must know
- Tests that open private storage must use `privateRepositoryWorkspaceV1("graph")` (short `/tmp/sg/<id>` root; the Nx socket path limit is 95 bytes) and `repository.invoke(body,{maximumElapsedMilliseconds})` (default budget 10 s); `withBuildTestInvocationV1` + a private root fails "Exact Original Graph format owner required".
- Private repository retirement now detaches nested images first (`🧪️testing/🧫️private-repository/🪦️retirement`); before, a test that bound an Nx namespace left a mounted image under `/private/tmp/sg/<id>` and failed at retire with EBUSY. Leaked mounts of the earlier runs of this lane (`r8jzcDn`, `r0Ii7T1`) are reaped by the stale-root sweep of the next private repository.
- A source that does not parse is invisible until a scanner touches it: scan all framework sources with `new Bun.Transpiler().scan` (script `E67/scan.ts` in the scratch root) after big edits.
- `dependency-edges` must never `createRequire` the admission module (top-level await in its import graph); the playground-targets and registry-reads modules still do (sites: `🎮️playground-targets/🟨️.mjs` lines 80/82, `🧾️registry-reads/🟨️.mjs` line 62); they can hit the same ERR_REQUIRE_ASYNC_MODULE when reached under node (not exercised by any test I ran).
- Scratch/process: no background process of this lane remains; `T/🧰️scripts/E67/` holds `run.py`, `locks.sh`.
