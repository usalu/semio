# 🗺️ Registrar Handoff — Framework Surface Family Crate Consolidation

Consolidated `🧰️framework/🔨️modules/🗺️surface/`'s 5 implementation dirs (paint, board-2d, terrain,
node-graph, tiled-map) into ONE crate `semio-framework-surface` at `📦️packages/🦀️rust/` (Shape V2,
`role = "framework"`), with ONE wasm-bindgen wrapper `@semio-tech/framework-surface-rs`
(`wasmBaseName = "framework_surface"`). Every domain is now `pub mod {paint,board_2d,terrain,node_graph,tiled_map}`
inside this one crate (`#[path]`-wired to each domain's `🦀️component.rs`, mirroring `🧮️math/📦️packages/🦀️rust/📦️lib.rs`'s convention).

The 5 old implementation dirs (`.../🎨️paint/⚡️implementations/🦀️rust`, etc. — Cargo.toml, lib.rs,
script.ts, package.json, project.json, pkg/) were deleted. Their per-domain `pkg/` wasm-pack wrappers
(`@semio-tech/framework-surface-{paint,board-2d,terrain,node-graph,tiled-map}-rs`) no longer exist;
everything consumes `@semio-tech/framework-surface-rs` going forward.

**Every item below is OUTSIDE `🗺️surface/**` (this ticket's exclusive ownership) and was intentionally
left unedited** per this session's explicit instruction: "update renderer-react import sites in lockstep
IF they live under your ownership; if import sites are outside surface/, list them in registrar-handoff
instead of editing." None of these files were touched.

## 1. Root `Cargo.toml` — workspace members (NEVER touch, per constraint)

Replace these 5 lines (currently dead paths):
```
    "🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/⚡️implementations/🦀️rust",
    "🧰️framework/🔨️modules/🗺️surface/🏔️terrain/⚡️implementations/🦀️rust",
    "🧰️framework/🔨️modules/🗺️surface/🎲️board-2d/⚡️implementations/🦀️rust",
    ...
    "🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/⚡️implementations/🦀️rust",
    ...
    "🧰️framework/🔨️modules/🗺️surface/🎨️paint/⚡️implementations/🦀️rust",
```
with ONE line:
```
    "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust",
```

## 2. renderer-react TS import sites (outside `surface/`, not edited)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/WasmSessionLoader/🟦️component.tsx`
  `ENGINE_SESSION_IMPORTERS` (5 entries) all import distinct old packages:
  ```
  "node-graph": () => import("@semio-tech/framework-surface-node-graph-rs"),
  "paint-2d": () => import("@semio-tech/framework-surface-paint-rs"),
  "tiled-map": () => import("@semio-tech/framework-surface-tiled-map-rs"),
  terrain: () => import("@semio-tech/framework-surface-terrain-rs"),
  "board-2d": () => import("@semio-tech/framework-surface-board-2d-rs"),
  ```
  Replace all 5 with the single package; session classes are unique (`GraphSession`, `RasterSession`,
  `MapSession`, `TerrainSession`, `BoardSession`) so `createEngineSession` itself needs no other change:
  ```
  "node-graph": () => import("@semio-tech/framework-surface-rs"),
  "paint-2d": () => import("@semio-tech/framework-surface-rs"),
  "tiled-map": () => import("@semio-tech/framework-surface-rs"),
  terrain: () => import("@semio-tech/framework-surface-rs"),
  "board-2d": () => import("@semio-tech/framework-surface-rs"),
  ```

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
  `dependencies` lists all 5 old `@semio-tech/framework-surface-*-rs` entries — collapse to one:
  `"@semio-tech/framework-surface-rs": "workspace:*"`.

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
  `alias` array has 5 entries pointing each old package at `wasmEngineStub` — collapse to one entry for
  `@semio-tech/framework-surface-rs`.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/⚙️vite.config.ts`
  - `FRAMEWORK_ENGINE_OPTIMIZE_DEPS_EXCLUDE` includes `"@semio-tech/framework-surface-node-graph-rs"` and
    `"@semio-tech/framework-surface-board-2d-rs"` — replace both with one
    `"@semio-tech/framework-surface-rs"` entry.
  - alias entry `{ find: "@semio-tech/framework-surface-board-2d-rs", replacement: .../🎲️board-2d/⚡️implementations/🦀️rust/pkg }`
    → update to `{ find: "@semio-tech/framework-surface-rs", replacement: .../🗺️surface/📦️packages/🦀️rust/pkg }`.

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📜️script.ts`
  `buildEngineWasm()` hardcodes two separate builds:
  ```
  const graphScript = join(repoRoot, "./🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/⚡️implementations/🦀️rust/📜️script.ts");
  if (runCmdStatus("bun", [graphScript, "wasm"], ...) !== 0) throw new Error("framework-surface-node-graph wasm build failed");
  ...
  const boardScript = join(repoRoot, "./🧰️framework/🔨️modules/🗺️surface/🎲️board-2d/⚡️implementations/🦀️rust/📜️script.ts");
  if (runCmdStatus("bun", [boardScript, "wasm"], ...) !== 0) throw new Error("framework-surface-board-2d wasm build failed");
  ```
  Collapse to ONE build of the consolidated crate:
  ```
  const surfaceScript = join(repoRoot, "./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts");
  if (runCmdStatus("bun", [surfaceScript, "wasm"], ...) !== 0) throw new Error("framework-surface wasm build failed");
  ```
  (`engineWasmScriptPath()` in the same file needs no change — once the plugin `engines` entries in §3
  point at `./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust`, it resolves the sibling `📜️script.ts` directly.)

## 3. Plugin Cargo.toml files (never touch — plugin directories forbidden)

`[[package.metadata.semio.playground]]` `engines = [...]` rows still list the 5 dead paths:
```
✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:69   engines = ["./🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/⚡️implementations/🦀️rust"]
✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:22            engines = ["./🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/⚡️implementations/🦀️rust"]
✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:29            engines = ["./🧰️framework/🔨️modules/🗺️surface/🏔️terrain/⚡️implementations/🦀️rust"]
✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml:20         engines = ["./🧰️framework/🔨️modules/🗺️surface/🎨️paint/⚡️implementations/🦀️rust"]
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:32         engines = ["🧰️framework/🔨️modules/🗺️surface/🎲️board-2d/⚡️implementations/🦀️rust"]  (no leading "./")
```
All 5 → `engines = ["./🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust"]` (puzzle keeps its own no-leading-`./` convention if that matters to its own tooling; verify).

`gis`'s Cargo.toml also has real Rust `[dependencies]` path entries (declared but not `use`d anywhere in
its `.rs` sources today — safe mechanical rename):
```
framework_surface_tiled_map = { path = "../../../../../🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-surface-tiled-map", default-features = false }
framework_surface_terrain = { path = "../../../../../🧰️framework/🔨️modules/🗺️surface/🏔️terrain/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-surface-terrain" }
```
→ collapse to one dependency: `framework_surface = { path = "../../../../../🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust", package = "semio-framework-surface" }`
(note: `default-features = false` was set on the tiled-map line only — re-check whether that still applies once merged; the new crate has no optional features today, so it's a no-op either way).

## 4. Other non-plugin Rust consumers (outside `surface/`, not edited)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
  (lines ~22, ~30) declares path deps on old `node-graph`/`tiled-map` crates but neither is `use`d in that
  target's `.rs` sources today (grep-confirmed) — safe mechanical rename to one
  `framework_surface = { path = "…/🗺️surface/📦️packages/🦀️rust", package = "semio-framework-surface" }`.

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/⚡️implementations/🦀️rust/🌍️world/Cargo.toml` (line ~16)
  depends on old `framework_surface_terrain`, package `semio-framework-os-kernel-surface-terrain`.
  **This one IS actually used in code** — `📦️lib.rs:4`: `use framework_surface_terrain::TerrainSessionCore;`.
  Needs BOTH the Cargo.toml dependency swap (`package = "semio-framework-surface"`, same relative depth,
  path now `…/🗺️surface/📦️packages/🦀️rust`) AND the `use` statement updated to
  `use semio_framework_surface::terrain::TerrainSessionCore;` (crate ident is the Cargo-normalized
  `semio-framework-surface` → `semio_framework_surface`, with `TerrainSessionCore` now nested one level
  under the `terrain` module instead of at the old crate root).

## 5. Pre-existing BLOCKING bug found during verification (unrelated framework family, out of scope)

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/⚡️implementations/🦀️rust/Cargo.toml`
(the `infinite_board_port_directed_dag` crate, a dependency of `node_graph` in the new surface crate)
has 4 stale path dependencies left over from math's own family consolidation (`🧮️math`'s graph sub-crates
were merged into `semio-framework-math` at `🧮️math/📦️packages/🦀️rust`, mirroring what this ticket just
did for surface, but the dag crate's manifest was never updated):
```
mathematical_graph_manifest = { path = "…/🔨️modules/🧮️math/🕸️graph/🛂️manifest/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-math-graph-manifest" }
mathematical_graph_dsl = { path = "…/🔨️modules/🧮️math/🕸️graph/🗣️dsl/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-math-graph-dsl" }
mathematical_graph_drawing = { path = "…/🔨️modules/🧮️math/🕸️graph/🖊️drawing/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-math-graph-drawing" }
mathematical_graph = { path = "…/🔨️modules/🧮️math/🕸️graph/⚡️implementations/🦀️rust", package = "semio-framework-os-kernel-math-graph" }
```
`🧮️math/🕸️graph/⚡️implementations/🦀️rust` (the base one) no longer exists at all
(`semio-framework-os-kernel-math-graph-manifest`/`-dsl`/`-drawing` still exist standalone as of this
session, but flickered in/out during verification — a concurrent math-family session appears to be
mid-migration). All 4 should become submodule imports of the single `semio-framework-math` crate
(mirrors `🧮️math/📦️packages/🦀️rust/📦️lib.rs`'s `pub mod graph { … pub mod manifest; pub mod dsl (n/a — not wired yet); pub mod drawing; … }`), e.g.
`mathematical_graph = { path = "…/🧮️math/📦️packages/🦀️rust", package = "semio-framework-math" }` with code
updated to `semio_framework_math::graph::…`. **This blocks `cargo check` on the new
`semio-framework-surface` crate today** (via `node_graph`'s dependency chain) — not something this ticket
can fix (`♾️infinite/🎲️board/…` is a different framework family, explicitly out of exclusive ownership).
Flagging for whoever owns that family / the math consolidation ticket.

## 6. Environment note

The repo was observed to be genuinely volatile during this session — multiple concurrent sessions appear
to be actively restructuring `🖱️ui` (wgpu target's `build.rs`/`Cargo.toml` disappeared and reappeared
several times), `🧮️math` (graph sub-crate dirs likewise), and this exact ticket's own
`🗺️surface/**` implementation dirs (deleted by this session, observed recreated once by another
session, re-deleted). `cargo check` runs were repeatedly interrupted by these unrelated dirs going
missing mid-build; see §5 for the one that is a real, stable (not transient) blocker. If the old
`🗺️surface/{paint,board-2d,terrain,node-graph,tiled-map}/⚡️implementations/` dirs are present again at
merge time, they must be deleted — the source of truth is now
`🗺️surface/{paint,board-2d,terrain,node-graph,tiled-map}/🦀️component.rs` +
`🗺️surface/📦️packages/🦀️rust/`.
