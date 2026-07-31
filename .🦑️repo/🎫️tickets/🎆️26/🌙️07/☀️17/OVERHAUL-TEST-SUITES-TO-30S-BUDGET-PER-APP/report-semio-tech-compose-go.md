# @semio-tech/compose-go — test suite overhaul report

## Status: SKIPPED — pre-existing compile failure unrelated to test content

`go build ./...` in `compose/client/lib/go` fails with:

```
# github.com/usalu/semio/go/compose
./main.go:17733:1: syntax error: non-declaration statement outside function body
./main.go:17735:1: syntax error: imports must appear before other declarations
```

`main.go` (18128 lines) contains **two** `package compose` declarations (line 13 and line 17733), the
second followed by its own `import (...)` block, in the middle of the file (around a
`// #region kit_graph` marker: "Kit graph session (TypeScript parity: commitKitGraphChange, backbone,
transactions, history)"). This looks like another concurrent session mid-appending a new
`kit_graph`/backbone module's content directly onto the end of `main.go` without finishing the merge
(duplicate package/import header left in place).

This is unrelated to the test-suite-trimming task (test file is `main_test.go`, untouched by the
duplicate block) and is production source code owned by another in-progress change. Per instructions,
I did not attempt to fix it and did not modify `main.go`.

## What was done
- Read `compose/client/lib/go/script.ts` and `project.json`: `test` target runs
  `bun ./📜️script.ts test` -> `runCmd("go", ["test", "-v", "./..."], { cwd: this.root })`
  (not yet routed through `runTestBudgeted`/`runCargoTestBudgeted` — moot until the file compiles).
- Read `main_test.go` (3763 lines, 27 top-level `func Test*`) to survey scope; did not classify/delete
  tests since the package doesn't currently build (can't validate that deletions are safe or that the
  remaining suite would pass).
- No files edited.

## Next steps (for whoever owns the concurrent `kit_graph` change, or a re-run of this ticket)
1. Finish/fix the merge in `main.go` so there is a single `package compose` + single `import` block.
2. Re-run this ticket unit once `go build ./...` succeeds, to baseline `go test ./...` timing and
   classify/trim `main_test.go`'s 27 test functions, then wire `script.ts`'s `TestScript` through
   `runTestBudgeted`/`runCargoTestBudgeted`-equivalent (Go: wrap `runCmd("go", ["test", ...])` with
   `runTestBudgeted` from `../../../../repo/lib/js/index.ts`).
