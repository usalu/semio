# 🔲️ `s.wfc.grid2d` in the browser — what was broken, what it takes to fix it

Slice **B2**, ticket `26/09/18/EXTRACT-WFC-PLUGIN`. Target: the react dev playground for variant
`grid2d` on `http://127.0.0.1:6042/?plugin=wfc` (server owned by the coordinator; never restarted).
Every log, report, screenshot and probe output referenced below lives under
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/EXTRACT-WFC-PLUGIN/🗑️generated/playground-grid2d/`.

---

## 0. Verdict

The editor is alive: both panes paint, the navbar example switcher moves between `pipes` and
`terrain` without journalling an edit, an armed `pin`/`mask` click on the grid pane edits exactly the
cell under the pointer and `⌘Z` reverts it, and `solve` paints the inferred assignment for BOTH
examples — vector pipes and bitmap terrain. Every probe step's `faultLines()` is empty.

At ticket-open **not one of its 27 verbs could be dispatched at all** and the grid pane threw before
it painted a pixel. Six defects, five of them mine to fix, one a framework fact worth writing down.

---

## 1. The six faults, in the order the browser surfaced them

### 1.1 The grid pane never rendered: `Board2d` is unreachable for any plugin but puzzle

Boot probe #1 (`🗑️generated/playground-grid2d/grid2d-boot/`): only ONE `[data-surface-id]` host
mounted, and the body read *"Grid Render error: The current app has no registered board session
factory."*

`Board2dHost` (`🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx:940`) resolves a
per-app wasm session through `resolveAppSurfaceSessionFactory(registrations, {pluginId, appId})` and
**throws** when none matches. The only `kind: "board-2d"` registrations in the whole repo are
`PUZZLE_BOARD_SESSION_FACTORIES` — puzzle's own wasm-bindgen `BoardSession` cdylib — and the dev
shell passes exactly that one list as `surfaceSessionFactories`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts:13,61`). A `Board2d` pane owned by any other
plugin therefore cannot paint. **`🧱️block`'s three board windows are dead for the same reason** —
worth a ticket of its own.

**Fix taken:** `wfc-grid2d-grid` is a `Canvas2d` pane now (`SurfaceKind::Canvas2d`), the same host the
preview pane already drew through, which needs no per-plugin session. Nothing was lost: a grid of
equal cells is a pure integer division away from a pointer sample, and the pane keeps its three
utilities and all twenty document verbs. The alternative — minting a wfc board-session cdylib, or
registering wfc's app ids against *puzzle's* wasm — is a framework-shaped change this slice does not
own.

### 1.2 `setActiveExample` was declared by no window kind

`semio: app "s.wfc.grid2d@1/*#editor" dropped action "setActiveExample" … no window kind declares it`.
Declared now on BOTH editor panes and on the viewer pane, `ActionKind::Mutation` / `Migrated`, with a
`Grid2dEditorCommand::SetActiveExample { example_id }` decoded from `exampleId`/`id`/`value`.

It emits `Effect::LoadDocument` (via a new `reset_document_effect`) rather than any mutation — a
whole-document replacement is not expressible as a `Grid2dMutation`, carries no inverse and belongs
in no undo ladder. That IS the declared-state-diff contract in its strongest form: **zero** history
patches, so re-selecting the boot example cannot mint a phantom edit. The probe measures this
directly (`example-terrain` / `example-pipes`, `edits: 0` on both).

`ArtifactEditor::build_document_store_initialization_job` had to be overridden as well
(`bounded_document_store_initialization_job`): the trait default REFUSES the replacement envelope, so
without it every swap faults at the archive-load boundary. Same override `🗒️note`, `🏗️fem`, `🖍️draw`
and `🏭️process` carry.

### 1.3 The real blocker: no retained tool proof, so EVERY verb was dispatch-dead

With the declaration in place the refusal only changed shape:

```
setActiveExample refused: dispatch-failed — typed command 'setActiveExample' has no exact
controller/owner/factory/tool/schema proof
```

`Grid2dEditorCommand`'s hand-written `impl protocol::OpBinary` never overrode `TOOL_JOB_IDS`, so it
inherited the trait default `&["typed-command"]`
(`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1211`). `registry.tool_job_registration` then
computes `expected = TOOL_JOB_IDS ∩ migrated = {}`, registers no bounded proof, and
`qualified_tool_proof` answers `interactive-job.missing-factory` for **every** app verb. All 27 —
the fourteen document mutations included — were dead in the browser while all 208 unit tests passed.
Sibling slice B4 hit the identical fault in grid3d.

Ported the sibling `🖼️bitmap` recipe into `✏️editor/🦀️.rs`:

| piece | what it is |
|---|---|
| `GRID2D_TOOL_IDS` (27) | the action ids, also `OpBinary::TOOL_JOB_IDS` |
| `grid2d_command_id` | command → its own action id (the registry keys on it) |
| `GRID2D_PUBLICATION_CONTRACTS` | Artifact ×16, WindowConfig ×7, HostOnly ×4 — read off each arm's emit |
| `Grid2dCommandWork` | one bounded first step calling `Grid2dEditor::dispatch` |
| `Grid2dCommandJobFactory` | the app-owned `ToolJobFactory` + `ArtifactOwnedToolJobFactory` |
| `bounded_first_step_tool_proofs!` | the proof catalog, same 27 ids |
| `build_artifact_store_one_item_preparation_factory` | without it every Artifact-lane verb answers `publication-authority-missing` |
| `build_tool_job`, `command_id`, `register_tool_job_factories` | the wiring |

`handle`'s body moved verbatim into a public `Grid2dEditor::dispatch(command, doc, cfg, view_state)`
so the trait entry point and the retained job run the same code; the job reads `view_state` and the
window config off `ArtifactOwnedToolJobContext`, which is how the armed utility reaches a click.

### 1.4 Two action ids may never share one command variant

`Canvas2dHost` syncs its camera as `setCamera { camera: { x, y, zoom } }` (debounced, nested) while
the palette form states `set-camera { x, y, zoom }` flat. Decoding both onto `SetCamera` looks
harmless and is not: `dispatch_typed_command_inner` refuses a command whose `command_id` is not the
action id it was admitted under. `SyncCamera` is its own variant now, writing the same config. (B4
hit this with `worldSelect`.)

### 1.5 The preview kept a stale solve cache across a document swap

`solve_json` is per-pane WINDOW CONFIG and the document can be replaced under it. After switching
`pipes → terrain` the preview painted the pipes assignment over the terrain grid as a field of dark
red "unknown tile" boxes labelled `elb`/`pip`/`emp` — see `interact-3/9-solve-terrain.png` (the
before shot, kept deliberately).

`preview::cached_commit` takes the document now and drops a commit that addresses a cell outside the
grid or names a tile the document does not have. A derived cache must be validated against its
source, not merely parsed.

### 1.6 A solved bitmap board overflowed the fixed UI surface — the solve ran and nothing was drawn

Terrain's `solve` produced **no** visible change and no fault line. The guest console told the truth:

```
ui.fixed-capacity: fixed UI admission failed at scene-surface.encode:
surface payload exceeds fixed capacity with 87337 bytes
```

64 cells × a 4×4 bitmap each = 1 024 bounds layers. The per-TILE ceiling (`MAX_BITMAP_PIXEL_RECTS =
64`) is not a bound on the BOARD. Added `MAX_BOARD_LAYERS = 256`, shared out over the assigned cells
as a per-cell pixel budget: a board over budget draws one palette-averaged swatch per cell, which
still shows the solved assignment (terrain now paints its grass/sand/water field — see
`terrain-solve.png`). A small board still draws real pixels, and a unit test pins both halves.

---

## 2. What changed

| file | change |
|---|---|
| `✏️editor/🎭️modes/✏️edit/🪟️windows/🔲️grid/🦀️.rs` | `Board2d` → `Canvas2d`; `layers_json` (one bounds layer per cell, colour per state, pinned cells labelled + `selected`); `cell_at` (the inverse of `screenToWorldLogical`, then integer division); `owns_surface`; `canvas_actions()`; `setActiveExample` declared |
| `✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` | canvas + `setActiveExample` actions; shared `effective_camera`; document-validated `cached_commit`; `MAX_BOARD_LAYERS` budget |
| `✏️editor/🪟️window/🦀️.rs` | `effective_camera` — an untouched camera centres on the authored grid, so a click and the pixel under it can never disagree |
| `✏️editor/🦀️.rs` | `SetActiveExample` / `CanvasPointerDown` / `CanvasGesture` / `SyncCamera` commands, `example_document`, `reset_document_effect`, `armed_pick`, public `dispatch`, the whole §1.3 retained-tool block, `build_document_store_initialization_job` |
| `👁️viewer/🦀️.rs` | a six-variant host-dispatched command channel with its own `ViewerApp` proof catalog, so the boot announcement and every hover are ACCEPTED instead of refused |
| `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` | those six actions declared on the read-only pane |
| 4 × `🧪️tests/🔬️unit` | updated + 8 new laws (surface kind, canvas verb roster, pointer inversion, `TOOL_JOB_IDS` ↔ manifest ↔ command join, example resolution, load-not-edit, foreign-cache drop, board budget) |

New probes in the ticket root: `🐍️grid2d-console-dump-probe.mjs`, `🐍️grid2d-interact-probe.mjs`,
`🐍️grid2d-viewer-probe.mjs`.

---

## 3. Probe results (all against the coordinator's serve on 6042, `--use-angle=metal`)

A `Canvas2d` pane carries no DOM text, so every probe fingerprints the canvas bitmap: distinct
colour count, a hash, and one pixel counter per cell state (`#2563eb` pinned, `#7f1d1d` masked,
`#38bdf8` the pipes tile ink).

### 3.1 Boot — `🗑️generated/playground-grid2d/boot/`

`data-semio-os-ready="grid2d"`, **both** hosts mount with a painted canvas (grid 57 distinct
colours, preview 52), no host fault, **0 fault lines**. Screenshot `boot/final.png`.

### 3.2 Interact — `🗑️generated/playground-grid2d/interact/`, 9 steps, **0 faulty steps**

| step | evidence | screenshot |
|---|---|---|
| boot | both canvases painted | `1-boot.png` |
| example → Terrain | combobox `Terrain`, grid repaints (pinned px 6→8, masked 9→0), **0 history patches** | `2-example-terrain.png` |
| example → Pipes | repaints back to the exact boot hash, **0 patches** | `3-example-pipes.png` |
| arm `pin` + one click at the pane centre | pinned px **6 → 9**, hash changes | `4-pin-click.png` |
| `⌘Z` | pinned px **9 → 6**, hash back to the boot hash exactly | `5-undo-pin.png` |
| arm `mask` + one click on cell (1,1) | masked px **9 → 15** | `6-mask-click.png` |
| `⌘Z` | masked px **15 → 9**, boot hash again | `7-undo-mask.png` |
| `solve` (pipes, VECTOR) | tile-ink px **0 → 102**; real cyan pipe strokes | `8-solve.png` |
| `solve` (terrain, BITMAP) | preview 48 → 90 distinct colours; grass/sand/water field | `9-solve-terrain.png` |

`8-solve.png` is the one picture worth opening: grid pane left with the pinned cell (blue, labelled
`elbow-ne`, amber outline) and the masked cell (dark red), preview pane right with the solved pipe
network.

### 3.3 Viewer — `🗑️generated/playground-grid2d/viewer/`

`playground.navbar.roles.viewer` pressed; the viewer's own pane renders one painted canvas (51
colours), no host fault, and two hover moves over it produce **0 fault lines** — where before the
fix the same hover produced `canvasPointerMove refused: dispatch-failed` twice and the boot
announcement produced `setActiveExample refused: undeclared-action` (`viewer-1/report.json` holds
that baseline).

### 3.4 Rust gates

| command | result | log |
|---|---|---|
| `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-wfc-grid2d --features component-app-assembly --lib -j 4` | **211 passed / 0 failed** | `test-6.txt` |
| `cargo check -p semio-s-plugin-wfc --lib -j 4` | **GREEN** | `check-plugin-5.txt` |
| `CARGO_PROFILE_WASM_DEV_DEBUG=false cargo check … --target wasm32-wasip2 --lib --tests` | **GREEN, 0 warnings from this crate** | `check-wasm-8.txt` |
| `bun nx run @semio-tech/wfc-plugin:describe` | GREEN (grid2d: 40 / 22 / 19 window actions, all three panes `canvas-2d`) | `describe-5.txt` |
| `activate-grid2d-react-dev` | GREEN; staged descriptor re-read from `…/dist/dev/🔌️plugin-modules/🀄️wfc/🔣️.json` to confirm it carries the change before every browser run | `activate-4.txt` |

---

## 4. Two things that cost real time, for whoever reads this next

**Playwright's synthesised mouse input never reaches this canvas in headless Chromium.**
`elementsFromPoint` puts the canvas on top with `pointer-events: auto`, and `page.mouse.move/down/up`
at those exact coordinates produces *no* guest dispatch at all — no edit, no refusal, nothing. A
`PointerEvent` dispatched on the canvas element does reach it (proved by the guest's own
`[DEBUG] typed-operation slots` line). `clickSurface` in the interact probe delivers presses that
way and says so in its docstring. Diagnostics kept: `🗑️generated/playground-grid2d/pointer-probe.mjs`,
`hit-probe.mjs`.

**A fleet-wide `prebuild_lock_exclusive` deadlock, and how it ends.** For ~15 minutes twelve cargo
processes across the whole agent fleet sat on `Blocking waiting for file lock on artifact directory`
with **zero rustc alive**; `sample` put every one of them inside `cargo::util::flock::acquire` on
`⚡️cache/cargo/target/debug/.cargo-lock`. Killing ONE participant — my own queued `cargo test` —
released the cycle and rustc restarted within seconds. Diagnose with `ps aux | grep -c "[r]ustc"`:
zero rustc with many waiting cargos is a deadlock, not a slow build.

---

## 5. Remaining gaps

1. **`🧱️block`'s three `Board2d` windows are still dead** for the reason in §1.1 — nothing in this
   ticket touches them. Closing it properly means either a generic board session in the framework or
   a per-plugin cdylib; both are framework-shaped.
2. **A bitmap board over 256 layers previews as one averaged swatch per cell**, not as pixels
   (§1.6). The real fix is a `data:` URL `image` layer minted in Rust — `📓️grid2d.md` §8 gap 4,
   now board-wide rather than per tile.
3. **The viewer's example switcher is inert.** `ArtifactViewer` declares neither
   `build_document_store_initialization_job` nor `build_artifact_store_one_item_preparation_factory`,
   so a viewer structurally cannot admit a `LoadDocument` envelope. It accepts and validates the id
   (which is what removes the boot refusal) and does nothing else. Switching examples is an editor
   gesture. Slice B5 was failing `cargo check -p semio-s-plugin-wfc` on exactly those two `E0407`s
   while this was written.
4. **The solve cache is still per-pane**, so a second preview pane shows an unsolved grid until
   `solve` runs in it (`📓️grid2d.md` §8 gap 3, unchanged).
5. **`solve` must be re-armed between runs.** Pressing the same Actions row twice disarms it; the
   probe closes and reopens the Actions pane, then presses the row and the framework chord. Not a
   grid2d defect — the shell's arming contract — but it makes a scripted second run look like a
   silent failure.
6. **The camera re-centres when a pan lands exactly on the world origin**, because `effective_camera`
   treats `(0, 0)` as "never written". A written-flag on the window config would be exact.
7. **One 404 at boot** (`Failed to load resource: 404`) — present before this slice, from the shell's
   own asset probing, not from any wfc route; it produces no fault line and no visible effect.
