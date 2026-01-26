# Previously

- The shared `Panel` component did not enforce the panel level background, so panels could appear as border-only.
- The toolbar surface was rendered without a panel background, so it remained transparent.

# Plan

- Make the shared `Panel` component scope its subtree with `LevelProvider level="panel"` and paint `bg-panel`.
- Treat the toolbar surface as a panel: render it under `LevelProvider level="panel"` and paint `bg-panel`.
- Align root docs to state that all panel surfaces use panel level background.

# Changes

- Restored `LevelProvider level="panel"` and `bg-panel` painting for the shared `Panel` component.
- Rendered the Sketchpad toolbar under `LevelProvider level="panel"` and applied `bg-panel` to its container.
- Updated `AGENTS.md` to document panel background enforcement.
