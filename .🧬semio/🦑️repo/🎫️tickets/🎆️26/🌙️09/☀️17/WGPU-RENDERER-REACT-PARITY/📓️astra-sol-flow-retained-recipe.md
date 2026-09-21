# Flow Retained Recipe and Close Repair

## Fail-first evidence

Flow Native7 ran six selected laws: two passed and four failed. `node_graph_move_wire_publishes_the_requested_widget_layout` aborted with `ordered-map root must be explicitly retired before drop`. A focused replay with `RUST_MIN_STACK=134217728` and `RUST_BACKTRACE=1` located the drop in `retained::artifact::recipe::Recipe::advance`, reached from `Preparation::advance`, `ArtifactStore::advance_apply_batch`, and the real typed `VcsArtifactApp` publication. The other three failures timed out in the fixture close ladder with `document store close awaits a retained reader or owner`:

- `node_graph_edit_rejects_an_unknown_operation_instead_of_dropping_it`
- `main_scene_declares_its_scoped_graph_interaction_targets`
- `framework_graph_selection_projects_back_to_raw_scene_node_ids`

The complete six-law receipt is `🗑️generated/astra-runtime/flow-native7/run.log`; the focused stack is `🗑️generated/astra-runtime/flow-native7-move-backtrace.log`.

## Recipe cause and repair

The first repair targeted phase 23's placeholder handoff. Flow Native8 disproved that assignment: all three close laws passed, while the move law still aborted at `Recipe::advance` with the same `OrderedMap<WidgetLayout>` drop. The fresh focused backtrace is `🗑️generated/astra-runtime/flow-native8-move-backtrace.log`.

The exact phase-22 cause is `OrderedMap::begin_set`/`begin_remove` taking `&self`: the update cursor owns a cloned persistent root. Phase 22 first moved `scene.layout` into local `map`, constructed the cursor from a clone, and then let the original nonempty `map` fall out of scope. Its fail-closed `Drop` produced the recorded abort before phase 23 could run.

Phase 22 now transfers that original map into the editor's typed `Layouts(OrderedMap<WidgetLayout>)` retirement owner after constructing the cursor. Phase 23 installs the cursor result into the empty scene placeholder. The original and result roots retain their exact persistent sharing until the recipe's ordinary explicit close drains the original through the framework Flow retirement domain.

## Close-ladder cause and repair

`FlowRetirement` is explicitly reserve-then-close: `close_step` returns `Blocked` while `next_allocation_bytes` names a required continuation page, and retained callers must use `close_page`. The plugin's document snapshot, working-scene root, editor retirement adapter, and presence retirement adapter all called the raw trait method. A populated scene therefore reached the document-store disposer but never reserved the page required to decompose its widgets/specs/layout owners. This directly explains the stable 30-second close refusal without weakening the store's retained-reader invariant.

All four retained adapters now call `FlowRetirement::close_page`, preserving the one-item and byte grant while allowing the domain frontier to reserve exactly one required continuation page. Flow Native8 confirms the three formerly blocked fixture close laws pass. Its result was 5/6 passed with only the independently diagnosed move-map drop remaining.

## Validation boundary

All touched Flow Rust sources parse and match `rustfmt --check --config skip_children=true`. No Cargo, native test, activation, or generated-artifact command was run by this packet. Root owns the serial native lane. The phase-22 map transfer awaits that lane's focused rerun.

The focused rerun expression is:

`test(renderer_operation_rows_decode_through_one_closed_vocabulary) | test(operation_parser_refuses_beyond_the_retained_route_row_and_wire_authorities) | test(node_graph_move_wire_publishes_the_requested_widget_layout) | test(node_graph_edit_rejects_an_unknown_operation_instead_of_dropping_it) | test(main_scene_declares_its_scoped_graph_interaction_targets) | test(framework_graph_selection_projects_back_to_raw_scene_node_ids)`

## Direct phase-22 cancellation law

`move_recipe_cancellation_after_layout_cursor_creation_retires_both_map_roots` drives the neutral `move-widget` recipe under every declared one-item byte grant until phase 22 has constructed its live `UpdateCursor`. Before cancellation it proves that the scene destination is empty, the cursor owns the candidate root, the retirement queue owns the original populated layout root, and the source `Arc` is still live. The existing close ladder must then release bytes, reach exact terminal emptiness, and expire the source weak reference. This covers cancellation at the ownership boundary that Native8 exposed, independently of the end-to-end typed publication law.

Exact new filter: `test(move_recipe_cancellation_after_layout_cursor_creation_retires_both_map_roots)`.

## Flow Native9 receipt

Root's focused Flow Native9 run passed all seven selected laws, including the end-to-end move publication and the new phase-23 cancellation boundary; 246 tests were outside the filter. Nextest run ID: `93f491ce-d5eb-483a-8317-6ede926bf5c5`, 81 ms test time, Nx 2m32s. Receipt: `🗑️generated/astra-runtime/flow-native9/run.log`.
