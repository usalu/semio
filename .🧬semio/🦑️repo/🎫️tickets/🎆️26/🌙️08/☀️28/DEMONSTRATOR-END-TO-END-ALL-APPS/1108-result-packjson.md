# 1108 packjson
Edits (all UNCOMPILED; upstream blocks pack-json build):
- W-packjson-1: Cargo.toml dev-dep semio-framework-value-testing-admission (LOCK NEEDED logged); parse-differential oracle compares serde_json::from_str(to_json_string(to_dsl_value(v))) (serde only in dev-deps); sink test close() ported onto EffectBank + effect close_step.
- W-packjson-3 (pending compile): parser effect_vec now reserves first, charges after success, drops reservation + actual=0 on charge refusal (was charge-before-reserve without refund). Candidate-slot charge-before-reserve_one left (no refund API on NativeDecodeControl).
Compile status: W-packjson-2..4 rc 1 only in semio-framework-value-retirement lib (value 🦀️.rs:179 `close_step` not in trait RetirementCursor; peer value-layer port in flight). No pack-json error reached yet.
Not started (need a compiling tree): 16 reds (derive receiver open work action, close turn work 1 vs 0 performed = value-layer lane `nested`, 3 Receive stalls, retirement panicking-observer, wire reds).
Rerun: chk.sh W-packjson-N 🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml native --tests ; then packdebt/tst.sh
Reruns W-packjson-5..7 (rc 1): still only upstream errors (native-decoding construction, receiver-protocol, owned list NativeReceiver port, clone-source-testing duplicate imports, persistent SealedShared::admit). No pack-json error reached. Edits unchanged.
W-packjson-8..10 (rc 1): only `semio-framework-value` lib (56 -> 50 errors) and native-receiving-original (37) fail; zero pack-json errors reached.
Ported admit_leaf/admit_branch/SealedShared::admit callers (encode tree tests: persistent, immutable, tree/tests persistent_fixture) onto an EffectBank `paid`/`sealed_row` helper (admit_from + effect); dev-dep testing-admission added to encode Cargo.toml (LOCK NEEDED). Uncompiled.

## Update (after value facade went green)
- semio-framework-pack-json-canonical lib: FIXED-verified W-packjson-13 (rc 0, native). 733 contract port: tree `alias_demand`/`alias` (RetainedOwnedProjectionAdmission admit+take with OriginalEffectReply, replaces RetainedOwnedProjection::project and source.project_owned; wire root alias via `source.try_borrow().project(0,..)`), `close_step(body,..)`, admission.take_output(effect), source.take_authority_into (reply slot), SharedUtf8 tree node as Node::String.
- pack-json lib (W-packjson-16): parser cursors ported (JsonProjectionIteratorRetirement / JsonCompletedFrameRetirement: inherent close_step(body,effect) paying record_retirement_work, local `json_retirement_reply!` close_into), sink owner.step(body,..), operation close_factory_ticket(slot,body,..). All non-receiving errors gone.
- REMAINING: 39 lib errors, all in 📥️decode/🧳️receiving/🦀️.rs (OriginalJsonReceiving driver) + dependents (🫴️unique, 🪪️native-input): needs port to the new NativeReceivingChild::poll/OriginalEffectAuthority protocol (RetainedCloneSourceTake gone -> take_authority_into reply; admission.take_output(effect); advance gets OriginalSourceTurnWitness; NativeReceivingOriginal has no begin_close/close_step/failure; FactoryOwnedSourceAuthority::transfer_copy_byte_demand/take_original gone; RetainedCloneSourceAdmission::new has 2 args). Depends on value-native-owned receiving protocol being final.
- Test-side: encode tree tests + sink/parse-differential ported (uncompiled). 16 reds not started (need lib compiling).
Rerun: chk.sh W-packjson-N 🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml native --tests

## Update 3 (receiving driver + native-input)
- OriginalJsonReceiving driver ported (NativeReceivingChild::poll with OriginalSourceTurnWitness, reply-shaped receiver close via pub `close_native_receiver`, source/typed-source `take_output(effect)`, `retained_alias` for typed/canonical projection, `take_authority_into` / `transfer_original_into` relays, `X::settle` close ladder): pack-json lib + all test targets COMPILE (W-packjson-20, W-packjson-24).
- semio-framework-pack-json-native-input lib FIXED-verified (W-packjson-33, rc 0).
- semio-framework-pack-json-owned-input (🫴️unique): remaining errors are in its dependency `🫴️receiving/📥️owned` (value-side, not pack-json code).
- Test harness ported to the 733 Authorities/EffectBank model (shared `🧰️support` with `🔐️authorities`; producer-driven close). Tests RUN now (W-packjson-30/35): grammar probe test passes; remaining red: close tests need reply transfer (`transfer_reply_into` + `transfer_producer`, written in support and sink close, final run W-packjson-36 unverified), receiving `canonical` row faults (assert faulted==!canonical at tests:62; error text printed by [DEBUG] line), typed-source projection refuses with "original projection must be driven or returned through its resident frame" (staged OriginalProjectionFrame protocol for FactoryOwnedSourceAuthority: TypedProjection phase must use admission begin_projection/accept_projection/finish_projection instead of retained_alias on try_borrow).
- Temporary `[DEBUG]` prints remain in receiving tests and sink close helper (remove when green).
- LOCK NEEDED logged for canonical, pack-json (testing-admission, testing-allocator dev-deps).
Rerun: C/1105b-tools/packdebt/tst.sh W-packjson-N 🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml native -p semio-framework-pack-json
STATUS: BLOCKED pack-json tests red: typed-source staged projection protocol + reply transfer verification pending (libs green)

## Final (context limit)
- All `[DEBUG]` prints removed (receiving tests, sink close helper).
- W-packjson-36 (foreground): rc 1 from UPSTREAM only: value-native-owned `🔄️map/🦀️.rs:11` macro `native_mapped_receiver_engine!` cannot find `RetainedCursorRelease` (foundation lane edit in flight). No pack-json error; test results of the last edits therefore unconfirmed. Last confirmed run W-packjson-30: lib 1 passed / 4 failed (close, receiving x3), sink target 0 passed / 9 failed (close helper lacked reply transfer, since added), other targets ok (2+1+1 passed).
- Not done: typed-source projection onto staged OriginalProjectionFrame (TypedProjection phase uses retained_alias on try_borrow, refused with "must be driven or returned through its resident frame"); the 16 original reds not re-triaged.
- Next agent: rerun `tst.sh W-packjson-N <pack-json Cargo.toml> native -p semio-framework-pack-json` once value-native-owned compiles; port TypedProjection via admission begin_projection/accept_projection/finish_projection.
STATUS: BLOCKED value-native-owned compile (RetainedCursorRelease); pack-json libs green, tests unconfirmed
