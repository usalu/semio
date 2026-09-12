# Go Test Dispatch Follow-up — 2026-09-12

## Scope and result

The four reachable Go branches behind the repo client's `test` command now use the same canonical compiler-input projection as package build, test, and development routes. Bundle, file, section, and definition selections no longer invoke `go test ./...` or semantic descendant package patterns. One registered Bun command owns dispatch, and its Nx target and launch entry expose the focused portable acceptance test.

No ticket or goal lifecycle state was changed. No Git modifying command, compatibility wrapper, migration script, runtime dependency, or `.devcontainer` hook was added.

## Reachability

- The native client root registers `testCommand` with `root.AddCommand(testCommand(...))`.
- `testCommand.RunE` resolves requested scopes, then calls `runTestScope`.
- `runTestScope` reaches `runBundleTests`, `runFileTests`, `runSectionTests`, and `runDefinitionTest` for the four narrowing levels.
- `runAllTests` and `runTechnologyTests` also reach `runBundleTests`, so they inherit the same projection.
- The CLI MCP server registers `test` as an MCP prompt through `mcp.NewPrompt("test", ..., handleTestPrompt)`. It does not register a test execution tool, so no MCP RPC branch bypasses or duplicates the repaired dispatcher.
- The workspace launch gate `⚡️nx test` reaches package tests through Nx. The focused `⚖️gate🐹️test-dispatch` launch entry reaches `@semio-tech/repo-lib:test-go-dispatch` directly.

## Implementation

The repo library's existing canonical Go planner remains the sole overlay implementation. It now includes conventional package directories containing direct Go leaves as well as owners projected from semantic source and test directories. Nested modules and opaque fixtures/packages remain excluded. For the live CLI module the resulting declared set is:

```text
.
./cmd/repo
./internal/command
./internal/eventstore
./internal/glob
./internal/graphql
./internal/humanize
./internal/id
./internal/ignore
./internal/mcp
./internal/mcpserver
./internal/search
./internal/templatefunc
./internal/yaml
```

No selected package contains an emoji semantic descendant. The neutral planner fixture now includes `./internal/direct` to prove that a true conventional package is retained.

The existing repo-library `📜️script.ts` owns the permanent `go-test <module-root> <input-path|-> [go-test-args...]` executable route. A selected semantic input is matched to its virtual overlay input and then reduced to its real owning package. `-` selects the plan's complete declared package set. The package `🟦️.ts` only adds package enumeration and an optional validated package set to the existing planner/runner; selection and executable routing stay in `📜️script.ts`.

The Go CLI delegates every Go scope to that one command. Test filters remain native `-run` arguments. File, section, and definition scopes select the source/test input's owner package. Bundle scopes select the complete plan. Child stdout and stderr still stream to the command writers and the existing `Running:` progress line remains. `runExternalCommand` now uses `exec.CommandContext(cmd.Context(), ...)`, so caller cancellation reaches the Bun dispatcher instead of leaving the child detached.

Section parsing now calls the existing `extractEntityEmoji` grapheme helper and trims the remainder. This consumes the whole leading emoji sequence, including U+FE0F, while leaving `Flat` unchanged.

## Portable fixture and native oracle

Contract: `canonical-go-test-dispatch-v1`.

The language-neutral JSON vector materializes:

- a root owner with only `🧩️component/🐹️.go` source and `🧪️tests/🧮️addition/🐹️.go` tests;
- a local `go.mod replace` dependency with only `⚡️effects/🐹️.go` source;
- a bundle expectation that both tests execute;
- a definition expectation that only `TestSelectedDefinition` executes.

The independent native negative oracle runs direct `go test ./...` against the materialized module and requires Go's `malformed import path` rejection for semantic descendant discovery. The subject then calls the actual `runTestScope` dispatcher for bundle and definition scopes. Marker files prove the exact executed cases, and captured stdout proves progress/output streaming plus the exact definition filter.

The initial red run failed before a test executed:

```text
Running: go test ./... (in <temporary-module>)
malformed import path "example.com/semio/dispatch-fixture/🧪️tests/🧮️addition": invalid char '🧪'
malformed import path "example.com/semio/dispatch-fixture/🧮️addition": invalid char '🧮'
FAIL
```

That was the old `runBundleTests` behavior. The final fixture uses registered semantic members `🧩️component` and `⚡️effects`; its independent direct-native oracle reproduces the same recursive-discovery rejection, while the actual dispatcher succeeds.

## Validation

- **PASS** — registered Nx/launch route:
  `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/repo-lib:test-go-dispatch --skipNxCache`
  - `1 pass, 0 fail`
  - `TestCanonicalGoTestDispatcher` passed.
  - Nx reported `Successfully ran target test-go-dispatch for project @semio-tech/repo-lib` with cache skipped.
- **PASS** — direct focused dispatcher route:
  `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts test go-test-dispatch`
  - `1 pass, 0 fail`; the outer canonical CLI run compiled every declared CLI package and the actual inner bundle/definition dispatch completed.
- **PASS** — canonical planner plus native Go oracle:
  `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts test go-input-projection`
  - packages `[".", "./internal/direct", "./internal/text"]`; `1 pass, 0 fail`.
- **PASS** — VS16 section selection:
  `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts test -run '^TestCollectGoTestsInSection$' -count=1`.
- **PASS** — direct-package compiler-plan census: live CLI plan reported 14 packages, 9 replacements, and zero non-ASCII package selections.
- **PASS** — no-test native compile through the dispatcher for MCP, coordinator, and repo Go library:
  `bun <repo-lib-script> go-test <module> - -run '^$' -count=1` for each module.
- **PASS** — Windows amd64 CLI cross-build through `runCanonicalGoBuild` with `GOOS=windows GOARCH=amd64 GOWORK=off`.
- **PASS** — JSON parse for both new fixture/schema documents and both extended canonical-discovery documents.
- **PASS** — `git diff --check` on the shared modified integration files.
- **PASS** — source audit found the four repaired branches calling `runCanonicalGoTestDispatch`; none retains `./...`.

A broad TypeScript compiler probe was not used as acceptance. It exited 2 with 1,708 diagnostics in the concurrently modified workspace. The first exact diagnostic was:

```text
.storybook/scopes.ts(21,39): error TS6059: File '.../🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts' is not under 'rootDir' '.../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library'.
```

No baseline claim is made for that broad compiler result. The requested bounded Bun, Nx, Go, section, package-selection, local-replacement, and Windows checks all passed. No broad CLI suite was run.

## Direct native-tool limitation

Direct `go test ./...` remains invalid for a module whose source/test domains are semantic directories because Go interprets those directories as import paths. Routine repository build/test/dev routes and the native repo client dispatcher use the canonical plan. Direct native callers must supply the generated overlay and declared package set; the retained fixture intentionally verifies that an unprojected recursive call fails.

## Exact changed-file list

| Repository-relative path | Absolute path |
| --- | --- |
| `.vscode/launch.json` | `/Users/ueli/Documents/semio/.vscode/launch.json` |
| `.vscode/🧩️launch.seed.jsonc` | `/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component/🐹️.go` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component/🐹️.go` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧬️schema/🚦️test-dispatch/🔣️.json` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧬️schema/🚦️test-dispatch/🔣️.json` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🚦️test-dispatch/🔣️.json` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧫️fixtures/🚦️test-dispatch/🔣️.json` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🚦️test-dispatch/🐹️.go` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🚦️test-dispatch/🐹️.go` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🐹️canonical-go-discovery/🔣️.json` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🐹️canonical-go-discovery/🔣️.json` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚦️test-dispatch/🟦️.ts` | `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚦️test-dispatch/🟦️.ts` |
| `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-go-test-dispatch-2026-09-12.md` | `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-go-test-dispatch-2026-09-12.md` |

The executable used for the Windows cross-build was temporary ticket output and was deleted with `🗑️generated/sol-go-test-dispatch` after its successful result was recorded here.

