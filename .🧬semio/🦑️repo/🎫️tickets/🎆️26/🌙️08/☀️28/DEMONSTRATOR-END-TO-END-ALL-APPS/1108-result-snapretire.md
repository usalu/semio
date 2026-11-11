# Lane snapretire: snapshot receive close needs a producer invocation

## Cause
`retire_decode` / `retire_encode` (`io/⏱️control/🛫️snapshot/🦀️.rs`) hand `ControlledRetirement::step` / `close_factory_ticket` an effect built by `native.receive_effect` / `effect_allocated`, which has no producer invocation. Owners closing through `native_retirement_reply!` (`Fem{2,3}dReceiving::close_into`) refuse that.
`retire_inputs` is the single `retire_effect` caller, so both natives share the fix.

## Edits
- `semio-framework-value-control-contract` `🏭️producer/🦀️.rs`: new public `with_original_invocation(source, effect, call)` (resident synchronous producer action; `drive` + free `transfer_reply` + `finish`, no extra work or bytes). Re-exported from the contract `🦀️.rs`.
- `semio-framework-job-context` `⏱️context/🦀️.rs`: its private `OriginalInvocationCall/Producer` copy removed; both `with_original_effect` sites call the shared helper.
- `semio-framework-io-control` snapshot `🦀️.rs`: `invoked(effect, operation)` wraps the retirement turn in `with_original_invocation`; used by `retire_encode` (`run`) and `retire_decode` (`receive_effect` closure). Other close sites in io (`sqlite-snapshot`, `octets`, `write`) have no `.step`/`close_*` calls on retirement owners (grep), so nothing else needed.

- Layered faults found behind the first one (each surfaced only after the previous was fixed):
  1. `NativeSnapshotReceiving` lacked `native_header_release` (terminal ticket refused: "no explicit native header lifecycle"); added (terminal-empty only, extra 0/0 like every `ControlledRetirement` owner).
  2. fem 2d/3d `receive_authoring` always installed its own `OriginalStepRecipient` via `with_construction_recipient`, which the decoder refuses when an original job recipient is already installed ("cannot substitute its original receiving authority"). New `NativeDecodeControl::has_construction_recipient()` (`value/🛬️decode/🏭️construction/🦀️.rs`); both fem natives reuse the installed recipient and only fall back to their own when none is installed.
  3. fem 2d/3d `advance` quoted the source admission step with the constant `ACTION_WORK=4`, but the `lease` phase of `RetainedCloneSourceAdmission` needs more (`advance_work_demand`), so the admission returned zero progress and the snapshot loop refused ("cannot admit its frontier"). The effect macro gained a `units;` arm and the admission step is now quoted with `admission.advance_work_demand()` (quote = actual cost).

  4. `StagedSourceAdmission` had no forward work quote; added `advance_work_demand` (`value/retained-clone/⚙️engine/🔗️source-owner/🦀️.rs`, birth 1, clear 1, ready = take_output demand, else the inner admission's).

## Verify
- Native `🌎️hub/Cargo.toml -p semio-framework-io-control -p semio-framework-job-context` rc0 (W-snapretire-1); wasm32-wasip2 same crates + `semio-framework-value-control-contract` rc0 (W-snapretire-2).
- 3d `--test original_fem_receiving` (W-snapretire-t2..t11): the producer-invocation error is gone. The run now fails further in, at a different fem-owned fault (t11): 75 items received, then `original projection must be driven or returned through its resident frame` from `RetainedCloneSourceAdmission::advance` (`retained-clone/🔗️source/🎟️admission/🦀️.rs:65`, phase `projectionStart`). The staged source admission now needs its projection driven by `OriginalProjectionProducer` (new API) between `identity` and `leaseInitialize`; fem `advance` phase 0 still calls plain `admission.advance`. That port belongs to fem3d (2d identical); not done here.
- Non-fem snapshot receive test: none could run. `semio-framework-io-control` own test targets are stale against the new retirement-contract API (38+28+6 errors, e.g. `semio_framework_value_retirement_contract` missing in dev-deps, `admits` 5 params; W-snapretire-t13), and the writer sqlite snapshot tests (W-snapretire-t14) are blocked by `semio-plugin-publication-mounted` (10 errors, factoryticket lane).
- Peer reds met on the way (not mine, waited out): `ReplyStash` in the factory crate and `semio_framework_value::close_owner` missing in `semio-os-history-record-decode` (both factoryticket mid-port, resolved by them).
- No `[DEBUG]` left (grep clean).

## Rerun
- libs: `"$C/1105b-tools/chk.sh" W-snapretire-<n> 🌎️hub/Cargo.toml native|wasm -p semio-framework-io-control -p semio-framework-job-context`
- fem: `"$C/1105b-tools/runtime/tst.sh" W-snapretire-<n> "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.toml" native -p semio-s-artifact-fem-3d-model --test original_fem_receiving` (same with ◻️2d / 2d-model)

## Debts
- fem 2d/3d: drive the source-admission projection (`OriginalProjectionProducer`) in `advance` phase 0; then the retire path (this lane's fix) is exercised for the first time by those two tests. 2d edits (`has_construction_recipient` branch, `units;` effect arm) are compiled (W-snapretire-t10 compiled 2d-model) but not run.
- The retire fix itself (invocation wrapper, `native_header_release` on `NativeSnapshotReceiving`) is compile-verified only; no passing runtime test reached the close yet.

STATUS: BLOCKED snapretire fix compiled native+wasm (W-snapretire-1/2) and the fem close error is gone, but fem 2d/3d receive tests now stop earlier at the unported source-admission projection (fem3d work); no runtime test reached the retire close
