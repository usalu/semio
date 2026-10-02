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
- `tool_transaction_shape_fault` (`OS/🔌️plugin/🦀️.rs`; since S2-W1G's §15 refinement its signature is
  `(verb, transaction, phase, coalesced, children, carried, foreign)`) refuses a transaction riding a coalesced amend. A
  COMMITTED transaction across owned children is legal (§12). Only a §15 *streamed* or *aborted* one across children is
  refused, because a streamed transaction grows the app's own document. A child the instance does not hold is refused by
  `dispatch_emit_group` before anything lands.
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

### S2.4 Milestone 16:35 (resume after the 13:30 usage cut)

**§12 framework laws (new).** All four pieces are WRITTEN; the Rust law is UNVERIFIED and the TS oracle is VERIFIED.
- Fixture `OS/🔌️plugin/🧫️fixtures/🧫️composed-child-history/🔣️.json` (4 cases) with its schema `…/🧬️schema/🔣️.json`
  (draft 2020-12). The cases cover:
  - a gesture on parent + child joins the parent row;
  - one transaction across two members is one row (`(+N)` label);
  - edits without a transaction are rows of their own, in moment order, labelled by description, leaf or op line;
  - edits that rows already carry are never backfilled.
- Rust law `🧪️tests/🧪️composed-child-history/🦀️.rs` (`member_backfill_answers_every_fixture_case`) runs every case through
  `time_travel::member_backfill`. It is mounted next to `time_travel_tests` in `mod app` (`OS/🔌️plugin/🦀️.rs`, one
  `#[cfg(test)] mod` line). It is UNVERIFIED because the plugin lib-test target did not build (the peer-deleted
  `🛰️declaration-channels` fixtures).
- TS twin `🧪️tests/🧪️composed-child-history/🟦️.ts` (`memberBackfill`, `composedChildHistoryOracle`) is an independent
  derivation, with Ajv 2020 strict validation and 2 hostile rows. It is wired into the plugin `📦️packages/🦀️rust/📜️script.ts`
  test task. Run `bun 🗑️generated/s2-flowcad/run-composed-child-history-oracle.ts`: `composed-child-history-oracle cases=4`
  (PASS).

**Flow §12 end-to-end laws** (the predecessor's 4, plus my 2 F6 laws) live in `✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs`.
They are WRITTEN BUT UNVERIFIED: `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow --lib --tests`
was killed by the usage cut after 32 minutes. Its dependency `semio-framework-artifact-flow-flow` lib compiled (2 warnings).

**`DslValue::Bytes` in my tree.** `OS/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs` gained 2 arms:
- `property_value_from_dsl` maps `Bytes` to `PropertyValue::Array` of byte numbers, which is the JSON projection.
- `preview_tree_collapsed_summary` maps `Bytes` to `[N bytes]`.
That crate's lib compiled with these arms.

**Node-graph TS twins** (`RE/🧑‍🎨engine/🧪️tests/{🫳️node-graph-gestures,🔗️node-graph-wire-edit}/🟦️.ts`):
- I repointed the vocabulary source scans from the procedural generation3d guest, whose code shape had moved, to the
  reference flow guest's explicit closed row declarations (`action_row_fields(row, operation, &[…])`).
- The pointer-capture scan now anchors on the flow host's current `issueFlowGestureStep(session.pointer{Down,Up}Screen(`
  call.
- Result: vitest `--config 🔍️w3-t-flowcad-node-graph.config.ts` passes **22/22** (it was 19/22, and 21/22 for the
  predecessor).

**REPO-PATH-BUDGET collateral in my trees.** Its name-keyed text rewrite had changed references to directories it never
moved. I moved the directories to the names the rewritten references use (plain `mv`, no git):
- flow `…/🌊️main/🎚️config/🧪️tests/🔬️window-ownership` → `🔬️window`;
- cad `…/🪟️windows/🎚️config/🧪️tests/🔬️window-ownership` → `🔬️window`;
- cad `🧬️schema/🧪️tests/🪪️document-contract` → `🪪️document`;
- cad `🧬️schema/🧫️fixtures/🪪️document-contract` → `🪪️document`.
A scan of every `#[path]`/`include_str!` in my trees now finds 0 dangling references, apart from my own new test file
listed in the CAD section below.

**CAD §17.4, part 1: streamed gumball.** This needs no plugin-local machine. A host streams
`translateSelection|rotateSelection|scaleSelection {…cumulative offset from the press…, gesture, commit?|abort?}`, and the
framework scrub runtime does the rest:
- every tick stays provisional, and the render overlay folds the press's `drag-selection` leaf on the committed document;
- the release lands ONE edit of the release's cumulative offset;
- a host abort leaves zero trace.

To make that hold, the press now owns the transaction: `settle_tool_operation` clears a ref the verb minted itself
(`cad_transform_tool_emit`) before the press decides, so a tick never logs a row. The edit is in the runtime seam region
listed in S2.3. Law `a_streamed_gumball_press_is_one_transaction_and_a_cancel_leaves_zero_trace` (transform tool tests) is
WRITTEN BUT UNVERIFIED.

**Host request for SPATIAL** (`RE/🌐️World3dHost`): during a gumball drag, dispatch the CAD transform verb on every
pointer sample with the cumulative delta, the press id as `gesture`, and `commit: true` on release. On
Escape/pointercancel/blur, dispatch `abort: "<reason>"` with no delta. Today the host sends ONE delta at release, which
stays correct as a one-shot press.

**CAD §17.4, part 2: `CadEngagementScratch` moves to a window transient.** IN PROGRESS. Landed so far (additive):
- `CA/✏️editor/🎭️modes/✏️edit/🪟️windows/🫧️transient/` with `🦀️.rs` and the schema-first `🧬️schema/🔣️.json`.
  `CadWorldWindowTransient {engagementInput, engagementStep, engagementPane, engagementSessionJson,
  lastFinalizedInteractionId}` comes with its `Snapshot` mutation, JSON pack/DSL, retirement, an ephemeral-transfer
  preflight, the owner macro, `register`, `from_snapshot`/`current`/`addressed`, and laws in `🧪️tests/🔬️unit/🦀️.rs`.
- One owner per pane window (`📐️shape|🏢️building|🔥️energy|🏛️structure-classic/🫧️transient/🦀️.rs`, replacing
  `📌️.empty.md`), mounted in the crate root and in each window module.

Still to do:
1. Delete the engagement and preview-stamp fields from `CadConfig`, together with its JSON, TS, graphql and proto schema
   files.
2. Replace `preview_transition_snapshot_of` and the `engagement` config amend with window-transient publication through a
   CAD retained work that answers `CompleteWithEphemeral`.
3. Render and HUD read the transient.
4. Update the tests.

### S2.5 CAD §17.4 part 2 — `CadEngagementScratch` lives in the world-window transient (source complete 17:10)

- **Config surfaces.** `CadConfig` no longer carries `engagementInput`, `engagementStep`, `engagementPane`,
  `engagementSessionJson`, `engagementPreviewOperationJson`, `engagementPreviewGeneration` or `lastFinalizedInteractionId`.
  They are gone from every surface, with no compat:
  - Rust `🎚️config/🦀️.rs`, plus `CAD_PREVIEW_GENERATION_MAX` and `deserialize_cad_preview_generation`;
  - the schema mirror `🧬️schema/🦀️.rs`, `🔣️.json`, `🟦️.ts` (parser), `🔗️.graphql` and `🛰️.proto` (fields renumbered 1–6).
- **Runtime boundary.** `cad_runtime_from(cfg, transient)` reads the engagement from the window transient;
  `cad_config_from_runtime` writes config fields only; the new `cad_transient_from_runtime` writes the engagement.
  `runtime_of(cfg, transient)` is the one boundary.
- **Publication.** `publish_engagement(runtime, ctx)` hands a changed engagement to `CadDispatchCtx.next_window_transient`.
  `CadDispatchCtx` is now `{interaction, view_state, window_transient, next_window_transient}`, built with
  `CadDispatchCtx::new`.
- **Deleted.** The operation-stamped config checkpoint machinery: `preview_transition_snapshot_of`,
  `CadPreviewOperationIdentity`, `CadPreviewStamp`, `CadGesturePreview`, `CadPlayApp::gesture_preview`,
  `CAD_ENGAGEMENT_COALESCE_KEY` and `engagement_amend` (the `Emit::amend_config` under `"engagement"`).
- **Retained route.** The ONE `CadRetainedCommandWork` replaces `BoundedArtifactCommandWork` for every CAD tool. It reads
  `context.window_transient` and completes with `CompleteWithEphemeral { window_transient: [addressed] }` when the
  engagement changed. A dispatch that addresses no window (an agent, the palette) keeps none.
  `CAD_RETAINED_ENGAGEMENT_TOOL_IDS` (`engagementInput`, `engagementRepeatLast`, `engagementAbort`, `worldPointerMove`)
  dispatch without config admission. Publication contracts:

  | Tools | Lanes |
  |---|---|
  | `engagementSubmit`, `engagementPossibleSelect`, `worldPointerDown` | Artifact + WindowTransient |
  | `engagementInput`, `engagementRepeatLast`, `engagementAbort`, `worldPointerMove` | WindowTransient |
  | `setActiveExample` | Config + WindowTransient |
  | `importCadFile` | WindowTransient |

  The fixture `🧫️fixtures/🗄️retained-jobs/🔣️.json` `intendedLanes` was updated to match.
- **Registration and reads.** `register_window_transient_owners` registers the 4 pane owners. Render, window engagements
  (per window) and measures read the transient through `TransientView`.
- **Commands.** The engagement commands (`🤝️engagement/🦀️.rs`) publish only the transient, plus ONE transform-tool
  transaction on commit. `importCadFile` and `setActiveExample` reset the addressed window's engagement. `addNode`,
  `setNodeSelection`, `setReferenceSelection`, `referenceHover` and the exports read the runtime through the transient.
- **Tests (CAD unit).** The harness gained `drive_result` and `drive_engagement(app, scene, action, args, config, transient)
  -> (emit, next transient)`. The engagement region was rewritten:
  - `engagement_starts_box_interaction_session`;
  - `world_pointer_move_updates_live_preview_without_committing_or_emitting_mutations`;
  - new `every_engagement_step_is_window_transient_state_never_config`;
  - `engagement_repeat_last_restarts_the_last_finalized_interaction`;
  - `engagement_steps_are_window_state_until_the_one_committing_edit`, which replaces the coalescing law.
  The 7 config-checkpoint preview-stamp laws are deleted together with the mechanism they guarded.
- **Transient contract (schema-first, language-agnostic).**
  - Fixture `🫧️transient/🧬️schema/🧫️fixtures/🪪️document/🔣️.json` (2 valid, 4 invalid).
  - TS twin `🧬️schema/🟦️.ts` (`parseCadWorldWindowTransient`, `applyCadWorldWindowTransientMutation`).
  - Ajv contract `🧬️schema/🧪️tests/🪪️document/🟦️.ts` is wired into the CAD package `verify cad-document-contract`.
    `bun 🗑️generated/s2-flowcad/run-cad-window-transient-contract.ts` → `cad-window-transient-contract rows=6` (PASS).
  - The Rust laws in `🧪️tests/🔬️unit/🦀️.rs` read the same fixture.
- Needs a coordinator `describe` for cad: the config schema and publication lanes changed.

### S2.6 §12 design notes (how the landed slice meets the brief)

- **Downstream parent mutations.** A member-store edit replays the member's own downstream suffix (Report mode), and its
  per-mutation outcomes reach the history rows of that store (`pending` and outcome flags are matched on
  `HistoryMutationEntry.store`). A parent's own mutations fold over the parent snapshot alone. The composing parent holds
  only the child coordinate (flow: `FlowSnapshot.content`), so no parent mutation reads child content during its diff. A
  parent mutation that *depended* on the child would therefore be a cross-store dependency, and the store model has no
  such edge: its outcome is unchanged by the member replay, and its durable row keeps its own outcome.
- **Stale parent never re-mints the live child.** Finalize on a member runs `commit_finished_replay` on the member store
  only. The parent store generation does not move, so `follow_derivable_children` (the only path that re-mints a
  derivable child from the parent's cached working scene, or retires one) does not run. `publish_time_travel_member`
  re-derives the parent's view from the replayed member (it republishes the immutable children root), so every render seam
  and the next verb read the replayed child. The flow law `editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows`
  pins this end to end: the content child id is unchanged, the member holds the edited offset, and the main window
  renders it.
- **Lints for my scope** (`schema mutation-inputs|mutation-payloads --under …`, cwd `🧪️test`), all **0 findings**:

  | Scope | `mutation-inputs` | `mutation-payloads` |
  |---|---|---|
  | `PL/📐️cad` | 61/61 inputs | 28/28 witnessed |
  | `PL/🌊️flow` | 27/27 | 30/30 |
  | stdio semio `🌊️flow` subset | 24/24 | 15/15 |

  The three CAD leaves that were `leafUncatalogued` are now catalogued.
- **TS typecheck** (`tsc --noEmit --strict`) of the new twins: 0 errors. Files: the CAD transient `🟦️.ts` and its contract,
  and the plugin `composed-child-history` oracle.
- **Taxonomy** for my new directories: see S2.7 (S2-TAX's scoped report of 22:35 covers the CAD and flow subsets).

### S2.7 Status 02:45 (resume after the 23:00 cut; fleet rule 26 CARGO HOLD)

**Rust verification is still pending: the CAD, flow and plugin lib tests are WRITTEN BUT UNVERIFIED.** No compile has
reached my crates since the CAD §17.4 migration. The tool output for these attempts is in `🗑️generated/s2-flowcad/`:

| Attempt | Outcome | Owner |
|---|---|---|
| `cad-check-4` (21:37–21:59) | killed (137) while still in dependencies | environment |
| `cad-check-5` (22:30) | `No space left on device` in the `semio-framework-os-infinite` build script (466 MiB free) | disk |
| `cad-check-6` (22:46) | `semio-s-artifact-stdio-semio` E0277/E0308: the peer's `ObjObject.faces: Vec<u64>` (22:04) vs the mesh obj serializers | peer, fixed 22:47 |
| `cadflow-check-7` ×2 (22:47–23:07) | fleet gate closed (13–22 `rustc` ≥ 8) | fleet load |
| from 02:45 | rule 26 CARGO HOLD | coordinator |

`🗑️generated/s2-flowcad/cargo-slice.sh` is the bounded, gated slice runner. It waits for the gate, runs cargo until the
deadline, then interrupts it, so the next slice resumes from the shared build-dir.

**Peer edits in my files (reviewed, consistent with my code).** The value-crate sweep (00:00) rewrote three things in the
CAD transient `🫧️transient/🦀️.rs`:
- `semio_framework_value_derive` derive paths;
- the `dsl::json` codec instead of `serde_json`, which is only a dev-dependency of CAD (that would have been a compile error
  of mine);
- `semio_framework_value::retirement`.

The same sweep touched flow `🌊️main/🫧️transient`, `✏️editor/🦀️.rs` and `OS/🌊️flow/…/📸️snapshot` (19:41). It also added a
`semio-framework-value` dependency to both manifests.

The sweep left `Some(to_value(dict)).map_or(Empty, …)` in `OS/🌊️flow/…/📸️snapshot/🦀️.rs`; it is always `Tree`. I simplified
it to `DagPreviewContent::Tree { json: to_value(dict) }` (02:55, one-line compile-atomic edit).

**Static review against the current framework APIs (no compiler).** These signatures match the CAD retained route and
transient:
- `ArtifactCommandWork::{tool_id, extent, step}`;
- `ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral }`;
- `EphemeralEmit::default()` with `window_transient: Vec<WindowTransientMutation>`;
- `ArtifactOwnedToolJobContext.{view_state, window_config, window_transient}`;
- `WindowTransientMutation::of::<O>(id, m)`;
- `WindowTransientSnapshot::{window_kind_id, get::<O>}`;
- `TransientView.window`;
- `CadConfig`'s 6 fields == `cad_config_from_runtime`;
- `CadPlayRuntime::default().engagement_step == "Idle"` == the transient's rest state.

The seam in `🛠️tool-machine/🦀️.rs` is unchanged since 13:17.

**Taxonomy (from S2-TAX's scoped `verify taxonomy report`, 22:35, `🗑️generated/s2-tax/findings-classified.tsv`).**
- Mine, fixed: `🫧️transient/🧪️tests/🔬️schema` was `directory-kind-unresolved`. I renamed it to `🔬️unit` (the registered
  `members-of-tests` name, used by 11 sibling transients) together with its `#[path]`, as one atomic move+edit.
- Shared and unregistered, kept for sibling consistency: `🪪️document` is no registered kind, and neither is
  `🪪️document-contract`. The same holds for `🔬️window` and `🔬️window-ownership`. My
  `🫧️transient/🧬️schema/{🧪️tests,🧫️fixtures}/🪪️document` mirror the CAD config schema's
  `🎚️config/🧬️schema/{🧪️tests,🧫️fixtures}/🪪️document`. Both trees, and flow's `🎚️config/{🧪️tests,🧫️fixtures}/🔬️window`,
  need ONE taxonomy registration. Owner: S2-TAX / REPO-PATH-BUDGET, whose shortened names these are.
- Pre-existing in my tree (09-09, REPO-PATH-BUDGET class): flow `👯️duplicate-widget/🚫️rejects` is
  `projection-catalog-coverage` / `directory-kind-unresolved`.
- `🧪️composed-child-history` / `🧫️composed-child-history` resolve through the open `test-case` kinds.

**§17.4 conformance (streamed gumball).**
- §13.1 makes the runtime's window-keyed `ScrubMachine` runner the ONE continuous-control runner, and forbids
  plugin-local scrub machines. CAD therefore streams the gumball through that runner: stream = tagged ticks, then commit
  or abort. The runner's provisional leaves render through the overlay.
- CAD's own window transient holds the per-frame engagement state (`CadEngagementScratch`), never config.

**After "hold lifted", in this order (one cargo at a time, gated, `--message-format=short`):**
1. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-flow-flow --lib --tests --keep-going`
2. `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-flowcad cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad --lib`
   (baseline 459 passed / 2 failed; new: `a_streamed_gumball_press…`, `every_engagement_step_is_window_transient_state_never_config`,
   `engagement_steps_are_window_state_until_the_one_committing_edit`, the transient `🔬️unit` laws)
3. the same for `-p semio-s-artifact-flow-flow --lib` (the 4 §12 laws + the 2 F6 laws)
4. `… cargo test -p semio-framework-plugin --lib -- composed_child_history time_travel scrub`
5. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-flow-flow --target wasm32-wasip2 --lib`

## Session 3 — 2026-10-02

Successor S3-FLOWCAD (coordinator `⚪b7db773a…`). Same ownership as S2 (flow + cad trees, the §12 composed-child seam as
region-scoped edits in `OS/🔌️plugin`). Scratch: `🗑️generated/s3-flowcad/`.

### S3.1 Repair check (rule 28) — 10:57

- Since S2.7 (02:45), files in my trees changed only by peers: `semio-framework-2d` dependency added to the cad and flow
  manifests, `📐️cad/🦀️.rs` (−3 lines), `OS/🌊️flow/…/🌿️vcs`, extension `📜️script.ts`s and release artifacts. No half-written
  edit of mine. S2's last attempts (`cadflow-check-8/9`, `cadflow-lib-10/11`, 03:00–03:50) died in the peer
  `RecordSpecProducer` refactor (kernel store/pack/dsl, plugin `🪟️window/🎚️config/📥️retained`), which has landed since.

### S3.2 Verification (in progress)

`cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-flow-flow --lib --tests --keep-going`
(10:57–11:21, `🗑️generated/s3-flowcad/check-1.txt`): **cad lib + lib-test: 0 errors** (cad lib-test 31 warnings, pre-existing
classes); **flow lib: 0 errors** (21 warnings); **flow lib-test: 6 errors, all in my test files**, fixed at once:
- `✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs` ×5: `&mut **app` on `FlowAppFixture`/`&mut FlowApp` (the fixture is no longer boxed) →
  `app` in the helpers, `&mut *app` in law bodies;
- `🧵️retained/🗿️artifact/📬️preparation/🧪️tests/🔬️unit/🦀️.rs:73`: `envelope().edits` → `envelope().vcs.edits` (peer store move).

### S3.3 §12 closure (source, 11:30)

- **Bug fixed — a member finalize announced only its LAST transition.** `publish_time_travel_member` sent
  `store.envelope().transitions.last()` on the member lane, so a finalize as NEW ALTERNATIVE (pending-edit commit + `Branch` +
  scoped `Supersede`) reached other replicas as a `Supersede` scoped to an alternative they never heard of. Now
  `TimeTravelStoreCommand::Commit` records the transition ids before `commit_finished_replay` and answers
  `TimeTravelCommit::Finalized { authored }` = the encoded envelopes of EVERY transition the finalize authored, in order;
  `publish_time_travel_member(authored)` sends them all (`send_member_mutations` is a no-op without a backbone). The
  `TimeTravelStoreCommand::Transition` / `TimeTravelStoreOutput::Transition` pair is deleted (`OS/🔌️plugin/⏪️time-travel/🦀️.rs`,
  §12 regions only).
- **Laws (flow, `✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs` region `🔖️ComposedChildHistory`):**
  - `editing_a_node_drag_offset_replays_the_member_and_the_parent_scene_follows` now also pins that the parent never re-mints
    the live child (same content child id after the overwrite);
  - NEW `finalizing_a_member_edit_as_a_new_alternative_branches_the_member_and_keeps_the_live_child` (alternative finalize on
    the member store: same child id, member views a new alternative, branch folds the edited offset, parent scene follows,
    row superseded);
  - NEW `a_member_finalize_reaches_the_other_replica_on_the_member_lane` (two replicas over `MemoryBackbone`: A edits its
    drag in time travel and finalizes as an alternative; B folds the member lane and converges on the edited member and the
    same member alternative — this is the law that pins the bug above);
  - helpers `release_drag_as`, `history_edit_as`, `edit_drag_offset`, `finalize_as_alternative`, `member_alternative`.

### S3.4 N6 + §20 closure — every node-graph edit is an id record, every flow edit an intent leaf (source 12:15)

Coordinator items: gap audit N6 (React `setHostSnapshot` for non-drag edits) and closure census §(b) S3-FLOWCAD (10 flow
whole-scene `SetSnapshot` call sites, flow guard `coalesce_key`).

**Record shape (relayed to S3-GRAPHS), schema-first:** `RE/🧬️schema/🔣️node-graph-edit-rows/🔣️.json` +
fixture `RE/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json` (8 accepted, 14 refused rows, 4 journal answers, 5 add-node cases).
`nodeGraphEdit {operations: Row[≤256]}`, every row closed, entities by id:

| row | fields |
|---|---|
| `connect` | `sourceNodeId, sourcePortId, targetNodeId, targetPortId` |
| `disconnect` | `synapseId` |
| `move` | `gestureId, nodeIds (≥1, unique), dx, dy` (unchanged §13.3 record) |
| `setSlider` | `widgetId, value` (absolute; the release value of an inline slider) |
| `insertPort` | `nodeId, side: input\|output, index (u32)` |
| `delete` | `nodeIds, synapseIds` (unique ids, at least one in total) — replaces the ambient `deleteSelection` row |

Add-node is the guest's own verb (flow: `addWidget {kind, neuronKind?, label?, action?, format?, x, y}`, `$defs/AddWidget`);
the owner mints the id. Deleted rows: `setHostSnapshot`, `deleteSelection` (no legacy).

**One journal, both hosts.** `DagGraphEdit` gains `SetSlider {node_id, value}` and `InsertPort {node_id, side, index}`;
`dag_graph_edit_rows_json(edits)` is THE row encoder (`OS/♾️infinite/…/🕸️dag/🦀️.rs`), used by the flow host
(`take_graph_edits_json`) and the generic `GraphHost`; the wgpu bounded builder writes the same rows (`write_graph_edit_action`).
- DagHost: `node_positions`, `journal_moves_since` (one `move` per distinct offset), `journal_port_insert`; an inline-slider
  release journals `setSlider`; `journals_wire_edits` deleted.
- Flow host (`OS/🌊️flow/🖥️host/🦀️.rs`): `gesture_changed_content` / `hostSnapshotChanged` deleted — the journal narrates every
  content change (moves always, port inserts at the press, slider value at release, wires); `align_selection` journals its moves;
  flow wasm `alignSelection` (2593) answers the rows.
- `GraphHost` (`FW/🗺️surface/🕸️node-graph/🦀️.rs`): press baseline (`press_positions`, retired stepwise), release journals moves,
  `align_selection` journals moves, `take_graph_edits_json`; `GraphSession.takeGraphEditsJson` (wasm export); the press now routes
  through `GraphHost::pointer_down_screen`.
- React `RE/🕸️NodeGraph/🟦️.tsx`: `commitGraphFixture`, `graphEditSignature`, `commitGraphFixtureIfEdited`, `commitFixture`,
  `graphGestureAnswer` deleted. Generic surface: release and align dispatch `takeGraphEditsJson()` rows (`dispatchGraphEdits`);
  flow surface: release and align dispatch the journal rows (`nodeGraphEditRows`); Spotlight commit and catalogue drop dispatch
  `addWidget` (`flowAddWidgetArgs`, the same args the wgpu drop sends) instead of a host-local add + whole fixture.
- wgpu: `write_graph_edit_action` writes `setSlider`/`insertPort`; the catalogue drop also carries `label`.

**Flow guest (`PL/🌊️flow/…/✏️editor`):**
- `flow_content_leaves(base, next)` (+ `flow_node_leaves`): the id-keyed intent leaves (`remove-edge`, `insert-node`,
  `set-node-kind/label/position`, `remove-node-param`, `set-node-param`, `set-edge-endpoints/kind`, `insert-edge`, `remove-node`)
  in a dangle-free order; a pure reorder is no edit. `flow_scene_publication` (now `Result`) publishes them, so ALL 10 census call
  sites (`flow_content_edit` ×2 retained, `host_scene_edit` ×5: disconnect, remove-widget, reorganize, connect-media-ports,
  delete-selection, `rename-flow-widget`, `node-graph-edit`) land intent leaves. **The one surviving `set-snapshot` is
  `flow_scene_replacement` for `setActiveExample`** (loading an example replaces the whole document — the genuine replace
  intent §20.3 names). No flow gesture caller of `SetSnapshot` is left.
- `nodeGraphEdit`: rows `delete`/`insertPort` added, `setHostSnapshot`/`deleteSelection` deleted; the batch no longer reads the
  ambient selection (`node_graph_edit::apply`, `spotlight_commit::apply` deleted; the live and retained routes call `handle`).
- `addWidget` gains `label`, `action`, `format` (the descriptor fields a catalogue row carries; outputs/sliders added by drop no
  longer lose them). The binary/text command vector `optional_field_rows_keep_their_pre_migration_bytes` may need a re-seal
  (greenfield wire break, see verification).
- §20.1: the flow child-group output guard no longer reads `emit.coalesce_key`; flow/cad tests asserting `coalesce_key` now assert
  `transaction` (`✋️drag` test, cad `🧭️transform` test, cad unit ×4). Remaining `coalesce_key: None` are 4 `Edit` struct literals
  (flow `ED:387`, `📬️preparation:370`, cad `ED:1582,1793`) that S3-CLOSURE deletes with the field.

**Laws added:** dag `🧪️tests/🧪️node-graph-edit-rows` (encoder == fixture rows; moves per distinct offset; port insert journalled
once) and the wire-edit test now reads the production encoder; flow `the_guest_reads_exactly_the_shared_node_graph_edit_rows`,
`host_edits_land_as_the_intent_leaves_that_reproduce_the_scene` (fold law, 4 cases), `a_delete_row_removes_the_named_widget_and_its_wires_as_intent_leaves`,
`a_connect_row_is_one_insert_edge_leaf`; TS `RE/🧪️tests/🧪️node-graph-edit-rows/🟦️.ts` (Ajv 2020 strict + journal reader + add-node).
`🫳️node-graph-gestures` fixture `operationFields` gains `setSlider`/`insertPort`; its two source-scan tests follow the new contract.

TS verification: `./node_modules/.bin/vitest run --config T/🔍️w3-t-flowcad-node-graph.config.ts` → **26/26 passed, 3 files**
(`🗑️generated/s3-flowcad/ts-node-graph-2.txt`).

### S3.5 Shared decoder adoption + id delete row (12:45)

- **One decoder.** S3-GRAPHS landed `semio_framework_tool_machine::{node_graph_edit_rows, NodeGraphEditRow, NodePortSide}`
  (`FW/🛠️tool-machine/🦀️.rs` region `🔖️NodeGraphEditRows`, law against my fixture). Flow's `operations_from_action` now decodes
  through it (`impl From<NodeGraphEditRow> for FlowNodeGraphEditOp`; only the flow wire byte authority stays local); flow's own
  copy (`action_row_fields`, `action_string`, `action_id`, `action_ids`, `NODE_GRAPH_EDIT_ROOT_FIELDS`) is deleted. The two TS
  source scans (`🫳️node-graph-gestures`, `🔗️node-graph-wire-edit`) read field names from the shared decoder's `closed(&[…])`.
  Re-run: vitest **26/26 passed** (`ts-node-graph-3.txt`).
- **Menu delete names its ids** (coordinator: the framework helper still emitted the refused ambient row).
  `node_graph_delete_selection_spec(label, is_de, node_ids, edge_ids, dispatch)` (`OS/🔌️plugin/🦀️.rs` ~15296): ViaNodeGraphEdit
  emits `{operation:"delete", nodeIds, synapseIds}` from the selection resolved when the menu opens; an empty selection still
  omits the row (no reason field on `ContextMenuItemSpec`; gen2d pins omission), so no empty or ambient delete is dispatched.
  The 6 callers changed mechanically (`nodes.len(), edges.len()` → `&nodes, &edges`): trinity jack + rewriting, gen2d, gen3d,
  sequence, dag. Law `OS/🔌️plugin/🧪️tests/🧪️node-graph-delete-row/🦀️.rs` (`a_menu_delete_names_its_selection_as_one_delete_row`,
  mounted beside `composed_child_history_tests`).
- React typecheck `tsc -p RE/🎯️targets/⚛️react/…/tsconfig.json` (103 s): 6 errors, **none in my files** (store worker `line`,
  `🐚️Shell` `idleInstalledServiceStatu…`, `🛠️ShellHelpers` staged-arg test — peers).
- Rust: every cargo check since 12:31 dies in the peer `semio-framework-schema-registry` extraction
  (`🧬️schema/📇️registry/🦀️.rs:349/511` duplicate definitions, 28 errors; reported to the coordinator). My Rust is WRITTEN BUT
  UNVERIFIED until that crate is green.
