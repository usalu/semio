# 🔁️ 1107 Hang Audit: Driver Loops That Re-Advance On Pending

Lane `hang`, read-only, static. No cargo, no edits, no git. Paths are relative to `/Users/ueli/Documents/semio/`.
Scanned: `🧰️framework`, `✏️s`, `🌎️hub`, `♻️mit-bestand`. Method: `command rg` for `loop {` and `while` heads in files that mention `Pending`, then a 25-line window per head for step APIs (`advance`, `close_step`, `step`, `drive`, `settle`, `with_original_effect`, `receive_effect`, `*_step(`) and a bound check (`for`, `break`, `turn`, `max`, `budget`, `limit`, `idle`, `deadline`, `stagnant`). The window pass is a heuristic, so the list is thorough for the hot path but not a proof of completeness.

Rule applied (brief W2 and W4, and the 1107 addendum): a loop is a spin risk when it re-calls a step API on `Pending` or `Working` with the same grant, does not change the grant or state, has no turn bound and no idle or stagnation counter, and does not yield.

## Most likely cause of the reported spin

`semio-os-kernel-neural-engine-model` test `original_budgeted_eval_normal_boundary_turns_match_oracle_and_cancel_same_source`.

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/🧪️tests/🦀️.rs:22`: `for turn in 0..1000000`. Each turn computes one grant from `state.next_step_demands(copy)` and calls `original_caller.run(grant, |cx| state.step(cx))`. The turn loop has no progress check. Contrast the close loop on line 25, which has `close_idle<64`.
2. `⏱️evaluation/🦀️.rs:113-117` (`BudgetedEvalState::step`): returns `Ok(BudgetedEvalStep::Working)` with zero progress when `cx.should_yield()`, when the grant is below demand, or when `with_original_effect` returns `None` (quote not admitted, Pending{work:0}). Under the new protocol this is retry-next-turn backpressure. The test re-advances with the same grant, so the 1000000-turn budget is burned at 100% CPU. The `for _ in 0..2` zero-grant probe on the same line does the same within one turn.
3. Line 26 (`for _ in 0..1000000` output drain) and line 27 (registry retirement drain) also have no idle counter. They are bounded only by the turn cap.
4. The per-fixture-row times (rows x copy {1,3,64} x cancel_at 5 values) multiply the 1M cap, so the test looks like an infinite spin.
5. Fix direction: give the turn loop and both drains an idle counter like line 25 (fail after N zero-progress turns), and treat a `Working` with unchanged demand as a refusal instead of a retry in the same turn. Do not raise the cap.

Same shape, bounded only by a large cap and no idle check: `⏱️evaluation/🌊️wave/🧪️tests/🦀️.rs:18` (`for turn in 0..100000`) and `:21` (`for _ in 0..100000`).

## Ranked findings

Risk: H = likely to spin, sync or no yield, same grant. M = unbounded but yields, or only spins on a rare path. L = bounded in practice or returns on Pending.
Pane path: whether it plausibly sits on a demonstrator pane path (not verified by running anything).

| # | Risk | Pane path | file:line | Why |
|---|---|---|---|---|
| 1 | H | no (neural test) | `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⏱️evaluation/🧪️tests/🦀️.rs:22`, `:26`, `:27` | Turn cap 1M, no idle counter, same grant on Working. The reported hang. |
| 2 | H | not checked | `🧰️framework/🔨️modules/📡️replication/🔗️causal/🔀️transition/🔁️fold/🦀️.rs:323-333` (`finish_cold`) | `loop { match self.step(self.next_step_grant(64,4096)..) { Pending => {} } }`. Sync, no yield, no bound. Same grant each turn while Pending. Production cold callers: `…/🔗️causal/🔀️transition/🦀️.rs:420` and `:602`, `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🧬️schema/🦀️.rs:225`. |
| 3 | H | plausible (plugin docs import) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🚪️ingress/🦀️.rs:563-564` | `while !pending.fill_one_page(..)? {}` and `while !matches!(pending.retire_source_step(..), Complete(_)) {}`. Sync, no bound, grant recomputed from unchanged demand. Spins if a page or source step is Pending with zero work. |
| 4 | H | plausible (presence snapshot) | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📸️snapshot/🔤️json/🦀️.rs:58`, `:63`, `:70` | `while admission.phase()!="ready" { effect_step(..)?.ok_or_else(unfunded)? }`. `advance` returning Ok with no progress and no phase change repeats forever. The `None` branch errors, so the risk is the Ok-without-progress branch. |
| 5 | M-H | no (neural model) | `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/🧱️model/📚️dictionary/🦀️.rs:165-177` (`Dictionary::from_value`) | `loop { match cold_input_step(..) { Ok(_) => {} } }`, no bound. `cold_effect` turns an unadmitted quote into Err, so Pending does not loop here, but an Ok step with no progress and no dictionary does. Closing `while !input.terminal_is_empty()` at `:171` is guarded by `moving()` (errors on no progress). |
| 6 | M | plausible (plugin startup) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🏗️startup/🦀️.rs:82` | `while !owner.ready() { epoch += 1; caller.turn(..) }`. Test driver, bumps epoch each turn so the grant is not the same, but no bound. |
| 7 | M | plausible (run root) | `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🧠️wasmtime-node-host/🦀️.rs:205`, `:210` | `while !admit_root_step()? { yield_once().await }` and `while !return_actor_turn(..)? { yield }`. Unbounded but yields, so livelock not 100% CPU. |
| 8 | M | no | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🔌️plugin-instance-handle/🎫️operation-admission.rs:20` | `loop { reserve(..) Pending => yield_once().await }`. Yields. |
| 9 | M | no | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:774` | `while self.active.close_step()? { yield_once().await }`. Yields, unbounded. |
| 10 | M | no | `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:524` | `while self.advance_ready_one() {}`. Terminates when the queue drains. Spins only if advance returns true without progress. |
| 11 | M | no | `🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/🦀️.rs:332-338` | `loop { match cursor.step() { Pending => {} } }`. No grant, no bound, sync. |
| 12 | M | no | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs:124` | `loop { drive_step(..); yield_once().await }` until terminal. Yields. |
| 13 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-drag-history/🦀️.rs:68` | `while let ReplayStep::Pending(_) = replay.step(..)`. Spins if the step stays Pending. Test. |
| 14 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🧪️tests/🔬️unit/🧲️selection-and-retirement-laws/🦀️.rs:592` | `while retire_cancelled_world3d_asset_step(..) {}`. Test, no bound. |
| 15 | M (tests) | no | `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-prepared-unit/🦀️.rs:453`, `:475`, `:476` | `while !close_packet_step(..) {}`. Test, no bound. |
| 16 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🚪️mounted-output-admission-laws/🦀️.rs:59` | `while !outputs.close_step(1, 4096).unwrap().complete {}`. Fixed grant, no bound. |
| 17 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🥾️catalog-bootstrap/🧪️tests/🔬️unit/🦀️.rs:424`, `:437`, `:465` | `while !terminal.terminal_is_empty()` style drains. Test, no bound. |
| 18 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🚦️io-driver/🧪️tests/🔬️db-io-retained-fixtures/🪤️lost-owner-ring-close-interruption-and-queued-callback/🦀️.rs:66`, `:84` | `while db_io_lost_owner_close_step(..) {}`. Test, no bound. |
| 19 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🖥️native/🧪️tests/🔬️retained-turn-fixtures/🦀️.rs:437` | `while control.close_step() {}`. Test, no bound. |
| 20 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🖼️wgpu-ui-image-loading/🦀️.rs:82` | `while retire_cancelled_renderer_asset_step() {}`. Test, no bound. |
| 21 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/📋️accepted-table-editable-text-and-stepper/🦀️.rs:92`, `:95` | `while input.drive_text(..)`. Test, no bound. |
| 22 | M (tests) | no | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🪄️presented-focus-rebases-and-ink-intent/🦀️.rs:220` | `while !close_scene_inte..`. Test, no bound. |
| 23 | M (tests) | no | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🏷️types/📏️shared-border-tests/🦀️.rs:174` | `while !draw.finalize_shared_borders_step().unwrap() {}`. Test, no bound. |

Checked and safe (no spin): `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/♻️failure/👶️birth/🦀️.rs:100` (returns on every Pending), `🧰️framework/🔨️modules/🧵️job/🏃️headless/🦀️.rs:193-216` (stagnation counter, 16 turns), `🧰️framework/🔨️modules/🌱️value/🧪️tests/…/owned-control-laws/🦀️.rs:18` and `🧰️framework/…/🧪️tests/🚦️owned-controls/🦀️.rs:44` (bounded), `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts:337` (deadline), `🌎️hub/🚀️local-relay/🛰️servers/🟦️.ts:269,301` (reads a body under a byte cap), `♻️mit-bestand/🧺️demonstrator/🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts:57` (worklist with visited set).

No TypeScript driver loop in the four scopes spins on a step API `Pending`. The `while (...)` hits in `✏️s/🧑‍💻dev/🗄️stdio` are parsers or generator drains over finite input.

## Counts and caveats

- Unbounded driver loops that call step APIs and have no idle or stagnation guard: 23 entries above (#1-#23). Ten are production helpers (#2-#5, #7-#12) and thirteen are tests or fixtures (#1, #6, #13-#23).
- Not run: nothing executed (lane rule). The neural test verdict rests on static reading of `⏱️evaluation/🦀️.rs:113-122`; confirm with the first lane run by checking whether `BudgetedEvalStep::Working` repeats with an unchanged `grant` at turn 0 of the failing row.
- Not verified: whether #3, #4 and #7 sit on a live demonstrator pane path. I did not trace the pane call graph.
- Scratch files went to the session scratchpad, not the ticket. No file in the ticket was changed.

## Fix rules to apply (from the brief)

1. Add an idle or stagnation counter to every loop in #1 and #2 that fails after N turns with no progress; never raise the cap.
2. A `Pending` with zero work from an unadmitted quote is a typed refusal or a yield to the next turn, never a same-turn retry with the same grant.
3. Production loops in #3, #4, #5 need a Pending branch that returns a `Blocked`/refusal, per W4 (a close that cannot progress returns a typed refusal, never Pending forever).
