# Native mechanism (Rust: border-presentation, wgpu, TUI)

## Changes
- `📏️border-presentation`: new `states.rs` (`resolve_box_states`, `own_box_state`, `BoxStateMarks`, `BoxStateInput`, errors, allocation demand) and `silhouettes.rs` (`box_silhouette_source_segments`, `box_silhouette_segments`, `box_segment_id`, demand). Empty edge set is valid: borderless box draws nothing at rest and all four sides when stated. `SharedBorderState` gained `Invalid` (between Active and Waiting); marks are a u16 set. Tests: `states.test.rs`, `silhouettes.test.rs` (JSON cases via include_str!, oracle closure invariant, row-of-cells oracle, borderless law). Obsolete `native-seams.test.rs` deleted (its `nativeSeams` cases left compositor.cases.json).
- Dock `shared-borders.rs` (product) parses `invalid`.
- wgpu: `NodeFlags::HOVER_OWNER` (bit 15); `EventRouter::update_hover` resolves the chain through `resolve_box_states` (interactive = control kinds, activatable stack, interactive disclosure; disabled passes hover up); `HOVERED` still marks the whole chain for hover-reveal. Painters use `hover_owner(flags)` (control-nodes, reference, stack frame); `presence.hover` sets both flags. Focus was already leaf-only and keyboard-only (`focus_ring_visible`).
- `presence_overlay`: selected ring is now on the box's own seams (was outset 1 px). Refused input paints `Invalid`.
- TUI Window chips: hover belongs to the chip only when the pointer is on the chip itself (Close/Maximize/NewTab no longer hover the chip); a hovered chip closes its perimeter including the seam interior in its own colour.
- Laws: wgpu (`✍️input-caret-and-focus-ring-paint-laws`): hovered cell in a group closes its perimeter, no parallel line, group normal; selected presence stays on seams; non-owner chain node paints no hover. Events: hover owner law. TUI: hovered chip closure law, chrome emphasis oracle updated.

## TUI active chip closed
- `paint_tab_seam` and the neighbour-joint seam now close every chip on the seam in its state colour (active or hovered; junctions are `┴`/`┬`/`├`/`┤`, never bends). Updated all fixtures at once: pointer-routing golden row 3, window-contour oracle (no opened seam, state-coloured interior) and feature, chrome-emphasis oracle and feature, and the unit window laws (renamed `window_hairline_closes_under_every_tab`, `window_bottom_hairline_closes_under_every_tab`).
- `native-seams.test.rs` coverage: two adjacent stated cells resolve to one seam in the higher state is covered by silhouettes case 3 (active vs hover) via `silhouettes.test.rs`; the rtl mirroring was geometry only. No extra test needed.

## Final test counts (repo cargo ... test --offline)
| crate | passed |
|---|---|
| semio-ui-border-presentation | 13 |
| semio-ui-draw-contract | 23 |
| semio-ui-wgpu-paint-laws | 72 |
| semio-ui-wgpu-events-laws | 90 |
| semio-ui-tui-unit-laws | 100 |
| semio-ui-tui-laws | 102 |
| semio-renderer-wgpu-dock-laws | does not compile: `shared-borders.rs:66` passes `&[]` where `split_axis_extent` now wants `&Vec<usize>` (unrelated to Invalid/HOVER_OWNER; `Invalid` parse added to Dock `shared-borders.rs` itself) |

## Left
- wgpu has no stateful window chips; native Tree rows paint themselves.

## Priority change (normal < active < invalid < waiting < loading < introduced < celebrated < focus < hover)
- `SharedBorderState` declaration order (the resolver and the wgpu draw list use the derived `Ord`), `own_box_state` priority list, and the Dock `shared-borders.rs` `BorderState` follow the new order. No other rank tables exist natively; the TUI chip roles only rank hover over active.
- wgpu painters: checkbox, control-nodes and reference controls now choose hover, then focus, then pressed/selected (focus used to lose to selected). The selected presence overlay resolves under a control's own focus on the same seams.
- New laws: `a_focused_cell_next_to_a_selected_cell_keeps_its_full_perimeter_in_focus_colour` (wgpu paint laws) and `box_silhouette_focused_cell_keeps_its_perimeter_beside_a_selected_cell` (border-presentation).
- NOT RE-RUN: the machine disk is full (552 MiB free; `repo` needs 5.5 GiB and refuses to start, plain cargo is refused by the compiler ingress). Counts from the previous run (before this change): border-presentation 13, draw-contract 23, wgpu paint 72, wgpu events 90, tui unit 100, tui laws 102; Dock laws pending after the coordinator's `Vec::new()` fix. Expect +1 in border-presentation and wgpu paint.
