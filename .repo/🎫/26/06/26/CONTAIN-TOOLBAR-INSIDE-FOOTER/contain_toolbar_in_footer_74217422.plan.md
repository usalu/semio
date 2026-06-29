---
name: Contain toolbar in footer
overview: Make the shared footer toolbar always fit inside the footer's height and stay centered (instead of overflowing above it) for every UI, by scoping the toolbar height to the footer height in CSS.
todos:
  - id: ticket
    content: Open/reopen a repo MCP ticket for containing the footer toolbar (read repo://goals, associate to a goal)
    status: completed
  - id: css-scope
    content: In ui/styling/js/ui.css, scope --toolbar-item-height to --size-medium under [data-slot=footer] so the toolbar matches the footer height in all modes
    status: completed
  - id: css-cap
    content: "Add [data-slot=toolbar-anchor] [role=toolbar] { max-height: 100% } safety cap and keep centered alignment"
    status: completed
  - id: verify
    content: Run footer/styling tests + typecheck and verify runtime in compact and touch (incl. fullscreen auto-hide) that the toolbar is centered inside the footer
    status: completed
  - id: close-ticket
    content: Close the ticket with a summary and list of touched files
    status: in_progress
isProject: false
---

# Contain the toolbar inside the footer for all UIs

## Problem
All UIs (products via `ProductShell`, playgrounds via `PlaygroundView`) render their toolbar through the single shared `Footer` in [ui/react/index.tsx](ui/react/index.tsx), inside a centered `data-slot="toolbar-anchor"` overlay. The footer bar is `h-medium` (`--size-medium`), but the toolbar is sized by `--toolbar-item-height`, which becomes `--size-large` in `.touch` mode. The anchor centers with `align-items: center` and no height cap, so a taller toolbar overflows upward and appears to float above the footer (and pokes out when the footer auto-hides in fullscreen).

## Fix (single source: shared footer)
Edit only [ui/styling/js/ui.css](ui/styling/js/ui.css) (the `toolbar-anchor` rule already lives there, outside `@layer`, so it overrides Tailwind utilities):

- Scope the toolbar height to the footer height so it can never exceed the bar, in every size mode:
  - Add `[data-slot="footer"] { --toolbar-item-height: var(--size-medium); }` (overrides the `.touch` `--size-large`). This makes `ToolbarZone` (`h-[var(--toolbar-item-height)]`) match the footer height.
- Harden the anchor against any future taller content by capping the inner toolbar:
  - Add `[data-slot="toolbar-anchor"] [role="toolbar"] { max-height: 100%; }` (the `UIToolbar` root has `role="toolbar"`).
- Keep the existing centering (`justify-content: center; align-items: center`) so it stays horizontally and vertically centered within the footer.

Because the toolbar then exactly fills (and never exceeds) the footer height, it is fully contained and centered, and it hides together with the footer's `translate-y-full` / fullscreen auto-hide.

## Why this covers "all UIs"
`ProductShell` passes the toolbar via `<Footer toolbar={slotToolbar} />`, and both the platform (`PlatformView`) and `PlaygroundView` build their `slotToolbar`/`UIToolbar` and feed it through that same `ProductShell` → `Footer`. There is no other placement of `UIToolbar`, so the CSS scope applies everywhere.

## Verification
- Run the styling/footer tests and typecheck for the touched package.
- Manually confirm runtime in a compact (desktop) and a `.touch` playground that the toolbar sits centered inside the footer with no upward overflow, including when the footer auto-hides in fullscreen.

## Ticket / repo conventions
- Work inside a repo MCP ticket (read `repo://goals`, reopen/open a ticket); keep any temporary artifacts inside the ticket folder; close with a summary of touched files when done.