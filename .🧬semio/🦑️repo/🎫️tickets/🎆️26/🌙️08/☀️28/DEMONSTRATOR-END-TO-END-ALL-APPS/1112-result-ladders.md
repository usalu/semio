# 1112 ladders

## Verified (lib)
- native W-ladders-14 rc0 and wasm W-ladders-15 rc0: `-p semio-plugin-instance-ladders -p semio-plugin-instance-app` (also W-ladders-8 native, W-ladders-9 wasm).
- Rerun: `"$C/1105b-tools/chk.sh" W-ladders-N Cargo.toml native|wasm -p semio-plugin-instance-ladders -p semio-plugin-instance-app`

## Edits
- maintenance-ladder:547: pass the grant to ownership `close_empty_backing_step`, `debit_original` on Err (no with_original_effect).
- instance-initialization (stores, ephemeral, ephemeral/declaration, registration, registration/close, registration/windows, services): effect-shaped closes via new `services::synchronous` + `close_original_effect_step`; `ControlledRetirement::step(body,effect)`; registry birth via `with_native_receiving` + refuse_scope/post_receiving; `Decoder=(NativeAllocationIdentity,u64)`; NoPresence factory is `SharedValueRetirementFactory<NoPresence>`.
- plugin/🌱️initialization/🌱️genesis: `OriginalStoreBirth::{lease_actor,take_actor,restore_actor}` (actor lease under the caller's own effect, used by stores + registration).
- instance (app): plugin-app-impl media-transfer (new `receive_nested` shape), close-terminal tag close, `drive_framework_reserved_registry_job(.., cx)` (2 call sites, one cfg(test)), `pending_effects` -> `Result<Option<Vec<Effect>>,OriginalTurnFailure>` (render/projections: yield or no grant = `Ok(None)`); close-ladder arms LocalQuery / PresenceLocalReads / LatestWinsKeys (`begin_close` + `advance`) / two `.into()` ambiguities.
- Tests ported: cancellation issuer tests (`OriginalJobFixtureCaller::admit` + `lend_context` + `retire`), guest-contract reactor/context/call output tests (escrow extent receipt, `owner.step(body,effect)`), runtime command custody manifest metadata-tests (`retired_poll`), store testing prelude (`decode(.., None, ..)`), testing fixtures (config mutation `RetireOwned` seed/cursor, config preparation child step, set-label: removed dead pre-port `source_frame/label_*` methods, ported `ErasedSnapshotRetirement::close_step(body,effect)`), instance-construction (sources borrow order, factory `SharedUtf8::admit(.., &mut actual)`, close_actor).

## Not verified / open
- `--tests -p semio-plugin-instance-ladders -p semio-plugin-tests-12` (W-ladders-13) rc101. Remaining reds, all upstream of tests-12 and NOT mine to finish here:
  - semio-os-store-testing-doubles: 108 errors (🏭️preparation, ♻️retirement, 🗄️store, 🧾️callers, 🌱️retained-edit): SnapshotRetirementFactory/prepare_close_state_step trait shape (body param, no `&Arc<..>`), `artifact_retirement_box_close_step(.., body, effect)`, `OriginalCallerSourceParent::admit` gone (callers 201), Sync bounds cascade. Gateway lane crate.
  - semio-plugin-testing-laws: 22 errors (reached only after fixtures compiled).
- The set-label consumer test (`app-typed-command-full-operation/child-operations/laws:364`) still calls `read_from_encoding_source` / `label_into`, absent in the fixture before my edit as well.
- No tests were run (tst.sh not used).
- Tree-wide behaviour changes to review: `pending_effects` now returns `Ok(None)` on yield/no grant (matches the trait doc); close-ladder LatestWinsKeys now drives `begin_close`+`advance`.

STATUS: FIXED-verified W-ladders-14 (native) W-ladders-15 (wasm); --tests BLOCKED by testing-doubles (108) and testing-laws (22) W-ladders-13
