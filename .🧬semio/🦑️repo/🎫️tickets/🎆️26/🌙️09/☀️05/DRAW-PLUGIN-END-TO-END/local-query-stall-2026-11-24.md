# Local interaction query stall and clipboard no-ops (2026-11-24, guest r36)

Probes (all in this folder, run from the repo root with `PLAYWRIGHT_BROWSERS_PATH=$PWD/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, `timeout 280 repo tools exec --light bun -- "$T/<probe>"`; they patch the served modules with `page.route`, no live source touched):
`probe-local-query-trace.mjs` (every command turn: lane acquire/release, per-continuation guest answer, frames out, job start/terminal), `probe-local-query-unblock.mjs` (adds experiments after 200 idle refusals; `MODE=step|continue|none`), `probe-fuel.mjs` (reads the actor's remaining original stock from the host bootstrap on every receipt; `ITEMSX`/`FUELX` scale `🔣️stock.json` inside the probe only), `probe-clipboard-trace.mjs` (copy/paste/cut with the same tracing, drain tracing, lane census, stock readout; `KEYS`, `WAIT`).

## 1. Who issues the query, and when

`LocalInteractionQuery` is the shell's own read of the local interaction slice: `ShellHost/⌨️registers-and-context-menu` `observeLocalInteraction` -> `plugin.readLocalInteraction` -> `AppChannelClient.readLocalInteractionPages` -> `🏠️local-interaction-query` -> an `AppCommand::LocalInteractionQuery` through `PluginRuntime/🚚️handle-turn-drive runQueuedTurn`, lane `Interactive`. It is refreshed after every command and after every job slice that reports progress (`requestIsolatedJobUiPoll`), so while a heavy isolated job runs there is one query per 32 job steps (about 30 in the first 8 s of one polygon job). Healthy queries are one ingress page: offered 2 or 3 times (`backpressure`, status `more-work`), then `command-complete`, 2 to 8 ms, lane released.

## 2. Evidence of the stall (run `lq1`, `unblock4`, `unblock8`, `fuel1`, `fuel2`; 5 of 8 polygon runs; 3 runs without it)

Sequence just before it (all runs): a job slice returns (`job-terminal ... running 1056` is the slice log), the next `LocalInteractionQuery` takes the lane. The guest answers the first offer of its page with `commandIngress=backpressure`, turn status `idle`, `deferredEvents` none, no effects, and answers the same to every re-offer (105,372 offers in 65 s, about 0.6 ms each). `runQueuedTurn` re-offers without bound (only the 300 s wall at the top of the page loop ends it), so the actor's command lane is held and no job slice, refresh or later command runs ("Add Line" never produces a row).

What the guest does, tested inside the held lane (`probe-local-query-unblock.mjs`):
- A page-less continuation turn (`submitTurn(actor, acks)`, 8 in a row) answers `idle`, no ingress, no effects. So the guest holds no command owner and has no work: it is not waiting for the host to continue something.
- `shardClient.stepJob` on the running job, issued from inside the held lane, is refused at once (1 ms, `original-job.call-refused`; in one earlier run it did not return within 27 s). So the job crossing is refused too: the lane is not a deadlock with the job.
- Pending acknowledgements: 0. Deferred events: none. The effects of every healthy neighbour turn are the ordinary `send-message` frames.

Guest side, read from `⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs` `drive_command_ingress`: with a page and no owner the turn answers `Backpressure(page cursor)` with an idle turn exactly when `turn.resume()` is false (`settle_step`/`lend_step` of the live ingress slot cannot pay: `OriginalActorReturnSlot::pay` refuses when the turn's retained grant has no item, copy bytes or fuel left, or `should_yield`) or `admit_command_origin` is refused. Job steps (`jobs::step_job_original` -> `dispatch_job_step`) are funded by the same source.

## 3. Cause (proven by measurement, mechanism inferred)

The actor's whole retained work is one finite, never refilled stock: `🔨️canvas/🧵️renderer-work/🔣️stock.json` `operation` = 8,388,608 items, 2 GiB copy, 512 MiB capacity, 512 MiB release, 50,000,000 fuel, 1 h. `KernelOriginalBootstrapStock.admit()` issues it once per actor activation; `OriginalOperationSource.receive` subtracts every receipt (`next[axis] -= receipt.spent`, `fuel -= workUnitsSpent`) and `issue()` hands the guest the whole remainder as its turn grant. Turns, quiet command continuations, drain turns and job crossings (step, output page reads, acknowledgements) all draw from it (`component-bridge-source`: `exclusive`, `fundedJob`, `originalSourceJob`).

Measured (`probe-fuel.mjs`, host bootstrap readout; items remaining): boot 8.39 M -> 8.13 M; ellipse job (1,200 steps, 18 s) 7.3 M -> 4.9 M; rectangle 4.7 M -> 3.7 M; polygon job 3.67 M (46.8 s) -> 2.91 M (52.8 s) -> 1.74 M (58.8 s) -> 0.91 M (64.8 s). The stall begins at 62.9 s (items about 1.3 M); afterwards only idle refusals draw it (about 15 k per 6 s). One job step costs about 2,500 items, one quiet continuation turn about 150 items and 150 fuel, a command needs 700 to 1,600 such turns (`setCamera` about 730, `cut` and `paste` about 1,050, `copy` 1,580), so a session of a few commands and 2 to 3 heavy jobs spends the whole item stock.

Causal test: with the served stock changed to `maximumItems` x4 inside the probe (`ITEMSX=4`), the same run (ellipse, rectangle, polygon, line) has no refusal, the polygon reaches the canvas after 21.9 s, "Add Line" produces a row and changes the canvas (1.2 s, 1.1 s), items remaining 24.5 M at the end. One run only for the x4 case; the unmodified stock stalled in 5 of 8 runs (3 runs completed, one with the polygon on the canvas after 7.6 s).

Proven: refusals begin while the guest is idle with plenty of fuel and copy stock but about 1 M items; the host spins on them holding the lane; the same run with 4x items does not stall. Inferred, not proven: the exact guest demand that exceeds the remaining items (candidates: `OriginalCallerReturnPlace::reserve_rows`/`admit_sized` of the ingress slot, the quote `original_turn_ceiling` documents as unfundable); I could not instrument Rust (no component build allowed). Not the cause: the snapshot read lease, retirement or the job's held output slot (the job had no held output, the guest owned nothing, a page-less turn was idle); not a lost page or acknowledgement (nothing pending in either direction). The "host holds the lane so the job cannot release something" deadlock is excluded: the job step is refused by the guest, not blocked by the lane.

Native comparison: `ShardLoop::pump` handles one event per iteration and then steps jobs, never holding an actor for a whole command; and the native relay funds every job request with its own `JobFunds::self_issued(job, ...)` (`🔀️post-turn-relay/🎟️instance-slots-leases-and-cancel-helpers: relay_job_funds`), so a job never draws on the actor's turn stock. The browser worker funds jobs from the actor source.

## 4. Clipboard (`probe-clipboard-trace.mjs`, 4 runs; not the same cause)

- First `Meta+C`: `copy` is dispatched, takes the lane for 1,580 quiet continuations (1.3 to 1.8 s), answers `command-complete`; the outcome is one `Invocation` frame carrying the typed operation id and `Ephemeral`; the settled effects are two `send-message`, no `clipboard-write`, no `spawn-job`. The command returns status `more-work`, so `drainTypedOperations` starts (`drain-call`). In the next 40 s it runs about 29,000 guest turns (epoch 39,474 -> 68,469), spends 4.4 M fuel (48.68 M -> 44.28 M) and 67 k items, and never ends, never yields an effect, a frame or a wake. So no fragment reaches `clipboardFragmentRef`, and the first `Meta+V` finds nothing to paste: no `paste` command is ever dispatched (no lane acquisition in the trace). Nothing is lost: the copy simply has not finished (the effect cannot be emitted before the reserved clipboard job `✏️editor/📋️clipboard/🧵️job` completes) and the `clipboard-write` projection added by `worker-wire-tests-2026-11-24.md` is not reached in the browser plugin path at all.
- `cut` and `paste` (after a copy that never completed) hold the lane for about 1,050 continuations (3.3 s, 5 s under load; the shell prints "command stalled ... continuation 737/16 ..." at 5000 ms) and then answer `command-complete` with only `send-message` frames and no row change. They are slow, not stuck: the lane is released every time in my runs. The `command stalled` text is the shell's stall watch firing during the ordinary quiet-turn ladder, not the page refusal above.
- Common cause with section 3: no. The copy stalls at the start of a session (items 8.1 M, fuel 48.7 M left), where the stock is plentiful, and its drain never terminates; section 3 needs a depleted stock. What they share is the cost model: the host drives every guest slice as a whole turn that is charged from the same finite stock, so a never-ending drain (4.4 M fuel in 40 s: the 50 M fuel is gone in about 8 minutes of an idle stuck operation) and ordinary use both consume it.
- Why the copy operation does not finish is not found: its drain turns emit nothing observable. Candidates in Draw: the clipboard job needs a `SnapshotRead` lease (`Workspace.source`/`unsealed`/`sealing`), which is a guest-internal lease; nothing in the host trace shows who would release it. Not verified.

## 5. Fix in this change (host TypeScript, live)

`PluginRuntime/🚚️handle-turn-drive/🟦️.ts`: a command page the guest refuses with `backpressure` while idle (no deferred events, status `idle`) `PAGE_IDLE_REFUSAL_LIMIT` = 256 times in a row now fails the command with the typed fault `plugin.command-page-refused ... its retained stock cannot fund the page` (the page loop previously spun up to the 300 s wall holding the actor lane). The lane is released at once, so job slices, refreshes and the stall watch run again and the person sees an error instead of a dead editor. Healthy refusals repeat a few times at most (measured: 2 to 3), the counter resets on any other answer.

This does not fund the actor: after the fault the actor is still out of items, every later crossing is refused. That needs a change in the worker (`component-bridge-source` `fundedJob`/`originalSourceJob`, `retained-turn` `OriginalJobFunds`, embedded into the worker by `originalOperationWorkerSource`), which is not live without the coordinator's rebuild and which I could not verify here, so I did not make it:
1. Fund every job request with its own standalone funds as the native relay does (`new OriginalJobFunds(job, now, wall)` without `turn`/`drive`, the path `OriginalSelfIssuedTurns` already has), so jobs stop drawing on the turn stock (about 70 % of the spend in the polygon run). Needs a rebuild and the guest's `original_job_binding` (job identity = operation of the start crossing) checked with it.
2. Give a heavy command's quiet continuations and the drain a quiet-turn backoff (the command loop has one for 15 s, then 25 to 250 ms; `drainTypedOperationTurns` has none and spent 4.4 M fuel on a stuck copy).
3. Make the guest answer a typed `plugin.original-stock-exhausted` fault when `retained_grant`/fuel is zero instead of `Backpressure`+`idle`/`call-refused`.
Changing `🔣️stock.json` would hide it and is not a fix.

## 6. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts` (`PAGE_IDLE_REFUSAL_LIMIT`, idle-refusal counter in the page loop).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧵️turn-dispatch-lifecycle/🟦️.tsx` (test `faults a command page the idle guest keeps refusing instead of holding the actor lane`, 30 s timeout because the first host yield after the preceding test takes 5 s in bun).
- This note and the four probes above. No Rust, no stock, no component build touched.

## 7. Commands and results

- `SEMIO_TEST_LEVEL=long repo tools exec --light bun -- test -t "idle guest keeps refusing|reoffers a page" ./<engine>/🧪️tests/🔌️plugin-runtime-suite/🟦️.tsx`: 2 pass, 0 fail (the other is the existing page-reoffer law). The new test alone: 1 pass. Offers recorded before the fault: exactly 256, all page 0.
- Full PluginRuntime suite (`SEMIO_TEST_LEVEL=long ... bun test ./<engine>/🧪️tests/🔌️plugin-runtime-suite/🟦️.tsx`, output `🗑️generated/vitest-full.txt`): started, still running after 15 minutes at host load 14 (bun prints at the end); no verdict yet.
- `probe-fuel.mjs` `ITEMSX=4`: no refusal, polygon on the canvas 21.9 s, line row 1.2 s (1 run). Unmodified stock: stall in 5 of 8 runs.

## 8. Not verified

- The full PluginRuntime suite verdict (see section 7). The change is one counter inside an already tested loop; the two directly affected tests pass.
- The rebuilt-guest or worker fixes in section 5 (1 to 3), the Rust-side typed fault, and any run with the page-refusal fault on the live server: after the fault the polygon still does not reach the canvas in a stalled run, so `probe-latency.mjs` (5 runs, polygon on the canvas, 4th add) and `draw-journeys.sh` are NOT made green by this change. I did not run them after it.
- The exact guest demand above the remaining items; whether a release guest spends the same items; why the clipboard operation never completes; the x4 stock result is a single run.
