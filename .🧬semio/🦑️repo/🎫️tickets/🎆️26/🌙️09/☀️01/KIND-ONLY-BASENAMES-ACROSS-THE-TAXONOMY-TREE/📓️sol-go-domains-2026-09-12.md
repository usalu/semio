# Go Domain Source Taxonomy Execution

Date: 2026-09-12  
Workspace root: `/Users/ueli/Documents/semio`  
Lane: repo CLI, repo MCP, and repo coordinator Go sources, compiler inputs, build/test/dev routes, and the generated CLI entity-kind projection.

## Result

The 27 named Go implementation leaves in this lane now live either as an anonymous `🐹️.go` in an exact native Go package directory or as an anonymous `🐹️.go` below a schema-registered semantic domain directory. A census of the three module trees found 39 Go files including existing tests and fixtures; every basename is `🐹️.go`.

The ASCII `cmd/repo` and `internal/*` directories remain exact Go import, entrypoint, and `internal` visibility contracts. They are package boundaries rather than language-owner exemptions, and their anonymous leaves do not prevent another implementation kind from living beside them. Multi-concern module roots use semantic directories. The compiler plan projects those source leaves into their owner package with the standard Go `-overlay` mechanism.

`canonicalGoPlan` reads its test directory, anonymous Go filename, opaque directories, and admitted semantic source directory names from the taxonomy. It stops current-module discovery at nested `go.mod` boundaries, follows filesystem-backed `replace` directives recursively, and adds dependency source projections without adding dependency or semantic directories to the current test package list. CLI, MCP, coordinator, root MCP build, native shell bootstrap, native Windows bootstrap, and launch-based dev paths now converge on `runCanonicalGoBuild` or `runCanonicalGoTests` through their existing `📜️script.ts` and Nx routes. The MCP build/test targets declare the entity-catalog generator dependency needed to materialize their transitive CLI input in a fresh workspace. No source copies, runtime adapters, or migration scripts were added.

The coordinator platform sources retain their exact content build constraints:

- `🐚️unix-durability/🐹️.go`: `//go:build !windows`
- `🪟️windows-durability/🐹️.go`: `//go:build windows`

The entity-kind generator contract, producer, tests, source documentation, ignore rule, and CLI source reference now point to `⌨️cli/🏷️entity-kinds/🐹️.go`. Regeneration produced the same SHA-256 as the former generated path.

## Test-Driven Compiler Contract

The language-neutral JSON fixture `canonical-go-input-projection-v2` declares source domains, test cases, opaque fixture ownership, a nested local module, and exact expected package/source/test selections. Ajv validates it independently. The test materializes two native Go modules and treats `go test -overlay` output as the toolchain oracle.

Red evidence before source projection: the registered Nx target failed because the expected `🧮️addition/🐹️.go` source was absent from the replacement map.

Red evidence before transitive local-module projection:

```text
expect(plan.packages).toEqual(vector.expectedPackages)
Received: [".", "./dependency", "./internal/text"]
Expected: [".", "./internal/text"]
```

Final direct registered script result:

```text
[DEBUG] Canonical Go discovery oracle {"packages":[".","./internal/text"],"tests":["TestPrivateAddition","TestPrivateUppercase"]}
1 pass
0 fail
13 expect() calls
```

An earlier registered Nx green run passed the same-module v2 fixture. The final transitive Nx attempt did not reach the target because concurrent workspace state made project-graph construction fail with both `ENOENT .../💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/Cargo.toml` and `Source project does not exist: npm:@asamuzakjp/css-color`. The exact registered `📜️script.ts test go-input-projection` route then passed the final fixture and native oracle.

## Move and Identity Map

All paths below are repository-relative; prepend the exact workspace root `/Users/ueli/Documents/semio/` for the absolute path. `byte-identical` means the pre/post SHA-256 values match. `path-reference-only` means replacing the one documented new generated-source path in a comment with its former path restores the pre-move hash.

| Old path | New path | Pre SHA-256 | Post SHA-256 | Identity |
|---|---|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/command/🎮️command.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/command/🐹️.go` | `44db0bf96695b9e2333a5244cd6b491c4949c45d3c37dfa138c5035f421ce674` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/eventstore/🗄️eventstore.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/eventstore/🐹️.go` | `a3a0019d20443f37f6f57c044c07211fc818f0144019c2b0a2ae30a2195609a4` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/glob/🃏️glob.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/glob/🐹️.go` | `1dec88377a5c37768bed32697d94d7b83bf254366983a12349d4cf6e2bacb420` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/graphql/🔗️graphql.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/graphql/🐹️.go` | `0efea89e21739ea9fd3da68e1c3d31f173218b0b375706093ad9a0001eef3532` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/humanize/🗣️humanize.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/humanize/🐹️.go` | `dc8fb0d71e52188ec3ff623f435de5c607ce216dcf5519e15ef585999f91474f` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/id/🪪️id.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/id/🐹️.go` | `23002924f4f460b154eab75d3dab8b16bd47e607221c04bf4b5d82026fc73a57` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/ignore/🙈️ignore.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/ignore/🐹️.go` | `a1dc784dbf3d37229b46aa6ca80fcf924e8974988383d250761bd2d696dcbf89` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/mcp/🔌️mcp.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/mcp/🐹️.go` | `a1ca09d602fc213e0406886ca9b97761b537601f19e30609ffe1dd76a84705b4` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/mcpserver/🖥️server.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/mcpserver/🐹️.go` | `4d96796a13ad1b27e01ae694565525f5bfbdc1afa998ade97a78f7142c76f6b7` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/search/🔎️search.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/search/🐹️.go` | `cc620429b955eb86f7f242fe0a05b69bb480d0e05e7c364ab0a47bde33c21cd1` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/templatefunc/🪄️templatefunc.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/templatefunc/🐹️.go` | `d771101f4cba358e0c36cb708526ed3a1f29583ab78d0852d1198eddd708f0fa` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/yaml/🧾️yaml.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/internal/yaml/🐹️.go` | `f2bbe01a5d01913c3cfc69560cb2e0944fa4ec4fbab6a1da5a03e29b955aa205` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📤️event_export.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📤️event-export/🐹️.go` | `d69effc399d2dae560f12decbd4f2ea518975ae9cd2d57d894bff3b3fe8ac055` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧩️component/🐹️.go` | `b6d1c1b5bd96d8986b8eb4cf9429cc03ca2be76dc3759a301b198fcd57f95e8f` | `29fad384d28df1b44dc6c87d534ef2aff4818e1b6bfd230a3baf8b20f8b674b7` | path-reference-only: generated source comment updated |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go` | `29342aadd5d45848a4046968b8a591871e34d0ced2b1039588e1d2facefb7770` | same | byte-identical after regeneration |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📜️protocol.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📜️protocol/🐹️.go` | `f267443c98055944a1c036e0bb3d1ac7257ba57e73deb115ecc0223fb8218304` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📡️event.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📡️event/🐹️.go` | `45f40be49e96f472b993fd51397ae7ee448b6de7f2641d48742524eadba7047c` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🖥️server.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🖥️server/🐹️.go` | `a67fc22c366fb74363847444a790c281141b207933bc2a1fbbc62f729ae84533` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🗄️repository.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🗄️repository/🐹️.go` | `242dc6335ec056ae4e697d6fd93df59d17002e7e4e8b45a7a4252e9e8a1b9007` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🚚️transport.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🚚️transport/🐹️.go` | `9a16218b30601ff68868d2b82629f7163e56f9d58b3ec95a8b88341878af895e` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧩️component.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧩️component/🐹️.go` | `29a1875fa300aad1a6cbce657715975c77e24572b6822fe1f094ab091ce93005` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🐚️durability_unix.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🐚️unix-durability/🐹️.go` | `8d8a25a38c6d2634e785b1eb8ffa265e9572a8f31f1cfaf87154f0b415e412c8` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📚️repository.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📚️repository/🐹️.go` | `c98ea539c36b39d87b4f8af4899811977810e7803ca2baa4284dcc773e85f681` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🗄️event_store.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🗄️event-store/🐹️.go` | `2dbe92007af47c9f5115de5e3549cd0e0566311c721d709dbfb29a896771e8ea` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🛡️durability.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🛡️durability/🐹️.go` | `9652906e07c510750bd41f86e357e52050ba6ada6047143fb4736b244e3c8c97` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧩️component.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧩️component/🐹️.go` | `fe6e73bff1323dc29c605a12eb75ec1908e50c5bb0c96f5aa18b8aa5c688dd36` | same | byte-identical |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🪟️durability_windows.go` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🪟️windows-durability/🐹️.go` | `0c9eb52a1d6efe527afee22db372b1e1efb16290c537ec1b42236872e5bf8b17` | same | byte-identical |

Summary: 26 byte-identical moves and one source whose only byte change is its generated-source path comment. An active-tree exact old-path scan outside ticket history returned zero references.

## Validation

| Check | Result |
|---|---|
| Final language-neutral fixture through `📜️script.ts test go-input-projection` and native `go test -overlay` oracle | PASS: 1 test, 13 assertions |
| Entity-kind schema/generator tests | PASS: 6 tests, 50 assertions |
| Entity-kind `generate` then `check` | PASS; 58 kinds, 2 emoji-shadowed; Go output hash `29342a…770` unchanged |
| Current-host CLI build | PASS |
| Current-host MCP build, including transitive CLI projection | PASS |
| Current-host coordinator build | PASS |
| Windows amd64, `CGO_ENABLED=0`, CLI/MCP/coordinator builds | PASS for all three |
| MCP full native tests | PASS: `ok github.com/usalu/semio/repo/mcp` |
| Coordinator full native and TypeScript tests | PASS: Go `ok`; 2 Vitest files and 118 tests passed |
| CLI focused G1/event export/glob/entity catalog/root metadata/internal command/internal eventstore contracts | PASS across root and both tested internal packages |
| Shell and Windows bootstrap route subtests | PASS |
| POSIX bootstrap syntax | PASS via `zsh -n` |
| PowerShell parser | Not run: `pwsh` unavailable on this macOS host; the Windows route text contract and Windows Go cross-build passed |
| Launch registration | PASS: `⚖️gate🐹️compiler-input-projection` present in seed and derived launch |
| Post-move Go census | PASS: 39/39 Go file basenames are `🐹️.go` |
| Exact active old-path scan | PASS: zero results |

The current broad CLI suite compiled the root and internal packages, then failed assertions. The first current failures were retained exactly:

```text
TestMcpBootstrapAssetsStayRepoRelative/devcontainer_builds_the_repo_client_from_its_canonical_source:
failed to read /Users/ueli/Documents/semio/.devcontainer/post-create.sh: no such file or directory

TestFileHeaderId/code_ts:
FileHeaderId("compose/js/src/index.ts") = "🗃️compose🗃️js🗃️src💻index", want "🏘️compose📜️js🗃️src💻index"

TestFileHeaderId/code_go:
FileHeaderId("repo/client/client.go") = "🗃️repo🗃️client💻client", want "🧰️repo⌨️client💻client"

TestCollectGoTestsInSection:
expected non-empty pattern for Alpha section
```

The same run also reported other catalog/identity, removed autofix, missing legacy fixture path, and logging/config assertions. This lane did not infer a baseline for those failures. The exact current broad output was inspected before generated logs were removed. Focused tests covering the moved domains, entity catalog, both revised bootstrap routes, MCP, and coordinator passed.

## Direct Tool and Editor Boundary

Routine launch entries invoke Nx targets. CLI, MCP, and coordinator `dev` targets depend on their overlay-aware build target and execute the resulting binary. No repository VS Code setting or launch entry was found that invokes `gopls`, `go test ./...`, or `go build ./...` for these modules.

Direct `go build`, `go test`, or `gopls` recursive discovery does not know the schema-derived overlay. In particular, `./...` can treat semantic source directories as separate Go packages. Callers must use the registered Nx build/test/dev targets, whose compiler plan selects only the declared owner package set. This is a native tool interface limitation; the authored taxonomy remains implementation-neutral and contains no duplicated compatibility sources.

## Exact Supporting File List

Each link label is the exact repository-relative path and each target is its exact absolute path. The move table above is the authoritative removed/created source-file list. The following files carry compiler, schema, producer, consumer, launch, and validation changes for this lane:

- [`.gitignore`](/Users/ueli/Documents/semio/.gitignore)
- [`.vscode/🧩️launch.seed.jsonc`](/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc)
- [`.vscode/launch.json`](/Users/ueli/Documents/semio/.vscode/launch.json)
- [`📜️script.ts`](/Users/ueli/Documents/semio/📜️script.ts)
- [`🧰️framework/🔨️modules/🧬️schema/⚛️component.rs`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/⚛️component.rs)
- [`🧰️framework/🔨️modules/🧬️schema/🔣️.json`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🔣️.json)
- [`🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts)
- [`🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/package.json)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🐹️canonical-go-discovery/🔣️.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🐹️canonical-go-discovery/🔣️.json)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/README.md`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/README.md)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🟦️typescript/📜️script.ts)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go/📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go/📜️script.ts)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go/📋️project.json`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go/📋️project.json)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🧪️tests/🤝️protocol-contract/🐹️.go)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🗄️g3-event-store/🐹️.go`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🧪️tests/🗄️g3-event-store/🐹️.go)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh)
- [`🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1)
- [`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-go-domains-2026-09-12.md`](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-go-domains-2026-09-12.md)

The generated output [`🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go) exists at the new contract path and remains intentionally ignored by the exact `.gitignore` rule.
