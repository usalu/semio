STATUS: GREEN for `cargo check` lib of semio-framework-dsl-record-model, native (dslowned-4 rc 0) and wasm32-wasip2 (dslowned-5 rc 0), and the derive crate (dslowned-6 rc 0). UNVERIFIED: derive record expansion in a consumer (semio-plugin-decl-views blocked by job-context, pack-record-retained, replication-history/conflict reds, dslowned-19), `--tests` (old-API tests in 📥️owned/*/🧪️tests and 🎟️work not ported), derive expansion in a consumer crate, pack borrowed cursor compile, external callers (os dsl receiving, store native-snapshot, space body-receiving, viewport, plugins).
Logs: 🗑️generated/1105b/dslowned-{4,5,6}.txt

## Decision C (dsl-private OwnedFieldReceiving, moves FieldValue payloads)
New trait shape (📥️owned/🧬️contract): `demands`, `select_advance -> OwnedAdvanceSelection::{Local{work},Nested}`, `advance_nested(source,native,body,max_work,grant)` (control-driven children only: NativeReceivingOriginal in the intrinsic cursor), `advance_local(source,body,effect)`, `take_into(output,effect)`, `close_demands`, `close_work_demand`, `close_step(effect)`; all return `RetainedWorkPoll` (OwnedPoll/OwnedTake/OwnedClose). `native.receive_retained_work`/`checkpoint`/capacity guards are gone from the cursors: the driver opens the effect (quote = select_advance work / close_work_demand).
- Single charge: every turn reports `effect.performed_work()` (helpers done/settle/owe in contract); grant-priced children (builder advance/adopt/finish/take, PagedUtf8AppendCursor, take_output) record only their owed remainder via `owe`.
- Ported: contract (OwnedSlot, DeclaredRecordShape split into pure `check` + effect `step`), storage (reserve_empty/initialize_seed/take_box_into/release), scalar, text (PagedUtf8AppendCursor), vector (allocate_empty_vec via effect.actual()), array, wrapping (Option/Box/Statement), mapping (OwnedReceivingMap::map_into takes effect, `map_work_demand`), persistent (effect-native sealed birth/publication replacing NativeSharedAdmission; `shared_admission()` removed), intrinsic (nested NativeReceivingOriginal::advance, effect take/close; new dep semio-framework-value-native-receiving-original, path refresh needed no lock).
- Derive (✨️derive/📥️owned) emits select/nested/local/close_work_demand; mapper impls use effect map_into.
- Other fixes in the crate: DslSequenceView impls for PersistentList and (T,T) moved to the ownership crate (orphan E0210); allocation projection of NativeAllocationRefusal via `owned()`; ProjectionOwned/Frame made `pub` doc-hidden (private type in RetireOwned::RetirementCursor).
- pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor: BorrowedProjectedPackCursor advance/advance_intrinsic/close take `&mut RetainedAllocationEffect`, expression child runs under `effect.nested`; own actions `effect.performed(1)`. Its siblings 🎒️document/⏳️cursor and 📨️wire still call grant signatures (and other stale source/admission APIs): next owner.

## Caveats
- Nested selection quote for `NativeReceivingOriginal::demands` uses u64::MAX max work.
- Retained (unresolved handle) allocation refusals project as Null in the encode projection.
- Old tests/fixtures under 📥️owned/*/🧪️tests and 🎟️work need a port onto OriginalJobFixtureCaller/effect helpers.

## Follow-up (coordinator: pack siblings)
GREEN native + wasm32-wasip2: semio-framework-pack-record-encode-cursor (dslowned-16 native rc 0, dslowned-17 wasm rc 0) including 🎒️document/⏳️cursor, 📨️wire and ⏳️cursor, all on `&mut RetainedAllocationEffect`. Also green native: semio-ui-viewport (dslowned-18, expands dsl_owned_scalar_receiving).
- dsl-record-emission 🛫️encoding: RecordRetirement now has an inline enum cursor; the finished child writer retires inside advance_unit through a bounded AuthoringWallet step; borrowed/record.rs borrow fix.
- dsl-record: transparent cursor ported; new `receiving::receive_wrapped` replaces the removed map_outcome for newtype borrowed-family receivers (transparent + scalar newtype macros).
- pack preflight schema cursor: advance/close take effect (reserve/release via nested effect).
- document cursor: own action work is recorded once when no effect-aware child recorded (record_own); foreign plain values in paged lists use `Held<T>` with an inline HeldCursor; shared reserve_list/release_list in ⏳️cursor.
- Next blockers for the derive-expansion proof are not mine: job-context (11), pack-record-retained (21), replication-history (59) / conflict (18).
