---
name: Operator-Typed Brep Nodes
overview: Switch channel typing from schemas to required-operator capabilities across neural/flow, then replace the OpenCascade JS brep kernel with a pure-Rust brepkit-backed flow module so every procedural node has accurate, capability-typed channels instead of generic in/out.
todos:
  - id: channel-operators-core
    content: "neural/engine: replace ChannelSpec.schema with operators: Vec<String>; add Registry capability map (schema->operators) + finalize to auto-fill output provided operators; adjust channel_schema fallback; update builders; keep dispatch/variadics/inject_channel_defaults working; update neural tests."
    status: completed
  - id: channel-operators-modules
    content: Update flow/module/wasm glue and all in-repo modules (core/math/text/logic/dictionary/list) registrations + tests to operator-typed channels.
    status: completed
  - id: channel-operators-flowcore-react
    content: flow/core port layout (neuron_io_layout/input_spec_to_port) and flow/react FlowChannelSpec + connection compatibility switched to required-operator set-containment; update flow/react vitest.
    status: completed
  - id: brep-rust-interface
    content: Create geometry/brep/engine (Rust BrepKernel trait + handle/kind/mesh value types) as the external-lib interface; wire Cargo workspace, script.ts, launch.json.
    status: completed
  - id: brep-rust-brepkit
    content: "Create geometry/brep/brepkit (brepkit-backed impl): Topology+handle registry, supported ops, STEP/STL IO, and tessellation (tris+normals+edges+face groups) to mesh buffers; tests."
    status: completed
  - id: brep-flow-module
    content: "Create flow/module/brep WASM module: register geometry schema + operator-typed brep operators (reusing math point/vector + core schemas), stateful kernel, and wasm-bindgen manifest/evaluate/tessellate/dispose; tests."
    status: completed
  - id: procedural-integration
    content: Rewire procedural/react to load the brep WASM module via the normal flow path; preview tessellation through the wasm bridge; update procedural/play wiring, Vite/aliases, launch.json, root workspaces, extension-tree counts; update brep tests in procedural/react.
    status: completed
  - id: remove-opencascade
    content: Remove geometry/brep/js OpenCascade kernel and brepjs/brepjs-opencascade dependencies and dead contracts.
    status: completed
  - id: validate-e2e
    content: "Run cargo tests (neural + all flow modules + brep crates) and vitest (flow/react, procedural/play); browser-probe procedural: capability-typed ports, box->fillet->translate renders, compatibility enforced."
    status: completed
isProject: false
---

# Operator-Typed Channels + Pure-Rust Brep Nodes

## Goal

1. Channels declare a SET of required operators (capability typing), replacing the single `schema: ValueType` on `ChannelSpec`. Connection validity = source value's schema supports all required operators.
2. Replace the OpenCascade JS brep kernel with a pure-Rust kernel built on [brepkit](https://github.com/andymai/brepkit), exposed as a real flow WASM module (`flow/module/brep`).
3. Every procedural node (the brep catalogue) is defined with accurate named, capability-typed input/output channels — no generic `in`/`out` fallback.

Scope decision (confirmed): port only brepkit-supported operations and REMOVE the rest; fully remove the OpenCascade JS kernel. Greenfield, no legacy/compat.

## Architecture

```mermaid
graph TB
  subgraph rust [Rust]
    neural["neural/engine<br/>ChannelSpec.operators + capability map"]
    wasmglue["flow/module/wasm<br/>manifest + dispatch glue"]
    brepiface["geometry/brep/engine (Rust)<br/>BrepKernel trait (interface)"]
    brepimpl["geometry/brep/brepkit (Rust)<br/>brepkit-backed impl"]
    brepmod["flow/module/brep<br/>schemas+operators, stateful Topology, tessellate export"]
    neural --> wasmglue --> brepmod
    brepmod --> brepiface --> brepimpl
  end
  subgraph js [TypeScript]
    flowreact["flow/react<br/>FlowChannelSpec.operators + compat"]
    proc["procedural/react<br/>loads brep wasm, preview via wasm tessellate"]
    brepmod -->|wasm| proc
    neural --> flowreact --> proc
  end
```



Brep is stateful (arena `Topology` + handle map) while neural operators are stateless dict-in/dict-out. The brep WASM module keeps a thread-local kernel: operators allocate handles and return `{ "$schema": "geometry", "handle": "solid-1", "kind": "solid" }`; a wasm-bindgen `tessellate(handle, tolerance)` + `dispose(handle)` feed the R3F viewport (mirrors today's `MeshTransfer`).

## Phase 1 — Operator-typed channels (core model)

Files: [neural/engine/lib.rs](neural/engine/lib.rs), [flow/module/wasm/lib.rs](flow/module/wasm/lib.rs), all `flow/module/*/lib.rs`, [flow/core/lib.rs](flow/core/lib.rs), [flow/react/index.tsx](flow/react/index.tsx).

- `ChannelSpec`: replace `schema: ValueType` with `operators: Vec<String>`. Keep `default`, `label`. `ValueType`/`FieldSpec`/`Schema` stay (data-shape + dispatch).
- `Registry`: add a `finalize()`/capability pass building `schema_id -> Set<operator_id>` (operators whose impl signatures include that schema). `register_operator` gains the operator's produced output-schema id(s) so the registry can auto-fill each OUTPUT channel's provided `operators` (= operators supported by produced schema). INPUT channels author required `operators` explicitly.
- Compatibility rule (single source of truth): input.required ⊆ output.provided.
- Dispatch unchanged: still keys on runtime `$schema` via `operator_signature` (L615-633); change `channel_schema` fallback (L606-613) to use the channel default's `$schema` (or empty) since `schema` is gone.
- `inject_channel_defaults` (L561-572) unchanged (uses `default`).
- ChannelSpec builders: replace `number()/list()/dictionary()/value()/text_default()` etc. with capability-based helpers (e.g. `requires(["math.add"])`, plus `*_default` that still emit full schema'd default dicts). Numbers feeding `math.add` require `["math.add"]` and accept number/point/vector.
- `flow/core` port layout (L347-398): `value_type` derives from `operators` (display + compat) instead of `spec.schema.id()`.
- `flow/react`: `FlowChannelSpec.schema` -> `operators: string[]`; update manifest interfaces (L69-138) and any connection/port-compat logic to set-containment.
- Update all in-repo operator registrations (core/math/text/logic/dictionary/list) + their tests to the new channel shape. Keep multi-impl dispatch and variadics intact.

## Phase 2 — Pure-Rust brep kernel (brepkit, behind interface)

Per repo rule "no direct external dependency; wrap behind an interface":

- `geometry/brep/engine` (Rust): a `BrepKernel` trait + value types (geometry handle, kind, mesh buffers) — the interface. Mirrors the contracts in [geometry/brep/js/index.ts](geometry/brep/js/index.ts) (GeometryKind, MeshTransfer, Aabb).
- `geometry/brep/brepkit` (Rust): brepkit-backed impl (git deps `brepkit-topology`, `brepkit-operations`, `brepkit-io`). Holds `Topology` + handle registry; implements primitives, booleans, fillet/chamfer/shell/draft/offset/thicken, extrude/revolve/sweep/loft, section/split/slice, measure, query, heal/sew, STEP/STL IO, and tessellation (adaptive deflection + edges + per-face groups) → mesh buffers.
- Add both crates to root `Cargo.toml` workspace; add `script.ts` per crate (build/test/wasm) and register in `launch.json` following existing grouping.

## Phase 3 — `flow/module/brep` WASM module + node catalogue

- New crate `flow/module/brep` (mirror `flow/module/math`): `Cargo.toml`, `lib.rs`, `project.json`, `package.json`, `script.ts`; uses `flow/module/wasm` glue and depends on `geometry/brep/engine`.
- Register a `geometry` schema (handle/kind fields) and operators for every supported node, each with accurate named, operator-typed channels (e.g. `brep.bool.fuse` inputs `a`,`b` require `["brep.bool.fuse"]`; `brep.xform.translate` `geometry` requires a transform capability, `offset` requires `["math.move"]`/vector; `brep.measure.volume` output requires nothing). Reuse math `point`/`vector` and core `number`/`text`/`list` schemas (brep depends on core+math being loaded).
- Stateful kernel: thread-local `geometry/brep/brepkit` instance; operators read input handle dicts, call the kernel, return geometry dicts.
- Export via wasm-bindgen: `manifest()`, `evaluate(kind, json)`, `tessellate(handle, tol)`, `dispose(handle)`.
- Determine final node set against brepkit's real API; drop nodes brepkit can't do (candidates: gears, minkowski, ellipsoid, helix, some draw2d/sketch2d + 2D booleans, polyhedron) — confirm during implementation.

## Phase 4 — Procedural integration + remove OpenCascade

Files: [procedural/react/index.tsx](procedural/react/index.tsx), [procedural/play/*](procedural/play), [geometry/brep/js/index.ts](geometry/brep/js/index.ts), build configs.

- Replace `BREP_FLOW_KINDS`/`BREP_EVAL_HANDLERS`/virtual-module overrides with loading the brep WASM module through the normal flow module path; `ProceduralExtensionHost` keeps only: load brep wasm, register its manifest, and expose `tessellateGeometry` by calling the brep wasm `tessellate` (preview at L1409-1451 unchanged downstream of `MeshTransfer`).
- Remove `geometry/brep/js` OpenCascade kernel and `brepjs`/`brepjs-opencascade` deps; delete now-dead TS contracts or move the still-needed mesh-buffer types into the wasm bridge.
- Update Vite/optimizeDeps/aliases for `@semio-tech/flow-module-brep` (mirror module-core handling), and procedural play `script.ts` wasm build list + `vitest.config.ts` aliases.
- Update `launch.json`, root `package.json` workspaces (`flow/module/*/pkg`), and `procedural/play/index.ts` extension-tree counts.

## Validation

- `cargo test` for neural + every flow module + brep crates.
- Vitest for `flow/react` and `procedural/play` (extend existing test files only — no new test files; update brep tests in `procedural/react`).
- Browser probe of procedural dev server: catalogue shows brep operators with named capability-typed ports (not in/out), a box→fillet→translate graph evaluates and renders a mesh, and connection compatibility respects required-operator sets.

## Risks / notes

- brepkit coverage gaps shrink the node catalogue (acceptable per scope).
- Tessellation parity: must produce triangles + normals + edges + per-face groups to match the existing viewport `MeshTransfer`.
- This is large; phases 1 and 2/3 are independent and can proceed in parallel, but Phase 3 channel definitions depend on Phase 1 landing.

