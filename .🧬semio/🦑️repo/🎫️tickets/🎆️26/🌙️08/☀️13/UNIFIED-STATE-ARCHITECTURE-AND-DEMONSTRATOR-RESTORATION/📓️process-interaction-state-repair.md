# Process Interaction State Repair

## Scope

Restore the Process 3D app's interaction state after the typed-command migration: example loading,
selection, hover, camera/utility changes, world picking, and right-click menus. Remove the shell
runtime exception reported after utility/tool interaction.

## Findings

1. `Process3dCommand` already contained the app's typed command vocabulary and every handler wrote
   through `Process3dConfigMutation`, but `Process3dPlayApp` did not implement
   `ArtifactApp::command_from_action`. React and wgpu deliver declared action ids and JSON arguments
   at this boundary, so all Process app actions reached the framework default rejection instead of
   the typed command channel.
2. The World3d host emits `contextMenuAt` before requesting an app menu. Process neither declared
   that action nor had a typed command/handler for it, and its `ArtifactApp::context_menu` remained
   the empty default.
3. World mesh/object picking was a no-op; only face ids reached config state.
4. `ShellHost` resolved the active plugin into `program` but tested a later local named `plugin`
   before initialization in both utility and tool activation branches. That temporal-dead-zone read
   caused the supplied `ReferenceError` and the visible error state.
5. Controller ids are not globally unique across a plugin and its extensions. Resolving an action
   by controller id could therefore send Process interaction state to an extension handle instead
   of the plugin that owns the active/spawned session.
6. Framework window instances qualify body keys as `<body>:<window-id>`. A right-click rerender used
   `process.play.main:process-workpiece`, which Process treated as an unknown panel and replaced the
   world with an error message.
7. The manifest gained first-class `InteractionDefinition` and `InteractionRef` fields, but
   `AppBuilder` still discarded them. The partial migration also left `ActionKind::Interaction`
   unhandled in the history panel.
8. The architecture ticket explicitly records A2/A3 presence/transient producers as not started.
   Process rendering currently consumes the typed config lane; moving only this app's writers into
   presence would create two sources of truth because the framework render contract does not yet
   receive `PresenceView`. This repair therefore closes the action-to-config command path rather than
   inventing a transitional mirror.

## Repair

- Added one exhaustive `command_from_action` transport boundary covering all 37 Process commands,
  including the payload aliases used by renderer, panel, and palette callers.
- Added and declared typed `contextMenuAt`, with selection mutations for face/mesh/object targets.
- Made `worldPick` update object selection and clear stale face/object selection where appropriate.
- Added an app-owned Process menu exposing add-step, selected-step removal, undo, and redo.
- Made shell dispatch resolve the program from the target session's `pluginId`, including utility,
  tool, active-session, and spawned-session paths; corrected the two temporal-dead-zone guards.
- Normalized instance-qualified Process body keys before app rendering and added regression coverage.
- Completed the framework app builder's interaction registry: declarations and window references
  now propagate, invalid references/specs are rejected, framework interaction actions/keybindings
  are injected, and interaction commands have a history-panel icon.
- Extended the existing Process test module with command vocabulary/wire coverage, declared-action
  bridge coverage, interaction payload decoding, registry-backed hover-to-render state, and the
  Process context-menu contract.

## Verification

- `SEMIO_TEST_LEVEL=long CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=... bun nx run
  @semio-tech/process-plugin:test-long --skip-nx-cache`: **164/164 passed**, 0 skipped.
- The Process WASM program rebuilt successfully and published into the framework dev plugin modules.
- In the running React app, changed the example from **Timber Beam Joinery** to **Drilled Plate** and
  confirmed the combobox and rendered session remained live.
- Hovered the workpiece and confirmed the visible hover shading changed without a `setHover` failure.
- Right-clicked the workpiece and confirmed the Process **Object Menu** rendered with **Add Step**,
  **Undo**, and **Redo**, while the 3D body remained present.
- Expanded Utilities and activated **Drill**; it remained pressed and the app did not enter its error
  state.
- Post-interaction browser logs contained no `action failed`, `Cannot access 'plugin'`, or
  `Unknown body` errors.
- `git diff --check` passed for all files in this repair.
