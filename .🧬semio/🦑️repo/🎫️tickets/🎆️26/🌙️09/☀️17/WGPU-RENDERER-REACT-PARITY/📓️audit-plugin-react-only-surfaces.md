# React-only UI surface census — wgpu parity gaps beyond the core Interpreter/element matrix

Lane: **complete census of every React-only UI surface across the repo** (ticket
`26/09/17/WGPU-RENDERER-REACT-PARITY`). Companion to `📓️audit-interpreter-elements.md` (which covers the
19 `ui_contract::Component`/wgpu `UiNode` kinds and layout rules in depth — not repeated here) and
`📓️audit-scenes-engine-canvas.md` (pre-Wave-1 `SurfaceKind` dispatch baseline). Read-only, no code
changed. Status.md's own Wave 2 plan already lists `W2f: plugin editors with no wgpu target (CAD
spatial tree + census)` — this report IS that census, and it finds a second item (`🎞️animate`) status.md
did not yet name.

## 1. Method

Four parallel read-only passes: (a) direct `find`/`wc -l`/`python3` verification of every plugin and
product directory for `.tsx`/`.rs` presence (done directly, not delegated — see §2); (b) full
directory-level census of the ~35 `🧱️elements/*` under the os renderer engine; (c) full census of the
~60 `🧱️elements/*` under the framework `🖱️ui` module; (d) `SurfaceKind`/`ExternalSlot` enumeration vs
current (post-Wave-1) wgpu dispatch reach; (e) React-only chrome outside `🧱️elements/` (storybook-only
widgets, `🖱️ui/🔨️modules/*`). (b)–(e) are separate agent passes, folded in below.

Emoji-path note (recurring across this ticket's audits): BSD `grep` on this machine silently returns
zero output — not an error — on some emoji-laden `.tsx` files. Every "zero hits" claim below was
produced with `find`/`python3 -c "open(...).read()"` or cross-checked that way.

## 2. Plugin- and product-level React-only surfaces (directly verified, exhaustive)

**Method:** `find "✏️s/🔌️plugins" -name "🟦️.tsx" -not -path "*/node_modules/*"` and the same for
`🧰️framework/🛍️products/*/`, then inspected every hit's parent directory tree directly. This is a
complete enumeration, not a sample — of 33 plugins under `✏️s/🔌️plugins/*`, only **three** contain any
`.tsx` at all outside `node_modules`/tests: `🎞️animate`, `📐️cad`, `🧩️puzzle`. No plugin has a
`🧱️elements/` or `🎯️targets/⚛️react/` folder directly under its own root — React code lives deep inside
a specific artifact-standard's `✏️editor/` subtree instead, a different layout than the shared
`🧱️elements/<Name>/🎯️targets/{⚛️react,🧊️wgpu}` co-location pattern used by the os renderer/framework `ui`
elements (ticket `26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE`). Of 5 products under
`🧰️framework/🛍️products/*`, only **`💻️os`** (already exhaustively covered by the companion audits) and
**`🎤️presentation`** ship any React UI; `🖥️server`, `🦑️repo`, `📓️print` have zero `.tsx` anywhere
(confirmed: `find ... -iname "*.tsx"` returns nothing) — genuinely out of scope, not gaps.

| # | Surface | React path (lines) | wgpu path | Reached via | Reusable Rust | Size |
|---|---|---|---|---|---|---|
| 1 | **CAD spatial-tree editor** (the one the interpreter audit already flagged) | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️.tsx` — **6 766 lines**, one file, no `⚛️react` subfolder (tsx sits directly in `📺️renderer/`) | **ABSENT** — `find ... -iname "*wgpu*"` under the whole `📐️cad` plugin returns zero hits, confirmed twice | Not through `resolveComponentSceneHost`/`ExternalSlot`/anything in `🗣️Interpreter`, `🔌️PluginRuntime`, or `🪪️WasmSessionLoader` (`ImportedSlot` as a term does not exist anywhere in this codebase — grepped/python-scanned all three files, zero hits). Per-artifact-standard editors like this one are their own Nx-buildable TS package (`✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript`, pkg `@semio-tech/cad-js`) mounted as a dedicated editor app/route outside `UiDocumentStore`/`ui_contract` entirely — confirmed by the interpreter audit's Checkbox finding ("bypasses `UiDocumentStore`/`Interpreter`/`ui_contract` entirely") and independently here (no reference to this renderer path found anywhere via package-name grep either). | **Minimal.** Of the CAD editor's 9 `⚙️engine/*` submodules only `⚙️engine/🦀️.rs` (top-level, 2 files) and `🕹️interaction/` (2 files) are Rust — `🗿️artifact`, `🧬️typology`, `🎬️actions`, `🎰️stately`, `📺️renderer`, `📔️registry`, `🏃️runtime` all have **0** `.rs` files (pure TS state machine + registry + renderer). Porting to wgpu means porting the *engine* first, not just adding a paint layer. | **XL** — near-greenfield; this is the biggest single packet in the whole census |
| 2 | **Animate presentation-deck editor** (NOT previously named in status.md's Wave 2 plan — new finding) | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/` — `🟦️.tsx` **5 559** + `🎨️.css` **1 406** + `🔨️modules/🔌️pdf-canvas-port` **122** + `🔨️modules/📝️markdown-html-compiler` **366** = **~7 453 lines** | **ABSENT** — `find ... -iname "*wgpu*"` under the whole `🎞️animate` plugin returns zero hits | Same architecture as CAD: own Nx TS package, own editor route, outside `ui_contract` | **High reuse, unlike CAD.** `⚙️engine/` has **34 real `.rs` files** across `⏱️rate`(3), `🎥️video`(7), `🎞️animation`(3), `🔤️text`(3), `🎬️scene`(4), `📐️geometry`(4), `🎛️config`(4), `📷️camera`(3) plus the top-level `🦀️.rs` — the whole animation/scene/camera/timeline *model* is already Rust. Only the **paint layer** (this renderer file) is React/DOM-only (canvas 2D draws + PDF/markdown compile helpers). | **L** — real work, but "add a wgpu paint layer over an existing Rust engine," not a re-architecture |
| 3 | **Puzzle 5D visualization target** | `✏️s/🔌️plugins/🧩️puzzle/🎯️targets/⚛️5d-react/🟦️.tsx` (638) + `📦️packages/🟦️typescript/🟦️.tsx` (2) = **640 lines** | **ABSENT** — no `wgpu` hit anywhere under `🧩️puzzle` | Standalone `🎯️targets/⚛️5d-react` sibling target (parallel to how puzzle2d/puzzle3d each have their own react+wgpu boot targets per project memory — but 5D has no wgpu sibling target at all) | **Very high reuse** — `🗿️artifacts/🖐️5d/` already has **335 `.rs` files** (the puzzle engine core, shared lineage with 2d/3d). The React file is a thin visualization shim over an already-complete Rust artifact. | **S** |
| 4 | **Presentation product deck viewer** (product-level, distinct file from #2 — verified by `md5`/`diff`, not a duplicate) | `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` **9 415** + `🎨️.css` **1 406** (pkg `@semio-tech/presentation-react`) | **ABSENT** — the only thing under this product's own `🎯️targets/` is `⚛️react` | Standalone product, own route/bundle, same non-`ui_contract` architecture | Shares the `🔌️pdf-canvas-port`/`📝️markdown-html-compiler` helper modules with #2 (same lineage, different consuming app) — otherwise unaudited | **L**, but **flag for scope confirmation** — a read-only kiosk/export deck-viewer may be intentionally React/DOM-only forever (same category as `📓️print`'s PDF output), not necessarily a wgpu-parity target. Worth asking the ticket owner before sizing a packet. |

**Everything else checked and cleared:** every other plugin (`🔱️trinity`, `📸️remodel`, `🖨️raster`,
`🌊️flow`, `🏭️process`, `📕️norm`, `🎪️demonstrator`, `🧱️block`, `🕸️dag`, `🗄️stdio`, `💡️reasoning`,
`🎬️sequence`, `✒️writer`, `🪐️space`, `🌀️procedural`, `🌿️vcs`, `🌍️gis`, `📜️imperative`, `🪵️sourcing`,
`🗒️note`, `📋️forms`, `🏛️architect`, `🎥️shooting`, `➗️mathematical`, `📏️layout`, `🏗️fem`, `🖍️draw`,
`📖️playbook`, `💠️lowpoly`, `🔋️energy`, `🗟️artifacts`) has **zero** `.tsx` anywhere outside
`node_modules` — these render exclusively through the shared `ui_contract`/wgpu `Scenes` path (per
project memory: puzzle2d/3d, generation3d, trinity, note, layout, forms, raster, energy all have
documented React **and** wgpu boot recipes), so they are not React-only surfaces and are correctly out
of this census's scope.

## 3. `🧱️elements/*` under the os renderer engine (35 directories)

Base path for every row: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/<Element>/`.
`🗣️Interpreter/🟦️.tsx`'s `resolveComponentSceneHost` (switch, lines 358-391) dispatches 15 scene-kind
strings (`canvas-2d, world-3d, node-graph, text-editor, table, paint-2d, tiled-map, board-2d,
icon-render, ink-canvas, graph-timeline, block-list, diff-view, event-feed`) to a host component, with
`SURFACE_KIND_SCENE_FIELD` (lines 408-424) as the matching wire-tag→field map; `default: return
undefined` for anything else. Elements not in that registry are instead imported directly by
`🏛️ShellHost/🟦️.tsx` or `🐚️Shell/🟦️.tsx` — chrome, not scene-kind, components.

| Element | React path (lines) | wgpu path (lines) or ABSENT | How reached | Reusable Rust engine | Size |
|---|---|---|---|---|---|
| ⚙️EngineCanvas | ABSENT (no top-level .tsx) | `🎯️targets/🧊️wgpu/🦀️.rs` (5358) | N/A — reverse gap, this **is** the shared paint engine (§5) | — | N/A |
| ✏️TextEditor | `🟦️.tsx` (769) | ABSENT | scene registry `"text-editor"` → `TextEditorHost` | wasm session (`WasmSessionLoader`) | M |
| 🌉️ProgramBridge | ABSENT | `🎯️targets/🧊️wgpu/🦀️.rs` (1107) | N/A — reverse gap | — | N/A |
| 🌐️World3dHost | `🟦️.tsx`+`⏯️tool-run-trace` = **8071** | ABSENT | scene registry `"world-3d"` → `World3dHost` | heavy wasm/wgpu-adjacent — talks to native engine already | L — largest file in the whole census |
| 🌳️GraphTimelineHost | `🟦️.tsx` (96) | ABSENT | scene registry `"graph-timeline"` | none | S |
| 🎛️UtilityTree | `🟦️.tsx` (444) | ABSENT | NOT in scene registry — mounted directly by `ShellHost/🟦️.tsx:648` | none | M |
| 🎞️Scenes | ABSENT | `🎯️targets/🧊️wgpu/🦀️.rs` (6940) | N/A — reverse gap, scene-composition engine (§5) | — | N/A |
| 🏛️ShellHost | **13 777** across `🟦️.tsx`(11059)+12 submodules | ABSENT | mounted by `🐚️Shell/🟦️.tsx:69` — Shell itself has an 18k-line wgpu port but its main child **ShellHost does not** | imports UiDocumentStore/ChromePanels/PluginRuntime/ShellSync/ShellSearch/UtilityTree | L — core app chrome, huge |
| 🐚️Shell | `🟦️.tsx` (1283) | `🎯️targets/🧊️wgpu/🦀️.rs` (18142) | both exist | — | — |
| 💬️AgentChatPanel | `🟦️.tsx` (30) | `🎯️targets/🧊️wgpu/🦀️.rs` (65) | both exist (both thin — wgpu side flagged as possible stub) | — | — |
| 📃️UiDocumentStore | 866 (`🟦️.tsx`633+2 submodules) | ABSENT | NOT in scene registry — mounted by `ShellHost/🟦️.tsx:383` | — | M |
| 📊️Table | `🟦️.tsx` (280) | ABSENT | scene registry `"table"` | — | M |
| 📌️ChromePanels | `🟦️.tsx` (1408) | ABSENT | NOT in scene registry — mounted by `ShellHost/🟦️.tsx:618` | — | L |
| 📐️Canvas2dHost | 1381 (`🟦️.tsx`978+`⏯️tool-run-trace`141+`GumballOverlay`262) | ABSENT | scene registry `"canvas-2d"` | wasm hit | L |
| 📡️EventFeedHost | `🟦️.tsx` (153) | ABSENT | scene registry `"event-feed"` | — | S |
| 📤️SegmentedDownload | `🟦️.ts` (170) — **no `.tsx` at all, not a React component** | ABSENT | not wired into react barrel/Shell/ShellHost — only its own tests reference it | — | S, questionable scope — likely pure-logic helper, not a UI surface |
| 🔄️ShellSync | `🟦️.tsx` (106) | ABSENT | NOT in scene registry — mounted by `ShellHost/🟦️.tsx:643` | — | S |
| 🔌️PluginRuntime | 4475 (`🟦️.tsx`4343+2 submodules) | ABSENT | NOT in scene registry — imported by `ShellHost/🟦️.tsx:619,649` and `Shell/🟦️.tsx:71` | **is** the wasm-loading layer other hosts depend on | L |
| 🔎️ShellSearch | `🟦️.tsx` (225) | ABSENT | NOT in scene registry — mounted by `ShellHost/🟦️.tsx:647` | — | M |
| 🔗️AgentBridge | `🟦️.tsx` (476) | `🎯️targets/🧊️wgpu/🦀️.rs` (585) | both exist | — | — |
| 🔺️DiffViewHost | `🟦️.tsx` (233) | ABSENT | scene registry `"diff-view"` | — | M |
| 🕸️NodeGraph | `🟦️.tsx` (3733) | ABSENT | scene registry `"node-graph"` | strong wasm/wgpu-adjacent hits | L |
| 🖋️InkCanvasHost | `🟦️.tsx` (1609) | ABSENT | scene registry `"ink-canvas"` | none | L |
| 🖌️Paint2dHost | `🟦️.tsx` (697) | ABSENT | scene registry `"paint-2d"` | wasm hit | M |
| 🖥️Board2dHost | 1563 (`🟦️.tsx`1514+`⏯️tool-run-trace`49) | ABSENT | scene registry `"board-2d"` | same engine `⚙️EngineCanvas`'s wgpu tests target | L |
| 🖼️IconRenderHost | `🟦️.tsx` (74) | `🎯️targets/🧊️wgpu/🦀️.rs` (95) | both exist (both thin — possible stub, flagged for manual check) | — | — |
| 🗣️Interpreter | `🟦️.tsx` (2274) | `🎯️targets/🧊️wgpu/🦀️.rs` (2515) | both exist — this is the dispatch registry file itself | — | — |
| 🗺️WorldTerrainLayer | `🟦️.tsx` (248) | ABSENT | NOT in scene registry despite being re-exported via the react barrel — likely mounted as an overlay layer inside World3dHost, not a standalone scene kind | wasm hit | M |
| 🚦️AgentPresence | `🟦️.tsx` (48) | `🎯️targets/🧊️wgpu/🦀️.rs` (93) | both exist (both thin) | — | — |
| 🛂️SpaceAdministration | `🟦️.tsx` (459) | ABSENT | **not wired anywhere** in react barrel/Shell/ShellHost — only its own dedicated test references it | — | M, but flag as likely orphaned/unreached in the live app |
| 🛠️ShellHelpers | 5725 (`🟦️.tsx`5335+4 submodules) | ABSENT | NOT in scene registry — imported extensively by `Shell/🟦️.tsx:67-160`, a critical Shell dependency | wasm/wgpu-adjacent hits | L |
| 🛰️Dock | ABSENT (no top-level `.tsx`) | `🎯️targets/🧊️wgpu/🦀️.rs` (1847) | N/A — reverse gap | — | N/A |
| 🤖️AgentApprovals | `🟦️.tsx` (160) | `🎯️targets/🧊️wgpu/🦀️.rs` (238) | both exist | — | — |
| 🧩️BlockListHost | `🟦️.tsx` (272) | ABSENT | scene registry `"block-list"` | — | M |
| 🧭️TiledMapHost | `🟦️.tsx` (1262) | ABSENT | scene registry `"tiled-map"` | wasm hit | L |
| 🧵️TaskManager | `🟦️.tsx` (397) | ABSENT | **not wired anywhere** — `PluginRuntime/🟦️.tsx:545`'s own header doc says it is still "registrar-only, unmounted work" — confirmed dead in the live app | — | M, but flag as dead/unmounted, do not size for porting |
| 🪪️WasmSessionLoader | `🟦️.tsx` (394) | ABSENT | NOT in scene registry — imported by `Shell/🟦️.tsx:163`; consumed by nearly every host above as the wasm-session bridge itself | **is** the shared session/engine-bridge layer other elements call into | M, but is core infra, not a leaf UI surface |

**Reverse gaps (wgpu exists, React absent):** ⚙️EngineCanvas, 🌉️ProgramBridge, 🎞️Scenes, 🛰️Dock — these
are engine-internal/composition modules, not standalone UI surfaces; not part of the React-only gap.

**Dead/orphaned (React exists, wgpu absent, AND not reached from any live mount path):**
📤️SegmentedDownload (`.ts` only), 🛂️SpaceAdministration, 🧵️TaskManager (explicitly self-documented as
unmounted). These should be excluded from wave-2 sizing, not treated as parity gaps, until confirmed
live.

**Manually checked, neither is a stub, but one self-documents a real gap:** 🖼️IconRenderHost wgpu (95
lines) is a legitimate CPU-rasterized Lucide icon-atlas implementation — thin because the job is thin,
not a stub. 💬️AgentChatPanel wgpu (65 lines) is also real but **its own header comment states the gap
outright**: the header half (title + presence dot) is real chrome paint, but the chat *transcript* body
is not rendered at all, because React's body is an arbitrary hosted React subtree
(`BasicChatPanel`, via `Tree`'s `emptyState` escape hatch, `📌️ChromePanels/🟦️.tsx:1376-1391`) and "the
wgpu panel pipeline has no such hatch — panel content is exclusively `UiNode`/`UiTree` rendered by the
fixed-credit `MountedLayout` engine — so a transcript cannot be projected until the content-projection
packet lands." This is the exact same root cause as status.md's Wave-2-deferred "panel
content-projection escape hatch" item — confirms it's still open and names `AgentChatPanel`'s transcript
as one of its concrete forcing cases (the module doc also names `RightPanelKind::Chat`).

**Note on the biggest items in this table:** `🏛️ShellHost` (13 777 lines) and `🛠️ShellHelpers` (5725
lines) having no wgpu counterpart at all looks alarming in isolation, but both are already covered by
the companion `📓️audit-shell-window-system.md` and Wave 1 packets W1h/W1i/W1j (panel anchors, window/
dock semantics, overlays) — their *behavior* is being ported piecemeal into `🐚️Shell`'s 18k-line wgpu
file rather than 1:1 file-for-file, so treat this row as a cross-reference to that lane, not a new
undiscovered gap.

## 4. `🧱️elements/*` under the framework `🖱️ui` module (60 directories)

Base path for every row: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/<Element>/`. This is the co-located
`🎯️targets/{⚛️react,🧊️wgpu}` sibling pattern (ticket `26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE`).
"Component/UiNode kind" marks the 19 wire-level `ui_contract::Component` variants (already covered in
depth by `📓️audit-interpreter-elements.md` §2 — not re-litigated here beyond presence/absence); everything
else is chrome composed ad hoc in React with no wire-level identity.

| Element | React path (lines) | wgpu path (lines) or ABSENT | Production-used? | Component/UiNode kind | Size |
|---|---|---|---|---|---|
| ↔️Resizable | `🟦️.tsx` (250) | ABSENT | Yes | ad-hoc chrome | M |
| ↕️Collapsible | `🟦️.tsx` (165) | ABSENT | Yes | ad-hoc chrome | S |
| ⌨️Command | `🟦️.tsx` (438) | ABSENT | Yes — `🔎️ShellSearch` | ad-hoc chrome | M |
| ☑️Checkbox | `🟦️.tsx` (30) | ABSENT | Weak — barrel re-export only, no confirmed direct JSX use | ad-hoc chrome | S |
| ⚙️VirtualFileSystem | `🟦️.tsx` (586) | ABSENT | Yes — os renderer react target | ad-hoc chrome | M |
| ⚡️ActionGroup | `🟦️.tsx` (245) | ABSENT | Yes — used by 🔀️Toggle, react barrel | ad-hoc chrome | M |
| ➖️Divider | **ABSENT (react too)** | `🎯️targets/⌨️tui/🦀️.rs` (20), no wgpu | N/A | Component::Separator | TUI-only orphan — neither web target exists |
| ⭕️Ring | `🟦️.tsx` (218) | `🎯️targets/🧊️wgpu/🦀️.rs` (46) | Yes | Component::Ring | both exist |
| 🆔️ElementId | `🟦️.tsx` (117) | ABSENT | Yes, widely (132 hits) | ad-hoc chrome (utility) | S |
| 🌈️Surface | `🟦️.tsx` (243) | **no per-element wgpu file** — routed centrally through `🖱️ui/🎯️targets/🧊️wgpu/📨️scene_slots/🦀️.rs` | Yes, heavily | Component::Surface | architecture note, not a gap — Surface hosts arbitrary scene renderers so it can't be a self-contained widget |
| 🌳️Tree | `🟦️.tsx` (4736) | `🎯️targets/🧊️wgpu/🦀️.rs` (294) | Yes | Component::Tree/TreeSection/TreeItem | both exist (companion audit already covers the virtualization gap in depth) |
| 🎀️Ribbon | `🟦️.tsx` (103) | ABSENT | Weak — hits are mostly `📓️print` product (likely unrelated name collision) | ad-hoc chrome | S |
| 🎗️UiLabel | `🟦️.tsx` (29) | ABSENT | Yes, across plugin editor panels | ad-hoc chrome | S |
| 🎚️Slider | `🟦️.tsx` (555) | `🎯️targets/🧊️wgpu/🦀️.rs` (61) | Yes | Component::Slider | both exist |
| 🎛️ToggleGroup | `🟦️.tsx` (243) | ABSENT | Weak — taxonomy/fixtures + one CAD-renderer hit | ad-hoc chrome | M |
| 🎨️Canvas | `🟦️.tsx` (2136) | ABSENT | Yes but mostly self-contained window-layout primitive | ad-hoc chrome | L |
| 🎬️Scene | `🟦️.tsx` (2153) | ABSENT | Yes — helper/gizmo lib, react target barrel + 🦴️Skeletons | ad-hoc chrome (geometry helpers) | L |
| 🎴️IconSelector | `🟦️.tsx` (250) | `🎯️targets/🧊️wgpu/🦀️.rs` (30) | Yes | Component::IconSelect | both exist |
| 🏷️Label | `🟦️.tsx` (373) | ABSENT (tui-only, 22 lines) | Yes, very widely (~48 files) | ad-hoc chrome (i18n helper) | M |
| 🐚️ShellScope | `🟦️.tsx` (254) | ABSENT | Yes — os shell schema | ad-hoc chrome | M |
| 👥️PresenceBar | `🟦️.tsx` (151) | `🎯️targets/🧊️wgpu/🦀️.rs` (152) | Yes | not a Component variant — chrome widget with its own wgpu sibling | both exist |
| 💡️ChromeControlHint | `🟦️.tsx` (130) | ABSENT (handled generically by shared `⚡️events`/`🖌️paint`) | Yes | ad-hoc chrome | S |
| 💬️Dialog | `🟦️.tsx` (609) | ABSENT | Yes — `🤖️AgentApprovals` | ad-hoc chrome | M |
| 📃️List | **ABSENT (react too)** | `🎯️targets/⌨️tui/🦀️.rs` (49), no wgpu | N/A | not a Component variant | TUI-only orphan |
| 📊️Table | `🟦️.tsx` (565) | ABSENT (tui-only, 155 lines) | Yes — os `📊️Table`/`🧵️TaskManager` | ad-hoc chrome | M |
| 📋️MenuItem | `🟦️.tsx` (23) | ABSENT | Yes, widely (cad/writer/sequence editors) | ad-hoc chrome | S |
| 📐️Layout | `🟦️.tsx` (153) | ABSENT | Weak — own story + one internal test only | ad-hoc chrome | S |
| 📑️Tabs | `🟦️.tsx` (246) | ABSENT (tui-only, 39 lines) | **No** — only own tests/stories, no product consumer found | ad-hoc chrome | M, but flag as likely storybook/test-only, matches the companion audit's independent "zero production call sites" finding for Tabs |
| 📚️I18n | `🟦️.tsx` (717) | ABSENT | Yes — react target barrel + os ShellHelpers | ad-hoc chrome | M |
| 📜️Scrollable | `🟦️.tsx` (53) | ABSENT | Yes — 🖼️Panel, presentation plugin editor | ad-hoc chrome | S |
| 📝️Field | `🟦️.tsx` (54) | ABSENT | Yes — real use in `🗣️Interpreter/🟦️.tsx` | ad-hoc chrome | S |
| 📨️UIDialog | `🟦️.tsx` (124) | ABSENT | Yes — os ShellHost dialog-origin | ad-hoc chrome | S |
| 📻️TableAvatar | `🟦️.tsx` (90) | ABSENT | Weak — barrel + taxonomy hits only | ad-hoc chrome | S |
| 🔀️Toggle | `🟦️.tsx` (387) | `🎯️targets/🧊️wgpu/🦀️.rs` (32) | Yes | Component::Toggle | both exist |
| 🔌️Ports | `🟦️.tsx`(169)+`📡️interactive-jobs`(130) | ABSENT | Yes — 🕸️Diagram, react target | ad-hoc chrome (scene/host port plumbing) | S |
| 🔑️KeyValue | **ABSENT (react!)** — confirmed repo-wide, not just this dir | `🎯️targets/🧊️wgpu/🦀️.rs` (25) | N/A | **Component::KeyValueList — the one true inverse case: wgpu renders it, no React implementation exists anywhere** | flag for ticket owner, not a normal porting packet |
| 🔘️Button | `🟦️.tsx` (45) | `🎯️targets/🧊️wgpu/🦀️.rs` (35) | Yes | Component::Button | both exist |
| 🔚️Footer | `🟦️.tsx` (45) | ABSENT (tui-only, 26 lines) | Weak — own story + test import list only; real product footers composed ad hoc | ad-hoc chrome | S |
| 🔝️Navbar | `🟦️.tsx` (246) | ABSENT (tui-only, 34 lines) | Yes — os ShellHost, react target | ad-hoc chrome | M |
| 🔣️Icons | `🟦️.tsx` (681) | ABSENT | Yes, widely | ad-hoc chrome (backs IconSelect visuals) | M |
| 🔤️Textarea | `🟦️.tsx` (159) | ABSENT | Yes — os TextEditor | ad-hoc chrome | S |
| 🔲️WindowSilhouette | `🟦️.tsx` (401) | ABSENT | Yes — os Dock/Shell wgpu files reference it, react target | ad-hoc chrome | M |
| 🔳️ButtonGroup | `🟦️.tsx` (155) | ABSENT | Yes — react target, ShellHelpers | ad-hoc chrome | S |
| 🔽️Select | `🟦️.tsx` (825) | `🎯️targets/🧊️wgpu/🦀️.rs` (322) | Yes | Component::Select | both exist (companion audit already covers the keyboard/typeahead gap) |
| 🕰️HistoryTable | `🟦️.tsx` (237) | ABSENT | Yes — os GraphTimelineHost | ad-hoc chrome | M |
| 🕸️Diagram | `🟦️.tsx`(1842)+`📐️layout`(1501) | ABSENT | Weak by JSX-tag grep but re-exported via barrel — needs a deeper check | ad-hoc chrome | L |
| 🖱️ContextMenu | `🟦️.tsx` (1127) | ABSENT | Yes — used by 🌳️Tree and many os hosts (`ContextMenuController`) | ad-hoc chrome | L, but companion audit already rates the underlying context-menu *system* as one of the strongest wgpu-parity areas found (defined once, shared taxonomy) — this React-side file itself has no wgpu twin because wgpu implements the equivalent directly in `🐚️Shell`/`🎞️Scenes`, not as a co-located sibling |
| 🖼️Panel | `🟦️.tsx` (576) | ABSENT | Yes — os ShellHelpers, react target, Layout | ad-hoc chrome | M |
| 🗨️Popover | `🟦️.tsx` (444) | ABSENT | Yes — os ShellSync | ad-hoc chrome | M, companion audit already flags Popover content as P1-missing on wgpu (positioning scaffolding exists, nothing paints content) |
| 🚗️UiDriver | `🟦️.tsx` (278) | ABSENT | Yes — os config schema, UiPreferences | ad-hoc chrome | M |
| 🚧️WindowContentDeadLine | `🟦️.tsx` (121) | ABSENT | Yes — used by 🪟️Window, react target | ad-hoc chrome | S |
| 🦴️Skeletons | `🟦️.tsx` (180) | ABSENT (referenced conceptually only) | Yes — os react target | ad-hoc chrome | S, matches companion audit's "no skeleton/placeholder-shape concept anywhere in wgpu" finding |
| 🧙️Wizard | **ABSENT (react too)** | `🎯️targets/⌨️tui/🦀️.rs` (96), no wgpu | N/A | not a Component variant | TUI-only orphan |
| 🧪️NavbarExampleSelect | `🟦️.tsx` (89) | ABSENT | Yes, oddly (likely unrelated "example select" name collision in other plugins) | ad-hoc chrome, dev-only navbar example picker | S |
| 🧭️PanelTabBar | `🟦️.tsx` (643) | ABSENT | Yes — 🖼️Panel, 📐️Layout | ad-hoc chrome | M |
| 🧱️DragHandle | `🟦️.tsx` (73) | ABSENT | Yes — os BlockListHost, react target | ad-hoc chrome | S |
| 🧾️Form | `🟦️.tsx` (23) | ABSENT | **No** — only own tests/stories, no product consumer found | ad-hoc chrome | S, trivial wrapper, matches companion audit's "trivial on both sides" verdict |
| 🪙️Chip | **ABSENT (react too)** | `🎯️targets/⌨️tui/🦀️.rs` (17), no wgpu | N/A | not a Component variant | TUI-only orphan, matches companion audit's independent "orphaned on the React side too" finding for Chip |
| 🪜️Stepper | `🟦️.tsx` (290) | `🎯️targets/🧊️wgpu/🦀️.rs` (49) | Yes — os ShellHelpers/measure-controls | Component::NumberStepper | both exist |
| 🪟️Window | `🟦️.tsx` (426) | ABSENT (tui-only, 156 lines) | Yes — 🎨️Canvas, react target | ad-hoc chrome | M |
| 🪵️Log | **ABSENT (react too)** | `🎯️targets/⌨️tui/🦀️.rs` (50), no wgpu | N/A | not a Component variant | TUI-only orphan |

**Notable structural findings from this section:**
- **`🔑️KeyValue` is the one true inversion in the whole census**: a real `Component::KeyValueList` wire
  variant with a working wgpu renderer (25 lines) and **zero** React implementation anywhere in the
  repo, confirmed by a repo-wide (not just this-directory) check. Whatever plugin emits
  `Component::KeyValueList` today renders correctly on wgpu and would need a net-new React component to
  render at all — worth flagging to the ticket owner as a React-side gap, not a wgpu one.
- **Five directories are TUI-target-only orphans**, sitting entirely outside the React-vs-wgpu question:
  `➖️Divider`, `📃️List`, `🧙️Wizard`, `🪙️Chip`, `🪵️Log` — each contains only a
  `🎯️targets/⌨️tui/🦀️.rs` file and nothing else, never built for web at all on either target.
- **`📑️Tabs`, `🧾️Form`, and weakly `📐️Layout`/`🔚️Footer`** have React implementations re-exported
  through the public package barrel but no confirmed product consumer (JSX usage found only in their own
  tests/stories) — these corroborate, independently, the companion `📓️audit-interpreter-elements.md` §3
  findings for Tabs/Chip ("zero production call sites... not a wgpu regression"). Not wave-2 porting
  candidates; flag for cleanup instead.
- **`🌈️Surface`** has no per-element wgpu sibling by design — its rendering routes through the shared
  `scene_slots`/`SurfaceKind` machinery (§5) rather than a co-located file, since it hosts arbitrary
  external scene renderers.

## 5. `SurfaceKind`/`ExternalSlot` enumeration vs current wgpu dispatch reach (post-Wave-1, verified against disk)

**Pre-Wave-1 baseline** (from `📓️audit-scenes-engine-canvas.md`, dated before Wave 1 ran): of 15
`SurfaceKind`s, production `render_component_scene_step` only reached **4** (`World3d` direct;
`NodeGraph`/`TiledMap`/`Board2d` via `engine_canvas::sync_engine_scene`) — the other **11** fell through
to a blank themed panel, real paint code stranded behind `#[cfg(test)]` since a 2026-09-08
zero-warnings sweep.

**Current state (directly re-verified against the live `🦀️.rs` files, not wave-report prose):
all 15 `SurfaceKind`s are now dispatch-reachable in production. No ABSENT/placeholder arm remains.**
`render_component_scene_step` (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1385`) dispatches in three tiers:

- **Tier 1, direct retained draw** (lines 1387-1394): `World3d→render_world3d_surface_step` (1388/1619),
  `Canvas2d→render_canvas_2d_step` (1389/1529), `InkCanvas→render_ink_canvas_step` (1390/1559),
  `IconRender→render_icon_render_step` (1391/1584, delegates into `infinite_world::world::render_world_3d`).
- **Tier 2, list-chrome kinds** via `scene_kind_is_list` (line 1327) → `render_list_scene_step` (1506):
  `Table`(1993), `VirtualFileSystem`(6700), `GraphTimeline`(2762), `BlockList`(2288), `DiffView`(2452),
  `EventFeed`(2584).
- **Tier 3, engine-canvas vello-texture kinds**, phase 4 → `engine_canvas::sync_engine_scene`
  (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2918-2926`): `NodeGraph→sync_node_graph_scene`(2920/~2278),
  `TiledMap→sync_tiled_map_scene`(2921/~2451), `Board2d→sync_board2d_scene`(2922/~2696),
  `Paint2d→sync_paint2d_scene`(2923/~2829), `TextEditor→sync_text_editor_scene`(2924/~2858); `_ => false`
  (2925) is the only remaining fallback arm, but nothing reaches it — every other kind is caught by
  Tier 1/2 first.

Two previously-reported blockers are confirmed resolved in the current tree: the `canvas_lum`/
`canvas_set_lum`/`canvas_sat`/`canvas_set_sat` deletion W1a flagged as a W1b compile error, and the
`InkDocumentJson.grid_*` fields W1a's own report said were missing — both present and wired now
(grid fields at `🎞️Scenes/…/🦀️.rs:3880-3883`, read at 5699-5700).

### Extension/"ExternalSlot" — important correction to the task's framing

There is **no per-kind `ExternalSlot` registry** analogous to `SurfaceKind`. React has exactly one
generic `Component::type === "extension"` kind, and **both of its implementations are stubs**, not just
on wgpu:
- `ExtensionView` (`🗣️Interpreter/🟦️.tsx:2122-2130`) unconditionally renders `"Extension unavailable:
  {component.extension}"`.
- Legacy `resolveExternalSlots` (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:505-522`) does the same; its own
  comment (512-518) explicitly calls wiring a contributor's UI body through `AppChannelClient.refreshUi`
  "the dedicated follow-up work package this ticket flags" — i.e. genuinely unimplemented, not a stale
  placeholder someone forgot.
- wgpu has **zero** references to `ExtensionView`/`resolveExternalSlots`/`UiExternalSlotNode` anywhere
  under `🎞️Scenes`/`⚙️EngineCanvas`'s wgpu targets — but there is nothing on the React side to reach
  parity *with* yet, so this is a React/kernel-side gap orthogonal to this ticket's wgpu-parity mandate,
  not a wgpu deficiency.
- Separately: `resolveExternalSlots`/`UiExternalSlotNode`/`ExternalSlot` DO exist as real, wired concepts
  in the **other, older** `🧰️framework/🔨️modules/🖱️ui/` stack (with its own `🎯️targets/🧊️wgpu` — this is
  the "second schema" the companion `📓️audit-interpreter-elements.md` §1 already describes: `ui_contract`
  vs the private wgpu `UiNode`). That stack is pre-migration/legacy relative to the
  `📺️renderer/🧑‍🎨engine` tree this ticket targets — worth a one-line confirmation to whoever owns
  migration status in case it's still load-bearing somewhere live.

The names the task brief listed (`@semio-tech/flow-core`, `GraphSession`, `RasterSession`,
`MapWasmSession`, `EditorWasmSession`, `TerrainSession`) are **not** a separate extension-kind registry —
they are the wasm sessions backing specific `SurfaceKind` scene hosts, loaded via
`🪪️WasmSessionLoader/🟦️.tsx`:

| SurfaceKind | React host | wgpu handling (file:line) | Backing wasm session | Still-open gap |
|---|---|---|---|---|
| canvas-2d | Canvas2dHost | Tier 1, `1389/1529` `render_canvas_2d_step`→`render_canvas_2d`(3435) | — | none found |
| world-3d | World3dHost | Tier 1, `1388/1619` `render_world3d_surface_step` | — | none found (was already production pre-Wave-1) |
| node-graph | NodeGraphHost | Tier 3, `sync_node_graph_scene`(2920/~2278) + label/overlay phases 7/8 (1477-1494) | `FlowWasmSession`/`createFlowSession` (`@semio-tech/flow-core`) — note `createGraphSession`/`GraphSession` is exported by WasmSessionLoader but **never consumed**, likely dead/vestigial | node double-click deliberately dropped from the list-hit path by W1a, not yet re-added to engine_canvas's own input path |
| text-editor | TextEditorHost | Tier 3, `sync_text_editor_scene`(2924/~2858), buffer/gutter/selection via `EditorHost::build_scene()` | `EditorWasmSession` | completions/rename-input/context-menu popups confirmed still test-only (0 production hits, present only under `🧪️tests/`) |
| table | TableHost | Tier 2, `1512/1993` `render_table` | — | none found |
| paint-2d | Paint2dHost | Tier 3, `sync_paint2d_scene`(2923/~2829), shared `RasterHost` compositor (same one React uses) | `RasterWasmSession` | marquee/lasso drag overlay + navigator-pane pan/wheel not wired (single-point pick only) |
| virtual-file-system | VirtualFileSystemHost (special-cased in `renderComponentSceneHost:546-552`, not the switch) | Tier 2, `1513/6700` `render_vfs` (restored from a deleted-outright state, `git show 860e015bf6`) | — | none found |
| tiled-map | TiledMapHost | Tier 3, `sync_tiled_map_scene`(2921/~2451), bespoke pointer dispatch | `MapWasmSession` | none found |
| board-2d | Board2dHost | Tier 3, `sync_board2d_scene`(2922/~2696), bespoke pointer dispatch | — | none found |
| icon-render | IconRenderHost | Tier 1, `1391/1584` → `infinite_world::world::render_world_3d` | — | paints synchronously, no pending/loading/error state vs React's async `iconRenderPort` |
| ink-canvas | InkCanvasHost | Tier 1, `1390/1559` `render_ink_canvas`(5681) | — | none found |
| graph-timeline | GraphTimelineHost | Tier 2, `1514/2762` | — | none found |
| block-list | BlockListHost | Tier 2, `1516/2288`, geometry via `block_list_plan` | — | none found |
| diff-view | DiffViewHost | Tier 2, `1517/2452` | — | none found (React also has no click verb here — matched, not a gap) |
| event-feed | EventFeedHost | Tier 2, `1515/2584` | — | none found (W1a fixed the `{entryId}`→`{surfaceId,id}` payload drift) |
| *(extension)* | `ExtensionView` (stub) | no wgpu reference anywhere | n/a | React/kernel-side stub, see above — not a wgpu gap |

### Cross-cutting gaps that apply to every/most `SurfaceKind` (still open, confirmed against current code)

1. **Surface context menus for every kind** — `scene.menu` (React's per-row `menu.id`/`hits`) has
   **zero** references in `🎞️Scenes/…/🦀️.rs`; `ui_wgpu`'s `OverlayKind`/`open_overlay` scaffolding exists
   but is `pub(crate)` and nothing wires a right-click into it. This is exactly `W2b` in status.md's
   Wave 2 plan — confirmed still needed.
2. **Drag-and-drop** for Table/VFS rows and BlockList palette entries — hit targets carry `drag_data`
   but it's shadowed by the surface catch-all per W1a's own §8 finding; not fixed since.
3. **Pointer modifiers** (shift/ctrl) — `UiEvent::PointerDown/Up/Scroll` still has no modifier fields,
   so VFS multi-select always resolves as plain-replace.
4. Wasm32 `cargo check`/`cargo test` for this crate were not completed during Wave 1 (only native
   compiled clean) and were not run by this read-only audit either — current full-build pass/fail status
   is unverified from research alone; flag for the integrator pass status.md already calls for before
   Wave 2 starts.

**Net effect on this census:** the `SurfaceKind` dispatch skeleton itself is essentially closed (all 15
reachable) — the remaining work in this lane is entirely the cross-cutting interaction gaps above (context
menus, drag-drop, modifiers) plus a handful of per-kind polish items (TextEditor popups, Paint2d
marquee/navigator, NodeGraph double-click, IconRender async state), not missing dispatch. None of this
lane needs a *new* wave-2 packet beyond what status.md's `W2a`/`W2b` already scope — this section mainly
serves to confirm those scopes are accurate against current disk state.

## 6. React-only chrome outside `🧱️elements/`

### 6.1 `🧰️framework/🔨️modules/🖱️ui/🔨️modules/*` vs the `🖱️ui` wgpu target

| Chrome feature | Path (lines) | wgpu twin? | Notes/size |
|---|---|---|---|
| control-keybinding-context | `⌨️control-keybinding-context/🟦️.tsx` (245) | **ABSENT** | App-level hotkey registry/dispatch (`useControlHotkey`, chord matching, `aria-keyshortcuts`). wgpu `events.rs` only handles intra-widget nav keys (Select typeahead/Tab/arrows) — no `EventModifiers`-driven action-registry concept, zero "keybinding" hits. **L** |
| keybinding-persistence | `💾️keybinding-persistence/🟦️.ts` (43) | **ABSENT** | localStorage-backed keybinding overrides, ties into OS `🎚️UiPreferences/🟦️.ts:163` (`keybindingOverrides` field). Zero hits anywhere in wgpu. **S** |
| keybinding-text-interpretation | `🔤️keybinding-text-interpretation/🟦️.ts` (141) | **ABSENT** | Chord parsing/formatting/mod-key resolution (`parseKeybindingChords`, platform meta/ctrl swap). Same gap family as the two above. **M** |
| control-hotkey-presentation | `⌨️control-hotkey-presentation/🟦️.tsx` (42) | **ABSENT (partial)** | Inline hotkey badge; wgpu only folds hotkey text into the tooltip (cites `💡️control-tooltip-presentation/🟦️.ts:16` directly), no standalone badge render. **S** |
| flow-direction-context | `🧭️flow-direction-context/🟦️.tsx` (51) | **ABSENT, self-documented** | `events/🦀️.rs`'s own comment: *"RTL mirroring is not applied — this target carries no per-window flow direction yet."* **M** |
| fuzzy-ranking | `🔎️fuzzy-ranking/🟦️.ts` (129) | **ABSENT** | Command-palette fuzzy search/ranking; zero "fuzzy" hits in wgpu (note: W1c's palette work ported `rankFuzzyItems` into the *Shell* wgpu file per status.md — cross-check before treating this as fully open, may be a naming/location mismatch this pass didn't catch). **M** |
| control-tooltip-presentation | `💡️control-tooltip-presentation/🟦️.ts` (19) | **twin exists**, `⚙️engine/🦀️.rs` cites the source line directly | OK |
| chrome-control-presentation | `🎛️chrome-control-presentation/🟦️.ts` (63) | **twin exists**, `🖥️chrome/🦀️.rs` | OK |
| form-control-presentation | `📝️form-control-presentation/🟦️.ts` (25) | **twin exists but documented gap** — no `focus-visible` distinction (pointer focus shows the same ring as keyboard focus); already tracked as packet **W1o** | OK/tracked |
| status-border-presentation | `🌀️status-border-presentation/🟦️.ts` (47) | **twin exists** — shader-based `comet_alpha` reproduces the CSS spin/pulse keyframes | OK |
| surface-presentation | `🌈️surface-presentation/🟦️.ts` (16) | **twin exists** — glass/veil/scrim in `theme.rs`/`paint.rs` | OK |
| menu-item-presentation | `📋️menu-item-presentation/🟦️.ts` (21) | **twin exists** — `paint_select` hover/highlighted/selected logic | OK |
| shell-floor-presentation | `🏠️shell-floor-presentation/🟦️.ts` (22) | **partial/uncertain** — 6-level `Level` enum exists but same-level double-paint suppression not confirmed verbatim | worth a follow-up, not sized here |
| border-presentation | `📏️border-presentation/🟦️.ts` (16) | **twin exists** — `border_normal`/`border_emphasized`/`border_element` match 1:1 | OK |
| interaction-presentation | `🖱️interaction-presentation/🟦️.ts` (59) | **twin exists** — generic `NodeFlags::HOVERED` covers it, not class-for-class | OK |
| element-identity, class-name-composition(+slot), style-variants | small TS build-time helpers | N/A | pure type/CSS-class-string composition, no rendering concern on either side |

**Biggest single chrome finding:** the keyboard-shortcut cluster — `control-keybinding-context`(245) +
`keybinding-text-interpretation`(141) + `keybinding-persistence`(43) + `control-hotkey-presentation`(42)
= **~471 lines with zero wgpu counterpart** for an app-level hotkey registry/dispatch table (distinct
from W1c's *shell shortcut rows*, which is a fixed list, not a user-remappable registry — confirm the
distinction before assuming W1c already closes this). `flow-direction-context`'s RTL gap is notable for
being *self-documented in the wgpu source itself*, not merely absent.

### 6.2 Storybook-only / dead chrome surfaces

Checked the 10 most demo-suspicious-sounding story components (`UIIntroduction`, `BasicChatPanel`,
`NavbarExampleSelect`, `IconShotFrame`, `CanvasPickMenu`, `SelectionMarquee`, `SortableTreeItems`,
`UnifiedGumball`, `DragHandle`, `ActionDropdown`) — **none are dead**; every one is imported by a real
production element (ShellHost, ShellHelpers, AgentChatPanel, IconRenderHost, World3dHost, NodeGraph,
Paint2dHost, Tree, Scene, Table/PanelTabBar/Canvas, ActionGroup). This repo uses `🎭️<name>/🧪️.story.tsx`
exclusively — no `*.stories.tsx` convention exists. Not an exhaustive trace of all stories, just the
most plausible dead-code candidates; none panned out.

### 6.3 CSS-only visual features with no wgpu equivalent

`🎨️styling/🖌️ui/🎨️.css` has ~350 `@keyframes`, almost all per-icon SVG animations (separate concern from
chrome). Chrome-relevant ones with **no confirmed wgpu equivalent**: `introduction-demo-zoom-in/-out`,
`-trail-draw`, `-press-ripple`, `-press-ripple-diamond`, `-wheel-roll` (all onboarding-tour/`UIIntroduction`
demo visuals — DOM/CSS-only by nature, arguably fine to stay React-only); `celebrate-border-burst`,
`celebrate-border-spin` (a celebratory border effect, zero "celebrate" hits anywhere in wgpu — genuine
small gap). Loading/waiting border spin+pulse **are** already covered (§6.1's `status-border-presentation`
row).

### 6.4 OS product level, outside `🧱️elements/`

Everything chrome-adjacent under `📺️renderer/🧑‍🎨engine/` already lives inside `🧱️elements/` (§3). The one
notable file outside it: `📺️renderer/🧑‍🎨engine/🎚️UiPreferences/🟦️.ts` (163 lines) — persisted-preferences
host adapter (appearance/layout/driver/locale/theme/**keybindingOverrides**); itself N/A for painting
(pure storage adapter) but its `keybindingOverrides` field is exactly the data §6.1's absent wgpu hotkey
dispatch would need to consume if/when that packet is built. No drag-and-drop-chrome or command-palette-
registration module was found outside `🧱️elements/`.

## 7. Prioritised wave-2 packet list

Grouped the way Wave 1 grouped list-like/canvas-like `SurfaceKind`s: small same-shape gaps bundled into
one packet, large standalone items kept separate.

**Plugin/product-level (§2) — standalone, each its own packet:**

- **P-cad** (XL): CAD spatial-tree editor — port `🗿️artifact`/`🧬️typology`/`🎬️actions`/`🎰️stately`/
  `📔️registry`/`🏃️runtime` engine logic to Rust *and* build a wgpu paint layer for the 6 766-line
  renderer. This is what status.md's `W2f` already names; this audit confirms it's the single biggest
  item found anywhere in the census and that almost none of the supporting engine is reusable as-is
  (only 2+2 of 9 `⚙️engine/` submodules are Rust today).
- **P-animate** (L, new — not in status.md's Wave 2 plan, recommend adding to `W2f` or a sibling packet):
  Animate presentation-deck editor wgpu paint layer (~7.5k lines of React/CSS/TS). Unlike CAD, the
  engine (rate/video/animation/text/scene/geometry/config/camera — 34 Rust files) already exists; this
  is scoped paint work, not an engine port.
- **P-puzzle5d** (S): thin wgpu visualization layer over the existing 335-file Rust 5D puzzle artifact.
  Cheap given how much Rust already exists — good candidate to bundle into whichever wave has spare
  capacity, or fold into P-animate's wave for a "small React-only visualization targets" packet.
- **P-presentation-scope** (decision, not a packet): confirm with the ticket owner whether the
  `🎤️presentation` product's 9.4k-line deck viewer is in scope for wgpu parity at all, or is
  intentionally React/DOM-only like `📓️print`'s PDF output. If in scope, size it as its own L/XL packet
  separate from P-animate (different file, different product, shared helper modules only).

**Interaction/dispatch gaps (§5) — these are refinements of status.md's existing `W2b`/`W2a`, not new
packets; this audit's contribution is confirming they're still open against current disk state:**

- **P-context-menus** (M, = `W2b`): wire `scene.menu` per-row context menus for all 15 `SurfaceKind`s —
  `OverlayKind`/`open_overlay` scaffolding exists but nothing calls it from a right-click on a scene.
- **P-scene-interaction-remainder** (M, = `W2a`/scattered W1a/W1b follow-ups): drag-and-drop for Table/
  VFS/BlockList (shadowed hit targets), pointer modifiers (shift/ctrl) for multi-select, TextEditor
  completions/rename/context popups (still test-only), Paint2d marquee/navigator pan, NodeGraph
  double-click re-add, IconRender async loading state.

**Chrome-outside-elements gaps (§6) — new packets, not previously named in status.md:**

- **P-hotkey-registry** (M): app-level remappable keyboard-shortcut registry/dispatch — 
  `control-keybinding-context`(245) + `keybinding-text-interpretation`(141) +
  `keybinding-persistence`(43) + `control-hotkey-presentation`(42) ≈ 471 React/TS lines with zero wgpu
  counterpart. Distinct from W1c's fixed shell-shortcut-row list — confirm the distinction before
  assuming W1c already covers this; `🎚️UiPreferences/🟦️.ts`'s `keybindingOverrides` field is the
  existing data model to consume. Worth checking whether `fuzzy-ranking`(129, command-palette search) is
  actually already closed by W1c's `rankFuzzyItems` port before sizing it in — possible naming/location
  mismatch this pass didn't resolve.
- **P-rtl-flow-direction** (S): `flow-direction-context` — self-documented zero-support gap in
  `events/🦀️.rs`'s own comment.
- **P-chrome-polish** (S, bundle): `celebrate-border-burst`/`-spin` CSS effect with zero wgpu reference;
  `shell-floor-presentation`'s same-level double-paint suppression (uncertain, needs a follow-up look
  before sizing). Onboarding-tour (`UIIntroduction`) demo animations are arguably fine to leave
  React-only — flag, don't size.

**`🧱️elements/*` directory-level gaps (§3, §4) — both fully censused (35 + 60 directories):**

- **P-shell-dead-code-cleanup** (S, cleanup not porting): confirm-and-retire 📤️SegmentedDownload
  (`.ts`-only, not even a component), 🛂️SpaceAdministration, and 🧵️TaskManager (self-documented
  "registrar-only, unmounted") from §3 before anyone sizes a wgpu port for them — they may not need one.
  Bundle with §4's own weak/orphaned finds: 📑️Tabs, 🧾️Form, 📐️Layout, 🔚️Footer (no confirmed product
  consumer beyond their own tests/stories) and the five TUI-only orphans (➖️Divider, 📃️List, 🧙️Wizard,
  🪙️Chip, 🪵️Log — never built for any web target at all, out of scope entirely).
- **P-agent-chat-transcript** (part of the already-known "panel content-projection escape hatch" item
  in status.md's Wave-2-deferred list — this audit found and confirmed the concrete forcing case):
  `AgentChatPanel`'s wgpu file self-documents that only the header paints; the transcript body needs the
  generic `UiNode`/`UiTree`-only panel pipeline to grow an escape hatch for arbitrary hosted React
  subtrees (`RightPanelKind::Chat`) before it can render anything.
- **P-keyvalue-react-gap** (flag, not a wgpu packet): `🔑️KeyValue` is the one true inversion found in
  the whole census — wgpu renders `Component::KeyValueList` (25-line implementation) but **no React
  component exists anywhere in the repo** for it. This is a React-side gap to raise with whoever owns
  that lane, not something wave-2 wgpu work can close.
- **P-popover-dialog-content** (M, already named as P1 in the companion audit): Popover/Dialog have real
  positioning/focus-trap scaffolding (`OverlayKind`) on wgpu but nothing paints their *content* — 444 +
  609 React lines with no wgpu rendering at all. Confirmed independently by both this census (§4) and
  `📓️audit-interpreter-elements.md` §3.

---
*All planned sections (§2 plugin/product census, §3 os-renderer-engine elements — 35 dirs, §4 framework
`🖱️ui` elements — 60 dirs, §5 SurfaceKind/extension dispatch, §6 chrome outside elements, §7 packet list)
are complete, each directly verified against the current working tree by independent read-only research
passes and cross-checked against this ticket's companion audits and Wave 1 reports where they overlap.
No cargo/build commands were run by this audit (out of scope); the current wasm32/full-test compile
status called out in §5 remains for the integrator pass status.md already schedules before Wave 2 starts.*
