# Flow Native 24: Remaining Failure Audit

## Scope and evidence

This is a read-only audit of `🗑️generated/astra-runtime/flow-native24-full/run.log`, which records 246 passing and 7 failing tests. One failure is the already-confirmed one-byte `ArtifactStoreBatchPublication::close_step` authority-retirement issue. It is excluded from the six findings below because the owner has the repair.

The remaining receipt separates into two live Flow composition defects and three fixture expectations/setups, plus the archive manifestation of the same composition defect:

| Receipt | Classification | Evidence-backed cause |
| --- | --- | --- |
| `node_graph_edit::batched_delete_selection_clears_the_node_selection_on_the_scene` | Production | A graph edit is emitted as a parent `FlowMutation`; the retained parent recipe searches its cache-backed target and cannot find it. |
| `node_graph_edit::spotlight_commit_shares_the_node_graph_edit_vocabulary` | Production | The same parent route emits `ConnectWidgets`, which has no retained parent recipe. |
| `flow_document_text_round_trips_store_with_applied_operation` | Production manifestation | `MoveMediaNode` repoints the parent at a cache-only child handle, leaving the stated child absent from the store before archive/load. |
| `host_from_snapshot_deletes_edge_selected_by_synapse_domain` | Fixture setup | The test constructs a host without registering the extension/test bridge that resolves the starter graph's `math.add` kind. |
| `two_instances_converge_on_disjoint_edits` | Fixture setup | Both replicas use the helper's fixed `instance_id: 1`, so peer delivery is correctly treated as self/duplicate traffic. |
| `flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows` | Fixture expectation | The test counts pre-coalescing writes, whereas the retained result correctly publishes one final config snapshot per affected window. |

## Live child-publication boundary

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:112` routes delete/connect/disconnect/move through `node_graph_edit_result`. Its host callback is `fallible_host_operations` in `✏️editor/🦀️.rs:2690`, which diffs a reconstructed host and returns parent `FlowMutation` values.

`FlowGraphOperationWork` includes `nodeGraphEdit` and `spotlightCommit` in its parent-operation vocabulary at `✏️editor/🦀️.rs:1806`, and executes that route at `:1860`. `✏️editor/🎮️commands/✅️spotlight-commit/🦀️.rs:18` delegates entirely to the same result function.

That is incompatible with the actual content ownership. `FlowSnapshot::from_host_snapshot` at `🧬️schema/📸️snapshot/🦀️.rs:39` mints a child handle/cache; `to_host_snapshot` at `:46` reads that cache. The root Flow artifact's `flow_content_child_handle_and_cache` at `🌊️flow/🦀️.rs:304` makes a content-addressed child and attaches only a local `FlowWorkingScene` cache. It does not install that child as a stored composed member.

The retained parent recipe only knows `DeleteWidget`, `DisconnectWidgets`, `MoveWidgets`, and `ReplaceWidget` (`✏️editor/🧵️retained/🗿️artifact/🧬️recipe/🦀️.rs:78`). Its phase-4 target lookup returns `Flow recipe target is missing`; it has no `ConnectWidgets` case at all. Those are the exact receipt faults for delete and spotlight/connect.

The archive failure proves the same defect along a second live entry point. `✏️editor/🎮️commands/🚚️move-media-node/🦀️.rs:18` uses the parent host-operation route, and `FlowDirectStoreWork` (`✏️editor/🦀️.rs:1061`) emits `FlowMutation::MoveWidgets`. The binary law then reads `parent.content.child_id` and asks `app.child_store("content", &id)` in `🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs:47`; it correctly fails because that new cache-only id was never stored. This is a production document-integrity defect, not an archive fixture mismatch.

### Narrow repair boundary

Do **not** enlarge the parent `Recipe` to accept `ConnectWidgets`; that would preserve a parent mutation whose content child is not durably published.

Move graph-content verbs to the existing child mutation lane, against the existing `snapshot.content.child_id`:

1. Read `SemioFlowSnapshot` through `ArtifactOwnedToolJobContext.children.typed_read`.
2. Produce a typed `SemioFlowMutation` for delete, connect, disconnect, and move.
3. Emit `ChildEmit` and retain it through `FlowChildGroupWork`, preserving the parent's content handle.

`✏️editor/🎮️commands/➕️add-widget/🦀️.rs:57` is the production precedent: it reads the typed child and returns a child emission. `FlowChildGroupWork` at `✏️editor/🦀️.rs:1423` has the matching retained-child protocol, though its current tool list contains only `addWidget`.

The first regression laws should execute actual command dispatch, not recipe string matching:

1. `NodeGraphEdit` delete-selection and its `SpotlightCommit` alias publish a `TypedOperationResultLane::Child`; the parent content id remains unchanged; the child node is absent; close reaches terminal empty.
2. `NodeGraphEdit` connect produces the exact typed child mutation and leaves the parent pointer and child-store membership unchanged.
3. After real `MoveMediaNode`, assert `child_store(parent.content.child_id)` before archive/load; then retain the existing text round-trip equality assertion.
4. Cover retry/cancel/close on the retained child group, following the existing Add Widget route.

## Fixture synchronizations

### Disjoint-replica convergence

`✏️editor/🧪️tests/🔬️unit/🦀️.rs:631` builds two apps using `flow_app_with_registry`. That helper binds both to `artifact_app_laws::meta("local").instance_id`. The shared helper in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:7187` always returns `instance_id: 1`. The test then supplies that same identity to both replicas.

`MemoryBackbone::pair` at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19266` supplies correctly crossed queues. The observed divergent edits therefore match self/duplicate filtering under colliding replica identity; there is no receipt evidence of a Flow or backbone convergence defect.

Repair the fixture with explicit distinct instance ids (for example A=1 and B=2) in both app binding and action metadata, while keeping distinct actors and crossed transport. Assert the two disjoint child edits converge, then checkpoint convergence.

### Edge selected through host snapshot

The direct test at `✏️editor/🧪️tests/🔬️unit/🦀️.rs:610` builds `FlowSnapshot::default`, a session, and `host_from_snapshot`, then tries to select starter edge `s1`. The starter graph includes a `math.add` node (`🧬️schema/📸️snapshot/🦀️.rs:269`), but this test does not establish the extension registry/test bridge that resolves that kind.

The application-side test helper registers the needed local math extension at `✏️editor/🧪️tests/🔬️unit/🦀️.rs:62`. The framework's real edge-selection proof uses `host_with_test_bridge()` at `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs:2126`. Selection-domain parsing itself still accepts `nodes`, `edges`, and `handles` in `✏️editor/🦀️.rs:2737`; the framework's directed-DAG implementation confirms the same wire fields at `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3442`.

Synchronize this test by installing and retiring its local extension fixture before host construction, or make it a registration-free framework-host test using the existing bridge. It must not rely on order-dependent global test initialization.

### Window configuration publications

The window test sends four persistent config actions (two each for `left` and `right`) and one transient action, then expects `(4, 1)` at `✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:74`. The receipt contains two `WindowConfig` lanes and one `WindowTransient` lane.

`FlowMainWindowConfigOwner` emits one full `Snapshot` for each addressed window (`🎚️config/🦀️.rs:108`). The framework result owner deliberately coalesces a window-config amend per playback frame (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29448`). Two final persistent publications are therefore correct. The later assertions already validate both final configurations, persistence/reload, and the transient state.

Change only this expected lane count to `(2, 1)` while retaining those semantic assertions. Reinstating four pages would reject the current retained coalescing contract.

## Confidence and limits

High confidence: child-store loss and unsupported parent recipe are source-backed, reachable production defects. High confidence: replica-id collision and config count are fixture errors. Medium-high confidence: the direct edge test is missing its required kind registry; source confirms both the missing setup and the valid selection-domain protocol, but no new run was performed in this audit.

No source was changed and no command was run.
