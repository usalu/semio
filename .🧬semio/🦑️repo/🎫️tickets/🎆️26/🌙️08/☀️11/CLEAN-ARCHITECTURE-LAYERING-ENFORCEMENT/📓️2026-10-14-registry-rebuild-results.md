# Registry Rebuild Composition Results

## Completed Ownership Change

The neutral registry owns its seven generic input/descriptor/registry stages and three framework guest checks. S owns its stdio guest check, activation, convergence invocation, and flow bindings in `✏️s/🧑‍💻dev/🔁️rebuild/🔣️.json`. Hub owns catalog preflight and publication in `🌎️hub/🔁️rebuild/🔣️.json`. S explicitly composes these inputs into thirteen stages and four guest checks through the neutral schema contract. Removing every specific contribution leaves the neutral plan intact; removing each guest/catalog contribution preserves the other contribution.

The schema admits exact fields, portable Cargo workspace paths of at most 256 characters, declared dependency stages and Nx command vectors whose first word is nx. The parser additionally enforces unique stage ids, dependency order and crate-qualified features. Empty guest check arrays permit catalog-only contributions. The former readRebuildChain/readGuestFrameworkChecks entry points are retired; there is one plan parser and explicit composition API, with no compatibility wrappers.

Development activation convergence now lives at the general development verification owner. Its canonical command is `@semio-tech/framework-os-dev:verify-staged --variant s`; registry no longer imports development activation or registers this command. The runner accepts general execution/lease ports and owner-authored plan input. The production adapter uses bounded async owned commands and the caller's existing cancellation signal, releasing its lease on completion, failure and cancellation.

The existing all-plugin launcher now routes to `@semio-tech/s-rebuild:rebuild-all`. The neutral rebuild remains separately registered. Plan inspection, owner guest checking and both scoped tests are registered in Nx and the launch catalog, with the same commands also authored in `.vscode/🧩️launch.seed.jsonc`. No packages, Git state, AGENTS files, worktrees or repository goals were modified by this agent.

## Verification

Fixtures and independent Ajv admission cases were authored before the implementation. Initial canonical root Nx attempts did not reach test execution: shared graph/hash preparation remained active for over three minutes; the no-daemon retry waited for graph construction in another process. Those attempts were cancelled and are not passing test evidence. The later isolated root attempt was interrupted by the active-turn interruption. There is no canonical root target pass claim.

The ticket-owned minimal Nx workspace `v` executes current original source files without source copies. The final observed `architecture-source-verification:test` run passed **16 tests, 0 failures, 95 expectations** across the registry and S test files. Portable JSON cases prove deletion independence and lease release on success/failure/cancellation. Ajv independently validates plans and malformed vectors; its schema-driven reference ordering agrees byte-for-byte with the composition output. Jsonc-parser independently reads authored and projected launch configurations and proves eight canonical routes appear once and match exactly.

Real runtime output observed neutral plan `steps=7 guestChecks=3`, composed S plan `steps=13 guestChecks=4`, and runner progress for successful, failed and cancelled executions through controlled execution/lease ports. These tests do not compile every Wasm component or publish the actual trusted catalog.

The final scoped `architecture-source-verification:strict` run passed TypeScript strict checking for the neutral executor, S composition, S script, development convergence implementation, and both current test files. The wider strict attempt including registry script transitively reached unrelated existing playground tests and failed: `⭐️default/🧪️tests/🟦️.ts` accesses fixture catalogDefault fields absent from some rows, and `🖼️assets/🧪️tests/🟦️.ts` has assert undefined-predicate typing and an EventEmitter.once import typing error. Those files were not changed by this agent.

Commands used from ticket/v:

```text
NX_DAEMON=false NX_WORKSPACE_DATA_DIRECTORY='../🗑️generated/v/registry-test-nx' bun /Users/ueli/Documents/semio/node_modules/nx/dist/bin/nx.js run architecture-source-verification:test --files='<registry rebuild test>,<S rebuild test>' --group=registry --outputStyle=stream
NX_DAEMON=false NX_WORKSPACE_DATA_DIRECTORY='../🗑️generated/v/registry-nx-data' bun /Users/ueli/Documents/semio/node_modules/nx/dist/bin/nx.js run architecture-source-verification:strict --files='<neutral executor>,<S composition>,<S script>,<development convergence>,<both tests>' --group=registry --outputStyle=stream
```

## Exact Generated Catalog Follow-On

The concrete generated catalogs remain under registry/🤖️generated. Moving bytes alone would leave the producer and consumer authority unchanged. `registry/📽️projection/🟦️.ts` emits generated TypeScript imports relative to the registry owner and writes fixed registry-root output directories in GenerateScript, PreviewGeneratedScript, CheckGeneratedScript and CheckScript. Registry discovery and projection consume a repository catalog snapshot rather than a deployed caller-owned catalog.

The required follow-on is to supply owner-authored publication inventory and output roots to that producer, relocate S/Hub concrete projections with their caller composition, and replace compile-time catalog constants in framework consumers with supplied neutral inventory ports. Confirmed direct consumers include development `♻️activation/🌐️browser-host/🏗️staging`, `♻️activation/🏃️execution`, `♻️activation/🔍️freshness`, `🚚️distribution/📋️plan`, `⚖️parity/🏃️execution`; plugin `🏗️build/📦️materialization` and `📊️size`; repository library `🎮️playground`; and renderer/browser build consumers. The moved development convergence verifier still reads the shared generated catalog, so relocating generated data also requires that verifier to accept an explicit deployment catalog root. This larger producer-consumer correction was not represented as complete by merely moving generated assets.

## Changed Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔁️rebuild/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🔁️rebuild/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🧫️fixtures/🔁️rebuild/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔎️verification/🧾️convergence/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`
- `🌎️hub/🔁️rebuild/🔣️.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `✏️s/🧑‍💻dev/🔁️rebuild/🔣️.json`
- `✏️s/🧑‍💻dev/🔁️rebuild/📋️project.json`
- `✏️s/🧑‍💻dev/🔁️rebuild/📜️script.ts`
- `✏️s/🧑‍💻dev/🔁️rebuild/🟦️.ts`
- `✏️s/🧑‍💻dev/🔁️rebuild/🧪️tests/🟦️.ts`
- `✏️s/🧑‍💻dev/🔁️rebuild/🧪️tests/🧫️fixtures/🔣️.json`

## Generated Output Cleanup

Removed only this agent’s root-attempt `🗑️generated/reg-rebuild` subtree and isolated `🗑️generated/v/registry-test-nx`, `registry-nx-data`, and `registry` artifact directories after verification. Shared verification inputs, other agents’ generated outputs, permanent fixtures and all markdown evidence remain. Final strict source check passed in 7.8 seconds; the post-change source test rerun passed the same 16 tests and 95 expectations.
