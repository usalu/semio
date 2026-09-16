# 🔍 Slice D — inspector panel + the nine `patch*` commands (2026-09-16)

Agent: slice D (Opus). Crate `semio-s-artifact-fem-2d`, everything behind `--features component-app-assembly`.

## 1. Design

**One flat payload for every field of every entity.** A control binds `Trigger::Change` to a `patch*`
action carrying only `{field, id}`; the host merges the control's own scalar under `value` and the
editor root's `scalar_text` bridge stringifies numbers and bools. So `{id, field, value: String}` is
the whole wire shape, no per-field action is declared, and a new field is one match arm plus one row.

**Every patch emits exactly one existing mutation, whole-record.** `replace-{node,element,material,
section,support,region,load,combination}` carry the complete record with one field changed; the
mutation guards (identity match, plausibility bounds, reference resolution) stay the single place
admissibility is decided, so a handler never duplicates a rule it could drift from. The one exception
is `patchLoadCase`: a case owns its `loads` collection, and re-sending that collection to rename the
case would make every concurrent `add-load` on the same case a lost update — so `name` emits
`ChangeLoadCaseName` and `selfWeight` emits `ChangeLoadCaseSelfWeight`, the two narrow changes.

**Three refusal shapes, one code family.** `fem2d.patch.<noun>-missing` (no such entity),
`fem2d.patch.<noun>-field` (the addressed variant has no such field — `wx` on a nodal load is an
unknown field, not a silent no-op), `fem2d.patch.<noun>-value` (unparsable text, unknown enum
spelling). A value equal to the current one returns `Emit::default()` rather than opening a revision.

**Two spellings that are not in the plan's field table.**
- `patchSupport` has no scalar fixity field: `fixed` is a DOF set, so the inspector shows one toggle
  per planar DOF and each toggle is a membership edit. The rebuilt set is always emitted in
  `FemDof::ALL` order, so two hosts toggling different DOFs converge on one spelling of the set.
- `patchCombination` gained `addTerm` next to `term:<caseId>`. An "add a term" select carries the
  chosen case id as the control's own `value`, and `field` has to stay constant across its options —
  `term:<caseId>` cannot express that. `addTerm` appends at factor `1.0` and no-ops when the case is
  already superposed. A `term:<caseId>` factor of exactly `0` REMOVES the term rather than leaving a
  zero-weighted superposition that still pins the referenced case against deletion.

**Analysis settings are read-only in the inspector.** `setAnalysisSettings` takes three typed
`Option` numbers and the host merges only `value`, so a single control cannot address one of them
from a panel with no window context. Slice E's results panel owns the `windowId`-tagged bindings; the
inspector's summary reports `modalCount` / `bucklingCount` / `deformationScale` and edits nothing.

**Selection plumbing is slice A's.** The panel resolves the first selected id through
`interaction::fem2d_entity_kind` (granularity string) and a load's case through
`interaction::fem2d_load_owner` — the private resolvers this slice started with were deleted the
moment those landed. Unresolvable ids fall through to the document summary. A load row inside a load
case dispatches `interactionSelect` at `load` granularity, the same action a viewport pick or an
artifact-tree row sends, so there is exactly one selection channel.

**Bounds.** Reference selects cap at 24 options (`UiFixedList` admits 32); nested listings (a case's
loads, a combination's terms) page through `paged_panel_section` at 8 rows against
`PanelRowBudget::new(panel_page_rows())`; the widest section is 7 rows ≈ 16 nodes, well inside
`UI_DOCUMENT_NODES = 128`. Every `UiMapBuilder` key is ascending (`field` < `id`; `domainId` <
`merge` < `method` < `targets`).

## 2. Files

Owned and written by this slice:

| File | What |
|---|---|
| `✏️editor/📌️panels/🔍️inspection/🦀️.rs` | the panel: definition, admission/control helpers, nine entity sections, summary, render |
| `✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | 14 panel tests |
| `✏️editor/🎮️commands/🩹️patch-node/🦀️.rs` (+ `🧪️tests/🔬️unit/🦀️.rs`) | `ReplaceNode` |
| `✏️editor/🎮️commands/🩹️patch-element/🦀️.rs` (+ tests) | `ReplaceElement`, incl. `Bar`↔`Beam` |
| `✏️editor/🎮️commands/🩹️patch-material/🦀️.rs` (+ tests) | `ReplaceMaterial` |
| `✏️editor/🎮️commands/🩹️patch-section/🦀️.rs` (+ tests) | `ReplaceSection` |
| `✏️editor/🎮️commands/🩹️patch-support/🦀️.rs` (+ tests) | `ReplaceSupport`, DOF-set membership |
| `✏️editor/🎮️commands/🩹️patch-region/🦀️.rs` (+ tests) | `ReplaceRegion` |
| `✏️editor/🎮️commands/🩹️patch-load/🦀️.rs` (+ tests) | `ReplaceLoad`, case resolved by lookup |
| `✏️editor/🎮️commands/🩹️patch-load-case/🦀️.rs` (+ tests) | `ChangeLoadCaseName` / `ChangeLoadCaseSelfWeight` |
| `✏️editor/🎮️commands/🩹️patch-combination/🦀️.rs` (+ tests) | `ReplaceCombination` |

Touched outside the slice's own leaves — additive only, two lines:
`✏️editor/🗣️terminology/🦀️.rs` gained `selected` ("Selected"/"Ausgewählt") and `add_term`
("Add Term"/"Term hinzufügen"), all four locale×terminology cells, per the "add the label you need"
instruction. `✏️editor/🦀️.rs` and the crate root were NOT edited.

## 3. Field matrix

| Command | Field | Control | Mutation |
|---|---|---|---|
| `patchNode` | `x`, `y` | number input, step 0.1 | `ReplaceNode` |
| `patchElement` | `kind` (`bar`/`beam`) | select | `ReplaceElement` (variant swap, ids kept) |
| | `start`, `end` | select over node ids | `ReplaceElement` |
| | `materialId`, `sectionId` | select over materials/sections, labelled by name | `ReplaceElement` |
| `patchMaterial` | `name` | text input | `ReplaceMaterial` |
| | `e` | number input, step 1e9 | `ReplaceMaterial` |
| | `nu` | slider 0…0.49 step 0.01 | `ReplaceMaterial` |
| | `rho` | number input, step 10 | `ReplaceMaterial` |
| `patchSection` | `name` | text input | `ReplaceSection` |
| | `area` | number input, step 1e-4 | `ReplaceSection` |
| | `iy` | number input, step 1e-6 | `ReplaceSection` |
| `patchSupport` | `nodeId` | select over node ids | `ReplaceSupport` |
| | `tx`, `ty`, `rz` | toggle (membership in `fixed`) | `ReplaceSupport` |
| `patchRegion` | `name` | text input | `ReplaceRegion` |
| | `thickness` | slider 0.01…2.0 step 0.01 | `ReplaceRegion` |
| | `meshSize` | number input, step 0.1 | `ReplaceRegion` |
| | `materialId` | select over materials | `ReplaceRegion` |
| | `outline`, `holes` | read-only counts | — (polygons are edited in the viewport) |
| `patchLoad` (Nodal) | `nodeId` | select over node ids | `ReplaceLoad` |
| | `dof` | select `Tx`/`Ty`/`Rz` (case-insensitive parse) | `ReplaceLoad` |
| | `value` | number input, step 100 | `ReplaceLoad` |
| `patchLoad` (MemberUdl) | `elementId` | select over element ids | `ReplaceLoad` |
| | `wx`, `wy` | number input, step 100 | `ReplaceLoad` |
| `patchLoad` (Area) | `regionId` | select over regions | `ReplaceLoad` |
| | `pressure` | number input, step 100 | `ReplaceLoad` |
| `patchLoadCase` | `name` | text input | `ChangeLoadCaseName` |
| | `selfWeight` | toggle | `ChangeLoadCaseSelfWeight` |
| | (its loads) | `interactionSelect` pick rows, paged | — |
| `patchCombination` | `name` | text input | `ReplaceCombination` |
| | `term:<caseId>` | number input per term, step 0.05 (0 removes) | `ReplaceCombination` |
| | `addTerm` | select over cases not yet superposed | `ReplaceCombination` (factor 1.0) |
| — (summary) | analysis settings | read-only rows | edited from the results panel |

## 4. Tests

| Module | Tests |
|---|---|
| `patch_node` | 4 |
| `patch_element` | 4 |
| `patch_material` | 4 |
| `patch_section` | 4 |
| `patch_support` | 4 |
| `patch_region` | 4 |
| `patch_load` | 4 |
| `patch_load_case` | 4 |
| `patch_combination` | 5 |
| `panels::inspection` | 14 |
| **total** | **51** |

Every command has four shapes: the value applied end to end (`dispatch(setActiveExample{demo})`, then
the patch, asserted on `app.snapshot()`), the emitted mutation asserted directly (one mutation, right
kind, the edited field changed and every other field carried through), a refusal triple (unknown
field, unparsable value, missing entity) and an unchanged-value no-op. Refusals and emitted-mutation
shapes go through `handle` with `ArtifactView::new` — the shared `context::dispatch` helper unwraps,
so a `Fault` would panic there instead of being observed, and the direct route also keeps those
assertions independent of the app dispatch path.

Panel tests project the built node with `artifact_app_laws::project_and_retire_fixture_tree` and
assert on row ids, bound action names and resolved label text: one test per entity kind, the
multi-selection header, the summary, the unresolvable-id fallback, four German spellings, and the
body-key route on the live app.

## 5. Verification

Logs under `🗑️generated/w-d/`.

- ✅ `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --tests --message-format short` — exit 0, zero errors, zero warnings from this slice's files (`check-5.txt`).
- ✅ `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --target wasm32-wasip2` — exit 0 (`check-wasm.txt`).
- ⚠️ `RUST_MIN_STACK=134217728 cargo nextest run … -- patch_ inspection …` — **50 of 60 pass; the 10 failures are all dispatch-based and are a crate-wide regression, not this slice** (`nextest-7.txt`).

### The crate-wide dispatch regression (blocks the last 10)

Typed-command dispatch currently applies nothing. Reproduced on the untouched, long-standing
`editor::fem2d::commands::add_node::tests::add_node_action_emits_op_2d`, which asserts
`result.mutations.len() == 1` on a bare `AddNode` and gets zero (`nextest-8.txt`). Every other
pre-existing dispatch test fails the same way — `add_material_action_emits_op_2d`,
`add_section_action_emits_op_2d`, `add_support_action_emits_op_with_fixed_dofs_2d`,
`add_bar_and_add_beam_actions_emit_ops_2d`, `add_region_action_emits_set_region_2d`, all three
`remove_selection` tests, `set_analysis_settings_partial_args_keep_current_2d`,
`undo_restores_document_after_add_node`, `the_editor_boots_on_the_bundled_example_document`
(`nextest-5.txt`). Two further symptoms seen while this slice was verifying, both since gone:
`interactive-job.publication-contract` at `new_app_with_registry` while
`FEM2D_PUBLICATION_CONTRACTS` lagged behind the fifteen new command rows (it now covers all 33 —
checked), and `artifact store reached Drop without its exact terminal-empty shallow-shell witness`.
The boot document also changed to the bundled demo mid-session, which is what breaks the `add_*`
tests' `materials[0]`/`regions[0]` index assumptions.

Nothing in slice D's files can fix this; the ten tests are written against the correct end state and
go green with it. The 50 that pass already cover every patch semantic and every inspector section.

## 6. What this slice waited on, and what it now depends on

- **Slice C (mutations)** — landed. `ReplaceNode`, `ReplaceLoad` (`new_load: Box<FemLoad>`),
  `ChangeLoadCaseName` and `ReplaceCombination` are coded against their real names and shapes;
  nothing here is stubbed.
- **Slice A (interaction)** — landed. The panel resolves the selected id with `fem2d_entity_kind` and
  matches on the granularity constants; `patch-load` resolves its owning case with
  `fem2d_load_owner`. The private resolvers this slice started with are deleted.
- **Slice B (terminology)** — landed. Every label the inspector binds resolves; this slice added the
  two the roster was missing.
- **Coordinator** — owed: the dispatch regression above. `FEM2D_PUBLICATION_CONTRACTS` is already
  complete. Nothing else is outstanding from slice D's side.

## 7. Form layout (2026-09-16 14:30)

**The defect.** Browser-verified on the React lane (port 6086, 14:20): the inspector showed only
`ID n1` and the two `Actions` rows. Every form control was gone and no field was editable.

**The cause is the container, not the controls.** The body was a `PanelTreeBuilder` — a
`Component::Tree` whose `tree_section`s held `field(label) > control` rows. The React `Interpreter`
does not interpret a tree node by node: `uiTreeNodeToTreePanelConfig` maps its sections onto
`TreeDataSection`/`TreeDataItem` records and reads only `label`, `description`, `items` and `action`
off each row. A child that is not a tree item has nowhere to go in that mapping and is dropped
before it ever reaches a DOM element. The rows that survived were exactly the ones built from
`tree_item_desc`/`tree_item_with_action`. Nothing was wrong with the bindings, the argument maps or
the nine `patch*` commands — they were assembled correctly and never drawn.

**The fix.** The body is now a `ui::column` of `ui::section`s, the layout slice E's results panel and
puzzle3d's Settings panel have always used. `Component::Container(role: Section | Field)` is
interpreted structurally: a section renders `<Section>` with its children, a field renders `<Field>`
with its label and its children, and the control inside the field renders itself. Section 1's
"one flat payload" design is unchanged — every control still binds `Trigger::Change` to
`patch*` with `{field, id}` and the host still merges `value`.

| Was | Is |
|---|---|
| `PanelTreeBuilder::new(ROOT)` + `.section(id, label, open, rows)` | `ui::column().try_id(ROOT)` + `ui::section(label).try_id(id).default_open(true)` |
| `tree_item_desc` read-only row | `ui::field(label) > ui::text(value)`, keyed `{row}.value` |
| `tree_item_with_action` load pick row | `ui::button` bound `Trigger::Activate` → `interactionSelect`, keyed by the load's own id |
| `tree_item_with_action` Focus / Delete | `ui::button` bound `Trigger::Activate` → `focusEntity` / `removeSelection` |
| `paged_panel_section` | local `paged_rows` — same `LIST_ROWS_MAX` / `PanelRowBudget` accounting, but its continuation row is `ui::text("+n")`, because the framework helper's is a tree item |
| `tree.interaction_domain(FEM2D_INTERACTION_DOMAIN)` | dropped — `interaction_domain` exists only on `TreeBuilder`, and the panel never used framework-owned tree selection: every pick row dispatches `interactionSelect` with the domain in its own argument map |

**What did NOT change.** Every id (`fem2d-play-inspection.*`), every label (`Fem2dLabels`, no new
entry needed — all four locale×terminology cells untouched), every bound action and argument-map
shape, the `SELECT_ITEMS_MAX = 24` / `LIST_ROWS_MAX = 8` bounds, the multi-selection header, the
document summary, the add-term select, and the signature
`render(doc, &Fem2dInteractionSnapshot, &Fem2dLabels) -> UiAssemblyResult<BuiltNode>` the
coordinator routes. The widest body (a region, 7 rows) is now 1 column + 2 sections + 7 fields +
7 controls + 2 buttons = 19 nodes against `UI_DOCUMENT_NODES = 128`; the largest (a combination with
8 terms) is 28. Read-only values pass through `ui_label`, so they stay inside the 512-byte text
budget.

**Tests.** The 14 panel laws keep their guarantees — the projection keys every node by its builder
id, so the row-id, action-name and label assertions read unchanged — and were strengthened from
substring checks to `component.type` checks through a new `node_at`/`component_at` walker: a select
row is a `select`, a slider a `slider`, a toggle a `toggle`, a read-only value a `text`, a pick row a
`button`. Two laws were added, for 16:

- `a_node_section_carries_a_number_input_bound_to_patch_node_2d` — the node group's X row is an
  `input` component of kind `number`, bound to `patchNode` naming field `x`, nested in a `container`
  field row. This is the law the tree layout violated.
- `the_actions_group_binds_focus_and_delete_as_buttons_2d` — Focus and Delete are `button`s, and a
  material (no viewport geometry) offers Delete only.

Three tests additionally assert `!carries_a_tree(json)`: no `tree`, `treeSection` or `treeItem`
component survives anywhere in this panel's body.

**Verification (2026-09-16 14:30).**

- ✅ `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --tests --message-format short` — exit 0, zero errors, zero warnings from this panel's two files.
- ✅ `cargo nextest run … --profile fundamental -- inspection --skip quick:: --skip long:: --skip exhaustive::` — **16 tests run: 16 passed**, 1228 skipped.
- ✅ `cargo check -p semio-s-artifact-fem-2d --features component-app-assembly --target wasm32-wasip2` — exit 0.
