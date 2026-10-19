# Result gofix7: repo MCP Go module (`repo-mcp`) builds, vets and tests green

Module: `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp` (`github.com/usalu/semio/repo/mcp`, go 1.25, in `go.work`). Go toolchain: `/Users/ueli/.local/go/bin/go` go1.25.0 darwin/arm64 (matches `go 1.25`).

## Root cause
`🧪️tests/🛡️hermetic-root-guard/🐹️.go` was the pre-gofin6 design and wrong, not the other test:
- it imported `github.com/usalu/semio/repo/client`, a module this module neither requires nor may depend on (the client duplicates the module stack and is being removed), so the whole test package failed to build;
- it forbade every `SetRootDir(`/`GetRootDir(` spelling, i.e. it forbade a test from binding a root it owns. The hermetic design (gofin5/gofin6, `workspace/🏖️sandbox`) is the opposite: the test binary runs in a sandbox repository it owns (`sandbox.Run`), and only a resolution or binding that lands inside the real checkout is a violation. `TestRepositoryReopenPromptContract` binding a private temp repo is legitimate under that design.
No exemption by name: the guard now has the same shape as the 15 other modules.

## Changes
- `…/💻️client/🔌️mcp/🧪️tests/🛡️hermetic-root-guard/🐹️.go`: rewritten. `TestMain` = `sandbox.Run("mcp", nil, m.Run)`; `TestTestsRunAgainstAnOwnedRoot` = `sandbox.Problems("🛡️hermetic-root-guard")` (bound root, cwd, and a scan of all other case sources for `os.Getwd(`, `runtime.Caller(`, `FindRepoRoot(".")`, `FindRepoRoot("")`).
- `…/🧪️tests/🤝️protocol-contract/🐹️.go`: the sandbox moves the cwd, so cwd-relative reads now go through `sandbox.OriginPath` (`repositoryContractFixture`, `🧩️component/🐹️.go`, `🚚️transport/🐹️.go`); `TestRepositoryReopenPromptContract` binds its private repository with `sandbox.BindRoot` (restores on cleanup) and compares canonical paths; import of `workspace/sandbox` added.
- `…/💻️client/🔌️mcp/go.mod`: `require` + `replace` for `github.com/usalu/semio/repo/workspace/sandbox` (same as the other modules).
- Canonical layout untouched (one `🐹️.go` per member dir; the guard stays `package main` like its siblings). No other files edited (the `fixups` executor's response-code files were not touched).
- Tools (kept): `gofix7-tools/go.ts` (canonical Go overlay + `go <verb>` for this module without nx), `gofix7-tools/snap.sh` (tracked `.🧬semio` status + goal/ticket mtime snapshot).

## Verification (all foreground, real checkout, hermetic after the fix)
Commands run from `C/gofix7-tools` (`bun go.ts <go verb> …` = canonical overlay from `canonicalGoPlan`/`writeCanonicalGoWorkspaceOverlay`, `GOWORK=<root>/go.work`; the module's package is `.` because the layout is taxonomy-named, so `./...` is not valid there; `.` is the only package the plan yields):
- Before the fix: `bun go.ts vet` -> `no required module provides package github.com/usalu/semio/repo/client` (the reported failure).
- `bun go.ts build -o /dev/null` -> exit 0.
- `bun go.ts vet` -> exit 0, no output.
- `bun go.ts test -short -count=1 -v` -> `ok github.com/usalu/semio/repo/mcp 0.356s`; 26 top-level tests PASS, 0 FAIL, 0 SKIP (56 PASS lines incl. subtests).
- `bun go.ts test -count=1 -v` (full, non-short) -> `ok … 0.227s`; 26 top-level PASS, 0 FAIL, 0 SKIP (56 PASS lines incl. subtests). Includes `TestTestsRunAgainstAnOwnedRoot`, `TestRepositoryReopenPromptContract`, `TestToolResultShapeContract` (9 vectors).
- The exact body of the `repo-mcp:test` target, `cd …/💻️client/🔌️mcp && SEMIO_SCRIPT_PROCESS_INVOCATION=<envelope 540 s, owner gofix7> bun ./📜️script.ts test` -> exit 0, `ok github.com/usalu/semio/repo/mcp 0.335s`, stages loading/running/complete.
- Mutation check (temporary test `TestDebugMutationLeaksTheCheckout` calling `workspace.SetRootDir(sandbox.Origin())`, removed again): `go test -run 'TestDebugMutation|TestTestsRunAgainstAnOwnedRoot'` -> exit 1 with `resolved /Users/ueli/Documents/semio via TestDebugMutationLeaksTheCheckout` and `workspace root is bound to the real repository root`. The guard bites.
- `gofmt -l` on the new guard file: clean.
- Checkout safety: `snap.sh before/after/final`. The tracked `.🧬semio` status differs only by peer files in other areas (`vit2-tools/runvt.py`, `tst3a-tools/r.py` gained `M`) and files modified in `…/☀️02/TEACHING-ARCHITECTURE-DEPLOY-READY/**` (peer builder/release docs and script). No goal file and no ticket file was touched by the Go runs (the sandbox guard also reported zero resolutions of the real root in every run).

## Not done / open
- `NX_FORCE_REUSE_CACHED_GRAPH=true bun repo run repo-mcp:test` was started once and aborted by my own 570 s alarm while still queued at `repository:build running` (load average about 75); not a Go failure. Its script body passes when run directly (above). `bun repo run repo-mcp:build` (writes the cached binary, coordinator) was not run; `go build` of the module is green.
- Tests that need the TS toolchain (dispatcher) do not exist in this module, so nothing is skipped.
