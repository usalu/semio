# Result go-all2: planner twin parity, gofmt drift, `-tags exhaustive` on every Go module

Root `M` = `🧰️framework/🛍️products/🦑️repo/🔨️modules`, `F` = `🧰️framework/🔨️modules`. Toolchain go1.25.0 darwin/arm64 (`~/.local/go`, the only provisioned gofmt/go).

## Freeze note
The coordinator's hard freeze (library, package.json, root script, taxonomy) arrived mid-task. Nothing under `M/📚️library/` was edited in this task (the TS planner `adapter` skip and the `hostAdapterCases` vector were already landed by go-all). The Python twin and Go-side files are outside the freeze.

## 1. Planner multi-implementation parity
- Python twin `C/gocanon/plan.py` now skips `package adapter` cases in `🧪️tests/<case>` (exactly where the TS planner does; the package-module adoption path is unchanged in both). It also accepts an optional layout JSON (`plan.py <root> <overlay.json> [layout.json]`) so it can run the fixture layout (`sourceDirectoryNames` etc.) instead of the repo taxonomy.
- New parity test `C/gocanon/parity.test.ts`: materializes the shared language-agnostic vector `M/📚️library/🧪️tests/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json` (main vector incl. `hostAdapterCases`, plus `packageModule`) and runs `canonicalGoPlan` (TS) and `plan.py` over it; asserts identical overlay maps (sources+tests) and identical package lists, and that the adapter case is in neither plan. A second test compares both planners on two real modules (`M/🖥️server/🎛️coordinator`, `M/📚️library`).
- Mutation check: removing the adapter skip from `plan.py` fails both tests (0 pass / 2 fail); restored: 2 pass / 0 fail.
- The twin and its parity test live in the ticket (the only other planner implementation is this ticket tool; no permanent twin exists in the repo).

## 2. gofmt drift
`gofmt -w` on `M/🔗️graphql/🧪️tests/🏹️policy/🐹️.go` and `M/⌨️cli/🧪️tests/🛠️tool/🐹️.go` (import ordering plus the unformatted `TestDashboardAnalysisScopesAndPolicies`). `gofmt -l` on both and on the file below: clean.

## 3. Build tags
Declared tags in tracked/untracked non-ticket Go: `exhaustive` (9 files in `M/⌨️cli`, `M/🌳️tree`, `M/🔗️graphql` tests), `windows` / `!windows` (`F/📁️filesystem/🛡️durability` unix and windows durability), `ignore` (2, never built).
`bun go.ts --label exh --tags exhaustive --steps build,vet,full --jobs 2` (runner extended with `--tags` and `--run`): all 32 modules, build 0, vet 0, full 0 failures. Gains over untagged: tree 46 top-level (was 34), graphql 59 (40), cli 143 (107); everything else identical to go-all (client cli 565/0/4, server 27, hooks 80, model 95, ...). Two interruptions were a peer's transient invalid-taxonomy edit ("Invalid taxonomy schema" planning failures); the 16 errored modules were re-run after the taxonomy validated again.
`GOOS=windows` and `GOOS=linux` build+vet of the durability module: exit 0 both (windows needed one retry for the same transient taxonomy fault).

### Root cause fixed
`M/⌨️cli/🧪️tests/📃️exhaustive-list/🐹️.go` `TestExhaustiveSectionCommands/Rust` was skipped as "known issue with mod-based section format". The test fixture was wrong, not the product: Rust sections are `mod x { // 🔖️Name` ... `} // 🔖️Name` (`RustLanguage` regexes, `FormatSectionStart/End`), and `// #region` is a policy breach for Rust. Fixture now uses the mod form and the skip is removed. Run with `--run TestExhaustiveSectionCommands --tags exhaustive` on the cli module: Rust passes (before removal of the skip with the old fixture: `Section not found: Renamed`).
Remaining skips are by design: tickets live network, cli dispatcher children (run inside their owner), `TestHookCommandStdinPiped`, live ticket comment.

## Hermeticity / safety
`git status --short .🧬semio` before/after every run: no `🎫️ticket.json` or `🎯️goals/**` line changed (diff of before/after shows only peers' new untracked report files). Runner guard never tripped. All exhaustive cases use the sandbox/owned temp repo; no real-checkout writes observed.

## Files changed
- `M/⌨️cli/🧪️tests/📃️exhaustive-list/🐹️.go` (Rust fixture, skip removed)
- `M/🔗️graphql/🧪️tests/🏹️policy/🐹️.go`, `M/⌨️cli/🧪️tests/🛠️tool/🐹️.go` (gofmt only)
- tools: `C/gocanon/plan.py`, `C/gocanon/parity.test.ts`, `C/go-all-tools/go.ts` (`--tags`, `--run`)
- logs under `C/🗑️generated/go-all-logs/` (labels `exh`, `rust`, `xwindows`, `xlinux`)

## Verification
- `cd C/gocanon && bun test ./parity.test.ts`: 2 pass, 0 fail (and mutation fail as above).
- `cd C/go-all-tools && bun go.ts --label exh --phase exh --tags exhaustive --steps build,vet,full --skip-done --jobs 2` until `total=32/32 tripped=no`; table via `bun table.ts exh exh`.
- `bun go.ts --label rust --tags exhaustive --run TestExhaustiveSectionCommands --steps full --only ⌨️cli/📦️packages/🐹️go`: Rust subtest PASS.
- Not run: nx/bazel paths, `bun repo verify dependencies`, the library TS test (frozen; unchanged since go-all).

## Open
- The generated launch/taxonomy churn from peers can make planning fail transiently; re-run with `--skip-done` after deleting errored `res-*.json`.
