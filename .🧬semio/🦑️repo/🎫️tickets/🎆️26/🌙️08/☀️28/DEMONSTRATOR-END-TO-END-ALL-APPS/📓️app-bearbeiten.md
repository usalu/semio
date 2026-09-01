# ✏️ Bearbeiten (`s.process.process3d@1/*#editor`) — fixture / window / interactivity diagnosis

Plugin: `✏️s/🔌️plugins/🏭️process`, artifact tree `🗿️artifacts/🧊️process3d`, standard `🔖️1`/subset `✳️any`.
Declared default example: `timber-beam-joinery` (`PROCESS3D_EXAMPLE_TIMBER`,
`.../✏️editor/🦀️component.rs:79`). Reported window body key `process.play.main` confirmed.

**Headline: exactly one window opens, and it does render a visible mesh — but the checked-in
`timber-beam-joinery` fixture text is missing the two DSL lines that carry the real stock/step
geometry, so the "beam" that appears is a generic 1×1×1 box with 0 steps, not a timber beam with
joinery cuts. Every step-editing command (add/remove/move/update/enable a step, click-to-place
cut/drill/attach, face-drag push/pull) is wired to a mutation whose `diff()` is a hard-coded,
self-documented no-op. Workshop/machine editing, stock swap, camera, sun, locale, and cursor
navigation are all real.**

## 1. Editor component, mode/window layout, default windows

`create_process3d_app()` (`.../✏️editor/🦀️component.rs:1230-1391`) builds ONE mode (`edit`,
`PROCESS3D_MODE_EDIT`) and stitches ONE window kind:

```rust
.mode_def(edit::definition())
.default_mode_id(edit::PROCESS3D_MODE_EDIT)
.window_kind_def(workpiece::definition())
.default_layout(edit::layout())
.panel_tab_def(document_panel::definition())
.panel_tab_def(catalogue::definition())
.panel_tab_def(workshop_panel::definition())
.panel_tab_def(inspection::definition())
```
(`🦀️component.rs:1254-1261`)

The mode's layout (`.../🎭️modes/✏️edit/🦀️component.rs:16-18`):
```rust
pub fn layout() -> WindowLayout {
    create_default_layout(&[workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into()], "row", None, Some(&["Workpiece".into()]))
}
```
Only one window id is in that array — this mode IS the app's `default_layout`, so exactly **one
window opens by default**:

| id | title (en/de) | body key | surface kind |
|---|---|---|---|
| `process-workpiece` | "Workpiece" / "Werkstück" | `process.play.main` | `SurfaceKind::World3d` |

(`.../🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs:16-43`, `definition()`). The four
`.panel_tab_def(...)` calls (document, catalogue, workshop, inspection) are side-panel tabs, not
separate windows — they render inside the app chrome, not as `WindowKind` entries in the layout.
Utilities `select/cut/drill/attach` are scoped to this one window
(`window_kind_utilities(...PROCESS_3D_PLAY_WINDOW_MAIN...)`, `:1338`).

## 2. `setActiveExample` — matches, but the checked-in fixture text is incomplete

`.../✏️editor/🎮️commands/📄️artifact/🦀️component.rs:39-64`, full match:
```rust
let snapshot = match payload.example_id.as_str() {
    crate::editor::process3d::PROCESS3D_EXAMPLE_PLATE | "plate" => plate_document(),
    "" => Process3dSnapshot::default(),
    _ => default_document(),
};
Ok(Emit { effects: vec![crate::editor::process3d::reset_process3d_document_effect(&snapshot)], ..Default::default() })
```
`"timber-beam-joinery"` falls into the `_` arm → `default_document()`
(`🧬️schema/🦀️component.rs:327-329`): `Process3dSnapshot::parse_dsl(TIMBER_EXAMPLE_DSL).unwrap_or_default()`,
where `TIMBER_EXAMPLE_DSL` = `include_str!(".../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio")`
(same file backs the `demo` `ExampleSource`, `📚️examples/🎬️demo/🦀️component.rs:10`).

The DSL text has 8 fields: `workshop`, `stockId` (`"beam"`), `stockLabel` (`"Timber Beam"`),
`stockPose`, `stockSolid` (a child-handle URI), `steps` (a child-handle URI), `toolSolids`,
`resolvedUpTo=null`. **It is missing `stockPayload=` and `stepPayloads=` lines** that the
hand-rolled parser (`🧬️schema/📸️snapshot/🦀️component.rs:141-192`, `print_process3d_snapshot_body`/
`parse_process3d_snapshot_body`) both prints and expects:
```rust
"workshop={}\nstockId={}\nstockLabel={}\nstockPose={}\nstockPayload={}\nstockSolid={}\nsteps={}\nstepPayloads={}\ntoolSolids={}\nresolvedUpTo={}"
```
Parsing succeeds anyway (missing keys just leave the field at its `empty_process3d_snapshot()`
seed — the parser only errors on an *unrecognized* line, `:184-186`), so `stock_payload` stays
`Stock::default()` = `WorkingSolid::Box{1.0,1.0,1.0}` (`🦀️component.rs:404-405,420-424`) and
`step_payloads` stays `vec![]`. The same 8-line shape (same two fields missing) is present in
`PROCESS_3D_PLATE_EXAMPLE_TEXT` too (`📸️snapshot/📝️text/🦀️component.rs:17-25`) — both bundled
examples are stale against their own struct/codec. The round-trip test
(`timber_example_fixture_parses_and_round_trips`, `:133-136`) only checks parse→print→parse
equality, not that the payload equals the intended geometry, so it does not catch this.

## 3. Document → surface

`workpiece::render()` (`.../🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs:115-123`):
```rust
pub fn render(fixture: &Process3dSnapshot, config: &Process3dConfig) -> UiAssemblyResult<BuiltNode> {
    let (meshes_json, instances_json) = preview_payload_cached(fixture);
    MeshWindowKit::render(&MeshView { camera_json: ..., meshes_json, instances_json, selection_json: ... })
}
```
`evaluated_preview_payload` (`:92-107`) calls `process_working_scene_from_snapshot(fixture)`
(`🗿️artifacts/🧊️process3d/🦀️component.rs:826-832`):
```rust
pub fn process_working_scene_from_snapshot(snapshot: &Process3dSnapshot) -> ProcessWorkingScene {
    let mut stock = snapshot.stock_payload.clone();
    stock.id = snapshot.stock_id.clone();
    stock.label = snapshot.stock_label.clone();
    stock.pose = snapshot.stock_pose.clone();
    ProcessWorkingScene { stock, steps: snapshot.step_payloads.clone() }
}
```
— i.e. rendering reads `stock_payload`/`step_payloads` (the inline literal fields), never the
`stock_solid`/`steps` composed child *handles* (doc comment, same file, `:822-825`: "no
`LinkResolver`/`ChildStoreFactory` seam reaches `ArtifactApp::handle`"). `processed_mesh`
(`🧬️schema/💡️inferences/🦀️component.rs:239-245`) replays the scene through a real kernel
(`ProcessKernelReplay`, cut/drill/attach + tessellate) — this part is real, not a stub. But fed the
`timber-beam-joinery` fixture as parsed today, `scene.stock.solid` is the default unit box and
`scene.steps` is empty, so the kernel replay produces **a plain 1×1×1 box, zero cuts**, labeled
"Timber Beam" (label comes straight from `stock_label`, unaffected by the payload gap) — visible,
not blank, but not the intended joinery geometry. `engagement()` (`:127-167`) computes
`len = scene.steps.len()` for the stepper's `max`; with the fixture as-is that's `0`, so the
step-cursor stepper shows `"0/0 steps"` and has nothing to scrub through.

## 4. Interactivity — real vs. no-op

Real, working, no gaps found:
- **Workshop/machine editing** — add/remove/rename/re-icon/replace-capabilities all route through
  targeted mutations with real `diff()` bodies (`🧬️mutations/🏭create-machine`,
  `❌delete-machine`, `🔖rename-machine`, `🎨change-machine-icon`,
  `🔁replace-machine-capabilities` — each mutates `Workshop.machines` for real). Wired from
  `.../✏️editor/🎮️commands/🛠️workshop/🦀️component.rs:39-118`.
- **Stock swap** (`setStock`, `.../🎮️commands/🪵️stock/🦀️component.rs:23-40`) mints a whole new
  `ProcessWorkingScene`/snapshot via `process_working_scene_to_snapshot` and dispatches
  `Effect::LoadDocument` — a real, visible geometry change (box/cylinder/sphere).
- **Inspector patch** (`.../🎮️commands/🔎️inspector/🦀️component.rs:75-100`) — machine label/capability
  params and stock label/pose route to real mutations (`RenameMachine`, `ReplaceMachineCapabilities`,
  `ChangeStockLabel`, `MoveStock`). Stock *dimension* edits and any `step:<id>` target are
  explicitly `None` (documented gap, same LinkResolver reason as §6).
- **Camera / sun / locale / contributions / engagement-input** — all config-lane mutations
  (`.../🎮️commands/🎥️camera`, `☀️sun`, `🗣️locale`, `🧩️contribution`, `🎛️engagement`) — plain field
  writes on `Process3dConfig`, no LinkResolver dependency, fully real.
- **Cursor navigation** (`setCursor`/`stepCursor*`/`engagementSubmit`'s `back`/`forward`/`all`) all
  emit `ChangeCursor`, whose `diff()` is real (`🧬️mutations/⏱️change-cursor/🔺️diff/🦀️.rs:6-11`) — it
  genuinely updates `resolved_up_to`. It is just visually inert against the shipped timber fixture
  because there are 0 steps to reveal (see §3).

Stubbed (documented no-ops, not `todo!()`/`unimplemented!()` but functionally identical — every one
returns `protocol::MutationOutcome::empty().warn("mutation.no-op", ...)`):
- `addStep`, `removeStep`, `removeSelectedStep` → `CreateStep`/`DeleteStep` diffs
  (`🧬️mutations/🌱create-step/🔺️diff/🦀️.rs:12-17`, `🗑️delete-step/🔺️diff/🦀️.rs:9-12`).
- `moveStep` → `ReorderSteps` diff (`🔀reorder-steps/🔺️diff/🦀️.rs:11-13`: *"Step reorder is a
  documented no-op pending a link resolver for the composed steps child."*).
- `updateStep` → unconditionally emits `RenameStep`+`ChangeStepEnabled`+`ChangeStepOrigin`+
  `ReplaceStepMeasure`, all four are no-op diffs (`🏷️rename-step`, `🔘change-step-enabled`,
  `🧷change-step-origin`, `📐replace-step-measure`, each `🔺️diff/🦀️.rs`).
- `setStepEnabled` → `ChangeStepEnabled`, same no-op.
- `world_pointer_down` (click-to-place cut/drill/attach, `.../🎮️commands/🌍️world/🦀️component.rs:52-73`)
  and `world_face_drag_end` (push/pull face drag, `:92-107`) both build a real `ProcessStep` and call
  `insert_step_mutations(fixture, step)` → `CreateStep` → the same no-op.

Net effect: **every user gesture aimed at editing the process timeline itself (placing a cut/drill/
attach in the 3D view, dragging a face, the "Add Step" palette action, deleting/reordering/toggling/
retyping a step) is accepted by the UI, dispatches, returns `Ok`, and changes nothing** — the mutation
pipeline swallows it with a `mutation.no-op` warning. Only whole-document resets (`setActiveExample`,
`setStock`, `setSnapshot`) can put new steps into the document at all.

## 5. Extension dependency (`process-extension-{concrete,metal,robotic,wood}`)

The `timber-beam-joinery` fixture's workshop is `generic_machines()`
(`🗿️artifacts/🧊️process3d/🦀️component.rs:227-…`, ids `saw`/`drill`/`attacher`/`circularSaw`/
`tableSaw`/`bandSaw`/`chainSaw`/`drillPress`/`cncRouter`/`dowelJig`/`screwGun`, all `catalog_id:
None`) — baked directly into the DSL text, independent of any runtime extension.

The material-specific catalogs (`wood_catalog()`, `concrete_catalog()`, `metal_catalog()`,
`robotic_catalog()`) that the "install machine" catalogue panel offers are **also compiled directly
into this same core plugin crate** and always present via `builtin_installed_catalogs()`
(`.../✏️editor/🦀️component.rs:1602-1610`):
```rust
fn builtin_installed_catalogs() -> Vec<MachineCatalogs> {
    vec![GenericCatalog.into(), wood_catalog().into(), concrete_catalog().into(), metal_catalog().into(), robotic_catalog().into()]
}
pub fn installed_catalogs(contributions_json: &str) -> Vec<MachineCatalogs> {
    let mut catalogs = builtin_installed_catalogs();
    catalogs.extend(contributed_machine_catalogs(contributions_json)...);
    catalogs
}
```
The separate `process-extension-wood` (etc.) WASM modules only *add* extra catalogs via the host's
`contributions_json` (`ProgramContributionEntry`/`TopicContribution`, default `"[]"` when nothing is
contributed, `.../🎚️config/🦀️component.rs:95-96`). **`timber-beam-joinery` does not depend on
`process-extension-wood` (or any other extension) being loaded** — the window renders, the built-in
generic/wood/metal/concrete/robotic catalogs are all still offered, and workshop editing still works
with zero extensions loaded. A missing extension only means the host-contributed *additional*
catalogs (layered on top of the always-present built-ins) don't show up; nothing goes blank or
inert. (Build-timestamp check: in this checkout both `♻️mit-bestand/🧺️demonstrator/dist/extensions/*`
and `.🦑️repo/⚡️cache/.../plugin-modules/process-extension-*` show the four extension `.core.wasm`
files built a few seconds *after*, not before, `.../plugin-modules/process/semio_s_plugin_process_component.core.wasm`
— the "older" premise did not reproduce in either dist tree I could find, though this is
architecturally moot for §5's conclusion either way.)

## 6. `todo!()`/`unimplemented!()`/`TODO`/`FIXME`/placeholder scan

No `todo!()` or `unimplemented!()` anywhere under `✏️s/🔌️plugins/🏭️process/`. No blank-window or
input-inert code path either — every command handler returns a real `Emit`. The actual gaps are the
two found above, both self-documented in code comments rather than left as bare stubs:

1. **Fixture content gap** — `timber-beam-joinery`/`drilled-plate` example DSL text is missing
   `stockPayload=`/`stepPayloads=` lines (§2), so the loaded document's *visible* geometry/step-count
   silently degrades to defaults even though the document's `stockSolid`/`steps` fields still carry
   plausible-looking (but unresolvable) child-handle URIs.
2. **Interactivity gap** — every per-step mutation's `diff()` is a hard-coded no-op "pending a link
   resolver for the composed steps child" (§4), repeated verbatim across `🌱create-step`,
   `🗑️delete-step`, `🔀reorder-steps`, `🔘change-step-enabled`, `🏷️rename-step`,
   `🧷change-step-origin`, `📐replace-step-measure` — all traceable to the same
   `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave-4 migration that turned `steps`/`stock_solid`
   into composed `ArtifactChild` handles without adding a `LinkResolver`/`ChildStoreFactory` seam
   (doc comment, `ProcessWorkingScene`, `🗿️artifacts/🧊️process3d/🦀️component.rs:518-526`).

## Verdict

1. One window opens by default: `process-workpiece` ("Workpiece"/"Werkstück"), body key
   `process.play.main`, `SurfaceKind::World3d`; four panel tabs (document/catalogue/workshop/
   inspection) sit alongside it, not as separate windows.
2. `setActiveExample("timber-beam-joinery")` resolves (via the `_` match arm) to `default_document()`
   → parses the bundled DSL text via `Effect::LoadDocument` — this part works.
3. That DSL text is missing `stockPayload=`/`stepPayloads=` lines the codec itself requires, so the
   parsed document's stock defaults to a bare `1×1×1` box with `0` steps — labeled "Timber Beam" but
   geometrically wrong, and the step-timeline stepper shows `0/0`.
4. The workpiece window is genuinely non-blank (a box renders, kernel replay is real) and camera/sun/
   locale/cursor/workshop-machine/stock-swap/inspector edits all work end-to-end.
5. Every step-level edit — add/remove/move/enable/retype a step, click-to-place cut/drill/attach,
   face-drag push/pull — dispatches successfully but is absorbed by a hard-coded `mutation.no-op` in
   every step mutation's `diff()`; the document never actually gains, loses, or changes a step.
6. No `process-extension-*` module is required for `timber-beam-joinery`: its workshop and all four
   material catalogs are compiled into the core plugin itself; a missing extension only trims the
   *extra* host-contributed catalogs, nothing breaks.
7. No literal `todo!()`/`unimplemented!()`; the two defects are self-documented ("documented gap" /
   "documented no-op") migration leftovers, not silent stubs.
8. Net user experience today: open Bearbeiten → one 3D window shows a plain box labeled "Timber Beam",
   0/0 steps; camera orbit, sun, locale, workshop machine management, and swapping the stock shape all
   work; clicking/dragging in the viewport to actually cut/drill/attach the beam, or editing/deleting/
   reordering a step, silently does nothing to the document.
9. To reach "correct fixture": add the missing `stockPayload=`/`stepPayloads=` lines (regenerate via
   `process_working_scene_to_snapshot` + `print_dsl()`, the technique already used for the plate
   example) to both bundled example DSL files.
10. To reach "interactive": land the `LinkResolver`/`ChildStoreFactory` seam the wave-4 migration
    deferred, then re-wire the seven no-op step-mutation `diff()` bodies to actually mutate the
    resolved `steps`/`stock_solid` content instead of warning `mutation.no-op`.

Findings written to:
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/📓️app-bearbeiten.md
