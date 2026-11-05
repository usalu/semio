# Mounted Cancellation Audit

Read-only source audit, 2026-11-04. No heavy run/native result.

## Source Reviewed

Lowpoly editor /🧪️tests/🔌️mounted/cancellation.rs and cancellation.json, mounted support, framework fixture dispatch/render helpers, retained-command work and cancellation handler.

## API Findings

PluginApp methods, context LowpolyApp deref, fixture dispatch signature and maintenance/publication calls match existing support settle APIs. Test module is included from mounted root. extrudeDistance is valid: editor /🎮️commands/🔷️mesh-edit/🦀️.rs:34 consumes payload.extrude_distance or that utility parameter. dispatch_fixture_action ensures cancellation issuer birth before handling actions.

Framework cancellation handler validates exact operation/generation plus actor/instance, terminal state and lease before setting user_cancel_requested and cancelling. Result terminal/fault and ACK drains match existing context settle pattern. has_pending_typed_operations includes live operations and effect/event/UI/completion outboxes, so draining while pending is appropriate.

## Actionable Fixture Gaps Sent To Runtime

1. Inspect/render current progress before first tick. Current loop advances maintenance and publication before reading an already-pending progress scope; tiny extrude can pass the intended boundary. Lowpoly work stage zero returns Progress and the following work step synchronously reduces and returns Complete. Source does not establish that one scheduler tick always completes, but the fixture needlessly risks missing its first visible opportunity.
2. Assert snapshot unchanged and operation still live immediately after obtaining the binding. Current successful break skips the bottom assertions.
3. After stale-generation refusal, prove the same live operation remains uncancelled (enabled exact cancel/status), not merely is_err. A faulty implementation cancelling then returning Err would satisfy current staleRejected assertion.
4. String dispatch_fixture_action exercises mounted handle_action, not addressed host appId admission. Comparing rendered scope with assembled controller proves binding value, but test claims should retain this boundary.
5. committed_edits checks count of applied mutation rows only. Thus historyUnchanged is narrower than full unchanged history. Snapshot equality plus this count proves no persisted document mutation in this path, not no audit/command history entry.

## Outcome Assertions

Current source drains result pages with ACK, rejects Fault, requires a Terminal page, drains completion/effect/event/UI, and checks eventual progress removal plus no late completion/result/effect for sixteen ticks. These are meaningful retirement checks. Exact cancellation should follow a confirmed live uncancelled binding before any reducer effect is published. Native execution is still outstanding.

## Coordinator Correction

Current test observes rendered live progress before advancing the host, so it can use the first cancellation opportunity. It separately checks live/unpublished state after obtaining the binding. Stale refusal preserves the same enabled exact control, live operation, identical snapshot and unchanged committed mutation count before the exact cancellation. The neutral output now calls that count committedEditsUnchanged; it does not claim full audit-history equality. Root scoped whitespace check passes; Rust execution remains pending67153.
