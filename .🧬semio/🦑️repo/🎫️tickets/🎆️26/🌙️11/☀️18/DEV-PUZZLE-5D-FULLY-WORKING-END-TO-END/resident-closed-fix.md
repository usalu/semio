# Resident "closed" failure fix: review and regression test

## Verdict
The fix (`workspace.loads` counter around `pool.acquire`, sweep and LRU eviction skip `loads>0`) is correct: the decrement sits in `.finally`, so abort and failure paths also release it; `lastUsedAt` is refreshed on completion, so the sweep does not close the workspace right after the load.

One gap refined: the memory-pressure branch of `sweep` closed every workspace when no plan was governed. A load whose requester disconnected is not governed, so it could still be closed. The branch now treats any workspace with `loads>0` as governed (`🌐️host/🟨️.mjs`). The drain path (`drainAndExit`, line about 375) intentionally closes everything and is unchanged; the `closed` check after the load still guards it.

## Test
`🧪️tests/⏳️load-outlives-idle/🟦️.ts` (resident tests dir), fake clock via `now`, direct `service.sweep()`:
1. sweep after the clock passes `workspaceIdleMilliseconds` while the load is blocked: workspace stays, plan answers `plan-reply`.
2. `maximumWorkspaces:1` with a second workspace opened during the first load: both stay, both plans answer.
Old behaviour (reasoned, not executed, to avoid editing the shared file): case 1 sweep closes the workspace, so the status assertion and the plan fail with `closed`; case 2 the LRU loop closes the loading workspace.

## Commands
`repo tools exec --light --lang=en bun -- test ./<resident>/🧪️tests/⏳️load-outlives-idle/🟦️.ts`: 2 pass, 0 fail. The non-light `repo tools exec bun` waited on machine capacity (swap above ceiling) for 5 min and was aborted.
