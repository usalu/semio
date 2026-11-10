# foundation result
Native lib FIXED-verified: value-shared-issuance, value-native-encoding (W-foundation-1), value-native-bounded-ordering (W-foundation-3), value-native-decoding (W-foundation-4), value-persistent (W-foundation-5). retained-work edited (u128 work), passed once queue was fixed by declint.
Edits: sealed/🦀️.rs (publication removed, DUPLICATE_WORK, try_duplicate via duplicate_into); encode ♻️retirement + 🏭️construction (body params, work borrow, finish_accepted); decode 🏭️construction (work borrow, receive_effect_typed lends effect, receive_allocation_effect, sealed publish/prepare_birth); persistent admit_leaf/admit_branch take an effect (use SealedShared::admit_from); bounded-ord imports.
Debts: wasm not run for these crates; test callers of admit_leaf/admit_branch (grant,&mut None) in pack-json, sealed-shared-list, store mutation-apply and persistent tests still old form; refused construction accept leaves an action that asserts on drop (only on recipient fault).
Window-config (W-wincfg-107, 97 errors): all in upstream crates owned by peers: value-native-list-receiver (nested, reply-shaped NativeReceiver port), job-outcome/job-return-escrow-publication (declint). None in window-config yet.
Rerun: chk.sh W-wincfg-108 🌎️hub/Cargo.toml native -p semio-plugin-window-config
STATUS: PARTIAL foundation crates FIXED-verified W-foundation-5; window-config BLOCKED on nested + declint crates

## Update 19:5x
- wasm32-wasip2 lib FIXED-verified (W-foundation-6): shared-issuance, native-encoding, bounded-ordering, native-decoding, persistent, retained-work.
- admit_leaf/admit_branch test callers ported (sealed-shared-list, persistent tests, mutation-apply immutable-root); SealedShared::admit callers ported in persistent/edit admit_funded helpers, value-edit named index (lib), mutation-apply semantics/apply. pack-json left to packjson (claims line).
- clone-source-testing duplicate import fixed.
- persistent --tests (own manifest, W-foundation-10): 143 errors, none about admit; stale cursor test API (advance/close_step/begin_close/adopt/finish, OriginalRetirementFailure path). Unowned test debt.
- Remaining SealedShared::admit test sites outside my crates: retirement sealed tests/work/frontier, retirement demand test, projection pkg, decode construction tests:103, retained-clone cursors.rs/unit tests, job source, tool-run (toolrun), pack-json.
- window-config: W-wincfg-110 stops in value-native-owned (nested, 229 errors mid NativeReceiver port) and earlier job-outcome/escrow-publication (declint). Window-config not reached.
- Source budget: raw non-test .rs of window-config is 306,579 B in 29 files (gate counts closure differently; was 250,370 B); cannot be gated until compile reaches it.
STATUS: PARTIAL

## Update 20:0x
- Real source gate for window-config (bun 1105b-tools/bundle/closure.ts <crate>/🦀️.rs): 18 files, 254,863 B / 262,144, 2,037 / 4,000 lines. Under budget; the earlier 306 KB figure was a raw directory count. No split now; headroom 7,281 B, the next port must not grow it.
- Refused-construction drop assert: cannot be fixed inside encode/decode; the contract has no way to settle a refused accept (finish needs no reply, finish_accepted needs Accepted). Requested OriginalConstructionLoan::finish_refused from operation in the claims log; encode and decode quote_construction call it once it exists.

## Update: finish_refused
- Added OriginalConstructionLoan::finish_refused (control-contract 🏭️construction/🦀️.rs) and a law (refused_accept_settles_through_finish_refused_and_drops_without_assert). encode and decode quote_construction return the refusal through it, so no action is left to assert on drop.
- Verified: control-contract + native-encoding + native-decoding lib, native W-foundation-12 rc0, wasm32-wasip2 W-foundation-13 rc0.
- Law not run: W-foundation-14 refused by the source gate (control-contract test closure over 262,144 B: producer tests file 46,109 B vs 22,682 remaining); needs a test-closure split by the control-contract owner.

## Update: control-contract test split
- Producer tests moved to [[test]] "producer-original-effects" (path 🏭️producer/🧪️tests/🦀️.rs, public API only; record_forward_metadata made pub for its law). LOCK NEEDED logged.
- W-foundation-17 (native, own manifest) rc0: lib 41 passed (includes refused_accept_settles_through_finish_refused_and_drops_without_assert), producer target 9 passed.

## value-edit / value knock-on (W-wincfg-113 / W-foundation-18)
- value-edit (56 errors) is not a call-site fix: its named-index cursors (📇️index {🦀️.rs, ♻️transfer, 🌳️root, 🌳️root/🛬️receive}, 🚚️commit, named/🦀️.rs, edit 🦀️.rs) implement the old RetainedCloneCursor::advance / RetirementCursor::close_step / NativeReceiver::close_poll API. The 733 trait is reply-shaped (advance_into / close_into / recover_*_into with OriginalEffectReply, body param, 4-arg NativeReceiving structs). Each implementor needs a producer rewrite, not an arity edit.
- value (49-56 errors) is the same class: shared-utf8 clone, paged clone/append/receive/map, publication root retirement, type (clone cursor), text utf8, retained-clone/paged BoundedOrd(Cursor) impls, and the OriginalSlot advance_read/return_read removals.
- Idiom reference for callers: the persistent test wallet (take_immutable: crate::effect::run + OriginalEffectReply::borrow(&mut slot) + cursor.take(..effect, reply)); implementor reference: refusal2's persistent-test port and nested's NativeReceiver port, neither done yet. I did not guess the implementor shape for 100+ sites.
- Not edited in this step. Claim released for re-assignment (suggest one lane per module family: named-index, paged, shared-utf8+text, publication+type).
STATUS: BLOCKED value-edit/value need reply-shaped cursor implementor ports; window-config unverified

## value-edit scoping (after reading the 733 exemplars)
- Size: named-index producers ~134 KB of hand-written state machines (📇️index 22 KB, ♻️transfer 15 KB, 🌳️root 14 KB, 🌳️root/🛬️receive 29 KB, named 25 KB, 🚚️commit 13 KB, edit root 16 KB). Every nested call changes shape: PersistentSet advance/take/begin_close/close_step -> advance_into/take_into/begin_close_paid/close_into with OriginalEffectReply and recover_*_into after unwind; RetainedCloneBinding::close_one, admission.advance/take_output/return_inputs, retirement.begin_granted/step_granted also changed.
- The exemplar (VecCursor) carries per-route unwind recovery state (close_route/advance_route/copy_reply) for each nested call; a port must reproduce that for each of ~12 nested call sites per producer, and the 733 laws exercise it.
- Not started: I judge this a multi-hour rewrite that cannot be done correctly in a few compile cycles (each cycle ~3-5 min, errors cascade). Recommend splitting by file (index+transfer, root+root/receive, named+commit) across lanes, each taking a PersistentSet-call-site idiom from one agreed first port.
STATUS: BLOCKED value-edit not ported

## value umbrella: source budget
- text/utf8 close_step fixed (step_granted takes body). type clone cursor: its errors are derive-expansion (RetainedClone/RetireOwned derive), not hand-written; declint.
- semio-framework-value closure = 268,912 B / 262,144 (24 files, 3,073 lines): admission refuses before the compiler (W-foundation-19), so no value compile can happen until it is split. Largest: retained-clone/📦️paged 42.7 KB, 📦️paged 33.0 KB, shared-utf8/clone 22.0 KB, paged/receive/map 21.5 KB, paged/receive 18.5 KB, clone 17.4 KB, paged/append 15.4 KB. Proposal: move the paged family (📦️paged/** + retained-clone/📦️paged, ~134 KB) into its own crate with a trait seam; owner instance-b (active in those files). Tool: 1105b-tools/bundle/closure-list.ts.
STATUS: BLOCKED value over source budget; window-config unreached

## semio-os-semio, window-config-mutation, semio-ui-locale
- FIXED-verified native (W-foundation-23) + wasm32-wasip2 (W-foundation-24): os-semio (ControlledRetirement::step takes body), ui-locale label retirement and plugin-window-config-mutation (inherent close_step(body,effect) + explicit close_into reply; facade has no native_retirement_reply macro re-export).
- W-wincfg-114: the value facade is green now. Window-config still blocked by owned red crates: io-control (35) + job-clock (1, duplicate OriginalSourceClock import) = instance-b, pack-json (39) = packjson, replication-retained-field (23) = retfield, value-snapshot-read (28) = snapread.
- Rerun: chk.sh W-wincfg-115 🌎️hub/Cargo.toml native -p semio-plugin-window-config, then wasm, then --tests.
STATUS: PARTIAL window-config waits on instance-b/packjson/retfield/snapread crates

## Update: macro + wincfg-115 frontier
- ui-locale and window-config-mutation now use semio_framework_value_retirement::native_retirement_reply! (W-foundation-25 native, -26 wasm rc0); LOCK NEEDED logged for the new dependency.
- W-wincfg-115 frontier: diagnostic, io-control, job-clock, pack-json-native-input, replication-local-interaction, replication-retained-field, value-snapshot-read, ui-scene-mesh. Fixed mine: scene-mesh (mesh-owner step body), local-interaction (update step body), diagnostic close_step ports. W-foundation-27: only 4 diagnostic errors remain, all from the NativeReceive derive emitting NativeDefaultSlot::take_local (derive ✨️derive/🛬️receive:83 vs slot without it).
- Still owned elsewhere: io-control + job-clock (instance-b), pack-json-native-input (packjson), retained-field (retfield), snapshot-read (snapread), derive take_local (declint/nested).
STATUS: PARTIAL window-config waits on owned reds

## Update: dsl-record-model + wincfg-116
- semio-framework-dsl-record-model (🫴️receiving/📥️owned) verified native W-foundation-28 + wasm W-foundation-29 rc0; ported by iocontrol (W-iocontrol-10/11), nothing left for me.
- W-wincfg-116 frontier: 4 crates, all claimed: dsl-record-emission (5), pack-format-catalog (2), replication-wire-core (2) = declint; os-presence-native-snapshot (5) = iocontrol. Window-config itself not yet reached.
- Rerun: chk.sh W-wincfg-117 🌎️hub/Cargo.toml native -p semio-plugin-window-config, then wasm, then --tests.
STATUS: PARTIAL

## W-wincfg-117 (native lib, rc 1, 68 errors, none in window-config)
job-context 1 (private into_source_loan/into_borrowed_parent, driver claims job-context), job-park 2 + job-return 6 (jobs: close_original_effect_step gone, super depth), replication-retained 54 (retained/port-c). Window-config not reached; native, wasm, --tests unrun.
Rerun: chk.sh W-wincfg-118 🌎️hub/Cargo.toml native -p semio-plugin-window-config, then wasm, then --tests.
STATUS: PARTIAL

## W-wincfg-118
- Frontier: job (5), tool-run (13), os-history-schema (3), store-message-ledger (3), os-vcs (18); window-config unreached.
- Mine edited (unverified): message-ledger close_step(body) at 137/579 + replay.rs:476, history-schema retirement.step(body,..) at 233. W-foundation-30 was then blocked by a replication-retained regression (peer restructuring: retained_routed, preflight_retained_diff, RetainedMutationAdvance unresolved).
- Open design item: spr/history/👁️source holds Arc<HistoryLog> but SharedControlledRetirement::new now takes SharedOriginalStorage<HistoryLog> (raw Arc has no conversion by design).
- os-vcs (18) untouched: close_step -> inherent+macro, history-ledger read cursors on the new reply-shaped cursor API, group-visibility prepare_close_state_step/close_state_step(body).
STATUS: PARTIAL
