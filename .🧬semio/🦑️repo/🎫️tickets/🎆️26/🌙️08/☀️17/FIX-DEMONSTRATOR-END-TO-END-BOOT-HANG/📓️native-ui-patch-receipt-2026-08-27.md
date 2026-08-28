# Exact Native UI Patch Receipt

## Executed Codec Boundary

The canonical shared schema and fixture were read from actor/lifetime/patch before implementation. Two TypeScript tests first failed on the missing codec/constant/equality (2 failed, 89 skipped, 0.476s; `🧪️guest-ui-patch-receipt-codec-red-1.log`). The implementation then passed the complete actor suite: **91/91**, six files, 3.65s, start 19:25:42 (`🧪️guest-ui-patch-receipt-codec-green-1.log`).

The fixed maximum is 35 bytes: canonical unsigned LEB128 activation generation, u32 instance, guest lifetime, and positive u64 patch sequence. Tests consume all four shared vectors, all eight malformed values, every truncated prefix, invalid scalar domains, and the five zero/one-patch pairings. Strict Ajv validates the language-neutral fixture and an independent webassemblyjs encoder reproduces all four byte strings, including maximum u64 values. Equality is wire identity only, not a native retirement capability.

Dag owns the shared schema, Rust codec, Kernel and WIT. The WIT source now declares the exact receipt in TurnResult and both PatchAck/PatchRejected. Production native adoption is still held for its own RED checkpoint. This TypeScript result does not claim native guest behavior, full patch retirement or all-app success.

## Scheduler Checkpoint

The issued-patch scheduler follow-up first stopped before collection on the peer-owned discovery parser placeholder (`🧪️guest-ui-patch-scheduler-red-1.log`). The next actual test failed because coercion erased receipt bytes: **1 failed/623 skipped**, 8.04s (`🧪️guest-ui-patch-scheduler-red-2.log`). The fix preserves exact bytes by reference and forwards the producer's bigint sequence, including a value above JavaScript's safe integer domain. The focused lifecycle scheduler cohort then passed **3/3**, 621 skipped, 9.88s, start 19:34:53 (`🧪️guest-ui-patch-scheduler-green-1.log`). No full-renderer result is inferred from this selected run.

The real private UI-ACK scheduler cohort passed **3/3 selected**, 615 skipped of 618, 11.40s, start 19:19:06 (`🧪️guest-lifecycle-scheduler-ui-ack-1.log`). It covers the dedicated captured lifecycle work, mailbox preservation, and actual private UI publication token submission after ordinary operation revocation. This checkpoint precedes the issued native patch-receipt wire adoption.

## Live Integration Boundary

### Host Authority and Generated Producer

Host receipt TDD reproduced the missing check (1 failed/91 skipped, 0.458s; `🧪️guest-ui-patch-authority-red-1.log`). Full actor GREEN then passed **92/92**, six files, 4.11s, start 19:30:55 (`🧪️guest-ui-patch-authority-green-1.log`). Native authority now captures the exact producer receipt, rejects missing/malformed/foreign/duplicate identities and two-patch turns, and preserves existing original claims. Frozen receipt metadata is separate from mutable raw bytes. The UI owner is updating its private token and fixture helper against this source.

The generated bridge test reproduced four missing byte mappings (1 failed/60 skipped, 3.64s; `🧪️guest-ui-patch-producer-red-1.log`). Full producer GREEN passed **61/61**, 34.30s, start 19:29:00 (`🧪️guest-ui-patch-producer-green-1.log`). It executes the generated module and covers all shared receipt vectors, exact ACK/Rejected forwarding, five cardinality cases and refusal before guest invocation for missing/invalid/foreign receipt events.

Post-test source hashes: materializer `a246d95516306aa6fdbfb32bcaf8bdf825c685bc20f12eeb09eaa7af5b4c1d5c`; dev test script `9cf27bc33e650ab84ad1d803e2a6ffd0b1d46da98b5077bcff794a9033d1282c`; actor authority `1c0bdd0e992198a46c11696ce57ab10454c0822c2f734b7b68bab64119f21fec`. No pre-test hash stability claim was made for these three gates. The codec source hash is `922af44c7d06c952e11f2def377359fdeffc0d71afe14d19945a4878e3dd4f36`.

The renderer's concrete OwnedUiPatchIntake contract and its source were read. The callable retains the exact source and aggregate through lookup, one-input admission, paired publication, private ACK, and local close. Its source is not yet released for live mounting. Original response-envelope ownership must be installed at pending transport dispatch before pending removal, heartbeat recomputation, error grafting or external continuation. An await-only wrapper is insufficient for failure envelopes and unknown wrapper fields.

No source restoration, cleanup, generated-output publication or heavy compiler run occurred.
