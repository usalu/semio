# Native Receiving Success Assembly

The sampled MCP call exceeded its 48 million host physical items before the first Input receipt. Each sampled stack passed through `NativeDecodeControl::receive_original_effect_dyn`, `NativeReceivingEffect::take_assembled`, and `OriginalReceivingPrimary::empty`. The assembler replaced the inline receipt owner before distinguishing successful results from failures.

The native receiving frame now checks the child result, each refusal field, the native allocation outcome, and the resident receipt completion before replacing the inline receiving owner. When those checks allow success, the vacant owner stays in place and a benign allocation outcome is dropped. The failure branch still transfers each original cause and records the same settled progress. A receipt completion alone still retains its original data and selects the same invariant refusal. Both paths use the same work accounting and frame reset.

The shared custody fixture covers clean success and a receipt-only failure. The native test asserts the receipt message retains its owned storage, the selected refusal kind, the returned progress, the remaining work, and vacancy after transfer. The Bun test independently checks receipt presence and work subtraction with SQLite.

- Direct native relink against 25 exact cached dependencies passed all five receiving-frame tests.
- The Bun and SQLite fixture passed both tests.
- The WASI stdio MCP replay is being verified with the main ticket.
