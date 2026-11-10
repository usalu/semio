# 1108 lane macros
## Step 1 (coordinator priority) DONE
- native-tree-engine (🧱️bulk): only error left was the `paid` closure error type in 🧱️bulk/🦀️.rs; fixed. FIXED-verified W-macros-2 (native), W-macros-3 (wasm32-wasip2): `-p semio-framework-value-native-tree-engine --lib`.
## Step 2 protocol close macros DONE
- protocol crate (`value/🛬️decode/🧳️owned/🧬️protocol`): every close macro now emits the reply shape (`close_step`/`recover_close_step`, `close_original_step`/`recover_close_original_step`, each a thin wrapper over a `*_close_route!` body macro expanded with a constant `recover` flag). Converted: `ordered_close_step`, `scalar_close_engine`, `option_close_engine`, `result_close_engine`, `native_mapped_receiver_engine` (also adds witness `advance_nested` + `advance_pending`, header bound `I:NativeValueSource`), `list_/shared_/ordered_original_close_bridge`. New helpers `native_close_denied`, `native_close_begun`.
- Removed with no replacement: `ordered_failure_accessors` (and its calls in 🧺️set/🗂️map), and every `ordered.failure`/`cleanup.failure` branch (ordered_close_demands, set/map_close_work_quote): `NativeReceivingChild` has had no `failure` field since 733.
- 🌳️value/🦀️.rs: removed `NativeReceiveSlot::close_step` and `close_original_step` (slot keeps `close_into` family).
- 🔄️map/🦀️.rs: struct/impl bounds `I:NativeValueSource` (slot requires it).
- 🌳️value/🔀️variant/🦀️.rs: `NativeVariantSelector::close_step/close_original_step` replaced by reply-shaped `close_into`, `recover_close_into`, `close_original_into`, `recover_close_original_into` (needed by result_close_engine). `settle_step` (poll, used by advance_local and derive enum) left.
## Verification
- FIXED-verified W-macros-13 (native), W-macros-12 (wasm32-wasip2): `-p semio-framework-value-native-receiver-protocol --lib`.
- W-macros-11 (`-p semio-framework-value-native-owned --lib`, native): no error at any macro invocation site; crate still red from implementor-owned code (below).
- Rerun: `"$C/1105b-tools/chk.sh" W-macros-<n> 🧰️framework/Cargo.toml native -p semio-framework-value-native-owned --lib`
## Open items / debts (not mine)
- `advance_pending` missing: 🧺️set, 🗂️map, 🔀️variant (NativeResultReceiver), 🔄️map try-mapped/box, unit variant; hand-written old `close_step` in 🌳️value string/sequence receivers (`sequence_close_work_quote` macro does not exist), 🔄️map NativeTryMappedReceiver, 🔄️map/📦️box, 🔀️variant/🔢️unit.
- Poll adapters still alive and to be deleted once their callers (list, leaf, 🧳️original wrapper, header tests) are ported: `close_native_receiver_backing`, `native_close_nested` (protocol); `native_close_settle` is receivers2's.
- variant selector `advance_local`/`settle_step`/`demands` still on the old NativeRecordMatcher API (variant owner).

## Step 3 (coordinator: macro-driven impls and owned/🌳️value roots)
- 🌳️value/🦀️.rs: scalar, string (cursor `advance_into`/`take`/`close_into` relayed through new protocol `native_relay`), option, sequence (select_advance/advance_nested/advance_local/close_dispatch; reservation/storage via control only in Nested) all reply-shaped.
- 🔄️map try-mapped: `close_dispatch` + advance_pending. 📦️box was already ported.
- 🧺️set and 🗂️map: redesigned on the effect-based ordered engine: shared key/value admission `SharedOwner::admit_from`, cursor `admit_update_cursor`, `advance_native`, child/residual close through `native_relay`. Removed `ordered`/`cleanup` NativeReceivingChild, both authorities, and macros ordered_shared_admission, ordered_cleanup_guard, ordered_failure_accessors.
- Verification: protocol crate FIXED-verified W-macros-18 (native), W-macros-19 (wasm32-wasip2). native-owned W-macros-17 native: no error in any file I own.
## Remaining blockers (not mine)
- 🔀️variant (advance_pending, NativeRecordMatcher demands/advance/close_step) and 🔢️unit: nested/matcher. ⚠️refusal (try_project_owned, OriginalFailureStorage): refusal2. Root 🦀️.rs lines 19-23 macro expansions of those.
- Poll helpers `close_native_receiver_backing`, `native_close_nested` still called by: paged/🛬️receive/🗂️map, 🧳️original, 📋️list, 🔐️shared-slots, ♻️header tests, owned/🦀️.rs re-export. Delete once nested/others stop calling them.
- native-owned wasm not run (crate still red).
## Step 4 (poll helpers removed)
- Deleted `close_native_receiver_backing`, `native_close_nested`, `native_receiver_backing_demands` and their re-exports.
- 🧳️original wrapper: resident `NativeReceiverBackingClose`, `close_reply`, `close_route`; `close_inner`/`close_borrowed` relay the RetainedCloneStep close into the RetirementStep reply (SourceRetirement pattern); RetirementCursor `close_into`/`recover_*`/`close_original_into`/`recover_*` for wrapper and seed; `begin_close` is now the paid `begin_close_into` folded into the first close turn.
- 📦️paged/🛬️receive/🗂️map: removed cleanup child+authority, child begin/close through `native_relay` as Local actions.
- Removed the stale ♻️header closed-header law (poll subject deleted) and its integration test; backing close law lives in protocol 🧪️tests/backing.rs. Debt: `📐️.json` moveWork/freeWork numbers are no longer asserted anywhere.
- Verified: protocol native+wasm (W-macros-25); native-owned W-macros-24 native: only derive/⚠️refusal errors (matcher/refusal2). Paged crate and native-owned wasm not yet runnable.
## Step 5 (coordinator: try-map first, then closed-header law)
- 🛡️try-map (semio-framework-value-native-owned-try-map) ported to the reply shape: FIXED-verified W-macros-35 native, W-macros-36 wasm32-wasip2. This also means semio-framework-value-native-owned itself compiles native and wasm.
- Closed-header law restored from HEAD via read-only `git show` and ported to `NativeReceiverBackingClose::close_into`: protocol `🧪️tests/closed-header.rs` (+ `[[test]] closed-header`), PASS W-macros-42 (native). The old owned-crate integration test (`native-closed-header.rs`) is replaced by this protocol test because the effect-bank harness lives there and owned has no such dev-dependency (no Cargo/lock change needed).
- Fixture `♻️header/🎟️work/📐️.json` (+ schema 🔣️.json and 🟦️.ts) updated DELIBERATELY, because the contract changed:
  - handoff copyBytes 13/25 -> 9/17 (`3p+1` -> `2p+1`): `NativeOwnedStorage::box_demand` is `Option<NonNull>+bool+Option<Box>`; the test additionally requires the backing dispatch header on top (total d.copy_bytes = box_demand + header > box_demand).
  - freeWork 1 -> 2: the paid free now also records the backing dispatch step (quote == measured == 2).
  - poisonDropCalls 0 -> 1: the new backing runs the receiver's real destructor exactly once on the paid free (same contract as protocol backing.json `realDestructor`); the old poll helper deallocated without Drop.
  - Unchanged and still asserted exactly: moveWork 2, deniedWork 0, cachedWork 0, stock, freshBoxAllocations 0.
- Open: protocol `🧪️tests/backing.rs` does not compile (`FixtureSource::from_owner` missing, foundation's clone-source-testing) - not mine.
STATUS: BLOCKED native-owned tests/derive law reruns pending others; macros-lane files compile native+wasm (native-owned lib green W-macros-35/36)
