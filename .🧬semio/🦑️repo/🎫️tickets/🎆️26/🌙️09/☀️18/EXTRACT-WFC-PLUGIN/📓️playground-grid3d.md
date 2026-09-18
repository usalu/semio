# 🧱️ B4 — the `s.wfc.grid3d` playground, end to end in a real browser

Slice B4. Variant `grid3d`, react renderer, served by the coordinator at
`http://127.0.0.1:6044/?plugin=wfc` (log `🗑️generated/playground/serve-grid3d.log`).
Everything below was **run**, and every number quoted comes out of a probe artefact under
`🗑️generated/playground-grid3d/`.

---

## 1. What the boot probe found

`🐍️grid3d-console-dump-probe.mjs` (a clone of `🐍️wfc-console-dump-probe.mjs` that also reads
`data-instances-json`, `data-status-json`, per-host canvas state and every HTTP ≥ 400 / failed
request). Output `🗑️generated/playground-grid3d/boot/{state.json,console.txt,network-failures.txt,final.png}`.

| | reading |
|---|---|
| shell beacon | `data-semio-os-ready="grid3d"`, `data-semio-os-error` null |
| title / picker | `semio · wfc · grid3d`, navbar example combobox reads **Building Blocks** |
| `window:wfc-grid3d-grid` | 708×807, 1 canvas, **3 meshes, 48 instances** (`cell` 46, `cell-pin:floor` 1, `cell-masked` 1) |
| `window:wfc-grid3d-preview` | 708×807, 1 canvas, **4 meshes, 47 instances** (`tile:air` 27, `tile:wall` 14, `tile:roof` 4, `tile:floor` 2) |
| preview `status_json` | `{"state":"solved","solved":47,"cells":47,"seed":7}` |
| network | one 404, `…/🧩️extension-modules/watch` — the dev server's own SSE endpoint, present on every variant, nothing to do with grid3d |

**The solve is real and visible.** 4 × 3 × 4 = 48 cells, one masked, so 47 to fill, and the preview
publishes exactly 47 instances — one mesh instance per solved cell, each scaled into its own
non-uniform cell box. `final.png` shows both panes painting the block: the grid as a translucent cage
with the pinned cell tinted, the preview as the solved assignment.

**The canvas readback in the probe is a false negative, the screenshot is the oracle.** Reading a
World3d canvas back with `drawImage` + `getImageData` returns a single flat colour, because the
renderer's context is not created with `preserveDrawingBuffer` — the drawing buffer is already
presented and cleared by the time a script can copy it. `distinct: 1` therefore means nothing; the
`page.screenshot()` of the same frame shows the geometry. Any future probe that wants to assert
"visibly non-empty" must screenshot the host's bounding box and count colours in the PNG, never ask
the page.

**No interactive-ceiling fault.** `ArtifactEditor::render` solves inline on the render path
(`👁️preview`'s own §7.5 caveat) and the console carries no `interactive-ceiling`, no quarantine and no
`more-work` spin across a 60-second boot. The 48-cell example solves well inside the 8 ms budget, so
the solve stays on the render path: moving it to a resumable tool run would be cost with no symptom
to pay for. That trade flips the moment a document is much larger than the bundled examples — the
honest fix is then a retained tool run whose result caches in the window transient, not a bigger
budget.

---

## 2. What the baseline interaction probe found: the editor was completely dead

`🐍️grid3d-interact-probe.mjs`, output `🗑️generated/playground-grid3d/interact-baseline/`. Four real
faults, and they compound.

### Fault 1 — `setActiveExample` was declared nowhere

```
semio: app "s.wfc.grid3d@1/*#editor" dropped action "setActiveExample" dispatched from window kind
"wfc-grid3d-grid": no window kind declares it (window kinds: wfc-grid3d-grid, wfc-grid3d-preview).
```

The shell dispatches its boot `setActiveExample` against the FOCUSED window's kind. The picker
therefore did nothing at all: the combobox label changed to "3D Pipes" while the document stayed on
`blocks` (the probe's `example-switch-away` step shows the hosts unchanged, still
`cell-pin:floor: 1`).

### Fault 2 — no `command_from_action`, so every declared action was unreachable

```
setActiveTile refused: dispatch-failed — action 'setActiveTile' is not a framework-reserved action
(history/clipboard/revert/filter/noteShellCommand) — app actions are dispatched exclusively through
the typed command channel now
```

That is `ArtifactEditor::command_from_action`'s **trait default**, which refuses everything. grid3d
never overrode it, so all seventeen actions the grid window declares — every document verb, the pick,
the camera — were dead in the browser and only the crate's own typed tests could drive the editor.
This is the fault that matters most: the window's action list looked complete and was entirely
decorative.

### Fault 3 — the pick granularity was pruned

```
interaction selection lost reason=validate-state-pruned
  dispatched=vortex=object:0:0:1;wfc.grid3d.cells=handle:0:0:1
  validated=wfc.grid3d.cells=cell:0:0:1
```

The scene set `domain_id` but left `domain_granularity_id` unset, so `World3dHost` picked with its own
fallback granularity `handle`, which the `wfc.grid3d.cells` domain never declares — the target was
pruned on validation. The cell key itself (`0:0:1`) was correct all along.

### Fault 4 — no retained tool proof, so every verb is still dead after the bridge

Fixing fault 2 only moved the wall one step: the re-staged build answered

```
setActiveExample refused: dispatch-failed — typed command 'setActiveExample' has no exact
controller/owner/factory/tool/schema proof          (interactive-job.missing-factory)
```

`InteractiveJobClassification::Migrated` is the only live classification, so **every** dispatch runs
through the retained route, and that route admits a verb only against an exact app-owned tool
registration. grid3d had none at all — no `TOOL_JOB_IDS`, no publication contracts, no
`bounded_first_step_tool_proofs!`, no `register_tool_job_factories`/`build_tool_job`. A declared,
bridged, `Migrated` action is still dead without them.

The complication grid3d has and the sibling `🧊️3d` does not: `ArtifactCommandInputs` carries no
`ViewModel`, and three grid3d verbs need one — `pickCell`/`worldSelect` read the ARMED UTILITY,
`setActiveTile`/`setCamera` address one window INSTANCE's own config. The view state is reachable, but
only through `input.context` (`ArtifactOwnedToolJobContext::view_state` / `window_config`, `🗒️note`'s
own retained shape). So the reducer was split out of `ArtifactEditor::handle` into the free
`grid3d_command_emit(command, doc, cfg, view_state)`, and both routes — the direct one and
`Grid3dCommandWork::step` — call it, which is what keeps a verb from meaning two different things
depending on how it arrived.

### Fault 5 — one bad action poisons the whole instance

After the first `dispatch-failed`, every later input answers

```
plugin.internal.prior-outcome: runtime live cleanup faulted for instance …
```

`interactionSelect`, `interactionHover`, `setCamera` and all four `undo` presses were refused this
way. So a single unbridged action does not fail alone: it takes undo, the camera and hover with it for
the rest of the session. That is why the baseline probe shows `undoPatches: 0` — undo was never
broken, it was poisoned.

---

## 3. The fixes

All in `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`.

**`🦀️.rs` (editor root)**

- `Grid3dEditorCommand` gained four variants: `SetActiveExample`, and the three verbs the host's
  DOMAINLESS lane dispatches — `WorldSelect` (an instance pick), `SetHover`, `WorldPick` (a click that
  hit nothing, or a sub-object component). The last two are deliberately inert: a hover is per-frame
  state and putting a store write on the pointer path is the wrong trade, and clicking empty space is
  not an edit. `WorldSelect` is `PickCell`'s twin rather than an alias, because
  `dispatch_typed_command_inner` rejects a command whose `command_id` is not the action it was
  admitted under.
- `example_snapshot(id)` resolves `blocks` / `pipes-3d`; an unknown id **faults**
  (`wfc.grid3d.example.unknown`) rather than opening an empty grid — the picker sends the id it read
  off the manifest, so a mismatch is a registration bug (`🗒️note` learned this when the navbar sent
  `demo` and the editor matched only `semio`).
- `reset_document_effect()` — `Effect::LoadDocument { pack, spr }` over an EDIT-FREE
  `store::empty_document_spr`. Never a live `ArtifactEnvelope`: such an envelope owns a bounded
  retirement authority whose `Drop` traps the guest, which is what `🧊️process3d`'s own boot
  `setActiveExample` hit.
- `build_document_store_initialization_job` → `bounded_document_store_initialization_job`. The trait
  default answers `Err(envelope)` — it REFUSES the envelope a `LoadDocument` hands the store, and an
  example switch is exactly such a swap.
- `build_artifact_store_one_item_preparation_factory` → the framework's generic bounded preparation.
  Without it every route declaring the `Artifact` publication lane is registered as *unsupported* and
  stays dispatch-dead.
- `command_id()` — one manifest action id per variant. The default answers one opaque
  `"typed-command"` for all of them, which makes a history row unaddressable and breaks the
  declared-action ↔ command round trip.
- `command_from_action()` — the full bridge, 21 actions. Coordinates and colours are read as numbers,
  `axis`/`direction` are matched case-insensitively against the artifact's own wire labels, and a
  missing required argument answers `wfc.grid3d.action.missing-argument '<action>.<field>'` instead of
  silently defaulting.
- The reducer moved out of `ArtifactEditor::handle` into the free
  `grid3d_command_emit(command, doc, cfg, view_state)`, which BOTH dispatch routes call — see fault 4.
- `SetActiveExample` handling: **re-selecting the example already open emits nothing**
  (`Emit::default()`), so the picker cannot write the phantom edit that leaves `canUndo` true on a
  document nobody changed.
- A whole `//#region 🧵️Retained`: `GRID3D_RETAINED_TOOL_IDS` (21), `GRID3D_PUBLICATION_CONTRACTS`
  (`Artifact` for the document verbs, `WindowConfig` for `setActiveTile`/`setCamera`, `HostOnly` for
  the example picker and the two inert host verbs), `Grid3dCommandWork`,
  `Grid3dRetainedCommandJobFactory`, `register_tool_job_factories`, `build_tool_job` and the
  `bounded_first_step_tool_proofs!` block — ported from the sibling `🧊️3d` slice.

**`🎭️modes/✏️edit/🪟️windows/🧱️grid/🦀️.rs`**

- Declares `setActiveExample`, `worldSelect`, `setHover` and `worldPick` alongside the seventeen
  existing verbs (all inherit the file's `InteractiveJobClassification::Migrated` sweep).
- Staged argument forms are attached HERE, in `definition()`, not through the app builder's
  `action_args` — that method searches only app-scope actions and **silently drops** what it cannot
  find, which is why the first attempt produced rows with no form and
  `wfc.grid3d.action.missing-argument 'maskCell.x'` on every press.
- `render()` now takes the ARMED UTILITY and picks the pick lane from it:
  - `select` → the scene declares `domain_id` **and** `domain_granularity_id: "cell"`, so the host
    routes the hit through the framework verbs and fault 3 is gone;
  - `pin` / `mask` → the scene declares **no** domain, so `World3dHost` falls back to its
    plugin-private lane and dispatches `worldSelect {ids:["x:y:z"]}`, which the bridge maps onto the
    same reducer arm as `pickCell`.

  This is the only shape that can work. Under a declared domain `World3dHost` dispatches
  `interactionSelect` and *nothing else* — a framework selection write, which can never become a
  document mutation, and no hook anywhere turns one into an app command. A click under a writing
  utility is an edit, so it must not ride the selection lane at all. The granularity has to be spelled
  either way: `World3dHost` reads `mesh`/`object` as "address the hit by ARRAY INDEX" and dispatches
  `worldPick` with a number in place of a cell key.

  The cost of leaving the domain is that the domainless lane's own three verbs (`worldSelect`,
  `setHover`, `worldPick`) must all be declared — each one was found by a probe run, one per round.

---

## 4. Verification — every command run, with its result

| # | command | result |
|---|---|---|
| 1 | `cargo check -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib --tests -j 4` | ✅️ exit 0 (`🗑️generated/playground-grid3d/check-14.log`) |
| 2 | `RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-wfc-grid3d --features component-app-assembly --lib -j 4 -- --test-threads=4` | ✅️ **207 passed, 0 failed, 2 ignored** (`test-11.log`) — 201 before this slice, six new laws |
| 3 | `cargo check -p semio-s-plugin-wfc --lib -j 4` | ✅️ exit 0 (`check-plugin-2.log`) |
| 4 | `activate-grid3d-react-dev` | ✅️ exit 0, "1 completed components (changed)" (`activate-6.log`) |
| 5 | boot probe | ✅️ `boot-3/`, zero fault lines |
| 6 | interact probe | ✅️ `interact-5/`, **eight steps, zero fault lines on every one** |
| 7 | viewer probe | ✅️ `viewer/`, `window:wfc-grid3d-view` with 4 meshes / **47 instances**, no faults |

### What the interaction probe proves, step by step (`🗑️generated/playground-grid3d/interact-5/`)

| step | evidence |
|---|---|
| boot | grid 48 instances (`cell` 46, `cell-pin:floor` 1, `cell-masked` 1), preview `solved 47/47 seed 7` |
| arm a tile | `setActiveTile` staged form filled and executed — the per-window-config lane |
| **pin on the canvas** | `pin` utility armed, one click at (401, 469): `cell-pin:floor` **1 → 2**, `cell` 46 → 45 |
| **mask on the canvas** | `mask` utility armed, one click at (448, 406): `cell-masked` **1 → 2**, cells to fill 47 → 46 |
| **undo** | three presses: grid back to `cell-pin:floor` 1 / `cell-masked` 1, preview back to `solved 47/47` — both canvas edits were real document edits, not view state |
| example → 3D Pipes | grid 27 instances (`cell-pin:pipe-x` 1), preview `solved 27/27 seed 19` |
| example → Building Blocks | back to 48 / `solved 47/47` |
| re-select the open example | **zero history patches** — no phantom edit |

`4-mask-pick-on-canvas.png` shows it: the Actions pane listing every verb, the Utilities bar with
`Mask` armed, and the newly masked cell drawn as a dark cage inside the block.

### One honest behaviour worth naming

After the probe's pin, the preview reports `{"state":"contradiction","solved":0}` and paints an empty
scene. That is correct, not a regression: the `blocks` rule set admits `floor` only in certain stacks,
so pinning it into the cell under the cursor makes the grid unsatisfiable. The artifact answers
`satisfiable: false` with no rows — a grid with no consistent assignment is an ANSWER, never an
inference failure (`📓️grid3d.md` §5.4) — and the undo step restores `solved 47/47`.

### New tests

Grid window: `a_writing_utility_hands_the_pick_to_the_app_instead_of_the_framework_selection` (asserts
`domain_id`/`domain_granularity_id`/`selection_json` per utility, on the real `World3dScene` rather
than a rendered node's debug text) and `the_grid_window_declares_the_example_picker_and_the_world_pick`.

Editor: `every_declared_action_bridges_to_the_command_it_names` (walks all 21 non-reserved window
actions through `command_from_action` and asserts `command_id` round-trips — the regression guard for
fault 2), `both_pick_lanes_carry_the_same_cell_key`,
`every_declared_verb_carries_a_retained_proof_and_a_publication_lane` (the guard for fault 4: a verb in
the roster with no publication lane, or a declared verb with no proof, is dispatch-dead), and
`the_example_picker_replaces_the_document_and_re_selecting_the_open_one_is_inert`.

---

## 5. Probes in this ticket folder

| script | what it does |
|---|---|
| `🐍️grid3d-console-dump-probe.mjs` | boot: readiness beacon, per-host mesh/instance/status counts, HTTP ≥ 400 and failed requests, screenshot |
| `🐍️grid3d-interact-probe.mjs` | the eight-step interaction battery above; every step records its console delta, its fault lines and a screenshot |
| `🐍️grid3d-viewer-probe.mjs` | switches the navbar role to Viewer and checks `wfc-grid3d-view` renders |
| `🐍️grid3d-discover-probe.mjs` | selector calibration — dumps the Actions pane / utility bar / staged-form element ids |

All take `SEMIO_PROBE_URL` and `SEMIO_PROBE_OUT`, launch chromium with `--use-angle=metal`, and write
under `🗑️generated/playground-grid3d/`.

## 6. Notes for the plugin root and W2

1. The viewer declares ONE window (`wfc-grid3d-view`), by design (`📓️grid3d.md` §4) — "both windows"
   is the EDITOR's layout, and the viewer shows the solved scene only.
2. The one 404 on every page load, `…/🧩️extension-modules/watch`, is the dev server's own SSE
   endpoint and is present on every variant.
3. The in-page canvas readback (`drawImage` + `getImageData`) is a **false negative** for World3d: the
   context has no `preserveDrawingBuffer`, so it always answers one flat colour. Judge "visibly
   non-empty" from `page.screenshot()`, never from the page.
4. The solve still runs on the render path and stays inside the interactive budget for both bundled
   examples. A document much larger than these would need a retained tool run whose result caches in
   the window transient; nothing in the live app asks for that yet.
