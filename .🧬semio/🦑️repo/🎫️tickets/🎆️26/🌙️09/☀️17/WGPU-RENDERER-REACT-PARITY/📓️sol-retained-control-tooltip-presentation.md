# Retained Control Tooltip Presentation

## Boundary

The retained WGPU UI now consumes the existing `TooltipStep::Reveal` clock event without manufacturing an authored overlay subtree. Each window keeps one private accepted tooltip record containing the exact surface token, authored document node id, resolved accepted accessibility label and shortcut, accepted accessibility generation, and presented interaction epoch.

Candidate frames may remap that document id only for geometry. They never read candidate label text. The paint ladder measures the accepted text, applies the shared tooltip top/center placement including collision flipping, and composites `paint_tooltip` through the overlay route after retained document and scene paint.

Pointer/router changes revalidate the record after the presented interaction epoch advances. A different hover leaf, immediate pointer leave/cancel, document replacement, hidden accepted surface, or surface close retires it. A rejected candidate preserves it. A frame opened under an older interaction epoch retires before publication.

The canvas visual does not claim the DOM `role=tooltip`/`aria-describedby` relation; that requires a shared accessibility projection contract.

## Neutral and independent laws

Shared schema and fixture:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/💡️retained-control-tooltip/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/💡️retained-control-tooltip/🔣️.json`

Actual React oracle:

- `reveals the shared accepted label after dwell and dismisses immediately`
- Result: 1 passed, 1 skipped by focused filter.
- Receipt: `🗑️generated/sol-tooltip/react-2.log`

Native WGPU laws prepared for the parent UI run:

- `accepted_control_tooltip_reveals_after_dwell_paints_in_overlay_and_dismisses_immediately`
- `tooltip_keeps_accepted_text_across_candidate_discard_and_retires_after_replacement`

The native laws use the shared placement fixture for top and collision-flipped positions, drive the production overlay route, preserve accepted text against an unaccepted replacement, preserve it after discard, and retire it after accepted replacement and close.
