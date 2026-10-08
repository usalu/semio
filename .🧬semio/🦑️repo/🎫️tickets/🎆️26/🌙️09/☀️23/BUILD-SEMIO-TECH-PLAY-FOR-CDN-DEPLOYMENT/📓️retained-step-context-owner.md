# Retained Step Context Ledger Owner

The mounted child opening driver currently invokes `StepContext::new`, which creates and then drops an original Arc ledger each turn. The existing ledger contains fixed atomic counters only; payload pages belong to retained writers/payloads, each holding an original ledger alias. Its existing `terminal_is_empty` checks all operation and stream counters.

The new owner will live in Job/⏱️context/📦️owner. It will admit its one Arc allocation before creation, issue zero-allocation contexts using the same original ledger, and retain that ledger until all contexts/writers/payloads return it. Whole-frame release must use `Arc::try_unwrap`, retaining its returned Arc on refusal. Exact allocator observations must match birth/query and same-turn physical release. No close cursor or new ledger may be created at teardown.

Neutral corpus covers stable operation/generation identity over 1, 65, and 257 turns. Ajv validates the schema and JSON Patch independently validates alias retention and final ownership. Current native baseline uses the existing constructor under the real shared System HeapWitness; it requires zero heap birth/free for the paid retained turn and therefore exposes the actual repeated-ledger defect. Neither baseline has completed yet.

Source scope: new owner subtree, narrow root include/reexport, and test-only canonical shared allocator registration. Root owns the mounted driver integration and will call the owner at a separate exact birth turn.

Actual baseline: native `retained-step-context-existing-red.log` ended Nx EXIT1 with 1 run / 0 pass / 1 fail / 34 outside; the current allocating constructor requested and freed 128 bytes during the paid turn. Neutral source baseline ended 1 pass / 1 fail, 974 assertions, on missing owner.

Current native `retained-step-context-owner-final.log` ended uncached Nx EXIT0: 2 run / 2 pass / 34 outside, native .100s, total11.5s. Original stable identity and fuel/sequence were tested across1/65/257 turns, all0 birth/free. Exact whole128-byte birth and release matched System. Zero work/zero bytes/one-below retain without allocation/free. Separate live context, writer, and committed payload each block the original ledger and resume after their actual page/alias closure; no count precheck substitutes for atomic unique unwrap. Source `retained-step-context-source-current.log` is2pass0fail982assertions. An intermediate compile-only borrow correction and fixture empty-writer disposal correction are retained separately; they are not claimed as production semantic receipts.
