# Archive poll starvation: the poll waited behind a refresh settle that held the command lane

## Root cause (host side, TS; not the guest, not the shard worker)
The guest was never the blocker. Temporary `[DEBUG]` probes in the shard worker message handler (run `starve2`/`starve3`) showed every guest turn takes 1-4 ms and the worker's MoreWork drive stops correctly (`steps`, 511 polls, about 0.8 s per crossing). The shell's `PollDocumentArchiveLoad` command never reached the worker for the whole load. Host probes in `runQueuedTurn` (`starve3`): `cmd queued ... op=PollDocumentArchiveLoad` at 01:20:45, `cmd lane acquired after=84146` ms.

The holder of the per-actor command-ingress lane (`serializeCommandIngressForActor`, key `command-ingress:<actor>`) was a refresh-ui settle (`🧩️module-loader/🟦️.ts:252` -> `settlePluginTurn(..., drainOperations=true)` in `🔁️turn-settle/🟦️.ts`). While the load runs the guest answers `more-work` with nothing to publish, so `hasWork()` stays true; `quiesced` is blocked by `outstanding()` (the requested surface is missing during the replacement) and the only slice rule, `settleYieldsToRefreshV1`, needs an operation-progress effect. The loop therefore ran up to `PLUGIN_UI_CONTINUATION_LIMIT` (4096) continuations (the worker log shows back-to-back `surface-visible` turns), holding the lane for the whole load. Queued polls and cancels waited 34 s (Concrete Forest), 65-84 s (boot Demo).

## Fix
- `🧵️turn-dispatch/🟦️.ts`: `commandIngressQueuedV1(actorId)` (pending calls behind the running one on the lane).
- `🔁️turn-settle/🟦️.ts`: `settleYieldsToQueuedCommandV1(acknowledgementsOwed, queued)`; a draining settle now slices (returns) whenever a host call waits and no acknowledgement or deferred event is owed. The post-loop "stopped without publishing requested surfaces" refusal is skipped for a sliced settle (the later drain publishes).
- `🔌️PluginRuntime/🟦️.tsx`: re-exports for the test dependencies.
- Test: `🧪️tests/🔂️settle-continuations/🟦️.tsx` new law "hands the command lane back to a waiting host call unless an acknowledgement is owed" (pure predicate plus the real thunk scheduler queue). Registered through `🔌️plugin-runtime-suite/🟦️.tsx`; `repo typecheck` of turn-settle passed. The suite run via `repo test-file` timed out at 420 s ("Original Script deadline refused", shared bun runner busy), so the new law was NOT executed by the runner; no language-agnostic JSON fixture was added (the rule is scheduler-local TS).
- No worker/shard-worker change. All `[DEBUG]` probes were removed (message handler and handle-turn-drive restored byte-for-byte; the published worker was regenerated without them with `publish-shard-worker.ts`, a worker-only publish script in this ticket).

## Browser results (hub v18, no wasm rebuild needed; vite serves the TS live)
- `starve4` (`process-cancel-load.json`): banner "Document load: Loading document: 29052 of 29058" with "Cancel replay" appeared 6.5 s after the pick (was: never), the click was delivered (`historyEditCancelReplay`), document unchanged after cancel (`["Demo",1]`). Run still ends `fin 1` because the cancel then poisons the guest: `plugin.internal.prior-outcome: runtime live cleanup faulted` (Viewer/Undo refused afterwards). That is the already analysed guest defect in `cancel-replay-cleanup.md`/`cancel-receipt.md`, fixed in Rust and awaiting the coordinator's wasm hub rebuild; not touched here.
- `starve5` (`process-interactions.json`): `fin 0`, example switches None/Demo/Concrete Forest, Viewer, Editor, Actions, Increase, Undo all passed.

## Remaining
Rebuild/publish the hub with the cancel-cleanup Rust fix, then rerun `process-cancel-load.json` for `fin 0`. Re-run the settle-continuations law when the shared runner is free.
