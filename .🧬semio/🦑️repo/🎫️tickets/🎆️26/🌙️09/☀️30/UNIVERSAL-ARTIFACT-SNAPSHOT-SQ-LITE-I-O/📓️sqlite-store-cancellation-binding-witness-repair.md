# SQLite Store Cancellation Binding Witness Repair

## Actual Native Diagnosis

Root's mounted Store allocation four-law Native run (`Nextest 3cf6d4bd-b6c4-4b20-a0cb-4f935be4bc6a`) reported four run, three passed, one failed, 56 outside. The focused diagnostic run (`Nextest fb9729ff-3f30-42df-849a-9b7c5bd4c887`) reported one failed, 59 outside. Receipt: `🗑️generated/root-authentic-shared-store-canceled-allocation-exact-diagnosis.log`. The failing message is `Binary/DecodeNative: exact canceled admission 245830 must reach the same owned copy: Err("limit exceeded: native text exceeds caller allowance"); remaining 0`.

The original cancellation selector accepted any `DecodeNative` callback with the 140000-byte literal total and an interior completion above 65530. Actual binary decoding first validates a borrowed UTF-8 slice, then copies its owned text. Borrowed validation publishes the same literal-byte progress total but does not admit the literal text copy. Prior metadata/record backing may already be admitted. Replaying that prior-only admitted ceiling can therefore correctly refuse the subsequent text-copy preflight before reaching the originally observed validation callback. This Native failure does not establish that production refunded an owned canceled text copy.

The correction targets the actual record binding callback immediately before `AllocationRecord::__dsl_from_record_controlled`. Only the DecodeNative cancellation selector requires this test-local binding scope. EncodeNative already targets its controlled projection's owned String copy. Production controls, byte limits, 140000-byte literal, threshold 65530, Binary/Text coverage, actual canceled admission, exact replay and subsequent cumulative refusal remain unchanged.

## Schema-First Source Contract

The allocation fixture schema now requires a closed exact `cancellation` object with three explicit stages: borrowed validation in DecodeNative admits zero literal bytes; owned binding in DecodeNative admits 140000 literal bytes; owned projection in EncodeNative admits 140000 literal bytes. Its portable preceding admission uses eight bytes and does not pretend to encode platform-specific native metadata layout.

The Source law uses system `node:buffer.isUtf8` to validate the existing UTF-8 bytes, and independent BunSQLite to measure the actual literal BLOB byte length. It distinguishes a borrowed canceled stage's zero literal admission and consequent next-copy preflight refusal from a canceled owned stage's complete admission and exhausted exact replay ledger. Existing ledger fixtures remain intact.

Registered portable baseline completed with the new schema/law and the original fixture lacking `cancellation`: `bun nx run @semio-tech/framework-os-kernel:test-snapshot-native-admission --skip-nx-cache --args=portable`. Receipt: `🗑️generated/sqlite-store-cancellation-scope-source-baseline-red.log`: exit 1, 9 passed, 2 failed, 188 assertions, 5.4 seconds Nx duration. The actual failures were the closed-schema missing required contract and the new cancellation-frontier law's missing stages. This route returns before its Native command when `portable` is passed; this lane does not run Cargo.

## Mounted Tests-Only Correction

The handcrafted fixture now names all three stage semantics. The Native test helper `decode_at_binding` enters a `Cell<bool>` scope immediately before the actual controlled record construction and clears it after the returned success/refusal/cancellation Result. Both the generous canceled input and its exact admitted-ceiling replay require that scope before refusing a DecodeNative interior callback. Binary parser callbacks with the same literal progress are observed and accepted before binding begins. EncodeNative still cancels its existing actual owned projection copy. The canceled admission measurement, exact replay, zero remaining assertion and following cumulative refusal remain intact for Binary and Text in both directions. No production or limit was changed.

The same existing cancellation law also directly checks the genuine native borrowed UTF-8 validator with zero owned allowance: its interior cancellation has zero admitted bytes, and the following owned text copy is refused at its preflight with zero ownership retained. This explicitly distinguishes borrowed validation from the actual scoped Store binding witness. The Store suite remains four tests.

Registered portable rerun passed: `🗑️generated/sqlite-store-cancellation-scope-source-current-green.log`: exit 0, 11 passed, 0 failed, 203 assertions, 5 files, 485 milliseconds test runtime, 7.6 seconds Nx duration, uncached. The independent Source laws do not execute the changed Rust Native witness. Rustfmt syntax parsing for the final Native test exited 0; readback is `🗑️generated/sqlite-store-cancellation-binding-tests-parser.rs`. Fresh Store4 Native validation remains Root-owned and pending.

Changed files are confined to `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🧪️tests/💰️allocation/`: `🧬️schema/🔣️.json`, `🧫️fixtures/🔣️.json`, `🟦️.ts`, `🦀️.rs`; plus this ticket report. Existing launcher and task registrations were reused. No production file, runtime dependency, script, Git state, goal or ticket lifecycle was modified in this lane.
