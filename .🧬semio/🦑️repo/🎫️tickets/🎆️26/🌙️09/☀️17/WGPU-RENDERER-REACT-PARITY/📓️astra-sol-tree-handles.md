# Retained Tree Transfer and Sort Handles

## Result

The production retained Tree path now consumes the active UI driver's canonical drag mode. The Default/Handle driver paints and publishes one trailing semantic handle and only that physical rect can arm a drag. The Surface driver omits the handle and arms the row. Transfer rows use `move` and preserve their authored payload; sort-only rows use `grip-vertical` and a bounded empty payload. Static and disabled rows do not arm.

The same token geometry owns paint, hit publication, and event gating. Label text, descriptions, inline actions, and inline controls reserve the trailing handle band, so the handle does not overlap existing row content. The glyph uses `ICON_TREE_ROW` and theme spacing; no pixel constant was introduced.

The Shell host boundary now forwards both `TreeItem` and `TreeDragHandle` pointer down/up events into the retained `EventRouter`. This closes the production gap between the already-published semantic hits and the router: a handle can arm and promote through the real Shell ingress, while a label click still commits its published `Activate` selection with the owning `windowId`.

The Display producer seam is closed as well. `PanelProjection::tree_item` now carries each authored `UiTreeItemNode.drag_data` entry into the bounded retained `TreeItemProps` map instead of replacing the map with `None`. MIME keys and payloads remain exact, sorted-map admission is deterministic, and oversize or over-capacity transfer data faults projection rather than silently becoming a sort row.

## Production Contract

| Seam | Final behavior |
| --- | --- |
| Driver transport | Shell passes `chrome_build.driver.drag` through every production `render_ui_document_step` call. Interpreter refreshes `Ui::set_driver_drag` before each retained opportunity. |
| Geometry | `TreeRowMetrics` owns the token-derived handle extent. `tree_drag_handle_rect`, reservation, and control-before-handle helpers are shared by paint, input, and routing. |
| Paint | `retained_tree_node_step` paints the role-specific trailing glyph in a dedicated retained-output step. This keeps status borders, chevrons, and icons inside their existing bounded output reservation. |
| Hit publication | Handle mode publishes `tree.drag.sort.<id>` or `tree.drag.transfer.<id>` as `HitKind::TreeDragHandle`; the row remains a normal Tree hit. Surface mode puts the payload and both-axis drag policy on the row and publishes no handle. |
| Event routing | Shell classifies `TreeItem` and `TreeDragHandle` as retained-router-owned. Pointer-down arms only the driver-owned source geometry. A handle click or drag does not activate/select the row. Pointer movement promotes the admitted source; pointer-up emits the existing commit/cancel commands. |
| Policy changes | Changing Handle/Surface while a Tree drag is pending or active releases capture, clears the source, and emits `DropCancelled` for an active session. |
| Shell semantics | Flat hit ids distinguish label, sort handle, and transfer handle for start/hover/drop routing. Label selection, chevrons, inline actions, and controls retain their existing paths. |
| Shell-owned panel projection | Display Windows rows retain their `application/x-compose-window-template` payload through `panel_ui_records`; retained reconciliation therefore publishes a transfer handle and Shell decodes it into `DockDragKind::NewWindow`. |
| Cursor | Published handles use Grab and captured handles use Grabbing. |

The retained engine invalidates the current window paint/hit generation when the driver changes. All seven `render_ui_document_step` callers have the new argument, including the two direct renderer fixtures.

## Shared Fixture and Oracles

`🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️tree-drag-handles/🔣️.json` is the language-neutral corpus. It covers a transfer row with a window-template payload, a sort-only row, a static row, Default/Handle, and Compact/Surface. It declares visible handle ids and the physical source that may start each drag.

The React engine-contract law renders the real `TreeItem` under `UiDriverProvider` with `DEFAULT_UI_DRIVER` and `COMPACT_UI_DRIVER`. It compares derived roles, actual handle DOM, semantic `data-drag-role`, and the native `draggable` row attribute against the shared fixture. The retained input law reads the same fixture and verifies row payloads, semantic handle ids/kinds, payload ownership, and clipped handle rects.

Native event laws exercise actual `PointerDown → PointerMove → PointerUp` routing. They prove that a Handle-driver label press cannot promote, the canonical trailing handle can promote and cancel, Surface preserves whole-row initiation, and a driver change cancels an in-flight drag.

The Shell host law publishes a real three-record Tree document, paints it under the Handle driver, promotes its retained hit registry into `InputState`, and sends the handle journey through `ShellState::handle_pointer_button` / `handle_pointer_move`. It asserts one live retained drag with the original window-template payload, retirement on release, no label-selection action from the handle, then exactly one scoped `selectTreeItem` action from a label click.

A second host law starts at the real `build_display_windows_ui` producer, opens its real main-kind section, projects it through `panel_ui_records`, publishes a retained lease, reconciles and paints it, and routes the resulting hit through Shell. It requires `tree.drag.transfer.framework.display.windows.main.kind`, forbids the sort id, decodes the original window-template MIME payload, and requires a pending `DockDragKind::NewWindow` for `main`.

## Validation

- Focused React fixture oracle passed: **1 passed, 654 skipped**.
  - `SEMIO_TEST_LEVEL=long NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test --skip-nx-cache -- --run '../../../../🧪️tests/🔬️engine-contract/🟦️.ts' --silent=true --reporter=dot -t 'shared Tree drag fixture'`
- Read-only `rustfmt --edition 2021 --emit stdout` parser validation passed for all edited UI layout, paint, input, event, engine, cursor, WGPU export, Interpreter, Shell, and native law files.
- Read-only `rustfmt --edition 2021 --check` parsed the Shell host repair and its published-document law. The repository has pre-existing formatting deltas, so this was a syntax/parser check rather than a clean formatting verdict.
- No Cargo, native, wasm, or WGPU build was started in this packet. The root-owned native build and paired physical WGPU journey remain the runtime verdict. The paired journey should use the now-published transfer handle and remove its temporary whole-row fallback.

## Source Boundary

The Tree source/API batch is complete and frozen. No Tree call-site or initializer work remains. The queued exact shadow-role migration has not started, so the current compiler can consume a complete Tree contract without observing a partial scene schema.
