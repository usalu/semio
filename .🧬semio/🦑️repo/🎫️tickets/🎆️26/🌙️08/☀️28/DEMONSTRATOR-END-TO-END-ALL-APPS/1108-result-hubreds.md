# Lane hubreds: hub map W-main-182 reds (pre-733 API)
Order: window-transient, infinite-board-dag-graph, puzzle-jobs, fem-2d-model. Verify: chk.sh W-hubreds-<n> 🌎️hub/Cargo.toml wasm -p <crate...>

## Edits (unverified until the labels below finish)
- window-transient: `transient/🧾️issuer` CapturedWindowMutationIssuer RetireOwned via `retirement_seed` (cursor = FactoryAuthority::new, inline); `transient/🦀️.rs` crate names `semio_plugin_window_config_registry`, `::semio_os_kernel::`.
- dag-graph: `camera-fit-and-entity-screen-json` DagViewportState / DagGraphState RetireOwned via `retirement_fields_type!/retirement_fields_seed!`.
- puzzle-jobs: WorkClosing step_granted(body,effect); JobOutbox::close_step(effect), retire_step(cx)->OriginalTurnFailure through close_original_effect_step, job_close_step through close_original_effect_step; BrowserCaller::admit/retire_step re-built on OriginalRetirementPump + OriginalCallerSourceAdmission (prepare/admit_backing/publish, cancel_birth, retire_roots); no wallet.
- fem-2d-model: diff retained_empty_option_patch! x4; snapshot Fem2dSnapshot no longer Clone; cold.rs effect-shaped through pump turns (admit_from, advance_into/adopt_into/finish_into/take_into, `retire`); native.rs slot api (source_witness, close_into, take_authority_into, begin_close paid by close_into, native_retirement_reply!); binary snapshot + write-pack drive_cursor 6-arg closures with with_allocated_effect / with_effect.
- LOCK NEEDED fem-2d-model Cargo.toml (+semio-framework-job normal dep).

## Labels
W-hubreds-1 (wasm, window-transient only; queued).
STATUS: BLOCKED edits written, wasm verification queued (W-hubreds-1)
