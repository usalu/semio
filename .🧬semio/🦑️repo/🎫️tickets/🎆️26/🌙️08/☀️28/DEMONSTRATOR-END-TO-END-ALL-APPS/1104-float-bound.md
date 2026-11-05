# Bounded Decimal Parsing Research

This is design evidence for Flow's still-unmounted scalar parser work; it is NOT acceptance of an implementation or a native behavior pass.

A positive binary64 rounding midpoint can be written with odd numerator at most `2^54-1` and binary denominator exponent at most 1075. Multiplying that numerator by `5^1075` gives at most 768 significant decimal digits. A registered Node BigInt calculation independently confirmed that `(((1n<<54n)-1n)*5n**1075n).toString().length` is 768.

Rust's own decimal fallback keeps a 768-digit buffer and a separate truncation marker. See the [official Rust decimal sequence source](https://doc.rust-lang.org/nightly/src/core/num/imp/dec2flt/decimal_seq.rs.html). This supports the finite-digit bound; it does not by itself prove the proposed streaming normalize-and-reparse algorithm, lexical equivalence, exponent clamping, signed zero or cancellation/deadline behavior.

Flow must preserve Rust float grammar including special values and malformed-field defaults. Required evidence includes exact midpoint/tie cases, overflow/subnormal/underflow boundaries, long zero/nonzero tails, sign and exponent extremes, trailing junk, finite original byte grants, cancellation and real deadline measurements. Precharging fuel before an arbitrary-length synchronous parse was explicitly not accepted as a bounded implementation.
