# Brief: Go Split Executor

Also read `brief-executor.md` (hard rules). Go is not behind the cargo ingress gate: you MAY run `go build`, `go vet` and `go test` for YOUR package only (`cd` into the package dir; root has `go.work`). Run them in the foreground; keep test runs scoped (`go test -run <pattern>` or the package only, `-count=1`), never `./...` from the repo root.

## Rules
- Goal: every file in your package under 1,000 lines (target ≤ 600), one file per semantic component. In Go a package is a directory: split by moving top-level declarations (types with their methods, function groups) into sibling files in the SAME directory, named like the existing siblings (emoji-prefixed, e.g. `🔌️mcp.go`, `🔬️_test.go`): `<emoji><component>.go`, tests `<emoji><component>_test.go`. Pick a fitting unique emoji per file.
- Lossless: move declarations verbatim with their doc comments; adjust only `import` blocks (run `gofmt -l`/`goimports`-equivalent manually: `go build` tells you unused/missing imports). No behaviour change, no renames, no new abstractions.
- Test files: keep every `Test*`/`Benchmark*`/`Fuzz*` name identical; count before/after must match (`/usr/bin/grep -c '^func Test'`).
- Text readers: find anything that reads your file by path as text (scripts, sha pins, taxonomy fixtures, `project.json` inputs, embed directives `//go:embed`) and re-point it.
- If the repo's taxonomy has a schema listing allowed Go file names (search `taxonomy` / `fileKinds` for `.go`), conform to it or extend it consistently.
- Verify: `go build ./` + `go vet ./` + `go test -count=1 ./` in your package dir (or a fast subset if the full test run exceeds ~10 minutes; record what you ran) before and after; compare.
Report `C/result-<prefix>.md`: per file before/after lines, new files, test counts, commands + outcomes.
