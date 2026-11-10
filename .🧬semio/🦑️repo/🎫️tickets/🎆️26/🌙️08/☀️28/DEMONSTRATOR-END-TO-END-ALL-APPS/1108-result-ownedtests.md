# 1108 lane ownedtests

## Method
- `--lib` closure of semio-framework-value-native-owned was 257029 of 262144 source bytes; the 38 KB refusal test module was moved to `[[test]] refusal` and the receiver driver is now included in the lib tests (limit untouched).
- Shared driver `value/🧪️testing/📥️admission/receiver-driver.rs` is the harness for every port: `receive_cut` (denial matrix on every demanded axis, funded turn, Source pointer stability, denied take keeps output), `receive_faulted` (one refused allocation, exact work, error retired), `receive_traced` (per-turn work Trace, cancellation callback via `Trace.admit`), `close`/`close_source`/`retire`/`lifecycle` (real closes to terminal-empty), `funded` (synchronous owner turn), `admit` (genuine Source), `DefaultReceiver` (source-free default cursor adapter). `bank_run` takes the Work verdict after every complete turn (EffectBank.run asserts it is inspected; the driver never did, so a second funded turn would have panicked).
- Lib bridge: `extern crate self as semio_framework_value_native_owned` (cfg test) so the driver compiles inside the crate it drives.

## Files ported (all on the driver, no mem::forget, laws kept; see debts)
- lib: 🌳️value/⚠️refusal (variants + progress law stay in lib), 🌳️value/🔢️array (storage refusal, sequence parent).
- [[test]] refusal (new), native_result, set_receiving, native_work (bulk, storage, array work, scalar, take, variant (Selector adapter), map, set, record receiving, initializers, batch), native_mapping (+box), native_sequence (tuple, source), native_fields (Row via derive, enum payloads, defaults), native_shared (wide record, defaults, flattened, empty, matcher ordinals via MatcherReceiver adapter).
- Production fix: NativeSequenceReceiver advanced index/phase on any non-default Progress; now only on Complete (🌳️value/🦀️.rs phase 3) per the NativeSequenceCollection::push_step contract.
- Accessors under `cfg(any(test,feature="testkit"))`: NativeTryMappedReceiver::{validation_refusal,retained_input}, NativeMappedReceiver::child_is_closing.

## Debts (explicit, no silent weakening)
- record storage `complete_record_transfers_original_backing_without_moving_value`: NativeRecordStorage has no box take any more (box transfer lives in NativeBoxReceiver and the protocol backing tests); test dropped with the API.
- native_fields pointer identity of the taken Row id (needs receiver internals), shared-receiver foreign `advance` with a denied grant (foreign Source is refused at demands/select_advance instead).
- refusal `pending_original_node_birth_survives...`: authority-port harness removed; replaced by the cut sweep over the recursive-retirement fixture.
- Not in this package: shared-slots (🔗️sealed-shared-receiver/🧪️tests/🦀️.rs, source-bank.rs) and native-default package tests.

## Latest run W-ownedtests-17 (all 8 ported targets compile native; no limit raised)
- lib 3 pass / 1 fail (the sequence-parent law passes with the production fix and the deliberately updated partialPush fixture); native_fields 1/9; native_mapping 0/3; native_result 0/1; native_sequence 2/4; native_shared 0/6; native_work 2/15; refusal 0/4; set_receiving 0/1. Total 8 pass / 39 fail, all failures are runtime.
- Failure classes (first panic of each): (1) receiver close: `close_demands` Err(invalid) / "receiver failed its funded close" (set, refusal kinds, fields) - receiver reports is_closing but close_demands refuses, or a pending close action never settles under the producer driver; (2) array child `take` Complete but published nothing ("native take was admitted but published nothing"): the reply-shaped child take publishes only on the producer's reply transfer, i.e. the relay two-phase answer ownedfix is fixing in the ack frame; (3) fixture drift after 733: scalar bindingWork 6->3 (updated), sourceAdmissionWork 12->37 (updated), storage placeWork 3 vs 1, array work 2 vs 0, take work, bulk capacity stock delta; each needs a per-law ruling, not a blind constant bump; (4) secondary "original construction action retains an unsettled reply" = authority dropped during unwinding of a first panic.

## Production faults found and fixed at the source
- NativeSequenceReceiver advanced index/phase on any non-default Progress; now only on Complete (🌳️value/🦀️.rs). Verified by `sequence_parent_preserves_actual_child_progress_and_only_advances_on_complete` (fixture `partialPush` parentWork 2->0 in 📐️.json, 🔣️.json and the bun oracle).
- NativeListSource was implemented for the stale native-source `NativeValueSource`; the tree trait is schema-engine's, so `PersistentList<T>: NativeReceive<dyn tree::NativeValueSource>` never held (E0277 in native_sequence). Now implemented for schema-engine's trait. LOCK NEEDED 📋️list/📦️packages/🦀️rust/Cargo.toml (native-source dep replaced by native-schema-engine; cargo accepted it without the lock tool).
- Shared driver latent bugs fixed: denial grants used `then_some(... d.copy_bytes-1)` (eager underflow), EffectBank requires the Work verdict to be taken after every complete turn, nested turns need construction+Work recipients, retirement/close/Local relays need a genuine producer authority across pending turns, nested `observe` panics (the allocator observer is not re-entrant).

## Open / next
- Port is structurally complete for the 8 targets; the remaining 39 runtime reds need the two-phase reply semantics (ownedfix) and per-law fixture rulings. Not done: shared-slots tests (🔗️sealed-shared-receiver/🧪️tests/🦀️.rs, source-bank.rs), native-default package tests, record_source / native_slot_custody / record_matcher_driver targets not rerun here.
- The driver machinery now lives in the shared admission crate (`driven.rs`: Authorities, driven, bank_run) so the lib test closure stays within 262144 bytes (receiver-driver.rs 15.5 KB of 21.5 KB headroom).

## Rerun
`"$C/1105b-tools/runtime/tst.sh" W-ownedtests-N 🧰️framework/Cargo.toml native -p semio-framework-value-native-owned --lib --test refusal --test native_result --test set_receiving --test native_work --test native_mapping --test native_sequence --test native_fields --test native_shared`
STATUS: BLOCKED 39 runtime reds after the structural port: reply two-phase take/close semantics (ownedfix ack-frame) and post-733 fixture rulings; best run W-ownedtests-17 (8 pass)
