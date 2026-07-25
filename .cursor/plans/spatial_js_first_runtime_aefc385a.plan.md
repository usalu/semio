---
name: Spatial JS First Runtime
overview: "Implement `@spatial/js` as the first spatial runtime: a single monolith TypeScript package covering the full architecture from `.repo/✍️/spatial.md` — static factory spec, statechart runtime, Topologic-inspired topology kernel, derived Surface/Part views, document/history, renderer-neutral display IR, and an R3F adapter — using only monorepo runtimes (bun, nx, vitest, react, three, r3f, vite, tailwind)."
todos:
 - id: ticket
   content: Open repo MCP ticket 'Spatial JS First Runtime' under best-fit goal (read repo://goals first)
   status: completed
 - id: schema
   content: Author spatial/schema/json/{factory,topology,expression,display}.json canonical schemas
   status: completed
 - id: fixtures
   content: Author spatial/fixture/{factory.json,cell-complex.json,...} language-agnostic test data (box + extrude + offset-surface factories, nakagin cell-complex sample)
   status: completed
 - id: core-scaffold
   content: Scaffold spatial/js/core (package.json, project.json, script.ts, vitest.config.ts, tsconfig.json)
   status: completed
 - id: core-impl
   content: Implement spatial/js/core/index.ts monolith with all regions (Vec, Refs, Expr, Spec, Topology, Kernel iface, DerivedViews, Statechart, Factory, Document, Display, Factories)
   status: completed
 - id: brepjs-scaffold
   content: Scaffold spatial/js/kernel-brepjs (package.json with brepjs dep, project.json, script.ts, vitest.config.ts)
   status: pending
 - id: brepjs-impl
   content: Implement spatial/js/kernel-brepjs/index.ts adapting brepjs Solid/Face/Edge/Vertex into @spatial/js/core KernelAdapter
   status: pending
 - id: r3f-scaffold
   content: Scaffold spatial/js/renderer-r3f (package.json with react/three/r3f, project.json, script.ts, vitest.config.ts, play/)
   status: pending
 - id: r3f-impl
   content: Implement spatial/js/renderer-r3f/index.tsx (interaction adapter, display adapter, hooks, FactoryCanvas/FactoryDisplay) + play/ demo running the box factory through brepjs kernel
   status: pending
 - id: tests
   content: Write inline import.meta.vitest tests per package; run `bun nx run @spatial/js-core:test`, `:kernel-brepjs:test`, `:renderer-r3f:test` and fix until green
   status: cancelled
 - id: smoke
   content: Run `bun nx run @spatial/js-renderer-r3f:dev` and capture [DEBUG] runtime logs proving full pipeline (spec load → factory → brepjs → r3f mesh)
   status: cancelled
 - id: close
   content: Close ticket with summary + file list
   status: cancelled
isProject: false
---

# Spatial JS First Runtime

## Goal

Bring up `spatial/js/` as `@spatial/js`, the first spatial runtime, end-to-end from topology kernel to R3F renderer. Pure TypeScript, monorepo-only deps.

## Package layout

Mirror `elements/lib/react/topology` exactly (one monolith package, inline tests, play app):

- `spatial/js/index.tsx` — monolith, regions listed below
- `spatial/js/r3f.tsx` — R3F bindings separated so headless consumers don't pay react/three import cost (single re-export from `index.tsx` for non-react use)
- `spatial/js/package.json` — name `@spatial/js`, deps: workspace `react`, `react-dom`, `three`, `@react-three/fiber`, `@react-three/drei`; devDeps: vite, vitest, tailwind, @vitejs/plugin-react, typescript
- `spatial/js/project.json` — nx targets `dev`/`build`/`test` invoking `bun ./script.ts`, env `SPATIAL_JS_PLAY_PORT=6020`/test port `6041`
- `spatial/js/script.ts` — copy of `elements/lib/react/topology/script.ts` task router (dev = vite play, build = vite build, test = vitest run)
- `spatial/js/vitest.config.ts` — jsdom env, `include: ["index.tsx", "r3f.tsx", "play/index.ts"]`, `includeSource` same set
- `spatial/js/play/{index.html,main.tsx,index.ts,vite.config.ts,globals.css,package.json,project.json,vitest.config.ts}` — interactive demo wiring the box factory through the R3F adapter
- `spatial/js/play/fixture/box.factory.json` — canonical static box-factory spec used in tests and demo
- Workspace plumbing: add `"spatial/js"` and `"spatial/js/play"` to root `package.json` workspaces array; nothing else outside the package folder

## `index.tsx` regions (single source of truth)

Each region is a self-contained layer; later regions only depend on earlier ones. Every exported definition starts with an emoji docstring per `AGENTS.md`.

1. `//#region 🧲Header` — module docstring linking to `.repo/✍️/spatial.md`
2. `//#region 🧮Vec` — `Vec3`, `Vec2`, basic vector math (add/sub/cross/dot/norm/distance). No external math libs.
3. `//#region 🪪Refs` — branded ID types: `VertexRef`, `EdgeRef`, `WireRef`, `FaceRef`, `ShellRef`, `CellRef`, `CellComplexRef`, `ClusterRef`, `SurfaceRef`, `PartRef`. `EditableEntityKind` / `DerivedEntityKind` unions exactly per spec.
4. `//#region 🗺️Expr` — declarative expression AST + evaluator: `path`, `const`, `$event`, `var`, `let`, `exists`, `all`, `any`, `not`, comparisons, arithmetic, `abs`, `distance`, `notEmpty`. Used by guards, action values, display params.
5. `//#region 📜Spec` — JSON schema types: `FactorySpec`, `StateMachineSpec`, `StateSpec`, `TransitionSpec`, `ActionSpec` (vocabulary: `assign|clear|append|emit|raise|openTransaction|commitTransaction|rollbackTransaction|requestPreview|kernel.query|resolveEditable|setDiagnostic|clearDiagnostic`), `GuardSpec`, `DisplaySpec`, `SelectionSpec`, `CommitSpec`, `HistorySpec`, `RequiresSpec`. `parseFactorySpec(raw): FactorySpec | null` for runtime validation.
6. `//#region 🧱Topology` — Topologic-inspired storage. Each entity owns geometry data directly (Vertex→Vec3; Edge→`{vertices:[VertexRef,VertexRef], curve:{kind:"line"|"polyline"|"bezier", controls:Vec3[]}}`; Wire→ordered EdgeRefs + closed flag; Face→outer WireRef + holes + surface geometry `{kind:"planar", normal, origin}|{kind:"mesh", vertices, triangles}`; Shell→FaceRefs + adjacency; Cell→closed ShellRefs; CellComplex→CellRefs + shared-face index; Cluster→arbitrary refs, nestable). `TopologyGraph` class holds maps keyed by ref id, exposes `parents()/children()/adjacency()`, manifold/closed checks.
7. `//#region 🔌Kernel` — `KernelAdapter` interface: capability negotiation (`capabilities(): KernelCapabilities`), construction operations (`vertex.create`, `edge.create`, `wire.create`, `face.create`, `cell.createBox`, `wire.extrudeToCell`, `cell.boolean`), queries (`face.area`, `face.normal`, `cell.volume`, `cell.boundary`), tessellation (`entity.tessellate → MeshPreview`). `InMemoryKernel implements KernelAdapter` backed by `TopologyGraph` — pure JS, no brepjs.
8. `//#region 🪞DerivedViews` — `DerivedViewService` computing `SurfaceView` (group/split Faces by `exposure: external|internal`, `stance: horizontal|vertical`; preserves total area) and `PartView` (group/split Cells by `overlap: none|difference|intersection`; preserves total volume). Cache invalidated by topology revision. `resolveSurface(ref): FaceRef[]`, `resolvePart(ref): CellRef[]`.
9. `//#region 🎬Statechart` — pure-TS statechart interpreter (no XState). `StateMachineInstance` with `send(event)`, `getState()`, `subscribe()`; runs transient transitions, executes ActionSpec list against context via Expr evaluator, evaluates GuardSpec.
10. `//#region 🏭Factory` — `compileFactory(spec): FactoryIR`, `createFactoryRuntime(spec, {kernel, document, history?}): FactoryRuntime`. Runtime emits `FactorySnapshot {factoryId, state, context, display, capabilities:{canCommit,canCancel,canUndo,canRedo}, diagnostics, revision}`. `commit()` produces a `DocumentCommand`.
11. `//#region 📄Document` — `ModelDocument {topology, operations: ShapeNode[], derivedViews}`. `DocumentCommand {do(doc,kernel), undo(doc,kernel)}`. Two-tier history: factory-local step undo (snapshot policy from spec) + document command stack. `History` class.
12. `//#region 🖼️Display` — renderer-neutral display IR: `DisplayPrimitive` (`mesh|curve|point|label|entity-highlight|linear-handle|box-preview`), `DisplayModel {prompt, items, handles, diagnostics}`. Resolved from `DisplaySpec` per state.
13. `//#region 📦Factories` — concrete static factory specs implemented as TS builders that emit JSON-equivalent IR:
    - `boxFactorySpec` (idle → pickFirstCorner → pickSecondCorner → pickHeight → ready → committed; commit emits `cell.createBox`)
    - `extrudeFactorySpec` (selectWire → setDistance → ready → committed; commit emits `wire.extrudeToCell`)
    - `offsetSurfaceFactorySpec` (selectSurface → resolves to Faces via kernel query → setDistance → commit emits `face.offset`)
14. `//#region 🧪Tests` — `if (import.meta.vitest)` block, one `describe` per layer covering: Vec math; Expr evaluator (paths, guards, let); spec parser rejects malformed JSON; TopologyGraph round-trip + manifold detection; InMemoryKernel createBox + cell.volume; DerivedViewService surface classification preserves area, part classification preserves volume; Statechart transient + guarded transitions; boxFactory full run (`start → pointer.down × 2 → set.height → confirm`) yields commit with non-null cell ref; extrude + offsetSurface factories produce expected commits; document undo/redo restores topology; history excludes `pointer.move`.

## `r3f.tsx` regions

1. `//#region 🧲Header`
2. `//#region 🎮InteractionAdapter` — `createR3FInteractionAdapter()` mapping `ThreeEvent<PointerEvent>` → `pointer.move|pointer.down` factory events with `Vec3` worldPoint + modifiers.
3. `//#region 🖼️DisplayAdapter` — renders `DisplayModel` items into R3F: `mesh`→`<mesh>`, `curve`→`<Line>` from drei, `point`→sphere, `label`→`<Html>` from drei, `box-preview`→procedural box mesh, `linear-handle`→drei `<TransformControls>`-style axis, `entity-highlight`→tinted overlay.
4. `//#region ⚛️Hooks` — `useFactoryRuntime(spec, opts)`, `useFactorySnapshot(runtime)` (subscribes via `useSyncExternalStore`), `<FactoryCanvas runtime>`, `<FactoryDisplay runtime>`, `<FactoryInteractionLayer runtime>`.
5. `//#region 🧪Tests` — render `<FactoryDisplay>` with a synthetic snapshot in jsdom, assert children counts per primitive kind; smoke-test `useFactoryRuntime` lifecycle with box factory + InMemoryKernel.

## `play/` demo

Self-contained Vite app launching a box factory in an R3F canvas: pick two corners on the ground plane, drag height, confirm. Side panel shows live `FactorySnapshot` JSON, document operation log, undo/redo buttons. Loads `play/fixture/box.factory.json` via Vite JSON import to prove static-spec loading works end-to-end. Mirrors structure of `elements/lib/react/topology/play`.

## Validation gates

Before closing the ticket:

- `bun nx run @spatial/js:test` → vitest passes (all inline `describe` blocks).
- `bun nx run @spatial/js:build` → vite build of the play app succeeds.
- `bun nx run @spatial/js:dev` then a Playwright smoke (added to `play/e2e/box.spec.ts` only if needed) confirms canvas mounts and a committed box appears.
- No new entries in root `package.json` `dependencies` beyond workspace pointers; no new pnpm/npm-only deps.

## Ticket / process

- Open via repo MCP `ticket_open` under the most-fitting goal from `repo://goals` (titled "Spatial JS First Runtime"). All scratch artifacts (debug logs, sketches) under `.repo/🎫/.../spatial-js-first-runtime/`.
- Single agent (this one) executes; if scope reveals >1h of independent work blocks (e.g. R3F adapter polish vs core kernel), spawn one parallel generalist subagent per block, all editing the existing monolith files.
- Close via `ticket_close` with file list.

## Out of scope (deferred)

- Real brep kernel (brepjs adapter): explicitly skipped — `InMemoryKernel` is sufficient for the first runtime since `KernelAdapter` is the boundary.
- WASM extension hatch, remote kernel RPC, multi-quality preview cache: interfaces stubbed but only `fast-mesh` quality implemented.
- Visual statechart authoring / Stately integration: factory specs authored as TS builders that emit JSON.
- Spatial Python / Rust runtimes (`spatial/py`, `spatial/rs`): not part of this ticket.
