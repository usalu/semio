# Scoped Commands and Window-Local Actions Implementation

## Outcome

Command identity and dispatch now follow structural ownership from the manifest through both shell renderers. `CommandDefinition` no longer serializes a scope. Full definitions are contained by `OsDefinition`, `PluginManifest`, `AppDefinition`, and `ModeDefinition`; window kinds contain full `ActionDefinition` values. Owner-qualified command and action addresses are the wire identity, so a local id may be reused safely by different owners.

The filesystem taxonomy now recognizes the same ownership facets. Every OS, plugin, app, and mode has an explicit `🎮️commands` directory, including empty markers, and every window retains its required `🎬️actions` facet.

## Runtime and UI

- Plugin commands have a plugin-program registry and executor rather than delegating to the focused app.
- App and mode commands enter the typed app command channel. Mode dispatch verifies the active mode.
- Window action invocations validate plugin, app, mode, window kind, and exact window instance, then inject the authoritative window instance id into the typed handler arguments.
- Declared mutation/view/shell kinds are retained independently from ownership, with operation-lane checks at command execution boundaries.
- React uses `handleCommand` for non-OS commands and the distinct addressed action envelope for window actions.
- React and WGPU resolve OS, plugin, app, and active-mode commands into collision-free address keys, share the same ordering/visibility/argument/keybinding contract, and clear invalid staged invocations when context changes.
- Global discovery excludes window actions. Focused-window controls and shortcuts use only the addressed window action registry.
- Structured platform keybindings replace command display strings. Resolution precedence is reserved shell shortcuts, active mode, app, plugin, then OS; editable fields suppress command activation.

## Fullscreen

`os.toggleFullscreen` is the single entry point for the navbar control, palette, and shortcut. Browser React targets the owning shell root and reports Fullscreen API rejection. WGPU web targets its mounted canvas, while native WGPU toggles the winit window's borderless fullscreen state. Fullscreen state drives pressed state, icon, localized enter/exit accessible labels, and exit behavior. Defaults are F11 on Windows/Linux and Control+Command+F on macOS.

## Representative Migrations

- Animate `resetGrid` is an app-owned mutation command.
- Flow Generate's five verbs are definitions owned by the Generate mode and are unavailable when that mode is inactive.
- Empty plugin command facets make the absence of plugin-wide behavior explicit where no genuinely app-independent command exists.
- The synthetic multi-owner runtime coverage proves plugin dispatch across app instances, app ownership rejection, active-mode gating, collision-safe local ids, and exact addressed window action targeting.

## Changed Areas

- Manifest schema and generated TypeScript: `🧰️framework/🔨️modules/🚂️manifest`.
- Plugin builders, registries, executors, and existing tests: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.
- React and WGPU resolver, shell host, bridge, input, fullscreen, and existing tests: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer`.
- Fullscreen accessibility strings and state presentation: `🧰️framework/🔨️modules/🖱️ui`.
- Taxonomy rules, discovery validation, and existing tests: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.
- Required empty command-facet markers under `💻️os/🎮️commands` and every `✏️s/🔌️plugins/<plugin>[/🎛️apps/<app>[/🎭️modes/<mode>]]/🎮️commands` owner without commands.
- Flow Generate and Animate Present definitions/aggregators under `✏️s/🔌️plugins/🌊️flow` and `✏️s/🔌️plugins/🎞️animate`.

## Operational Notes

The repository MCP connection remained unavailable, so the required goal association and ticket lifecycle were recorded directly in this ticket folder. No modifying Git command was used. Temporary command diagnostics are retained here; newly added `[DEBUG]` instrumentation was removed before completion.
