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
  `🛠️tool-machine/🧪️tests/🧪️node-graph-edit-rows/🦀️.rs` (open taxonomy pattern; first placed under an unregistered `🔬️` kind, moved after `verify taxonomy report` flagged `directory-kind-unresolved`) against the renderer fixture `🧫️node-graph-edit-rows` (all accepted rows map
  to their record, all refused rows refused alone and in a batch, closed bounded root). Flow adopted it (coordinator). Guests:
  - dag: `setHostSnapshot`/`deleteSelection` decoders deleted; `insert-port` (refused: no dag host inserts ports) and
    `delete {nodeIds, synapseIds}` (named wires, then `remove_nodes_operations`) added; `apply`/`apply_with_state` deleted
    (one `handle`); tests: connect-then-delete-the-named-node, renderer-fixture law, `every_command` rows; setHostSnapshot law deleted.
  - mathematical: rows `move`/`connect`/`disconnect`/`delete` via the shared decoder (`setSlider`/`insertPort` refused); the
    `addNode` row became the guest's own verb `addNode {x, y}` (`MA/✳️any/✏️editor/🎮️commands/➕️add-node/🦀️.rs`, command row
    appended, tool ids, publication contract, extent, retained initialize/finish, ToolExecutionContract, bridge, manifest
    args/describe, retained-law fixture `actions` + `registrationIdentity` 8); Actions-pane default is a `move` row; tests
    moved to `AddNode`, full connect/delete rows, unknown row refused, renderer-fixture law.
    Publication authority (`✏️s/🔌️plugins/➗️mathematical/🧫️fixtures/📣️publication-authority/🔣️.json`, `🧬️schema/🔣️.json` route enum,
    `🧪️tests/📣️publication-authority/🟦️.ts`) gains the `addNode` Artifact route and (10-03) the `setActiveExample` HostOnly route; the
    pre-existing red (stale package gates, missing HostOnly lane) is fixed — green, see S3.2.
  - sequence: the RETAINED route (`SequenceNodeGraphState`, the live path for `nodeGraphEdit`) decodes rows via the shared decoder,
    applies `move` (it silently ignored drags before), `connect`, `disconnect`, `delete` (named ids, slot children discovered);
    `setHostSnapshot` stages and the ambient-selection delete are deleted; a batch that moved steps commits as ONE composed-child
    tool transaction (`drag_transaction` → `Emit.transaction` survives `sequence_artifact_emit_to_child`). The non-retained
    `handle` mirrors it; `apply` deleted. Tests: delete-the-named-step, renderer-fixture law.
  - hub space: rows via `space_node_graph_row` (refuses `setSlider`/`insertPort`); `delete` = named edges then `RemoveNode`;
    `setHostSnapshot` + `space_move_leaves` deleted; `edit` returns `Result`; tests: delete law, renderer-fixture law.

- **Audit X2 — ONE drag-emit helper and ONE clock in `🛠️tool-machine` (`TM/🦀️.rs` region `🔖️NodeDrag`).**
  `authoring_clock(logical)` (host wall-clock ms, actor 0), `NodeDragEmit<M> {Commit(ref, leaves), Plain(leaves), Nothing}` +
  `committed()`, `node_drag_emit(app_id, verb, authoring_seed, gesture, leaves)` (tool `<app_id>#<verb>`; seedless view = plain;
  no leaves = nothing); `semio-framework-job` dep; law `a_node_drag_emits_one_transaction_plain_leaves_or_nothing`. Plugin crate
  (`OSM/🔌️plugin/🦀️.rs`): `impl From<NodeDragEmit<M>> for Emit` (commit / plain / default) and `Emit::node_drag_child::<S, M>(drag,
  slot, child_id)` (composed-child twin, §12). Every copy switched (mechanical edits in other plugins' trees):
  - drag-emit wrappers → `node_drag_emit(..).into()`: dag `✏️node-graph-edit` (`dag_node_drag_emit`), mathematical `🕸️node-graph-edit`
    (`equation_edit_emit`), hub space `✏️node-graph-edit` (`space_node_drag_emit`), wires `✏️editor` (`wires_node_drag_emit`),
    sequence (`✏️editor` `drag_transaction` + `🎮️commands/🕸️node-graph` via `Emit::node_drag_child`), wfc 2d/3d `🛠️tools/✋️drag`
    (`wfc{2,3}d_drag_tool` replaces `_tool_clock`/`_tool_commit`/`_tool_emit`; editor call sites + tests via `.committed()` /
    `NodeDragEmit::Plain`), flow `🛠️tools/✋️drag` (`flow_drag_tool` + `flow_drag_tool_emit` over `Emit::node_drag_child`; tests),
    procedural gen2d `generation2d_node_drag_emit` and gen3d `✏️node-graph-edit` (inline match + `generation3d_gesture_clock`
    deleted; `🧭️transforms` uses `authoring_clock(0)`), trinity rewriting `🕸️node-graph-edit` (`drag_tool_clock` deleted).
    No `node_drag_commit(` call remains outside `🛠️tool-machine`.
  - clock one-liners → `authoring_clock(n)` (16 files; wrappers with a single home inlined and deleted: `cad_transform_tool_clock`,
    `bitmap_brush_tool_clock`, `layout_transform_tool_clock`, `puzzle3d_transform_tool_clock` (3 files), `puzzle2d_select_tool_clock`,
    `lowpoly_tool_clock`; tick wrappers kept with the helper body: `note_ink_tool_clock`, `drawing_tool_clock`; direct sites: raster
    paint-stroke, process3d world, shooting gumball, fem 2d/3d gumball, plugin `⏯️tool-run:1947`, plugin `🛠️tool-machine:197,562`;
    the puzzle3d wrapper's callers in `🧊️3d/…/✏️editor/🦀️.rs:2757,4506`). No `default_now_ms().unwrap_or(0), logical` literal remains
    outside `🛠️tool-machine`.
  - `[TRACE]` in the dag drag hot path: `DAG` (`♾️infinite/🎲️board/…/🕸️dag/🦀️.rs`) already holds none (peer, 05:56); removed the
    wires retained-work `[TRACE]` eprintln (`W/✏️editor/🦀️.rs`). Left: 5 `[TRACE]` in `WGPU` (S3-FLOWCAD per audit), hub space
    `openSpace` (space is red, S3-INFRA).
- **Audit G1 — node-drag history laws.** Framework law `OSM/🔌️plugin/🧪️tests/🧪️node-drag-history/🦀️.rs` (`app::node_drag_history`,
  feature `artifact-app-testing`): `assert_node_drag_edits_replay_like_a_fresh_fold(guest, schema, base, log, edits, fold)` (per
  edit: preview = fold of the leaves before it; Report replay from the edited leaf = fresh fold of the edited log; never blocks
  finalize; overwrite commits that head; store retires) and `node_drag_transaction_rows(patch)`. Guests (re-offset AND re-target
  edits of a drag with a downstream drag):
  - wires `W/🧬️schema/🧬️mutations/🧪️tests/🧪️node-drag-history` (`move-nodes`), mounted in the aggregate `🧬️mutations/🦀️.rs`; mounted gesture
    half appended to `wires_batched_move_lands_on_its_last_sample_and_a_cancel_moves_nothing` (cancel = no Artifact lane + node
    unmoved, already there; a real release = exactly ONE `move-nodes` tool-transaction row and the node moved by offset/zoom);
  - mathematical `MA/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️node-drag-history` (graph `move-nodes`), mounted in that aggregate;
    mounted half = existing `a_node_drag_is_one_transaction_of_one_relative_move` (one row + zero-offset zero trace);
  - trinity rewriting `…/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️node-drag-history` (`drag-working-nodes` on the committed `✋️moves`
    graph); mounted half =
    existing `one_canvas_drag_is_one_history_row_labelled_from_its_leaf` / `a_released_working_graph…`;
  - sequence `SQ/🎮️commands/🕸️node-graph/🧪️tests/🧪️history` (child `drag-nodes` on the default sequence's content); mounted half =
    existing `a_node_drag_record_is_one_child_transaction`;
  - hub space: NOT written (crate red from the peer DSL extraction, S3-INFRA sweeping; coordinator: don't touch).
  - placement: a leaf's `🧪️tests/` holds only its fixture vectors (`verify taxonomy report`: `projection-catalog-coverage` "no exact
    one-to-one vector registry" when a law sits there), so the laws live in each aggregate's `🧪️tests/🧪️node-drag-history`
    (`taxonomy-4.txt`: no finding on them; the plugin-crate folder reports clean).
- **Peer API drift fixed in my trees** (found by `check-10`): `MutationMessage::warn`/`MutationOutcome::warn` → `warning` (wires,
  13 sites incl. my 2 leaves + `wires_partial`); `protocol::{Terminology, Locale}` → `semio_framework_ui_locale::…` (10 wires leaf
  tests); `dsl::json`/`store::json` → `semio_framework_pack_json` (+ `JsonMemberPolicy::Reject`) in wires editor and 3 sequence test
  files; mathematical `ArtifactStoreOneItemPreparation::close_step` → `ValueError` (`InvariantViolated`); the String-typed
  controlled codecs moved onto `ValueError` with typed refusal kinds (rows/work → `WorkLimit`, bytes/ownership →
  `OwnershipLimit`, allocation → `AllocationFailed`, else `InvalidValue`; trait boundaries keep `String` via
  `ValueError::into_message`, record closures via `TextError::from_value_error`): wires `📸️snapshot/{🪶️sqlite,📦️pack}`, mathematical
  `📸️snapshot/🪶️sqlite` and `🚪️io/📸️snapshot/💾️binary/{🦀️.rs, 🛫️encode, 🛬️decode}` — by the ticket input script
  `T/🧪️s3-graphs-value-error-sweep.py` (literal refusals + `Result<_, String>` + `--keep`/`--wrap` boundaries), then hand edits;
  wires `🤖️generated/🧠️wires/🦀️.rs` re-rendered with the current graph emitter (`renderGraphArtifacts`, in memory; only that file
  differed) because `wires/🛂️manifest/📇️outputs.json` (like all 8 plugin graph catalogs) lacks the new `contractId`/`policy`
  fields the catalog parser now requires; the mathematical editor test gained its missing `DeleteNode`/`DisconnectNodes` import
  (peer footprint law).

### S3.1b §20.15 — composed content is edited only on the child lane (coordinator, 10-03 ~11:00)

Scope (S3-AGNOSTIC gate `parentLeafReadsChild` in `schema mutation-editability`): sequence 8, dag 17 (`dag_working_scene`),
mathematical 16 (`equation_graph` 12, `equation_geometry` 4), wires 12 (`find_board_node` 8, `find_board_edge` 2,
`wires_working_board` 2), imperative/procedure 4 (`resolve_steps` 3, `procedure_working_scene` 1) — plus one
`composed_reload_law!` per plugin (S3-AGNOSTIC harness, `semio_framework_plugin::composed_reload_law!(plugin, Editor, manifest,
"../..")`: save → fresh load → equal head/DSL/child packs/rendered windows + one history edit on the reloaded instance).

- **Sequence — source done (gate 8 → 0 by construction, compile blocked).** `SequenceMutation` is now the uninhabited parent
  vocabulary (`✳️any/🧬️schema/🧬️mutations/🦀️.rs`, manual `Mutation`/`SemanticMutation`/`OpText`/`OpBinary`/`ColdRetire`
  like `NoConfigMutation`; twins `🔣️.json` = `"not": {}`, `🟦️.ts` = `never`, `🛰️.proto` = empty message; `🔗️.graphql` deleted —
  GraphQL has no empty input). Editor (`SQ/🦀️.rs`): every route edits a working scene read through `doc.children` and
  publishes `sequence_scene_leaves(scene, target)` child leaves (`sequence_retained_artifact_emit` builds the target scene;
  reorganize plans positions then diffs; the node-graph state drops its parent diff stages — `Complete(leaves)`, and a drag is
  `node_drag_emit` → `Emit::node_drag_child` on the `content` member, which also replaces the old parent-lane transaction path
  behind the "no transaction row" failure); the parent store preparation (`SequenceArtifactStorePreparation*`, the JSON
  byte-measure machinery, `admit_/prepare_sequence_artifact`, `sequence_delete_inverse`), `sequence_scene_after_mutations`,
  `sequence_artifact_emit_to_child`, `ops_from_host_mutation` and the `build_artifact_store_one_item_preparation_factory`
  override are deleted (no Artifact-lane tool exists). Operations (`⚙️operations/🦀️.rs`): store bridge + parent projection only
  (detection assembly, `KINDS`, apply/inverse and the case bridges deleted with their fixtures/tests). Crate root: step /
  dependency module trees, the io mutations facet mount and the `op`/`spr` shims removed; `diff_replace_content` deleted;
  genesis child pack and the G1 law built from the new `default_host_snapshot()` (no `local_owner` read). Deleted (files):
  `🪜️step` 109, `🔗️dependency` 41, `✳️any/🧪️tests/{🪜️mutate-sequence-1-any-step,🔗️mutate-sequence-1-any-dependency}` 2+2, the
  matching `🧫️fixtures` 2+2, `✳️any/🚪️io/🧬️mutations` 14, `✏️editor/🧪️tests/🔬️retained-json-contract` + `🧫️fixtures/🔁️retained-json.json`,
  `🧬️mutations/🧪️tests/🔬️structural-correspondence` (replaced by the empty-vocabulary law `🧪️tests/🔬️unit`), operations
  `🧫️fixtures` + `🧪️tests`; subset registry `🪆️subsets/🔣️.json` keeps only `any`; oracles drop `mutationCatalogs`; carrier-contract
  drops the mutation carrier-pair law; diff text law rewritten (parent `schema` delta, content handle untouched).
  `composed_reload_law!` added next to `history_edit_acceptance_law!`. S3-AGNOSTIC: sequence's representative child leaf =
  flow `drag-nodes`.
  Open: `🏭️generator` (+ its two standalone engines, nx project `@semio-tech/fixture-generator-sequence-1-any`, 2 launch.json
  options) only wrote the deleted parent carrier pairs — dead, to delete; the `csv-rfc4180-reader` / json-carried oracles and the
  `sequence-step-graph-mutation-semantics` decision lose their mutate capabilities (the identity round-trip case still tags
  them); central `🔣️taxonomy.json` still lists the four deleted case names (coordinator: regenerate). Readers outside the gate
  still on `local_owner`: `topology` inference, json/md/csv serializers (`try_to_host_snapshot`) — S3-AGNOSTIC's framework seam.
- **dag / wires / mathematical / imperative — design, in progress.** dag and wires compose `s.stdio.semio@v1/graph`, whose
  vocabulary has absolute `move-node` but no relative drag: a graph `drag-nodes {targets, dx, dy}` leaf (the flow twin) is needed
  for the node-drag tool and G1; dag's bridge must make the native `position`/`label` authoritative on decode (today the
  `dag.node` JSON property carries x/y too, so a native move would not survive decode). mathematical's three children
  (notation text, results table, computed value) are DERIVED from the graph state (`equation_children_from_state`), so its
  graph edits become child leaves on those three members; procedure composes a flow child (path) and a text child.

### S3.2 Verification (in progress; usage cut ~13:05, reboot ~17:00, resumed 18:41)

| Command (scratch `🗑️generated/s3-graphs/`) | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-{dag-dag,mathematical-equation,sequence-sequence} --lib --tests --keep-going` (`check-1.txt`, 11:21–11:35, before the row-contract edits) | mathematical lib + lib test **0 errors**; sequence lib + lib test **0 errors**; dag **1 error** (peer derive hygiene, `🚪️io/🧬️mutations/📝️text/🦀️.rs:33`, fixed in source since) |
| `check-2.txt` (dag only, 11:4x) | killed by me after 35 min idle in `prebuild_lock_exclusive` (sampled; no rustc child) |
| `check-3/4/5.txt` (dag + wires + mathematical + sequence, 12:35 / 12:52 / 13:06) | **BLOCKED in peer crates**: `semio-framework-schema-registry` duplicate definitions (12:35), then `🛂️manifest/🦀️.rs:1266-1269` + `♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🦀️.rs:48` (schema split sweep) — reported to coordinator |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-graphs cargo test -p semio-framework-tool-machine --lib` (`test-tool-machine-1.txt`, 12:55) | **31 passed, 0 failed** — incl. the 3 new `node_graph_edit_rows_tests` laws against the renderer fixture |
| `bun ./📜️script.ts test contract exhaustive --case mutate-dag-1` (`contract-dag-1.txt`, 11:22–12:0x, repo-wide 3410 breaches) | dag catalog: **0** `mutation-kind-uncovered` / `mutation-kinds-deferred` / `mutation-vector-*` breaches (create-node drift fixed); remaining dag rows are 3 `manifest-only-mutation` (stale runtime inventory cache, see S3.4) |
| `python3 T/🧪️s3-graphs-dag-oracle-vectors.py` (stubbed `semio_repo_test`, runs every registered oracle handler against the committed vectors, checks handler ids = feature ids = catalog kinds) | dag **34/34** scenarios, 17 kinds; wires (`… 🔌️wires/…/✳️any 📡️mutate-wires-1`) **24/24**, 12 kinds |
| Ajv (draft-07, `strict:false`) on the 3 new dag rejection payloads (+ extra-field negatives) | **6/6** |
| wires TS twins `parseMoveNodes`/`parseSetNodePositions` + Ajv on committed + real-effect payloads and 8 malformed variants (`bun -e`) | **23/23** (duplicate positions are the declared `unique-node-ids` invariant: twin refuses, draft-07 cannot) |
| `check-6.txt` (18:46, after the reboot) | SIGTERM (exit 143) after 14 min waiting on a lock — not a compile result |
| `check-7.txt` (19:0x–19:19) | **BLOCKED in the kernel**: `semio-framework-os-kernel` 61 errors, all `unresolved import semio_framework_dsl` (`🗣️dsl/{🦀️.rs:16, 📖️grammar:3, 🧠️lsp:4-9, 👪️family/*, 🖋️notation:17-19}`, `🏪️store/📦️codec/🪶️snapshot-capability/{🛬️native-decoding:3, 🛫️native-encoding:5}`) — peer dsl crate split, reported |
| `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-inputs\|mutation-payloads --under <tree>` (`lint-1.txt`, 19:2x; the root `schema` router no longer carries these subcommands) | **0 findings** in 11 of 12 runs: dag 43/43 inputs (20 leaves), 57/57 payloads, 20/20 witnessed; mathematical 37/37, 19/19, 18/18; sequence 17/17, 40/40, 8/8; s space 9/9, 8/8, 6/6; OS workflow 41/41, 25/25, 25/25; reasoning payloads 39/39, 14/14 witnessed. reasoning inputs 27/27 with **2 `leafUncatalogued`** (the new wires `move-nodes`/`set-node-positions`, cleared by the central `schema generate`, S3.4) |
| closed-bundle shape of the 5 new fixture bundles (dag `🧪️rejects` ×3, wires `🧪️reports` ×2) vs the contract's `MUTATION_FIXTURE_DIRECTORIES/FILES` + one diff variant + canonical case file (`python3` inline) | **5/5** |
| `bun ./📜️script.ts verify taxonomy report --scope 🧰️framework/🔨️modules/🛠️tool-machine` (`taxonomy-1.txt`, 19:33) | 1 error, mine: `directory-kind-unresolved 🧪️tests/🔬️node-graph-edit-rows` → moved to `🧪️tests/🧪️node-graph-edit-rows`; re-run (19:35) blocked by a peer syntax error in root `📜️script.ts:21073` ("Cannot use continue here") |
| same, 6 scopes after the move (`taxonomy-2.txt`, 19:4x) | `🛠️tool-machine` **clean**; mathematical `✏️editor/🎮️commands` (new `➕️add-node`) **clean**; dag + wires `🧬️schema/🧬️mutations` and `🧫️fixtures` scopes report `mutation-fixture-invalid` ("scenario root is absent") / `directory-kind-unresolved` / `mutation-fixture-unpaired` for EVERY bundle, the pre-existing ones (e.g. `✂️disconnect-nodes/🧪️rejects`) exactly like the new ones — a scoped-report artifact of the taxonomy engine, no finding specific to this work |
| `check-10.txt` (10-03, →05:56; dag + wires + mathematical + sequence `--lib --tests --keep-going`, after TREE GREEN core) | wires lib 169 / lib test 189, mathematical 107, sequence lib test 11 errors — peer API drift (ValueError codecs, `warn`→`warning`, json paths, private `Terminology`, stale generated manifest); fixed in my trees (S3.1) |
| `check-11.txt` (06:1x, same 4 crates) | wires lib + lib test **0 errors**; sequence lib + lib test **0 errors**; mathematical 1 + 3 (PackError mapping, missing test import) — fixed since; **dag never compiles: it depends on `semio-s-artifact-stdio-{svg,zip}`**, red from the same ValueError drift (`🎒️zip/…/🪶️sqlite` 46, `🎨️svg/…/🧬️schema` 6; S3-STDIO's trees) |
| `test-12.txt` (06:2x–06:30, `cargo test --lib`, private target, wires + mathematical + sequence) | mathematical **405 passed / 6 failed**, wires **207 / 13**, sequence **218 / 6** — triage in S3.3 (1 mine fixed: wires descriptor count 10 → 12; 1 mine open: sequence one-transaction law) |
| `bun -e verifyMathematicalPublicationAuthority(...)` (`pub-authority-1.txt`, 06:5x) | **green**: routes=8 (Ajv strict), oracle owned, 3 hostile mutations rejected, 2/2 example `bun test`s. Fixed: the `setActiveExample` HostOnly route (fixture + schema enum/lane `HostOnly`/min-maxItems 8 + oracle lane regex), and the package gates now check the repo-normalized manifest (`name`, description prefixed by it, no runtime `dependencies`, Ajv as dev dependency) instead of the pre-normalization description/`dependencies` |
| `check-12.txt` (06:58, tool-machine + plugin + my 4 crates) | kernel red mid-migration (`os_dsl::ValueRefusalKind`, `PackError::TextRefusal`, `into_…`; peer); tool-machine lib test `serde_json`/`blake3` unresolved because its tests must build through the ROOT workspace (non-member of `✏️s`) — not a code fault. The sqlite trait moved to `ValueError` at 06:53: dag, wires, mathematical (+ binary `{de,en}code_sqlite_native`) and sequence codecs moved onto it (wrappers removed; `validate_sqlite_snapshot_subset` via `IoError::from_value_error`) |
| `taxonomy-3/4.txt` (verify taxonomy report on the new law folders) | plugin `🧪️node-drag-history` clean; leaf-level placement flagged `projection-catalog-coverage` → moved to the aggregates, no finding left on them |
| `taxonomy-5.txt` (10:4x, after the usage cut; the 3 aggregate `🧪️tests` scopes, the 3 vacated leaf scopes, the plugin law folder) | relocation complete on disk (no `🧪️history` dir left under any leaf, old crate-root mounts 0, one aggregate mount each, trinity fixture `include_str!` re-based to 4 levels); every `🧪️node-drag-history` folder **clean**; remaining findings pre-existing peer ones (`🔬️kinds-conformance`, `🔬️structural-correspondence` unregistered kind; trinity `✋️drag-working` descriptor identity; wires scoped "scenario root is absent") |
| `check-13.txt` (10:46–10:50, ROOT workspace: `-p semio-framework-tool-machine -p semio-framework-plugin --lib --tests`) | tool-machine lib + lib test **0 errors** (`NodeDragEmit::committed`, `authoring_clock`, the emit law); plugin lib **0 errors** (`From<NodeDragEmit>`, `Emit::node_drag_child`, `authoring_clock` sites); plugin lib test 129 errors, **none in `🧪️node-drag-history`** — all in peer test fixtures still implementing the pre-06:53 `String` sqlite trait (`🧪️tests/{🖥️test-app-mutations-document, 🧩️composition, 🧬️mutation-fixtures-*, 🔬️app-declarations-fixture}`, `🏗️builder/🧪️tests/🔬️schema-stamping`) |
| `test-13.txt` (06:3x, sequence drag law with `[DEBUG]` probes) | did not build: `🔨️modules/🚪️io/🦀️.rs:2406…` `IoError` lost `message` (peer `IoError {cause: ValueError}` migration in flight, 06:31/06:48) |

Still owed once the peer schema split compiles (one command at a time, gated): `cargo check` of dag + wires + mathematical +
sequence (`--lib --tests`), hub space (`--manifest-path 🌎️hub/Cargo.toml -p semio-hub-space --lib --tests`), OS workflow
(`-p semio-framework-artifact-workflow-workflow --tests`), infinite dag (`-p semio-framework-artifact-infinite-dag --lib`, for
`bundled_demo_fixture_is_canonical`); then `cargo test --lib` of dag, mathematical, sequence, wires, hub space, workflow (private
target); `wasm32-wasip2` checks of `semio-hub-{dag,mathematical,sequence,space,reasoning}`; both schema lints `--under` each tree;
`test parity exhaustive --case mutate-dag-1` / `mutate-wires-1` (generated hosts).

### S3.3 Open items

- Sequence census item F (`ED:1242` `work_items: 1`): `admit_sequence_artifact_mutation` declares one row for every parent
  `SequenceMutation`, but `DeleteStep`'s inverse is `sequence_delete_inverse` = 2×steps + edges rows, so a parent-lane delete
  would be refused by `fold_batch_item`. Today no live tool publishes on the parent artifact lane (every sequence tool is
  `Child`/host-only and `sequence_artifact_emit_to_child` asserts no artifact mutation remains), so it never folds; the
  parent preparation is either dead (delete it) or must declare a derived footprint (§20.5, S3-CLOSURE). Not changed here.
- Census M items (hand-rolled `protocol::Edit { … coalesce_key: None … }` literals: dag `ED:481`, sequence `ED:1363`,
  mathematical `ED:919`, space core/home, hub space `:626`) are mechanical and wait for the CLOSURE `next_edit` helper.
- The S3-AGNOSTIC G12 dag seeding gap is unchanged here (theirs); facts in §4 of session 2.
- Other `nodeGraphEdit` guests outside this WP (procedural gen2d/gen3d, trinity rewriting/jack) still need the shared decoder.
- Stale doc comments in the mathematical snapshot rework (`MA/✳️any/🧬️schema/{🦀️.rs:31, 📸️snapshot/🦀️.rs:52, 🔺️diff/🦀️.rs:36}`)
  still mention the deleted `to_dsl_value` bridge; no code calls it (peer-owned files, left alone).
- 12 bundled `.dsl.semio` assets outside my trees remain in bare-record list form (list in the coordinator message, 11:3x).
- `test-12` failure triage (wires 13, mathematical 6, sequence 6):
  - **composed-child materialization gap (S3-AGNOSTIC / S3-FLOWCAD)** — the content child's local owner is absent after a pack
    load / fresh decode: wires `wires-drag-child-not-materialized` (`W/✏️editor/🦀️.rs:360,407`, `local_owner::<WiresWorkingScene>()`)
    in `pointer_down_selects_the_hit_node_inline` + 4 `window_transient` drag tests, `two_instances_converge…` (`node-1` missing on
    the peer), `codec_retention_law…` (fresh text decode: 0 nodes), `every_declared_wires_verb…` (agent lane diverges on
    `addNode`/`addRelationship`); sequence `history_edits_end_to_end` + `semio_payload_law_sequence_mutation` panic at
    `🎬️sequence/🦀️.rs:300` ("sequence child scene must be materialized before use"), `render_produces_a_read_only_scene…` (no
    steps); mathematical `renders_node_graph_scene` (no nodes), `render_produces_a_table_scene…` (empty JSON), likely the same.
  - **sqlite native controls not enforced** — wires ×2, mathematical ×2, sequence ×2 (`decode_sqlite_snapshot_native` /
    `encode_…` under `max_rows: 30` / small `max_value_bytes` return `Ok`, `!native_work` fails); sequence's codec was swept by the
    peer, so the cause is shared (store / sqlite-snapshot), not the per-plugin ValueError move.
  - `history_edit_inputs_resolve` (wires): payload `$ref` `https://json.schemas.assets.semio-tech.com/framework/value/schema.json`
    unresolved (`create-node`, `connect-nodes`) — peer schema split; mathematical `history_edits_end_to_end` (change-coefficient)
    and `math_document_text_round_trips_through_store`, wires `reorganize_start_complete_finalize_is_one_undo_entry` not triaged
    further (no change of mine touches them).
  - **mine, open:** sequence `a_node_drag_record_is_one_child_transaction` — the step moves, but 0 transaction rows. The
    `[DEBUG]` probes (`SQ/🦀️.rs` `drag_transaction`, the law) never ran (framework red) and were removed at 10:5x; the next
    sequence test run decides between an empty authoring seed in the retained step's `operation`, empty `artifact_mutations` at
    `drag_transaction`, and the composite publication dropping the parent row's `transaction`.

### S3.4 Coordinator actions

- Re-describe dag, mathematical (new `addNode` verb, new nodeGraphEdit default/describe), sequence, hub space and reasoning
  (wires gained two leaves); central `schema generate` for the two wires leaves (catalog + `🤖️generated`).
- `bun ./📜️script.ts test inventory --artifact s.dag.dag --standard 1 --subset any` (and `s.mathematical.equation … graph`,
  `s.reasoning.wires … any`) so the runtime inventory cache lists the new leaves (clears `manifest-only-mutation`).
- Re-activation of dag / mathematical / sequence / space / reasoning after the checks are green.
- Graph catalog contract: the 8 plugin `🛂️manifest/📇️outputs.json` (draw, puzzle 2d/3d/5d, writer, wires, trinity rewriting/jack)
  lack `contractId`/`policy`, so their graph generate refuses ("missing or unknown fields"); after migrating them, regenerate (the
  wires output already equals the current emitter).
- Sqlite-trait `ValueError` sweep (06:53) still owed outside my trees: procedural gen2d/gen3d and flow codecs keep the
  `(||…)().map_err(ValueError::into_message)` wrappers (S3-PROCEDURAL / S3-FLOWCAD), stdio zip/svg (S3-STDIO; blocks dag), the
  plugin-crate test fixtures listed under `check-13`. Then, one gated run each: `cargo check` + `cargo test --lib` (private
  target) for dag, wires, mathematical, sequence; root-workspace `cargo test -p semio-framework-tool-machine --lib` and the
  plugin crate's `node_drag_history` users; the X2-touched crates (wfc 2d/3d, flow, gen2d/gen3d, trinity rewriting, raster,
  process3d, cad, wfc bitmap, note, shooting, layout, puzzle 2d/3d, fem 2d/3d, drawing, lowpoly) `--lib --tests`.

## Session 4 — 2026-10-04

S4-GRAPHS (Opus, successor of S3-GRAPHS). Scope: dag, sequence, imperative/procedure, space, shared node-drag helpers
(`🛠️tool-machine`), graph child leaf `drag-nodes`, §20.15 dag (17) + procedure (4), D6 sequence drag row, dead sequence
`🏭️generator`. Scratch: `🗑️generated/s4-graphs/`.

### S4.0 Status (kept current)

- 02:2x started; rule 34 repair-first: no half-edit of S3-GRAPHS in dag/sequence/imperative/tool-machine (content = index); the
  graph `drag-nodes` leaf was already complete on disk (S3, 10-03 12:04: aggregate, binary tag 13, text, json/ts/proto/graphql,
  fixture `✋️drags`, feature rows, Python oracle) — only `x-semio-inverse-rows {perTarget: targets:1}` was missing (added). The dead
  sequence `🏭️generator` is already deleted (staged `D`, 8 files) with no reference left in `.vscode`/`.claude` launch files,
  `✏️s/Cargo.toml` or the sequence tree.
- 02:4x **D6 root cause** (static): `close_streamed_transaction_unit` (PLG, migrated publication) stripped `emit.transaction` from
  every Commit-phase emit with no artifact mutations — every child-only tool transaction (`Emit::node_drag_child`,
  `commit_child_transaction`, child scrub presses) lost its ref before `PendingChildGroupPublication::new(…, emit.transaction.take())`.
  Fixed: the ref is kept when `child_emits` is non-empty (abort still strips). Run pending.
- 02:5x graph child vocabulary for dag/wires/procedure/trinity (coordinator-approved): `set-node-property`, `resize-node`,
  `rename-node` (tags 14–16) + optional `at` on `create-node`/`create-edge` so `delete-node`/`delete-edge` undo is byte-exact
  (content address kept). An interim `restore-node`/`restore-edge` attempt was backed out (coordinator chose `at`). The wave made
  `stdio-semio` red for a few minutes (registration landed before the `RetireOwned` match) — fixed; rule 39 noted.
- 03:24 **STDIO-SEMIO GREEN** (`cargo check -p semio-s-artifact-stdio-semio --lib` exit 0) incl. the edge trio
  `set-edge-property`/`add-edge-property`/`remove-edge-property` (tags 17–19, S4-TEXT request). Graph lib tests (`-- graph::`):
  run 2 219/115 (fixtures used the plain JSON codec, diff fixtures stale since the peer's `width`/`height`/`properties` adds) →
  switched fixture tests to the declared JSON codec (`decode/encode_semio_graph_snapshot_json`), regenerated 15 diff fixtures from
  their after-snapshots, `patch-snapshot` production added to the graph op grammar → run 3 318/16 (create-node/create-edge/
  add-node-port payload fixtures + Nakagin feature rows lacked the new fields — patched; `fixture_honesty_law`: the wires asset
  DSL is 7-field, peer drift, open).
- 03:4x–04:1x **dag §20.15 conversion written** (see S4.2): `DagMutation` uninhabited, 17 parent leaf dirs + fixtures + io
  mutation codecs + `🌳️mutate-dag-1` deleted, editor/viewer/commands/reorganize compose the `content` graph child.
- 04:15 usage cut; 06:45 resumed. First dag lib check (04:24): 8 errors — 1 mine (`typed_read` deref), 7 peer drift in my
  tree (`DagExpandedPaths`, `IoError {cause}`, `close_step → ValueError`) — all fixed in source.
- 07:2x two dag/procedure batch checks killed (exit 137) by disk pressure while compiling dependencies; rules 42/43/44 then
  froze tests and cargo. All my later verification is the Python oracle run plus the stdio-semio checks the coordinator
  authorized one by one.
- 11:5x–12:2x **procedure §20.15 conversion completed** (see S4.3). It covers the bridge, scene, genesis, commands, engine,
  windows, the handle-only demo asset, grammar trimming, and every test file.
- 12:1x **F15 (audit-s4-core)**: `remove-node-property`/`remove-edge-property` are keyed by property name. Details in S4.1.
- 12:2x **graph oracle/fixture drift fixed** (coordinator-approved). Details in S4.1.
- 12:33 stdio-semio check gave no verdict: the plugin crate was red from a peer (`⏪️time-travel` `HistoryPageStack` E0425 ×2).
  12:38: no verdict again, because a build-dir sweep deleted a fingerprint dir mid-run.
  **12:42 STDIO-SEMIO GREEN** (exit 0, 0 errors, 230 warnings). 12:49 GREEN again after the dead-wrapper removal (rule 45; 228 warnings).
- 12:4x notices: every retained tool-mismatch in dag/sequence/space/procedure now raises the framework `app.command.tool-mismatch`.
  Payload-extent refusals raise `mutation.too-large`. Both codes come with framework en/de notices.

### S4.1 Graph vocabulary (`s.stdio.semio@v1/graph`, shared child lane)

- Leaves added: `set-node-property`, `resize-node`, `rename-node`, `set-edge-property`, `add-edge-property`,
  `remove-edge-property` (binary tags 14–19). Each ships:
  - leaf / diff / inverse modules;
  - a schema with full `x-semio-ui` en/de;
  - a descriptor;
  - a fixture quintet plus its fixture test;
  - text keyword, grammar, ebnf and g4 entries;
  - json/ts/proto/graphql twins;
  - oracle catalog rows, feature rows and Python oracle verbs.

  `drag-nodes` gained `x-semio-inverse-rows {perTarget: targets:1}`.
- `create-node`/`create-edge` carry an optional `at` insert index (absent = append, bounded 1048575). The `delete-node` and
  `delete-edge` inverses fill it with the original index, so delete followed by undo restores byte-identical snapshot bytes
  (same content address). Law: `delete_then_undo_restores_byte_identical_snapshot_bytes`.
- **F15**: `remove-node-property {node_id, key}` and `remove-edge-property {edge_id, key}` replace the positional `{…, index}`.
  A history edit of an earlier row can therefore no longer re-target the remove.
  - Their inverse is `add-*-property` at the entry's BASE index (byte-exact).
  - The `add-*-property` inverse is `remove-*-property {key}` and is emitted only when the add applies. The node variant used
    to undo a no-op add by detaching the pre-existing entry; that bug is fixed.
  - Updated together: Rust, leaf schemas, text codec + grammar/ebnf/g4, TS/proto/graphql, RetireOwned, unit/fixture tests,
    committed mutation fixtures (`key: "weight"`), Python oracle, feature rows (`ComposePieceAttributes.name`, `ConnectionKind`).
- Set-property labels render scalar values (`semio_value_scalar_label` in `🔢️value/🧬️schema/📸️snapshot`).
- **Oracle/fixture drift** (peer layout change had left both sides unable to pass):
  - The Python oracle `🌳️mutate-semio-graph/🐍️.py` now reads and prints the current 9/4/8-field DSL — node
    `x,y,width,height`; port `name,kind,category,[properties]`; edge `…,optional-text×2,[properties]` — and the matching pack
    layout.
  - It keeps geometry as the declared JSON words `{"bits": <16 hex>}`, so its documents equal the Rust declared-JSON projection
    and the `{bits}` spec vectors.
  - The tower carrier (`🧫️fixtures/🌳️mutate-semio-graph/…/🗣️.dsl.semio` + `🎒️.pack.semio`) and the wires asset
    (`🖼️assets/🕸️wires/…`) were re-emitted by the oracle at the new layout, with content unchanged (width/height 0, empty
    category/properties). The provenance docstrings say so.
- Dead code removed: `enc_port_kind`/`dec_port_kind` wrappers in the graph op text codec.

### S4.2 dag §20.15 (17 findings)

- `DagMutation` is uninhabited, with empty json/ts/proto/graphql twins and a unit law.
- Editor, viewer, commands, windows, panels and inferences read the `content` graph child: `DagScene`,
  `dag_scene_from_children`, `dag_derivable_scene`, `genesis_dag_child_pack`.
- Every gesture yields its graph intent leaf directly (`create_node_leaf`, `create_edge_leaf`, `delete_edge_leaf`,
  `drag_nodes_leaf`, `rename_node_leaf`, field/remove helpers). There is no snapshot→diff path.
- Reorganize runs on S4-WIRES-MATH's child-target ToolRun (`ToolRunDefinition.member = "content"`, `member_ops`).
  Its 4 anonymous faults are now `dag.layout-run.{layered-load,layered-layout,layered-result,start}` with en/de.
- Laws: `composed_reload_law!("dag", …)` and `composed_child_history_law!("dag", …, addNode)`. The parent-only
  `history_edit_acceptance_law!` was removed: zero parent leaves would exhaust it.
- Deleted (leaf dirs): `↔️move-node`, `✂️disconnect-nodes`, `🌱create-node`, `🎚️set-slider`, `🏷️rename-node`,
  `📍️set-node-positions`, `📐resize-node`, `🔀reorder-nodes`, `🔁replace-node-kind`, `🔡change-node-abbreviation`,
  `🔤change-node-name`, `🖼️change-node-icon`, `🗃️replace-node-properties`, `🗑️delete-node`, `🚚️move-nodes`,
  `🤝️connect-nodes`, `🧮change-node-operator-kind`.
- Also deleted: `✳️any/🧫️fixtures`, `✳️any/🚪️io/🧬️mutations`, `✳️any/🧪️tests/🌳️mutate-dag-1`,
  `🕸️dag/🧫️fixtures/🧫️child-owner-isolation`, and the command-envelope test.

### S4.3 procedure/imperative §20.15 (4 findings)

- The program is the `flow` child and the seed is the `text` child.
  - Bridge: one node per step (nested bodies included). Order is a chain of `sequence` edges (`s-{from}-{to}`); a body slot is
    one `body` edge (`b-{owner}-{slot}`, port = slot); params are one JSON value each. The seed is one JSON run.
  - Scene: `ProcedureScene{path,seed}`, whose Drop retires the seed. `procedure_scene_from_children` composes it; the
    derivable demo and empty program feed genesis and inference.
  - Edits: `procedure_flow_leaves` yields point-invertible flow leaves (severed edges first, inserted edges last), published
    as ONE child edit by `procedure_edit_emit`.
- `ProcedureMutation` is uninhabited. The parent text and pack carry only `schema/flow/text`. The grammar, ebnf and g4 no
  longer have `path=`/`seed=`. The dead `StepNodeDsl`/`PathDsl` twins were removed.
- `ImperativeHost::from_scene`. Commands use `schema::operations` program edits (`path_ref_in`, `next_step_id`,
  `insert_step`, `remove_step`, `move_step`, `set_step_params`).
- The demo asset is handle-only: `demo-flow`/`demo-text`, with derivable content = `schema::default_path()`.
- Tests: all rewritten.
  - Root, operations, diff-text, snapshot-text, inference, engine, panel and viewer windows.
  - sqlite: 5 local-scene tests and the `🎛️native-scenes.json` fixture deleted (local owners no longer exist).
  - Editor: `live_scene` from the member stores; convergence and idempotence over the composed scene;
    `composed_reload_law!` + `composed_child_history_law!("imperative", …, addStep)`.
- Deleted: the 4 leaf dirs (`🌱create-step`, `🔀reorder-steps`, `🔧edit-step-params`, `🗑️delete-step`), their binary/text
  codecs, the structural-correspondence test, `✳️any/🧫️fixtures/🧬️mutations`, `✳️any/🧪️tests/🛟️mutate-procedure-1`, and
  `📜️procedure/🧫️fixtures/🧫️child-owner-isolation`.

### S4.4 D6, sequence, space

- D6 root cause and fix are in S4.0. `🏭️generator` was already deleted with no launch rows left.
- Notices: `sequence.retained.tool-mismatch`, `s.space.index.retained.tool-mismatch`, and the space home editor/viewer
  anonymous faults now raise `app.command.tool-mismatch`; the payload-extent faults raise `mutation.too-large`.

### S4.5 Verification

| Command | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-semio --lib --message-format=short` | 12:42 **GREEN** (0 errors, 230 warnings) |
| same, rule-45 re-check after removing the dead port-kind wrappers | 12:49 **GREEN** (0 errors, 228 warnings) |
| `python3 T/🧪️s4-graphs-python-oracle-rows.py` (graph oracle alone: every mutate/inverse row on the tower + every spec vector) | 54 passed, 2 not runnable standalone (`patch-snapshot` needs the real `semio_repo_test.patched_snapshot`) |
| `python3 T/🧪️s3-graphs-dag-oracle-vectors.py` | not applicable: dag has no parent kinds and `🌳️mutate-dag-1` is deleted; the graph oracle above covers dag's child leaves |
| dag / sequence / procedure `cargo check --lib` (native + wasip2) | OWED (rule 44) |
| dag / sequence / procedure / graph `cargo test --lib`, incl. D6 `a_node_drag_record_is_one_child_transaction` | OWED (rule 43/44) |
| `cargo test -p semio-framework-tool-machine --lib` | OWED (rule 43/44) |
| hub `semio-hub-{dag,sequence,space,imperative}` check native + wasip2 → COMPOSITION GREEN | OWED (rule 44) |
| G1 replay law | OWED (rule 43/44) |

### S4.6 Coordinator actions

- Run, once cargo is open, as ONE gated batch: `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-dag-dag -p
  semio-s-artifact-imperative-procedure -p semio-s-artifact-sequence-sequence -p semio-s-artifact-space-space
  -p semio-s-artifact-space-home --lib
  --message-format=short`, then `--tests`, then the hub crates with `--target wasm32-wasip2`. The dag/procedure sources have
  NOT been compiled since the conversion.
- Central `schema generate` for the graph leaf schemas (6 new + the 2 keyed removes); `🧰️framework/…/📚️library/🔣️schema-catalog.json`
  and `📓️schema-catalog.md` still list `index`.
- Describe wave: dag, imperative (`🌎️hub/🧩️compositions/📜️imperative/{🔣️.json,🛂️.descriptor.semio}` still name the old demo
  child ids `imperative-flow-ea1c…`/`imperative-text-affb…`), sequence, space, stdio.
- Test inventory: `bun ./📜️script.ts test inventory --artifact s.stdio.semio --standard v1 --subset graph`; dag and
  procedure now have no parent kinds (their inventories are child-lane only).
- Run the `mutate-semio-graph` differential through the repo host. The Rust side must reproduce the oracle-emitted tower and
  wires carriers byte for byte.
- `launch.json`: unchanged (no new or removed executable commands).

### S4.7 AUDIT-TOOLS items (`📓️audit-s4-tools.md`) — PARKED at the coordinator's usage limit

State when parked. Every edited file is consistent; no edit was left half-done. Everything below is source-only, and all cargo
is OWED (rule 44).

- **F7 (major), DONE in source**: a reloaded sequence no longer panics. All in the sequence plugin, which is not a shared crate.
  - Root `🦀️.rs`:
    - `sequence_working_scene` (`.expect("…materialized…")`) is deleted.
    - New `sequence_derivable_scene(snapshot) -> Option<ColdOwner<SequenceWorkingScene>>` (genesis content when the
      `content` child id is the genesis one). `genesis_sequence_child_pack` uses it.
  - `🧭topology/🦀️.rs`:
    - `compute_sequence_topology(snapshot)` = derivable scene or the EMPTY topology (total for decoded, reloaded and remote
      parents).
    - The Kahn pass moved to `compute_scene_topology(steps, edges)`.
    - Not done: the `protocol::Inference::infer(&snapshot)` signature gets no request dependencies, so the
      `child:<slot>/<id>` head pack (`inference_child`) is unreachable from this trait. Wiring it needs a framework hook
      (S4-LOAD follow-up).
  - `📸️snapshot/🦀️.rs`:
    - `ColdRetire` uses `if let Ok(Some(owner))`, so the `.expect("exact sequence scene owner")` is gone.
    - `to_host_snapshot` (an expect) is `#[cfg(test)]` only; shipped code uses `try_to_host_snapshot`.
  - `✏️editor/🦀️.rs`:
    - Dead `next_available_step_id`, `SequenceHost::from_snapshot` and `host_from_snapshot` are deleted.
    - `Default for SequenceHost` builds from `default_host_snapshot()`.
    - The unused `default_snapshot` import is gone.
  - Tests:
    - topology: uses `compute_scene_topology`, plus a new law `a_non_derivable_parent_infers_the_empty_topology_and_genesis_its_chain`.
    - editor: the `next_available_step_id` test is deleted; `crate::default_snapshot` is imported explicitly.
    - viewer main window: uses `sequence_derivable_scene`.
  - OWED: `cargo check -p semio-s-artifact-sequence-sequence --lib --tests`, then `cargo test --lib` (topology and inference laws).
- **F13, NOT STARTED** (only read). Next steps:
  1. dag/sequence `🧬️schema/🔺️diff/` defines `DagDiff`/`SequenceDiff`, which is still the uninhabited mutation's
     `type Diff`. Check `protocol::Mutation`'s `Diff` bound for a framework unit diff before deleting the 5 files
     (`🔣️.json`, `🔗️.graphql`, `🦀️.rs`, `🟦️.ts`, `🛰️.proto`). Then do the same for `🚪️io/🔺️diff/{💾️binary,📝️text}` and its tests.
  2. Rewrite `🎬️sequence/…/✳️any/🔮️oracles/🔣️.json`: the `csv-rfc4180-reader` capability `sequence-1-mutate` and the
     decision `sequence-step-graph-mutation-semantics` still name the deleted parent leaves.
- **F3 (major), NOT STARTED.** `🗑️delete-node`'s diff must refuse degree > 1024 with `mutation.too-large` (or declare the rows
  from the degree), plus a cap+1 law.
  - stdio-semio is a shared crate in the activation closure, so stage the edit under `🗑️generated/s4-graphs/` (rule 45) and
    apply it as one write followed by the gated check.
- **F18, NOT STARTED.** Scope:
  - `x-semio-ui` reference descriptors (`role: target`) on `add-node-property` `node_id`/`index`, and on the bare
    `id`/`node_id` of move-node, delete-node, delete-edge, change-node-label, change-node-kind, add-node-port and
    remove-node-port;
  - payload key casing unification;
  - `➕add-node-property/🔺️diff` line comment → docstring, and `.expect("checked above")` → named fault;
  - record the O(graph) clone per leaf as a follow-up.

  This is also stdio-semio, so stage it per rule 45.

## Session 5 — 2026-10-05

S5-GRAPHS-WIRES (Opus, successor of S4-GRAPHS and S4-WIRES-MATH; coordinator `⚪3f26aaa1…`). Scope: dag, sequence,
imperative/procedure, space (+ home), wires, mathematical and their hub compositions; the stdio-semio `🕸️graph` child
vocabulary and `⏯️tool-run` (both inside the landing closure, rule 51). Scratch: `🗑️generated/s5-graphs-wires/`.
Companion sections: `📓️s3-wires-report.md` and `📓️s3-math-report.md` § Session 5.

### S5.0 Status (kept current, newest last)

- 01:10 started. Rule 46 repair-first: files in the owned trees newer than the 10-04 18:27 stash-pop are peer sweeps
  (21:19 pack-error conversion, 23:19 command/root files) plus S4-WIRES-MATH's own 21:0x–22:55 wave; no half-edit found by
  reading, the compile below is the real check.
- 01:11–01:17 **first compile of the inherited trees** (`check-lib-1.txt`, gate v3, 6m29s, exit 101): one batched native
  `--lib --keep-going` over all seven crates. Six crates type-check; procedure had 2 errors, both peer drift in my tree.
- 01:2x procedure fixed in source (see S5.1). Build gate v4 (rule 55) in force from here on.
- 01:26 `--lib --tests` batch attempt 1 (`check-tests-1.txt`): no verdict, the kernel was red from a peer
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs:219` E0308 `EncodeOptions`). 01:31 attempt 2
  (`check-tests-2.txt`): no verdict, `semio-framework-pack` red (13× E0308 `VerificationLevel`,
  `🎒️pack/🌱️value/🦀️.rs:3125–3283`). Both reported to `main`; the Codex pack peer is mid-wave (rule 56). No cargo of mine
  while `🗑️generated/coord/foundation.status` reads RED.
- 01:3x **F3 staged** (S5.5) while the foundation is red; the Python half of its law is run and green on the staged oracle.
- 01:3x design §22.8 input declarations applied and gate-verified (S5.6).
- 01:43 foundation GREEN (rule 56). 01:48–01:52 **`--lib --tests` batch 3** (`check-tests-3.txt`): dag, sequence, space,
  home, wires, mathematical type-check incl. their test targets; procedure lib green, its lib-test had 2 errors in my
  tree, fixed in source (S5.1).
- 01:57–02:00 **F3 landed** under the `stdio` lock (rule 58), verifying check green, lock released 02:00:41.
- 02:0x sequence fault notices (S5.7) prepared and dry-run; applied together with the next seven-crate batch.
- 02:13–02:16 **batch 4 GREEN** (`check-tests-4.txt`): all seven crates `--lib --tests`, 0 errors, with F3 in stdio-semio,
  the two procedure test fixes and the sequence notices applied. The fault gate then shows sequence 88 → 1 (S5.7).
- 02:25–02:30 hub wasip2 batch 1 (`check-hub-1.txt`): `semio-s-artifact-space-space` red on wasm only
  (`🪐️space/🫀️core/🧫️fixtures/🦀️.rs:58–59` E0603 `store::DslValue` is private) — fixed in source
  (`semio_framework_value::DslValue`; the crate already depends on it).
- 02:33–02:38 hub wasip2 batch 2 (`check-hub-2.txt`, pre-wave-B tree): space-space 0 errors / 9 warnings, home 0 / 3;
  the ONLY failing crate was `semio-hub-space` (189 errors, e.g. `semio_framework_plugin::{Label, LocalizedLabel,
  app_labels}` unresolved in `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/**`); dag, sequence, imperative, reasoning,
  mathematical hubs raised no error in that run. No COMPOSITION GREEN was announced from it: the tree changed (wave B).
- ~02:40 usage cut (whole fleet). **07:28 resumed** (rule 62). Repair-first: F3 is complete on disk (all four targets equal
  their staged `*.after.*` copies; the `stdio` lock was released 02:00:41 after the green check); the Python law passes on
  the tree (4/0); the sequence notices and the space fixture fix were compiled before the cut (batch 4, hub batch 2).
  Peer edits in my trees since the cut: wave B / sweeps at 06:14 (dag editor root, space core + home config, equation editor
  root + two codec tests, `PLG/⏯️tool-run`, both tool-run test files), a sqlite contract wave in dag at 05:49, and a
  65-file conversion of the hub space engine at 04:58.
- 07:35 batch 5 started (seven crates `--lib --tests` on the wave-B tree, cold after the 04:22 prune).
- 07:35–~07:45 **batch 5 was cut by the 07:45 usage cut** (`check-tests-5.txt` ends without dag, sequence or wires ever being
  checked). Verdicts it did reach on the wave-B tree: procedure lib 0 errors / 8 warnings and lib-test 0 / 18 (the two
  S5.1 test fixes hold), home 0 / 1 and 0 / 2, space 0 / 4 and 0 / 5, mathematical lib 0 / 32; **mathematical lib-test 1
  error** (wave-B shape: `✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:546` passes 4 arguments to
  `ArtifactStoreOneItemPreparationFactory::preflight`, which takes 3). dag, sequence, wires: NO VERDICT.
- 16:46 resumed for the coordinator's tool-run job only (`tool_run` law family 33 / 9). Everything of that job is in
  `📓️s3-wires-report.md` § T5: wave `tool-run-settle` landed 17:06:32 + 17:08:02, law run 39 / 3, follow-up staged.

### S5.1 Changes

- `…/📜️procedure/…/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs` (L11): the one test file the
  §20.15 rewrite missed still built a `ProcedureSnapshot` for renders that take a `ProcedureScene`; it now derives the
  default document's scene (`crate::procedure_derivable_scene(&crate::ProcedureSnapshot::default())`).
- `…/📜️procedure/…/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs` (L5): the `restore` helper returns
  `Result<_, semio_framework_value::ValueError>` (the sqlite snapshot ABI no longer yields `String`).
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (2 sites, L194 and
  L212): `dsl::json::to_json_string` → `semio_framework_pack_json::to_json_string`. The JSON printer left the `dsl` crate;
  the procedure crate already depends on `semio-framework-pack-json` and the same file already calls it elsewhere. A sweep
  of all six plugin trees and the six hub compositions for `dsl::json::` finds no other site.

### S5.2 Verification

| Command | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-dag-dag -p semio-s-artifact-imperative-procedure -p semio-s-artifact-sequence-sequence -p semio-s-artifact-space-space -p semio-s-artifact-space-home -p semio-s-artifact-reasoning-wires -p semio-s-artifact-mathematical-equation --lib --keep-going --message-format=short` (01:11–01:17, `check-lib-1.txt`) | exit 101. Type-checked with warnings as proof: **dag 0 errors / 13 warnings** (first compile since the §20.15 conversion), **sequence 0 / 13**, **space 0 / 4**, **home 0 / 1**, **wires 0 / 161**, **mathematical 0 / 32**; **procedure 2 errors** (E0433 `dsl::json`, fixed in S5.1) / 8 warnings. Dependencies in the same run: stdio-semio 0 / 228, plugin 0 / 283, tool-run 0 / 91, kernel 0 / 425 |
| same seven crates `--lib --tests --keep-going`, `CARGO_BUILD_JOBS=3` (01:48:56–01:52:20, `check-tests-3.txt`; includes the 39 input declarations of S5.6) | exit 101. 0 errors with warnings as proof (lib / lib-test): **dag 13 / 30**, **sequence 13 / 82**, **space 4 / 5**, **home 1 / 2**, **wires 161 / 259**, **mathematical 32 / 42**; **procedure lib 0 errors / 8 warnings**, procedure lib-test **2 errors** (E0433 `ProcedureSnapshot` in `🔬️semantic-contract`, E0308 sqlite `restore`), fixed in S5.1 |
| same seven crates `--lib --tests --keep-going`, `CARGO_BUILD_JOBS=3`, gate v5 (02:13:24–02:16:43, `check-tests-4.txt`; after F3, the procedure test fixes and S5.7) | **exit 0, 0 errors, Finished in 3m18s**. Warnings as proof (lib / lib-test): dag 13 / 30, procedure 8 / 18, sequence 13 / 82, space 4 / 5, home 1 / 2, wires 161 / 259, mathematical 32 / 42. `stdio` was held by S5-TEXT-STDIO from 02:10:36 while this ran; their wave was not red for my crates |
| hub `semio-hub-{dag,sequence,space,imperative,reasoning,mathematical}` `--target wasm32-wasip2 --lib` → COMPOSITION GREEN | OWED |

### S5.3 Open items

P2 laws (math fixture writer, `tool_run_tests::member`, composed reload/child-history laws, D6 one-row law), P3 audit
residue (F3, F18, F7 verify, F6/F21), P4 sequence's 78 anonymous retained refusals, P5 follow-ups — none started yet.

### S5.4 Coordinator actions

None new yet; S4.6 stays in force (central `schema generate` for the graph leaf schemas, describe dag / imperative /
sequence / space / stdio, `test inventory … --subset graph`, the `mutate-semio-graph` differential).

### S5.5 Audit F3 — `delete-node` cascade bound (LANDED 01:57 under the `stdio` lock, lib green)

**Decision (mine, read from code): the refusal code is `mutation.target-referenced`, not `mutation.too-large`.** The audit
and the resume sheet prescribe "diff refuses degree > 1024 with `mutation.too-large`". That cannot be written as a leaf
outcome:

- The outcome vocabulary is frozen at nine codes plus `mutation.apply.<detail>` (`OutcomeCode`,
  `🧫️fixtures/🧫️outcome-code/🔣️.json`). `verify mutation-outcome-law` fails any other code in a `MutationOutcome`, and the
  store refuses to persist it (`validate_persisted_message`: "history carries unknown mutation message code").
- `mutation.too-large` is a store FAULT (`VcsError::TooLarge`, a framework notice), raised when a staged edit's real rows
  exceed the footprint declared from `inverse_rows()` (`STORE` ≈19870 and ≈20049). That generic backstop already exists,
  so the audit's "nothing refuses it" no longer holds for the authoring path; it stays as the second line of defence.
- `mutation.target-referenced` (Error, "the target is still referenced elsewhere in the document, state-dependent") is the
  vocabulary's own code for exactly this refusal. It blocks finalize like every Error (design §16.1) and is labelled
  en/de by the guest time-travel runtime (`TT:3631` "Target still referenced" / "Ziel wird noch referenziert"; React
  `ShellHost:934` maps it too; I found no wgpu-side table, the wgpu shell renders the guest's words). Intended effect, not
  yet run: a replayed `delete-node` whose degree grew past the bound after an upstream history edit reports the Error on
  its own row.

Adding a tenth outcome code would be a framework wave (enum, fixture, schema, TS twin, guest words, both hosts' labels,
like §22.17); it is not needed here.

**The wave** (four files, all under `GRAPH = ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph`,
inside the landing closure):

- `GRAPH/🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs`: new `cascade_edges_maximum(payload)` =
  `MutationLeaf::inverse_rows(payload) - 1`, so the bound is read from the schema's `x-semio-inverse-rows` (1025) and
  never restated; `diff` refuses a larger degree with `MutationOutcome::refuse(OutcomeCode::TargetReferenced, …)` naming
  the node. The `// 🚫️async` line comment became docstrings.
- `…/🗑️delete-node/↩️inverse/🦀️.rs`: empty undo for a refused delete, so the rows never exceed `inverse_rows()` on any base.
- `GRAPH/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: law `delete_node_refuses_one_edge_above_its_declared_cascade_bound`
  (at the bound: applies, undo = declared rows, exact restore; one above: one `mutation.target-referenced` Error naming
  the node, empty diff, empty undo; the aggregate answers the leaf's declared rows).
- `GRAPH/🧪️tests/🌳️mutate-semio-graph/🐍️.py` (independent oracle): `DELETE_NODE_CASCADE_EDGES_MAXIMUM`,
  `severed_edges`, the same refusal in apply and inverse.

Staging and landing: `python3 T/🧪️s5-graphs-f3-stage.py --stage | --land | --revert` (explicit targets, single-occurrence
anchors, `--land` refuses when a target moved since staging). Copies: `🗑️generated/s5-graphs-wires/f3/*.{before,after}.*`.

Language-agnostic law: `python3 T/🧪️s5-graphs-delete-node-cascade-bound.py [--oracle <path>]` reads the bound from the
committed leaf schema and checks the oracle at the bound and one above.

| Command | Result |
|---|---|
| `python3 T/🧪️s5-graphs-delete-node-cascade-bound.py --oracle T/🗑️generated/s5-graphs-wires/f3/oracle.after.py` | 4 passed, 0 failed (declared 1025, maximum 1024) |
| same law on the oracle in the tree, before landing | 2 passed, 2 failed — expected: the law bites (no bound constant, a 1025-edge hub is deleted) |
| `🧪️s4-graphs-python-oracle-rows.py` pointed at the staged oracle | 54 passed, 2 failed — identical to the tree's baseline (the two `patch-snapshot` rows need the real `semio_repo_test.patched_snapshot`) |
| `python3 T/🧪️s5-graphs-f3-stage.py --land` (01:57, `stdio` lock held 01:57:05–02:00:41) | landed 4 targets (none had moved since staging) |
| `python3 T/🧪️s5-graphs-delete-node-cascade-bound.py` on the oracle in the tree, after landing | 4 passed, 0 failed |
| `CARGO_BUILD_JOBS=3 cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-semio --lib --message-format=short` (01:57:17–02:00:31, `check-f3-1.txt`) | exit 0, 0 errors, **228 warnings** (the count before the wave: no new warning), Finished in 3m13s |
| Rust law: `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-stdio-semio --lib delete_node_refuses_one_edge_above_its_declared_cascade_bound` | OWED — the stdio-semio lib-test target has not been compiled this session |
| wasip2: `semio-hub-*` compositions that embed stdio-semio | OWED with the hub batch |

Not in this wave: the gate half of F3 (a bound + 1 property test for every `bounded` leaf, incl. the 55 patch leaves at
128) is S5-GATES'.

### S5.6 Design §22.8 — declared input metadata (DONE in source, gate-verified)

Routing from S5-GATES through the coordinator (01:35). Measured with the gate in the tree at 01:36:
`bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/<plugin>" --inputs`
listed **39 inferred inputs in 17 leaf payload schemas** of my trees (mathematical 25, reasoning 5, dag 3, imperative 3,
space 3; sequence has no inputs). Each now declares `x-semio-ui`: widget, role, en + de label and description, and a
`step` on every interactive number.

- Script (input, kept): `python3 T/🧪️s5-graphs-input-declarations.py --check | --apply` — explicit file table, explicit
  schema-node paths, refuses a missing node or a node that already declares a different control, rewrites a file only
  when it round-trips byte for byte through the 2-space JSON form (16 of 17 do; the wires canvas `🎥️set-camera` schema
  is the one named exception and is normalized to that form).
- Conventions copied from declared siblings, none invented: `Pan X` / `Pan Y` steppers and the log `Zoom` slider
  (`step 0.05`, `softMin 0.1`, `softMax 8`, snaps 0.25–4) of the trinity graph windows for the dag / wires / equation
  cameras; `hidden` + `discriminator` "Position in set" for an undo-owned insert index (`create-node.at` of the shared
  graph vocabulary) on equation `create-node.index` and `connect-nodes.index`; `reference` + `target` + `ref.kind: point`
  for the index `remove-point` addresses (`move-points.indices`); `text` for minted identities and names; `multiline`
  for the two imperative JSON-text config leaves.
- In this report's trees: dag `✏️editor/🎚️config/…/🎥️change-camera` (3); imperative `✏️editor/🎚️config/…/{📸️replace-config,
  🧩️set-contributions, 📤️set-run-output}` (3); space `🧬️mutations/{🌱create-artifact (row id, name), 🏷️rename-artifact
  (newName)}` (3). Mathematical (25) and wires (5) are listed in their own reports.

| Command | Result |
|---|---|
| `python3 T/🧪️s5-graphs-input-declarations.py --check` (before) | would apply 39 declarations in 17 files |
| `… --apply`, then `--check` again | applied 39 in 17 files; second run: 0 to apply, 39 already declared (idempotent) |
| gate `--inputs` per plugin, before → after | mathematical 17 declared + 17 inferred → **32 + 0**; reasoning 5 + 2 → **7 + 0**; dag 2 + 3 → **5 + 0**; imperative 0 + 3 → **3 + 0**; space 7 + 2 → **9 + 0**. (These are the gate's own summary lines. They count fewer inputs than the `--inputs` table has rows — 27 inferred in the summaries against 39 inferred table rows — and I did not trace the difference; the mathematical total moving 34 → 32 is consistent with the two inputs now declared `hidden`, which is my inference.) |
| gate `--json` per plugin, after: residual findings by class | mathematical 3, reasoning 12, dag 17, imperative 4, space 0 — every one is a stale central-catalogue row (34× "malformed … is not JSON" for a leaf schema the §20.15 conversions deleted, 2× `leafUncatalogued` for `move-points` / `set-point-positions`); 0 input findings |
| Rust: the schemas are embedded by `#[derive(dsl::MutationLeaf)]`, so the crates must re-compile | OWED with the next `--lib --tests` batch (foundation red) |

S5-GATES re-measured at 01:47 (through the coordinator): input-gate count 0 for my trees.

Not verified: how the two renderers draw these controls (no serve of these plugins exists this session; config-lane
leaves never appear as history rows by L4, so their declarations only satisfy the gate).

### S5.7 Sequence fault notices — P4 (DONE in source, compiled, gate-verified)

The fault-notice gate (all scopes, `bun …/🧪️test/📜️script.ts schema fault-notices --json`) listed **88 findings** in the
sequence plugin at 01:45: 85 anonymous `Fault::from(text)` refusals (code `app.message`, English only; 75 in the editor
root, 3 in the main-window config, 3 in the script-window transient, 3 in the node-graph command, 1 in the example
command), 2 coded refusals without a notice row (`sequence.retained.{config,example}-command`) and the two-segment
`sequence.child-projection`. The census's "78" is the history-editing scope of the same set.

- **One helper**: `pub(crate) fn sequence_fault(code, detail) -> Fault` in the editor root (`⚠️ Errors` region). Every
  refusal goes through it; the English slug each site carried stays as the fault's detail, the person reads the notice.
- **One table**: `sequence_fault_notices()` — 20 codes, en + de — published through `ArtifactEditor::fault_notices`
  (new on `SequencePlayApp`).
- **Codes group refusals by what the person can do**, not by which internal buffer refused (64 slugs → 20 codes + one
  framework code): `sequence.content.{dialect, unavailable}`, `sequence.window.unavailable`, `sequence.editor.capacity`,
  `sequence.run.{capacity, step-missing}`, `sequence.resume.invalid` (every checkpoint / replay-overrun refusal),
  `sequence.publication.lane`, `sequence.retained.{artifact-command, config-command, example-command, close}`,
  `sequence.node-graph.{malformed, unsupported}`, `sequence.import-media.{missing, undecoded}`,
  `sequence.viewport.camera`, `sequence.action.unhandled`, `sequence.example.unparsable`, `sequence.child.projection`
  (renamed from the two-segment code, editor + viewer). A retained step handed a command of another route (4 slugs)
  raises the framework's `app.command.tool-mismatch`, as the dispatcher at the bottom of the file already did.
- Files (6): `SUB/✏️editor/🦀️.rs`, `SUB/👁️viewer/🦀️.rs`, `SUB/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🦀️.rs`,
  `…/🪟️windows/📜️script/🫧️transient/🦀️.rs`, `SUB/✏️editor/🎮️commands/📚️example/🦀️.rs`, `…/🎮️commands/🕸️node-graph/🦀️.rs`
  (`SUB = ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any`).
- Script (input, kept): `python3 T/🧪️s5-graphs-sequence-fault-notices.py --check | --apply` — six explicit files, every
  literal refusal must be in the slug table, per-file site counts are pinned, the table and the raise sites must agree,
  nothing is written unless all six convert and no `Fault::from(` survives.

| Command | Result |
|---|---|
| `python3 T/🧪️s5-graphs-sequence-fault-notices.py --check` | would apply 88 sites in 6 files, 20 notice rows, 64 slugs mapped |
| `… --apply` (02:13, immediately followed by batch 4) | applied 88 sites in 6 files |
| batch 4 (`check-tests-4.txt`) | sequence lib 0 errors / 13 warnings, lib-test 0 errors / 82 warnings — the counts before the wave |
| fault gate, sequence, before → after (`fault-notices-{before,after}.json`) | **88 → 1**. The one left is `faultNoticeDescriptor` "describe owed": the committed hub descriptor does not publish the 20 codes yet |
| A law that raises a refusal and reads its notice in en / de through the shell | NOT written; the notices are proven declared and syntactically valid by the gate, not rendered |

Not done (same mechanism, outside P4's sequence scope; counts from the same gate run, all scopes): mathematical 46
(44 anonymous, 2 syntax), reasoning 21 (19 anonymous, 1 syntax, 1 descriptor), dag 15 (7 anonymous, 5 missing rows incl.
`dag.node-graph-edit.{malformed,unsupported}`, 2 syntax `dag.unhandled-action` / `dag.child-projection`, 1 descriptor),
imperative 7 (4 anonymous, 3 syntax), space 57 (11 anonymous, 46 missing rows).

### S5.8 Owed at the 17:25 park (exact commands; all need build gate v6 and no activation flag)

1. Tool-run follow-up: `zsh T/🔐️lock.sh acquire landing S5-GRAPHS-WIRES` → `python3 T/🧪️s5-graphs-tool-run-row-label.py --apply label`
   → release → train line; `--apply diagnostic` only together with the rebuild
   `zsh T/🚦️gate.sh 2 18 && CARGO_BUILD_JOBS=4 RUST_MIN_STACK=268435456 cargo test -p semio-framework-plugin --lib --features artifact-app-testing --no-run`,
   then `cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust && RUST_MIN_STACK=268435456 <binary> --test-threads=4 tool_run`.
2. Wave-B tree, my seven crates: fix mathematical `✏️editor/🧪️tests/🔬️unit/🦀️.rs:546` (drop the fourth `preflight`
   argument), then `zsh T/🚦️gate.sh && CARGO_BUILD_JOBS=3 cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-dag-dag -p semio-s-artifact-imperative-procedure -p semio-s-artifact-sequence-sequence -p semio-s-artifact-space-space -p semio-s-artifact-space-home -p semio-s-artifact-reasoning-wires -p semio-s-artifact-mathematical-equation --lib --tests --keep-going --message-format=short`.
3. Hubs on channel 23: `zsh T/🚦️gate.sh && CARGO_BUILD_JOBS=3 cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-dag -p semio-hub-sequence -p semio-hub-space -p semio-hub-imperative -p semio-hub-reasoning -p semio-hub-mathematical --target wasm32-wasip2 --lib --keep-going --message-format=short`
   → one "COMPOSITION GREEN <plugin>" each. None announced yet. `semio-hub-space` had 189 errors at 02:38 (stale
   `semio_framework_plugin::{Label, LocalizedLabel, app_labels}` imports); a peer converted 65 of its files at 04:58, not re-checked.
4. Laws never run: math fixture writer then math / wires lib laws; `documents_reload_identically` and
   `child_history_edits_end_to_end` for dag, sequence, procedure, wires; D6 `a_node_drag_record_is_one_child_transaction`;
   the Rust F3 law `delete_node_refuses_one_edge_above_its_declared_cascade_bound` (stdio-semio lib-test never compiled).
5. Fault notices not named yet: mathematical 46, reasoning 21, dag 15, imperative 7, space 57 (gate run of 01:45).

### S5.9 dag `addNode` faults at publication — every-editor acceptance law (10-06, 02:02 →)

Fault (S5-AGNOSTIC 10-05 19:13, `📓️s5-every-editor-faults.md` dag row): the seed gesture `addNode` of
`child_history_edits_end_to_end` — and the same click in the product — ends in `plugin.internal` "typed-operation emitted a
store lane absent from its exact factory publication contract".

**Cause (read from code, both sides): my inherited §20.15 conversion, not N1.** The typed-operation route holds every
emitted lane against the factory's contract before it publishes (`PLG` ≈32817–32828: `!emit.child_emits.is_empty() &&
!publication_lanes.contains(&Child)` → that fault). Since the conversion all nine graph verbs of the dag editor publish
leaves of the composed `content` child (`dag_child_emit`, `Emit::node_drag_child`) and `DagMutation` is uninhabited, but
`DAG_RETAINED_PUBLICATION_CONTRACTS` still declared `Artifact` for all nine (and a `Config` lane `nodeGraphEdit` never
writes). So not only `addNode`: `removeNode`, `deleteSelection`, `nodeGraphEdit`, `connectMediaPorts`, `disconnect`,
`moveMediaNode`, `renameDagNode` and `patchDagNodes` would each have faulted the same way. sequence declares `Child` for
the same verbs, which is why it passes.

**Wave `dag-child-lane`** — `python3 T/🧪️s5-graphs-dag-child-lane.py --check | --apply | --restore`:

- `…/🕸️dag/…/✳️any/✏️editor/🦀️.rs`: the nine graph verbs declare `[Child]`; docstring says why.
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: new law `every_graph_verb_publishes_on_the_child_lane_and_none_on_the_artifact_lane`
  (no contract names `Artifact`; each of the nine is exactly `[Child]`). The law that EXECUTES `addNode` through the
  typed-operation route already exists and is the one that found this: `composed_child_history_law!("dag", …, addNode)`.
- The lanes are not part of the committed hub descriptor (0 `publicationLanes` in `🌎️hub/🧩️compositions/🕸️dag/🔣️.json`),
  so this change owes no describe.

In the same `landing` hold (coordinator-approved, S5-RUNTIME's region): `python3 T/🧪️s5-graphs-tool-run-row-label.py --apply label`
— the history row of a member tool run keeps the tool's label (`PLG` ≈28586, one match arm).

| Step | Result |
|---|---|
| dry runs against the live tree (02:03) | dag: would apply 11 edits in 2 files; label: 1 edit in 1 file |
| `landing` hold 02:04:20 (≈ 1 s): both applied; two train lines | applied |
| coordinator train | `FRAMEWORK GREEN 02:05:12 through: 02:04:20 S5-GRAPHS-WIRES dag-child-lane` — the plugin lib compiles with the label arm. The dag tree is outside the train's closure. |
| `zsh T/🚦️gate.sh 2 6 && CARGO_BUILD_JOBS=3 cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-dag-dag --lib --message-format=short` | **NOT RUN** — the gate stayed closed for its 15 minutes (`GATE CLOSED 02:19:36 shared-cargo=4 free=7GiB after 900s`, limit 2) and exited 5; no cargo of mine started. The dag lib and its test file are WRITTEN BUT UNVERIFIED by a compiler (the lib edit swaps one enum variant per row, `Child` exists and the import is unchanged) |

Not verified: no law ran (no test build allowed at 6–7 GiB disk). Whether `addNode` now publishes, and whether the
acceptance law has a second fault behind this one, is unknown until the family runs. The new contract law and the label
arm's effect on the member finalize law are equally unrun.

Owed, in order:
1. `zsh T/🧪️s5-agnostic-run-family.sh dag semio-s-artifact-dag-dag` (acceptance family incl. `child_history_edits_end_to_end`).
2. `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-dag-dag --lib every_graph_verb_publishes_on_the_child_lane_and_none_on_the_artifact_lane`
   (same test binary as 1).
3. Shared plugin test binary rebuild, then `tool_run` (expect 41 / 1: panel + finalize green, the cap law still red).
