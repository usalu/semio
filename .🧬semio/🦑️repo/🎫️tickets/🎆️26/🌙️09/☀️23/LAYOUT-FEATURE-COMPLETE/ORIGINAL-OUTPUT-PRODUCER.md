# Original Segmented Output Producer and Seal

Implemented for the active Layout artifact ticket on 2026-10-21. No ticket or goal was closed. No Layout source, Git state, worktree, AGENTS file, or external runtime dependency was changed.

## Public First-Party Contract

The existing media contract exposes these inherent methods on ArtifactOutputChunks:

- original_push_demands() -> RetirementDemand
- push_original(&self, source: &mut Option<Vec<u8>>, credit: Option<&ArtifactMediaExportCredit>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure>
- original_seal_demands() -> RetirementDemand
- seal_original(&self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure>

All caller authority comes from the actual supplied StepContext. Both ports require an original source parent and inspect the actual remaining grant, CPU/deadline and cancellation. No grant is fabricated from a demand. Both demand quotes require one item and depth one, with zero allocation and release demand. Producer copy demand is the native Vec header plus five usize counters and one bool tag (65 bytes on a 64-bit platform); seal copy demand is one usize and bool (9 bytes on a 64-bit platform). Callers must use the methods rather than those platform-specific numbers.

Undergrant, cancellation, exhausted work and busy authority return false without a receipt, source movement or output/credit changes. Original source validation is performed only after grant admission and nonblocking gate acquisition; an admitted validation refusal posts exactly the bounded validation receipt and one fuel unit while retaining the exact source and leaving queue and credit unchanged. A missing source is an explicit funded invariant refusal. Duplicate seal is an explicit funded invariant refusal, so the calling job must retain a successful seal stage.

Producer publication acquires the output queue gate and optional credit writer gate before validation. The existing direct credit method uses the same writer gate. Credit overflow or an already exceeded credit is refused without poisoning unchanged credit. The fixed queue's push returns its original Vec on refusal; the new port restores that exact Vec into the source Option. After the fixed queue accepts without allocation, all counters and optional credit commit while both authorities remain held. Existing atomics remain the O(1) observation interface. Queue total and outstanding bytes retain distinct histories if disposal has previously removed a chunk.

The older push(Vec) and ordinary seal() entry points remain available for ordinary callers; Layout should use the new original ports. No public signature depends on a third-party type. The credit writer gate is inline and introduces no extra heap allocation or external runtime dependency.

## Tests and Evidence

The formal language-neutral JSON schema and fixture were authored before producer/seal implementations. The fixture has 20 producer cases and eight seal cases: each independent grant axis, CPU, cancellation, queue/credit busy, empty/oversized/missing source, sealed queue, slot/allocation/byte/credit refusal, exceeded credit, ordinary success, uncredited success and original allocation slack on success/refusal.

Native laws use an admitted OriginalCallerSourceParent, the original borrow_context, actual with_retained_loan and the same retained recipient identity. They compare source pointer, length, capacity and contents; queue byte/slot counters; credit bytes and exceeded flag; exact copy/item receipts and CPU; and controlled retirement of source, queue and credit. Every nonzero physical release demand is also tried one byte short through the same original context and must yield without fuel/receipt changes. Terminal physical ownership is required empty. Additional laws check that direct credit shares the writer gate and that unsupplied replacement contexts are refused.

The owned ByteOracle test interface wraps the existing serde_json library: expected source-byte JSON and actual queued-byte JSON must agree in addition to pointer identity. No dependency was added. The ticket checker uses the existing AJV 8.20.0 library to independently validate the fixture against the formal schema, then audits the source ordering and authentic-caller law wiring.

Executed through the permitted Node REPL: schema validation passed for all 28 cases; source contract audit passed; all added source/schema/test paths are under 256 UTF-16 code units (longest is 130). The checker is original-producer-check.mjs in this ticket. It explicitly reports nativeCompiled:false and nativeExecuted:false.

Native Rust compilation and law execution remain unverified: the task's established native-runner.md documents that repo is absent from PATH and configured to a nonexistent macOS binary on this Windows workspace; no raw Cargo/Rustc/Bun bypass was used. The source and fixture checks are not native execution evidence.

## Files

Updated:

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/🦀️.rs — shared credit writer authority and producer module mount.

Created:

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📤️producer/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📤️producer/🧬️schema/🔣️.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📤️producer/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🎞️media/📤️producer/🧪️tests/🧫️fixtures/🔣️.json
- Ticket original-producer-check.mjs (retained checker input)
- Ticket ORIGINAL-OUTPUT-PRODUCER.md (this report)

No generated output files were retained.
