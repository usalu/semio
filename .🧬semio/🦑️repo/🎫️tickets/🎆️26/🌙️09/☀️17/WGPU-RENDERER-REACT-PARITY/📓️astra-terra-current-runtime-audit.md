# 🧭 Current Runtime and Shell-Chrome Audit

## Scope and evidence boundary

This is a read-only source and diagnostic audit. It covers the diagnostic artifacts under `🗑️generated/astra-runtime`, especially `paired-checkpoint-7-settled-2/steps.json` and `parity.md`, plus the current React and WGPU shell paths. No build or test was run in this audit.

The settled-2 diagnostic now proves that closing the last two windows completes on both renderers: both record `{ "closed": 2, "remaining": 0 }`. Its reopen failure is specific and actionable: WGPU publishes three `tree.drag.sort.framework.display.windows/...` controls and no transfer control, then reports `Window template tree.label...kind exposes no transfer handle`.

Do not treat the remaining boot, tour-dismissal, or `registerBrushMesh` action differences in that run as product defects. The current diagnostic still has a late scene-registry publication race, and the root diagnostic work owns that probe limitation. Earlier fixed 450 ms composite waits also made close/reopen observations premature; settled-2 removes the close-last false negative. Fullscreen and later chord observations that occur after a failed reopen are likewise not independent failures.

## P0 — Display tree payload is dropped during panel projection

This is the direct cause of the sort-versus-transfer failure. It is a serialization/projection loss, not hit ownership or stale retained-tree state.

| Stage | Source and finding |
| --- | --- |
| Producer | `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7491-7497` creates the Display kind row with `draggable: Some(true)` and `drag_data: Some(window_template_drag_data(&kind.id, None))`. Projection templates do the same at `:4734-4742`. |
| Loss | `PanelProjection::tree_item` at the same file `:4619-4661` copies `draggable: item.draggable` into `ui_contract::TreeItemProps` but hard-codes `drag_data: None`. |
| Faithful downstream behavior | `🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:582-585` copies contract `props.drag_data` to the retained `UiTreeItemNode`. `🧮️layout/🦀️.rs:156-161` defines `Some(data)` as `Transfer` and absent data as `Sort`; paint therefore emits the observed `tree.drag.sort.*` handle. |
| Input ownership | `⚡️events/🦀️.rs:200-237` finds the retained tree spec by row identity, and the pointer path arms the payload from that spec. It is correctly seeing `None`; changing hit routing cannot restore an omitted payload. |

The minimal correct change is to convert `item.drag_data` to the bounded `ui_contract::TreeItemProps.drag_data` map in `PanelProjection::tree_item`. It must preserve every MIME entry and `None` only when the author supplied `None`; it must not synthesize a payload in layout, paint, or input.

The current generic retained-input test hand-wires a contract `TreeItem` with `dragData`, so it proves only the downstream route. Add one producer-to-render law that:

1. creates the actual Display tree with `build_display_windows_ui`;
2. runs `panel_ui_records`, document publication, reconcile, and hit registration;
3. requires `tree.drag.transfer.framework.display.windows/...puzzle3d-main.kind` with `application/x-semio-window-template` JSON;
4. forbids the corresponding `tree.drag.sort.*` id; and
5. drags it into an empty dock and requires one new active window.

That law spans the exact seam missed by `🧪️tests/🔬️wgpu-shell-input/🦀️.rs`'s manually authored transfer record. `🧪️tests/🪟️window-lifecycle-template-drag/🦀️.rs` already covers the state transition after a valid payload exists, so it should remain focused on dock semantics.

## P1 — Footer sync leaf is incorrectly conditional and lacks its panel body

React always builds three sync utilities even while detached; `buildFrameworkSyncUtilities(null)` is tested to return the detached choices. `🏛️ShellHost/🟦️.tsx:9335-9373` uses them to create `s-sync-status`, whose folded footer label is `Remote: detached` and whose body is `SyncAttachCard`.

WGPU instead initializes `sync_backbone_uri` to `None` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:5513`) and only adds `s-sync-status` when that URI exists (`:7983-7991`). This directly explains the missing detached sync control in the earlier checkpoint. It conflicts with its own `sync_pill` implementation at `:8581-8610`, which intentionally returns `Remote(Detached)` when no status/backbone exists.

There is a second closure gap: `shell_owned_panel_leaves` at `:18385-18394` does not include `s-sync-status`, and `publish_shell_panel_document` at `:18401-18422` has no Sync Attach Card branch. Unconditionally adding only the leaf would make an openable tab with no retained body.

Implement the complete React-equivalent leaf: include `s-sync-status` unconditionally in the bottom-left dock, publish its panel document, and project the existing file/folder/remote and attach/detach state through retained controls. Preserve the detached label as the tab's title; do not move sync controls back into the footer. Extend the shell-owned-leaf closure law with cold `sync_backbone_uri == None`, requiring one bottom-left `s-sync-status` control labelled `Remote: detached`, a body with all three choices, and no legacy footer utility controls.

## P1 — WGPU has no Task Manager dock leaf or body

React constructs `os.task-manager` at `🏛️ShellHost/🟦️.tsx:8791-8796` and includes it in the bottom-right tab list at `:9713-9719`. Its mounted view deliberately distinguishes an unattached runtime from an empty actor list (`🧵️TaskManager/🟦️.tsx:396-405,457-460`). The command opens the same bottom-right path at `ShellHost:9586-9589`.

No `task-manager`, `TaskManager`, `os.task`, or `openTask` occurrence exists in the WGPU Shell target or its Shell tests. This is a source omission, independent of settling and artifact identity.

The correct implementation is a shell-owned `os.task-manager` bottom-right leaf, command routing for `os.openTaskManager`, and a retained body whose unavailable state says that no runtime is attached. Only add actor rows and suspend/resume/cancel controls when the target has an actual metrics/dispatch bridge; an invented zero-row table would misrepresent the React state. Add a dock/command law that requires the leaf in every session, opens its anchor from the command, and distinguishes no-runtime from no-actors.

## Search pane — do not remove the source guard without fresh data

The former long checkpoint saw Search on React but not WGPU. Current WGPU source follows the same rule: it refreshes `window_engagements` only for the Engagements section (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6538-6540`); `window_has_search_spec` returns true for an input or non-empty possibles list (`:17050-17058`); and `build_window_search_ui` correctly returns no body without an input (`:17230-17240`). React passes `windowEngagementToSearchSpec(resolvedEngagement, ...)` in both its kind and instance paths (`🏛️ShellHost/🟦️.tsx:10556-10564,10596-10604`).

That makes the remaining hypothesis a guest-to-WGPU `WindowEngagement` publication/refresh issue, not a pane-chip or layout omission. Before changing the guard, capture the WGPU engagement for the same window and revision as React's reference: `input`, `possibleEngagements`, and the rendered chip ids. A law should feed an input-only engagement and a possibles-only engagement through the actual refresh/publication path, requiring the shared Search toggle; it should then require a body only for the input case. This avoids manufacturing a blank Search pane where React intentionally returns `null`.

## Dock geometry and fixture widths

The desktop split hit extent already matches React's fine-pointer contract: React exports `RESIZABLE_HIT_TARGET_MIN_FINE_PX = 20` at `🖱️ui/🧱️elements/↔️Resizable/🟦️.tsx:18-29`, and WGPU uses `SPLIT_HIT_MIN_PX = 20` at `🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1500-1501`. WGPU registers that 20 px rect centered on each horizontal or vertical divider (`:1549-1571`) and registers dual-axis corner hits. There is no evidence that the current desktop fixture width is a defect.

React increases the target to 28 px for coarse pointers; WGPU currently uses a fixed 20 px target. That is a real parity requirement only when pointer modality is available to the WGPU host. Do not silently widen desktop hits, because it would lose the fine-pointer match. Instead, carry pointer capability into the WGPU layout/input context and select 20 or 28, then add a two-modality geometry law checking divider center, extent, and corner square. Keep visual divider width (`6`) separate from hit extent.

The reported broad chip-width differences are not enough to change dock constants: native glyph metrics, registry settlement, and the giant-gizmo issue are separate concerns. A useful fixture must compare semantic rectangles after the same font/viewport has settled, and identify the control and width source before changing a geometry token.

## Fullscreen and chord routing

The current route exists end to end in source. `ShellShortcut::ToggleFullscreen` calls `apply_os_command` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14598-14605`), that sets `fullscreen_toggle_requested` (`:18180-18186`), the renderer frame turns it into a boolean directive (`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:14372-14375`), and browser boot calls `canvas.requestFullscreen()` or exits it (`🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:549-553`). The shortcut table includes `mod+shift+f`, with a law for reserved routing and latch behavior in `🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs:47-90` and `🧪️tests/⌨️wgpu-chord-example-role-parity/🦀️.rs:69-84`.

The latest seven-step sequence never reaches an independent fullscreen assertion after the transfer failure. Do not patch chord routing from that result. Add one browser integration probe that issues the chord as its first user gesture, observes the worker directive and `document.fullscreenElement`, then repeats it to exit. It should run with no focused field, overlay, sync card, or dock drag because those are intentional idle gates.

## Execution order

1. Fix and regress the panel tree `drag_data` projection seam. It unblocks reopen and makes all subsequent state checks meaningful.
2. Add the complete detached Sync Attach leaf and the Task Manager shell leaf/body. These are proven current-source omissions.
3. Capture the actual Engagements payload before making any Search change.
4. Run fullscreen and geometry checks independently after a semantic-ready gate; add coarse-pointer support only if the host reports that capability.

