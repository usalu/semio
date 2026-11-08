STATUS: GREEN for lib compile of the 13 owned crates (valuefix-106 rc 0), of semio-framework-value-edit (valuefix-118 rc 0, from 140 errors) and the new effect law (valuefix-103: 3 passed). UNVERIFIED: queue/ordered/retained-clone test targets (test debt belongs to testdebt-b, valuefix-104 red on the adoption quote).

# 1107 valuefix result

## Verified items
1. Reborrow: CONFIRMED from code. `reborrow(g,m)` requires `m <= remaining_work_units()` (= min(receipt remaining, ceiling - spent)). `run_closure_effect` builds the effect with ceiling `u64::MAX`, so `effect.maximum_work_units()` always exceeds the finite receipt. It also fails after the parent performed any unit. `?` sites return OwnershipLimit, `expect` sites panic (the Some branch has no catch_unwind). No depth subtraction is expected; the contract only needs "child <= what is left".
   Rule (sent to runtime, receivers): `effect.nested(grant)` / `effect.nested_within(grant, authored_max)`: infallible, grant intersected with the parent's, ceiling = min(authored_max, remaining) with 0 on an inconsistent receipt. `reborrow` is unchanged as the strict check.
   Fixed: 95 sites mechanically (fail-closed script `1105b-tools/valuefix/nested.py`, only exact `X.reborrow(G, X.maximum_work_units()[.saturating_sub(..)]).expect|?` or param `maximum_work_units)?` shapes; rest listed MANUAL) in retained-clone, ordered, decode slots, native-default, queue, read, equality, paged; 6 manual sites by hand.
   Left: native-owned (receivers), derive (runtime), value-lane crates (edit, factory, clone-source, native-decoding/encoding, retirement, shared-issuance, snapshot-*, shared-utf8, projection): rerun `nested.py --apply` after adding their dirs to `scope`.
2. Unrecorded charges: queue owner-pop FIXED (`effect.performed(1)?` before `*work=1`). Bulk close frame-pop/failure-take and refusal advance_nested phases 2/4/5 are native-owned: rule sent to receivers/refusal, not edited.
3. Panics/underflows in owned crates: string-cursor 4 expects replaced by typed invariants; dynamic `unwrap` replaced; queue `maximum_depth-1`, `maximum_copy_bytes-placement`, `maximum_work_units()-1`, `maximum_depth-retained` now checked (typed DepthLimit/OwnershipLimit); equality grant/work subtractions checked or saturating; field/dynamic/record/sequence/owned-entry grant literals use saturating_sub (fail-closed zero grant). Left: std-collections `validated standard collection entry` expects (pre-existing, unchanged lines); paged-text expects (crate not lib-green, ledger); `maximum_work_units-N` in equality guarded by quotes. Tuple/bulk/refusal unwraps are native-owned (rule sent).
4. Zero-progress: queue `close_original_birth_step` depth shortfall now returns DepthLimit refusal instead of work-0 Progress (fixed). Tuple stage 2, refusal phase 1/3, bulk are native-owned (rule sent). Denied-grant quoting is native-owned.

## Laws
- `🎟️control/🧬️contract/📦️effect/🧪️tests/work-nested.json` (+schema, `work-nested.test.ts` with SQLite oracle, labels plain) and Rust test `nested_effect_gets_exactly_the_live_remaining_work_where_the_whole_ceiling_is_rejected`: asserts the old whole-ceiling reborrow is rejected in 5 of 6 cases while `nested` yields min(remaining) and clamps a widened grant; tests passed (valuefix-103).
- Queue pop/depth: covered by existing bank-driven turn law (`retirement_turn_original_queue_...`), whose bank rejects unrecorded work; it currently fails earlier on the adoption quote that testdebt-b is fixing (`placement_depth`).

## Logs
`C/🗑️generated/1105b/valuefix-101..106.{txt,rc}`.

## semio-framework-value-edit (second assignment)
- Ported to the effect protocol: `IndexedEditPlan::advance`, `IndexedCollectionCommit::advance/close_step`, `NamedEditPlan::advance`, `NamedKeyIndexPreparation::advance`, `AdmittedNamedKeyIndex::project`, `NamedPersistentRoot::project_index/project_rows`, its preparation `advance`, the transfer cursor `advance`, and `NamedPersistentRootReceiver` on the new NativeReceiver (select_advance / advance_nested for the resident list child / advance_local / take / recover, Poll close side with Pending propagation).
- Work accounting: `record_owed` + `finish` (edit root) record on the effect exactly the work an action owes beyond what children recorded, so reported work equals performed work. Allocation goes through `allocate_empty_vec` (real bytes in the effect marker, also on refusal). Public advance/close fns now take `&mut RetainedAllocationEffect` instead of a grant; commit `begin_close` is a free flag, children start closing through `begin_close_paid` inside `close_step`.
- Helper traits `Settle`, `Polled`, `Exit` unify Outcome and Poll children; a pending close inside one advance is an invariant failure, in the receiver close it propagates.
- Left: pre-existing `unwrap/expect` on unchanged lines (named base_order, index directory, ranked key row); `Local{work}` quotes for data-dependent phases are the ceiling (`maximum_work_units`), not exact; the edit crate tests still use the grant API (test debt, not run); `OriginalVectorCursor::admit/take_front` keep grant signatures (owed work recorded by the caller).
- Log: C/🗑️generated/1105b/valuefix-107..118.{txt,rc}.

## pack-json (third assignment, split with aggregator)
- Mine, now clean in valuefix-124: parser-and-grammar-cursor, controlled-and-borrowed-bridge (+sink), decode/borrowed, json-error. Remaining crate errors are only aggregator's decode/receiving and decode/operation.
- RetireOwned moved to the new protocol (leaf macro, retirement_fields_type/seed, derive on Value, JsonError via the String cursor). Step API now takes `&mut RetainedAllocationEffect`; every loop unit is recorded on the effect (reported == recorded). Standalone `parse` uses `AuthoringWallet::run` with a work stock sized from the input length.
- Limits: the JSON writer cursor refuses typed when an encode allocation quote stays pending (no resume path); legacy `RetirementStep::Child/Bytes` kept in the projection iterator retirement (zero reported work); json-error/leaf cursors drop structured data on retirement.
- Quotes: runtime confirmed a ceiling `Local{work}` quote is admitted when reported == recorded and quote <= maximum_work_units (value-edit receiver).

## Review-2 and wallet audit (confirmed from code, fixed unless noted)
- Reborrow(max): deflate `initialize_one`, window-config mutation `close_step`, dsl-schema encoding (+3 other sites via nested.py) -> `effect.nested`, depth `saturating_sub`. Compiled: deflate (valuefix-128); window-config and dsl-schema UNVERIFIED.
- Canonical tree `last_turn_work=1` without performed: now the pure unit is recorded with `effect.performed(1)` only when the turn made progress and children recorded nothing; error paths report 0. pack-json compiled (valuefix-128). No new test (testkit-gated).
- Tool-run os native tick: `consume_fuel(1.max(performed))` -> `consume_fuel(performed)`, fuel debited before the retained `?`, failure arm now debits `performed`. UNVERIFIED (os plugin, wasm). Tick receive constant `work:1` NOT changed (needs NativeDecodeControl charge-path reading; owner port-d).
- PNG: `PngDecodeError`/`ImageDecodeError` gained `WorkLimit` and `AllocationFailed`; work-record and allocation refusals keep their kind instead of `Invalid`. Compiled (valuefix-128). JPEG/source/handoff phases still grant-based and unrecorded (separate unported decoders).
- Flood `account_close` expect -> typed invariant refusal.
- ui-locale `is_err()||` assertion: NOT confirmed as new; the removed lines contain the same form. Left.
- Left unfixed: io-control `retire_decode` Pending dropping progress, other tool-run unwraps, window-config registry assert, sqlite expects (owners; no verdict from code).
- Wallet audit: `AuthoringWallet::run` now debits work even when settlement fails after the effect ran and finishes the action when the commit is refused (previously the early return left the reservation live and the drop assert panicked). New `AuthoringWallet::charge_work`. Law `work_recorded_before_a_refused_settlement_is_still_debited` red before, green after (valuefix-129 red, valuefix-130 6 passed).
- Pack JSON `parse`: stock now sized from input length (items, copy/capacity/release bytes, work), `NativeDecodeControl` bounded by it, each step's progress posted (`wallet.post`). Compiled (valuefix-128).
- Energy bridge: control limit from `bridge_stock()`, construction quantum capped by wallet, construction work debited via `charge_work` (UNVERIFIED, energie recompiles); energy `Default` expect and `retire_decoded` swallow deferred by energie.
- Presence JSON fresh 1024-unit wallet inside a turn: NOT fixed; needs a control-provided original effect in `native_snapshot_turn` (io-control/koordinator + runtime).
