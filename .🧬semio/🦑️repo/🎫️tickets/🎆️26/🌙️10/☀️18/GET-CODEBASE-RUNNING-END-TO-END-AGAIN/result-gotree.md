# Result: gotree (Go file splits in repo modules)

Root: `🧰️framework/🛍️products/🦑️repo/🔨️modules/<module>/📦️packages/🐹️go/`. Lossless split by top-level declaration (go/ast), original region markers (`// #region`) kept around each moved group, each new non-test file carries a Header region like `🔌️mcp.go`; test files carry a one-line header like the existing `🔬️_test.go`. Only import blocks adjusted. A throwaway tool (scratchpad, not in repo) did the split; its multiset check confirms every declaration text is identical before and after (tree 252+41, test-runner 222, hooks 196+90, languages 163 decls, 0 mismatches).

## Files (lines before -> after)
- `🌳️tree`: `🐹️.go` 4,789 -> 411 + 🏩️goal-render 361, 🔨️build 639, 🔍️search 402, 🖨️render 175, 🎊️query-cache 599, 📌️cache 375, 🔌️ports 107, 💿️records 471, 🌿️nodes 503, 🎯️goals 322, 📜️statutes 248, ⏲️mermaid 397. `🔬️_test.go` 2,004 -> 294 + 🧫️filter_test 515, 🔦️search_test 485, 🖼️render_test 304, 🪞️identity_test 444. `🔭️exhaustive_test.go` (708, build tag) untouched.
- `🏃️test-runner`: `🐹️.go` 3,957 -> 277 + 🌲️snapshot 356, 🗺️planning 480, ⚙️execution 196, 📊️parsing 449, 🔎️resolution 515, 🕸️test-command 574, 📖️extract 282, 📰️generate 516, 🖲️command-files 456.
- `🪝️hooks`: `🐹️.go` 3,384 -> 221 + 🧭️payload 395, 🛡️blocking 249, 🔶️dispatch 533, 📓️session 228, 🔖️micro-commit 190, 🧪️test-input 453, 🎫️checkpoint 249, 🚦️run 516, 🧬️input 547. `🔬️_test.go` 3,903 -> 163 + 🧬️input_test 433, 🛡️blocking_test 267, 🔶️run_test 410, 📓️logging_test 682, 🎫️track_test 457, 🧮️resolve_test 368, 🧪️test-input_test 232, 🧷️populate_test 608, 🔖️checkpoint_test 408.
- `🗣️languages`: `🐹️.go` 3,395 -> 239 + 🗣️parsing 461, 🎽️base 503, 🪶️comments 367, 🎶️typescript 314, 🎚️go 159, 🐍️python 83, 📜️csharp 84, ⏳️rust 174, 🪅️ruby 121, 📭️data-languages 272, 📇️registry 57, 🧾️analysis 86, 📝️sections 193, 🧩️json-sections 505. Package doc comment kept attached to `package languages` in `🐹️.go`.
Every file < 1,000 lines (max 682).

## Test counts (`func Test*`)
tree 32+12 -> 44 (7+9+4+8+4 in split files + 12 exhaustive); hooks 82 -> 82; test-runner 24 (unchanged file); languages 4+3 (unchanged).

## Commands (each in the package dir)
Before and after: `go build ./`, `go vet ./`, `go test -count=1 -v ./` (+ `go vet -tags exhaustive ./` for tree), `gofmt -l .` (clean).
- languages: 7 PASS before / 7 PASS after.
- test-runner: 19 PASS / 0 FAIL before and after.
- hooks: 79 PASS / 0 FAIL before and after.
- tree: 29 PASS / 3 FAIL before and after, identical failures (pre-existing, not caused by the split): `TestComputeCompositeFingerprintFallsBackWhenRecursiveSubmoduleStatusFails`, `TestG1ReindexCancellationPreservesLastValidIndex`, `TestG1ReindexMaximumPlusOnePreservesLastValidIndex` (the G1 ones fail on a missing fixture `../../../📚️library/🧪️tests/1️⃣g1-contract/🧫️fixtures/🔣️.json`).

## Text readers
None found: no `//go:embed` in the four packages, no reference to the split file names in the repo product tree (`project.json` / `📜️script.ts` do not name them); the Rust twin of languages reads the table JSON, not the Go file. No taxonomy `fileKinds` listing for `.go` was found under the framework tree; naming follows the existing `⌨️cli` precedent (`🔌️mcp.go`, `🖨️render.go`).

## Open
- Tree's 3 pre-existing failures (missing G1 fixture / submodule-status fallback) are untouched.
- Dependents of these packages were not rebuilt (public API unchanged).
