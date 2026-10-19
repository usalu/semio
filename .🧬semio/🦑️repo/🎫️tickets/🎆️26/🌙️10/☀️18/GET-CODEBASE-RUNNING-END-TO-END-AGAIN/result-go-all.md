# Result go-all: every Go module builds, vets, and passes `test -short` and the full suite under the canonical overlay

Root `M` = `🧰️framework/🛍️products/🦑️repo/🔨️modules`, `F` = `🧰️framework/🔨️modules`. Toolchain go1.25.0 darwin/arm64. Rows are top-level tests (pass/fail/skip) with the subtest total in parentheses. Per-module counts are `go test -json` actions, `-count=1`, GOWORK = repo `go.work` (the owner-level server module is outside `go.work`, so `GOWORK=off`).

## Method (kept in `go-all-tools/`)
- `go-all-tools/go.ts`: enumerates every tracked non-ticket `go.mod` (31 `go.work` members plus the owner-level `M/🖥️server/🎛️coordinator`, 32 modules), maps each through `canonicalGoPlan` (the library planner, same as `runCanonicalGoTests`), writes the module overlay, and runs `go build -o /dev/null`, `go vet`, `go test -short`, `go test` (full) with at most 2 modules at once. Resumable (`--skip-done`), per-module result files, `SEMIO_TEST_ARTIFACT_DIR` set to a short temp path (the authority test caps fixture paths at 256 UTF-16 units).
- Safety guard: before and after every step the runner compares tracked `git status` lines and files newer than a marker for `🎫️ticket.json` and `🎯️goals/**` (outside `🗑️temp`/`🗑️generated`/nested `.🧬semio`). It tripped once (see below); otherwise no ticket or goal JSON changed in any run. `git status --short .🧬semio | head -5` before/after: unchanged peer entries only (`D .../CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/*/launch.json`).
- `go-all-tools/table.ts` (summary table), `go-all-tools/fails.ts` (failing tests with output from a `-json` log).

## Root causes fixed
1. Cwd-relative fixture reads under the hermetic sandbox. The sandbox (gofin6) moves the test cwd into an owned temp repository, but several tests still read fixtures through `../`-paths authored for the old per-case cwd. They now go through the sandbox origin:
   - graphql `🧪️tests/🏹️policy` `TestSystemPolicy`: `sandbox.OriginPath(..)` and the authority vector now at its canonical location `🎛️dashboard/🧪️tests/🧫️fixtures/🧭️authority/🔣️.json` (the old `🎛️dashboard/🧫️fixtures/🧭️authority` does not exist).
   - client cli `🧪️tests/🗺️territory-and-system-policy` `TestSystemPolicy`: same path, via `sandbox.OriginPath` (import added).
   - cli `🧪️tests/🛠️tool` `TestDashboardAnalysisScopesAndPolicies`: `sandbox.OriginPath(.., 🎛️dashboard/🧫️fixtures/🏛️repo-action/🔣️.json)` (this fixture family still lives at `🎛️dashboard/🧫️fixtures`, where the TS and Rust dashboard consumers read it).
   - workspace `🧪️tests/🔬️unit` (`TestDotnetFormatterProjects`, `TestIdeConfigurationPaths`): in-package tests cannot import the sandbox (cycle), so the unit case declares a test-only `FixtureOrigin` that the external `🏖️sandbox` case `TestMain` assigns from `sandbox.Origin()`; the reads join it. Authority path corrected as above.
2. cli `🧪️tests/🛠️tool` did not compile (`vet: undefined: bytes`): `TestMcpBootstrapAssetsStayRepoRelative` (uses `runtime.Caller`, which the hermetic scan forbids, and reads checked-in files) had reappeared; it is held by the repository law `🧪️tests/🧱️checked-in-asset-contracts/🟦️.ts` (covers `.codex/config.toml`, `.mcp.json`, devcontainer, native bootstrap scripts), so the Go copy was removed and the `bytes` import added, unused `runtime` dropped.
3. Owner-level module `M/🖥️server/🎛️coordinator` (`github.com/usalu/semio/repo/server`, not in `go.work`) could not be planned: its `🧪️tests` hold four language-agnostic host adapters (`package adapter`) next to `package main` cases, and the planner mapped all of them into one package (`found packages main and adapter`). Package modules at `<owner>/📦️packages/🐹️go` already exclude them by package clause; the library planner (`M/📚️library/🧪️testing/🐹️go-plan/🟦️.ts`) now skips `package adapter` cases everywhere (`GO_HOST_ADAPTER_PACKAGE`; they run through the test host, never `go test`). Language-agnostic vector `hostAdapterCases` added to `M/📚️library/🧪️tests/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json` and asserted in `🧪️tests/🐹️canonical-go-discovery/🟦️.ts` (plan excludes it, the `go test` oracle never runs it). Mutation check: removing the skip fails the test (2 pass / 1 fail), restored (3 pass, 34 expects).
4. The same module had three stale tests from before the fixture/durability moves: `🧬️schema-contract` and `🗄️g3-event-store` read `🧫️fixtures/..` (now `🧪️tests/🧫️fixtures/..`), `🧯️g3-hostile-source` read the deleted `🪟️windows-durability/🐹️.go` (the Windows durability source now lives in `F/📁️filesystem/🛡️durability/📦️packages/🐹️go/🪟️windows-durability`; the test reads it there).

Client/server agreement: `M/💻️client/🔌️mcp` (the MCP response code the fixups edited) builds, vets and passes 27 top-level tests (66 incl. subtests); the repo client (`M/💻️client/⌨️cli`, 565 tests) and the server (`M/🖥️server/🎛️coordinator`, 27 tests, plus `coordinator/📦️packages/🐹️go` and its `📦️main`) are green, and no code outside tests needed to change. Canonical layout untouched (one `🐹️.go` per member dir).

## Files changed
- `M/🏠️workspace/🧪️tests/🔬️unit/🐹️.go`, `M/🏠️workspace/🧪️tests/🏖️sandbox/🐹️.go`
- `M/🔗️graphql/🧪️tests/🏹️policy/🐹️.go`
- `M/💻️client/⌨️cli/🧪️tests/🗺️territory-and-system-policy/🐹️.go`
- `M/⌨️cli/🧪️tests/🛠️tool/🐹️.go`
- `M/🖥️server/🎛️coordinator/🧪️tests/{🧬️schema-contract,🗄️g3-event-store,🧯️g3-hostile-source}/🐹️.go`
- `M/📚️library/🧪️testing/🐹️go-plan/🟦️.ts`, `M/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts`, `M/📚️library/🧪️tests/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json`
- tools: `C/go-all-tools/{go.ts,table.ts,fails.ts}`

## Results after (final full run, label `after`, 32 modules, 0 fail in every step)
Row `M/🖥️server/🎛️coordinator` appears twice: the first is the module `repo/coordinator` at `…/📦️packages/🐹️go` (no tests), the last is the owner-level `repo/server` module.

| module | pkgs | build | vet | short pass/fail/skip | full pass/fail/skip |
|---|---|---|---|---|---|
| F/📁️filesystem/🛡️durability | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/🧾️yaml | 1 | 0 | 0 | 9/0/0 (sub 9/0/0) | 9/0/0 (sub 9/0/0) |
| M/🔎️search | 1 | 0 | 0 | 12/0/0 (sub 12/0/0) | 12/0/0 (sub 12/0/0) |
| M/🪪️identity | 1 | 0 | 0 | 14/0/0 (sub 14/0/0) | 14/0/0 (sub 14/0/0) |
| M/🏠️workspace | 1 | 0 | 0 | 36/0/0 (sub 86/0/0) | 36/0/0 (sub 86/0/0) |
| M/🏠️workspace/🏖️sandbox | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/📐️model | 1 | 0 | 0 | 95/0/0 (sub 771/0/0) | 95/0/0 (sub 771/0/0) |
| M/🗣️languages | 1 | 0 | 0 | 8/0/0 (sub 8/0/0) | 8/0/0 (sub 8/0/0) |
| M/📡️events | 1 | 0 | 0 | 12/0/0 (sub 49/0/0) | 12/0/0 (sub 49/0/0) |
| M/🧩️providers | 1 | 0 | 0 | 21/0/7 (sub 68/0/7) | 28/0/0 (sub 75/0/0) |
| M/📜️statutes | 1 | 0 | 0 | 34/0/0 (sub 113/0/0) | 34/0/0 (sub 113/0/0) |
| M/🗂️codebase | 1 | 0 | 0 | 5/0/0 (sub 16/0/0) | 5/0/0 (sub 16/0/0) |
| M/📊️metrics | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/🚚️move | 1 | 0 | 0 | 17/0/0 (sub 17/0/0) | 17/0/0 (sub 17/0/0) |
| M/🧑️contributors | 1 | 0 | 0 | 6/0/0 (sub 14/0/0) | 6/0/0 (sub 14/0/0) |
| M/🎯️goals | 1 | 0 | 0 | 3/0/0 (sub 3/0/0) | 3/0/0 (sub 3/0/0) |
| M/🎫️tickets | 1 | 0 | 0 | 21/0/1 (sub 32/0/1) | 21/0/1 (sub 32/0/1) |
| M/📝️todos | 1 | 0 | 0 | 13/0/1 (sub 138/0/1) | 14/0/0 (sub 139/0/0) |
| M/🏃️test-runner | 1 | 0 | 0 | 20/0/0 (sub 78/0/0) | 20/0/0 (sub 78/0/0) |
| M/🪝️hooks | 1 | 0 | 0 | 73/0/7 (sub 1683/0/7) | 80/0/0 (sub 1729/0/0) |
| M/🌳️tree | 1 | 0 | 0 | 31/0/3 (sub 171/0/3) | 34/0/0 (sub 176/0/0) |
| M/🧪️test | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/🔗️graphql | 1 | 0 | 0 | 39/0/1 (sub 113/0/1) | 40/0/0 (sub 114/0/0) |
| M/⌨️cli | 1 | 0 | 0 | 106/0/2 (sub 357/0/2) | 107/0/1 (sub 449/0/1) |
| M/⌨️cli/🚀️bin | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/🖥️server/🎛️coordinator | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/🖥️server/🎛️coordinator/📦️main | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/💻️client/🔌️mcp | 1 | 0 | 0 | 27/0/0 (sub 66/0/0) | 27/0/0 (sub 66/0/0) |
| M/💻️client/⌨️cli | 10 | 0 | 0 | 478/0/91 (sub 3624/0/91) | 565/0/4 (sub 4019/0/5) |
| M/💻️client/⌨️cli/📦️main | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/📚️library | 1 | 0 | 0 | 0/0/0 (sub 0/0/0) | 0/0/0 (sub 0/0/0) |
| M/🖥️server/🎛️coordinator | 1 | 0 | 0 | 27/0/0 (sub 76/0/0) | 27/0/0 (sub 76/0/0) |

## Before (first full run, label `before`; 31 `go.work` modules, owner-level server module not yet enumerated)
Everything not listed here was identical to the after table. Failing before:

| module | before | after |
|---|---|---|
| `M/🏠️workspace` | short/full FAIL: 34 pass / 2 fail (`TestDotnetFormatterProjects`, `TestIdeConfigurationPaths`: fixture not found under the sandbox cwd) | 36 / 0 / 0 (86 incl. subtests) |
| `M/🔗️graphql` | short 38/1/1 FAIL, full 39/1/0 FAIL (`TestSystemPolicy`) | short 39/0/1, full 40/0/0 |
| `M/⌨️cli` | build ok, `go vet` FAIL (`undefined: bytes`), tests not compiled (0 run) | vet ok; short 106/0/2, full 107/0/1 (1 skip: needs a built `./cli` binary) |
| `M/💻️client/⌨️cli` | short 477/1/91 FAIL, full 564/1/4 FAIL (`TestSystemPolicy`) | short 478/0/91, full 565/0/4 |
| `M/🖥️server/🎛️coordinator` (owner-level `repo/server`) | build FAIL, vet FAIL, tests FAIL (`found packages main and adapter`); after planner fix 24 pass / 3 fail (stale fixture and durability paths) | short/full 27/0/0 |
| all 27 other modules | green | green, same counts (model 95 top-level / 771 total, hooks 80 / 1729, tree 34 / 176, providers 28, statutes 34, todos 14, workspace sandbox/test/metrics/library/bin/main modules have no Go tests) |

Skips (by design, short vs full differ because `-short` skips the slow suites): providers 7, hooks 7, tree 3, todos 1, graphql 1 in short; tickets 1 (live network), cli 1 (needs built binary), client cli 4 (two dispatcher children that run inside their owner, `TestHookCommandStdinPiped`, live ticket comment) in full.

## Verification
- `bun go.ts --label after --phase after --skip-done --jobs 2` from `C/go-all-tools` until `total=32/32 tripped=no` (each Bash call under 10 minutes; load average 30 to 45 throughout).
- `bun test ./🧪️tests/🐹️canonical-go-discovery/🟦️.ts` (from `M/📚️library`): 3 pass, 0 fail, 34 expects; mutation check described above.
- `gofmt -l` on the touched Go files: clean except `🔗️graphql/🧪️tests/🏹️policy/🐹️.go` and `⌨️cli/🧪️tests/🛠️tool/🐹️.go` (import-block formatting that was already unformatted before; not touched by me beyond the edited lines).
- Safety guard: tripped once mid-run (after the `🗂️codebase` build) with only removals in the watched set (a peer's `🗑️temp` fixture tickets disappearing); I narrowed the watch set to real tickets/goals (excluding `🗑️temp`, `🗑️generated`, nested `.🧬semio`), made it log both directions, and re-ran. No goal or `🎫️ticket.json` was added or modified by any Go run (sandbox guard reports zero resolutions of the real checkout in every module).

## Not done / open
- `go vet -tags exhaustive` and `go test -tags exhaustive` were not part of this task and were not run here (gofin5/gofin6 ran them in a copy).
- `bun repo verify dependencies literal-external` and the nx path (`bun repo run <project>:test`) were not run (coordinator-serialized).
- The 10 `go.mod` files under old tickets (scratch tools such as `gocomptest`, `go-split`) and the hub overlay copies are not repository modules and were not enumerated.
- The python twin of the planner (`gocanon/plan.py`) does not yet skip `package adapter`; it is a ticket tool only.
