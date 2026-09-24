# Lane-less history rows

## Decision

A settled typed operation that published no document edit and no shared-config edit **does log one command-log row**, under its declared kind, with no edit id. That is the row `dispatch_emit`'s empty `artifact_mutations` branch already files ("Without Operations"): `View` for `select` and the suggestion verbs, `Mutation` for a reducer that emitted nothing (empty delete, refused import, identical reimport, a staged import chunk). The row is not an undoable edit (`applied: false`, `op_lines` empty).

A faulted operation logs nothing. A `View` whose only durable emission is the per-window config lane logs nothing on both routes, so an orbit / grid / LOD tick does not pad the artifact command history.

## Why the no-row laws were green

`record_settled_typed_operation_command` ran from `retire_typed_operation_unit`, after `take_typed_operation_completion` had already built `history_patch`. The row existed, one command late. Laws that read only the admitting settle's patch saw zero rows.

## Where the row is recorded

`take_typed_operation_completion` records the lane-less row before `refresh_cache` / `history_patch`, so it rides the admitting command. Retirement keeps the same call; `command_logged` makes the second a no-op. `published_window_config` is set when the window-config lane publishes, and the View exception above consults it.

## Coalesced amend

A later tick of a gesture whose edit is already on the command log does not append a second row. It does mark that row dirty, so the admitting completion re-upserts it (`op_lines` grew; one undo step). Without the dirty mark the tick's patch was empty — the same one-command hole, for an amend rather than a lane-less settle.
