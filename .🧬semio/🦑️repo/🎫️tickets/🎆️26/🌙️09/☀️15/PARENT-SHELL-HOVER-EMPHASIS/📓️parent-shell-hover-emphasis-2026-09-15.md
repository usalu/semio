# Parent Shell Hover Emphasis

## Design principle

Parents are emphasized on hover: when the pointer is inside a **window**, **panel**, or **pane**, that unit’s border stroke uses `--border-emphasized-color` (not `:focus-within`, so clicks do not leave emphasis stuck).

## Implementation (React chrome)

Extended `ShellParentHover` in `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css`:

1. **Silhouette stacks** (`[data-window-silhouette]` — mode dock, `WindowChrome` panels/panes): on `:hover`, recolor **this stack's** `[data-window-silhouette-border]` (`>` direct child only so nested pane chips inside a window do not pick up the window hover).
2. **Chrome frame layer** (`[data-slot="chrome-frame"]` — e.g. mobile layout panel): on `:hover` for any `[data-slot="panel"]` or `[data-slot="pane"]`, not only `mobilePanel`.

Navbar/footer pseudo-edges already followed the same pattern.

## Tests

Vitest CSS contract in `🧪️owned-locale-detector-retirement/🟦️.tsx` updated to require the new selectors and to forbid `:focus-within` on panel/pane chrome frames.

## Out of scope

- wgpu shell window/panel/pane hover parity (navbar/footer already emphasize on hover in `Shell/🎯️targets/🧊️wgpu/🦀️.rs`).
- Golden `Window` without a silhouette stack (border emphasis is on the dock/panel/pane chrome hosts).
