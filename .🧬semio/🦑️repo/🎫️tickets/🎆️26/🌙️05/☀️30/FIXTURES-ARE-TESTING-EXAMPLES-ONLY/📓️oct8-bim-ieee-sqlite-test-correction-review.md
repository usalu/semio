# BIM IEEE SQLite Test Correction

Current BIM SQLite tests use Database.deserialize options.safeIntegers for integer-word fidelity rather than an unavailable instance method. The six authored binary64 words include positive/negative zero, infinities and distinct signed NaN payloads. Each word is written and read explicitly little endian through Buffer; stored rotation_bits and restored domain rotation bits are asserted exactly. The scalar expectation uses the authored finite decoded value, preserving negative zero rather than normalizing it.

If the actual SQLite scalar channel canonicalizes negative zero, the scalar assertion will expose that behavior independently of the exact bit channel; source inspection does not establish a passing runtime. Genuine per-snapshot schema validation, imported edits, semantic limits, cancellation and contradiction refusals remain present. No compiler or test run by this audit.

Publication reports that its current Presence peer close pays original String capacity before replacement with String::new and intends capacity-zero terminal witness. This is executor-reported draft behavior pending independent final source/receipt review, not runtime credit.
