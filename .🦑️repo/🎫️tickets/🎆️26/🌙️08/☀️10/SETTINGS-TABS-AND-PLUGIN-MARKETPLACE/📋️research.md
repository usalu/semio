# Settings And Marketplace UI Research

## Current Structure

- `ChromePanels/🟦️component.tsx` builds General, Theme, and Keybindings as separate `PanelTabNode` leaves.
- `ShellHost/🟦️component.tsx` flattens those leaves directly into the bottom-right dock, so each leaf becomes an individual footer panel toggle.
- Plugins and Extensions are produced by separate panel builders and are also flattened into separate bottom-right toggles.
- Extension catalog rows already carry `extendsHost`, which is the owning plugin id needed to nest extensions under plugins.

## Target Structure

- One Settings branch/toggle containing General, Theme, and Hotkeys leaves as its internal tabs.
- One Marketplace leaf/toggle containing plugin rows.
- Extension install controls stay in Marketplace, while installed/available extensions render as children of the plugin identified by `extendsHost`.
- Extensions whose host is not in the active registry still render under a synthetic host-plugin row, rather than returning to a separate extension section.

## Verification Scope

- Extend the existing renderer Vitest file with structural and interaction coverage.
- Run the renderer's Nx test and lint targets.
- Run a browser/runtime probe if a launchable app can boot without interference from unrelated concurrent workspace changes.
