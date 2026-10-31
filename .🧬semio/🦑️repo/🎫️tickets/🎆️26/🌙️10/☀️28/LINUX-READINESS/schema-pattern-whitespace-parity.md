# ECMA Pattern Whitespace Parity

Rust char::is_whitespace uses Unicode White_Space. ECMA regular-expression whitespace uses the language's WhiteSpace plus LineTerminator productions, including U+FEFF while excluding U+0085. This distinction is supported by the [Rust method documentation](https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace) and the [ECMA character-class semantics](https://tc39.es/ecma262/#sec-runtime-semantics-compiletocharset), with the [explicit whitespace table](https://tc39.es/ecma262/#sec-white-space).

Four shared neutral cases compare whitespace and non-whitespace classes for U+0085 and U+FEFF, with U+2028 and ASCII-space controls. The independent Ajv/JavaScript oracle passes. Actual public native run18809/PID1833705 settled1 at2026-10-29T22:58:03.119Z: seven tests pass and the pattern corpus fails exactly because the owned matcher accepts U+0085 as whitespace while the oracle refuses it.

Only the internal pattern whitespace predicate changed: the exact ECMA whitespace/line-terminator code-point set replaces Rust's broader/different property. No authored pattern, JSON scalar parser, validation budget or cancellation control changed. Native verification is included in Root's genuine URI causal-red run35014; that run may intentionally fail URI cases while separately proving this whitespace repair. No duplicate compiler was started.

The same prior native run proves the original-root document cycle repair green. A private Scope retains the actual original schema root during a sibling-document base transition; it does not clone the root or invent a URI alias.
