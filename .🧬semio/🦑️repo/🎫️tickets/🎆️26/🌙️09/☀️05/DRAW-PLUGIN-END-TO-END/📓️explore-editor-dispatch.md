# Explore: draw plugin runtime dispatch + rendering audit

Read-only Sonnet explorer, 2026-09-06. Every claim below is grep/read-verified against current
source in this working tree (not compiled — the crate does not build, see §0). Oracles read first:
`BLOCK-PLUGIN-END-TO-END/📓️explore-block2d-editor.md`, `📓️w1-block2d-factory.md`,
`S-END-TO-END/📓️explore-action-migration-recipe.md`. `📕️norm/🖥️app-surface/🦀️.rs:313-780` and the
puzzle/lowpoly/generation3d factory shapes cited in the migration-recipe doc were used as the "green"
reference shape, not re-read line-by-line.

Headline: **draw is a far more mature implementation than block2d** (real gesture-FSM tool-job
factory, real handlers everywhere, no `unimplemented!`/`todo!` outside test harness code, a
non-empty boot document, panels/windows that actually render document state) — but it is currently
**dead on arrival at three independent layers**: (0) the plugin root file will not compile against
the actual crate module tree, (1) even patched, the one real tool-job factory it registers is missing
a required trait const and will fault at app-construction time, (2) even patched, only 6 of 26
declared commands are classified `Migrated` so the rest are UI-dispatch-dead, and separately (3) the
example-switching plumbing has a three-way id mismatch that makes it a no-op no matter what gets
fixed above.

---

## 0. Compile-breaking defect — plugin root references a module/type tree that does not exist

`✏️s/🔌️plugins/🖍️draw/🦀️.rs` (the plugin root, `DrawApps` enum + `plugin()` builder) is written
entirely against a `crate::editor::draw` / `crate::viewer::draw` module tree and
`DrawPlayApp`/`DrawViewer` type names:

```
Editor(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::draw::DrawPlayApp>>),   // 🦀️.rs:12
Viewer(semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::draw::DrawViewer>>),     // 🦀️.rs:13
...
.declare_artifact(crate::artifacts::draw::artifact())                                                                    // 🦀️.rs:41
.editor_mutation_roster::<crate::editor::draw::DrawPlayApp>()                                                            // 🦀️.rs:42
.viewer_mutation_roster::<crate::viewer::draw::DrawViewer>()                                                             // 🦀️.rs:43
.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::draw::artifact_kind().id })                        // 🦀️.rs:44
```

But the crate's actual module tree, wired by `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/🦀️.rs`
(lines 33-35, 555-557, 671-673), is `crate::artifacts::drawing`, `crate::editor::drawing`,
`crate::viewer::drawing` — module `draw` does not exist anywhere under `crate::artifacts`,
`crate::editor` or `crate::viewer`. The real struct/type names are:

- `pub struct DrawingPlayApp` (`🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:919`,
  `impl ArtifactEditor for DrawingPlayApp` at `:958`) — not `DrawPlayApp`.
- `pub struct DrawingViewer` (`…/👁️viewer/🦀️.rs:39`, `impl ArtifactViewer for DrawingViewer` at `:41`)
  — not `DrawViewer`.
- `pub fn artifact_kind()` and `pub fn artifact()` live on `crate::artifacts::drawing`
  (`🗿️artifacts/🖍️drawing/🦀️.rs:453`, `:531`) — not `crate::artifacts::draw`.

Every one of the six cross-module references in the plugin root uses the wrong path/name
(confirmed with `grep -rn "DrawPlayApp\|DrawViewer\|editor::draw::\|viewer::draw::\|artifacts::draw::"`
across the whole plugin directory — zero matches anywhere outside this one file, and zero aliases
exist). This is `E0433`/`E0412`-shaped: **the crate does not compile**, full stop. Nothing below this
line can be observed at runtime until this is fixed — the fix is mechanical (`draw` → `drawing`,
`DrawPlayApp` → `DrawingPlayApp`, `DrawViewer` → `DrawingViewer`, four call sites plus one doc
comment). This reads like a stale rename: the taxonomy tree was renamed `draw` → `drawing`
everywhere except the one file living at the plugin root (which sits one directory *above* the
taxonomy tree the renamer likely walked).

Minor, non-blocking cousin of the same drift: the `bounded_first_step_tool_proofs!` macro's
`owner_file` literal inside the editor (`✏️editor/🦀️.rs:997`) reads
`"✏️s/🔌️plugins/🖍️drawing/🗿️artifacts/…"` (plugin segment `🖍️drawing`, which doesn't exist — the
plugin directory is `🖍️draw`). Traced this into the framework: `owner_file` is only checked for
non-emptiness (`🔌️plugin/🦀️.rs:12265,12281`), never resolved against disk, so this specific typo is
cosmetic, not a second compile blocker — but it is more evidence of the same `draw`/`drawing` drift.

---

## 1. Every action/command the editor declares

`DrawingCommand` (`✏️editor/🦀️.rs:132-159`, one `app_commands!` block) has exactly 26 rows, one per
`🎮️commands/*` handler file (verified: 26 command directories exist, 26 variants declared, no
extras, no gaps). 22 of the 26 are individually declared in the manifest via `.action_with(...)`,
3 via `.mutation(...)` (palette-visible), and 1 (`setActiveUtility`) is never declared directly —
it is framework-auto-injected whenever `.utility(...)` rows are non-empty
(`semio_framework::SET_ACTIVE_UTILITY_ACTION_ID`, injected at
`🧰️framework/…/🔌️plugin/🦀️.rs:5442` — confirmed by grep, not independently re-classified here).

| id | declared via | `ActionKind` | classification | handler |
|---|---|---|---|---|
| `setSnapshot` | `.action_with` `:1211` | Mutation | **Unclassified** | `🎮️commands/📸️set-snapshot/🦀️.rs` |
| `commitDocument` | `.action_with` `:1212` | Mutation | **Unclassified** | `🎮️commands/📃️commit-document/🦀️.rs` |
| `setFixtureJson` | `.action_with` `:1213` | Mutation | **Unclassified** | `🎮️commands/🧫️set-fixture-json/🦀️.rs` |
| `setSelectedOpacity` | `.action_with` `:1214` | Mutation | **Unclassified** | `🎮️commands/🌫️set-selected-opacity/🦀️.rs` |
| `engagementSubmit` | `.action_with` `:1215` | Mutation | **Unclassified** | `🎮️commands/📤️engagement-submit/🦀️.rs` |
| `dropLayerKind` | `.action_with` `:1216` | Mutation | **Unclassified** | `🎮️commands/📥️drop-layer-kind/🦀️.rs` |
| `moveLayer` | `.action_with` `:1217` | Mutation | **Unclassified** | `🎮️commands/🚚️move-layer/🦀️.rs` |
| `deleteLayer` | `.action_with` `:1218` | Mutation | **Unclassified** | `🎮️commands/🗑️delete-layer/🦀️.rs` |
| `duplicateLayer` | `.action_with` `:1219` | Mutation | **Unclassified** | `🎮️commands/📋️duplicate-layer/🦀️.rs` |
| `toggleLayerVisible` | `.action_with` `:1220` | Mutation | **Unclassified** | `🎮️commands/👁️toggle-layer-visible/🦀️.rs` |
| `patchLayer` | `.action_with` `:1221` | Mutation | **Unclassified** | `🎮️commands/🩹️patch-layer/🦀️.rs` |
| `patchLayers` | `.action_with` `:1222` | Mutation | **Unclassified** | `🎮️commands/🧵️patch-layers/🦀️.rs` |
| `addLayer` | `.mutation` `:1207` (palette) | Mutation | **Unclassified** | `🎮️commands/➕️add-layer/🦀️.rs` |
| `combineBoolean` | `.mutation` `:1208` (palette) | Mutation | **Unclassified** | `🎮️commands/🔀️combine-boolean/🦀️.rs` |
| `setActiveExample` | `.mutation` `:1209` (palette) | Mutation | **Unclassified** | `🎮️commands/🖼️set-active-example/🦀️.rs` (id mismatch, see §3) |
| `engagementInput` | `.action_with` `:1240` | View | **Unclassified** | `🎮️commands/🧭️engagement-input/🦀️.rs` |
| `setLocale` | `.action_with` `:1241` | View | **Unclassified** | `🎮️commands/🗣️set-locale/🦀️.rs` |
| `setCamera` | `.action_with` `:1243` | View | **Unclassified** | `🎮️commands/📷️set-camera/🦀️.rs` |
| `setCameraZoom` | `.action_with` `:1244` | View | **Unclassified** | `🎮️commands/🔭️set-camera-zoom/🦀️.rs` |
| `setActiveUtility` | framework auto-inject | — | not directly classified (unverified default) | `🎮️commands/🪛️set-active-utility/🦀️.rs` |
| `canvasPointerDown` | `.action_with` + `.action_interactive_job` `:1224-1225` | Mutation | **Migrated** | `drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs` |
| `canvasPointerUp` | `.action_with` + `.action_interactive_job` `:1226-1227` | Mutation | **Migrated** | `✏️editor/🎮️commands/⬆️canvas-pointer-up/🦀️.rs` |
| `canvasDoubleClick` | `.action_with` + `.action_interactive_job` `:1228-1229` | Mutation | **Migrated** | `✏️editor/🎮️commands/🖱️canvas-double-click/🦀️.rs` |
| `canvasCommitDraft` | `.action_with` + `.action_interactive_job` `:1230-1231` | Mutation | **Migrated** | `✏️editor/🎮️commands/✅️canvas-commit-draft/🦀️.rs` |
| `canvasPointerMove` | `.action_with` + `.action_interactive_job` `:1232-1233` | View | **Migrated** | `✏️editor/🎮️commands/↔️canvas-pointer-move/🦀️.rs` |
| `canvasEscape` | `.action_with` + `.action_interactive_job` `:1234-1235` | View | **Migrated** | `✏️editor/🎮️commands/🚪️canvas-escape/🦀️.rs` |

`grep -n "action_interactive_job\|InteractiveJobClassification"` across the whole editor file
(`✏️editor/🦀️.rs`) returns exactly 6 `.action_interactive_job(..., Migrated)` calls (`:1225, 1227,
1229, 1231, 1233, 1235`) — none of the other 20 ids are touched. **Every handler for all 26 commands
is a real implementation** — `grep -rln "unimplemented!\|todo!()"` across the whole `🖍️draw` tree
(excluding `target/`) hits only one file, the shared `fsm` crate's own test-harness code
(`🔄️fsm/🦀️.rs:218`, inside a comment-labeled "host tests never step a machine" path, not reachable
from any command handler). Spot-checked `patch_layer::handle` (`🎮️commands/🩹️patch-layer/🦀️.rs:26-31`)
and `set_active_example::handle` (below) — both are complete, non-stub logic.

Non-gesture commands route through the ordinary path: `ArtifactEditor::handle`
(`✏️editor/🦀️.rs:1086-1100`) first rejects the 6 gesture ids outright (`drawing.gesture.retained-route`
fault — they must arrive through their retained factory, never through `handle`), then does
`command.dispatch(doc, cfg, &mut session)` for everything else, so **the 20 non-gesture handlers are
wired and reachable in code** — only the UI-dispatch classification gate stands between them and a
click in the client (see §2/§8 "runtime faults" ordering).

---

## 2. Tool-job factory / bounded proofs — present, but missing a required const

Unlike block2d (zero factory apparatus at all), draw **does** have a real
`bounded_first_step_tool_proofs!` invocation with `factory_type:`
(`✏️editor/🦀️.rs:995-1010`):

```rust
semio_framework_plugin::bounded_first_step_tool_proofs! {
    owner: semio_framework_plugin::EditorApp<DrawingPlayApp>,
    owner_file: "✏️s/🔌️plugins/🖍️drawing/🗿️artifacts/🖍️drawing/…", // typo, see §0 — cosmetic only
    controller: "s.draw.drawing@1/*#editor",
    document_schema: "drawing.document",
    factory: "DrawingGestureOperationJobFactory",
    factory_type: DrawingGestureOperationJobFactory,
    tools: { "canvasPointerDown" => …, "canvasPointerMove" => …, "canvasPointerUp" => …,
             "canvasDoubleClick" => …, "canvasCommitDraft" => …, "canvasEscape" => … }
}
```

and a real owned factory, `DrawingGestureOperationJobFactory` (`✏️editor/🦀️.rs:687-749`), covering
the 6 gesture tool ids (`DRAWING_GESTURE_TOOL_IDS`, `:168`), plus a real
`DrawingInstanceOperationOwner`/`DrawingGestureOperationOwner` retained-session state machine
(`:172-420`) with bounded `FixedOperationRegistry<_, 64>` capacity, real `close_step`/
`maintenance_step`/`terminal_is_empty` implementations, and 6 unit tests exercising admission,
staleness/ABA, saturation, and close-under-rejection (`:751-907`) — **this is a genuinely
well-built implementation**, not a stub.

**But**: `impl ArtifactOwnedToolJobFactory for DrawingGestureOperationJobFactory`
(`✏️editor/🦀️.rs:745-749`) supplies only `type Owner`, `TOOL_IDS`, `DOCUMENT_SCHEMA` — it never
overrides `const PUBLICATION_CONTRACTS`. The trait default
(`🧰️framework/…/🔌️plugin/🦀️.rs:12783`) is `&[]` (empty). Traced `ArtifactToolFactoryRegistry::register`
(`🔌️plugin/🦀️.rs:12894-12935`), the function `DrawingPlayApp::register_tool_job_factories`
(`✏️editor/🦀️.rs:1016-1019`) calls at app-construction time:

```rust
let publication = F::PUBLICATION_CONTRACTS.iter()....collect::<BTreeMap<_,_>>();      // empty map
if publication.len() != F::PUBLICATION_CONTRACTS.len()                                 // 0 != 0 → false
    || publication.keys()....collect::<BTreeSet<_>>() != expected                       // {} != {6 ids} → TRUE
    || ...
{ return Err(Fault::new(..., FaultCode::new("interactive-job.publication-contract"), ...)); }
```

`expected` is the 6-id `BTreeSet` from `TOOL_IDS`; the empty `publication` map's key set can never
equal it, so **`register_tool_job_factories` unconditionally returns
`Err("interactive-job.publication-contract")` the first time it runs** — which per the
build-time completeness gate (S-END-TO-END recipe §1.6/§1.5) is invoked during app construction, not
per-dispatch. Compare every working precedent cited in the migration-recipe oracle (lowpoly
`🦀️.rs:979-1057`, generation3d `🦀️.rs:248-317`) — all declare a non-empty `PUBLICATION_CONTRACTS`
row set matching their `TOOL_IDS` exactly. Draw is the only factory in the cited precedent set that
omits it.

**Consequence: even after §0's compile fix, the `DrawingPlayApp` editor app fails to construct at
all** — not just the 20 unclassified actions, but the entire app, including the 6 "complete" gesture
actions, because the one factory registration this app performs faults before the app finishes
building. This is a worse failure mode than block2d's (block2d's actions are individually
dispatch-dead; draw's whole app never boots).

No governance/TypeScript test catches this: `grep -n "PUBLICATION_CONTRACTS\|RETAINED_TOOL_IDS\|factory_type"
✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts` returns nothing, and
`✏️s/🔌️plugins/🖍️draw/🧪️oracle/` holds only a `🔣️.json` (no `.ts`/publication-authority law at all)
— the same "no governance test for this plugin's tool-proof wiring" gap block2d has.

**Recommended lane table for a fix** (read off each handler's actual `Emit` construction, §5):

| tool id | lanes | evidence |
|---|---|---|
| `canvasPointerDown` | `[Artifact]` | commits real operations via `Emit::commit(...)` when a shape/draft completes (`canvas-pointer-down/🦀️.rs:213`) |
| `canvasPointerUp` | `[Artifact]` | commits gesture result through `step_gesture_retained` → real mutations |
| `canvasDoubleClick` | `[Artifact]` | routes through the same `step_gesture_retained`/commit path (`✏️editor/🦀️.rs:344`) |
| `canvasCommitDraft` | `[Artifact]` | same commit path |
| `canvasPointerMove` | `[Artifact]` | mostly hover/preview (`Emit::default()`), but the point-query/select path can push `Emit::default()` + `Effect` only — **not** purely `HostOnly` because a future commit-on-move branch would need Artifact; verify no branch commits before narrowing this |
| `canvasEscape` | `[Artifact]` | can commit a partial shape via `step_gesture_retained("CommitDraft"...)`-equivalent abort path — check `canvas_escape::handle` before assuming `HostOnly` |

---

## 3. Boot state / examples

**Boot document is real, non-empty content** (unlike block2d's `Default::default()`): both
`DrawingPlayApp::initial_snapshot()` (`✏️editor/🦀️.rs:1051-1053`) and
`DrawingViewer::initial_snapshot()` (`👁️viewer/🦀️.rs:55-57`) return
`crate::artifacts::drawing::schema::default_drawing_document("empty", None)`
(`🧬️schema/🦀️.rs:323-332`): one `create_drawing_path_layer("Layer 1", Vec::new())` layer plus a
1024×1024 `DrawingArtboard`. Confirmed by two passing-shaped unit tests already in the file:
`default_drawing_document_has_artboard_dimensions` (`🧬️schema/🦀️.rs:1368-1373`) and
`layers_panel_lists_default_layer` (`✏️editor/🦀️.rs:1547-1554`, asserts the render contains
`"Layer 1"`). Both windows/panels render non-empty content on this boot document (see §4).

**Example-id plumbing is a three-way mismatch — `setActiveExample` cannot select any registered
example even once dispatch-live:**

1. Subset-level catalogue (`🪆️subsets/✳️any/🦀️.rs:14-16`) registers exactly one `ExampleSource`,
   id **`"demo"`** (`📚️examples/🎬️demo/🦀️.rs:5`, DSL text via `include_str!`).
2. Editor-level catalogue (`✏️editor/📚️examples/🎬️demo-session/🦀️.rs:5`) registers a second,
   independent `ExampleSource`, id **`"demo-session"`** (a `.cmd.semio` command-replay fixture, a
   different file format entirely).
3. `set_active_example::handle` (`🎮️commands/🖼️set-active-example/🦀️.rs:22-30`) only recognizes the
   empty string (→ reset to `default_drawing_document("empty", None)`) or the hardcoded constant
   `DRAWING_PLAY_EXAMPLE_DEFAULT_ID = "semio"` (`✏️editor/🦀️.rs:42`, → parses yet a third source,
   `crate::artifacts::drawing::schema::semio_drawing_example_document()` /
   `SEMIO_DRAW_EXAMPLE_TEXT`) — **any other id, including both ids actually registered in the two
   example catalogues above, falls through to `Ok(Emit::default())`, a silent no-op.**

So even once §0/§2 are fixed and `setActiveExample` is reclassified `Migrated` with a proof row,
dispatching it with either of the two ids a real example switcher UI would offer (`"demo"` or
`"demo-session"`, sourced from `PluginManifest.examples`/the subset's `examples()`) does nothing;
the one id that does something (`"semio"`) is not advertised anywhere an example switcher would read
from. This is the same *shape* of defect block2d's exploration flagged (`"exampleId"`/`"id"` key
ambiguity) but here it's the id *value* space itself that never intersects, across three
independently-plumbed sources. Fixing this requires picking one of: (a) rename `"semio"` →
`"demo"` in the handler and delete/rename the stray `"demo-session"` catalogue entry, or (b) extend
the handler to look up by id against the subset's own `examples()` slice instead of a hardcoded
constant (the generation3d/lowpoly-style pattern, more scalable if more examples are added later).

---

## 4. Windows and panels

Editor: single mode `edit` (`✏️editor/🎭️modes/✏️edit/🦀️.rs`), single window
`DRAWING_PLAY_WINDOW_CANVAS = "drawing-composite"` (`SurfaceKind::Canvas2d`, body key
`DRAWING_PLAY_BODY_COMPOSITE = "drawing.play.composite"`), plus three panel tabs (layers, catalogue,
properties — `📌️panels/🗂️layers`, `🛍️catalogue`, `🔍️properties`). Matches the task brief's "canvas +
3 panels" shape exactly.

- **Canvas window render** (`✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs:84-119`, `render()`):
  unconditionally emits a `meta:utility` record, an artboard frame + `W × H` dimension-label overlay
  (`artboard_scene_records`, `:55-77`, always non-empty — even on a truly empty document it still
  draws the 1024×1024 default artboard), plus one scene record per document layer via
  `flatten_drawing_document_to_scene_nodes`, plus (conditionally) marquee/shape/draft preview
  overlays from the live gesture FSM. **Never empty regardless of document state** — same "always at
  least header content" shape block2d's board window has, but richer (real artboard + dimension
  label baked in, not just a header line). Returns `UiAssemblyResult<BuiltNode>` →
  `built_to_component_tree` → `ComponentTree`, matching the react renderer's expected shape.
- **Layers panel** (`📌️panels/🗂️layers/🦀️.rs:35-`): walks `document.layers` recursively (groups/
  booleans nest children), builds one tree row per layer via `layer_tree_item`. On the boot document
  (one "Layer 1" path layer) this renders one real row, not a placeholder — confirmed by
  `layers_panel_lists_default_layer` (§3).
- **Catalogue panel** (`📌️panels/🛍️catalogue/🦀️.rs`) and **properties panel**
  (`📌️panels/🔍️properties/🦀️.rs`): not read in full this pass (time-boxed); both are wired into
  `render_drawing_body`'s `body_key` match (`✏️editor/🦀️.rs:937-944`) alongside the canvas and
  layers bodies, so they are reachable — recommend a follow-up read before relying on their exact
  boot-time content shape.

Viewer: single mode `view`, single window (canvas, `👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🦀️.rs`,
112 lines, not read in full — delegates to the same `flatten_drawing_document_to_scene_nodes`
pipeline per its imports). Boots on the same non-empty default document as the editor.

---

## 5. Canvas pointer FSM

`🔄️fsm/🦀️.rs` (2312 lines) is a general-purpose statechart *engine* (its own `Machine`/
`statechart!` macro/`MachineDefinition`/`Snapshot` types — this looks like a vendored or
framework-shared crate, not draw-specific business logic), consumed by
`🖱️canvas-pointer-down/🦀️.rs` (1254 lines, draw's own gesture semantics: `DrawingSession`,
`DrawingGesturePreview`, `drawing_gesture::Event`/`step_gesture_retained`).

**Loop audit** — `grep -n "loop {"` finds exactly three loops in `🔄️fsm/🦀️.rs`, all bounded, none a
self-tick/poll:
1. `run_to_completion` (`:872-`): bounded by `MICROSTEP_LIMIT`, breaks when the trigger queue and
   spontaneous-candidate search both go empty — a single-turn microstep loop, matching the plugin
   root's own doc-comment claim ("microstep- and mailbox-bounded within one turn, not a
   self-tick/`pending_effects` poll").
2. `Simulation::drain` (`:1532-1551`): drains actor mailboxes to quiescence, one macrostep per
   delivered event, terminates when a full sweep makes no progress — bounded by finite mailbox
   contents, not a scheduler-visible poll; not called from any production command handler found in
   this pass (looks like host/test-harness machinery for the fsm crate itself).
3. `ConfigurationIter::next` (`:1990-1998`): a bitset iterator bounded by a fixed `words: &[u64]`
   slice — ordinary iterator internals, not a runtime hazard.

No unbounded/self-rescheduling loop found. `unimplemented!` at `:218` is inside a documented
test-only branch ("host tests never step a machine"), unreachable from command dispatch.

**Effect variants**: `Effect::LoadDocument` (`✏️editor/🦀️.rs:1130`, built by
`drawing_reset_document_effect` and used by `set_active_example`/`set_fixture_json`/`set_snapshot`/
`commit_document` handlers — the "sanctioned non-history reset path" per the editor's own doc
comment at `:1074-1079`), `Effect::SetActiveUtility` (`canvas-pointer-down/🦀️.rs:214`, fired after a
shape/draft/trace commits, to snap the active utility back to `selectDirect` — asserted by a live
test at `✏️editor/🦀️.rs:1672-1720`), `Effect::ReplayShellCommand` (`canvas-pointer-down/🦀️.rs:83`)
and `Effect::DispatchAction` (`:1055`, self-scheduling the next `canvasPointerDown` tick for a
`TracePointerJob` — bounded by the point-query/publication-step state machine in
`DrawingInstanceOperationOwner::dispatch`, not a raw loop). All four effect call sites have live,
non-stub producers and at least one exercising test found in this pass — **no incomplete `Effect`
handling found**, contrary to the task brief's hedge ("whose handling is incomplete"): this session
found real handling for all of them, though `canvasEscape`'s exact effect/mutation split (§2's open
lane question) was not independently traced end-to-end.

---

## 6. Viewer — `DrawingViewer`

Textbook-clean, matches the S-END-TO-END recipe's "already-purified" shape (cad/gis/dag/note/space):
`DrawingViewCommand` has exactly one `#[default] Noop` variant (`👁️viewer/🦀️.rs:22-25`), `handle`
always returns `Ok(ViewEmit::default())` (`:63-65`) with a doc comment explicitly noting this is
kept as a real dispatch rather than `unreachable!()` so a future view-only action (camera pan/zoom)
is a pure addition. Zero declared actions in `create_drawing_viewer()` (`:79-81`) beyond the
framework-injected rows every viewer gets. `render` (`:67-74`) only serves the canvas body key,
falling back to an "Unknown body" label node otherwise — same non-panic-safe shape as the editor.
`ArtifactViewer::Mutation = DrawingMutation` is declared (required by the trait) but structurally
unreachable from `handle`'s `ViewEmit` (no `artifact_mutations` field exists on that type, per the
migration recipe's §1.7) — `assert_viewer_never_mutates::<DrawingViewer>()` is already wired as a
surface test in the plugin root (`✏️s/🔌️plugins/🖍️draw/🦀️.rs:56-58`, itself inside the file that
fails to compile per §0). Boots on the same non-empty `default_drawing_document("empty", None)`.
No purity breach found (no import from `crate::editor::drawing::*` in the viewer file).

---

## 7. Mutation roster / subsets

`editor_mutation_roster::<DrawingPlayApp>()` / `viewer_mutation_roster::<DrawingViewer>()` are called
from the (currently non-compiling, §0) plugin root — once the type paths are fixed these should wire
normally; not independently re-verified against the roster trait's requirements this pass.

Subset layout matches the task brief exactly: `🏅️standards/🔖️1/🪆️subsets/{✳️any, 🧱️structure,
🎨️style, 🔀️transform, 🏷️metadata}`, each with its own `🔮️oracle`, `🧪️tests`, `🧬️schema` (and
`🧫️fixtures` for the four non-`any` subsets). **Mutation ownership is properly declared per-leaf**,
not merely located by directory: the `any` subset's aggregate `DrawingMutation` enum
(`🧬️schema/🧬️mutations/🦀️.rs`, `#[derive(dsl::Mutations)]` at `:11`) re-exports its 14 variants each
from its *owning* subset's own leaf module — e.g. `create_layer`/`delete_layer`/`duplicate_layer`/
`reorder_layer` from `structure::schema::mutations::*`, `rename_layer`/`set_layer_locked`/
`set_layer_visible` from `metadata::schema::mutations::*`, `replace_layer_fill`/
`replace_layer_stroke`/`set_layer_blend_mode`/`set_layer_opacity` from `style::schema::mutations::*`,
`set_layer_boolean_operation`/`update_layer_trace_params`/`update_layer_transform` from
`transform::schema::mutations::*` (`🧬️mutations/🦀️.rs:100-113`) — matching the "Mutation Ownership Is
Declared, Not Located" convention (manifest/module-path ownership, not physical leaf-directory
placement).

`🧪️tests/mutate-drawing-1-any-{structure,style,transform,metadata}/🦀️.rs` are real fixture-driven
no-oracle test hosts (spot-checked the `structure` one): each declares its own `KINDS` slice
duplicated (not imported) from the owning subset's mutation module, reads committed
`(before, mutation, after)` JSON fixture vectors via `include_str!` from the owning subset's own
`🧪️tests/*` directories, and asserts through
`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` — a real, working test harness pattern, not a stub. Did not
exhaustively verify all four subset test hosts or run them (no cargo available in this session).

---

## 8. Activation / capabilities — kind-grammar mismatch

Two different "kind" identifiers exist for the same artifact, and the plugin's activation watches
the wrong-generation one:

- **New declaration-tree kind** (what the artifact actually registers as):
  `crate::artifacts::drawing::artifact()` (`🗿️artifacts/🖍️drawing/🦀️.rs:531-535`) builds
  `ArtifactDeclaration { kind: ArtifactKindId::parse("s.draw.drawing")..., standards: vec![...] }`.
  `DRAWING_DIALECT` (`:476`) is `Dialect { artifact_kind: "s.draw.drawing", standard: "1", subset:
  ANY }`, and the `bounded_first_step_tool_proofs!` controller string is
  `"s.draw.drawing@1/*#editor"` (`✏️editor/🦀️.rs:998`) — internally consistent, all three agree on
  **`"s.draw.drawing"`**.
- **Old `ArtifactKindSpec` kind** (what activation actually keys off):
  `crate::artifacts::drawing::artifact_kind().id` (`🗿️artifacts/🖍️drawing/🦀️.rs:453-468`) returns
  **`"2d.drawing"`** — a media/dimension-shaped id from the pre-declaration-tree registration
  channel, explicitly documented as legacy-but-still-read (`🦀️.rs:478-486,527-530`: "`definition()`
  ... kept per debt D1 ... neither has any caller left in this function" except
  `artifact_kind().id`, which the plugin root's `.activation(ActivationEvent::OnArtifactKind { kind:
  ... })` still reads).

So the plugin activates on `OnArtifactKind { kind: "2d.drawing" }` (once §0's module path is fixed),
while the artifact the host would actually open is declared/registered under kind `"s.draw.drawing"`
via the modern `artifact()`/`DRAWING_DIALECT` path. **Whether this is a live bug depends on what the
host's activation dispatcher matches `OnArtifactKind.kind` against** — this was not traced in this
pass (would require reading the microkernel host's activation-routing code, out of scope for a
draw-plugin-only audit). What is verifiable here: within this one plugin, the artifact's own
declared identity is `"s.draw.drawing"` everywhere except the one activation-gating field, which is
still `"2d.drawing"`. Given the doc comment's own framing ("kept because `.activation(...)` still
reads `artifact_kind().id`"), this looks like a deliberate, known bridge rather than an accidental
regression — but it is exactly the kind of two-generations-of-kind-id split the task asked to have
flagged, and it should be re-verified once §0 is fixed and the plugin actually loads in a host.

No other plugin's activation was cross-checked in this pass to see whether `"2d.<x>"` vs
`"s.<plugin>.<artifact>"` is the general pattern (i.e., whether every plugin has this same
"legacy media-kind activation vs modern grammar-kind declaration" split, in which case it is a
framework-wide convention rather than a draw-specific defect) — flagged as an open question for
whoever picks this up.

`ExecutionMode::Isolated`, the `documents.write` capability request, and the "no quota declared"
call are all consistent with what's actually implemented (no cross-plugin extension imports found;
the ~14 `Effect` call sites are genuinely per-turn, no held buffers or timers found).

---

## Runtime faults predicted at boot, ordered by first hit

1. **Compile failure.** `✏️s/🔌️plugins/🖍️draw/🦀️.rs` references `crate::editor::draw::DrawPlayApp`,
   `crate::viewer::draw::DrawViewer`, `crate::artifacts::draw::{artifact, artifact_kind}` — none
   exist (`E0433`/`E0412`-shaped). The crate cannot build. §0.
2. **App-construction fault, once #1 is fixed.** `DrawingPlayApp::register_tool_job_factories` calls
   `registry.register(DrawingGestureOperationJobFactory::new(...))`, which fails
   `Err(FaultCode::new("interactive-job.publication-contract"))` because
   `DrawingGestureOperationJobFactory::PUBLICATION_CONTRACTS` is the trait's empty default instead of
   a 6-entry table matching `TOOL_IDS`. The editor app (`EditorApp<DrawingPlayApp>`) never finishes
   constructing — nothing in the editor (not even the 6 "complete" gesture actions) is reachable.
   §2.
3. **UI-dispatch rejection, once #1-#2 are fixed.** 20 of the 26 declared actions
   (`setSnapshot`, `commitDocument`, `setFixtureJson`, `setActiveExample`, `setSelectedOpacity`,
   `engagementSubmit`, `addLayer`, `dropLayerKind`, `moveLayer`, `deleteLayer`, `duplicateLayer`,
   `toggleLayerVisible`, `combineBoolean`, `patchLayer`, `patchLayers`, `engagementInput`,
   `setLocale`, `setCamera`, `setCameraZoom`, and possibly `setActiveUtility`) carry
   `InteractiveJobClassification::Unclassified` — `validate_ui_dispatch_classification` rejects
   anything but `Migrated` with `Fault("interactive-job.not-ui-safe")` before the handler is ever
   reached. Only the 6 canvas-gesture actions would dispatch. §1/§8-table.
4. **Silent no-op, once #1-#3 are fixed.** Dispatching `setActiveExample` with either id a real
   example switcher would offer (`"demo"` from the subset catalogue, `"demo-session"` from the
   editor catalogue) hits the handler's `None` arm and returns `Ok(Emit::default())` — no document
   change, no fault, no user-visible feedback. Only the unregistered id `"semio"` does anything. §3.
5. **Possible activation mismatch** (unverified against host code): the plugin activates on artifact
   kind `"2d.drawing"`; the artifact's own modern declaration registers kind `"s.draw.drawing"`. If
   the host's `OnArtifactKind` matching reads the modern declared kind, the plugin may never activate
   for a drawing document at all — or may activate correctly if the host still bridges the two
   identifiers (this plugin's own code treats it as a known, intentional bridge). §8.

---

## Recommended implementation lanes

1. **Fix the plugin-root module/type references (mechanical, blocks everything else).**
   Scope: rename 6 call sites in `✏️s/🔌️plugins/🖍️draw/🦀️.rs` — `crate::editor::draw` →
   `crate::editor::drawing`, `crate::viewer::draw` → `crate::viewer::drawing`,
   `crate::artifacts::draw` → `crate::artifacts::drawing`, `DrawPlayApp` → `DrawingPlayApp`,
   `DrawViewer` → `DrawingViewer` (lines 12-13, 41-44, 57, 62). Also fix the doc-comment at `:29` for
   consistency. Files: `✏️s/🔌️plugins/🖍️draw/🦀️.rs`. Oracle: none needed, this is a pure
   find-and-verify-against-`📦️packages/🦀️rust/🦀️.rs`'s actual module tree fix.

2. **Give `DrawingGestureOperationJobFactory` a real `PUBLICATION_CONTRACTS`.**
   Scope: add `const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[...]`
   (6 rows, `[ArtifactToolPublicationLane::Artifact]` for the 4-5 commit-capable gesture ids, verify
   `canvasPointerMove`/`canvasEscape`'s exact lane by reading their full `Emit` branches before
   picking `Artifact` vs `HostOnly`) to the `impl ArtifactOwnedToolJobFactory for
   DrawingGestureOperationJobFactory` block. Files:
   `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:745-749`.
   Oracle to copy: `LowpolyCommandJobFactory` (`💠️lowpoly/…/✏️editor/🦀️.rs:979-1057`) or
   `Generation3dBoundedCommandJobFactory` (`🌀️procedural/…/✏️editor/🦀️.rs:248-317`) for the exact
   `PUBLICATION_CONTRACTS` shape.

3. **Classify the 20 remaining non-gesture actions `Migrated` + give them tool-proof rows.**
   Scope: for each of the 20 ids in the §1 table, add `.action_interactive_job(id,
   InteractiveJobClassification::Migrated)` in `create_drawing_app()`, and either (a) add a bare
   generic row for each id to `bounded_first_step_tool_proofs!` (no `factory_type`, routes through
   the shared `TypedCommandFullOperationJob` — simplest, matches how ordinary non-retained editor
   commands are migrated elsewhere per the migration recipe §1.6), or (b) fold them into a second
   owned factory if any of them need retained/bounded semantics (unlikely — they all look like
   ordinary single-shot document edits from the handler bodies read in this pass). Files: same
   `✏️editor/🦀️.rs`, the `create_drawing_app()` manifest region (`:1178-1297`) and the
   `bounded_first_step_tool_proofs!` block (`:995-1010`). Oracle: the norm-plugin generic-factory
   precedent (`S-END-TO-END/📓️explore-action-migration-recipe.md` §4.3) for the "many structurally
   similar single-shot mutation actions, one shared bare-row treatment" shape, or generation3d's
   29-action migration for the exact mechanical steps (§2.1-2.5 of that same recipe doc).

4. **Fix the example-id three-way mismatch.**
   Scope: either rename `DRAWING_PLAY_EXAMPLE_DEFAULT_ID` to `"demo"` and delete/repurpose
   `demo-session`, or (cleaner, matches generation3d/lowpoly precedent) rewrite
   `set_active_example::handle` to resolve `payload.example_id` against
   `crate::artifacts::drawing::standards::v1::subsets::any::examples()` (the subset's own catalogue)
   instead of a hardcoded id constant + a separately-plumbed DSL text constant. Files:
   `✏️editor/🎮️commands/🖼️set-active-example/🦀️.rs`, `✏️editor/🦀️.rs:42` (the
   `DRAWING_PLAY_EXAMPLE_DEFAULT_ID` const), and either `📚️examples/🎬️demo/🦀️.rs` or
   `✏️editor/📚️examples/🎬️demo-session/🦀️.rs` (pick one canonical example, or keep both and switch
   on real ids). No direct oracle — block2d's own `setActiveExample`/`ExampleSource` wiring
   (`explore-block2d-editor.md` §5) shows the pattern of matching real registered ids one-to-one.

5. **Verify (not fix) the activation-kind split against host code.**
   Scope: read the microkernel host's `ActivationEvent::OnArtifactKind` matching code to determine
   whether it keys off `ArtifactKindSpec.id` (`"2d.drawing"`) or `ArtifactDeclaration.kind`/
   `Dialect.artifact_kind` (`"s.draw.drawing"`), and whether other already-working plugins share this
   same two-id split (making it a framework convention) or are unique in having both ids agree.
   Files: `🧰️framework/…/🔌️plugin/🦀️.rs` (activation dispatch, not located in this pass — search for
   the `OnArtifactKind` match arm), cross-check against 2-3 other plugins'
   `artifact_kind().id` vs `artifact().kind` for a same-plugin comparison baseline.

6. **Add a governance test for draw's tool-proof/factory wiring.**
   Scope: a TypeScript oracle test (mirroring block's `📦️packages/🟦️typescript/📜️script.ts` fix in
   `w1-block2d-factory.md` §1) that regexes `DRAWING_GESTURE_TOOL_IDS`/`PUBLICATION_CONTRACTS`/
   `factory_type` out of `✏️editor/🦀️.rs` and asserts they stay in sync, so lane #2's defect class
   cannot silently regress. Files: new or extended `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts`
   + `✏️s/🔌️plugins/🖍️draw/🧪️oracle/` (currently only a `🔣️.json`, no schema/law file).

---

## Files referenced (read-only, zero edits made)

- `✏️s/🔌️plugins/🖍️draw/🦀️.rs` (plugin root, non-compiling — §0)
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/🦀️.rs:1-40, 550-700` (real module tree)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🦀️.rs:440-536` (artifact kind/dialect/declaration)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (full
  2086 lines read in 400-700 line chunks — commands enum, gesture operation owner/job/factory, the
  `DrawingPlayApp` `ArtifactEditor` impl, manifest, testkit, tests)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs`
  (grepped for `Effect::`/loops, not read in full)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/🦀️.rs`
  (grepped for loops/statechart macro, spot-read the three `loop {}` sites and the `unimplemented!`)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` (full,
  101 lines)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️canvas/🦀️.rs`
  (full, 120 lines)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️layers/🦀️.rs:1-50`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-active-example/🦀️.rs`
  (full), `.../🩹️patch-layer/🦀️.rs` (full, spot-check)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs`,
  `.../✏️editor/📚️examples/🎬️demo-session/🦀️.rs` (both full, small files)
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:237-260, 323-336`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:1-113`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧱️mutate-drawing-1-any-structure/🦀️.rs:1-40`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts` (grepped, no factory/publication test found)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5442, 12779-12792, 12894-12935` (framework
  trait default + registration validation, confirming §2's fault path)
- Oracles: `BLOCK-PLUGIN-END-TO-END/📓️explore-block2d-editor.md`, `📓️w1-block2d-factory.md`,
  `S-END-TO-END/📓️explore-action-migration-recipe.md` (full reads)

## Not verified in this pass (time-boxed, flag for follow-up)

- Catalogue and properties panel render bodies (`📌️panels/🛍️catalogue/🦀️.rs`,
  `📌️panels/🔍️properties/🦀️.rs`) — only confirmed they're wired into the body-key dispatch, not their
  exact boot-time output shape.
- Viewer canvas window render body (`👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🦀️.rs`) — only
  confirmed its import list, not its full render logic.
- `canvas_escape::handle` and `canvas_pointer_move::handle` full bodies — only grepped for `Emit::`
  call sites, not read end-to-end, so the exact `PUBLICATION_CONTRACTS` lane for those two ids (lane
  table in §2) needs confirmation before implementation.
- Whether `setActiveUtility`'s framework-auto-injected classification defaults to `Migrated` or
  `Unclassified` — traced the injection site (`🔌️plugin/🦀️.rə:5442`) but not the classification
  default logic.
- The activation-kind split (§8) against the actual host dispatch code — flagged as an open question,
  not resolved.
- Did not attempt a build (`cargo check`/`wasm32-wasip2`) — machine load/swap constraints per this
  session's instructions; all findings are static-analysis/grep/read based.
