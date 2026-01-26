# Previously

- UI borders used a single border color token for both interactive elements and window frames.
- Tailwind usage relied on `border-border` / `divide-[color:var(--border-color)]`, making it hard to distinguish semantic border intent.

# Plan

- Add semantic border kinds: `element` and `window`.
- Map border kinds to design tokens so styling can change centrally.
- Update UI components to use `border-element` / `divide-element` for interactive UI elements.
- Keep the existing window border appearance via `border-window` / window border tokens.
- Document the border kinds in `README.md` and `AGENTS.md`.

# Changes

- Added `--border-element-color` and `--border-window-color` tokens and mapped them to Tailwind color utilities (`border-element`, `border-window`) via `@theme inline`.
- Updated Sketchpad UI components to use `border-element` / `divide-element` instead of the previous single-border token usages.
- Updated developer documentation to define the semantic border kinds.
