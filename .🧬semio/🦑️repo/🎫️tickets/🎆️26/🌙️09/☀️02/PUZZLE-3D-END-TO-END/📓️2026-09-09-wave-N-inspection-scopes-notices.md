# Wave N — Inspection body, per-command UI scopes, refusal notices, wgpu group label

Scope: the five open items W-L left in `📓️2026-09-09-wave-L-browser-interaction-defects.md` §5.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached `HEAD`, concurrent peers editing the same tree
  throughout. The coordinator owns `🔌️plugin/⚛️reactor/**`, `🖱️ui/🧠️runtime/**` and
  `🧬️contract/**/🏗️builder.rs` — untouched here.
- Cargo FOREGROUND only, `RUSTC_WRAPPER="" RUST_MIN_STACK=134217728`
  `CARGO_TARGET_DIR=…/scratchpad/target-p3d` (seeded private target). macOS has no `timeout`.
- Baseline handed over: 591 passed / 22 failed on
  `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib`.
- No browser run in this wave; no servers started.
- ⚠️ `grep` treats several files in this tree as binary (`🔌️PluginRuntime/🟦️.tsx`,
  `🏛️ShellHost/🟦️.tsx`, `🗣️Interpreter/🟦️.tsx`, …) and then prints NOTHING rather than
  "Binary file matches": every search below used `grep -a`. A `grep` without `-a` returning zero hits
  in this repo is not evidence of absence — it cost this wave a wrong first read of §5.2
  (`missingSurfaceIds` "does not exist").
- Three peer waves broke the build under me mid-flight and were waited out, not fixed:
  `UiRefreshSection::Catalogue` (non-exhaustive match in `🔌️plugin/🦀️.rs`), `NodeGraphScene`
  field renames in `🖱️ui/🎬️scene`, a root-`Cargo.toml` `workspace.dependencies.ui_contract` gap, and
  `main::PUZZLE3D_LANE_INSTANCES` + a `🚚️world3d-scene-lanes` fixture referenced by a unit test before
  either existed. Two peer breakages inside THIS crate's own testkit were repaired in place (§2.9).

## 1 Per-item cause + evidence

### 1.1 (§5.4) The gumball verbs never refused — and never ran the arm that would have

**Measured, in-process.** With the testkit's `settle` instrumented to print what it drains, a fixture
with nothing selected produced:

```
[DEBUG] settle took effect Notify { message: "Nothing is selected" }   ← duplicateSelection
[DEBUG] settle took completion operation=4 scope=None history=false
…
[DEBUG] settle took completion operation=7 scope=Full history=false    ← translateSelection
[DEBUG] translateSelection refusal effects: []
[DEBUG] settle took completion operation=8 scope=Full history=false    ← rotateSelection
[DEBUG] settle took completion operation=9 scope=Full history=false    ← scaleSelection
```

`duplicateSelection` refuses (`Notify`, scope `None`). The three gumball verbs complete with **scope
`Full` and no effect at all** — and `Full` is `Emit::default()`'s scope, i.e. the value `dispatch_step`
returns from its two bail-outs and the value nothing else in that path ever produces for a refusal.

A second probe (an `eprintln` at `dispatch_step`'s entry) settled it: `dispatch_step enter` printed for
`duplicateSelection`/`deleteSelection`/`selectSameKindSelection` and **never** for
`translateSelection`/`rotateSelection`/`scaleSelection`.

**Cause.** `build_tool_job` (`✏️editor/🦀️.rs`) routes those three ids to a dedicated retained work —
`"translateSelection" | "rotateSelection" | "scaleSelection" => Box::new(Puzzle3dScaleWork::new(tool_id))`
— and `Puzzle3dScaleWork::step` never calls `dispatch_step`. So W-L's
`refuse_without_selection` guard in `🎮️commands/🚀️translate-selection/🦀️.rs` (and its two siblings) is
**dead code on the only path that runs in production**: those three `🎮️commands/*` arms are reachable
only through `dispatch_puzzle3d_action`, which these verbs no longer enter.

What `Puzzle3dScaleWork` did instead, with an empty selection, was complete with
`Emit { artifact_mutations: [], coalesce_key: Some("gumball-translate"), ui_scope: UiDirtyScope::Full }`
— an empty edit, joined into the gesture's latest-wins coalescing group, forcing a whole-shell repaint,
with no notice. That is exactly the browser symptom W-L recorded for "Translate Selection": a menu row
that completes and does nothing.

W-L's stated reason ("the notice travels on the completion lane which the fixture does not drain") was
half right: the fixture did not drain the completion, but the notice did not exist either.

**Not observable in-process, second half.** `testkit::settle` drained
`take_typed_operation_completion()` and immediately `drop`ped it. A latest-wins command answers BEFORE
it runs (`dispatch_typed_command_inner`'s `latest_wins_target` arm returns `{operationId, generation}`
with `UiDirtyScope::None`), so its entire outcome — terminal scope included — lives on that completion.
Dropping it made every coalesced verb's `InvocationResult.ui_scope` a lie.

### 1.2 (§5.3) `panel_bodies: Vec::new()` — narrower than W-L stated, and wider than it looks

W-L reported that "`duplicateSelection`, the gumball verbs, `setSelectionFlag` etc. leave the inspection
panel stale by construction". **That is not what the code did.** `dispatch_step` started every arm at
`UiDirtyScope::Full`, and only four viewport verbs plus four command arms ever overrode it, so
`duplicateSelection` and `setSelectionFlag` repainted *everything* — wasteful, never stale.

The real defect the empty `panel_bodies` caused is one command wide, and it is a real one:

- `setFillCount` **applies planned placements** — it emits `artifact_mutations` — and declared
  `puzzle3d_fill_build_scope()`, which names no panel body at all. So committing a fill left the
  artifact outliner, the field inspector, the catalogue and the history panel showing the pre-fill
  document. Its own law (`set_fill_count_declares_narrow_ui_scope`) asserted `panel_bodies.is_empty()`,
  i.e. the defect was pinned as correct.
- `Puzzle3dScaleWork`'s terminal emit hard-coded `UiDirtyScope::Full` (§1.1), as did nine other retained
  works — ten independent spellings of the same decision.

The structural cause is that there was no place to make the decision: an author's only choices were
`Full` or a hand-written `Partial`, and every hand-written `Partial` in this app was written for a
tick loop, where `panel_bodies: Vec::new()` is correct.

### 1.3 (§5.5) `menu.group.more` renders with an empty label in the wgpu target

`shell_context_menu_item_from_spec` resolved a group row's label through `ribbon_parent_label`, which
covers exactly the 20 `RIBBON_PARENT_CATEGORIES` ids. `organize_context_menu` synthesizes the overflow
row itself as `menu.group.more` with `label: None` — `more` is not in the taxonomy, so the lookup
returned `None`, fell back to the spec's own `None`, and `unwrap_or_default()` produced `""`.
Same shape W-L fixed on the React side with `ui.contextMenu.more`; the Rust twin had no equivalent key.

### 1.4 (§5.2) The inspection panel: what was ruled out, and the one blank-panel path that is real

Both ends were already proven and are proven again here, so the defect could be narrowed rather than
guessed at:

- **The guest cannot emit an empty inspection body.** `panels::inspection::render` returns either
  `selected_section` or the `summary` fallback, and `PanelTreeBuilder::build` always produces a `tree`
  root with at least one `treeSection`. `selected_object_inspector_renders_that_object_field_group`
  (this crate, passing) drives a real pick and asserts the object field group.
- **A delivered body renders.** `uiNodeToTreePanelConfig` mounts the body under `InterpretedUiNode`
  inside the panel leaf, and the existing engine-contract law "hosts a semantic tree document inside a
  panel leaf" renders `tree → treeSection → treeItem` and finds the row text.
- **A body that has not arrived does NOT render blank** — measured, not assumed: rendering
  `pendingPanelUiNode()` through the real panel path produces
  `<div data-ui-node-id="1" data-ui-status="loading" role="status" aria-busy="true">` with a three-row
  skeleton. The interpreter's per-node `activity` wrapper already covers it. So "waiting for the first
  refresh" is not the blank panel either.
- **`refreshUi` cannot silently drop a body that was never published.** `missingSurfaceIds`
  (`🔌️PluginRuntime/🟦️.tsx`, `refreshUi`) collects every requested surface that is absent or unrooted,
  and `settlePluginTurn` **throws** `stopped without publishing requested UI surfaces` if the guest
  quiesces without publishing them. The 21:05 measurement recorded `refreshUi` **ok**, so the inspection
  surface was present and rooted at request time — and a rooted retained surface always projects
  (`projectOwnedUiSurface` returns `null` only for `view.root === null`). A mid-flight partial tree is
  impossible on that path: `acceptUiPatches` drives each patch's intake to `ready` and only then swaps
  the complete surface into `uiSurfaceByInstance`.

That leaves exactly one host-side state that renders as a silent blank rectangle, and it is real:
**a settled (`activity: "idle"`) `tree` body whose children resolve to zero `treeSection`s.** `TreeView`
(`🗣️Interpreter/🟦️.tsx`) rendered `<Tree sections={[]}/>` for it — no ring, no text, no marker, nothing
to distinguish "this body rendered nothing" from "this body was lost on the way in". Fixed at that
cause (§2.7); the remaining half of defect 6 is §5.1.

**Deliberately NOT done:** widening `missingSurfaceIds` into a revision-aware wait. The evidence above
says the read-back is not the hole — a requested surface is either absent (and then the settle throws)
or complete. Making the refresh wait for a *republication* would hang on every legitimately unchanged
body, which is the unbounded-wait shape the coordinator forbade and the shape that produced the 20:35
`4096 continuations` failures.

## 2 Changes (file:line)

1. **`✏️editor/🦀️.rs:1978-2063`** — the scope table, next to the command catalogue:
   `puzzle3d_selection_panel_bodies()` / `puzzle3d_document_panel_bodies()`, `Puzzle3dScopeClass`
   (`Quiet`/`Viewport`/`FillBuild`/`FillOptions`/`SuggestionsTick`/`FillApply`/`Selection`/`Document`/
   `Chrome`), `puzzle3d_scope(class)` and `puzzle3d_command_scope_class(action)`. An id absent from the
   table is `Chrome` (= `Full`), so a newly declared command is slow before it is wrong.
2. **`✏️editor/🦀️.rs:3025`** — `dispatch_step` now starts every arm at
   `puzzle3d_scope(puzzle3d_command_scope_class(action))` instead of `UiDirtyScope::Full`; the ad-hoc
   `ui_scope = match action { "setCamera" | … => puzzle3d_viewport_scope(), … }` override below it is
   deleted (the table says the same thing, once).
3. **`✏️editor/🦀️.rs`** — every retained work's terminal emit reads the table instead of spelling a
   scope: `:3318` (`addTargetVolume`), `:3698` (`self.tool_id`, kind weights), `:3937`
   (`addObjectKind`), `:4132` (`self.tool_id`, the gumball verbs), `:4278` (`patchInspector`), `:4682`
   (`worldRelocate`), `:5025` (`createAttraction`), `:5280` (`setActiveExample`), `:5546`
   (`addBrushObject`), `:5702` (`focusSelection`), `:5758` (`relocateTargetVolume`), `:6099`
   (`acceptSuggestion`), `:6431` (`setFillCount`). Ten hard-coded `UiDirtyScope::Full`s and three
   hand-picked partials collapse onto one decision.
4. **`✏️editor/🎮️commands/⚖️set-kind-weight/🦀️.rs`, `⏱️suggestions-tick/🦀️.rs`,
   `📋️register-brush-mesh/🦀️.rs`** — the three `*ctx.ui_scope = …` restatements that now merely repeat
   the table's own answer are removed (with their imports); `fill-build-tick`'s stay, because its
   `None`-on-idle branches are a genuine per-path decision, not a restatement.
5. **`✏️editor/🦀️.rs:3204-3213`** — `puzzle3d_notice_emit` returns `ui_scope: UiDirtyScope::None`
   instead of `Emit::effect`'s `Full` default. Nine existing placement refusals were forcing a
   whole-shell repaint for an emission that painted nothing.
6. **`✏️editor/🦀️.rs`** `Puzzle3dScaleWork` — `view_state` field + `bind_view_state` override (`:4079`),
   released in `close_step` and required by `terminal_is_empty`; and the refusal itself at `:4134`, the
   `VolumeSelection → Objects` transition: both selection cursors are exhausted there, so an empty
   `objects`+`volumes` set completes with `puzzle3d_notice_emit(…, nothing_selected)` — one localized
   notice, no mutation, **no coalesce key** (a refusal must not join the gesture's latest-wins group)
   and `UiDirtyScope::None`.
7. **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1226`,`:1245-1250`**
   — `TreeView` renders `treeStatusSection(record)` when no child resolves to a `treeSection`: the root
   record's own `activity` becomes the section's `loading`/`waiting`, and the section carries an
   `emptyState` (`ui.common.loadingSurface` / `ui.common.noData`). A tree body can no longer present as
   a blank rectangle.
8. **`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs:344-366`** —
   `CONTEXT_MENU_GROUP_ID_PREFIX`, `CONTEXT_MENU_OVERFLOW_CATEGORY` and `context_menu_group_label(id,
   is_de)` (taxonomy ids through `ribbon_parent_label`, the overflow id to `More`/`Mehr`) — the Rust
   twin of W-L's `contextMenuGroupLabel`. The four internal `"menu.group."` literals (`:383`,`:384`,
   `:508`,`:517`,`:534`) now read the consts, and `:1856` re-exports the three from `mod ui`.
   **`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:304-318`** — `shell_context_menu_item_from_spec` resolves through
   `context_menu_group_label` and keys the default folder icon on the prefix.
9. **`✏️editor/🧪️tests/🔬️testkit/🦀️.rs`** — `settle` returns `Puzzle3dSettled { effects, events, scope,
   completions }` (was a three-tuple that dropped every completion); `Puzzle3dCompletion { operation,
   ui_scope, history_patch }` is the projected `AppFrame::OperationCompleted` witness. `settle_into`
   folds the last completion's `ui_scope` in when the operation published none of its own, which is what
   makes `InvocationResult::ui_scope` mean the same thing for a coalesced verb as for a plain one.
   Also two peer breakages repaired in place (`Label` became a newtype under this wave):
   `text.value.as_str()` → `text.value.0.as_str()` at `:538` and `:642`.

New/changed laws:
- **`✏️editor/🧪️tests/🔬️unit/🦀️.rs`** `command_scope_classes_name_the_panels_they_change` — every
  declared `ActionKind::Mutation` whose scope is `Partial` must name the inspector, the artifact
  outliner and the framework history body, and must name the world body; `Full` always passes; `None`
  never does; and at least ten mutating verbs must actually be narrowed, so the law cannot be satisfied
  by falling back to `Full` everywhere.
- **`…/🔬️unit/🦀️.rs`** `selection_scope_names_the_inspector_and_not_the_catalogue`.
- **`…/🔬️unit/🦀️.rs`** `viewport_and_tick_scope_classes_name_no_panel_body`.
- **`…/🔬️unit/🦀️.rs`** `set_fill_count_declares_a_narrow_document_ui_scope` (was
  `set_fill_count_declares_narrow_ui_scope`, which asserted the defect).
- **`…/🔬️unit/🦀️.rs`** `selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice` —
  the three gumball verbs are now asserted exactly like the other three: exactly one real-prose notice,
  no mutation, `UiDirtyScope::None`.
- **`🖱️ui/🧪️tests/🔬️targets-wgpu-component-ui-ui-node-wire-format/🦀️.rs`**
  `context_menu_group_label_covers_every_group_row_organize_context_menu_can_emit` and
  `organize_context_menu_overflow_row_is_the_declared_unlabeled_group_id` (every group row the organizer
  emits resolves, in both locales).
- **`🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`** three: "renders a panel body that has not arrived yet
  as a loading surface, never as a silently empty panel"; "renders an idle tree body with no sections as
  an explicit empty state, distinct from loading"; "a partial scope naming a panel body requests that
  panel, and one that omits it does not".

## 3 Laws

1. **A verb's refusal lives where the verb actually runs.** `translateSelection` carried a
   `refuse_without_selection` guard in a `🎮️commands/*` arm that its own tool factory routes around. A
   guard on an unreachable path is not a guard — every retained work that resolves a selection itself
   owns its own refusal.
2. **A refusal never joins a coalescing group.** A latest-wins gesture folds successive invocations by
   `coalesce_key`; an empty emission carrying that key claims to be part of the drag. A refusal carries
   no key, no mutation, and `UiDirtyScope::None`.
3. **A notice-only emission paints nothing.** `Emit::effect`'s `Default` scope is `Full`, which made
   every localized refusal in this app cost a whole-shell repaint. Effects travel on their own lane; the
   host applies them before it ever consults the scope.
4. **A command's dirty scope is declared once, in a table beside the command catalogue.** Not spelled at
   each of thirteen terminal emits, and never defaulted to `Full` in the dispatcher — the class says
   what changed, the scope is derived from it.
5. **Every mutating command's scope names the panels its mutation changes.** A document edit moves the
   artifact outliner, the field inspector and the history panel. `Full` satisfies this trivially and is
   the honest answer for anything not yet classified; a `Partial` that omits one of the three is a
   guaranteed stale panel.
6. **A completion witness is part of the answer, not bookkeeping.** A command that answers before it runs
   has no outcome anywhere except its `OperationCompleted` frame; a fixture that drops it can assert
   nothing about that command, and will report a scope that was never emitted.
7. **A rendered surface never presents as nothing.** Loading, empty and lost must be three visibly
   different states. `<Tree sections={[]}/>` collapsed all three into one blank rectangle.
8. **A chrome vocabulary has exactly one string table per target, and it covers every id its own
   producer can mint.** `organize_context_menu` synthesizes `menu.group.more`, so the resolver beside
   `ribbon_parent_label` must answer for it — the taxonomy alone is not the closed set.
9. **`grep` in this repo is `grep -a`.** Several sources are classified binary and a plain `grep` then
   prints nothing at all, which reads exactly like "this symbol does not exist".

## 4 Commands + tails

```
cargo check -p semio-framework-plugin
    Finished `dev` profile [unoptimized] target(s) in 54.56s

cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
    Finished (no errors)

cargo check --target wasm32-wasip2 -p semio-s-plugin-puzzle
    Checking semio-s-plugin-puzzle v0.1.0 (…/🧩️puzzle/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 31.33s
    (puzzle 2d/5d warnings present — proof the crates really compiled)

cargo test -p semio-framework-ui --features wgpu --lib -- --test-threads=1 context_menu
    test result: ok. 12 passed; 0 failed; 124 filtered out

cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1 <filter>
    selection_scoped_commands_with_no_selection_refuse_with_exactly_one_notice ... ok
    command_scope_classes_name_the_panels_they_change ... ok
    selection_scope_names_the_inspector_and_not_the_catalogue ... ok
    viewport_and_tick_scope_classes_name_no_panel_body ... ok
    set_fill_count_declares_a_narrow_document_ui_scope ... ok
    set_object_kind_weight_declares_fill_options_ui_scope ... ok
    duplicate_selection_reselects_the_created_clones ... ok
    select_same_kind_widens_the_selection_to_every_object_of_that_kind ... ok
    select_same_kind_with_no_selection_leaves_the_selection_untouched ... ok
    fill_build_tick_is_ignored_when_fill_tool_is_inactive ... ok
    one_brush_mesh_page_validates_inside_one_command_work_budget ... ok
    a_worker_hop_resume_still_holds_the_registered_brush_mesh ... ok

cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1
    test result: FAILED. 613 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; in 107.01s
    (baseline handed over: 591 passed / 22 failed — the total grew because peers and this wave added
     tests; the failure set SHRANK from 22 to 9 and contains no new family)

SEMIO_TEST_LEVEL=standard bun x vitest run       (renderer react target, all 15 suites)
     Test Files  1 failed | 14 passed (15)
          Tests  574 passed (574)
    — the one failure is the same suite that cannot LOAD as in W-L: `🧩️package-integration` →
      `ReferenceError: self is not defined` from `🎯️targets/🧊️wgpu/…/🐚️plugin-bridge.ts:159`.
      Pre-existing, no file of this wave.

bun x tsc --noEmit -p tsconfig.json              (renderer react project)
    821 errors, 2 of them under 🧰️framework — both `Property 'dir' does not exist on type 'ImportMeta'`
    in `🗣️Interpreter/🟦️.tsx:1394`/`:1399`, a repo-wide pre-existing pattern in peer files and far from
    this wave's edit (`:1226`/`:1245`). ZERO in the new engine-contract laws, zero from the TreeView change.
```

The 9 Rust failures are the documented pre-existing families, verified individually where this wave
touched anything nearby:

- `fill_build_tick_is_a_view_action_with_narrow_ui_scope` — fails on `expected a Partial ui_scope for
  fillBuildTick, got None`, i.e. the fill tick's own idle branch, which this wave did not touch (the
  arm keeps its dynamic `None`). `📓️2026-09-09-remaining-test-failures-audit.md` §3.2.
- `fill_build_tick_only_plans_available_slider_range`,
  `fill_build_tick_only_polls_and_enqueues_one_isolated_worker_job`,
  `fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances`,
  `fill_render_reveals_the_full_available_plan_tagged_with_reveal_index`,
  `set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up` (`the maximum-delta timing
  proof requires a planned prefix`) — same audit §3.2.
- `gumball_translate_drag_coalesces_into_one_edit` — fails on its UNDO assertion
  (`left: [3.0, 0.0, 0.0], right: [0.0, 0.0, 0.0]`), exactly as W-L recorded; the test supplies explicit
  `ids`, so it never reaches the new refusal. Same audit §3.3.
- `two_instances_converge_disjoint_object_edits_via_backbone` — the explicit fail-closed VCS stub.
- 1 × `precompute::…::fill_worker_admitted_fixed_pages_survive_replan_and_mesh_supersession_until_retained_close`
  — the peer-owned precompute suite (13 of the 14 that failed at hand-over now pass).

`hover_suggestion_updates_the_brush_candidate_index_and_live_preview` also passes now (peer fix).

## 5 Not verified / left open

1. **Defect 6's remaining half is a browser probe, and it is now a two-way one.** This wave proved the
   guest cannot emit an empty body, that a delivered body renders, that an unpublished body renders as
   a loading skeleton, and that a dropped body makes `refreshUi` throw rather than answer ok. After a
   rebuild, a pick on the slab must show ONE of: the inspector's object field group (fixed), the
   three-row document summary (the selection had not landed when the panel re-rendered — a different,
   ordering defect), the loading skeleton (the body genuinely never arrived — then W-L's
   `refreshUi dropped requested body …` console record names it), or the new `No data` row (a settled
   tree body with no sections — which would mean something between the guest and `TreeView` drops the
   section, and is the only branch nothing in this tree can currently produce). The panel being folded
   is still an untested possibility: the 21:05 note itself flagged it.
2. **Nothing here is confirmed against the React target at runtime.** No browser run, no server.
   The gumball refusal reaches the shell through `subscribeOperationCompletions` →
   `applyHostEffects` → the `"notify" in effect` branch (`🏛️ShellHost/🟦️.tsx:4528`), which was read but
   not executed in a browser.
3. **`semio-framework-os-renderer-wgpu` does not compile in this tree** — three peer breakages
   (`ArtifactEvent::DocumentBackbone` non-exhaustive at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3602`,
   `NodeGraphScene::operators`/`catalogue_json` in `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`). The
   `shell_context_menu_item_from_spec` edit in that same file type-checked (the compiler reached E0004
   later in the file and reported nothing at `:304-318`), but the crate was never linked, so the wgpu
   shell change is compile-checked, not run.
4. **`relocateTargetVolume` and `worldRelocate` are declared `ActionKind::View` while emitting artifact
   mutations.** The scope law had to be written around this (it constrains declared Mutations only). If
   that declaration is wrong it is an undo/history defect, not a paint one, and is untouched here.
5. **`setFillCount`'s scope widened.** It now names four panel bodies and `measures` where it named
   none — correct (it edits the document) but it costs four panel renders per committed fill, on a
   coalescing slider. Not measured. The fill lane is W-J/W-K's; its own suite was already failing before
   and after.
6. **`Puzzle3dScaleWork`'s refusal is decided from `objects`/`volumes`, not from the raw selection.**
   A gesture whose selected ids all resolve to LOCKED volumes still passes the guard and completes with
   an empty mutation list — the locked filter runs later, in the `Volumes` arm. That is a narrower,
   separate refusal and was not added.
7. **The `🔌️plugin-runtime` vitest suite is still unreachable** (W-L §5.6): no vitest project references
   `🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`, so `refreshUi`'s behaviour has no suite. Nothing in
   `🔌️PluginRuntime/🟦️.tsx` was changed this wave.
8. **Two peer repairs were made inside this crate's testkit** (`Label` newtype, §2.9). If the peer
   reverts that contract change, those two lines break again.
