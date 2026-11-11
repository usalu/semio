# Lane reactorturn result (semio-plugin-reactor-turn effect/cx port, takeover from mediaenc)

## Design (as built)
- `🔌️extension-invocation-and-command-ingress`: `retired` is generic over `ControlledRetirement<T>` and runs `closing.step(bytes,effect)` through `close_original_effect_step` (the performed work is the charge, no flat fuel). `close_exchange_owner` quotes with `plugin_typed_owner_demand` and closes via `plugin_typed_owner_original(owner,closing,effect)` inside `close_original_effect_step`. `CommandIngressOwner::Presence` carries an `OriginalConstructionAdmission` (`construction`), closed in the retirement ladder after the exchange owner.
- `📥️receiving`: `release_page` runs `PagedList::release_empty_page(effect)` inside `cx.with_original_effect`, charges `performed` via `consume_work`, consumes the retained receipt of a `Ready` step; `Denied` leaves the page and charges nothing; `Work`/`Release` failures stay typed.
- `📥️receiving/📦️completed`: JSON grammar steps use `OriginalNativeDecodeSource::receive_effect_step` (work = `effect.performed_work()`); the intrinsic document birth, advance and close run through a local `through` helper (mirrors `job/🛬️receiving/📥️pack`), the birth takes its bytes inside the effect so a denied quote keeps them.
- `⌨️command-ingress-turn`: presence exchange admission uses `admit_original_construction_settled` over the resident `construction` field; a pending settle resumes next turn with the same admission and output.
- `🧊️cold-pair-loads-and-process-pool`: `ColdPairDocumentLoad` owns its boxed archive (stable donor address, which the archive admission pins), its `OriginalDocumentArchiveAdmission` and an `admitted` flag; the ingress page parks the load and `step_cold_pair_document_load` admits it through `plugin_begin_document_archive_load(.., cx)` (`Ok(false)` stays `Loading`, a refusal finishes the pair with `encode_fault_bytes(&admission_fault(..))`, a superseded unadmitted load ends `cold-pair.load-cancelled`). `RestoredDocumentLoad.archive` is boxed for the same reason.
- `🌀️poll-kernel-turn`: `pending_effects_runnable` comes from `plugin_drive_pending_effects(runtime,&mut effects,cx)`; `🏁️exchange-finish` raises (`retain`) the per-instance `pending_effects` owner on a `None` pull and lowers it (`publish`) on `Some`, so the answer is the truthful source.
- Upstream: `store/🦀️.rs:300` re-exported the nonexistent `NativeSnapshotEncodeEffectQuote` (removed in c2477fa4da5), breaking `semio-os-kernel` for every dependent. Dropped the name from the re-export (mcpprobe owns the follow-up port of `🌉️mcp/🏠️workspace/🚪️io/🔤️json:110`, logged).

## Verification
- Native `W-reactorturn-2` (`-p semio-plugin-reactor-turn`): rc0, 0 errors (warnings present, so the crate was really type-checked; none new from my edits).
- Native `W-reactorturn-t1` (`-p semio-plugin-runtime-exchange --lib reentry_laws`): rc0, `re_entry_after_a_none_pull_records_the_effects_exactly_once` ok (1 passed).
- Rerun: `"$C/1105b-tools/chk.sh" W-reactorturn-N Cargo.toml native -p semio-plugin-reactor-turn`; `"$C/1105b-tools/runtime/tst.sh" W-reactorturn-tN Cargo.toml native -p semio-plugin-runtime-exchange --lib reentry_laws`.

## Hub rerun W-reactorturn-hub-1 (wasm, rc1): remaining reds with owners
- `semio-plugin-reactor-turn`, `semio-plugin-runtime-exchange`, `semio-plugin-reactor-surfaces` (cold-pair), `semio-plugin-job`, `semio-plugin-instance-docs/jobs`, `semio-plugin-testing-caller`, `semio-os-kernel` are no longer red.
- `semio-plugin-instance-render` 6 errors, all in `instance/📥️staging/🏷️reserved-jobs/🦀️.rs` (206 and 235 `E0061` 3-arg call plus the dependent `?`/`bool is not an iterator` errors, 237 mismatched types): owner gen3dfinal (claimed 23:50, "instance-render staging: projections, reserved-jobs, tool-machine"); not instdocs (instance-docs is green).
- `semio-plugin-reactor-wit-bridge` 102 errors (first wasm compile of guestrt's statically fixed bridge): `semio_plugin_abi::wasip2::exports::semio::framework::reactor::{Event,Budget}`, `wit_events::{ActivationEvent,CompletionResult}`, `wit_capabilities::{CapabilityChange,CapabilityGrant}` are not found. The WIT (`plugin/🧬️schema/📜️reactor.wit`) does define `budget` in `reactor` and imports `event` from `events`, so the binding module paths (`events`/`capabilities` vs `exports::..::reactor`) in `🪢️bridge/🦀️.rs` and `🎟️event-lift/🦀️.rs` are what needs fixing: owner guestrt (see `1108-result-guestrt.md`, "wit-9 pending"). Also an uncommitted `abi/🦀️.rs` `pub mod job_kinds` addition by shellreds is in the tree (untracked dir `🚪️io/../🧬️schema/💼️job-kinds`), fine.
- Everything else of the hub was not reached; rerun after those two: `"$C/1105b-tools/hub.sh" W-reactorturn-hub-2`.

## Debts
- Inline test modules of reactor-turn (`#[cfg(test)]`, `🧪️tests/**`) were not compiled; only the lib check and `reentry_laws` ran.
- `plugin_typed_owner_work_demand` is no longer called by reactor-turn (effect-shaped closes settle the performed work); other callers keep it.
- `close_exchange_owner` and `retired` now charge the performed work, not a pre-quoted flat work; a Pending close keeps its owner and nothing is charged for a denied effect.
- Cargo lock: `semio-plugin-ownership` path dep (lock-154) was already applied for the check.

STATUS: FIXED-verified native W-reactorturn-2 (semio-plugin-reactor-turn rc0) + W-reactorturn-t1 (reentry_laws ok); hub W-reactorturn-hub-1 red in instance-render 6 (gen3dfinal) and reactor-wit-bridge 102 (guestrt)
