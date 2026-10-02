# 📓️ W3-T2-GRAPHS Report: Node-Graph Guests (dag, sequence, mathematical, hub space)

Executor S2-GRAPHS (coordinator `⚪552b484a…`). Brief: `🧭️plan.md` "W3-T brief" and "Session 2 roster"; checklist
`📓️resume-tools.md` §4.10; contract `📋️design.md` §5, §7, §10, §11, §12, §13.3, §17. Scratch: `🗑️generated/s2-graphs/`.

Aliases: `DG` = `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`WF` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow`,
`SP` = `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space`,
`MA` = `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets`,
`SQ` = `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## Session 2 — 2026-10-01

### 1. Census (before this work)

| Guest | Gesture | Path today | Defect |
|---|---|---|---|
| dag | node drag, React | `WasmGraphSurface` pointer-up → `nodeGraphEdit {setHostSnapshot}` → `dag_snapshot_mutations` | one ABSOLUTE `move-node` per moved node; and `dag_command_from_action` did not decode `nodeGraphEdit` at all (every shell dispatch faulted `dag.unhandled-action`) |
| dag | node drag, wgpu | `{operation:"move", gestureId, nodeIds, dx, dy}` | the guest's typed `DagNodeGraphEditOp` had no `move` row; unreachable as above |
| dag | `moveMediaNode {nodeId, x, y}` (palette/MCP) | `Emit::amend(move_node, "move-{id}")` | static key, per-tick amend, absolute |
| dag | canvas slider overlay | `nodeGraphEdit {setSlider, gesture, commit}` | no `setSlider` row in the guest |
| dag | inspector slider value/min/max | `patchDagNodes` → `Emit::amend(replace_node_kind + resize_node, "patch-{field}-{ids}")` | whole-kind replace + resize, static key; `patchDagNodes`/`renameDagNode`/`connectMediaPorts` were not decoded by the shell bridge either |
| dag | inspector name/id text fields | `commit("blur")` bound on `Trigger::Change` | React `InputView` fires the `commit` trigger for a blur-commit field (`Interpreter/🟦️.tsx:1424`), so the binding was dead |
| space (hub) | node drag, React | `setHostSnapshot` → `apply_flow_host_snapshot_to_os_workflow` | one absolute `MoveNode` per node |
| space | node drag, wgpu | `move` row read as the OLD absolute `{nodeId, x, y}` | silently dropped (fields absent) |
| space | `moveMediaNode` | `Emit::amend(MoveNode, "moveMediaNode:{id}")` | static key, per-tick amend |
| space | `nodeGraphEdit` bridge | shell bridge passes the operations ARRAY, handler parsed an OBJECT `{operations}` | every shell `nodeGraphEdit` was a silent no-op |
| space | parameters panel: name, text value, add-option | `Trigger::Change` per keystroke | `addOption` appended one option per keystroke; renames per keystroke |
| space | inspector label | `Trigger::Change` per keystroke | per-keystroke rename |
| space | inspector x/y, parameter numbers | number inputs, `Trigger::Change` | already continuous → framework scrub machine (absolute leaves); no change needed |
| mathematical | node move/add/connect/delete | retained job copies the whole graph, applies ops, `ReplaceGraph{whole graph}` | whole-graph rewrite per op |
| mathematical | `setAlgorithm` / `setDirected` | `Emit::commit(ReplaceGraph)` / `Emit::mutations(ReplaceGraph)` | whole-graph rewrite; intent leaves `update-graph-algorithm`/`change-graph-directed` already exist |
| sequence | every editor verb (layout, connection, node-graph, step: 13 sites) | `sequence_child_emit_from_host_mutation` → child `SetSnapshot{whole content}` | whole child snapshot per edit |

### 2. Changes (source)

**dag — leaves (schema-first, `DG/🧬️schema/🧬️mutations/`)**
- `🚚️move-nodes` `MoveNodes {ids, dx, dy}` (RELATIVE): `🦠️mutation/🦀️.rs`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🔣️.json`,
  `🧬️schema/🔣️.json` (full `x-semio-ui` en/de: reference target, steppers), `🦠️mutation/🟦️.ts` (`parseMoveNodes`). Outcomes:
  Fatal `mutation.invariant` (empty/duplicate ids, non-finite offset), Error `mutation.target-missing`, Warning
  `mutation.partial` / `mutation.no-op`, Error `mutation.target-mismatch` (non-finite result). Inverse = ONE absolute
  `set-node-positions` row (dag's retained one-item fold contract needs point-invertible rows, `🧬️schema/🦀️.rs:285`).
  Label "Move N node(s) by (dx, dy)" / "N Knoten um (dx; dy) verschieben".
- `📍️set-node-positions` `SetNodePositions {positions: [{id, x, y}]}` (ABSOLUTE, the exact undo of a drag and of itself),
  `x-semio-invariant: unique-node-ids`, TS `parseSetNodePositions`.
- `🎚️set-slider` `SetSlider {id, field: value|min|max, value}` (ABSOLUTE); clamps a value into range with Warning
  `mutation.clamped`, Error `mutation.target-mismatch` for a non-slider or crossing bounds; inverse ONE `set-slider`; TS
  `parseSetSlider`.
- Wiring: aggregate enum + `KINDS` (17) + `pub use`, helpers `dag_label_number`, `dag_targets_invariant`, `dag_partial`,
  `dag_move_leaves` (groups a snapshot's position changes by offset, tolerance 1e-6, into `move-nodes`);
  `dag_snapshot_mutations` emits `move-nodes` instead of per-node `move-node`; crate root mounts
  (`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs`); `📖️.grammar.semio`, `🔗️.graphql`, `🛰️.proto`, `🔣️.json` (oneOf),
  `🟦️.ts` union; text/binary DSL mirror (`DG/🚪️io/🧬️mutations/📝️text/🦀️.rs`); oracle catalog `kinds` + `deferredKinds` +
  three `mutationManifests` rows (`DG/🔮️oracles/🔣️.json`); wire witnesses
  `DG/🧫️fixtures/🧬️mutations/{🚚️move-nodes,📍️set-node-positions,🎚️set-slider}/🧾️wire-witness/🦠️mutation/🔣️.json` (feed the
  derive-generated `semio_payload_law_dag_mutation`).
- Laws (`DG/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` region `🔖️GestureLeaves`): relative move + one-row inverse +
  inverse law, outcome vocabulary, set-node-positions absolute/self-inverse/no-op, set-slider clamp/mismatch/inverse,
  snapshot moves grouped by offset (rounding noise included), en/de labels; text + binary round trip of every variant
  (`DG/🚪️io/🧬️mutations/📝️text/🧪️tests/🔬️unit/🦀️.rs`).

**dag — editor**
- `✏️node-graph-edit`: typed rows `SetHostSnapshot`, `DeleteSelection`, `Connect`, `Disconnect`, `Move` (the gesture
  record), `SetSlider`; `NodeGraphEdit::from_action_args` decodes the host wire by name (refuses unknown operations and
  malformed records); a `move` row or a snapshot that moved nodes commits through `node_drag_commit` as ONE tool
  transaction (`dag_node_drag_emit`, tool `s.dag.dag@1/*#editor#nodeGraphEdit`); `setSlider` yields the absolute
  `set-slider` leaf and rides the framework scrub machine (top-level `gesture`/`commit`).
- `🚚️move-media-node`: offset from base → ONE `move-nodes` transaction (tool `…#moveMediaNode`); unknown node refused by name.
- `🩹️patch-dag-nodes`: `name` → `change-node-name`; `value|min|max` → `set-slider` per slider; `Emit::mutations`.
  `replace_node_kind` + `resize_node` and both `Emit::amend` sites are gone (0 `Emit::amend` in the dag tree).
- `dag_command_from_action` (`DG/✏️editor/🦀️.rs`) now decodes `nodeGraphEdit`, `moveMediaNode`, `patchDagNodes`,
  `renameDagNode`, `connectMediaPorts` from shell args (the React `NodeGraph/🟦️.tsx` `nodeGraphActions.edit` and the
  inspector's blur commit dispatch exactly this `{action, args}` pair). An absent or incomplete payload is refused by the
  verb's own code (`dag.node-graph-edit.malformed`, `dag.move-media-node.malformed`, `dag.patch-dag-nodes.malformed`,
  `dag.rename-dag-node.malformed`, `dag.connect-media-ports.malformed`) instead of decoding into an empty edit;
  `NodeGraphEdit::from_action_args` refuses a missing `operations` array (an abort sends `operations: []` explicitly).
  The stale test `command_from_action_resolves_every_flat_verb_and_names_the_gesture_only_ones` (it asserted these five
  verbs fault `dag.unhandled-action`, "typed command channel only") is replaced by
  `command_from_action_resolves_every_flat_verb_and_decodes_every_structured_one` (`DG/✏️editor/🧪️tests/🔬️unit/🦀️.rs`).
- Demo asset `DG/🖼️assets/🎬️demo/🗣️.dsl.semio`: each `nodes` item is now a braced record `{ … }` (5 items). A peer's
  staged DSL grammar change (ticket 26/09/30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O, `📓️list-record-boundaries.md`:
  "every direct `List<Record>` item is `{ record fields }`, no bare-record fallback") migrated only ISO16757 assets, so
  `default_snapshot()` panicked in every dag test that touches the demo document.
- Inspector (`DG/✏️editor/📌️panels/🔍️inspection/🦀️.rs`): text fields bind `Trigger::Commit` with `commit("blur")`.
- Cargo: `semio-framework-tool-machine.workspace = true` in the dag artifact manifest.
- Laws (`DG/✏️editor/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs` region `✋️GestureLaws`, replacing the deleted
  `move_media_node_drag_coalesces_into_one_edit`): one drag = one edit = one row with `TransactionRef` and one undo; zero
  offset / ghost node = zero trace; two drags = two transactions; palette drop under its own tool; React snapshot drag =
  ONE `move-nodes` leaf; slider press = one transaction, cancel = zero trace; inspector patches are absolute leaves;
  editing a drag's `dx` through time travel replays the later drag and the overwritten head equals a fresh run; host wire
  decoding by name.

**OS workflow artifact (space's document; not in any fleet ownership list, edited because space needs the leaves)**
- New leaves `WF/🧬️schema/🧬️mutations/🚚️move-nodes` (`MoveNodes {nodeIds, dx, dy}`, binary tag 18, text opcode
  `move-nodes`) and `📍️set-node-positions` (`SetNodePositions {positions: [{nodeId, x, y}]}`, tag 19), descriptors, schemas
  with full `x-semio-ui`, leaf identity tests, wire witnesses `WF/🧫️fixtures/🧬️mutations/{🚚️move-nodes,📍️set-node-positions}/`,
  TS twins `WF/🧬️schema/🧬️mutations/🚚️move-nodes/🟦️.ts` (`parseMoveNodes`) and `📍️set-node-positions/🟦️.ts`
  (`parseSetNodePositions`, enforces `unique-node-ids`). These are the workflow artifact's first TS files; none of its
  other leaves has a twin.
- `WorkflowDiff::PlaceNodes {positions}` (validation + apply), `apply_workflow_operation` arms, retirement arms,
  `WorkflowNodePosition` retirement, aggregate schema oneOf, roster test (20 kinds, tags 0..19), op-text and backwards
  round trips, `move_nodes_is_relative_and_inverts_to_one_absolute_row` (`WF/🧪️tests/🔁️workflow/🦀️.rs`).

**space (hub)**
- `SP/🎮️commands/✏️node-graph-edit/🦀️.rs`: parses the operations ARRAY the bridge sends; `move` (gesture record) →
  `MoveNodes`; `setHostSnapshot` → `space_move_leaves` (absolute `MoveNode`s from `apply_flow_host_snapshot_to_os_workflow`
  regrouped by offset into `MoveNodes`) + structural ops; `disconnect {synapseId}` → `DisconnectEdge`; a drag commits as ONE
  transaction (`space_node_drag_emit`, tool `s.space.studio@1/*#editor#nodeGraphEdit`). Retained reducer passes the
  `ArtifactView` (`SP/🦀️.rs`).
- `🚚️move-media-node`: offset from base → ONE `MoveNodes` transaction; `Emit::amend` deleted.
- Panels: parameters name / text value / add-option and inspector label commit on blur (`Trigger::Commit`).
- Cargo (`🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/Cargo.toml`): path deps on the workflow artifact and the tool machine.
- Tests: `move_media_node_emits_one_relative_move` (spawn-app tests), snapshot drag = ONE `MoveNodes`, gesture record law,
  offset grouping.

**mathematical (`MA/`)**
- New graph-subset leaves `MA/🕸️graph/🧬️schema/🧬️mutations/🚚️move-nodes` (`MoveNodes {ids, dx, dy}`) and
  `📍️set-node-positions` (`SetNodePositions {positions: [{id, x, y}]}`, `x-semio-invariant: unique-node-ids`, the one-row
  inverse): leaf `🦀️.rs`, `🔺️diff`, `↩️inverse`, `🔣️.json`, `🧬️schema/🔣️.json` (full `x-semio-ui` en/de); wire witnesses
  `MA/🕸️graph/🧫️fixtures/🧬️mutations/{🚚️move-nodes,📍️set-node-positions}/🧾️wire-witness/🦠️mutation/🔣️.json`; crate root
  mounts; aggregate enum + `KINDS` (17) + aggregate `🔣️.json` oneOf + `📖️.grammar.semio` + `🟦️.ts` (interfaces +
  `parseMoveNodes`/`parseSetNodePositions`); hand-rolled text and binary codecs (tags 15, 16) + demo cases; graph oracle
  `kinds` + `deferredKinds` + manifest rows.
- Editor: `MA/✳️any/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs` owns the row decoder `EquationEditOperation` (now with
  the gesture record's `gestureId`) and `equation_edit_operation_leaves`: `addNode` → `create-node` (first free `n<k>`),
  `move` → `move-nodes`, `connect` → `connect-nodes` (first free `e<k>`, endpoints must exist, no parallel edge),
  `deleteSelection` → `disconnect-nodes` per incident edge then `delete-node` per node (point-invertible rows for the
  equation store's one-item footprint); `equation_edit_emit` commits a drag through `node_drag_commit` (tool
  `s.mathematical.equation@1/*#editor#nodeGraphEdit`). `🧮️set-algorithm` → `update-graph-algorithm`, `🧭️set-directed` →
  `change-graph-directed` (no-op when unchanged). The retained job (`MA/✳️any/✏️editor/🦀️.rs`) no longer copies the graph
  node-by-node and never emits `replace-graph` for these verbs: NodeGraphEdit = Initialize (working copy) → JSON bytes →
  decode → one step per row → Finish (extent `4 + json + rows`); SetAlgorithm/SetDirected = Initialize → Finish (extent 1);
  dead state (`rewrite_nodes`, `rewrite_edges`, `delete_ids`, `EquationOperationPhase`) deleted; bounded close retires the
  yielded leaves. `replace-graph` survives only for `setDocument` (whole-document set by intent).
- Laws: `a_node_drag_is_one_transaction_of_one_relative_move`, `graph_verbs_land_intent_leaves_only`
  (`🧮️set-algorithm/🧪️tests/🔬️unit/🦀️.rs`, replacing the absolute-row move test); `move_nodes_is_relative_and_inverts_to_one_absolute_row`,
  `gesture_leaves_follow_the_outcome_vocabulary` (aggregate unit tests); kinds count 17.
- Unblock: `equation_mutation_report_json` (`MA/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) called `protocol::to_dsl_value`, a
  serde bridge that a peer's value-crate migration removed (the 2 errors in `s-check-7`). `MutationMessage` now
  implements the first-party `ToValue` (`📡️replication/🎮️mutation/🦀️.rs:1063`), so both message lists encode through
  `ToValue`, and the stale "has not gained ToValue" comment inside the function is gone.

**sequence (`SQ/`)** — §12 is in source (`Emit::commit_child_transaction`, plugin lib compiled per S2-FLOWCAD S2.3)
- `SQ/✏️editor/🦀️.rs` region `🔖️ChildIntentLeaves`: `sequence_content_leaves(base, next)` diffs the composed Flow content
  child into intent leaves of the stdio semio flow subset (`remove-edge`, `remove-node`, `insert-node`, `set-node-kind`,
  `set-node-label`, `set-node-param`, `remove-node-param`, `drag-nodes` per offset, `set-edge-endpoints`, `set-edge-kind`,
  `insert-edge`); `sequence_child_emit_from_host_mutation` (13 editor sites) and the retained `sequence_artifact_emit_to_child`
  publish those leaves; the whole-content `set-snapshot` publish is deleted.
- `SQ/✏️editor/🎮️commands/🕸️node-graph/🦀️.rs`: the gesture record `move` moves its steps; a drag (record or snapshot)
  commits as ONE composed-child tool transaction (`node_drag_commit` + `Emit::commit_child_transaction`, tool
  `s.sequence.sequence@1/*#editor#nodeGraphEdit`). Cargo: tool-machine dep.
- Laws: `child_intent_leaves_reproduce_the_edited_content` (fold of the leaves equals the edited content, a two-step drag is
  ONE `drag-nodes`, no change no leaf), `a_node_drag_record_is_one_child_transaction`.

### 3. Verification

| Command | Result |
|---|---|
| `schema mutation-inputs --under ✏️s/🔌️plugins/{🕸️dag,➗️mathematical,🎬️sequence,🪐️space}` (baseline 12:36) | 0 findings each (36, 33, 17, 9 inputs) — `🗑️generated/s2-graphs/baseline-inputs.txt` |
| `schema mutation-payloads --under ✏️s/🔌️plugins/🕸️dag` (baseline) | 45/45 payloads, 17/17 leaves witnessed, 0 findings |
| `cargo check -p semio-s-artifact-dag-dag --tests` (`dag-check-1.txt`, `dag-check-2.txt`) | NOT COMPLETED: first run deadlocked in `prebuild_lock_exclusive` (sampled; killed my own cargo after 27 min), second run died with the 13:30 usage cut |
| `cargo check -p semio-s-artifact-dag-dag --tests` (`dag-check-3.txt`, 16:35–16:51) | FAILED in a peer crate before reaching mine: `semio-framework-ui` `🎯️targets/🧊️wgpu/⚡️events/🦀️.rs:691,2314` E0061/E0271 (in-flight peer edit) |
| `cargo check -p semio-s-artifact-dag-dag -p …-mathematical-equation -p …-sequence-sequence --tests` (`s-check-5.txt`, 17:26–17:32, after killing a fleet-wide `prebuild_lock_exclusive` cycle of 7 idle cargos, sampled per pid) | dag lib test: 0 errors, 22 warnings (2 in my code, fixed); sequence: 2 errors in my code (two import sites still called the deleted `sequence_child_replace_emit_from_parts`, fixed); mathematical: 2 errors in a PEER file (`✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`, staged by the sqlite-snapshot rollout, `store::os_io::IoResult` missing) |
| same command (`s-check-7.txt`, 21:59–22:15, after killing a second 7-cargo deadlock set incl. mine) | dag lib test: **0 errors**, 21 warnings; sequence lib test: **0 errors**, 58 warnings (pre-existing); mathematical lib test: 2 errors, both in pre-existing code of `MA/✳️any/🧬️schema/🧬️mutations/🦀️.rs:131-132` (`protocol::to_dsl_value`, removed from the kernel re-exports by a peer's in-flight value-crate migration) — no error in my mathematical code |
| `schema mutation-inputs --under` dag / mathematical / OS workflow (`lint-inputs-1.txt`, 17:12) | 0 real findings; only `leafUncatalogued` for the 3 + 2 + 2 new leaves (they enter the catalog with the central `schema generate`) |
| `schema mutation-payloads --under` dag / mathematical / OS workflow (`lint-payloads-1.txt`, 17:15) | 0 findings: dag 48/48 payloads, 20/20 leaves witnessed; mathematical 19/19, 18/18; workflow 25/25, 25/25 |

| Both lints, `--under` each scope (`lint-2.txt`, 03:0x): `schema mutation-inputs` / `schema mutation-payloads` | **0 findings in all 12 runs.** dag 43/43 inputs of 20 leaves, 48/48 payloads, 20/20 witnessed; mathematical 37/37 inputs, 19/19 payloads, 18/18 witnessed; OS workflow 41/41 inputs of 25 leaves, 25/25 payloads, 25/25 witnessed; sequence 17/17 inputs, 40/40 payloads, 8/8 witnessed; s space 9/9 inputs, 8/8 payloads, 6/6 witnessed; hub space has no leaves (0/0). The `leafUncatalogued` notes of `lint-inputs-1` are gone, so the new leaves are now catalogued |
| TS twins vs Rust wire witnesses (`bun -e`, 02:5x, single eval, allowed under the hold): each `parse<Type>()` of dag `move-nodes`/`set-node-positions`/`set-slider`, workflow `move-nodes`/`set-node-positions`, mathematical `move-nodes`/`set-node-positions` (inner payload of the externally tagged witness) | **23/23**: all 7 witnesses parse to an identical value (key-order-free compare), and 16/16 malformed variants (unknown field, duplicate ids, empty ids, infinite offset) are refused |
| Third-party oracle: Ajv (draft-07, `strict:false` for `x-semio-*`) compiles each of the 7 leaf `🧬️schema/🔣️.json` files and validates the same witnesses (`bun -e`) | **14/14**: 7 witnesses valid, 7 extra-field variants rejected (`additionalProperties:false`) |
| `cargo test -p semio-s-artifact-dag-dag --lib` (`dag-test-2.txt`, 22:2x) | NOT RUN: deadlocked in `prebuild_lock_exclusive`; at 22:48 I sampled and killed the idle set 59397, 84851, 89852 (mine), 89882 |
| `cargo test -p semio-s-artifact-dag-dag --lib` (`dag-test-3.txt`, 22:52–23:01, private target dir) | **136 passed, 92 failed**. 89 = `bundled dag example DSL must parse: expected LBrace` (peer's staged braced-record-list grammar vs the unmigrated dag demo asset — fixed in the asset since, see §2); 1 = my stale bridge test (rewritten, see §2); 1 = the peer G12 gate `history_edits_end_to_end` ("none of the 0 editable fixture case(s)", see §4); 1 = timing law `topology_of_a_large_layered_dag_stays_below_the_interactive_ceiling` (18.7 ms under fleet load). The demo-document panic masked my `✋️GestureLaws` and `🔖️GestureLeaves` tests, so they are still UNVERIFIED |

| `cargo test -p semio-s-artifact-dag-dag --lib` (`dag-test-4.txt`, 03:00–03:01, right after the hold was lifted) | BLOCKED before my crates: `semio-framework-os-kernel` has 30 errors from a peer's in-progress DSL `RecordSpecProducer` refactor (`🗣️dsl/🦀️.rs:508-700`, `🗣️dsl/🧬️schema/🦀️.rs:988`, `🗣️dsl/🪟️viewport/🦀️.rs:35,48`, `🏪️store/🦀️.rs:3869-4059`: "expected `RecordSpecProducer`, found fn item") |

dag and sequence type-check (lib + tests); everything else in §2 is WRITTEN BUT UNVERIFIED until the runs below land.
Fleet rule 26 (CARGO HOLD, 02:45) stops every cargo run until the coordinator lifts it. Pending after the hold, one at a
time: dag lib tests (rerun), sequence lib tests, mathematical lib tests (the 2 `to_dsl_value` errors are fixed in source, §2), workflow
`--tests` check and tests (root workspace), hub space `--tests` check and tests, `wasm32-wasip2` checks of
`semio-hub-{dag,mathematical,sequence,space}`, then both lints for sequence and space.

### 4. Open items

- React `NodeGraph/🟦️.tsx` now dispatches the gesture record on drag release (`nodeDragRow(record)`, line 1376), but
  `setHostSnapshot` is still sent at lines 977 and 3040. The guests turn a snapshot move into ONE relative leaf; the
  remaining snapshot sites belong to S2-FLOWCAD.
- CONTROLS: `document_input_commit_lint` does not flag a `commit("blur")` field bound on `Trigger::Change` (dead in React).
- G12 acceptance gate (`OS/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`, peer-owned, wired into dag, sequence and
  mathematical by its owner) can never pass for dag: `acceptance_cases` seeds each case from a JSON `⬅️before`, and the
  dag document's value form carries only the composed `content` child handle (`ArtifactChild::from_value` sets
  `local_owner: None`), so no JSON fixture can hold nodes. Every committed dag mutation case is a `🧪️rejects` case, so
  the gate finds 0 applied cases. The harness should seed from the case's DSL text (`⬅️before/🗣️.dsl.semio`) or from
  the app's default example. Adding a dag fixture alone cannot fix this.
- `mutate-dag-1` feature case not extended with the new leaves (they are `deferredKinds` in the oracle catalog).

### 5. Coordinator actions

- Re-describe dag, space (hub), mathematical and sequence after verification. The central catalog already lists the
  new leaves (`lint-2.txt` reports no `leafUncatalogued`).
- Braced record-list sweep: the UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O grammar change leaves about 23 `.dsl.semio`
  assets in the bare-record form (`git grep -l -E "=\[ [a-z-]+=" -- '*.dsl.semio'`). That includes the mathematical
  demo asset (`nodes=[ label=0 … ]`), which the same peer is currently reworking, so I left it alone. I migrated only the
  dag demo asset.
- G12: route the dag (composed-child) seeding gap to the acceptance-gate owner (§4).
- Mathematical: I fixed the `protocol::to_dsl_value` call sites (see §2). The in-flight SQLite snapshot I/O rework of
  the same artifact (peer) may still break its tests.

## Session 3 — 2026-10-02

Executor S3-GRAPHS (coordinator `⚪b7db773a…`). Scratch: `🗑️generated/s3-graphs/`. Focus (rule 29): verify → fix → close.

### S3.1 Repair-first diff (rule 28)

- `Emit::amend(` sites in dag / hub space / s space / mathematical / sequence / OS workflow trees: **0** (`git grep -n "Emit::amend(" -- '*.rs'`
  repo-wide finds only the plugin-builder contract test fixture `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:882`
  and a frozen ticket `.pre-patch.rs`). The "dag 2, space 1" sites of the census are the two `patchDagNodes`/`moveMediaNode` and
  the space `moveMediaNode` amends deleted in session 2 (§2).
- Session 2 ran `dag-test-5/6/7` after §3 was written (not reported): all three died in peer crates before reaching dag
  (`🎒️pack/🧪️tests/🧬️record-codec-laws` E0618, `🎒️pack/🌱️value/🏭️schema/🦀️.rs:70` E0308, `stdio-zip` `RecordSpecProducer` E0618).

- **Coordinator request (11:3x) — dag demo DSL "expected LBrace".** The grammar moved last: the braced `List<Record>` rule
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:982`, staged in the index, not in HEAD 4e36b2b; file mtime
  2026-10-02 04:17; peer ticket UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O, "no bare-record fallback") vs the OS infinite-dag asset
  `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🖼️assets/🎬️demo/🗣️.dsl.semio` (unchanged since 2026-09-22).
  Fixed: the asset now uses braced items (byte-identical to the migrated s-dag demo except `schema=dag.hostDocument`; pre-fix
  copy `🗑️generated/s3-graphs/infinite-dag-demo-before.dsl.semio`). Laws: the infinite-dag crate's existing
  `bundled_demo_fixture_is_canonical` (parse + canonical print) and new s-dag `every_bundled_example_parses_and_prints_canonically`
  (`DG/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs`, iterates the subset's declared `examples()`, now `pub(crate)` in `DG/🦀️.rs`).
- **Peer derive hygiene break** (reported to coordinator): `DslEnum` controlled encoding (`🗣️dsl/✨️derive/🦀️.rs:1401`) emits
  `let field=…` per field, shadowing a variant field named `field` → dag `🚪️io/🧬️mutations/📝️text/🦀️.rs:33` E0308 (`check-1.txt`).
  Fixed in my mirror only: `SetSlider { id, #[dsl(key = "field")] slider_field, value }` (wire key unchanged).
- **mutate-dag-1 completed for the three gesture leaves** (§4 open item; the catalog no longer defers them):
  - rejection fixture bundles `DG/🧫️fixtures/🧬️mutations/{🚚️move-nodes,📍️set-node-positions,🎚️set-slider}/🧪️rejects/` (closed
    quintet, `🔺️diff/🚫️.absent`, Error `mutation.target-missing`; move-nodes names every dragged id);
  - canonical leaf cases `DG/🧬️schema/🧬️mutations/<leaf>/🧪️tests/🧪️rejects/🦀️.rs` (after = committed, handle not re-minted, one
    Error diagnostic, payload invariant outranks lookup, empty inverse, canonical JSON, declared outcome), mounted in `🕸️dag/🦀️.rs`;
  - oracle catalog `DG/🔮️oracles/🔣️.json`: 3 vectors added, `deferredKinds` removed, `fixtureCoverage.vectors` 17, and the
    pre-existing create-node drift fixed (`directoryName` `🧪️rejects` → `🧪️rejects-a-duplicate-node-id`, the directory on disk);
  - feature `DG/🧪️tests/🌳️mutate-dag-1/🥒️.feature`: 3 rows in both Examples tables (real-effect params on the pipeline: drag
    scale+combine, place screen+mode, slider value 7.5) and the same create-node fixture fix; Rust adapter (`🦀️.rs` KINDS + arms)
    and Python second implementation (`🐍️.py` VECTORS, list-valued `path`) extended.

- **Coordinator item D2 — reasoning/wires node drag (design §20.2).** New relative leaf `🚚️move-nodes {nodeIds, dx, dy}` and
  its one-row absolute inverse `📍️set-node-positions {positions: [{nodeId, x, y}]}` (`x-semio-invariant: unique-node-ids`)
  under `W/🧬️schema/🧬️mutations/` (W = `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any`): leaf `🦀️.rs`,
  `🔺️diff`, `↩️inverse`, descriptor `🔣️.json`, `🧬️schema/{🔣️.json (full x-semio-ui en/de), 🔗️.graphql, 🛰️.proto, 🟦️.ts with
  parse<Type>()}`; aggregate enum + `KINDS` (12) + builders + helpers `board_node_position`/`wires_targets_invariant`/`wires_partial`;
  aggregate `🔣️.json`/`🟦️.ts`/`🔗️.graphql`/`🛰️.proto`; retirement arms + `WiresNodePosition` retire struct; crate-root mounts; Cargo
  `semio-framework-tool-machine`. Editor (`W/✏️editor/🦀️.rs`): `canvasPointerUp` no longer walks for x/y and emits an absolute
  `move_node`; it commits ONE relative `move-nodes` through `node_drag_commit` (`wires_node_drag_emit`, tool
  `reasoning-wires-play#canvasPointerUp`, gesture `<windowId>:<nodeId>`), zero offset = zero trace, cancel = zero trace; preview
  stays transient-only. WRITTEN BUT UNVERIFIED (builds blocked, see S3.2); evidence (fixtures, catalog, `📡️mutate-wires-1`) open.
- **Node-graph row contract (S3-FLOWCAD, binding).** Shared closed decoder in `🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs` region
  `🔖️NodeGraphEditRows`: `NodeGraphEditRow {Connect, Disconnect, Move(NodeDragRecord), SetSlider, InsertPort, Delete}`,
  `NodePortSide`, `node_graph_edit_rows(args)` (closed root: `operations` + scrub press fields, ≤ 256 rows); law
  `🛠️tool-machine/🧪️tests/🔬️node-graph-edit-rows/🦀️.rs` against the renderer fixture `🧫️node-graph-edit-rows` (all accepted rows map
  to their record, all refused rows refused alone and in a batch, closed bounded root). Flow adopted it (coordinator). Guests:
  - dag: `setHostSnapshot`/`deleteSelection` decoders deleted; `insert-port` (refused: no dag host inserts ports) and
    `delete {nodeIds, synapseIds}` (named wires, then `remove_nodes_operations`) added; `apply`/`apply_with_state` deleted
    (one `handle`); tests: connect-then-delete-the-named-node, renderer-fixture law, `every_command` rows; setHostSnapshot law deleted.
  - mathematical: rows `move`/`connect`/`disconnect`/`delete` via the shared decoder (`setSlider`/`insertPort` refused); the
    `addNode` row became the guest's own verb `addNode {x, y}` (`MA/✳️any/✏️editor/🎮️commands/➕️add-node/🦀️.rs`, command row
    appended, tool ids, publication contract, extent, retained initialize/finish, ToolExecutionContract, bridge, manifest
    args/describe, retained-law fixture `actions` + `registrationIdentity` 8); Actions-pane default is a `move` row; tests
    moved to `AddNode`, full connect/delete rows, unknown row refused, renderer-fixture law.
  - sequence: the RETAINED route (`SequenceNodeGraphState`, the live path for `nodeGraphEdit`) decodes rows via the shared decoder,
    applies `move` (it silently ignored drags before), `connect`, `disconnect`, `delete` (named ids, slot children discovered);
    `setHostSnapshot` stages and the ambient-selection delete are deleted; a batch that moved steps commits as ONE composed-child
    tool transaction (`drag_transaction` → `Emit.transaction` survives `sequence_artifact_emit_to_child`). The non-retained
    `handle` mirrors it; `apply` deleted. Tests: delete-the-named-step, renderer-fixture law.
  - hub space: rows via `space_node_graph_row` (refuses `setSlider`/`insertPort`); `delete` = named edges then `RemoveNode`;
    `setHostSnapshot` + `space_move_leaves` deleted; `edit` returns `Result`; tests: delete law, renderer-fixture law.

### S3.2 Verification (in progress)

### S3.3 Open items

### S3.4 Coordinator actions
