# Fix Puzzle3d setHover Emitting Operations

## Bug

`setHover` (View-kind) failed at runtime with:

`View-kind action 'setHover' must not emit operations`

Surface: `puzzle.3d.play.viewport`.

## Root cause

`Puzzle3dPlayApp::handle_action` always diffs `before` (live store `Value`) against
`serde_json::to_value(plugin Puzzle3dFixture)`. After any real document op apply, the store holds a
`puzzle_3d`-shaped projection (`skip_serializing_if`, optional `camera.projection`), which is not
byte-identical to the plugin fixture serializer. Pure runtime actions (hover/selection/…) therefore
emitted a spurious `SetDocument`/`SetCamera` and tripped the View-kind guard.

## Fix

Normalize `before` through the same plugin typed round-trip before calling
`puzzle3d_document_delta_operations` (and the same for puzzle5d). View-only actions then emit zero ops.
