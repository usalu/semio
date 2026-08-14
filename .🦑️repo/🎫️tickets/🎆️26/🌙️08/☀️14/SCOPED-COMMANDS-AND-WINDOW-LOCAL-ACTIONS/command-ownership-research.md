# Command Ownership Research

## Existing architecture

- `CommandDefinition` duplicates ownership in a serialized `CommandScope`.
- Plugin commands live on `PluginManifest`, while app and mode definitions are mixed in `AppDefinition.commands`; `ModeDefinition.commands` only stores references.
- `WindowKindDefinition.actions` stores references into the app-wide action registry.
- React and WGPU independently build OS command catalogs and dispatch non-OS commands inconsistently.
- The React path converts commands into focused-app actions; plugin-owned execution is therefore absent.
- WGPU skips argument-bearing non-OS commands and its native fullscreen implementation is a no-op.
- Command key strings are presentation-only; executable shortcuts use app action keybindings.

## Structural findings

- Plugin command facets are absent.
- All app roots already have command facets, mostly containing typed engine command payloads.
- Only Flow Generate currently has a mode-nested command facet; its five commands are nevertheless registered as app actions.
- Window action facets already exist and are taxonomy-required, but their definitions are still app-owned.

## Implementation decisions

- Derive command ownership from OS/plugin/app/mode containment.
- Address invocations by owner plus local command id.
- Address actions by plugin, app, mode, window kind, and exact window instance.
- Keep command/action effect classification independent from ownership.
- Use structured platform keybindings and one resolver contract across renderers.
- Route fullscreen, palette, chrome, and shortcuts through one OS command.

## Operational note

The configured repo MCP server closes during initialization even after rebuilding the ignored native client. The existing repo-MCP test target also reports unrelated bootstrap-path expectation failures. The ticket was therefore opened through the same repository client CLI as a fallback so all research, logs, and summaries remain attached to a ticket.
