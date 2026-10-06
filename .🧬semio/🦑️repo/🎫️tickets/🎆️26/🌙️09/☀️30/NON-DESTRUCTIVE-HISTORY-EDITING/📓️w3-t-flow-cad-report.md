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

### S3.6 Resume after the usage cut (~13:05) and the machine reboot (~17:00) — 18:40

- Repair check: every edit of S3.3–S3.5 is on disk and complete (dag journal encoder, flow host, `GraphHost`, flow guest
  decoder adoption + intent leaves, time-travel `Finalized { authored }`, plugin delete row, React `dispatchGraphEdits`). No edit
  was in flight at the cut. 13:00 state of the peer break: `🛂️manifest/🦀️.rs:1266` and `♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🧬️schema/🧬️mutations/🦀️.rs:48`
  (schema-state extraction, owner S3-INFRA).
- 18:44 `cargo check -p semio-framework-os-infinite -p semio-framework-surface -p semio-framework-os-flow -p semio-framework-plugin
  -p semio-framework-os-renderer-wgpu --lib --keep-going` (`check-9.txt`): **os-infinite (dag journal), surface (`GraphHost`),
  os-flow (flow host + wasm), plugin (§12 time travel, delete row): 0 errors.** renderer-wgpu: 158 errors, **none in my regions**
  (`write_graph_edit_action`, `flow_widget_drop_args` type-check clean); all in peer code (`WorkerCell<Ui>::with` bounds ×87,
  `&UiComponentSceneNode` ×41, arity ×21 — EngineCanvas `:957,1988,1990,2412,6006`, Interpreter, Shell).
- 19:02–19:26 ✏️s workspace checks (`check-10..14`): two runs killed by the deadlock breaker (rule 33) while waiting on locks, then
  the kernel went red in the peer DSL-crate extraction (`🗣️dsl/🦀️.rs:931` ambiguous `canonicalize`, then `dsl::Diagnostic`,
  `📡️spr/🧵️channel` ×253). Per coordinator: no polling; verification resumes on "TREE GREEN".
- **Flow TS source-contract suite** (`bun …/✏️editor/🧪️tests/🔬️source-contract/🟦️.ts`): was red on drift, now **passes (exit 0)**
  (`flow-source-contract-5.txt`). Fixes, all in that test file: the retained preparation digest arm is `1 | 2` since a peer
  removed the edit-line/id texts (18:58), asserted as a regex over the arm + `flow-content-sha256-` prefix; two `URL` bases lacked
  their trailing slash (`mutationFixtureRoot`, `duplicateRoot`), so every relative read resolved beside the directory; three
  plugin-level paths moved to `🌎️hub/🧩️compositions/🌊️flow/` (rule 22: `🦀️.rs`, `🧫️fixtures/🧹️surface-owners`, `🧪️tests/🔬️surface`).
- Re-run: `cad-window-transient-contract rows=6` PASS, `composed-child-history-oracle cases=4` PASS.

**Owed verification (runs on "TREE GREEN", in this order, gated, one cargo at a time):**
1. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-flow-flow --lib --tests --keep-going`
2. `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-flowcad cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad --lib`
   (baseline 459/2; S2.7 laws + transient laws)
3. same for `-p semio-s-artifact-flow-flow --lib` (§12 laws ×6 incl. the 2 new, F6 ×2, intent rows ×4,
   `optional_field_rows_keep_their_pre_migration_bytes` may need the AddWidget vector re-sealed)
4. `… cargo test -p semio-framework-plugin --lib -- composed_child_history node_graph_delete_row time_travel scrub`
5. `… cargo test -p semio-framework-os-infinite --lib -- node_graph_edit_rows wire_edit`
6. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-cad-cad -p semio-s-artifact-flow-flow --target wasm32-wasip2 --lib`
- 19:44 one gated re-check (`check-15.txt`): kernel still red in the peer DSL extraction (`📡️spr/🧵️channel/🦀️.rs` ×253,
  `⚖️protocol-laws` ×18, store ×9, vcs ×7). Parked for "TREE GREEN" (coordinator resumes me); the owed list above is unchanged.

**Coordinator actions (S3):** rebuild the `framework-surface` wasm bindings (new `GraphSession.takeGraphEditsJson`) and the
flow-core wasm (`pointerUpScreen` answers rows only, `alignSelection` answers rows) before any React node-graph probe; `describe`
flow (`addWidget` gains `label`/`action`/`format`; `nodeGraphEdit` row vocabulary) and cad (S2.5 config/lanes); `verify taxonomy
report` for the new open-pattern dirs (`RE/🧬️schema/🔣️node-graph-edit-rows`, `RE/🧫️fixtures/🧫️node-graph-edit-rows`,
`RE/🧪️tests/🧪️node-graph-edit-rows`, dag `🧪️tests/🧪️node-graph-edit-rows`, plugin `🧪️tests/🧪️node-graph-delete-row`); S3-CLOSURE
deletes the 4 `Edit { coalesce_key: None }` literals with the field; S3-GRAPHS/S3-PROCEDURAL/S3-TEXT drop their own
`setHostSnapshot`/`deleteSelection` decoders (shared `node_graph_edit_rows`).

### S3.7 Audit fixes (`📓️audit-s3-tools.md` S3-FLOWCAD F1–F7, X3) — source 05:57 (10-03)

Resume note: the session context was cut between ~20:30 and 05:47; on resume I found the F1 caller wave partly on disk (flow host
`journal_gesture_moves`/`take_graph_edits_json`, `GraphHost`, dag tests) and completed it compile-atomically below.

- **F1 (major) — no silent drop.** `DAG_GRAPH_EDIT_CAPACITY` = 256 = `NODE_GRAPH_EDIT_MAX_ROWS` (law in the flow guest). Only moves
  grow with the graph, so `DagHost::journal_moves(gesture, displacements)` reserves ALL move rows or refuses the WHOLE gesture
  (`DagJournalRefusal {rows, limit}`, journal emptied, refusal kept until `take_journal_refusal`); `push_graph_edit` no longer drops
  (a press journals ≤ 4 rows); `journal_drag` deleted. Every caller restores the baseline on refusal: flow host release (baseline
  snapshot + `rebuild_dag` + `carry_journal_refusal`), flow host align, `GraphHost` release and align (`restore_node_positions`).
  The JSON answer carries `"refused": {"rows", "limit"}` beside `operations` (renderers dispatch only `operations`).
  Laws: `a_two_hundred_node_gesture_journals_whole_and_an_oversized_one_is_refused_whole` (200 rows whole; 256 moves + 1 port →
  refused whole, nothing journalled, positions restored).
- **F2 (major) — ONE encoder.** `write_dag_graph_edit_rows(edits, sink)` + trait `DagGraphEditRowSink` own every row's operation,
  fields and order; the JSON answer (`DagGraphEditJsonRows`), the wgpu bounded builder (`BoundedGraphEditRows`, `⚙️EngineCanvas`)
  and the credit census (`dag_graph_edit_row_strings`) are sinks of it. wgpu's hand-written row builder is deleted; `index` is an
  integer on both. Law `the_row_string_census_names_every_written_field_and_text`; TS scans now assert the shared writer is used.
- **F3** wgpu `DagPointerPhase::Leave` → `pointer_cancel_screen` (zero trace) on both engines; `Up` alone releases.
- **F4** flow `insertPort.side` other than `input`/`output` is refused at execution (`node_graph_edit_result`), so a DSL/binary-decoded
  op cannot reach `add_output_port` by default.
- **F5** flow `tool_intent_kinds` (`#nodeGraphEdit`, `#moveMediaNode` → `drag-nodes`); law
  `a_release_that_wires_and_drags_is_labelled_by_the_drag`.
- **F6** §12 finalize: the session applies `Finalized` BEFORE the member publish (a publish failure can no longer strand it); the
  history row label is the glossary `TimeTravelLabel::MemberEdited` ("History of a composed part edited" / "Verlauf eines
  eingebetteten Teils bearbeitet") — Rust enum + `ALL` (37) + TS twin + schema enum + lifecycle fixture — instead of the raw
  `<slot>/<childId>` literal.
- **F7 — deferred (owner S3-W2A):** `node_graph_delete_selection_spec` keeps `is_de` like the whole `🖱️MenuBuilder` region
  (`selection_count_phrase`); a locale-glossary pass needs the menu-wide API and a `disabled_reason` on the UI crate's
  `ContextMenuItemSpec` (no such field today). Empty selection still omits the row (no refused row is dispatched).
- **X3** dag `[TRACE]` diagnostics deleted with their logging-only state (`dag_debug_log`, `dag_interaction_label`,
  `last_logged_lod`); wgpu per-gesture edit-list traces deleted. Kept: the digest-deduped wgpu hit/geometry census lines (a
  documented browser-probe channel, not per tick).
- TS: vitest node-graph suites **26/26** (`ts-node-graph-6.txt`).
- §20.9 (flow host materializes declared input defaults): written by S3-PROCEDURAL (`default_neuron_params_from_info`, agreed
  10-02 22:xx); I verify it with the flow lib run.

**Verification 06:00–06:08 (10-03):**
- `cargo check -p semio-framework-os-infinite -p semio-framework-surface -p semio-framework-os-flow -p semio-framework-time-travel
  -p semio-framework-plugin -p semio-framework-os-renderer-wgpu --lib` (`check-17.txt`): **os-infinite, surface, os-flow,
  time-travel, plugin: 0 errors.** renderer-wgpu: 10 errors, all `🧊️renderer/🦀️.rs:18985–18998` (`?` ValueError conversions, DSL
  class) — none in `⚙️EngineCanvas` (the `BoundedGraphEditRows` sink and Leave→cancel type-check).
- `cargo test -p semio-framework-time-travel` (private target): **14 passed, 0 failed** (label ALL == fixture incl. `memberEdited`).
- `bun test …/⏪️time-travel/🧪️tests/🧪️conformance/🟦️.ts`: **20 pass** after bumping the schema's `labels` min/maxItems 36 → 37
  (the first run failed exactly on that bound).
- `cargo check --manifest-path ✏️s/Cargo.toml -p cad -p flow --lib --tests` (`check-16.txt`): blocked by stdio zip/step/svg/gltf
  (`ValueError` conversions, DSL class, S3-INFRA). `cargo test -p semio-framework-os-infinite --lib` (`test-infinite-1.txt`):
  blocked by `🏪️store/♻️retirement/🦀️.rs:4-5` E0117 under the test feature set (peer).
- 06:32 one gated re-check (`check-18.txt`): ✏️s still blocked — stdio-step 125, stdio-zip 52, stdio-gltf 13, stdio-svg 6 (DSL
  `ValueError` class). Parked for "TREE GREEN (✏️s)". Owed list = S3.6 items 1–6, plus: dag laws
  (`node_graph_edit_rows_tests`, 4 incl. the 200-node and census laws; `wire_edit_tests`), flow F5 label law, plugin
  `node_graph_delete_row`, renderer-wgpu check once `🧊️renderer/🦀️.rs:18985` is green.

### S3.8 §20.15 — composed content only on the child lane (in progress, 11:45 10-03)

Gate `schema mutation-editability` → `parentLeafReadsChild`: flow 10 (`flow_working_scene` ×9, `precondition` ×1), cad 8
(`cad_pane_local_scene` ×5, `cad_selection_inverse_objects` ×3) (`🗑️generated/s3-flowcad/editability-*.txt`).

**Flow plan (S3-GRAPHS' sequence pattern):** `FlowSnapshot` holds only the content coordinate, so the parent vocabulary becomes
UNINHABITED (`pub enum FlowMutation {}` with hand `Mutation`/`SemanticMutation`/`OpText`/`OpBinary`/`ColdRetire` impls, empty
json/ts/graphql/proto twins). Every content edit is already a child-lane leaf (`flow_content_leaves`, S3.4); the retained
remove/disconnect/delete-selection routes build child leaves from the ids they scan instead of folding parent leaves. Deleted:
the 10 parent leaves (+ text/binary codecs, grammar, fixtures `🧫️fixtures/🧬️mutations/*`), `🌊️mutate-flow-1` (case + fixture +
python second implementation), the parent one-item preparation (`🧵️retained/🗿️artifact/**`, `🧵️retained/🧾️canonical/**`, the dead
generic `FlowStoreOneItemPreparationFactory` + test helpers), `flow_content_edit`/`apply_flow_mutation`, the subset oracle catalog.

## Session 4 — 2026-10-04

Owner: S4-FLOWCAD (Opus executor, successor of S3-FLOWCAD). Scope: flow §20.15 finish, cad §20.15, reload laws, D24 flow default
document, owed S3.6 runs, greenfield decision on `optional_field_rows_keep_their_pre_migration_bytes`.

### S4.1 Repair check (rule 34) — 02:10–02:50

- `git diff HEAD --stat` over the flow artifact tree: 124 files, mostly peer value/DSL sweeps (`dsl::` → `semio_framework_value::`,
  `Ok((|| { … })())` codemod wraps, `step(.., _cx)` signature) plus the S3.7 audit fixes. S3.8's own on-disk edit was ONE file:
  `🧬️schema/🧬️mutations/🦀️.rs` replaced by the uninhabited `pub enum FlowMutation {}` (hand `Mutation`/`SemanticMutation`/`OpText`/
  `OpBinary`/`ColdRetire`). Nothing else of the S3.8 deletion list had landed, so the flow crate was NOT compile-atomic: the ten leaf
  modules, `💾️binary`/`📝️text` facets, the parent one-item preparation (`🧵️retained/🗿️artifact/**`, `🧾️canonical`, `🔤️bytes`), the
  direct-store routes (`DeleteWidget`/`DisconnectWidgets`/`ReplaceWidget` builders), `flow_content_edit`, `♻️retirement::retire_mutation`,
  `🌊️mutate-flow-1` and their tests all still named the deleted variants and helpers (`apply_flow_mutation`, `KINDS`, `decode_flow_*`, …).
- References outside the flow crate (git grep, tickets excluded): only the stdio composition ledger row for `🌊️mutate-flow-1/🦀️.rs`
  (`🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧫️fixtures/🧩️composition/🔣️.json` `callers`) and the taxonomy member name `🌊️mutate-flow-1`.
- Decision: finish the S3.8 wave in one compile-atomic source wave (S4.2) before any cargo run.

### S4.2 Flow §20.15 deletion wave — source 11:55–12:40 (cargo check running)

(The session stalled 02:50–11:50 — no work happened in that window; status sent to `main` 11:53.)

- Parent vocabulary: `🧬️schema/🧬️mutations/🦀️.rs` = uninhabited `FlowMutation {}` (clean `match *self {}` bodies, value derives as
  sequence), twins `🔣️.json` (`not: {}`), `🟦️.ts` (`never`), `🔗️.graphql`, `🛰️.proto` empty, law
  `the_parent_vocabulary_is_empty_and_refuses_every_operation`.
- DELETED (rule 32, zero references re-proved with git grep, tickets excluded): the ten leaf dirs `➕️create-widget`, `🗑️delete-widget`,
  `🔢️reorder-widgets`, `🔁️replace-widget`, `🔌️connect-widgets`, `✂️disconnect-widgets`, `🔀️reorder-synapses`, `🔄️update-synapse-endpoints`,
  `📍️move-widgets`, `👯️duplicate-widget` (+ diff/inverse/tests/schemas), `🧬️mutations/💾️binary/**`, `🧬️mutations/📝️text/**`,
  `🧬️mutations/📖️.grammar.semio`, subset `🧫️fixtures/🧬️mutations/**` (20 quintets), `🧫️fixtures/🌊️mutate-flow-1`, `🧪️tests/🌊️mutate-flow-1`
  (case, `🐍️.py` second implementation, `🥒️.feature`), subset `🔮️oracles/🔣️.json` (only the deleted case's oracle + catalog `flow-1-any` +
  manifests), editor `🧵️retained/🗿️artifact/**` (one-item preparation, recipe, scene copy/hash + tests), `🧵️retained/🧾️canonical/**`,
  `🧵️retained/🔤️bytes/**`, editor fixtures `🧬️artifact-recipes.json`, `🧾️artifact-canonical.json`; `♻️retirement::SceneRetirementFactory`;
  the crate-root `op`/`spr` shims (all 40 `crate::op::FlowMutation` imports → `crate::FlowMutation`).
- Editor: the `📬️StorePreparation` region (generic `FlowStoreOneItemPreparationFactory`, `prepare_flow_artifact`, admit/bounded-byte
  helpers) is gone (only the three capacity constants stay, region `📏️StoreCapacity`); `build_artifact_store_one_item_preparation_factory`
  removed (parent lane has nothing to prepare); `flow_content_edit` deleted. New `flow_removal_leaves(child, node_ids, edge_ids)`: every
  named edge and every edge touching a named node (child order), then every named node — the retained `removeWidget`, `disconnect`
  and `deleteSelection` routes scan ids (`node_ids`/`edge_ids: Option<Vec<String>>`, closed through `Owner::Strings`) and publish
  `flow_content_leaves_emit(child_id, flow_removal_leaves(..))` on the content child; `patchFlowWidgets` no longer builds dead parent
  `ReplaceWidget`s. `🧵️retained/🦀️.rs` shrinks to `Owner::{Bytes, Strings}` (every other owner served only the parent preparation).
- `♻️retirement::retire_mutation` = `match mutation {}`.
- Tests/fixtures: `delete_cascade_inverse_restores_exact_edge_order_and_label` → `a_node_delete_is_child_removal_leaves_whose_inverses_restore_the_content`
  (fixture `🧹️delete-cascade` now pins `expectedLeaves` [remove-edge e1, remove-edge e3, remove-node cut]; child inserts append, so
  undo restores by id, not by index — order is not semantic in the child model, S3.4); diff test `move_widgets_diff_touches_only_the_content_slot`
  deleted (parent leaf); retained test `nested_dictionary_moves_without_cloning_and_retires_at_one_byte` deleted (owner removed).
  TS source contract: regions `🧬️ArtifactRecipes`, `🧾️ArtifactCanonicalShapes` deleted; `↩️DeleteCascadeOracle` rewritten (independent
  TS removal-leaf oracle + immer forward/inverse, compared by id); `🪪️ContentIdentityOracle` keeps digest/target/demo laws, drops the
  canonical/preparation/mutation-fixture/duplicate checks. Fixture schema: defs `FlowArtifactRecipes`, `FlowArtifactCanonical`,
  `FlowArtifactCanonicalMutation`, `FlowArtifactCanonicalEntry` deleted; `FlowDeleteCascade` pins `expectedLeaves`.
- Greenfield decision: `optional_field_rows_keep_their_pre_migration_bytes` DELETED — it pinned command bytes "captured from the pre-merge
  `flow_protocol` crate" (a compat pin); None/Some round trips stay covered by `every_command_round_trips_through_text_and_binary`
  (`AddWidget` with `None` fields, `SetGridVisible { Some(true) }`, `SetGridSnapEnabled { None }`).
- Shared files (Edit tool): taxonomy `members-of-tests` drops `🌊️mutate-flow-1`, `🟰️re`, `🟰️replaces`, `🟰️keeps`, `🗜️clamps` (no dir left
  on disk); stdio composition ledger `callers` drops the deleted `🌊️mutate-flow-1/🦀️.rs` row.
- Laws added (flow editor unit context): `composed_reload_law!("flow", FlowPlayApp, …, "../..")` (save → fresh load renders/folds
  identically, history edit on the reloaded document) and `composed_child_history_law!("flow", …, [("nodeGraphEdit", move add by (25, 5))])`
  (child-lane drag edited end to end on `content/<childId>`: overwrite, save/reload fold, alternative). The flow child-drag ONE-ROW
  assertion the coordinator asked for already exists and stays: `a_node_drag_is_one_child_transaction_row_naming_its_member_store`
  (`rows.len() == 1`, `transaction.tool == "s.flow.flow@1/*#editor#nodeGraphEdit"`, one `drag-nodes` row on `content/<childId>`), and
  benefits from S4-GRAPHS' `close_streamed_transaction_unit` fix.

### S4.3 Verification (rules 43/44: checks only, then CARGO FREEZE at ~12:27)

| Command | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow -p semio-s-artifact-cad-cad --lib --keep-going` (12:03–12:2x, `check-1.txt`) | the deletion wave compiles; flow 3 errors, all peer value/DSL fallout in my files, fixed after: `💡️inferences/🦀️.rs:23` `protocol::ValueError` private → `semio_framework_value::ValueError` (codemod wrap removed); `✏️editor/🦀️.rs:466` `Fault: From<ValueError>` missing → `map_err(\|error\| Fault::from(error.into_message()))`. cad 1 error `🪶️sqlite/🦀️.rs:256` `crate::CadArtifact` (S4-INFRA sqlite ABI sweep, rule 41c; already rewritten to `crate::schema::CadArtifact` on disk 12:17). Unused `serde_json::json` import → `#[cfg(test)]`. |
| re-check `check-2.txt` | STOPPED by me at 12:27 for rule 44 (CARGO FREEZE) while rebuilding peer deps — OWED |
| `bun ./✏️s/…/✏️editor/🧪️tests/🔬️source-contract/🟦️.ts` (`flow-source-contract-1.txt`) | PASS (exit 0) — new delete-cascade oracle + fixture schema |
| `./node_modules/.bin/vitest run --config T/🔍️w3-t-flowcad-node-graph.config.ts` (`ts-node-graph-1.txt`) | **26/26 passed**, 3 files |
| `bun ./📜️script.ts verify taxonomy report --scope ✏️s/🔌️plugins/🌊️flow/🗿️artifacts` (`taxonomy-flow-1.txt`) | 29 findings, all pre-existing (directory-kind-unresolved ×27 on dirs I did not create, path-too-long ×1, script disposition ×1); this wave created no directory |

**OWED (rules 43/44), exact commands, in order, on CARGO OPEN / TESTS RESUMED:**
1. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow -p semio-s-artifact-cad-cad --lib --keep-going --message-format=short`
2. `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow -p semio-s-artifact-cad-cad --target wasm32-wasip2 --lib`, then
   `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-flow -p semio-hub-cad --target wasm32-wasip2 --lib` (→ "COMPOSITION GREEN flow/cad")
3. D24: `python3 T/🗑️generated/s4-flowcad/d24-patch.py` (staged; os-flow is a rule-39 shared crate, so it is applied only together with
   an immediate `cargo check -p semio-framework-os-flow --lib`) — records math.add's declared defaults `a`, `b` = number 0.0 in
   `FlowHostSnapshot::default()` and `FlowArtifact::default()` (§20.9; wires still shadow literals, §20.10).
4. tests (TESTS RESUMED): `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s4-flowcad cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow --lib`
   (incl. `documents_reload_identically`, `child_history_edits_end_to_end`, `history_edits_end_to_end`,
   `a_node_drag_is_one_child_transaction_row_naming_its_member_store`, `a_release_that_wires_and_drags_is_labelled_by_the_drag`,
   `a_node_delete_is_child_removal_leaves_whose_inverses_restore_the_content`, `the_parent_vocabulary_is_empty_and_refuses_every_operation`);
   `… -p semio-s-artifact-cad-cad --lib` (baseline 459/2); `cargo test -p semio-framework-plugin --lib -- composed_child_history node_graph_delete_row time_travel scrub`;
   `cargo test -p semio-framework-os-infinite --lib -- node_graph_edit_rows wire_edit`; `cargo test -p semio-framework-os-flow --lib` (+ `-- flow_vcs`).
5. `bun ./📜️script.ts schema mutation-editability --json` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`): flow `parentLeafReadsChild` must read 0.

### S4.4 CAD §20.15 — plan (NOT STARTED in source; needs cargo, rule 41b forbids an unverified CAD save before the describe wave)

The 8 findings are the parent leaves that re-mint a pane's composed `s.stdio.semio@v1/model` child from its `local_owner` scene:
`create-object`, `delete-object`, `move-objects`, `rotate-objects`, `scale-objects` (diff/inverse read `cad_pane_local_scene`) and
`drag-selection`, `rotate-selection`, `scale-selection` (inverse reads `cad_selection_inverse_objects`). Conversion (flow/sequence pattern):
1. **Child leaves (stdio semio `🏛️model`, shared crate `stdio-semio`, rule 39; schema-first: leaf schema + full `x-semio-ui` en/de +
   `x-semio-inverse-rows perTarget:targets`, fixture quintets, third-party oracle row, TS twin):** relative `move-elements {targets, offset[3]}`,
   `rotate-elements {targets, pivot[3], axis[3], angle}`, `scale-elements {targets, pivot[3], factor[3]}` over `SemioModelElement.placement`
   (missing targets → `mutation.partial`, none → `mutation.target-missing`, non-finite → invariant). Existing `insert-element`,
   `remove-element`, `set-element` cover create/delete/absolute pose. Needs central `schema generate` + describe cad/stdio afterwards.
2. **CAD parent:** delete the 8 parent leaves (+ schemas, fixtures, tests, catalog/oracle rows, binary tags, grammar) — the parent keeps only
   child-slot lifecycle (`create-*-model`/`delete-*-model`, drawings), references and nodes; pane children stop re-minting
   (`cad_pane_rematerialized_child`, `cad_pane_child_diff_slot` die) — the child id stays stable like flow's `content`.
3. **Readers compose parent + child on read:** `cad_pane_local_scene(document, pane)` → `cad_pane_scene(document, children, pane)` reading
   `children.typed_read::<SemioModelSnapshot>(cad_pane_model_slot(pane), child_id)` through `objects_from_model_snapshot` (bundled catalogue
   fallback only for an uncomposed handle), at every window/engine/tool/tree site (~12).
4. **Writers:** gumball/transform tool commit, `transform.move/rotate/scale` interactions, create/delete object commands and inspector pose
   edits publish `ChildEmit::of::<SemioModelSnapshot, _>(slot, child_id, leaves)` as ONE child transaction (`Emit::commit_transaction` with
   `child_emits`, §12), labelled by the child leaf.
5. **Laws:** `composed_reload_law!("cad", …)`, `composed_child_history_law!("cad", …, [gumball drag])`, one-row-per-gesture law, gate
   `parentLeafReadsChild` cad 0.

### S4.5 Coordinator actions
- Run nothing new for flow until the OWED checks pass; then: `describe` flow (parent mutation vocabulary now EMPTY: descriptor's mutation
  roster/kinds change; `addWidget` `label`/`action`/`format`; `nodeGraphEdit` rows) and cad (S2.5 config/lanes); central `schema generate`
  (flow mutation catalog rows for the 10 deleted leaves disappear; `mutations.json` is `not: {}`).
- Stale generated artifacts to regenerate (rule 32): flow committed descriptor `🌎️hub/🧩️compositions/🌊️flow/🔣️.json` + `🛂️.descriptor.semio`
  and dev copies (they still list the 10 parent kinds), schema catalog rows `s/flow/flow/mutation/*`.
- Wasm rebuilds still owed from S3: `framework-surface` (`GraphSession.takeGraphEditsJson`), flow-core (`pointerUpScreen`/`alignSelection`
  rows) before any React node-graph probe.
- CAD §20.15 (S4.4) needs a cargo window: model-subset relative leaves in `stdio-semio` (cross-WP with S4-STDIO; I will write them) +
  central `schema generate` + describe stdio/cad after.
- Notices item (S4-GATES) DONE in source: `flow.retained.tool-mismatch` → framework code `app.command.tool-mismatch` (`✏️editor/🦀️.rs`, same idiom as wfc/gen2d/gen3d); covered by OWED check 1.

### S4.6 CAD §20.15 — staged waves (rule 45; coordinator approval 12:5x)

Staging tool `T/🧪️s4-flowcad-stage.py <wave> add|new|delete|status|diff|apply` (mirror tree `🗑️generated/s4-flowcad/stage-<wave>/tree/`,
manifest with the sha256 of every base; `apply` refuses the whole wave when any base drifted, writes nothing then). Waves:
- `stage-d24` (os-flow, shared crate): flow DEFAULT document records `math.add`'s declared defaults (replaces the earlier patch script).
- `stage-model` (stdio-semio 🏛️model, shared crate; SOURCE COMPLETE 13:4x): relative child leaves `drag-elements {targets, offset[3]}`,
  `rotate-elements {targets, axis[3], angle}`, `scale-elements {targets, factors[3]}` — diff off the BASE placement (translation + offset;
  Hamilton `delta ⊗ rotation` of the unit axis/angle turn; scale × factors), `mutation.partial` / `target-missing` / `no-op` / Fatal
  `invariant` (incl. `axis-nonzero`), exact inverse = one absolute `set-element` placement per addressed element. Schema-first: leaf
  schemas with full `x-semio-ui` en/de, `x-semio-inverse-rows {perTarget:{targets:1}}`, `x-semio-invariant`, reference
  `{kind: object, domain: cad, granularity: object}` (CAD's selection domain, so "Use selection" fills the targets); leaf descriptors
  (binary tags 11–13), protocol records, text grammar (`.grammar.semio`, `.g4`, `.ebnf`: `hex-list`, `triple`), union/TS/GraphQL/proto
  twins, `KINDS` (= wire-tag order), print/parse, demo cases, unit law `relative_placement_leaves_derive_from_the_base_and_undo_exactly`.
  Test platform `🏛️mutate-semio-model`: 3 Examples rows in each outline on the real capsule tower (two capsules dragged by a fractional
  offset; two quarter-turned capsules half-turned so the turn composes; one capsule scaled non-uniformly), 3 spec-vector rows + fixtures
  written by `T/🧪️s4-flowcad-model-vectors.py` (third statement of the semantics; `--check` idempotent), Python second implementation
  extended (`placement_motion`, apply + exact inverse; exercised locally: all 3 vectors apply+undo, all 3 tower rows undo to the tower),
  Rust adapter decoder arms, oracle catalog kinds + manifest rows. Finding for S4-STDIO: the stdio composition ledger's `callers[].sha256`
  for `🏛️mutate-semio-model/🦀️.rs` already mismatches the on-disk file (peer edits), so that ledger test is red independent of this wave.

### S4.7 PARKED (coordinator usage limit, 10-04) — exact state and next steps

Repository tree: consistent. Nothing under `📐️cad/**`, stdio-semio or os-flow was saved in place this session. All CAD/model/D24 work is
in the stage waves (`python3 T/🧪️s4-flowcad-stage.py <wave> status`: no DRIFT on `stage-cad` at park time). The flow §20.15 wave (S4.2) is
on disk, and its checks are still OWED (S4.3, rule 44).

`stage-cad` (CAD §20.15 plugin side, IN PROGRESS, not compile-checked, apply only at CARGO OPEN after `stage-model`):
- Done in stage:
  - Deleted the 8 parent object leaves plus their fixtures. Pruned the aggregate (`🦀️.rs`/`🟦️.ts`/`🔣️.json`/protocol/unit test/oracle rows).
  - Lossless ModelBridge (`"cad"` pset).
  - Crate root:
    - removed `cad_pane_local_scene`, `cad_pane_genesis_child`, `cad_scene_with_pane_objects`, `cad_composed_snapshot` and the
      duplicate `cad_working_scene_from_models`;
    - the genesis catalogue `cad_bundled_pane_scene` is keyed by stable child id and built from `inferences::forest_pane_scene`, with no
      `local_owner` read;
    - `cad_genesis_child_pack` reads only the catalogue;
    - new `CadComposedPanes::compose(snapshot, children)` reads child content only through `ChildContentView`, falling back to genesis
      objects. It holds `CadComposedPane { objects, genesis: Option<Arc<CadWorkingScene>> }` with `geometry(pane)`. New
      `cad_scene_pane_geometry`, `CadPaneId::index`.
  - Inferences:
    - `forest_pane_scene(pane)` is the single import, cached in a OnceLock;
    - `forest_play_document()` mints plain handles, with no `with_local_owner`.
  - Transform tool rewritten:
    - `CadToolEntry::{Transform, Create{pane, element}}`, `CadToolLeaf {pane, leaf: SemioModelMutation}`;
    - `CadPaneModels` + `cad_pane_models(snapshot, children)`;
    - `cad_tool_yields` folds per-pane model state;
    - `cad_child_leaves_emit` gives one ChildEmit per pane, with `commit_child_transaction` when a transaction exists;
    - `cad_transform_tool_emit` reads `doc.children`.
  - Editor:
    - writers on models: `create_object_entry`, `cad_object_copy`, `delete_object_leaves`, `duplicate_object_entries`,
      `apply_transformation_entries`, `cad_set_element_leaf` (facet-only `set-element`, foreign psets kept), `patch_objects_leaves`;
    - `try_commit_session_entries`/`engagement_submit_entries` take `&CadPaneModels`;
    - deleted the dead `object_field_mutation` and the document-based `cad_pane_objects`/`cad_pane_of_object`;
    - publication lanes: object, transform and engagement-commit tools are `Child` (+ `WindowTransient`);
    - `cad_retained_reduce` binds `ArtifactView::with_children(..).bound_to_operation(..)` from `context.children`;
    - `CadPlayView` gains `pub(crate) panes: CadComposedPanes` plus `CadPlayView::of(document, children, runtime, interaction)`;
      the 3 editor construction sites use it;
    - `collect_pane_solids` reads panes;
    - `import_media` `geometry:in` is now a real `insert-element` on the Shape child.
- NEXT (in order), all inside `stage-cad`:
  1. `🎮️commands/🧱️object/🦀️.rs`: models via `cad_pane_models(doc.snapshot, &doc.children)`. Then wire each command:
     - add → `cad_transform_tool_emit(doc, "addObject", vec![create_object_entry(..)])`, with label count from `models.model(pane)`;
     - delete → `cad_child_leaves_emit(&models, None, &delete_object_leaves(..))`;
     - duplicate → tool emit;
     - patch → `cad_child_leaves_emit(&models, None, &patch_objects_leaves(..))`;
     - update the module doc.
  2. `🤝️engagement` (lines 45/93/181) and `🔄️transform` (`apply_transformation_entries`, module doc).
  3. `📥️io` CadPlayView sites 63/79/107 → `CadPlayView::of(.., &doc.children, ..)`.
  4. Edit mode:
     - delete `cad_pane_working_scene`;
     - `build_world_scene_for_pane` reads `envelope.panes.pane(pane)`;
     - mesh cache keyed by the genesis `Arc` (`Option<Weak>`, a None key cached by digest);
     - fix the doc at :186/:342.
  5. Panels `🗿️artifact` :156 and `🔍️inspection` :78/:191/:231 → `envelope.panes`.
  6. Tests:
     - editor tests (CadPlayView ×6 → `CadPlayView::of(.., &ChildContentView::EMPTY, ..)`; :925 local-owner guard → panes);
     - inspection test :24;
     - transform tool tests (CadToolEntry/models);
     - example test :16 (`cad_bundled_pane_scene`).
  7. Sample-scene fixture: delete `materialized_shape_scene`/`materialized_objects`.
  8. Laws in the editor tests:
     - `composed_reload_law!("cad", CadPlayApp, …, "../..")`;
     - `composed_child_history_law!("cad", …, [("translateSelection", <ids of a forest shape object, dx>)])`;
     - a one-row-per-gesture law.
  9. Update the cad editor TS/source-contract regions, if any mention the deleted leaves.
- Risk to re-verify: the lossless bridge changes content-addressed pane child ids. Committed fixtures or tests that pin forest child ids
  must be re-sealed.

Coordinator AUDIT-TOOLS routing (`📓️audit-s4-tools.md`, received before the park). NOT STARTED; record only:
- F4 / D24 flow:
  - build genesis from a plugin catalogue keyed by stable child id;
  - compose-on-read reads only through `ChildContentView`;
  - drop `FlowWorkingScene` as a cache vehicle;
  - name or delete `flow-edit-scene-owner-missing`.
  - The cad pattern above is the template.
- F1 cad: covered by `stage-cad` above.
- F9:
  - cad `:2093` raw `cad-retained-command-tool-mismatch` → `app.command.tool-mismatch`;
  - flow `fault_notices()` en+de for its 32 named faults.
- F13: delete the dead parent `🧬️schema/🔺️diff` surface of flow (19 files) and imperative (19), plus `FlowStringList`. Coordinate
  imperative/procedure with S4-GRAPHS.

OWED (rule 44): every check in S4.3, plus the D24 and S3.6 runs. When CARGO OPEN comes, run them in this order:
1. `stage-model` apply → `cargo check -p semio-s-artifact-stdio-semio --lib`.
2. `stage-d24` apply → `cargo check -p semio-framework-os-flow --lib`.
3. flow plugin native + wasm32-wasip2 check.
4. `stage-cad` apply (after the NEXT list) → cad native + wasip2 check.
5. hub `semio-hub-flow`/`semio-hub-cad` wasip2 → message "COMPOSITION GREEN flow/cad".
6. At TESTS RESUMED: the S3.6/S4.3 test list.

## Session 5 — 2026-10-05

Owner: S5-FLOWCAD (Opus executor, successor of S4-FLOWCAD). Scope: P1 flow import red + owed flow/cad checks → COMPOSITION GREEN,
P2 staged CAD §20.15 path (`stage-model` → `stage-d24` → `stage-cad`), P3 laws, P4 flow fault notices / F4 / F13, P5 follow-ups.
Scratch: `🗑️generated/s5-flowcad/`. Build gate v4 (rule 55: cargo < 4, rustc < 6, `CARGO_BUILD_JOBS=3`).

### S5.1 Repair check (rule 46) and P1 — 01:12–01:26

- Stage waves re-checked with `python3 T/🧪️s4-flowcad-stage.py <wave> status`: `model`, `d24`, `cad` — NO DRIFT (every base sha still
  equals the tree). The trees are byte-current but the plugin API they target moved (see next bullet), so `stage-cad` needs an API
  re-derivation, not a merge.
- Peer refactor found in the flow tree (mtime 10-04 23:19, 5 files, newer than HEAD): owned-child emissions are now STEPPED
  preparations — `Emit.child_preparations: VecDeque<ChildEmitPreparation>` (`ChildEmitPreparation::of::<S, M>(slot, child_id, Vec<M>)`,
  `M: RetireOwned`), `ChildEmit::of` is gone (`ChildEmit::open` + `push`), tests drive `Emit::prepare_child_one`. The codemod left ONE
  red in my tree: `✏️editor/🎮️commands/🩹️patch-flow-widgets/🦀️.rs:12` imported `ChildEmitPreparation` from the crate root → fixed to
  `semio_framework_plugin::{app::ChildEmitPreparation, …}` (the dead `ChildEmit` import dropped; no framework re-export).
- RAN `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-flow-flow -p semio-s-artifact-cad-cad --lib --keep-going
  --message-format=short` (01:13–01:25, `check-1.txt`): **exit 0, Finished in 12m 15s; flow 18 warnings, cad 35 warnings, 0 errors**
  (the S4.2 flow §20.15 deletion wave is green natively at `--lib`).
- 01:27 `… --lib --tests` (`check-2-tests.txt`) exit 101 in 53 s: NOT in my trees — `🧰️framework/…/🗣️dsl/🦀️.rs:219:71 E0308
  pack::record::EncodeOptions vs os_pack::value::EncodeOptions` (kernel; Codex pack peer mid-wave, rule 56). Reported to `main`.
  OWED until `foundation.status` reads GREEN: the `--tests` check, both wasip2 checks, the two hubs.

### S5.2 `stage-cad` re-derivation and NEXT list — source in the mirror tree (01:30 →), NOT applied

Mirror: `🗑️generated/s4-flowcad/stage-cad/tree/` (tool `python3 T/🧪️s4-flowcad-stage.py cad status|diff|apply`; 41 paths, no drift).
- API re-derivation: `cad_child_leaves_emit(models, transaction, leaves: Vec<CadToolLeaf>)` builds one
  `ChildEmitPreparation::of::<SemioModelSnapshot, SemioModelMutation>` per touched pane in `Emit.child_preparations`
  (+ `transaction`), no `ChildEmit::of` / `commit_child_transaction`; callers pass owned leaves.
- `stage-model` needs ONE more file than staged: the stdio-semio crate root `🧿️semio/🦀️.rs` has an exhaustive
  `impl RetireOwned for model_mutation::SemioModelMutation` (≈ L633) → three arms (`seq![targets, offset]`,
  `seq![targets, axis, angle]`, `seq![targets, factors]`) must land in the same wave (closure file, landing lock).
- NEXT 1 DONE (`🎮️commands/🧱️object`): add → tool emit (refuses `cad.object.pane-uncomposed` for a pane without a model child),
  patch → plain `set-element` leaves, delete → plain `remove-element`, duplicate → tool emit.
- NEXT 2 DONE (`🤝️engagement` ×3 sites, `🔄️transform`: `apply_transformation_entries`, relative-leaf doc).
- NEXT 3 DONE (`📥️io`: 3 × `CadPlayView::of`); plus `importCadFile` of a single STEP/OBJ/STL/GLB object is now a real
  `insert-element` tool transaction on the addressed pane's child (lane contract `[Child, WindowTransient]`, refusal
  `cad.import-object-refused`; the old `cad.import-object-unavailable` "no child seam yet" refusal is gone).
- NEXT 4 DONE (edit mode: `cad_pane_working_scene`/`cad_pane_working_objects` deleted, `build_world_scene_for_pane` reads
  `envelope.panes`, mesh-lane cache keyed by `(pane, genesis Arc | None, objects digest)`).
- NEXT 5 DONE (panels `🗿️artifact`, `🔍️inspection` read `envelope.panes`); picking + geometry-import doc lines.
- NEW (design hole found): an EMPTY pane had no child, so `addObject`/constructions/imports landed nothing after §20.15.
  `default_document()` now composes the four EMPTY model children (`crate::cad_empty_pane_child`, content-addressed from the
  empty model) and the genesis catalogue resolves them — every pane has a child lane from the first render.
- F9 cad done in the mirror: `cad-retained-command-tool-mismatch` → `app.command.tool-mismatch`.
- Example law re-authored (`demo_asset_is_the_concrete_forest_and_every_pane_resolves`: child ids first, genesis pack round
  trip through the lossless bridge). The demo asset `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` pins the four forest child ids
  (`shape-model-bd0fface41bc0b4d`, …) — the ONLY tracked file that does; it is re-sealed after the first compile prints the
  new ids (lossless bridge changes the content digests).
- OPEN in the mirror: NEXT 6 (tests: editor 2249 lines, transform tool, inspection, artifact, sample-scene fixture = NEXT 7),
  NEXT 8 (laws), NEXT 9 (TS/source contract).
- Finding (pre-existing, not this wave): whole-scene importers (`scene_from_spatial_payload`, `cad_document_from_dwg`) mint
  content-addressed pane handles whose content no archive member carries (`Effect::LoadDocument {pack, spr}` has no members)
  → imported non-empty panes cannot derive a genesis pack. Route: S5-LOAD (`artifact:in` carrier with members).

### S5.3 P1 closed — COMPOSITION GREEN flow + cad (01:48), flow `--tests` repaired (01:58)

| Command (gate v4, `CARGO_BUILD_JOBS=3`) | Result |
|---|---|
| `cargo check --manifest-path 🌎️hub/Cargo.toml -p semio-hub-flow -p semio-hub-cad --target wasm32-wasip2 --lib --keep-going` (`check-3-hub-wasip2.txt`) | **exit 0**, 3m36s; flow-flow 18 w, cad-cad 35 w, hub-flow 18 w, 0 errors → "COMPOSITION GREEN flow", "COMPOSITION GREEN cad" sent (cad = pre-§20.15 tree) |
| `cargo check PM -p semio-s-artifact-flow-flow -p semio-s-artifact-cad-cad --lib --tests --keep-going` (`check-4-tests.txt`) | exit 101: **cad lib test green (99 w)**; flow lib test 6 errors, all in my test files (never compiled since S4.2 + peer API moves) |
| fixes: `➕️add-widget` tests `dsl::ToValue` → `semio_framework_value::ToValue` (×2); `✏️node-graph-edit` tests `protocol::Terminology/Locale` → `semio_framework_ui_locale::…`; `📸️snapshot/🧪️tests/🪶️sqlite` `restore` returns `ValueError`; `✋️drag` tests drop the deleted `Emit.description` | — |
| `cargo check PM -p semio-s-artifact-flow-flow --lib --tests --keep-going` (`check-5-flow-tests.txt`) | **exit 0**, 1m53s; lib 18 w, lib test 54 w |

### S5.4 P2 step 1 LANDED — `stage-model` (stdio-semio 🏛️model relative leaves) under the `stdio` lock, 02:14:47–02:31

Wave = `python3 T/🧪️s4-flowcad-stage.py model apply` (32 paths, no drift; pre-wave copies in `🗑️generated/s5-flowcad/pre-model/`) + three
arms in the exhaustive `impl RetireOwned for model_mutation::SemioModelMutation` of `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs`
(`DragElements → seq![targets, offset]`, `RotateElements → seq![targets, axis, angle]`, `ScaleElements → seq![targets, factors]`).
On disk now: leaves `✋️drag-elements`, `🔄️rotate-elements`, `🔍️scale-elements` (Rust + leaf descriptor + payload schema with full
`x-semio-ui` en/de incl. `step`/`snaps`, `x-semio-inverse-rows`), aggregate (`KINDS` 14, tags 11–13, text print/parse, demo rows),
twins (`.ts`, `.graphql`, `.proto`, `.protocol.semio`, `.g4`, `.ebnf`, `.grammar.semio`), oracle catalog rows, `🏛️mutate-semio-model`
test platform (Rust adapter, Python second implementation, feature rows, 3 fixture triples), unit law.

| Command (gate v5, `CARGO_BUILD_JOBS=3`) | Result |
|---|---|
| baseline BEFORE the wave: `cargo check PM -p semio-s-artifact-stdio-semio --lib --tests --all-features --keep-going` (`check-6-…baseline.txt`) | lib **green** (392 w); lib test 14 errors, ALL pre-existing in `🖼️image/…/🖼️tiff` (10) and `📽️presentation` (4) test files — S5-TEXT-STDIO's |
| same command AFTER the wave (`check-7-stdio-semio-wave.txt`) | lib **green** (392 w); lib test the same 14 tiff/presentation errors, **0 diagnostics in `🏛️model`** |
| `cargo check HM -p semio-hub-stdio -p semio-hub-cad -p semio-hub-flow --target wasm32-wasip2 --lib --keep-going` (`check-8-hubs-wasip2.txt`) | **exit 0**, 6m24s (stdio-semio wasip2 229 w, hub-stdio 1 w, hub-flow 18 w) |

Not run: the other 23 hubs that depend on stdio-semio (the wave only ADDS enum variants; `git grep` shows no exhaustive
`SemioModelMutation` match outside stdio-semio). OWED (tests, rule 48/55): `cargo test PM -p semio-s-artifact-stdio-semio --lib --all-features
-- relative_placement_leaves_derive_from_the_base_and_undo_exactly kinds_match_the_enum_and_the_catalog mutate_semio_model` — blocked
until the 14 tiff/presentation test errors are fixed (the lib-test target does not compile); python arm:
`python3 T/🧪️s4-flowcad-model-vectors.py --check`.
Coordinator actions from this wave: central `schema generate` (3 new leaf payload schemas `s/stdio/semio/v1/model/mutation/{drag,rotate,scale}-elements`),
describe stdio (model vocabulary 11 → 14 kinds), stdio composition ledger sha for `🏛️mutate-semio-model/🦀️.rs` (already stale before).

### S5.5 P2 step 3 LANDED — `stage-cad` (CAD §20.15) applied to the plugin tree 10-05 07:33 (written 10-06; the 07:45 cut hit before the report)

- Re-anchor after wave B: the one drifted base (`✏️editor/🦀️.rs`, 12 `description` hunks by S5-CHANNEL) was 3-way merged into the staged
  file (`git merge-file`, clean; scratch `🗑️generated/s5-flowcad/reanchor/`); tests re-authored by
  `🗑️generated/s5-flowcad/cad-tests-rederive.py` (exact-match, fail-closed). `python3 T/🧪️s4-flowcad-stage.py cad apply` → 41 paths
  (pre-wave copies: `🗑️generated/s5-flowcad/pre-cad/tree/`). Deleted (rule 32): the 8 parent object leaves
  `🆕create-object ❌delete-object 🚚move-objects 🌀rotate-objects ⚖️scale-objects ✋️drag-selection 🔄️rotate-selection 🔍️scale-selection`
  (schema + fixtures dirs), crate-root `cad_pane_local_scene`, `cad_working_scene_from_models`, `cad_scene_with_pane_objects`,
  `cad_pane_rematerialized_child`, `cad_pane_child_diff_slot`, edit-mode `cad_pane_working_scene/_objects`, fixture
  `materialized_shape_scene/_objects`, `CadComposedPanes::pane_of` (unused).
- RAN (gate v5, JOBS=3): `cargo check PM -p semio-s-artifact-cad-cad --lib --keep-going` **exit 0** 07:37 (34 w, `check-9`);
  `… -p cad-cad -p flow-flow --lib --tests` 07:40: flow lib-test green (58 w), cad 1 error (missing `PluginApp` import in my transform
  test) → fixed → `… -p semio-s-artifact-cad-cad --lib --tests` **exit 0** 07:53 (lib 33 w, lib test 95 w, `check-11`).
  S5-AGNOSTIC's family build 10-05 18:40 compiled both lib-tests again (flow 58 w, cad 95 w).
- NOT landed: `stage-d24` (os-flow, `landing`): staged, no drift at 07:28 10-05, never reached the queue.
- Stale since this wave (coordinator): cad descriptor `🌎️hub/🧩️compositions/📐️cad/🔣️.json` + `.descriptor.semio` (8 parent kinds gone,
  object tools publish on `Child`), central `schema generate`; stale non-staged twin `🧬️mutations/📖️.grammar.semio` (pre-existing: it
  still spells `add-object`/`set-pane-objects`, vocabulary of two generations ago) — delete or regenerate with the describe wave.

### S5.6 Acceptance faults of 10-05 (S5-AGNOSTIC, design §22.36 / §23) — source 10-06 02:02–02:25, strict economy

**flow — `setActiveExample {exampleId: "demo"}` faulted on its own asset.** Cause: `✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` spelled the
`layout` map's record values inline (`add=x=120 y=-40`); the DSL codec requires braces for a map of records since the
`RecordSpecProducer` refactor (`🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🦀️.rs:1043` parse, `🛫️encoding` print emit `{`): "expected LBrace,
found Ident 'x'" (0-based line 11). Fix: the asset line is `add={ x=120 y=-40 } preview={ … } slider={ … }` (the one line; nothing
else touched). Law added: `every_shipped_example_loads_through_set_active_example` (`🎨️set-active-example/🧪️tests/🔬️unit`): every
`FlowPlayApp::examples()` entry parses through the route's boundary and the reducer accepts its id. The existing byte law
(`demo_example_ships_the_laid_out_default_graph…`: asset == printer output) decides the exact whitespace; if it differs, the writer
re-seals: `cargo test PM -p semio-s-artifact-flow-flow --lib -- --ignored zzz_write_demo_example_asset`.

**cad — creation leaves left an unloadable document (`closure-rejected`, Incomplete).** Cause (read from
`🗑️generated/s5-agnostic/family-flowcad.test.txt:2544`): the law applies `create-energy-model {childId: "cad-energy-2"}` on the initial
document; `cad_genesis_child_pack` derived a member only for catalogue ids, so the handle the leaf hands out had no store. Second
latent fault of the same root: the demo asset pinned the four forest child ids as content digests that the lossless model bridge
(S5.5) changed → `setActiveExample demo` would have loaded four underivable panes. Fixes (all `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad`):
1. `🦀️.rs` — genesis is TOTAL over declared children: a catalogue id is born with its scene, every other declared pane child with the
   EMPTY model, a declared drawing child (`CAD_DRAWINGS_SLOT`) with the empty drawing; an undeclared child has none. A handle is two
   strings — content reaches a child only as an archive member or as child-lane leaves.
2. `🦀️.rs` + `💡️inferences/🦀️.rs` — bundled documents use STABLE NAMED child ids (`cad_named_pane_child(document, pane)` =
   `<document>-<pane>-model`; forest: `hexagonal-cut-concrete-forest-left-{shape,building,energy,structure-classic}-model`; new
   document: `cad-…-model`), never digests; `cad_model_child_for_pane` deleted. Demo asset re-authored by hand (4 ids × 2).
3. §22.36 / §22.20 — ten child-lifecycle leaves declared withdraw-only (`"editable": false`, fifteenth descriptor key): the five that
   hand out an owned child (`🧱create-shape-model ⚡create-energy-model 🏢create-building-model 🏛️create-structure-classic
   📐️create-drawing`: `childId`/`target` are identity, nothing else remains), `🧹delete-drawing` (its only input names an owned
   child) and the four inputless `delete-*-model`.
4. Acceptance wiring (`✏️editor/🧪️tests/🔬️unit`): `composed_reload_law!("cad", …)` and `composed_child_history_law!("cad", …,
   [("translateSelection", {objectIds: ["object-hexagonal-cut-concrete-forest-left"], dx: 1.5})])` — the forest shape object's
   relative `drag-elements` on `shapeModel/<child>` is cad's seeded editable case; G12's derived cases (`create-node`,
   `replace-references`, the reference leaves) now run on a loadable document.
RAN: `zsh T/🚦️gate.sh 2 6 && CARGO_BUILD_JOBS=3 cargo check PM -p semio-s-artifact-flow-flow -p semio-s-artifact-cad-cad --lib`
→ **exit 0** 02:19:40 (flow 20 w, cad 33 w; `check-12-lib.txt`). Test files of this turn are WRITTEN BUT UNVERIFIED (no test builds).
OWED, in order: (1) `zsh T/🧪️s5-agnostic-run-family.sh flow semio-s-artifact-flow-flow`; (2) `zsh T/🧪️s5-agnostic-run-family.sh cad
semio-s-artifact-cad-cad`; (3) `cargo test PM -p semio-s-artifact-cad-cad --lib -- demo_asset_is_the_concrete_forest
a_new_document_composes object_gestures_undo translate_selection_moves a_mounted_translate a_streamed_gumball`;
(4) `cargo test PM -p semio-s-artifact-flow-flow --lib -- every_shipped_example demo_example_ships set_active_example_demo`;
(5) `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/📐️cad" --json` (0 `inputless`)
and, cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, `bun ./📜️script.ts schema mutation-editability --json` (cad 0).
OPEN (mine): `stage-d24`; P3 CAD history-edit laws (upstream parameter → downstream geometric failure as per-mutation Error);
flow 77 anonymous refusals / F4 / F13; §22.10 cad gesture slot; hub wasip2 re-check after this turn. Finding for S5-LOAD (kept):
whole-scene importers (`scene_from_spatial_payload`, `cad_document_from_dwg`) mint content-addressed handles whose content no archive
member carries — with total genesis such an import now loads EMPTY panes instead of faulting; the carrier must ship members.
Coordinator: describe flow + cad (flow hub `🔣️.json` embeds the old demo asset text), central `schema generate`.
RAN 02:20 (bun, repo root): `schema mutation-inputs --under "✏️s/🔌️plugins/📐️cad" --json` → census cad: 28 leaves, **10 withdraw-only,
28/28 inputs declared, 0 inferred, 0 `inputless`**; 8 diagnostics left, all `malformed` rows of the 8 DELETED parent leaves that the
central catalogue still lists (`s.cad.cad.mutation.{create-object,delete-object,move-objects,rotate-objects,scale-objects,
drag-selection,rotate-selection,scale-selection}`) → cleared by the coordinator's central `schema generate`, not by a plugin edit
(`🗑️generated/s5-flowcad/gate-inputs-cad.json`). `schema mutation-editability` was started and not awaited (output
`🗑️generated/s5-flowcad/gate-editability.json`) — OWED read.
RAN 02:2x (bun, cwd repo test domain): `schema mutation-editability --json` → **cad: 19 leaves, 9 editable, 10 withdraw-only, 0 findings**
(was 8 `parentLeafReadsChild`); flow: 0 parent leaves, 0 findings. The gate's 16 remaining diagnostics belong to other owners
(`🗑️generated/s5-flowcad/gate-editability.json`). The OWED read above is done.
