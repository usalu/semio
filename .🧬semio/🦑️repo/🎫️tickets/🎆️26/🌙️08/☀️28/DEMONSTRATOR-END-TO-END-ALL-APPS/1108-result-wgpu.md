# 1108 wgpu result

## Edits (01:35, unverified until W-wgpu-1 returns)
- host-base 28 errors: NOT a manifest/lib fault. The whole wgpu partition (636 `cfg(target_arch="wasm32")` sites in 88 files, deps gated `all(wasm32, target_os="unknown")`) is browser-only; a11y2 checked it with `wasm` = wasm32-wasip2, where wasm-bindgen/js-sys/web-sys/semio-framework-async are (correctly) absent. Right check target: wasm32-unknown-unknown (chk.sh `native` mode + `--target wasm32-unknown-unknown`). No Cargo.toml edit, no LOCK NEEDED.
- semio-ui-wgpu-engine (5): ui-state-and-presented-input close_surface_one -> `close_original_effect_step(cx,demand,|effect|failure.step(bytes,effect))`; layout-stepping alias step -> `with_original_effect` result matched (ValueError -> WorkerJobDemandError::Refused), `consume_work(u128)`; dispatch-and-overlay-api `RetainedPaintPhase::Borders => "borders"`.
- dag/host (6): value `🔐️mutex/🦀️.rs` gained `RetireOwned for std::cell::Cell<T>` (reuses the mutex Cursor) for DagPaintState anim-phase cells; `DagHostRetirement` now `next_close_demand(body)` + `close_step(body,effect)->OriginalRetirementPoll`; NodeGraph element `GraphHostRetirement::close_step` uses `close_original_effect_step`.
- decl-staged-form (16): payload/control/body/output cursors are their own retirement cursors (`staged_close!` macro in payload.rs + `settle_close`), `native_retirement_reply!`, `next_computational_work`.

## Debt
- dag-host `🧪️tests/🎛️input` and flow host `🧹️retirement` tests still call the old `DagHostRetirement` grant API (`next_close_*_byte_demand`, `close_step(grant)`) and old `admit_owned_retirement`; not ported.
