# SET Demand Audit Correction

Low found a confirmed source/type P2 in the first native79 freeze: an accidental duplicate operation guard inside immutable `demands()` called the mutating operation helper, referenced unavailable `base`/`cx`, and tried to change `state.phase`. Rustfmt could parse this but cannot typecheck it. The initial79 hash manifest is preserved as generated/native-type-set-select-before.json.

The duplicate arm alone was removed. The preceding immutable capacity/copy/depth quote remains in `demands`; the real mutating operation branch remains only in `advance_inner`. The existing mounted original SET law already calls allocation-free repeated demands and denied-axis steps before every advance, including the actual operation phase15. A new adjacent helper law explicitly repeats Create/Add/Remove capacity quotes and asserts no allocation or binding construction. That law is mounted through the actual SET helper module. Its initial JSON test-expression delimiter was corrected before the refreshed syntax run.

The refreshed generated/native-type-set-select-source.json remains79 files with owned12/reference67,50 Rust syntax checks exit0 and maximum path146 UTF16 units. Authentic Cargo still exits101 before rustc. Only the producer correction and added helper law differ from the initial79 scope; no TypeScript/package or capability changes occurred. Tests remain authored/unexecuted and physical sufficiency is not certified.

This refreshed freeze is handed back to Low for exact rehash and bounded source review. The source/type defect is corrected at source level; genuine native compilation and execution are still unavailable.
