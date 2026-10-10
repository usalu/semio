# Store Exact U64 Receiving

The unchanged full Kernel15 native gate reached `original_vcs_retirement_preserves_real_history_visibility_genesis_and_full_physical_receipts` and failed at the original JSON receiver: `invalid type: floating point 1.8446744073709552e19, expected u64`. The checked-in `forwards` value was 18446744073709552000, exceeding the intended maximum 18446744073709551615 after JSON number rounding.

A domain schema now defines history mutation ordinals as canonical decimal strings bounded by the complete unsigned64 range. The existing neutral retirement suite includes BigInt comparison, independent decimal.js arithmetic, independent Ajv schema compilation, JSON string round trips, and native-width Buffer read/write. It rejects negative, leading-zero, overflow and numeric representations. The existing forward and inverse rows use exact decimal strings; the Rust fixture receives those strings directly with checked u64 parsing. Original history values, grants, body quantum, pointer, native birth/release and terminal Drop assertions are preserved.

The newly registered full source row `📐️artifact-io-store-exact-u64-source🧪️` runs the existing entire retirement suite, with its explicit first caller 660000ms policy and existing 30000ms child test boundary. Its first launch remains pending graph admission; because the shared fixture was corrected while this asynchronous graph admission was pending, it is not evidence of a neutral runtime RED. The genuine preceding native Kernel15 failure is the producer/receiving RED. Native acceptance remains pending a justified unchanged full Kernel retry after the current Kernel15 terminal deadline.

## Actual Full Source Result

The entire original retirement source suite finished with54pass7fail61tests and terminal1. The new exact-u64 ordinal clause passed using all four independent observations. Existing catalog constructor, one-item admission, hydration close/target, original deep cancellation, native snapshot encoder and named native grant clauses failed and remain open under their original assertions. This source result is not whole-suite acceptance, nor proof that the native u64 receiving row passed.
