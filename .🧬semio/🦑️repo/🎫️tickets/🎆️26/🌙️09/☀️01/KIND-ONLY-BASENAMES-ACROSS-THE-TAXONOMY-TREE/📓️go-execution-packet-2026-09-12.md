# Go Implementation Leaves and Compiler Input Mapping

## Observed Constraints

The broader census identifies 26 named Go source files in repo CLI, MCP, and coordinator owners. The schema already documents Go's import-path restriction through `go-internal-root`, `go-internal-package`, and `repo-command-entry`: imported package identities use compiler-accepted ASCII path segments. Those paths can retain semantic ASCII domain identities while their implementation leaves become anonymous `🐹️.go` files.

The MCP owner has multiple same-package implementation concerns: entry point, server, protocol, transport, repository, and event. The coordinator also has same-package repository/event-store/durability concerns and separate Windows/Unix durability implementations, selected by existing `//go:build` constraints. A simple move into subdirectories changes the Go package boundary; concatenating all concerns would erase the domain tree rather than solve that constraint.

## Existing Mechanism to Extend

Repo-lib already has `canonicalGoTestPlan` and `runCanonicalGoTests` in `📚️library/📦️packages/🟦️typescript/🟦️.ts`. The planner maps anonymous Go test leaves under semantic test directories to virtual `_test.go` paths through the standard Go `-overlay` option. It derives stable virtual identities from the source path and checks collisions with authored files. The test runner passes that overlay to the Go toolchain.

An execution lane should evaluate extending that compiler-input mapping to declared anonymous source leaves, preserving same-package ownership without changing runtime APIs. The domain tree can then contain `protocol/🐹️.go`, `transport/🐹️.go`, and target-specific durability leaves while the Go compiler sees a virtual flat package. Existing build constraints must remain effective in the virtual source inputs. ASCII internal Go package identities should continue to resolve normally.

This is a proposed build-tool projection, not verified implementation. The executor must design schema-backed source ownership, write a portable fixture, demonstrate a failing test, and validate build/test behavior with the actual Go toolchain on the native platform plus Windows cross-compilation. Source selection must not recursively absorb independent nested packages or test fixtures. Build, test, and development entry points must consume the same declared source plan through existing `📜️script.ts` routers and Nx targets.

No source migration script, runtime adapter, or compatibility reader is needed. Any overlay manifests are generated outputs and must remain within the ticket during validation. Existing Go tests and MCP protocol behavior should provide regression coverage after the source moves.
