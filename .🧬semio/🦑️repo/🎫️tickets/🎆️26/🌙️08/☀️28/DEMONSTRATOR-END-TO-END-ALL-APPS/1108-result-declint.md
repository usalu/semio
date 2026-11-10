# 1108 declint
- value-retirement: FIXED-verified native W-declint-6 rc0, wasm32-wasip2 W-declint-7 rc0 (Box<dyn RetirementCursor> impl ported to 733 trait: body param, retirement_contract loan path, delegating close_into/recover_close_into/close_original_into/recover_close_original_into; stale close_step removed).
- value-control-contract nested_within fix landed (operation lane verified).
- decl-tool-contract / interaction: NOT edited. W-declint-8 (native, both crates) rc 1, blocked upstream by foundation-owned crates: value-factory(3), native-bounded-ordering(9), native-encoding(2), retained-work(1), retirement-queue(10).
- Rerun: "$C/1105b-tools/chk.sh" W-declint-<n> Cargo.toml native -p semio-plugin-decl-tool-contract -p semio-plugin-interaction
STATUS: BLOCKED value-factory/native-bounded-ordering/native-encoding/retained-work/retirement-queue (foundation lane); decl-tool-contract+interaction untouched

## Update
- FIXED-verified native W-declint-11 + wasm W-declint-12: value-retirement-queue, value-factory, value-original-aliases (queue Poll-shaped step ladder with body; factory_retire_snapshot over SharedOriginalStorage; aliases close_step body).
- W-declint-13 (native, decl-tool-contract+interaction) rc 1: blocked by semio-framework-value-native-decoding (decode/🏭️construction/🦀️.rs, 6 errors; uses removed sealed .publication / RetainedWorkOutcome ?). Not mine/not claimed by declint.
STATUS: BLOCKED value-native-decoding (decode/construction); decl-tool-contract+interaction untouched
- W-declint-14 rc1 after foundation native-decoding fix: blocked by job-outcome(6), job-return-escrow-publication(2), native-receiver-protocol(5), value-persistent(2); waiting
- FIXED-verified native W-declint-17 + wasm W-declint-18: job-outcome, job-return-escrow-publication (close_step* take effect).
- W-declint-19 native (decl-tool-contract+interaction) rc 1: blocked by value-native-tree-engine (decode/owned/value/bulk, nested lane).
STATUS: BLOCKED value-native-tree-engine
- W-declint-21 native rc 1: blocked by value-native-owned (204 errors, nested/receivers2/macros lanes actively porting; was 225).
STATUS: BLOCKED value-native-owned (peer port in progress); decl-tool-contract+interaction untouched
- W-declint-23 native: blocked by semio-framework-value aggregate (49: derive RetainedClone/RetireOwned codegen + paged/shared-utf8/publication root still on removed advance/2-arg close_step), value-edit (56, nested), native-receiving-original (4), job-receiving state.rs:68 (1). Needs macros lane for derive proc-macro. Not edited by declint.
STATUS: BLOCKED value aggregate derive codegen
- RetainedClone derive (value/✨️derive/🧬️retained-clone/🦀️.rs) rewritten: struct+enum share expand_record, tuple-engine-shaped reply cursor (plan/route/advance_reply/close_reply). Derive crate native W-declint-25 rc 0; expansion unverified until semio-framework-value compiles (W-declint-26 stops in value-paged, instance-b).
- RetireOwned derive already matched 733 (close_into family); not touched.
STATUS: BLOCKED value-paged/shared-utf8 (instance-b, toolrun) before derive consumers compile
- W-declint-29 native semio-framework-value: derive-generated errors gone. Left: 8 in hand-written 📣️publication/🌳️root/🦀️.rs (toolrun): use retained_clone::OriginalEffectReply[Complete], advance_read/return_read.
STATUS: BLOCKED value publication root (toolrun) before value wasm / my two plugin crates
- NativeReceive derive default-slot stage 2 ported to slot.take(output,effect); diagnostic native W-declint-30 / wasm W-declint-31 rc 0.
- RetainedClone derive: native_terminal_release added (struct+enum); value W-declint-33, replication-ids W-declint-34 native rc 0. edit-c should rerun indexed_commit.
- W-declint-32 (decl-tool-contract+interaction) native rc 1: blocked by io-control(16), pack-json-native-input(13), replication-command-ingress(1), replication-error(1) (other lanes).
STATUS: BLOCKED io-control/pack-json-native-input/replication-error before plugin crates
- replication-command-ingress + replication-error: FIXED-verified native W-declint-36, wasm W-declint-37 (close_into wrappers).
- payload tests: 🎟️work/🧪️tests/🦀️.rs ported to effects with local effect.rs fixture; job-outcome --tests (W-declint-40) then only fails on serde_json (Cargo.lock lacks dev-dep: LOCK NEEDED). 🧪️tests/🦀️.rs uses the old grant-based PayloadCase harness (job/📬️outcome/📦️builder/🧪️testing) and is not compiled by any target I found; NOT ported.
- W-declint-41 (decl-tool-contract+interaction native) rc 1: next frontier dsl-record-emission 5, pack-format-catalog 2, replication-history-index 9, replication-retained-field 7, replication-wire-core 2, os-presence-native-snapshot 5 (stale close_step/complete APIs; unclaimed by declint).
STATUS: BLOCKED value-consumer frontier (6 crates above); decl-tool-contract+interaction untouched
- FIXED-verified: dsl-record-emission/pack-format-catalog/replication-wire-core (native W-declint-44, wasm W-declint-46); artifact-reference via RetainedEquality derive body param + value-persistent exports (native W-declint-51, wasm W-declint-52); ui-viewport via native_admit_default (native W-declint-55, wasm W-declint-56).
- payload/🧪️tests/🦀️.rs is compiled by the job crate (🧪️tests/🔬️unit), so kept (still grant-based).
- W-declint-57 (decl-tool-contract+interaction native) rc 1: job-context 22 (iocontrol), pack-record-encode-cursor 2 + pack-record-retained 2 (packjson), replication-mutation 7 (bridge).
STATUS: BLOCKED job-context/pack-record-*/replication-mutation (owned by other lanes)
- W-declint-58 native (decl-tool-contract+interaction) rc 1: job-context 15 (escrow), pack-intrinsic 1 + replication-edit 13 (toolrun). Nothing of mine left in the dependency chain.
STATUS: BLOCKED job-context/pack-intrinsic/replication-edit (other lanes); decl-tool-contract+interaction untouched
- interaction ♻️retirement/🦀️.rs ported blind (EDITED-unverified). Left in interaction: 📖️capture(130,230), 📡️live(170-192), 📃️query(44-229), 🔐️authority/📖️inputs(39), 🌱️genesis(20 OpBinaryReceiving). decl-tool-contract not touched. payload/🧪️tests/🦀️.rs (job unit tests) not ported (old PayloadCase harness).
- W-declint-59 blocked before interaction by pack-json Source extent refusal (packjson lane).
STATUS: BLOCKED pack-json source extent / job-context / replication-edit; interaction partially ported unverified
- interaction ported blind (unverified): capture, query, live, retirement, mutations (OpBinaryReceiving via pack-value). inputs left grant-local. genesis OpBinaryReceiving bound satisfied by mutation impl. decl-tool-contract still untouched; payload/🧪️tests/🦀️.rs still untouched.
- W-declint-61 native: upstream job-context 15, replication-conflict 6, replication-history 15.
STATUS: BLOCKED upstream (job-context/birth, replication-conflict, replication-history); interaction ported unverified
- W-declint-62 native (decl-tool-contract+interaction): rc 1, blocked by claimed upstream: job-park 2 + job-return 6 (lane jobs), replication-retained 54 (lane retained). Interaction/decl-tool-contract not yet reached by the compiler; interaction edits remain unverified; decl-tool-contract and payload/🧪️tests/🦀️.rs untouched.
STATUS: BLOCKED job-park/job-return/replication-retained
- W-declint-63 native (decl-tool-contract+interaction) rc 1: only semio-framework-job (5, iocontrol claimed 00:10) and semio-framework-replication-retained (20, retained lane, mid reply-shaped trait change). Interaction/decl-tool-contract still not reached.
STATUS: BLOCKED semio-framework-job, replication-retained
- W-declint-64 frontier (after retained/job green): actor-retained-turn 1, job-native-decode 1, job-publication 1, job-worker-authority 3, replication 2, ui-contract-values 14. Fixed blind: job-publication, job-native-decode, actor-retained-turn, worker-authority:307 (W-declint-65 queued, job-publication then showed only a path fix, applied). Open: worker-authority 240/328 need effect threading (40 callers); ui-contract-values (missing semio_framework_value_native_owned dep at fixed-list:364 + step(body) at original/merge/arguments); replication facade exports from retained.
STATUS: BLOCKED worker-authority design + ui-contract-values + replication facade; interaction/decl-tool-contract not reached
- W-declint-66 (job-publication/native-decode/actor-retained-turn native) running; result pending

## Handover (declint, context nearly exhausted)
- Ownership now: job-publication, job-native-decode, actor-retained-turn (verify), then interaction, decl-tool-contract, payload tests. Retained=replication facade, jobs=worker-authority cancel-alias threading, mutation=ui-contract-values.
- W-declint-66 = native check of job-publication+job-native-decode+actor-retained-turn, was queued behind capacity (repo cargo queue) at write time; read $G/W-declint-66.rc/.txt.
- Edited-unverified: job-publication/🦀️.rs close_step(effect); job-native-decode close_retirement_recipient(effect.grant().maximum_copy_bytes,effect); actor-retained-turn job/source step(child.maximum_copy_bytes,effect); interaction capture/query/live/retirement/mutations (OpBinary pack-value, deps semio-os-pack+semio-os-dsl, LOCK NEEDED).
- Not started: decl-tool-contract (emission site ~515, plugin_typed_owner_original(original,active,effect) callers), payload/🧪️tests/🦀️.rs (PayloadCase harness is grant-based; used by job crate 🧪️tests/🔬️unit).
- Rerun: "$C/1105b-tools/chk.sh" W-declint-<n> Cargo.toml native -p semio-plugin-decl-tool-contract -p semio-plugin-interaction  (then wasm, then --tests).
STATUS: BLOCKED queue/upstream; see handover
