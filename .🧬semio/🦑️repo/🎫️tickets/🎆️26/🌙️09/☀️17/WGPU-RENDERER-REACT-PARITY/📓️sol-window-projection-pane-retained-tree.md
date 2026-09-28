# Retained World Projection Pane

## Scope

This packet replaces the WGPU shell's bespoke Projection button column with the retained `Tree`/Pane path used by the other shell panels. It also repairs the engine-owner lifetime exposed by focusing one World window and then restoring its hidden sibling.

## Neutral Contract and React Oracle

The schema `engine/🧬️schema/🔀️projection-pane/🔣️.json` defines one bottom-right 300px tree pane, fifteen depth-first rows, exact labels/icons/pane-qualified camel-case ids, selected template, and the framework action `{ windowId, templateId }`. The fixture lives at `World3dHost/🧫️fixtures/🔀️projection-pane/🔣️.json`.

The mounted React oracle renders production `WorldProjectionKindSwitch`. It validates the fixture with Ajv 2020, checks the actual `tree` and fifteen `treeitem` roles, pane-qualified ids, disclosure buttons, selected 3-Point row, labels, and row activation of Orthographic. It caught two assumptions before the native gate: descendants remain directly qualified by the pane rather than by ancestor paths, and hyphenated template ids pass through the shared camel-case element-id grammar.

## WGPU Changes

- Projection owns a retained per-window `UiDocumentLease` with the same publish, paint, hit, accessibility, retirement, and fault boundaries as Actions, Search, and Measures.
- Rows use React's pane-qualified `childElementId` identities and the same fixed English taxonomy, icons, open disclosures, exact framework action, and per-window selection source.
- The generic retained Tree now owns full-width rows, icons, chevrons, scrolling, pointer activation, and tree/treeitem accessibility. The old `shell.projection.template.*` private action grammar and bespoke measured button column are removed.
- The pane body is right-aligned, 300px wide, ends above its bottom-right chip, and clips/scrolls within the window body.
- Action selection uses the existing camera, title, icon, and settle path, then republishes the retained Tree atomically.
- The selected row crosses the retained document boundary through a generation-fenced presence stamp after reconcile and before viewport/layout. It marks both the Tree's authored paint item and its mounted record row while leaving immutable `TreeItemProps` unchanged; candidate accessibility reads `selected: true` from that same accepted row.
- Action-only nested projection leaves remain Tree items by carrying the authored `defaultOpen`; the panel assembler's inline-toolbar rule remains limited to explicit verb leaves without disclosure state.

## Hidden Sibling Retention

`live_window_ids` previously used `dock_window_plan`, which omits hidden siblings during focus/maximize. The surface sync consequently retired the hidden World's scene state, projection seed, icon, title, and camera/fit state as though the window had closed. Retention now uses the committed dock's declared window ids. A focused paint plan may omit a sibling without retiring it; removing the sibling from the dock still retires its owner and per-window state.

## Verification

Focused Nx commands run under the ticket temp directory:

- `@semio-tech/framework-renderer-wgpu:test-wgpu-unit -- the_projection_chip_folds_its_own_pane_and_switches_its_template`
- `@semio-tech/framework-renderer-react:test -- ../../🧱️elements/🌐️World3dHost/🧪️tests/🔀️projection-pane/🟦️.tsx`: **2/2 passed**, Vitest 4.24 s, Nx 5.6 s.

The first WGPU run ended after 9 m 10 s with `E0599`: the renderer compiled against a `semio-framework-ui` rmeta built before the new presence API existed. This is a compile-time mixed-snapshot failure, preserved at `🗑️generated/sol-projection-pane/projection-rust-pre-presence.log`; it executed no assertion. The exact command is rerunning against the coherent source. No native passing result is claimed until it completes. The root agent owns full native/UI/WASM and physical browser acceptance.

The root-owned WGPU23 physical run verified that focus/unfocus restores both declared split siblings and retains the untouched Perspective mesh, camera, and accessibility token. It also exposed a separate retained-paint transaction failure: the Projection pane publishes a 300px body with fifteen bordered row strips and the complete accepted accessibility hierarchy, but no visible labels or icons. Selecting Curvilinear leaves the same blank body. A fresh capture confirmed that its cap does fold the pane after the accepted-frame delay; close activation is not an open defect.

Source tracing explained how this appearance was published. `Ui::frame_into_step` advanced retained paint directly into the shell's caller-owned `DrawList`; `retained_tree_node_step` emits row chrome before its label glyph phase. `paint_window_projection_step` released the chrome walk after 1,024 nonterminal opportunities and recorded a surface fault, so an incomplete direct-output candidate became the displayed frame.

The retained paint entry now matches the atomic ownership of `Ui::frame_step`: every node, scene, overlay and tooltip paints into `RetainedPaintFrame.candidate`. `DrawList::append_retained_candidate` rebases scene-layer, glass-layer and foreground-region indices, inherits the caller's active scissor/clip/glass context, and transfers the balanced candidate only at `Publish`. Hits and the paint census publish in that same terminal opportunity, which returns `Ready`; every `Pending` answer leaves the caller's layers, quads and hit registry unchanged. The 1,024-opportunity bound is unchanged. A monotonic frame-progress scalar now distinguishes work from a true parked opportunity in the Interpreter stall counter.

The first post-change focused Projection run again stopped before assertions, this time after 36 m 18 s in an external `semio-framework-plugin` mixed snapshot: `TableRowBuilder` no longer had `try_children`. The tee log contains no source span and current source contains no such call, so it is recorded as a compile-time dependency snapshot at `🗑️generated/sol-projection-pane/projection-rust.log`, not a Projection result. The exact budget law is rerunning on the coherent transaction source at `🗑️generated/sol-projection-pane/projection-atomic-budget.log`.

## Limits

The React oracle proves the production component's semantics and hierarchy in jsdom. It also confirms that the current React Tree rows have no row-level Enter/Space handler, so this packet does not claim keyboard semantics the authority does not provide. Focus/unfocus camera retention and cap folding are physically accepted in WGPU23. The atomicity source law is staged but no native passing result is claimed until the focused gate completes. Complete pane labels/icons, selection after complete paint publication, final pixel appearance, and platform assistive-technology output remain open until the next root-owned native runtime acceptance.
