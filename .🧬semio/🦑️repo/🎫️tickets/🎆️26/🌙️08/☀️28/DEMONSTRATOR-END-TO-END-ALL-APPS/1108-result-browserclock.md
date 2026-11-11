# 1108 browserclock result

## Design (as ruled)
- `OriginalSystemClock` is now the one target-neutral type (no cfg on the type, its impls, job-clock re-export, job-context re-export).
- Internal port `MonotonicPort` (`job/clock-identity-and-budget/⏱️recorded/🧭️monotonic/🦀️.rs`): `mark() -> Result<Mark,ValueError>`, `elapsed_micros(origin,now) -> Option<u128>`. `SelectedMonotonic` = `OperatingSystemMonotonic` (std Instant; native + wasip2) or `BrowserMonotonic` (`🌐️browser/🦀️.rs`, gated `all(wasm32, target_os="unknown")`, `performance.now()` via js_sys/Reflect, cached receiver, microsecond u64 mark, refuses with InvariantViolated when unavailable; no fallback, no zero clock). Nothing from js_sys is exported.
- Retirement cursor leaf of the clock now carries `Option<Mark>`.
- Cargo.toml: `⏱️recorded/📦️packages/🦀️rust/Cargo.toml` gained `[target.'cfg(all(wasm32, os unknown))'.dependencies] js-sys, wasm-bindgen (workspace)`. LOCK NEEDED logged.

## Tests
- language-agnostic: `🧪️tests/⏱️recorded/monotonic.{json,schema.json,test.ts}` (ajv + sqlite oracle), Rust `monotonic_port_elapsed_matches_the_language_agnostic_cases` and native `selected_monotonic_source_is_ordered_in_microseconds` in recorded `🧪️tests/🦀️.rs`; existing `system_clock_reads_its_actual_source...` now compiled on all targets.

## Verification
- W-browserclock-3 (hub manifest, wasip2, -p semio-framework-job-clock): rc0. Manifest fix per coordinator: recorded-clock deps are explicit `js-sys = "0.3.83"`, `wasm-bindgen = "0.2.106"` (no workspace=true) so every workspace resolves.
- W-browserclock-4 (shell-session manifest, native + `--target wasm32-unknown-unknown`, -p semio-framework-job-recorded-clock -p semio-framework-job-context): rc0. The two job-context errors are gone.
- W-browserclock-7 (recorded-clock own manifest, native tests): 12 pass incl. monotonic_port_elapsed_matches_the_language_agnostic_cases, selected_monotonic_source_is_ordered_in_microseconds, system_clock_reads_its_actual_source...; 1 FAIL `erased_host_admission_denial_keeps_the_actual_source_owner` (Host(u8) panicking producer through admit_typed_controlled_retirement; unrelated to the clock source, in retirement/erased admission; not investigated).
- Extra fix: semio-ui-wgpu-hosting Cargo.toml had `wasm-bindgen`/`web-sys` as `optional = true` with no feature, while sources use them unconditionally on browser (5 errors). Now non-optional. LOCK NEEDED logged.
- W-browserclock-9 (shell-session, browser target): hosting, job stack, ui-wgpu-engine etc. compile. Only red: `semio-os-store-sync-wasm` 8 errors (`semio_os_store_sync::now_ms` import, `acknowledged_edited_head`, 3-arg call, `RetainedWorkOutcome::unwrap_or_else`, `BackboneMessage::encode_op`) = the store-sync grant-to-effect port (mediaslots lane owns store-sync). The shell-session crate itself and navbar `active_locale()` are not reached until that is green; a11y navbar change untested on the browser target.
- Earlier W-browserclock-5 hit a peer's transient "Unbalanced Rust source delimiter" in decl-media (decltail edit in flight).
- Not done: wgpu shell crate on the browser target beyond the above (blocked by store-sync-wasm); wasm runtime execution of the clock (no browser harness; compile check only).

Rerun: `"$C/1105b-tools/chk.sh" W-browserclock-<n> "<repo>/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚶️shell-session/📦️packages/🦀️rust/Cargo.toml" native --target wasm32-unknown-unknown -p semio-renderer-wgpu-shell-session`

STATUS: BLOCKED shell-session on the browser target behind semio-os-store-sync-wasm (8 errors, mediaslots lane); browser clock itself FIXED-verified W-browserclock-4
