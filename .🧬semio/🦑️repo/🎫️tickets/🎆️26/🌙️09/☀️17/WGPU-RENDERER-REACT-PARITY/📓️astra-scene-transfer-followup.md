# Current Scene Transfer Follow-up

Source audit by Astra on2026-09-20. No runtime test was run for these findings. The old W14 scene report is historical; this note traces current production after W15 changes.

## Proven Remaining Differences

1. Table row transfer is retained only in source `SceneSurfaceState.drag`. `passive_scene_pointer_button` in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1437` clears it when release lies outside that surface (1447–1454), and `scene_transfer_drop_action` is called only with the same source scene (1490). Repository search finds no second production consumer of RowTransfer. React Table `📊️Table/🟦️.tsx:177–190` accepts any application/x-semio payload at the destination host, and rowDragStart213–215 places it in the browser transfer. A row dragged from TableA onto TableB therefore has no WGPU destination route; same-surface helper tests do not cover it.

2. WGPU BlockList explicitly replaces React drag reordering with move-up/move-down buttons. `block_list_plan` at Scenes2625–2690 says so and emits only move button actions; the only production source recognized by `scene_transfer_drag_source` is a palette entry. React `🧩️BlockListHost/🟦️.tsx:52–127` mounts driver-aware step/block sortable handles;232–254 computes `moveStep`,89–95 computes within-step `moveBlock` with dnd-kit closest-center semantics. Current WGPU has no equivalent step/block source/session/drop. The differing extra move buttons also change layout.

## Required Bounded Packet

Use one driver-aware list transfer/reorder authority across source and destination surfaces/windows. Preserve exact React payloads, scoped block sorting, cancellation, source closure, stale-generation rejection, and click suppression after a real drag. BlockList should paint the same handles and controls, reserving token-based geometry rather than adding an alternate action UI. Use a language-neutral fixture and the real React/dnd-kit/data-transfer oracle. Exercise actual Shell/Interpreter ingress, not helper-only calls: TableA→TableB drop, Escape/outside/close cancellation, BlockList step reorder, within-step block reorder, palette insertion, Handle label no-drag, and Surface initiation.

Queued after current critical host/accessibility/shadow integration packets. Do not claim scene interaction parity before this is completed and observed.
