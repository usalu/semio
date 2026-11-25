# Preview stall after on-demand activation (2026-11-25)

Probes (one capped headless chromium each, 6018): `preview-stall-probe.mjs`, `preview-stall-wire-probe.mjs`, `preview-stall-census-probe.mjs`, `preview-stall-effects-probe.mjs` (rewrites only the served turn-dispatch module to log every guest turn result's effects; repo untouched). Outputs `🗑️generated/preview-stall/`. No cargo law was run: the gate was closed (two other cargo processes, swap 3.0 GB). No Rust file was edited.

## Proven (live)
1. The guest DOES emit the activation requests: two `publish-event os.on-demand.request` (brep, math) at +3.0 s in the `surface-visible` turn, and a third (math, after the first push) at +39..41 s. So `on_demand_run_effects` and the poll work on this build; the 11-24 claim "the poll never carried the request" does not hold for the 10:05 component.
2. Nothing activates at boot by itself: no manifest fetch or `setContributions` before the manual `semioReactActivateOnDemand`. The boot request reached React before/without an effect (inferred below, item I1).
3. Two pushes cross as command pages (census `command-page` 55 -> 319 -> 636): push 1 finishes by +35 s, push 2 runs about +65..75 s. The "45 s" is the host spacing between the brep push and the math push, not a guest delay. The phase flips `faulted` -> `computing` when push 2 lands (`installed_flow_extensions_shared` then resolves brep).
4. After push 2, across 150 s of turn results there is no `dispatch-action` effect (no `toolRunStart`, no `flowEvalTick`), no `invokeExtension`, and no further `publish-event`. Only `send-message` status frames. Both `setContributions` responses carry `requestedEffects: []`.
5. The Tool runs panel is empty, `cancellable: false`: the host holds no run.
6. Installed pack (plugin 1) holds brep x2 and math x2 rows (duplicate rows by plugin id); `fold_host_flow_extension_contributions` keys by manifest id, so duplicates collapse and are not the cause. Math IS present in this run.

## Ruled out by reading
- Superseded-catalogue drain: runs only inside catalogue reads, never gates a poll or an effect; it cannot explain a missing effect (it costs ms per read, not 45 s).
- Registry retirement capacity (16): not reached with 2 pushes.
- `invokeExtension` is only produced by a tick (`finish_tick`); a tick is only dispatched by the run job; the job only exists after `toolRunStart`. So the missing hop is `toolRunStart` (or the tick before it), not extension dispatch.
- Unit laws for the pure pieces pass by design (`every_run_effects_row...`, `a_parked_preview_asks_once...`); the defect is in the live composition, not the pure functions.

## The guards that produce "zero effects" (all silent), in the order the poll meets them
`preview_eval_run_effects` returns `Vec::new()` with no log when: retain loop errors or does not complete; `windows` empty or graph not servable; `link.requested` is Some (the latch only clears when the host run view reports a started run; the carried identity is `None`, so it can never clear by identity); `owed` false and `restart_owed` false (a latch `armed`, or `unfinished` cleared by `abandon_window_tick`, which `finish_tick` calls on an unservable graph). The editor wrapper additionally maps any `with_mut` error (owner busy, session closing) to an empty list via `unwrap_or_default()`, and the host then lowers the pending-effects arm; with nothing changing after push 2 there is no later refresh, so one lost poll is final.

Candidates, not yet discriminated (marked INFERRED):
- I1 (boot): the request published at +3 s is dropped by `applyOnDemandRequestEffect` when the door `semioReactActivateOnDemand` is not installed yet (`door.semioReactActivateOnDemand?.(...)`), and the guest latch never re-asks for the same (generation, kinds).
- I2 (the stall): the one poll after push 2 returns empty through one of the guards above, then no further poll ever happens. The strongest: stale `link.requested`, or an `armed`/abandoned window latch (status `computing 0/1` means `chain.working`, i.e. some latch is armed/unfinished or `tick_scheduled`, while no run exists), or `with_mut` busy swallowed.
- I3: the contributions route's own carrying call contributes nothing live: at push 1 it did not set the ask latch (the poll had to ask at +40 s), consistent with an empty `windows` roster or `Some(false)` in `Generation3dContributionsWork::step`.

## Discriminator (one rebuild of the procedural component only)
`🗑️generated/preview-stall/preview-stall-debug.diff` adds `[DEBUG] ` lines at every guard (poll roster and unserved, each early return of `preview_eval_run_effects`, carrying inputs, the contributions route's `invalidated`/windows/widgets, swallowed poll fault). Touches generation3d preview-eval and editor only: forces rebuild of the procedural component, NOT flow, brep or math. Read the console after the second push: the last `pere` line names the guard.

## Fix shape once named (law is language-agnostic: extend `🧫️fixtures/⏯️preview-eval-run.json`)
- Latch: a start request addressed to no run identity must expire when the poll next sees `state == None` with the debt still owed, instead of blocking for ever; row `startRequestedNeverLanded`.
- Poll: a `with_mut` failure must not publish an empty poll; return the retained (`None`) answer so the arm stays raised (trait change in `ArtifactApp::pending_effects`), row `ownerBusyKeepsThePollArmed`.
- React: queue a request that arrives before the door exists and flush it on install; row in the `🧭️on-demand-run-effects.json` corpus.
- React must not drive a tick: ticks belong to the run job; React only forwards the one `toolRunStart` effect.

## Why 0/1 units
`PreviewChainStatus::units()` is `(nodes_done, max(nodes_total, nodes_done + 1))` while `working`. The session status census is still `{}` (no tick ever ran), so `0/0` becomes `0/1`: one phantom unit meaning "working, census unknown". It is not a document property.

## Addendum: coordinator steps (same day)
1. Applied `preview-stall-debug.diff` ([DEBUG] lines only, four files; Debug derives of `PreviewEvalRunRequest`, `ToolRunState`, `Fault` verified by reading). Reverse: `🗑️generated/preview-stall/preview-stall-debug-reverse.diff`; pristine copies of the four files were taken before the edit (scratchpad `orig/`). Rebuild agent asked to rebuild procedural before brep; not yet confirmed staged.
2. Door-order hypothesis (I1) is REFUTED by `door-order-probe.mjs` (served-module rewrites only): the door `semioReactActivateOnDemand` is set at 1.0-1.7 s; the guest's `os.on-demand.request` is consumed by `consumeTypedOperationEffects` at 2.4-3.7 s, i.e. AFTER the door exists. No fix of the "queue until door" shape is justified. What is proven: the effect arrives in the `refreshUi` settle (module-loader `refreshUi` -> `settlePluginTurn` -> `consumeTypedOperationEffects`), is never converted by `wireEffectToFriendly` (no call with publish-event) and never reaches `applyHostEffects` (only `loadDocument` does); nothing is parked in `pendingTurnEffects`. So it is lost between the refresh settle and `ownedUiRefreshResponse`/React. Not yet located: leading INFERRED cause is a superseded refresh (`refreshUi` abandons its response when the generation moves) which discards `requestedEffects`; the guest's ask latch then never re-asks. Next probe: log the refresh generation check around `requestedEffects`.
3. Native laws not started: cargo gate was closed (two other cargo runs, swap 3.0 GB at last check).
