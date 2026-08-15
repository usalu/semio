---
name: Realtime Interaction History
overview: Make the plugin-owned command log stream typed incremental history patches in the same response as every accepted interaction, while transporting precise UI dirty scopes so generator work cannot delay or destabilize history. Render the complete logical history through a host-side windowed projection and isolate per-section render faults so one generator window cannot empty the shell.
todos:
  - id: protocol-history-patch
    content: Add schema-first HistoryPatch, history snapshot cursor, and uiScope to Rust/TypeScript invocation frames
    status: completed
  - id: plugin-history-deltas
    content: Emit one indexed history entry per accepted action and immediate seed/load/ingest backfill deltas
    status: completed
  - id: host-history-projection
    content: Apply patches before effects and render the complete windowed history projection in ShellHost
    status: completed
  - id: scoped-refresh-faults
    content: Honor partial dirty scopes and isolate per-section refresh faults without clearing windows
    status: completed
  - id: interaction-alignment
    content: Update first-class interaction semantics so Interaction actions are distinct live history entries
    status: completed
  - id: verify-realtime-history
    content: Extend existing tests and verify monotonic real-time history plus generator fault isolation at runtime
    status: pending
isProject: false
---

# Real-Time Interaction History

## Confirmed causes

- [`VcsArtifactApp::record_command`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs) records commands in the plugin, but `finish_recorded` deliberately excludes `ActionKind::View` from live history refreshes. Consecutive view and shell commands are also folded, and the panel renders only the newest 300 rows.
- The channel drops `InvocationResult.ui_scope`; [`resolveUiDirtyScope`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts) therefore upgrades every action to a full refresh.
- `handleAction`, `refreshUi`, and generator `flowEvalTick` calls share one serialized WASM worker. History currently requires a second queued render call, so generator evaluation can delay it and a failing window render can fail the whole refresh.
- The active [first-class interaction plan](/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM/📋️master.md) currently says `ActionKind::Interaction` should skip history; this conflicts with the new requirement and must be corrected.

## Implementation

1. **Work in the existing restoration ticket and align interaction semantics.** Continue under [`UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION`](/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION/🎫️ticket.json), which is open and already owns demonstrator restoration. Update the first-class interaction master so every accepted framework interaction is a distinct command-history entry and is never excluded from live delivery. Raw pointer samples remain telemetry; each semantic action dispatched to the plugin gets exactly one row.

2. **Add a typed incremental history contract to the existing schema/protocol.** Extend [`InvocationResult`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs), its TypeScript mirror, and the existing app channel frames with an optional `HistoryPatch` containing ordered entry upserts, sequence/cursor metadata, and current undo/redo/checkpoint state. Carry `ui_scope` on the invocation frame as well, update the existing protocol schema mirrors and golden fixtures, and bump the channel version. Add a cursor-based history snapshot request for initial load and deterministic resynchronization after reconnect; do not create a second authoritative host log.

3. **Emit each history patch with the action response.** Refactor [`VcsArtifactApp`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs) so `record_command` always allocates a distinct sequence and marks that entry dirty instead of folding. Maintain edit/config-edit indexes and an invocation-local dirty-sequence set so ordinary actions emit one inserted row and undo/redo/revert emit only affected row updates. Move seed/load/ingest reconciliation out of lazy UI rendering and into their state boundaries so backfilled edits are immediately available to snapshot/delta reads. Remove the 300-entry view cap from the authoritative projection.

4. **Apply history before effects or UI rendering.** Update the app-channel client and [`PluginRuntime`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx) to decode `HistoryPatch` and `uiScope`. In [`ShellHost`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx), merge the patch into per-session history projection state immediately when the command response arrives, before scheduling `DispatchAction` effects or requesting any affected window bodies. Sequence and cursor checks trigger a snapshot resync on gaps or duplicates.

5. **Render a complete, scalable host-side history panel.** Replace the reserved `framework.body.history` round-trip with a dedicated host projection rendered by the existing shell panel path. Preserve current undo, redo, checkpoint, alternative, filtering, inverse actions, icons, labels, applied state, and accessibility. Keep every entry in the logical ordered projection and window the rendered rows by scroll position with overscan, so histories beyond 300 entries remain available without mounting unbounded DOM. Follow newest entries only while the user is already at the live edge; never steal scroll position while older entries are being inspected.

6. **Use precise refresh scopes and isolate render failures.** Remove history-panel scope widening from `finish_recorded`, because history now arrives in-band. Honor transported partial `uiScope` in [`ShellHost`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx), so selection, hover, camera, and flow-evaluation actions refresh only declared surfaces. In [`plugin_exchange`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs), encode a fault per failed UI section instead of aborting the complete refresh. Keep each section's last-known-good UI, show a scoped retry/error state only for that section, and never clear unrelated generator windows or the history projection.

7. **Cover the mechanism in existing tests and verify the demonstrator.** Extend existing Rust and TypeScript test modules for all action kinds, including `View` and `Interaction`; no folding; more than 300 entries; initial snapshot plus cursor resync; delta ordering; undo/redo/revert row updates; `uiScope` wire parity; and per-section fault isolation. Run the existing Bun/Nx launch targets for framework kernel/plugin/renderer and a forced-fresh demonstrator build. On port 6029, use temporary `[DEBUG]` latency/sequence logs to prove every accepted generator interaction appears monotonically by the next React commit from its command response, while `flowEvalTick` is active, and that an injected section render fault leaves all unrelated windows and history intact. Remove temporary logs, record results in a markdown report inside the restoration ticket, and close the ticket through repo MCP only when all checks and runtime behavior pass.
