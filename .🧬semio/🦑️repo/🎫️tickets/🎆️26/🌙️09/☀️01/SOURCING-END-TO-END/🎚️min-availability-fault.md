# Minimum availability fault (browser journey step 14): status 2026-11-24 (second pass)

## Confirmed natively
The gesture lead holds. The plain wire forms (`value` as int/float/string, `delta` +/-) pass; the first gesture form (`{"value":6,"gesture":"input:1","commit":false}`) failed natively before any fix with
`derived operation requires its installed semantic factory` (UnsupportedOwner, `derived-replay/🧩️operation` `derived_operation_birth_demand`).

## Root cause 1 (fixed)
A continuous control rides the framework scrub press. A press holding a `PressLeaf::Config` previews its leaf through the Config store's derived replay, which needs the Config store's `replay_preparation_factory`. The sourcing app overrode only the Config one-item preparation factory; its Config owners came from the framework default `bounded_artifact_store_owners`, which installs no semantic replay factory. Query/module/typology/sort never carry `gesture`, so min availability was the only sourcing filter that reached it.

Fix: `SourcingCurationConfigPreparationFactory` now also implements `ArtifactReplayPreparationFactory` (delegating to a bounded `ColdMutationApplyFactory`, the Drawing pattern), and the sourcing app overrides `config_store_owners_source_demands` / `build_config_store_owners` with `bounded_artifact_store_owners_with_shared_semantic_preparation` plus the original-receiving operation wire (same as the document owners).

## Root cause 2 (open, NOT fixed): narrowed to the native receiving of the overlay fold close
Backtrace (force-captured at the first `with_native_retirement_receiving`): `Maintenance ToolOverlay -> OriginalToolMachineFollow::advance -> OriginalToolOverlayFold::advance_input(phase 2) -> close_input -> close_native -> StepContext::with_native_retirement_receiving`.
Probes inside `NativeRetirementDecodeControl::receive_effect` (`🛬️decode/♻️retirement/🦀️.rs`) showed, every turn: `phase_before=Granted`, poll `Complete`, `invoked=false` (the operation closure is NEVER called), `phase_after=Empty`, no error. That is the `Some(false) => frame.cancel_unstarted()` branch of `NativeDecodeControl::receive_effect` (`🏭️construction/📦️effect/🧳️frame/🎛️driver`): the frame's work quote (`prepare_work(maximum_work)`, `maximum_work = cx.fuel_remaining()` = 2,000,000 in `close_native`) is refused by the original work port, the effect is cancelled unstarted and reported as an empty `Complete`. `close_native` then treats it as success with zero work, so the fold retries forever (silent no-op). The refusal is NOT the `remaining_window` check (that probe never fired); it is inside `port.prepare`.
Experiment: quoting the close at `owner.next_computational_work(4096)` instead of the whole fuel (a `close_native_work` variant used only by the fold's semantic close) made the closure run (`invoked=true`) and turned the hang into a typed fault: `original slot read changed its actual Source or Work` (`🧬️retained-clone/📦️original-slot`, `RetainedOriginalRead::authenticate`: the retained input read of the Config derived operation was born under another work binding than the close effect). Reverted (unverified against other apps' close paths); needs the binding of that retained read fixed, not just the quote.
Open question for the fix: why the document lane never reaches this state (its retained read is gone before close) while the Config lane keeps `failed_input`/the slot read alive until close.
Coordination: another agent is adding `dbg_false`/`DBG_FALSE` probes to `🧵️job/⏱️context/🛬️owned/🦀️.rs` and the replica overwrote my edits in `⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs` more than once, so two people are debugging this file at once.

## Swallowed cleanup fault
`RuntimeLiveCleanupJob::advance_source` swallowing the first fault was NOT touched this pass (no fault surfaced here: the stall is a silent no-op, not a fault). The silent `Ok(())` when `done == None` in `close_native` (`⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs`) is the same class of defect: a close that cannot run should be a typed refusal, not a loop to the work limit.

## Laws
- `the_min_availability_filter_settles_under_the_browser_grant_and_keeps_the_instance_alive`: passes.
- `the_min_availability_wire_payload_applies_in_every_number_form`: plain forms, passes.
- `the_min_availability_gesture_forms_settle_after_every_tick_release_and_abort`: `#[ignore]`d, documents root cause 2.

## Command honouring `gesture`/`commit`
Undecided: `sourcing_curation_command_from_action` ignores `gesture`/`commit` and emits an absolute config edit per tick and on release. Once root cause 2 is fixed this is correct (the scrub press holds ticks provisional and commits the release as ONE edit); no change needed in the command itself.

## Defect 2 fix (2026-11-24)

### Root cause
Not one defect but a chain, every link on the Config lane's stalled `ToolOverlay` close (document lane never reaches them because its overlay never holds a Source-born ledger read or a snapshot alias at close):
1. `close_native` quoted the retirement receiving scope at the whole remaining fuel (2,000,000). The scope's body fuel is smaller than its quote, so `prepare_work` refused, `receive_effect` cancelled the frame unstarted and returned an empty `Complete`, which `close_native` read as success: the fold retried forever.
2. The ledger read of the Config derived input was born under a Source-carrying effect (`paid_effect` -> `with_original_source_effect`) but returned under a Source-less `close_original_effect_step`, so `RetainedOriginalRead::authenticate` raised `original slot read changed its actual Source or Work` (a resident read follows any effect that carries a Source).
3. The snapshot alias retirement (`close_alias`) was quoted with a `FactorySharedRetirement` demand (no copy bytes) while `ArcRootRetirement::admit` needs its own admission demand, and ran under a native retirement effect that carries no factory invocation (`factory admission requires its original invocation`); the erased owner it produces has the same invocation need unless it requires original control.
4. `OriginalScrubDispatch::advance` published its decision (and dropped its binding) while its construction admission still held its receipt, leaving a non-terminal dispatch whose `close_step` failed `validate`.

### Fix
- `close_native(cx,d,work,run)`: the quote is the caller's bounded next-step `work` (`next_computational_work(4096)` for the semantic and erased owners, `ArcRoot` admission work elsewhere); it waits (`Ok`) while the turn's fuel is touched and cannot cover it; a `Complete` whose closure never ran is a typed `WorkLimit` refusal, never a silent success.
- `close_erased` (new, one place for both overlay and fold): an owner that `requires_original_control` closes under the native retirement control; every other owner under a plain original effect, which carries the factory invocation.
- `close_alias` runs under a plain original effect and quotes `ArcRootRetirement::original_admission_demands`; `close_demands` quotes the same.
- `close_original_source_effect_step` (new, job crate) = `close_original_effect_step` with a Source-carrying effect; `OriginalToolLedgerRead::close_step` returns its read through it.
- `OriginalScrubDispatch::advance` closes its construction admission before publishing the decision.

### Result
`the_min_availability_gesture_forms_settle_after_every_tick_and_release` (new, split off) passes: ticks (`commit:false`), release, second release. The abort form is NOT fixed and its law stays `#[ignore]`d: after `{"gesture":"input:3","abort":"blur"}` the abort decision is `Some(true)`, the host emission finishes empty (`finish_empty_host` -> `admit_empty`), and from then on `original_command_scope` never goes idle (`shared pending=true`), so the next dispatch exceeds its turn bound. That is Defect 3 (empty-host emission of an aborted press), downstream of everything above; abort alone and abort after a tick both reproduce it.

## Defect 3 fix (2026-11-24)

### Root cause
Not the empty host emission (`finish_empty_host` / `admit_empty` settle and retire correctly): the leak is the scrub dispatch shell. `OriginalScrubDispatch::advance` publishes its decision (`Some(true)` for an aborted press, phase 3) with its binding already dropped, i.e. terminal-empty, but leaves the shell itself in `tools.original_scrub_dispatch`. The shell is only taken by the next `prepare_original_tool_dispatch` pass (`owner.terminal_is_empty()` -> `close_original_retained_step` -> `take()`, which then returns the decision). `dispatch_action_received` checked `source.original_tool_dispatch==Some(true)` right after `prepare_action_typing_before` and went straight to `admit_empty`, so on every pass after the decision was published it short-circuited past `prepare_original_tool_dispatch`, and the shell was never taken. Tick and release publish `Some(false)`, skip that check and reach the real `prepare_action_tool_dispatch`, which takes the shell; that is why only abort leaked. The leftover shell keeps `shared_command_source_pending()` true (`tools.original_scrub_dispatch.is_some()`), so the command scope never goes idle and the next dispatch (the first plain `setFilterQuery`, which starts its own scrub dispatch) spins to its turn bound. Confirmed with a temporary `[DEBUG]` probe of the shared-pending terms: only `scrub=true`, shell `phase=3 binding=none host=none admission=empty`.

### Fix
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs` (`dispatch_action_received`): the empty-source shortcut now reads `source.original_tool_dispatch==Some(true)&&self.tools.original_scrub_dispatch.is_none()`, so a published decision first falls through to `prepare_action_tool_dispatch`, which takes the terminal-empty shell and returns the same decision; the following pass (shell gone) takes the shortcut. The typing-pending branches keep their check (no scrub shell exists there). The `[DEBUG]` probes were removed.

### Test results
- Curation lib (`--test-threads=1`): 105 passed, 0 failed, 0 ignored. `the_min_availability_gesture_forms_settle_after_every_tick_release_and_abort` is un-ignored and passes.
- `semio-plugin-scrub-host`: 1 passed, 2 failed (`original_operation_tag_registry_preserves_every_admitted_owner_when_full`, `original_scrub_host_roster_and_abort_match_independent_cases_without_cloning_or_ambient_custody`): the known pre-existing failures, unrelated to this fix.
- `semio-plugin-tool-machine`: 2 failed (`gesture_issuer::tests::original_gesture_issuer_matches_neutral_admission_and_exact_allocator`, `..._cancellation_keeps_original_decision_and_live_alias`), both in `release_birth` (`🖐️gesture/🎟️issuer/🧪️tests/🦀️.rs:32`) with `native retirement requires its original producer invocation` / `retirement field requires its original producer invocation`. Not caused by Defect 2: the tests close a `SealedSharedBirth` under a bare `EffectBank` effect that carries no producer invocation, while the committed value retirement (`🌱️value/♻️retirement/🦀️.rs`, unmodified in the working tree) requires one; none of the Defect 2 files (tool-run operations, job `close_original_source_effect_step`, `OriginalScrubDispatch::advance`) is on that path. Needs the test fixture (effect bank) to supply an invocation; left untouched.

## Defect 4 fix (2026-11-24): NOT FIXED, not reproducible natively; law infrastructure added

### What was added (test-first)
- `drain_live_cleanup_to_idle` (unit file `✏️editor/🧪️tests/🔬️unit/🦀️.rs`) replaces the blind `drive_live_cleanup`. It drives the same one-item maintenance units and fails after 256 consecutive zero-progress steps (`Progress(default)`, `Blocked`, `AwaitingInput`, or no admitted turn), mirroring `runtime_live_cleanup_nonterminal_status` / `RUNTIME_MAINTENANCE_ZERO_PROGRESS_LIMIT` (`⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`); it returns only on `Complete`. An `AwaitingInput` wait is not idle here (the actor resets its counter on it, which is how a wait can hide). Every existing min-availability law and `wire_forms_settle` now uses it.
- Two browser laws: `the_min_availability_browser_press_drains_its_cleanup_ladder_and_keeps_the_next_command_alive` (tick with ~55-char gesture id and `windowId`, release same id `commit:true` 4.65 s of pinned tool clock later, then `sortTable`; asserts `rendered_config().filters.min_availability == 25` and the sort column) and `..._without_a_follow_up_command`. Both load the real demonstrator contributions pack under the browser grant first and run every dispatch and drain under the browser grant (64 items, 520 KB).
- Result: both PASS natively (107 passed, 0 failed, 0 ignored, `--test-threads=1`). The browser stall is therefore NOT reproduced by: gesture tick+release, windowId, realistic gesture id, hold time (pinned clock), host grant, large contributions pack, a render of the Pool between tick and release, or a held (unreleased) render tree.

### Browser evidence not reproduced
`interactive-job.publication-stalled`: `setFilterMinAvailability` advanced 4096 publication units, `waiting on the tool settlement is not complete (stage, retained items, fuel) [2, 64, 42686878]`. Stage 2 = `OriginalToolSettlementStage::Scrub` (release), items 64 = a full fresh grant, fuel plentiful: so the wait is not budget exhaustion of the turn.
`settle_tool_operation` (`🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs`) returns `Ok(false)` silently in the Scrub stage from: (a) `OriginalPressSettlement::advance` (`🛠️tool-machine/🧵️settlement/🦀️.rs`) phases 3/4 when `child_grant` is `None` (`maximum_copy_bytes`/`capacity`/`release`/`depth` below the quoted demand, never retried differently), (b) `settle_press` while `original_press_settlement` waits, (c) an operation tag that is still absent. These are the only silent zero-progress waits on that path; (a) is the prime suspect (the Config writer demand scales with the retained config the browser holds, the native fixture holds far less).

### Next probe (needs the browser)
The press settlement prints `[DEBUG] press settle #N advance [DEBUG] phase=.. lane=.. index=.. mode=.. closing=.. writer=.. admission=.. yield=.. grant=..` at N<4 and every 512th call (`settle_press` in the tool-machine file, someone else's probe). At the stall the browser console shows the phase and grant: phase 3/4 with the writer demand above `maximum_copy_bytes`/`maximum_capacity_bytes` confirms (a); the fix is then a quoted-demand refusal (typed `WorkLimit`) when the demand exceeds the host's fixed per-turn maximum, or chunking the Config writer.

### Harness findings (relevant to future laws)
- A render after a gesture in `observe_registered_fixture_app` needs `let _arena = crate::editor_unit_tests::context::arena();` (drains built-node pages); without it the fixture arena close panics ("UI arena descendants remain borrowed at lifetime closure") and masks the real error.
- A render right after a dispatch under the browser grant fails with "original fixture render tree release remained pending": the fixture's single render turn keeps the narrowed 64-item grant after the guard drops (the tree release needs more than 61 items and the fixture does not renew the turn). This is a harness limit, not a browser defect; the laws therefore assert through `rendered_config()`.
- No source under `🧰️framework` was changed by this pass (temporary `[DEBUG]` probes in `🧵️instance/👷️jobs/📥️staging/🖼️render/🦀️.rs` were added and fully reverted).

## Defect 5: scrub settlement stall (2026-11-24)

### Typed waits (permanent)
- `OriginalPressSettlement::advance` (`🛠️tool-machine/🧵️settlement/🦀️.rs`) returns `Option<PressWait{phase,resource,demand,grant}>`: `None` = a funded step ran. `funding()` quotes every step against the largest grant any turn can have (`original_turn_ceiling`): a demand above it is a typed `WorkLimit` refusal naming phase, resource and both figures; a demand above only this turn's grant is a named wait.
- `settle_press` / `settle_tool_operation` (`🧵️instance/🖼️render/📥️staging/🛠️tool-machine/🦀️.rs`) return `ToolSettlementUnit{Complete,Advanced,Waiting(PublicationWait)}` (`🧵️retained-command/🔄️full-operation/🦀️.rs`). Context/emission mismatches, the follow-retirement-before-writer wait (demand vs grant, WorkLimit when never fundable), refused-tag owner of another operation and typing mismatches are `Waiting`; an `Advanced` unit that spent neither work nor retained currency is reported as `Waiting("ran no funded step")`. `PublicationWait` gained `phase`/`resource` and a `Display`, so the `interactive-job.publication-stalled` fault now reads `waiting on <cause> [phase '<p>', short of <resource>; stage, demand, grant] [..]`.
- `plugin_grant_shortfall` (ownership) names the first resource a demand overruns; `plugin_grant_funds` uses it.
- `close_native` (tool-run operations): a quote above the turn ceiling (work or grant) is a typed `WorkLimit` refusal instead of a wait forever.
- Tests: `a_grant_shortfall_names_the_first_unfunded_resource_with_its_demand_and_grant` (ownership), `a_funding_wait_names_its_phase_resource_demand_and_grant` (retained-command).

### Native reproduction: NOT reproduced
Added and passing natively (none reproduces the stall): `browser_grant()` now also arms the browser turn fuel (50,000,000; native fixture 2,000,000) for every browser law (the follow's demand scales with remaining work), `the_min_availability_release_sent_while_its_tick_still_publishes_settles_both` (tick and release dispatched unsettled, new `dispatch_wire_unsettled`). A starved-grant refusal law was tried and dropped: 64 KiB of copy bytes already funds every press step, below it the birth fails earlier.

### Browser evidence (rebuilt hub component with the Defect 3 fix and the typed waits)
- Recorder scenario `single` (fill, 0.5 s, Enter commits, blur) now PASSES in the browser: filter applies (2 rows), sort works. The first-tick Scrub stall was not seen.
- Scenario `quick` (fill, blur 90 ms later) still wedges: `plugin.internal.zero-progress` after the release, then every command is refused. Temporary `[DEBUG]` probes (removed) showed the ToolOverlay unit running with a healthy grant (64 items, 520 KB, fuel 49,999,5xx), no action source pending, and the follow stuck: `phase=1 lane=2 closing=true current=false published=false cfg_term=false` with every other child terminal and its close demand (copy 24, release 64, depth 4) fully funded. So the wedge is `OriginalToolMachineFollow::close_step` -> `self.config.close_step` (`OriginalToolOverlayFold`, the Config lane fold) making zero progress each unit, with a stale (`current=false`) follow that must close. This is the same lane as Defects 2/3. The earlier "Scrub stage [2,64,fuel]" stall is the other side of the same wait: `settle_press` waits in `awaits_writer` while the follow cannot retire.
- The silent exits of `close_native` (`⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs`: yield, `!fits`, fuel touched below the quote, scope not opened -> `None => Ok(())`) are the prime suspects for the fold's zero progress; a probe build to name which one fired was lost when the repo cargo lane was garbage-collected.

### Root cause: remaining unknown
Exact condition inside the Config fold close (`close_native` early exits) not yet identified. Next browser run: stage the hub build, run `record-min-availability-wire.ts quick`, and read the new typed refusal (`native retirement close needs ... but no turn grants more than ...`) if the quote is unfundable; otherwise add a `[DEBUG]` at the four early exits of `close_native` (copy the wasm out of the lane right after the build: lanes are garbage-collected).

## Defect 5 outcome (2026-11-25)

- Rebuilt hub (wasm 280,434,870 bytes, descriptor MATCH) staged after `repo run @semio-tech/framework-plugin-web:support` refreshed the shard worker (the bridge had refused with `original-turn.succession-unoffered`).
- Probes at all silent exits (`close_native` early return and unopened scope, `close_original_admission` yield/short returns, overlay fold `close_step` state, maintenance ladder follow unit) were armed. `record-min-availability-wire.ts quick` ×4, `single` and `journey`: no `zero-progress`, `publication-stalled` or `prior-outcome` fault and no probe on a stall path fired. The fold is closed normally (8 folds, one `close_step` each, `closing=false` → set).
- The stall no longer reproduces on the current framework head; the probe change was diagnostic only, so the cure came from concurrent framework changes between the failing (2026-11-24 evening) and this build. The typed waits of the Defect 5 work stay: a future regression names its phase, resource, demand and grant instead of stalling silently.
- All probes removed again (`⏯️tool-run/🎞️tick/🎮️operations/🦀️.rs`, `🔁️fold/🦀️.rs`, `🧵️job/♻️retirement/🎟️context/🦀️.rs`, `🪜️maintenance-ladder/🦀️.rs`).
- Browser journey: en 17/17, de 17/17 (see `✅️end-to-end-checklist.md`).
