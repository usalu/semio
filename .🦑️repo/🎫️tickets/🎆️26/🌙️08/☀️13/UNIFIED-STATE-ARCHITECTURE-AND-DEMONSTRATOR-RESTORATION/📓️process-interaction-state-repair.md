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
5. The architecture ticket explicitly records A2/A3 presence/transient producers as not started.
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
- Corrected both ShellHost utility/tool guards to test the initialized `program` handle.
- Extended the existing Process test module with command vocabulary/wire coverage, declared-action
  bridge coverage, interaction payload decoding, registry-backed hover-to-render state, and the
  Process context-menu contract.

## Verification

Verification is in progress. The first focused Nx run spent most of its 20-minute budget behind
another developer's Cargo target lock, then reached Process and exposed one context-menu fixture
type mismatch; that mismatch is corrected. The renderer quick suite reached no assertions before
its 15-second budget, and its long retry could not start a Vitest worker under the concurrent build
load. Both are being retried after the active workspace builds release their shared resources.
