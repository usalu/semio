# Previously

GoldenLayout window chrome controls and the splitter add-window overlay used non-Action UI buttons, causing inconsistent interaction styling and tooltip/id behavior.

# Plan

- Render all window-related buttons (window chrome actions and splitter add-window actions) as Action UI elements.
- Ensure GoldenLayout windows forward Action clicks to the layout controls.
- Document the window action control mechanism in README.md and AGENTS.md.

# Changes

- Implemented ActionGroup/ActionGroupItem-based splitter add-window overlay actions.
- Added Action-based window chrome controls for GoldenLayout windows by forwarding Action clicks to GoldenLayout control elements.
- Hid GoldenLayout native control buttons so window actions are presented exclusively as Action UI elements.
- Updated README.md and AGENTS.md to document window action controls.
