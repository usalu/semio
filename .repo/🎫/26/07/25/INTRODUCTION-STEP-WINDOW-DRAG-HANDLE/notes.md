# Introduction Step Window Drag Handle

## Change
- `UIIntroduction` info box header row: title | `DragHandle` (`data-slot="introduction-info-box-drag"`) | step count.
- Pointer drag via `usePointerDrag` overrides auto/`placement` position for the current step; resets on step change.
- Handle lives inline in the header (not absolutely on the top edge) so it is not clipped.
