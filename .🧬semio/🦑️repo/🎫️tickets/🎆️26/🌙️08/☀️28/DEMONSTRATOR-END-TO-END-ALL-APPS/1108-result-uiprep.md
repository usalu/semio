# 1108 uiprep result (GPU readback / ui-prepared drive stall)

## Root cause (not job work: the prepared job body was never entered)
`[DEBUG]` at the top of `PreparedRenderJob::step_callback` never fired in W-uiprep-6: `drive_step` returned `Preparing` on every turn. The fault was the fixture driver `semio_ui_prepared::testing::observe_prepared_step` (all in `prepared/🧪️testing/🦀️.rs`), four stacked defects:
1. Fresh `OriginalJobReturnOrigin` per call + ONE drive turn: the multi-turn receiver prepare (frame, return-table birth, claim) restarted from scratch each of the 4096 calls (Pending lost its state). Fix: repeat the drive turn (<=64) on the same origin while `Preparing`, fail-fast assert "prepared return origin kept preparing for 64 turns".
2. Return-table/publisher birth never ran: `caller.birth_returns(stock)` is now called right after `admit` (must precede any other turn: "Source deadline is already bound" otherwise).
3. The drive turn's grant was `UI_WORKER_RETIREMENT_POLICY` (1 item, 64 KiB): the per-turn origin-refresh birth chain (`chain_demand`, several items) is never fundable (`grant_admits` false -> `Narrow`), so `admit_return_origin` stayed false forever. The drive turn now uses the fixture `stock` (like every other fixture driver: wfc, procedural).
4. Stale receipt asserts: the body receipt now includes the genuine return claim (> 1 item), so `receipt.fits(budget.retained)` / `fits(UI_WORKER_RETIREMENT_POLICY)` are replaced by `fits(stock)` (the grant the turn really had).
- Test-side contract: world host unit-3 built `PreparedRenderInput::try_new(7, 1, ..)` while the prepared drive runs Generation(3) -> "prepared render generation is stale" fault; input generation is 3 now (`world/🎛️host/🧪️tests/🧪️unit-3/🦀️.rs`).
No production source edit in the job was necessary; the fail-fast stalls (items 1 and the 64-turn prepare guard) replace the 20 s spin by a 0.4 s failure. Without limit raises, AuthoringWallet.

## Verification
- on_a_real_gpu: W-uiprep-21 rc 0 (22 s on the real GPU, was a 20 s spin).
- world host unit-3: W-uiprep-22 14/14 (31.6 s). All [DEBUG] prints and the temporary Debug derive are removed (working tree grep clean).
- semio-ui-prepared own tests W-uiprep-23: lib 1 passed / 1 failed (`closing_shared_border_preparation_releases_in_progress_backing`: stale `receipt.fits(UI_WORKER_RETIREMENT_POLICY)`, it never ran before because the body was never entered). Fixed to `fits(testing::close_source::STOCK)`; rerun W-uiprep-24 is queued behind 14 builds (not yet green).
- Also edited (release note): `prepared/🧪️tests/📏️shared-border/🦀️.rs`.
- Debt: the fixture driver uses a hand-sized turn grant (`stock`); a production worker turn must fund `original_return_refresh_quote` (several items) for the per-turn return-origin refresh, the 1-item `UI_WORKER_RETIREMENT_POLICY` can never fund it.
Rerun: SLOT_HANG_SECS=100 tst.sh W-uiprep-<n> "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/📦️packages/🦀️rust/Cargo.toml" native --features testkit --test world_host_unit_3
Crate: tst.sh W-uiprep-<n> "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/📦️packages/🦀️rust/Cargo.toml" native --features testkit

## Coordinator follow-up (worker grant audit, 11/09 22:50)
Audit of every production driver of the 1-item policies (`UI_WORKER_RETIREMENT_POLICY`, `FRAME_BOOTSTRAP_ROOT_GRANT`):
- `UI_WORKER_RETIREMENT_POLICY` production uses: `ui-state-and-presented-input` close_layout_admission, `mounted_layout` collect-publish-and-close, `ui/runtime transaction`, shell fixtures. All are retirement/close paths on plain `StepContext::new` turns: no return origin, no birth service, no refresh. Not render drivers.
- `FRAME_BOOTSTRAP_ROOT_GRANT` (1 item): flows OsHost -> FrameBuildCursor.retained -> FramePreparation.retained -> `BatchDriveConfig.retained`. `config.retained` is read ONLY by `NativeWorkerAuthorityState::issued_grant` (worker/authority:219), which has NO production caller. The real prepared render step is `step_on_original_caller(cx)`: the job runs on the caller's `cx` with `issued_retained = cx.retained_grant()`.
- The production caller `NativeFrameCaller::dispatch_step` (`wgpu/🎟️native-caller`) runs the body with `cx.with_retained_loan(business, ..)` where `business` = the event stock minus the retirement reserve (33_554_432 items, `NativeFrameRootPolicyV1::initiating_event`), inside the actor entry that services the return birth/refresh. The headless runner uses `HEADLESS_GRANT` (64 items) + its own `birth.service`.
- Conclusion: no production worker/driver funds a render job with a 1-item grant on a Source turn; the 1-item prepared drive was a fixture defect only (fixed below). No production edit made. Residual debt: `FRAME_BOOTSTRAP_ROOT_GRANT`/`BatchDriveConfig.retained` is dead policy for the frame job (only read by the uncalled `issued_grant`); if `issued_grant` is ever wired in, it must be `policy + original_return_refresh_quote`. `original_return_refresh_quote` itself is actionbus3's (native-turn birth) and cx-based, so no cx-free production const can be derived without editing their file.
Fixture change (prepared/🧪️testing/🦀️.rs `observe_prepared_step`): the drive-turn grant is now `budget.retained` (policy) + `original_return_refresh_quote(cx)` (derived by a quote turn after `birth_returns`), receipt assert is against that grant, and the "up to 64 turns" bound is `PREPARE_STAGES` (=3: frame allocation, claim, entry). Source of the count: `OriginalJobReturnReceiver::prepare` has exactly the frame and claim stages (table/publisher birth is done up front by `birth_returns`). Verification W-uiprep-24 (crate) and W-uiprep-25 (unit-3) queued.

## Latest runs (11/10 00:30)
- W-uiprep-24 (crate) and W-uiprep-25 (unit-3, derived grant): 13/14 and 0/2 resp.; every red is the SAME panic in `job/⏱️context/🧪️testing/🎟️caller/🦀️.rs:85` ("original close releases what it allocates", root `retire_step` accounting). That file is actionbus3's (they changed close accounting at 00:20 to "released-retained", after these builds compiled), not mine; the prepared fixture changes themselves reached the close phase (drive + prepare passed).
- W-uiprep-26 (crate, against the tree with actionbus3's 00:20 caller change) is queued; if it is green, rerun unit-3 once (W-uiprep-27) for the final STATUS.

STATUS: BLOCKED on actionbus3's job testing caller close accounting (panic at caller 🦀️.rs:85); rerun W-uiprep-26 queued; last fully green: host unit-3 14/14 W-uiprep-22
