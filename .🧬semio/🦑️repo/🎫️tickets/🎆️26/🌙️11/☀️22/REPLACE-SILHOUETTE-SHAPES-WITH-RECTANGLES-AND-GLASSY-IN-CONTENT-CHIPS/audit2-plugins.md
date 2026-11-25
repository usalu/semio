# 🔎 Audit 2: Plugin Drawing Code Against the Frame Law

Read-only sweep of plugin and shared node-canvas drawing code, judged against `design.md` in this ticket: every frame-like element (node, card, group, panel, legend, window) is one axis-aligned rectangle with square corners, no shadow, no gap shapes; chips lie inside it as translucent glass. Ports, connectors, geometry glyphs, data marks, handles, sprite glows and depictions of foreign software are content and are not findings.

## Method

- Search terms: `silhouette`, `clip-path`, `box-shadow`, `drop-shadow`, `border-radius`, `rounded-*`, `shadow-*`, `NodeShape`, `circle`, `stencil`, `scissor`, `gap`, `cutout`, `notch`, `chip`, `footer`, `corner`, `radius`.
- Scope: `✏️s/🔌️plugins` (all plugin families listed in the request), `🧰️framework/🔨️modules/🖱️ui` (elements, targets, styling, wgpu, TUI), `🧰️framework/🔨️modules/🕸️graph`, `🧰️framework/🛍️products/💻️os/…/🎲️board`, `🧰️framework/🛍️products/📚️course` (mockups), `🧰️framework/🛍️products/🐾️pets`.
- Excluded: `🗿️artifacts`, `🏅️standards`, `🧬️schema` data, tests, fixtures, 3D B-rep and mesh geometry, and the `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🤖️generated` output.

## Headline

- The silhouette migration is done in code. `WindowSilhouette` no longer exists. `🔲️WindowFrame` and `🗂️WindowChrome` are present. Grep for `silhouette`, `stencil`, `gap` and `clip-path` in the Dock, view-chrome, WindowChrome and WindowFrame drawing code returns nothing outside tests.
- Plugin families contain no frame drawing. They emit semantic UI trees and board descriptors; the shape is decided in the framework. No plugin `.tsx` or `.css` file contains a shape, shadow or radius term.
- The live violations are: (a) the node shape default is `Circle`, (b) chrome class constants still name `rounded-*` and `shadow-*` on frame surfaces. Radius and shadow tokens are zero (`🎨️.css` lines 952–968; `radii::CHROME = 0.0` in `🔤️tokens/🦀️.rs:149`), so today they render square. They are latent contract violations, not visible ones.

## Findings

| # | file:line | shape | verdict | excerpt (proof) | fix | owning package | test |
|---|---|---|---|---|---|---|---|
| F1 | `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🔖️kinds/🦀️.rs:55-58` | graph node | Violation: node default is a circle | `pub enum NodeShape { #[default] Circle, Rectangle }` | Remove `Circle` from `NodeShape`, make `Rectangle` the only node shape and default. Keep circles only for explicit data marks (`🖊️drawing` `circular_layout` is geometry, not a node). Update `🖊️drawing/🦀️.rs:202,217` | `🧰️framework/🔨️modules/🕸️graph` (crate under `⚙️engine`) | `repo run <graph project>:test` |
| F2 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🗃️model/📸️session-json-gestures-and-catalog-parsing/🦀️.rs:10-11` | node from session JSON | Violation: missing or unknown shape falls back to circle | `Some("rectangle") => NodeShape::Rectangle, _ => NodeShape::Circle,` | `Some("rectangle") => …` only; unknown or missing shape is an error, or the default is rectangle | board os crate (`🎲️board`) | `repo run <os-board project>:test` |
| F3 | `…/🖼️scene/🧱️scene-and-style-state-impls/🦀️.rs:20-21` | node scene record | Violation: same circle fallback in the scene builder | `Some("rectangle") => NodeShape::Rectangle, _ => NodeShape::Circle,` | As F2. Drop the circle arm of `(radius, width, height)` at lines 24, 133, 150 | board os crate | `repo run <os-board project>:test` |
| F4 | `…/🧬️schema/📸️board-snapshot-records/🦀️.rs:7` | snapshot enum | Violation: schema still offers a circle node | `pub enum BoardNodeShape { Circle, Rectangle }` | Make it `pub enum BoardNodeShape { Rectangle }` or remove it, and fix the callers at `🌐️visibility-and-force-layout/🦀️.rs:15` and `🌳️buchheim-tree-layout/🦀️.rs:25` | board os crate | as F2 |
| F5 | `✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board/🎬️scene/🦀️.rs:78-79` | puzzle node | Violation: puzzle nodes default to circle | `let rectangle = node.get("shape")…== Some("rectangle");` / `self.push_text("shape", if rectangle { "rectangle" } else { "circle" }, control)?;` | Emit `rectangle` only, require `width` and `height`, drop the `radius` branch at line 80 | `✏️s/🔌️plugins/🧩️puzzle` (Cargo workspace, `Cargo.toml`) | `cargo test -j 2 -p <puzzle board crate>` inside `✏️s/🔌️plugins/🧩️puzzle` (one cargo at a time) |
| F6 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts:31` | floating menu surface (frame) | Latent: frame class names `rounded-md` and `shadow-sm` | `export const floatingMenuSurfaceClass = cn(glassClass, "overflow-hidden rounded-md border shadow-sm text-element", borderNormalClass);` Used by `📋️canvas-pick/🟦️.tsx:10`, `🧭️TiledMapHost/🟦️.tsx:10`, `🕸️NodeGraph/🪄️catalogue-spotlight/🟦️.tsx:12` | Drop `rounded-md` and `shadow-sm`. Add an assertion test that frame chrome classes contain no `rounded-` or `shadow-` (language-agnostic case plus Vitest) | `@semio-tech/ui` (`🧰️framework/🔨️modules/🖱️ui/package.json`) | `repo run @semio-tech/ui:test` (project name via `repo show_projects`) |
| F7 | `…/🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts:37` | floating ribbon (frame) | Latent: `rounded-md shadow-sm` | `export const floatingRibbonSurfaceClass = cn("overflow-hidden rounded-md border shadow-sm text-element", borderNormalClass, glassClass);` Used at `🧰️framework/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📐️projection/🟦️.tsx:132,160` | As F6 | `@semio-tech/ui` | as F6 |
| F8 | `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🎨️chrome-class-names/🟦️.ts:34` | glass overlay box (frame, dialog, intro, tutorial) | Latent: `rounded-lg` and `shadow-lg` | `export const GLASS_OVERLAY_BOX_CLASS = cn("text-foreground pointer-events-auto fixed z-tutorial max-w-sm rounded-lg p-double shadow-lg");` Used by `🧱️elements/📨️UIDialog/🟦️.tsx:120` and `🎯️targets/⚛️react/🎥️tutorial-ui/🟦️.tsx:10` | Drop `rounded-lg` and `shadow-lg`. Its own test `🧪️tests/🪟️uiintroduction-appearance-1/🟦️.tsx:306-308` already asserts absence of `b…` classes, extend it to `rounded` and `shadow` | `@semio-tech/ui` | as F6 |
| F9 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/💡️ChromeControlHint/🟦️.tsx:24` | chrome tooltip frame | Latent: `shadow-lg` | `"pointer-events-none fixed z-menu w-max max-w-sm border p-single text-xs text-balance text-popover-foreground shadow-lg",` | Drop `shadow-lg` | `@semio-tech/ui` | as F6 |
| F10 | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎓️introduction/🟦️.tsx:434` | introduction body (frame) | Latent: `shadow-lg` | `bodyClassName="p-double shadow-lg"` | Drop `shadow-lg` | `@semio-tech/ui` | as F6 |
| F11 | `…/🎯️targets/⚛️react/🎥️tutorial-ui/🟦️.tsx:210` | tutorial video overlay box (frame) | Latent: `rounded-lg` and `shadow-lg` on the fixed box that holds the video. The video pixels are content, the box is the frame | `className="pointer-events-none fixed z-tutorial rounded-lg object-cover shadow-lg"` | Drop `rounded-lg shadow-lg`; keep `object-cover` | `@semio-tech/ui` | as F6 |
| F12 | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📋️MenuItem/📖️stories/🧪️.story.tsx:19` | menu story frame | Latent: `rounded-md` on a menu frame sample | `<div role="menu" className="w-layout-floating-menu-sm rounded-md border p-single">` | Drop `rounded-md` | `@semio-tech/ui` (story) | `repo run @semio-tech/ui:test` |
| F13 | `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css:24,45,47` | course graph node mockup (node) | Violation: rounded node frame, `parameter` node at `2rem` radius, `panel` node at `0` | `.course-graph-node { … border-radius: 0.35rem; … }` / `.course-graph-node[data-widget="parameter"] { border-radius: 2rem; … }` | Set `border-radius: 0` on all nodes; keep the widget distinction through the border or fill | course product (`📚️course`) | `repo run <course project>:test` |

Not findings in the same family: `course/…/🗃️record-tables/🎨️.css:46` and `plots-and-crowd/🎨️.css:131,204` (`clip-path: inset(50%)` is a visually-hidden helper), and `🎨️styling/🖌️ui/🎨️.css:7455` (`clip-path: polygon(...)` on the native checkbox tick, a glyph).

## Reviewed, not findings

| file:line | shape | reason |
|---|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css:335,375,911,1582,6639-6676,7386-7545` | `border-radius: 0` | Already square, compliant |
| `🎨️styling/🖌️ui/🎨️.css:6715,7229,7387,7532,7633` | `box-shadow: none` | Compliant. Focus rings are removed, the chip's own lines carry focus |
| `🧰️framework/🛍️products/🎤️presentation/…/🎨️.css:10,13,92,146,206,215` | handle `border-radius: var(--spacing-single)`, `box-shadow: 0 0 0 hairline` | Interaction handles and selection rings are content. Radius `0` at 107 and 155 |
| `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🎨️.css:38` | `drop-shadow` on sprite | Actor glow, content |
| `🎚️Slider/🎯️targets/🧊️wgpu/🦀️.rs:103-117`, `🖌️paint/📶️progress-celebrate-and-widget-painters/🦀️.rs:245-303` | `push_rounded` on rail, thumb, dial, ring | Data marks and controls, content. Radius token is `0` anyway |
| `🖌️paint/🌲️retained-tree-node-painting/🦀️.rs:102,186` | `push_rounded` on tree header and row | Tree rows are content. Radius `0` |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🔘️trigger/🟦️.tsx:35`, `🧱️elements/💬️Dialog/🟦️.tsx:572`, `📻️TableAvatar/🟦️.tsx:61-89`, `👥️PresenceBar/🟦️.tsx:115-125`, `🎨️item-render/🟦️.tsx:49` | `rounded`, `rounded-full`, `rounded-xs` | Controls, avatars, presence dots, swatches are content marks |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🟦️.tsx:22,82,148` | `rounded`, `rounded-full` | Placeholder blocks for text and avatars, content |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/…` and `🧰️framework/🛍️products/💻️os/…/🧱️chrome-paint-and-navbar/🦀️.rs:405` | tab insert preview as rounded pill | Drop-preview marker, content |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🎨️chrome-classes/` `glassClass` usage | glass chips | Already the design's chip glass |
| `🧰️framework/🛍️products/💻️os/…/🥞️LayeredOverview/🟦️.tsx:14`, `🔶️panes`, `🧰️pane-helpers/🟦️.ts:87` | `clip-path` veil with a hole over the page | Overlay focus veil, not a frame border. Flag for owner decision if "no cutouts" must extend to overlays |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🎯️targets/⌨️tui/🦀️.rs` | TUI window | No rounded box glyphs (`╭╮╰╯` absent); square frames |
| `🧰️framework/🔨️modules/🕸️graph/🖊️drawing/🦀️.rs:196-217` | `handle_outside_node_clip_path` with `NodeShape::Circle` branch | Port and handle clip geometry; the node-shape variant itself is F1 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️.rs:19` | `DrawingKind::Circle → "circle"` | Drawing primitive, content |
| `✏️s/🔌️plugins/📕️norm/📇️registry/…/diagrams2d/🖼️canvas-lowering/🦀️.rs:70,428,434` | `MarkerShape::Circle` | Data marker, content |
| `✏️s/🔌️plugins/📕️norm/…/🪟️visual-windows/✅️checks/🩹️remedy-chips/🦀️.rs` | "remedy chips" | Semantic tree rows, not drawing. Rendered as tree items |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/🦀️.rs:18,20` | DXF circle and arc | Geometry, content |
| `✏️s/🔌️plugins/🔋️energy/…/🎬️scene/🦀️.rs`, `📕️norm/…/🧮️tessellation/…`, `🏭️process/…/🪵️wood`, `🔩️metal`, `🧱️concrete`, `🌊️flow/…/📐️brep`, `🌀️procedural`, `🀄️wfc` | `radius`, `clip`, `corner` hits | Physics, B-rep, engineering geometry, not UI frames |
| `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/…` | `PlanKind::Silhouette` | Standard artifact, excluded |

Plugin-side conclusion: the plugin families listed (flow, dag, trinity/jack/rewriting, note, animate, presentation, draw, writer, layout, forms, playbook, program/architect, reasoning, sequence, imperative, puzzle, block, sourcing, vcs, gis, space/home, stdio viewers, demonstrator) have no frame drawing. The only plugin finding is F5 (puzzle), because puzzle builds its own board scene. Dag, flow and note nodes take the framework default, which is F1 to F3.

## Lanes (disjoint files)

| lane | files | findings | owning package | test command |
|---|---|---|---|---|
| A: node shape (Rust) | `🧰️framework/🔨️modules/🕸️graph/⚙️engine/🔖️kinds/🦀️.rs`, `…/🕸️graph/🖊️drawing/🦀️.rs`, `…/🎲️board/…/📸️session-json-gestures-and-catalog-parsing/🦀️.rs`, `…/🎲️board/…/🖼️scene/🧱️scene-and-style-state-impls/🦀️.rs`, `…/🎲️board/🧬️schema/📸️board-snapshot-records/🦀️.rs`, `…/🎲️board/🧬️schema/🌐️visibility-and-force-layout/🦀️.rs`, `…/🎲️board/🧬️schema/🌳️buchheim-tree-layout/🦀️.rs` | F1, F2, F3, F4 | `🕸️graph` and `🎲️board` crates | `repo run <graph>:test`, `repo run <os-board>:test` |
| B: puzzle node (Rust) | `✏️s/🔌️plugins/🧩️puzzle/🔨️modules/🎲️board/🎬️scene/🦀️.rs` | F5 | `✏️s/🔌️plugins/🧩️puzzle` (Cargo) | `cargo test -j 2 -p <puzzle board crate>` |
| C: React chrome classes | `🎯️targets/⚛️react/🎨️chrome-classes/🟦️.ts`, `🔨️modules/🎨️chrome-class-names/🟦️.ts`, `🧱️elements/💡️ChromeControlHint/🟦️.tsx`, `🎯️targets/⚛️react/🎓️introduction/🟦️.tsx`, `🎯️targets/⚛️react/🎥️tutorial-ui/🟦️.tsx`, `🧱️elements/📋️MenuItem/📖️stories/🧪️.story.tsx` | F6, F7, F8, F9, F10, F11, F12 | `@semio-tech/ui` (`🧰️framework/🔨️modules/🖱️ui`) | `repo run @semio-tech/ui:test` |
| D: course mockup CSS | `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css` | F13 | course product | `repo run <course>:test` |

Lanes A, B, C and D share no file. C does not touch `UIDialog`, `Canvas`, or the r3f projection, which only import the constants. Fixing F6 to F8 in lane C covers the r3f and UIDialog call sites without editing them.

Greenfield: no compatibility layer, no deprecation. Remove the `Circle` variant rather than leave it unused. Run `cargo` one at a time with capped jobs.
