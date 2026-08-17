# UI Keybinding Barrel SCC Audit

## Scope

The authored keybinding implementation is split across the framework React barrel's `UiKeybindings` region and detached shell-action/panel-hotkey functions. It also depends backward on `ContextMenu.isAppleUiPlatform`, the barrel-owned `Anchor` contract, and a late `ElementId` import.

## Responsibility and Consumer Evidence

### Keybinding Text Interpretation

`parseKeybindingChords` and `formatKeybindingShortcut` are independently consumed by ChromePanels, World3dHost, and ContextMenu. This is a qualified shared module responsibility.

### Control-Keybinding Context and Resolution

The context/provider, binding lookup, control hotkey resolution, and control-binding hook reach independent terminals including ShellHost, UIDialog, ButtonGroup, ToggleGroup, and authored framework application behavior. This is a qualified shared module responsibility. Internal lookup helpers remain private to it.

### Control-Hotkey Presentation

`ControlHotkeyBadge` has two independent component consumers: ButtonGroup and ToggleGroup. It therefore qualifies as a specific shared UI module. Its shortcut class and inline-visibility helper are private implementation facets. `useControlHotkeyTooltipVisible` has zero active consumers and must be deleted.

### Persistence

Stored override reading is consumed by Shell and writing by ShellHost. This is a qualified shared persistence module responsibility. Its key and parser are private implementation facets unless another production component directly requires them.

### Shell Action and Panel Binding

`useActionHotkey`, `PANEL_TOGGLE_HOTKEYS`, and `usePanelChromeHotkeys` have only ShellHost as a production terminal and depend on the barrel-owned `Anchor` contract. They do not qualify for a shared module. Move them private into ShellHost only in a separate protected-owner lease; they are not required to break the ButtonGroup/ToggleGroup SCC.

## Current SCC

The React barrel imports ButtonGroup, ToggleGroup, UIDialog, and ContextMenu while those components import authored keybinding or chrome behavior back from the barrel. `formatKeybindingShortcut` also imports platform classification from ContextMenu. Context construction happens at module evaluation through `reactHostPort`, so late configuration cannot repair the cycle.

## Required Cycle-Free Ownership

Create specific UI-owner modules rather than new element umbrellas:

- keybinding-text interpretation/formatting, including private platform classification;
- control-keybinding context and binding behavior;
- control-hotkey presentation;
- keybinding persistence.

Each module imports only direct specific leaves and never the React barrel. ButtonGroup and ToggleGroup import chrome presentation directly and the badge from its specific module. UIDialog imports the control binding directly. ContextMenu imports formatting directly. The barrel removes authored definitions and explicitly registers only the public surface required by protected product consumers.

Public options and props must be repository-owned contracts; do not expose `Parameters<typeof useHotkeys>`, anonymous React props, or external-library types. Preserve wire/storage values and keybinding behavior. No product terminal file needs to change for the minimal SCC break because its required public symbols remain explicit barrel exports.
