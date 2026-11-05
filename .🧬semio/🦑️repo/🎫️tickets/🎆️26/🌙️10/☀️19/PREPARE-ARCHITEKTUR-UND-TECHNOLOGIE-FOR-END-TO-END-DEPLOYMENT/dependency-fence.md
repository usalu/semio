# Dependency Fence Diagnosis

The failed synchronization preserved the exact Source fence and reported no published bindings. The current error discards the first unequal endpoint, parent, or ancestor. A diagnostic classifier will retain the same comparison and report the first mismatch path and field before retrying.

Files: dependency storage TypeScript implementation and adjacent fence schema/corpus/tests. No ownership bypass or filesystem writer has been started.

## Implemented Diagnostic

The current exact equality condition is preserved. On refusal, the error now appends the first ordered mismatch with section, index, endpoint or ancestor path, field, expected value, and actual value. Decoded physical stamps identify dev, ino, mode, nlink, size, mtimeNs, or ctimeNs. The schema lives in `SourceFenceMismatch` in dependency storage install schema; the portable corpus and existing fence test module cover six authored diagnostic cases. Parent timestamp churn remains accepted.

## Validation

Observed RED: the original synchronization failed after about 35 minutes with only a generic fence error and no published bindings. The diagnostic regression initially required an absent classifier.

GREEN: `repo tools exec --light bun -- <ticket>/fence-check.ts D:/semio` exited 0. Six portable diagnostic cases matched both the implementation and an independent SQLite JSON classifier. The retained exact gate made the corresponding acceptance/refusal decision in every case. The pure runner checks the closed diagnostic output shape. Ajv is currently unavailable, so the integrated Ajv test remains pending until dependency synchronization succeeds. The runner transpiles extracted pure functions and the authored test only; it performs no installation, binding publication, or compilation.

The original retained fence snapshot was process-local and is no longer recoverable. Therefore the exact historical endpoint change cannot be identified retrospectively. The next authorized synchronization will report that evidence directly. No Source ownership check has been weakened or bypassed.

## Files

- Dependency storage `🟦️.ts`
- Dependency storage `🧬️schema/📥️install.json`
- Dependency storage `🧪️tests/🚧️fence/🔣️.json`
- Dependency storage `🧪️tests/🚧️fence/🟦️.ts`
- Ticket `fence-check.ts` retained verification input
- This report

## Bounded Diagnostic Follow-Up

The diagnostic contract now caps paths at 256 characters, indices at 65535 (the existing Source ancestry capacity minus one), and expected/actual values at 2048 characters. Field names use a closed enum. Missing-row and serialization fallbacks carry bounded serialized strings rather than unbounded object values. The publication gate still uses its original equality expression. Diagnostic formatting validates the output schema inside a guarded boundary; an unavailable or invalid diagnostic produces the original domain refusal, and never permits publication.

Test-first RED: the new missing-row fixture rejected the earlier unbounded object result. GREEN: nine portable cases now agree with an independent SQLite JSON classifier; five negative schema cases reject excessive path/value/index bounds, arbitrary object values, and unknown fields. Tests also assert the exact formatted valid error and the generic refusal when a diagnostic path exceeds the schema bound. The pure verification command exited 0 after the final changes. Ajv remains pending until dependency synchronization makes the existing test dependency available.

Historical cause remains unproven. Root observed stable current endpoints and is independently running the authorized retry; this agent performed no endpoint writes and launched no dependency writer.

A tenth portable case now proves a 4096-character observed value produces a 2048-character diagnostic value, independently reproduced with SQLite `substr`. The final pure verification exited 0 with ten cases, five negative schema cases, and the refusal-format assertions.
