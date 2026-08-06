# CORE State Violations Inventory (baseline)

**Scope:** authoritative / session / host-owned state outside `🧰️framework/🛍️products/💻️os/**`.  
**Checklist:** unchecked = still violates OS-exclusive state authority. Paths repo-relative.

## Rust — ✏️s modules

- [x] `✏️s/🔨️modules/◻2d/🗄️store/🦀️component.rs` — `DrawingStore` — 2D drawing document/session store outside OS `DocumentStore`
- [ ] `✏️s/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs` — `BrepkitKernel` — registry inside host-owned session only (seq minting removed Wave 2 🧊3d; document-op derive pending)
- [ ] `✏️s/🔨️modules/🧊️3d/📐️brep/🏟️arena/🦀️component.rs` — `Store` — generational arena OK inside engine compute / cache entry (documented Wave 2 🧊3d)
- [ ] `✏️s/🔨️modules/🧊️3d/📐️brep/🕸️topology/🦀️component.rs` — `Body` — compute-scoped topology payload (documented Wave 2 🧊3d)
- [ ] `✏️s/🔨️modules/🧊️3d/📐️brep/📜️history/🦀️component.rs` — `LabelSource` — lives only inside `Body` (documented Wave 2 🧊3d)
- [ ] `✏️s/🔨️modules/🧊️3d/🥽️mesh/🦀️component.rs` — `HalfedgeMesh` — value payload, not global mesh authority (documented Wave 2 🧊3d)
- [ ] `✏️s/🔨️modules/💭️mindmap/🧩️extension/🦀️component.rs` — `DefaultMindmapExtension` — mindmap graph/canvas extension holding document-adjacent state

## Rust — ✏️s plugins (engines / globals / hosts)

- [x] `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs` — `CAD_BREP_HOST` — `BrepEngineHost` replaces `CAD_BREP_KERNEL` (Wave 2 🧊3d; `OnceLock` until M3 injection)
- [x] `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs` — `PROCESS_BREP_HOST` — `BrepEngineHost` replaces `PROCESS_BREP_KERNEL` (Wave 2 🧊3d)
- [ ] `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/⚙️engine/⏳️session/🦀️component.rs` — `Puzzle3dEngine` — headless 3D puzzle engine session state
- [ ] `✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️kernel.rs` — `SimulationState` — building energy simulation timestep state
- [ ] `✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️meters.rs` — `MeterStore` — utility meter readings store
- [ ] `✏️s/🔌️plugins/🔋️energy/⚙️engine/🦀️output.rs` — `TimeSeriesStore` — simulation time-series output buffer
- [ ] `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/⚙️engine/🦀️component.rs` — `ImperativeHost` — imperative script host owning program state
- [ ] `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/⚙️engine/🦀️component.rs` — `SequenceHost` — sequence/timeline host JSON state
- [ ] `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🌍️world/🦀️component.rs` — `TrinityHost` — trinity rewrite world host state
- [ ] `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs` — `FlowPlayApp::eval_session` — off-main-thread flow eval session mutex
- [x] `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/⚙️engine/🦀️component.rs` — `FLOW_PLAY_NEURAL_CACHE` — static neural cache for flow play (removed; per-`FlowEvalSession` cache only)
- [ ] `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🦀️component.rs` — `Procedural2dPlayApp::eval_session` — procedural 2D eval session (shared pattern with flow)
- [ ] `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs` — `Procedural3dPlayApp::eval_session` — procedural 3D eval session
- [ ] `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎮️commands/🖱️canvas/🦀️component.rs` — `DrawSession` — draw app transient canvas/session state
- [ ] `✏️s/🔌️plugins/📏️layout/🎛️apps/📏️layout/🌉️wasm/🦀️component.rs` — `LayoutSession` — WASM layout play session owned by plugin
- [ ] `✏️s/🔌️plugins/📕️norm/🫀️core/🦀️component.rs` — `NormHost` — norm evaluation host over projection state
- [x] `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🦀️component.rs` — `PRESENCE_PEERS` — static presence peer registry (moved to `SpaceApp::presence_peers`)
- [x] `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/🦀️component.rs` — `STUDIO_PORTS` — static OS backbone port map for studio (moved to `HomeApp::studio_ports`)
- [ ] `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/⚙️engine/🎬️core/🦀️component.rs` — `BasicScene` — animate/present scene graph state
- [ ] `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/⚙️engine/🏃️motion/🦀️component.rs` — `MultiObjectTracker` — remodel motion tracker internal state

## Rust — 🧰️framework (excluding OS product)

- [ ] `🧰️framework/🔨️modules/✍️editor/🦀️component.rs` — `EditorHost` — text editor host caret/selection GPU session state
- [ ] `🧰️framework/🔨️modules/🗺️surface/🗺️tiled-map/🦀️component.rs` — `MapHost` — tiled map surface host state
- [ ] `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🦀️component.rs` — `RasterHost` — raster paint surface host state
- [ ] `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️component.rs` — `GraphHost` — node graph surface host state
- [ ] `🧰️framework/🔨️modules/🗺️surface/🎲️board-2d/🦀️component.rs` — `BoardSession` — 2D board WASM session + GPU binding
- [ ] `🧰️framework/🔨️modules/🗺️surface/🏔️terrain/🦀️component.rs` — `TerrainSessionState` — terrain edit session state
- [ ] `🧰️framework/🔨️modules/🧩core/🎯️action-bus/🦀️component.rs` — `ActionBus` — framework action routing bus state
- [ ] `🧰️framework/🔨️modules/🧩core/🖥️platform/🦀️component.rs` — `Platform` — platform notification / host chrome bridge state
- [ ] `🧰️framework/🔨️modules/🧮️math/🧩️wfc/🌐️domain/🦀️component.rs` — `DomainStore` — WFC constraint domain store (algorithmic CORE)

## TypeScript — ✏️s CAD / spatial

- [ ] `✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts` — `AttributeStore` — CAD entity attribute table with revision bumps
- [ ] `✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts` — `ActionRegistry` — registered spatial/document actions catalog
- [ ] `✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts` — `InteractionRegistry` — interaction spec registry for hosts
- [ ] `✏️s/🔌️plugins/📐️cad/🔨️modules/🫀️core/🟦️component.ts` — `InteractionRuntime` — live interaction session + commit/query state
- [ ] `✏️s/🔌️plugins/📐️cad/🔨️modules/🎰️stately/🟦️component.ts` — `StatelyStateEngine` — XState-backed interaction state machine
- [ ] `✏️s/🔌️plugins/📐️cad/🔨️modules/📐️brepjs/🟦️component.ts` — `BrepjsKernel` — TS B-rep kernel implementing `SpatialKernel`

## TypeScript — 🧰️framework UI chrome

- [ ] `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` — `localStorage` keys (`UI_CHROME_*`, compute workers, intro flags) — persisted UI chrome outside OS store
- [ ] `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` — theme snapshot storage (`UI_CHROME_THEME_*`, `UI_CUSTOM_THEMES_*`) — theme authority in browser storage
- [ ] `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🫀️core/🐚️ShellScope/🟦️component.tsx` — `ShellScope` / `SelectionModeStore` — per-shell selection scope state
- [ ] `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵Tree/🟦️component.tsx` — `Tree` — tree UI with shell-scoped drag/selection coupling
- [ ] `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🫀️core/🚗️UiDriver/🟦️component.tsx` — `UiDriver` — chrome driver profile + storage-backed customization
