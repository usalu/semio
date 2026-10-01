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
