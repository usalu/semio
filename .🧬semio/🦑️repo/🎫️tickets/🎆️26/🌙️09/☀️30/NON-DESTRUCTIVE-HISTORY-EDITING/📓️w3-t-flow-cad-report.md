# 📓️ W3-T-FLOWCAD Report: Flow and CAD Tool-Machine Conversion

Executor W3-T-FLOWCAD. Scope: the flow plugin (`✏️s/🔌️plugins/🌊️flow`) and the CAD plugin (`✏️s/🔌️plugins/📐️cad`), plus the
hosts that drive their gestures. Brief: `🧭️plan.md` "W3-T brief"; contract: `📋️design.md` §5, §7, §10, §11.

Aliases: `FL` = `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`CA` = `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `RE` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements`.

## 1. Census (before this work)

### 1.1 Flow

Document model: `FlowSnapshot` holds only the coordinate of a composed `s.stdio.semio@v1/flow` CONTENT child; widgets,
synapses and layout live in that child. Every live flow edit publishes on the CHILD lane as ONE whole-content
`SemioFlowMutation::SetSnapshot` (`flow_scene_publication`), even where the app first folds its own `FlowMutation`s
(`flow_content_edit`). The parent `FlowMutation` vocabulary (incl. absolute `move-widgets`) is used as fold vocabulary and
by the bounded retained artifact recipe (`🧵️retained/🗿️artifact`, supports delete/disconnect/move/replace only).

| # | Gesture | Host → verb | Commit today | Cancel today |
|---|---|---|---|---|
| F1 | node drag, React flow canvas (`RE/🕸️NodeGraph` + wasm `OS/🌊️flow/🖥️host`) | host-local drag; at release `hostSnapshotChanged` → `nodeGraphEdit {setHostSnapshot: <whole fixture JSON>}` | guest adopts the fixture → child `SetSnapshot` (whole content, absolute) | `pointerCancelScreen` restores the gesture baseline host-locally: zero trace |
| F2 | node drag, wgpu (`RE/⚙️EngineCanvas` bounded plan path) | `plan_graph_edits` → `nodeGraphEdit {move: {nodeId, x, y}} × N` (absolute, one row per node, one dispatch) | `host.move_widget` per row → child `SetSnapshot` | host-local, zero trace |
| F3 | `moveMediaNode {nodeId, x, y}` (palette / MCP; no host sends it) | one verb per tick | child `SetSnapshot`, `coalesce_key: move-<id>` per-tick amend (pattern B) | none |
| F4 | wire connect / reconnect / cut (both hosts) | `nodeGraphEdit {connect|disconnect}` (narrow intent rows) | `host.connect_ports` / `host.disconnect` → child `SetSnapshot` | host-local |
| F5 | catalogue drop / spotlight add | `addWidget` / `spotlightCommit` | child `InsertNode`/`InsertEdge` or `SetSnapshot` | – |
| F6 | inspector field edits | `patchFlowWidgets` | child `SetSnapshot`, `coalesce_key: patch-<field>-<ids>` amend | – |
| F7 | delete / disconnect verbs | `deleteSelection`, `removeWidget`, `disconnect` | `FlowMutation`s folded, published as child `SetSnapshot` | – |
| F8 | camera pan / zoom | `nodeGraphViewport` | window config | – |

No flow gesture is a state machine and none carries a transaction; there is no widget resize gesture (`WidgetLayout` is
`{x, y}` only). The `nodeGraphEdit` row vocabulary (`move` absolute) is shared by 12 guests (dag, sequence, procedural
2d/3d, mathematical, wfc 2d/3d, space, trinity, architect, flow, …) and by both hosts.

### 1.2 CAD

| # | Gesture | Host → verb | Commit today | Cancel today |
|---|---|---|---|---|
| C1 | gumball translate / rotate / scale (React `World3dHost`) | `transformBegin` (HostOnly), ONE delta `translateSelection {objectIds?, dx, dy, dz}` / `rotateSelection {ax, ay, az, angle}` / `scaleSelection {sx, sy, sz}`, `transformEnd` | `Emit::mutations` of ABSOLUTE `move-objects` / `rotate-objects` / `scale-objects` (one per touched pane, per-object final origin / quaternion / scale) | host-local (Escape / pointercancel drop the drag) |
| C2 | 60 declarative interactions (`CA/📚️examples/🖼️assets/🏗️modelDefinitions/*/🕹️interactions/*.json`, `spatial.interaction`) | engagement REPL: `engagementInput`, `engagementSubmit`, `engagementPossibleSelect`, `engagementRepeatLast`, `engagementAbort`, `worldPointerDown`, `worldPointerMove` | every step republishes the whole `CadEngagementScratch` as a CONFIG snapshot amended under `coalesce_key: "engagement"` (config history); the commit step runs `interaction::commit_session` → `CommitOutcome::{Objects, Move, Copy, Rotate, Scale, Unsupported}` → `create-object` × N or the ABSOLUTE C1 leaves, `Emit::mutations` + config | `engagementAbort` clears the session (a config edit) |
| C3 | inspector pose edits | `patchObject` / `patchSelection` (`origin.x`, `scale.y`, `orientation.w`, … `value` or `delta`) | absolute `move-objects` / `scale-objects` / `rotate-objects`; whole-value fields as `delete-object` + `create-object` | – |
| C4 | add / duplicate / delete object | `addObject`, `duplicateObject`, `deleteObject` | `create-object` / `delete-object` | – |
| C5 | reference overlay edits | `patchCadPlayReference`, `setReferenceHidden/Locked` | absolute `move-reference`, `change-reference-*` | – |

The Rust interaction runtime (`CA/✏️editor/⚙️engine/🕹️interaction/🦀️.rs`) is a hand-rolled interpreter
(`apply_event_generic`) of the JSON statecharts; the TS twin (`@semio-tech/cad-js`, `CA/✏️editor/⚙️engine/🎰️stately`) compiles
the same specs onto `@semio-tech/machine`. The TS engine is consumed by stories, TS extension modules and tests only — the
live React and wgpu hosts run the Rust guest. `openTransaction`/`commitTransaction` effects are declared in the spec type
and never interpreted.

## 2. CAD (done)

### 2.1 Design
- **Relative parametric leaves, one per touched pane** (pane-scoped so the exact inverse stays ONE absolute setter row,
  the CAD one-item preparation contract): `drag-selection {pane, targets, offset:[x,y,z]}`, `rotate-selection {pane,
  targets, axis:[x,y,z], angle}` (each object turned in place about its own origin, the semantics the gumball and the
  `transform.rotate` interaction already had), `scale-selection {pane, targets, factors:[x,y,z]}`. Diffs read the BASE
  pose (`cad_selection_diff`), so an edited leaf replays on any base. Outcomes from the frozen vocabulary: empty or
  repeated targets / non-finite numbers / zero axis / non-positive factor → Fatal `mutation.invariant` (`axis-nonzero`
  declared as `x-semio-invariant`); no target in the pane → Error `mutation.target-missing`; some missing → Warning
  `mutation.partial`; identity → Warning `mutation.no-op`. Inverse: one absolute `move-objects` / `rotate-objects` /
  `scale-objects` with the base values (exact, never a negated delta). Labels en/de: "Drag 2 objects by (1.5, -2, 0.5)"
  / "2 Objekte um (1,5; -2; 0,5) ziehen", "Rotate 1 object by 90°", "Scale 2 objects by (2, 2, 0.5)".
- The absolute `move-/rotate-/scale-objects` stay: they are the exact inverses and the inspector's absolute field sets
  (`origin.x = 5`), not gesture ops. Gesture-only absolute paths are deleted: `translate_objects_mutations`,
  `rotate_objects_mutations`, `scale_objects_mutations`, the editor's private `quaternion_product`.
- **Transform tool** `CA/✏️editor/🎭️modes/✏️edit/🛠️tools/🧭️transform/🦀️.rs`: `machine::statechart! cad_transform_tool`
  (`idle` —Records[entries_apply]→ `idle` do `yield_entries`), effects `ToolYield<CadMutation>`, driven by
  `ToolMachineRunner`, tool id `s.cad.cad@1/*#editor#<verb>`, ref minted from the admission's `authoring_seed`.
  Entries: `Transform(CadTransformRecord)` (drag/rotate/scale over literal deduplicated targets → one leaf per pane that
  owns a target) or `Leaf(CadMutation)` (a construction's / copy's `create-object`). Yields are folded on a running state;
  a no-op/Error/Fatal leaf is never yielded, so a stranger id or an identity motion leaves zero trace (no edit, no
  transaction). `cad_transform_tool_emit` publishes ONE edit via `Emit::commit_transaction` (plain leaves for a seedless
  view).
- **Entry points routed through the tool**: `translateSelection` / `rotateSelection` / `scaleSelection` (gumball, palette,
  MCP), `patchObject` / `patchSelection` origin DELTAS (`origin.x` with `delta`), and every interaction commit
  (`engagementSubmit`, `engagementPossibleSelect`, `worldPointerDown` → `try_commit_session_entries` →
  `engagement_emit`; tool id = the interaction id, e.g. `…#transform.move`). Absolute inspector values stay absolute.
- Hosts: the CAD gumball host already sends ONE delta at release (host-local preview, Escape/pointercancel drop with no
  dispatch) — that is the one-shot phase; no host change was needed. A streamed gumball (stream/commit/abort phases)
  would need a CAD window transient (CAD has `NoTransient`) — not built (open item).

### 2.2 Files (CAD)
- New leaves: `CA/🧬️schema/🧬️mutations/{✋️drag-selection,🔄️rotate-selection,🔍️scale-selection}/{🦀️.rs,🔺️diff/🦀️.rs,↩️inverse/🦀️.rs,🔣️.json,🧬️schema/🔣️.json,🧪️tests/<case>/🦀️.rs}`;
  wire witnesses `CA/🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json`.
- Aggregate: `CA/🧬️schema/🧬️mutations/🦀️.rs` (variants, KINDS, `🔖️SelectionTransform` region), `…/🔣️.json`, `…/🟦️.ts`,
  `…/💾️binary/📡️.protocol.semio` (tags 24–26), `…/🧪️tests/🔬️unit/🦀️.rs` (every_mutation + 8 wire witnesses),
  `CA/🔮️oracles/🔣️.json` (deferredKinds + catalog), `🗿️artifacts/📐️cad/🦀️.rs` (modules), `📦️packages/🦀️rust/Cargo.toml`
  (`machine`, `semio-framework-tool-machine`).
- Tool: `CA/✏️editor/🎭️modes/✏️edit/🛠️tools/🧭️transform/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`.
- Callers: `CA/✏️editor/🦀️.rs`, `…/🎮️commands/{🔄️transform,🤝️engagement,🧱️object}/🦀️.rs`, `…/🧪️tests/🔬️unit/🦀️.rs`.

### 2.3 Verification (CAD)
| command | result |
|---|---|
| `cargo check -p semio-s-artifact-cad-cad --tests` | ok (my warnings fixed; remaining warnings are pre-existing) |
| `cargo test -p semio-s-artifact-cad-cad --lib` (private target) | **459 passed, 2 failed** — the 2 known pre-existing (`demo_asset_is_the_concrete_forest_and_every_pane_resolves`, `every_example_load_is_admitted_and_settles_through_the_host_document_archive_door`); all new leaf laws (15), transform-tool laws (7), `semio_payload_law_cad_mutation`, wire witnesses, kinds catalog, engagement and translate laws pass |
| `schema mutation-payloads --under ✏️s/🔌️plugins/📐️cad` | 28/28 witnessed, 0 findings |
| `schema mutation-inputs --under ✏️s/🔌️plugins/📐️cad` | 3 `leafUncatalogued` (the three new leaves wait for the central `schema generate`), 0 other findings |

## Session 2 — 2026-10-01

Successor S2-FLOWCAD (coordinator `⚪552b484a…`). Ownership (coordinator 12:05): `PL/🌊️flow/**` (incl. extensions), `OS/🌊️flow/**`,
`RE/🕸️NodeGraph/**`, `RE/⚙️EngineCanvas/**`, the `🌊️flow` subset dirs of stdio semio, `PL/📐️cad/**`; §12 regions of `OS/🔌️plugin`
(owner S2-W2A) as region-scoped edits. Scratch: `🗑️generated/s2-flowcad/`.

### S2.1 Published contracts (for S2-GRAPHS, S2-W2A, S2-W2B, S2-W2C)

**Node-graph gesture record (design §13.3), as landed.** One `nodeGraphEdit` row per released node drag, dispatched ONCE at
release (never per pointer sample), by both hosts:

```
{ "operation": "move", "gestureId": string (non-empty), "nodeIds": string[] (≥ 1, unique), "dx": finite number, "dy": finite number }
```

- The row is closed (exactly these five fields). `dx`/`dy` are the ONE relative offset every listed node moved by, measured from
  where the press started. A click (zero offset) dispatches nothing. The absolute `{nodeId, x, y}` row no longer exists.
  §13.3's `kind: move` is spelled `operation: "move"`, the discriminator every `nodeGraphEdit` row already carries.
- Rust: `semio_framework_tool_machine::{NodeDragRecord, NODE_DRAG_OPERATION, NODE_DRAG_ROW_FIELDS, node_drag_commit}`
  (`FW/🛠️tool-machine/🦀️.rs` region `🔖️NodeDrag`). `NodeDragRecord::from_row` decodes and refuses by name, `to_row` encodes,
  `moves()` is the click filter. `node_drag_commit(tool, actor, gesture, leaves, clock) -> Option<(TransactionRef, Vec<M>)>` is
  the ONE node-drag machine: a one-shot press of the `Scrub` statechart. Empty leaves leave zero trace.
- TS twin: `nodeDragRecordFromRow`, `nodeDragRow`, `nodeDragMoves`, `nodeDragCommit` (`FW/🛠️tool-machine/🟦️.ts`). Fixture:
  `FW/🛠️tool-machine/🧫️fixtures/🧫️node-drag-law/🔣️.json` (3 valid / 7 invalid rows plus one commit vector). Schema:
  `$defs/NodeDragRow` in `FW/🛠️tool-machine/🧬️schema/🔣️.json`.
- Hosts: React `RE/🕸️NodeGraph/🟦️.tsx` `onNodeDragStop` → `nodeDragRow(record)` when `nodeDragMoves(record)`. wgpu
  `RE/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` `DagGraphEdit::Move { gesture_id, node_ids, dx, dy }` → the same row. The
  language-agnostic corpus `RE/🧫️fixtures/🫳️node-graph-gestures/🔣️.json` covers both hosts (rules `aReleasedDragPublishesMove`,
  `aClickIsNotAMove`, `aGestureBelongsToThePathThatStartedIt`).
- Guest recipe (flow reference: `PL/🌊️flow/…/✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🦀️.rs`): decode the row with
  `NodeDragRecord::from_row`. Map each moving record to the guest's own relative leaf over the record's nodes the document holds
  (flow: `drag-nodes {targets, dx, dy}`; GRAPHS: `move-nodes {ids, dx, dy}`). Commit through `node_drag_commit` with tool
  `<appId>#<verb>`, the admission's `authoring_seed` as actor and the press = the first record's `gestureId`. Publish with
  `Emit::commit_transaction` (own store) or `Emit::commit_child_transaction` (composed child, see below). A view without an
  authoring seed publishes the leaves plainly.

**Composed-child transactions (design §12), as landed in source.**

- `Emit::commit_child_transaction(transaction: TransactionRef, child_emits: Vec<ChildEmit>) -> Emit` (`OS/🔌️plugin/🦀️.rs`, `Emit`
  impl): children with no ops are dropped, and nothing touched is the empty emit. Parent ops may ride the same emit
  (`Emit { artifact_mutations, ..Emit::commit_child_transaction(tx, children) }`). `dispatch_emit_group(…, transaction)` passes
  the ref in `store::GroupMeta.transaction`. `CompositionCoordinator::dispatch_group` stamps it on every member's `Apply`
  (parent and every owned child lane), and so does the retained path `PendingChildGroupPublication.transaction`.
- `tool_transaction_shape_fault(verb, transaction, coalesced)` refuses ONLY a transaction riding a coalesced amend
  (`toolTransaction.shape`). A child the instance does not hold is refused by `dispatch_emit_group` before anything lands.
- Wire: `HistoryMutationEntry.store?: string` (`FW/🎠️kernel/🦀️.rs` + `🟦️.ts` + `🧬️schema/🔣️history-patch` pattern `^[^/]+/.+$` +
  fixture row `content/flow-content-1`) is the member store id `<slot>/<childId>`. It is absent for the app's own document
  store. A parent history row lists its own mutations first, then every owned-child mutation of the same gesture
  (`CommandLogEntry.child_edit_ids`), labelled from the child's own leaves. A reload backfill groups unlogged member edits by
  `TransactionRef`: edits whose transaction also landed a parent edit join that parent's row, the rest form one row per
  transaction (`⏪️time-travel/🦀️.rs` `member_backfill`).
- Verb: `historyEditBegin { mutationId, store? }` (`FW/🛂️manifest` `HISTORY_EDIT_ARG_STORE = "store"`, hidden arg). It is refused
  `busy` while a session is open on another store, and `unknownMutation` for a store the instance does not compose. Hosts
  must pass `store` from the row's `HistoryMutationEntry.store`; the Rust history panel already does
  (`args.push(HISTORY_EDIT_ARG_STORE, …)`).
- Runtime: `TimeTravelLedger.member: Option<TimeTravelMemberSubject>`. Every store read and command (`state_before`,
  `begin_report_replay`, replay stepping, `commit_finished_replay`, base/positions) runs on the member through
  `store::MemberStoreVisitor{,Mut}` (`TimeTravelStoreState<P, Mu>` is the typed owner per store). Render seams read
  `render_children_or(live)`, the live children with the member's preview or replayed head in its place. The parent snapshot
  is untouched. After finalize, `publish_time_travel_member` re-derives the parent: it republishes the member in the immutable
  children root, sends the member's `Supersede` transition on the member lane of the parent's backbone, and logs one history
  row. `pending` rows are the downstream mutations of the edited store only.

Status of both contracts at the time of writing: **WRITTEN, compile pending** (see S2.2).

### S2.2 Repair (rule 21)

- No half-written syntax in my trees. Every §12 symbol the 07:52 `⏪️time-travel` edit references exists in the store:
  `MemberStoreVisitor{,Mut}`, `visit_member{,_mut}`, `snapshot_read_erased_of`, `send_member_mutations`,
  `ChildContentView::with_member_read`.
- Environment repair: the shared cargo build-dir was poisoned by a scratch clone of the repo. 22 debug dep-info files pointed
  into `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🗑️generated/goal-ts/workspace-membership/physical-without-both/`, so cargo judged
  stale units fresh, among them semio-framework, replication, io-base64, job, ui and schema. Symptom:
  `semio-framework-replication` failed on base64 symbols that exist in the real tree. I deleted those 22 dep-info files (list in
  `🗑️generated/s2-flowcad/poisoned-dep-info.txt`) and reported it to the coordinator, who installed `T/🧼️fingerprint-guard.sh`.
- Compile attempts so far, each blocked by peers' in-flight edits outside my trees:
  `plugin-check-1/2` (poisoned base64), `plugin-check-3` (peer: `crate::os_vcs` at `OS/🔌️plugin/🦀️.rs:37270`, plus a missing
  `🛰️declaration-channels` test fixture), `plugin-check-4` (peer: store `protocol::Edit.line` mid-rollout, 4 sites + 1 pattern).

### S2.3 Owned-child scrub seam (§13.1 × §12) and flow F6 — WRITTEN, plugin lib compiles, laws not yet run

**Runtime seam** (`OS/🔌️plugin/🛠️tool-machine/🦀️.rs`; regions touched: module doc, `🔖️Runtime` struct + `Default` + accessor,
`🔖️Driver` `admit_tool_dispatch` abort branch, `retire_tool_windows`, `freeze_tool_machines`, `settle_tool_operation` Scrub branch):

- `ToolMachineRuntime` gains `child_scrubs: ScrubLedger<ChildEmit>` (accessor `child_scrubs()`) beside `scrubs: ScrubLedger<M>`.
  `ScrubLedger` and `ScrubMachine` are unchanged and generic: no new machine, and still no plugin-local scrub machine.
- On every tagged press dispatch (`gesture`, `commit`), `settle_tool_operation` hands the emit's `child_emits` to the
  window's child press, next to its own `artifact_mutations`. Ticks publish nothing on either lane, and the history logs
  nothing. The release publishes both lanes as ONE emit with `emit.transaction` set. The own ledger's ref wins when both
  lanes commit, and `GroupMeta.transaction` stamps it on every member, so parent and owned children carry ONE
  `TransactionRef`. A release that commits only child leaves stays logged as one row.
- Host aborts (`abort`, `retired` windows, time-travel freeze) drop the child press with zero trace, exactly like the own press.
- Child provisional leaves are NOT overlaid on renders, because the overlay folds the parent document only. During a press,
  the host-local control shows the dragged value (the NodeGraph inline slider lane does). An overlay of children
  (`render_children_or`-style) is an open item for S2-W2A if a second window must preview a child press.
- `cargo check -p semio-framework-plugin --lib` (12:40–12:46): 0 errors, 133 warnings (none in this file). The lib test
  target did not build because of the peer-deleted `🛰️declaration-channels` fixtures (routed to S2-INFRA).

**Flow F6** (my tree):

- `🎮️commands/🩹️patch-flow-widgets/🦀️.rs`: `patchFlowWidgets` now lands ABSOLUTE `set-node-param {id, key, value}` leaves
  (slider `value`, note `text`) on the content child: `widget_field_leaf`, `patch_flow_widgets_leaves`, `widget_leaves_emit`.
  It never publishes a whole-content `set-snapshot` and never coalesces. A value the widget already holds is no edit (empty
  emit, zero trace) instead of the old `flow.patch-widgets-unchanged` refusal, so a knob dragged back to its start leaves
  nothing. The retained route (`FlowDirectStoreWork`, `✏️editor/🦀️.rs`) completes through the same helpers. Its scan
  scratch retires through `retained::Owner::Mutations`.
- `🎮️commands/✏️node-graph-edit/🦀️.rs`: new row `{operation:"setSlider", widgetId, value}` (the row the NodeGraph inline
  slider lane already dispatched, which flow refused until now) maps to the same absolute leaf
  (`node_graph_edit_slider_leaves`). The closed argument root admits the framework scrub keys `gesture`, `commit` and
  `abort` (`NODE_GRAPH_EDIT_ROOT_FIELDS`). A dragged knob is ONE child transaction through the seam above.
- Static keys deleted: `duplicateWidget:<id>` (the duplicate already lands intent leaves `insert-node` + `insert-edge`, now
  one plain child edit per command), `generation-values` (that emit carries only the window transient, so the key was
  inert), and `patch-<field>-<ids>` (both routes).
- Laws added (`✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs`, region `🔖️ComposedChildHistory`):
  `a_dragged_inline_slider_is_one_child_transaction_and_a_cancel_leaves_zero_trace` and
  `patching_a_widget_value_is_one_absolute_node_param_leaf`. Both are WRITTEN BUT UNVERIFIED: the ✏️s workspace does not
  compile because of the peer's `DslValue::Bytes` non-exhaustive match in `♾️infinite/🗿️artifacts/🕸️dag` (routed to S2-INFRA).
- Collateral repair in my tree: REPO-PATH-BUDGET's text rewrite had changed the flow manifest `[[test]]
  set-contributions-registry` path to a nonexistent `🧩️set/` directory. I restored `🧩️set-contributions/`.
