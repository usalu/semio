# Phase 9/10 Literal Zero-Dependency Closeout Packets

## Decision and live census

This is a live, read-only plan.  It supersedes the historical 185/209/`75 Rust +
134 JavaScript` checkpoints in the Phase 9/10 reports; the checked current
baseline is `🔒️dependencies.json` (232 rows, commit
`215e369d07d8014806a43f8f75a1bba3c6015908`), while the current collector reports
230 rows and two baseline removals (`python:pypdf`, `python:simplejson`).  A
baseline is a ratchet, not permission to retain its rows.

| ecosystem | collector identities | production-reachable | kind census | correction needed before it can be a literal-external count |
| --- | ---: | ---: | --- | --- |
| Rust | 85 | 62 | 62 runtime, 2 build, 23 oracle, 3 runner | None known in collector identity ownership; oracle classification is not proof that a direct product manifest is test-only. |
| JavaScript | 70 | 31 | 31 runtime, 41 tooling, 3 oracle | None in identity collection; parity currently finds 84 manifests, 251 rows, 103 evidenced, 4 undeclared imports, and 1 lock workspace mismatch. |
| Go | 60 | 60 | 13 runtime, 58 build | **2 are first-party false positives**: `github.com/usalu/semio/repo/client` and `github.com/usalu/semio/repo/go`; literal external count is 58 once `go.work`/local replaces are honoured. |
| Python | 15 | 0 | 15 runner | The root `pyproject.toml` is Composition tooling yet is collected; the literal non-`compose/` product count is 0 only after that scope rule is made explicit. |
| .NET | 0 | 0 | none | No action. |
| **raw total** | **230** | **153** | — | **228 actual third-party identities after the two Go corrections; Python must be classified, not silently deleted.** |

The target is zero external source, build, test, and production dependencies.
The only operational boundary is the AGENTS-mandated toolchain: the Bun binary
(`packageManager: bun@1.2.5`, `engines.bun`) and Nx runner (`nx` plus the directly
required `@nx/*` runner packages).  That exception is narrow: it must not appear
in exported product APIs or product sources, and it does not exempt React,
TypeScript, Vite, test tools, or a lockfile closure.  All permanent runner logic
continues through root `📜️script.ts`; root `package.json` continues to launch it
with Bun/Nx.  Do not remove or replace Bun/Nx in any packet.

The current JavaScript parity gate is *already red*: four undeclared imports
(`vitest`, `semver`, `clsx`, `class-variance-authority`) and one
`workspace-missing` lock row in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/package.json`.
Its 148 unowned rows are migration inventory, not evidence of no use.

## Shared ownership and ordering

`📜️script.ts`, root `package.json`, root `Cargo.toml`, `bun.lock`,
`🔒️dependencies.json`, `go.work`, each Go `go.sum`, and `.vscode/launch.json` are
serial files.  A packet may change only its named domain manifests and source
tree, then hand the serial-file diff to the listed integration packet.  The
existing P10 boundaries are mandatory reuse points, not optional new adapters:

- `🧰️framework/🔨️modules/🖱️ui/…/🟦️build-tooling.ts` and
  `.storybook/🟦️lint-tooling.ts` for tooling;
- the OS development `🟦️config-tooling.ts` boundary;
- `@semio-tech/ui-react/runtime` for UI runtime imports; and
- P9aa's owned component interpreter in
  `…/🔌️plugin/🧠️interpreter/🦀️component.rs` and
  `…/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`.

The following packets are independently writable once their predecessor is
closed.  “Mutation” is a deliberately hostile, source/static check performed by
the packet owner; it is not a substitute for its language-agnostic behaviour
test and an in-repo/third-party-oracle comparison while that oracle exists.

| id / prerequisite | exact owned manifests and source boundary | removal scope | acceptance gate and faithful hostile mutation |
| --- | --- | --- | --- |
| **Z0 — verifier truth** (first; serial) | `📜️script.ts:13651-14470`, `🔒️dependencies.json`; read `go.work`, root `pyproject.toml`, all `Cargo.toml`, `package.json`, `go.mod`, `*.csproj` | Add all-ecosystem list/summary and a literal-external mode; recognise workspace Go module/replaces; classify the root Composition Python project explicitly; fail if a dependency registry calls a product manifest “oracle” without a test-only owner. | `verify dependencies` reports raw and corrected counts per ecosystem and a zero target; a synthetic first-party Go module is excluded, a synthetic external Go module is retained, a root Composition Python row is scoped out, and a direct runtime manifest cannot be hidden as an oracle.  **Collision:** no other packet edits `📜️script.ts` or the baseline. |
| **P9-A — Rust browser ABI** (after Z0) | root `Cargo.toml`; the 50 direct `wasm-bindgen`, 17 `wasm-bindgen-futures`, 13 `web-sys`, 5 `serde-wasm-bindgen` owners under `✏️s/🔌️plugins/**/📦️packages/🦀️rust/Cargo.toml` and `🧰️framework/**/📦️packages/🦀️rust/Cargo.toml`; source stays behind the owned platform/browser boundary | Replace the browser/Wasm ABI shim family (`wasm-bindgen*`, `web-sys`, `js-sys`, `serde-wasm-bindgen`) with owned ABI/data transfer code, by domain subtree rather than a repo-wide search/replace. | Static gate: no direct ABI crate in the named manifests or source imports. Mutation: malformed JS values, a missing optional property, and an interrupted callback must yield the owned error/cancellation contract without panicking or retaining a JS handle. **Collision:** root `Cargo.toml`; serialize its final deletion hunk. |
| **P9-B — plugin component host** (after P9-A and P9aa full-suite proof) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📦️glue.rs`, their Cargo manifests and the P9aa scale/stdio fixtures | Remove `wasmtime`, `wasmtime-wasi`, and `wit-bindgen` only when the owned interpreter is the production default for component, describe, scale, and stdio paths. | Preserve component import/export, resource lifetime, trap, and stdio cancellation parity. Mutation: unknown export, invalid component bytes, guest trap, and cancelled stdin all fail deterministically with no Wasmtime fallback. **Hard dependency:** P9aa explicitly says this proof is not yet complete. |
| **P9-C — native renderer** (after P9r) | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`; its `🌋️vulkan`, `🍎️metal`, `🧊️webgpu`, `🪟️d3d12` targets; OS renderer `…/📺️renderer/🧑‍🎨️engine/…/🎯️targets/🧊️wgpu/Cargo.toml` | Replace `wgpu`/window/graphics/text stack (`wgpu`, `winit`, `ash*`, `parley`, `swash`, `taffy`, `tiny-skia`, platform bindings) with the owned renderer path. P9r only removed executor/test remnants; it did not clear these. | Static: no native renderer crate declarations in these owners. Mutation: zero-sized surface, lost surface, unsupported adapter, and cancelled frame must return owned status and release resources. **Collision:** UI root Cargo manifest and renderer targets. |
| **P9-D — Rust hub storage/network** (after Z0; parallel to B/C) | `🌎️hub/📦️packages/🦀️rust/Cargo.toml`; OS DB, MCP, and server Cargo manifests under `…/🛢️db`, `…/🌉️mcp`, `…/🖥️server` | Replace `tokio` (9 owners), `tokio-tungstenite` (3), `futures` (4), `axum`, `sqlx*`, `neo4rs`, and direct database/wire clients with owned event-log, transport, scheduling, and cancellation interfaces. | Language-agnostic event-log/transport vectors match the temporary oracle. Mutation: dropped peer, duplicate event, malformed frame, DB unavailable, and cancellation while waiting neither hangs nor commits partial state. **Collision:** server protocol types and root Cargo workspace rows. |
| **P9-E — Rust geometry/document/codecs** (after Z0; parallel to B-D) | framework Rust and mesh-engine Cargo manifests; Puzzle Cargo manifest; document/print Rust manifests that own `gltf`, `nalgebra`, `parry3d`, `typst*`, `usvg`, `vello*`, `base64`, `blake3`, `miniz_oxide`, `ureq`, `getrandom` | One domain-owned replacement per codec/geometry/document contract; do not coalesce unrelated algorithms merely because their crate appears in the same lock closure. | Each contract has canonical vectors and temporary third-party comparison. Mutation: corrupt/truncated compressed bytes, non-finite geometry, invalid SVG/PDF input, unavailable entropy/network, and cancellation produce explicit owned errors. **Collision:** shared geometry primitives and workspace Cargo rows. |
| **P9-F — Rust macro and oracle evacuation** (after A-E) | all macro `Cargo.toml` owners of `proc-macro2`, `quote`, `syn`; `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml` plus any product manifest currently classified as oracle | First remove production direct declarations of `image`, `png`, `zip`, etc.; then move temporary comparisons to an isolated oracle owner; finally replace/remove every oracle and test runner crate. | The collector must show no product owner for an oracle row before it is excluded. Mutation: malformed macro/token input and corrupted fixture must make the owned diagnostic/decoder fail predictably, never silently accept. **Collision:** test fixture format and root Cargo workspace rows. |
| **P10-A — UI product runtime** (after P9-A where browser ABI is shared) | UI React package and consumers using `@semio-tech/ui-react/runtime`; direct owners of `react`, `react-dom`, `@react-three/fiber`, `@react-three/drei`, `three`, `three-mesh-bvh`, `@dnd-kit/*`, `@xyflow/react`, `i18next`, `react-i18next`, `xstate`, `katex`, `reveal.js`, `pdfjs-dist`, `sharp`, `brepjs*`, `dagre`, `ajv` | Replace product runtime imports through the existing owned runtime boundary; never add a second React façade. | Static: no listed dependency import/declaration in the packet owners; product behaviour vector agrees with its temporary oracle. Mutation: malformed document, drag cancellation, absent translation, invalid graph, and lost render context retain owned UI state without external fallback. **Collision:** UI package manifests, shared CSS/runtime exports, `bun.lock`. |
| **P10-B — JavaScript server/host** (after P10-A where UI is embedded) | direct `next`, `pg`, `pg-boss`, `@modelcontextprotocol/sdk`, `@napi-rs/canvas`, `esbuild` owner manifests and their host/server source trees | Own request routing, database job/event persistence, MCP protocol, native canvas and bundling contracts. | Request/event/MCP canonical vectors; mutation: malformed RPC, database disconnect, job replay, missing native capability, and cancelled bundle yield owned errors with no library-specific type leaking. **Collision:** root `package.json`, server package manifests, `bun.lock`. |
| **P10-C — JS tooling, types, and test oracle** (after A/B) | root `package.json`; 84 discovered JS manifests; existing `🟦️build-tooling.ts`, `.storybook/🟦️lint-tooling.ts`, OS `🟦️config-tooling.ts`; current undeclared sites listed above | Remove `typescript` (61 direct manifests), `vite` (9), `vitest` (27), Playwright/Storybook/ESLint/Tailwind and remaining `@types/*` only after owned build/test/config equivalents exist. Repair the four undeclared imports and one missing lock workspace as part of ownership, not with broad root declarations. | `verify dependencies parity js --no-unowned-rows` is clean, then the all-ecosystem verifier is zero outside the Bun/Nx exception. Mutation: an undeclared synthetic import, stale lock workspace, and a prohibited config import each make the verifier fail. **Collision:** all JS manifests, root package, lockfile, config boundaries. Serialize this packet. |
| **G1 — Go CLI contract** (after Z0) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/📦️packages/🐹️go/go.mod`, sibling `go.sum`, and that `🐹️go/` source tree | Replace CLI parser/template/glob/search/ID/YAML/SQLite roots (`cobra`, `sprig`, `doublestar`, `bleve`, `uuid`, `yaml.v3`, `modernc.org/sqlite`) with owned interfaces; transitive closure disappears from this module. | Command vectors for parse/template/glob/search/store; mutation: invalid flag, recursive glob, bad template, corrupt index, and cancellation leave no partial artifact. **Collision:** CLI uses the shared SQLite choice with G3; do not edit `go.work`. |
| **G2 — Go MCP contract** (after G1 for shared primitives) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🐹️go/go.mod`, sibling `go.sum`, and that `🐹️go/` source tree | Replace `mark3labs/mcp-go` and remaining wire/schema direct roots with owned protocol types. | JSON-RPC/MCP golden vectors; mutation: unknown method, invalid params, cancelled request, and oversized payload return an owned protocol error. **Collision:** shared Go protocol model and `go.work`. |
| **G3 — Go coordinator store** (after G1) | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🐹️go/go.mod`, sibling `go.sum`, and that `🐹️go/` source tree | Replace `modernc.org/sqlite` with the owned event-store contract; remove the remaining Go production closure. | Event-store replay/concurrency vectors; mutation: interrupted write, duplicate event, corrupt log, and unavailable store provide atomic/explicit results. **Collision:** shared store schema with G1 and `go.work`. |
| **P10-D — terminal integration** (strictly last; serial) | root `Cargo.toml`, root `package.json`, `bun.lock`, `go.work`, all `go.sum`, root `pyproject.toml`, `🔒️dependencies.json`, `📜️script.ts`, `.vscode/launch.json` | Delete empty dependency/lock rows only after all domain gates; codify the Bun/Nx-only exception; regenerate the freeze baseline only at actual zero. Add ordered launch gates beside the existing `⚖️gate` group: dependency truth, all-ecosystem zero/exemption audit, and JS parity. Launch commands call `bun ./📜️script.ts verify dependencies …`; existing build/test commands remain `bun nx …`. | Clean verifier: zero literal third-party rows in Rust/JS/Go/Python/.NET, zero production-reachable, zero JS unowned/undeclared/lock mismatch, and an explicit Bun/Nx exception report. Mutation: add one external row in each manifest family and one illicit non-Nx root tool declaration; all must fail before any build. **Collision:** every serial file; one integrator only. |

## Feasible wave plan

1. `Z0`; then run P9-A, P9-D, P9-E, G1, and the source work of P10-A in
   parallel, each avoiding serial-file edits.
2. After their owned contracts land: P9-B (only after P9aa parity), P9-C,
   P9-F, P10-B, G2, and G3.
3. P10-C owns the JavaScript manifest/lock convergence.  P10-D is the sole
   final manifest, verifier, baseline, and launch integration.

No packet may use `write-baseline` to hide a remaining dependency, declare an
import at root merely to make parity green, or remove Bun/Nx.  The exact
closeout condition is the Z0 corrected census at zero plus P10-D's source,
manifest, lock, and launch gates.
