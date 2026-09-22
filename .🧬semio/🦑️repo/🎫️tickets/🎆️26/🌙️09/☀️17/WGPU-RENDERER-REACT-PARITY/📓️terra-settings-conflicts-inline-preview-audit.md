# ⚔️ Settings Conflicts: Inline Actions and Diff Preview

## Scope and confidence

Read-only source audit on 2026-09-21. No browser run or test was performed.

**High confidence:** the WGPU Conflicts view diverges from React for every selected open conflict. The source has three independent live gaps: the browser has no conflict-roster bridge, WGPU projects Accept/Discard as nested rows rather than inline actions, and it discards the conflict payload needed for the preview. The existing WGPU row-action renderer also has no action hit or accessibility owner, so merely changing the panel builder would paint inert icons.

## React is the authority

[buildConflictsTree](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📌️ChromePanels/🟦️.tsx:1041) places two sibling `Button` controls in the selected conflict row and then, in that same row, places a bounded scrollable `ConflictDiffPreview`. The buttons have distinct Accept and Discard labels, icons, IDs, and resolution arguments.

A missing current-document snapshot does **not** remove the preview. The React host intentionally passes `currentDocumentText: ""` [here](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8747), while [conflictDiffText](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔺️DiffViewHost/🟦️.tsx:139) still supplies the quarantined envelope JSON or degraded-edit comment as its `after` side. Therefore React renders a valid additions-only unified preview in this case.

React also reads the authoritative open roster at session start [through `readConflicts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8757), and replaces it only with the `resolveConflict` reply [here](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:8567).

## Current WGPU behavior

| Concern | Evidence | Classification |
| --- | --- | --- |
| Empty roster | [WGPU builder](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8273) emits the same unavailable row as React. | Correct when the application has no open conflict. |
| Browser/WASM roster | `conflict_rows` and `seed_open_conflicts` are native-only [at lines 9099 and 9120](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9099). The WASM resolver only removes a local row [at line 9147](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9147). | Live browser feature gap. An empty WGPU browser panel can be either honest empty application data or this missing producer; it does not establish parity. |
| Data projection | `ShellConflictRow` retains only id, kind, code, and message [at lines 747–755](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:747). Native `conflict_rows` drops envelopes and degraded edit IDs [at line 9099](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:9099). | Live missing preview data on every target. |
| Inline structure | The builder makes Accept/Discard nested `UiTreeItemNode` children [at lines 8295–8309](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:8295). | Live structural parity defect. React uses sibling inline controls, not expandable child rows. |
| Panel projection | The retained panel assembler unconditionally writes `row_actions: Default::default()` [at line 5176](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5176). | Concrete local drop: a builder-only switch to `UiTreeItemNode.actions` would still lose both actions before the accepted UI document. |
| Row-action render/input | The generic contract already exposes bounded `TreeItemProps.row_actions` [at line 573](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:573), and retained paint draws row-placement icons [at line 734](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:734). Yet retained input explicitly says a tree action has no own rect and registers only its parent `TreeItem` action [at lines 739–772](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:739). The test-only expansion to Button nodes is cfg-gated [at line 1392](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:1392). | Live interaction and accessibility gap. The painter’s `label` is not drawn either, so its current output is an icon affordance rather than React’s labeled buttons. |
| Detail placement | The retained tree rehydrator turns every non-control child into a nested item [at lines 628–658](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:628). The shell panel assembler explicitly refuses `ComponentScene` [at lines 4876–4880](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4876). | Live missing generic detail-placement path. A `DiffViewScene` already exists [at line 2696](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:2696), but cannot currently be an accepted selected-tree-row detail. |

## Minimal coherent repair

### 1. Use a real, bounded Toolbar child for inline conflict actions

Do not retain the child-row workaround and do not use `TreeItemProps.row_actions` as the React Button twin. The current row-action painter is icon-only, retained input gives it no separate hit rectangle, and the accessibility projection exposes no action-level target. Filling that pre-existing path would be a separate product feature, not parity with React's labeled sibling Buttons.

Represent the group with actual canonical records: one `Component::Container` with `ContainerRole::Toolbar`, a horizontal `StackLayout`, and the two existing `Component::Button` records as direct children. `Toolbar` already maps to the retained Stack and accessible toolbar role, and ordinary Button records already have exact input, focus, action binding, and accessible-name ownership.

Add `TreeItemProps.inline_toolbar: Option<UiNodeId>` as an explicit direct-child relation. This is deliberately not `UiControlNode::Toolbar(UiStackNode)`: `UiControlNode` is a leaf-control mirror, so a Stack mirror would duplicate the canonical container/layout/record/action/accessibility contract and create a conflicting second control slot. Validate that the referenced direct child is a `Toolbar`, has horizontal flow, and has a small fixed direct-Button bound. `PanelProjection` emits the real Toolbar/Button records; the reconciler keeps that referenced child as an ordinary arena node and does not reinterpret it as a nested `UiTreeItemNode`.

Flex needs one narrow relation-aware branch. For this exact `inline_toolbar` child, begin with `flow_from_spec(authored)` so the Toolbar's horizontal direction, gap, padding, and child flow survive. Overlay only the tree-row anchoring/insets. Use intrinsic/hug width clamped to the usable row area, not the generic single-control width. Its Button children then use normal Toolbar flow. Other non-TreeRow children retain their current behavior.

### 2. Carry canonical conflict preview data and use a real browser producer

Extend the target-neutral shell projection with the exact preview `after` payload derived from the canonical conflict kind: pretty quarantined envelopes or the degraded-edit comment. Keep `before = ""`, matching React’s actual current host. Preserve the existing retained text limits when admitting the owned preview; do not add an unbounded panel-only string.

The WGPU browser bridge needs semantic equivalents of React’s `readConflicts` and `resolveConflict` exchanges. On resolution it must adopt the reply roster and keep it unchanged on rejection. Local `retain` is not an authoritative resolver.

### 3. Add one generic Tree-item detail slot

The current Tree record model has only child IDs and no child placement role; the rehydrator consequently treats a `DiffView` child as another TreeItem. Add an explicit, typed TreeItem detail-child reference/role to the schema and carry it through contract record construction, reconciler, layout, painter, input, and accepted-frame accessibility. The role must accept an ordinary bounded child node, including `ComponentScene(DiffView)`; it must not be a `ConflictPreview` or `SurfaceKind::DiffView` special case.

That is the smallest reusable extension that can place the selected detail below the row’s inline actions and inside the same bounded vertical scroll area. It also avoids weakening the panel assembler’s current loud refusal of unsupported scenes.

## Fail-first laws

1. **Language-neutral fixture:** replace or supplement [set-conflicts.json](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧫️fixtures/⚔️set-conflicts.json:69). Its legacy `{ conflictId, documentId, description }` shape only tests reducer state and cannot produce the current protocol preview. Use one open quarantined and one degraded conflict, a selected id, expected empty `before`, exact `after`, and both resolution bindings.

2. **WGPU accepted-frame law:** use the new inline-control test at [wgpu-display-conflicts-marketplace](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs:294) as the structural RED: publish the fixture through `panel_ui_records`, complete paint, seal, and ACK, then assert the selected conflict has one canonical Toolbar child and two real Button records with the exact resolution bindings. Extend it to hit the Accept Button rectangle and observe only `resolveConflict(conflictId, true)`; the parent selection action must not fire. Assert the selected detail is a real DiffView with the exact fixture text. Repeat for Discard and an unsuccessful reply, which must retain the row.

3. **React mounted-panel law:** extend the existing real codec/bridge oracle [at line 3922](/Users/ueli/Documents/semio/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:3922) with a mounted Conflicts-panel assertion: selected row has two named buttons in one control and a nonempty additions-only preview when `currentDocumentText === ""`.

4. **Shared DiffView rendering oracle:** retain the existing WGPU DiffView renderer test and feed it the same fixture payload. This validates the preview path rather than a duplicate text renderer.

## What this audit does not establish

The repository was not run. It does not claim that a current browser session contains an open conflict. It establishes that an empty view is valid for a real empty roster, while the WGPU WASM path currently cannot obtain a nonempty authoritative roster at all; these must remain separate acceptance cases.

