# Intro step 8 Select dropdown under veil

## Cause
`SelectContent` portals to `document.body` at `z-temporary` (30). The introduction veil is `z-tutorial` (10000). The trigger sits in elevated chrome (`z-tutorial + 1`), so the click works, but the dropdown paints under the veil and appears not to open.

## Fix
- Stamp `data-introduction-active` on `document.documentElement` while `UIIntroduction` is mounted.
- CSS raises select/popover/hover-card/context-menu content and `[data-radix-popper-content-wrapper]` to `z-tutorial + 3`.
- Context menu surfaces get `data-slot="context-menu-content"` for the same rule (step 9).
