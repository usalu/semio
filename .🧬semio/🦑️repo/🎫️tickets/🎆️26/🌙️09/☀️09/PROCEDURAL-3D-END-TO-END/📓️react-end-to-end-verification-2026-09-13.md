# React End-To-End Verification — generation3d on 6018 (lane `react-end-to-end-verification`)

Sixteen probes, one battery, run sequentially (one browser at a time) against the serve on
`http://127.0.0.1:6018/?plugin=generation3d`. Everything below is a number a probe read off the app's
own published evidence — `data-fixture-json`, `data-meshes-json`, `data-status-json`,
`data-selection-json`, the shell's `[DEBUG] performInvocation` lines, or bytes on disk.

- Battery: `🐍️react-battery.mjs` (`bun 🐍️react-battery.mjs [--only=a,b] [--list]`)
- New probe: `🐍️react-gap-probe.mjs` (the punchlist gaps no other probe covered)
- Recon used to aim it: `🐍️react-gap-recon.mjs`
- Scoreboard: `🗑️generated/react-verify/scoreboard.json`; per-probe artifacts in
  `🗑️generated/react-verify/<probe>/` (screenshots, `console.txt`, `results.json`, `run.txt`)
- Restage helper: `📜️restage-react-verify.sh`

---

## 1. TL;DR scoreboard

Final run: `🗑️generated/react-verify/battery-final.txt` (plus `battery-10.txt` for the last `gaps`
re-run). **15 of 16 probes green, 1125 s total, 5 page errors across the whole battery.**

| probe | ok | s | steps | page errors |
|---|---|---|---|---|
| boot | ✅ | 101 | 2/2 | 0 |
| **journey** | ✅ | 116 | **23/23** | 1 |
| interact (hover / select / orbit) | ✅ | 78 | 5/5 | 0 |
| generate-mode | ✅ | 33 | 11/11 | 0 |
| flow-window | ✅ | 71 | 2/2 | 0 |
| flow-wire (cut / reconnect) | ✅ | 122 | 3/3 | 1 |
| flow-reorganize | ✅ | 127 | 1/1 | 0 |
| cancel-preview | ✅ | 11 | 2/2 | 0 |
| keyboard-verbs | ✅ | 27 | 5/5 | 0 |
| io-surface | ✅ | 23 | 6/6 | 0 |
| export-encoding | ✅ | 21 | 2/2 | 0 |
| status-parity | ✅ | 199 | 10/10 | 1 |
| role-switch | ✅ | 35 | 7/7 | 0 |
| i18n-a11y | ✅ | 56 | 5/5 | 0 |
| customization-persistence | ✅ | 25 | 2/2 | 0 |
| **gaps** (new) | ❌ | 80 | **8/10** | 2 |

The two red `gaps` steps are `inspection-edit` (fixed in the tree, **cannot be restaged** — §5.1) and
`generate-chord` (a real defect, characterised but not fixed — §5.2). Everything else a user can
reach on the React renderer is green on one stage.

---

## 2. Journey — all 8 examples, edit + viewer + generate, per-example seconds

`🗑️generated/react-verify/journey/results.json`, 23/23 converged (the predicate is contract-based:
the flow window's published widget ids must equal the example's expected set AND the picker label
must match AND the preview must carry meshes, held stable for 3 s).

| example | edit s | edit meshes | viewer s | viewer meshes |
|---|---|---|---|---|
| No example | 3 | 0 | 3 | 0 |
| Hexagonal Mushroom Column | 4 | 3 | 3 | 3 |
| Rectangle Extrude Volume | 7 | 1 | 3 | 1 |
| Sphere Cut With Torus | 7 | 1 | 4 | 1 |
| Box Fillet Preview | 6 | 1 | 3 | 1 |
| Sphere Box Fuse | 7 | 1 | 4 | 1 |
| Face Sweep Extrude | 8 | 1 | 4 | 1 |
| Rectangle Wire Preview | 3 | 1 | 3 | 1 |
| Box Shell Preview | 5 | 1 | 3 | 1 |

Boot to a rendered hexagonal column: **10 s**. Mode/role hops: `generate-mode` 3 s,
`generate-added` 3 s (meshes 1), `back-to-edit` 3 s, `viewer-role` 4 s. Whole journey: 116 s.

Screenshots: `🗑️generated/react-verify/journey/<n>-<label>.png` (23 frames).

---

## 3. What each green probe actually proved

| probe | the reading that decided it |
|---|---|
| boot | `window:procedural-main` + `window:procedural-preview` mount, 0 page errors |
| interact | meshes 3 before the gesture; hover → `hoverTarget`/`interactionHover`; click → `selectedIds`/`interactionSelect`; drag → `setCamera`; 0 `unknown merge` |
| generate-mode | add ×2, select, rename (`Balcony Study`), remove, Form slider edit (generate preview mesh payload moves), gumball rail `["Move","Rotate","Scale"]`, `gumballActive:true` — 11/11 |
| flow-window | node-graph canvas paints (`dag draw lod=normal zoom=0.917`), 0 page errors |
| flow-wire | 6 wires published → cut removes `height@number->extrusion-axis@z` → redraw restores it |
| flow-reorganize | `mod+alt+L` moves all 7 widgets |
| cancel-preview | a `cancellable` status frame is published and the cancel affordance exists |
| keyboard-verbs | `mod+alt+d` → `cycleShowMode` (twice), `mod+alt+k` → `cycleLodMode`, `mod+z` → `undo`, `mod+shift+z` → `redo` |
| io-surface | both verbs published; `mod+shift+e` exports real bytes; `mod+o` imports a real `.stl` into the graph — 6/6 |
| export-encoding | `generation3d.dwg` 1096 B magic `AC1015`; `generation3d.stl` 3810 B magic `solid generation` — real binary, not base64 |
| status-parity | all three preview surfaces (edit / generate / viewer) publish the `World3dComputeStatusV1` contract across 10 hops, `No example` clears each |
| role-switch | mid-chain and post-convergence switches; `noActor` 0; all four window sets mount |
| i18n-a11y | German observed for every label the mode actually paints (edit: Workflow/Vorschau/KNOTEN/LEITUNGEN/Eingang/Ausgang/Ausgewertet; generate: Generationen/Formular/Vorschau; viewer: Vorschau); every surface shell named + focusable; `lang=de` and `appearance=dark` survive a reload |
| customization-persistence | locale and appearance both survive a reload (`light → dark → dark`, `semio.os.config`) |
| gaps 8/10 | §4 |

---

## 4. The new `gaps` probe — the punchlist items nothing else covered

`🐍️react-gap-probe.mjs`, artifacts in `🗑️generated/react-verify/gaps/`.

| step | ok | evidence |
|---|---|---|
| boot | ✅ | 7 widgets, preview meshes ≥ 1 |
| dock-resize | ✅ | dragging `[data-slot="resizable-handle"]`: panes `1082/509` → `1191/399` px |
| export-after-generate | ✅ | Add Generation → generate preview meshes 3 → back to edit → Actions pane → `generation3d.stl`, 3810 B on disk |
| wire-undo | ✅ | 6 wires → cut (`disconnect-synapse id=e1` in the history patch) → 5 → `mod+z` → 6, byte-identical to the start |
| doc-panel-select | ✅ | clicking `panel:procedural-play-document/height` invokes `interactionHover` + `interactionSelect`; `selectedIds` gains `height`; the row's `aria-selected` flips |
| catalogue-add | ✅ | the `Compound Cut` operator row dispatches `addWidget`; the graph grows `brep_bool_compoundCut_2` |
| actions-pane-de | ✅ | 39 Actions rows, all German (`Graph bearbeiten`, `Auswahl löschen`, `Dokument exportieren…`, `Neu anordnen`, `Element hinzufügen…`) |
| window-focus | ✅ | boot `procedural-main` → click flow body → `procedural-main` → click the preview's dock tab → `procedural-preview`, with the dock tab mirroring it |
| inspection-edit | ❌ | §5.1 |
| generate-chord | ❌ | §5.2 |

---

## 5. Defects found, and what happened to each

### 5.1 Document panel rows were inert — **FIXED, runtime-proven**

**Symptom.** A real click landed on `panel:procedural-play-document/height` (composed path
`tree-label → … → the row`, `pointer-events:auto`, 298×24 px) and **nothing happened**: no
`[DEBUG] performInvocation` at all, `aria-selected` stayed `false`, `selectedIds` stayed `[]`. The
Inspection panel therefore never left its "no selection" branch, so two punchlist items were blocked
by one cause. Recorded by `🐍️react-gap-recon.mjs` (`🗑️generated/react-verify/recon/recon.json`).

**Root cause.** The panel declared `.interaction_domain("graph")` and no per-row binding, on the
strength of a docstring citing `ui_tree_stamp_presence`. That symbol lives in
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/` — it is a **wgpu-only** contract. On the React path
`Component.interactionDomain` is carried faithfully all the way into the browser and then **never
read**: `TreeView`'s `<Tree>` call in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1407-1416`
passes no `onSelectionChange` and reads no domain, and a row's `onClick` exists only when the row
carries an `activate` binding (`:1369`). The framework's own working shape is the explicit per-row
binding — puzzle3d's document panel authors it, the renderer's engine-contract fixture encodes it,
and this app's OWN flow-window outline (`🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`, `node_row`) already
carries it.

**Fix.** `✏️editor/📌️panels/🗿️artifact/🦀️.rs` — every widget row now binds
`Trigger::Activate → interactionSelect` and `Trigger::HoverPreview → interactionHover` against the
`graph` domain with its own id as the `node` target, reusing the flow window's `graph_targets_json`
(made `pub(crate)` — a one-token change to that peer-owned file, nothing else touched).

**Proof.** Law `document_rows_bind_both_framework_interaction_verbs` +
`🧫️fixtures/🔬️unit/🔣️.json` (language-agnostic, same projection the flow outline's law uses) — passes.
Runtime: `gaps/doc-panel-select` green, `invoked: ["interactionHover","interactionSelect"]`,
`selectedIds` gains `height`.

### 5.2 `mod+shift+g` (Add Generation) reaches nothing in either mode — **OPEN**

`addGeneration` is declared only on the generations window
(`window_kind_action_refs(GENERATION_3D_PLAY_WINDOW_GENERATIONS, …)`, `✏️editor/🦀️.rs:2424`) while the
chord is app-wide (`:2497`). Measured:

- **edit mode** (`keyboard-verbs`, `add-generation` row): `invoked: []`. No window in that mode
  declares the verb, so the chord has no owner.
- **generate mode** (`gaps/generate-chord`): the Generations window IS mounted
  (`[data-slot="window"]#generation3d-generations`), a row inside it was really clicked
  (`focusedGenerations: true`), and still **`activeWindow: null`** — in generate mode no window ever
  becomes `[data-slot="window"][data-active="true"]` — and `chordInvoked: []`.

So the keyboard path to Add Generation is dead everywhere; the tree row still works (the journey and
`gaps/export-after-generate` both add a generation by clicking). Two candidate owning layers, neither
touched here: the dock never activating a window for the generate mode's layout (framework), or the
verb's window-scoping vs. an app-wide chord (`✏️editor/🦀️.rs`). Not fixed — a guest fix cannot be
restaged right now (§6) and the fix needs a product decision about which layer owns it.

### 5.3 `Export Document……` — double ellipsis — **FIXED, runtime-proven**

The shell appends `…` to every action row that carries staged args
(`🛠️ShellHelpers/🟦️.tsx:3854`), and `exportDocument` declared its own, so the Actions pane painted
`Export Document……` / `Dokument exportieren……`. Removed the declared ellipsis in
`✏️editor/🦀️.rs:2255` and `👁️viewer/🦀️.rs:1548`; `importDocumentRequest` keeps its own (argless, so
the shell adds none). New law `staged_argument_actions_declare_no_trailing_ellipsis` checks both
surfaces in both languages — passes. Runtime: `gaps/actions-pane-de` green.

### 5.4 Inspection panel has no editable control — **FIXED in the tree, NOT runtime-proven**

**Symptom.** With a slider selected the panel paints `Widget`, `Id: height`, `Range: 0..10` — and no
number input. `procedural-play-inspector.value.input` is absent from the DOM, so a slider parameter
cannot be edited from the inspector at all.

**Root cause.** The control was authored as a sibling `field` inside the panel section. A section
keeps only `treeItem` children (`collectTreeItems`, `🗣️Interpreter/🟦️.tsx:1272-1280`); a row's
NON-`treeItem` children are what the renderer mounts as its inline controls
(`collectTreeItemControls`, `:1286-1294`). A bare `field` beside the rows is silently dropped.

**Fix.** `✏️editor/📌️panels/🔍️inspection/🦀️.rs` — the value row is now a `tree_item` carrying the
input as its child. `cargo check -p semio-s-artifact-procedural-generation3d --features
component-app-assembly --lib` is clean.

**Not claimed:** the law `inspector_slider_control_rides_on_a_tree_row` was written but **could not be
run**, and the fix could not be restaged — §6. `gaps/inspection-edit` is still red on the served
(pre-fix) guest.

### 5.5 Example switching froze after the 4th example — diagnosed here, **fixed by another lane**

The first battery run caught it: from `Box Fillet Preview` on, the flow window's published widget set
stayed frozen at the Sphere Cut set for all five remaining examples (121 s each, never converging)
while the picker label and the preview kept moving. The console names the cause exactly
(`🗑️generated/react-verify/journey/console.txt`, t=37393 ms):

```
error typed-operation completion effects failed SemioFaultError:
  1:procedural-main: ResidentCredit { required: 2517394, aggregate: 33548883 }
    at retainedUiRefreshEffects (…/📺️renderer/🧑‍🎨engine/…)
```

One refused resident-credit repricing froze that surface permanently. Handed to the coordinator's
`react-example-switch-regression` lane, which root-caused it to `UI_RESIDENT_AGGREGATE_BYTES` being
only 4× the per-surface ceiling (`📓️react-example-switch-regression-2026-09-13.md`) and fixed it. The
final journey above (23/23) is on that fix.

---

## 6. What is NOT claimed

- **The inspection-panel fix (§5.4) is unproven at runtime.** No restage has succeeded since it
  landed: `activate-generation3d-react-dev` aborts in the plugin-descriptor probe with
  `app-definition.invalid: app s.procedural.generation3d@1/*#editor tool previewEval is not
  referenced by any mode` — a live, in-flight addition from ticket
  `26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS` (`✏️editor/🦀️.rs`, mtime 02:12). Three attempts,
  `🗑️generated/react-verify/restage.txt`. Everything else in this report is on the stage that lane's
  01:4x restage produced.
- **`inspector_slider_control_rides_on_a_tree_row` has never executed.** The crate's `--lib` builds
  clean, but the `--lib` TEST binary is broken by the same peer's mid-rename of `preview_eval`
  (`CancelPreviewEval`, `cancel_preview_eval_for`, `PREVIEW_CANCEL_ACTION_ID`, plus arity changes) in
  `🎮️commands/⏱️flow-eval-tick/🧪️tests/`, `🎮️commands/✅️flow-eval-resolve/🧪️tests/🔬️budget/`,
  `👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/` and `✏️editor/🧪️tests/🔬️unit/`. The other four
  laws in §5 did run and pass.
- **Nothing about the wgpu renderer (`:6118`)** — this lane is React only.
- **No claim that every example is pixel-correct.** The journey asserts the published widget set, the
  picker label and mesh COUNT, not geometry; geometry is `example-geometry`'s job.
- **`gaps/catalogue-add` proves one kind** (`brep.bool.compoundCut`). `🎮️commands/🧩️add-widget`
  answers `Emit::default()` when `host.add_widget` fails, so a kind that refuses is silent; the probe
  tries candidates in order and reports each, but the roster is not exhaustively swept.
- **Page-error counts are per-probe totals**, including benign repeats; the 5 in the final run are 1
  each in journey / flow-wire / status-parity and 2 in gaps, none fatal to their steps.
- Three probes (interact, generate-mode, export-encoding) failed once in `battery-7` purely from
  contention with a peer's concurrent journey run on an intermediate broken stage; they are green in
  the final run and that earlier run is not part of the scoreboard.

---

## 7. Files

Created:
- `🐍️react-battery.mjs` — the sequential battery + scoreboard
- `🐍️react-gap-probe.mjs` — the ten uncovered-gap steps
- `🐍️react-gap-recon.mjs` — the DOM recon that aimed it
- `📜️restage-react-verify.sh`
- `✏️s/…/🧊️generation3d/…/✏️editor/📌️panels/🗿️artifact/🧫️fixtures/🔬️unit/🔣️.json` — the document-row law
- `📓️react-end-to-end-verification-2026-09-13.md` (this report)

Updated:
- `✏️s/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs` — per-row `interactionSelect`/`interactionHover` (§5.1)
- `✏️s/…/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` — the document-row law
- `✏️s/…/✏️editor/📌️panels/🔍️inspection/🦀️.rs` — the value control rides on a tree row (§5.4)
- `✏️s/…/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` — the inspection-row law (unrun, §6)
- `✏️s/…/✏️editor/🦀️.rs` — `exportDocument` label (§5.3)
- `✏️s/…/👁️viewer/🦀️.rs` + `👁️viewer/🧪️tests/🔬️unit/🦀️.rs` — the same label, and its assertion
- `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `staged_argument_actions_declare_no_trailing_ellipsis`
- `✏️s/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs` — `graph_targets_json` made `pub(crate)`
- `🐍️customization-persistence-probe.mjs` — reads the scope's `data-ui-appearance`, not `body`'s
  background (the old predicate reported a flip that HAD applied as unchanged)

Generated (delete with the ticket): `🗑️generated/react-verify/`.
