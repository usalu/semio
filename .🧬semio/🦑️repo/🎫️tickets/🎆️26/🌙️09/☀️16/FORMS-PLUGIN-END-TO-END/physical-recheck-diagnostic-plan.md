# Physical Recheck Diagnostic Plan

Production source is deliberately unchanged while root session 66076 compiles. No schema/validator/refusal condition, retry rank, backoff, capacity or byte budget will be weakened.

## Exact Intended Files

After that terminal boundary, the proposed change touches these canonical paths:

- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json`: closed optional physical-file comparison definition; no new production $id.
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts`: carry this one structured, bounded diagnostic through existing ConcurrentEditError causes and append only its schema-defined operands to retry/exhaustion output. Arbitrary error.message text must not become retry output.
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts`: populate diagnostic directly from the existing claim and the already-returned fresh observation at the existing refusal. Missing observation operands are null. Do not reread to construct a diagnostic.
- New neutral fixtures and Bun tests under `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/📊️comparison/`, with schema refs to the existing production observation/churn schemas. Exact authored names: `🔣️.json` and `🟦️.ts`.

The ticket input drafts `physical-recheck-cases.json` and `physical-recheck-diagnostic.schema.json` define cases and the proposed shape. They contain newly authored neutral test data, not copied repository source.

## Test First

Run the new focused test against unchanged production first and retain the actual red result. The test must show current refusal lacks structured operands/retry output, rather than fail solely on a missing import. It can inspect the existing error object and retry event through the current public APIs.

Cases cover same-length differing bytes, differing lengths, UTF-8 multibyte bytes, identical-content rewrite accepted, and disappearance with null observed fields. Independent expected digests come from existing test dependency @noble/hashes SHA256, cross-checked against WebCrypto; byte lengths use TextEncoder. Ajv validates every diagnostic with strict additionalProperties:false. No runtime dependency is added.

For each changed-file case, assert exact reason/path/rank are unchanged, expected equals the original observation, observed equals the already-read second observation, and no file content or environment fields appear. Assert identical-content rewrites remain accepted. Assert body read counts/budgets remain the original observation plus the one existing recheck; no diagnostic read is permitted.

Retry tests retain exact attempt counts/delays and EN/DE language selection. The bounded comparison must survive exhaustion causes and appear in retry output. Non-comparison errors must keep their current text/shape. Unknown fields/digest shapes must be rejected by the schema.

## Why Churn Reporting Is Included

Current retry rendering reconstructs text solely from reason/path and omits raw refusal details. Merely embedding operands into CurrentPhysicalFilesV1's raw message would not expose them in the observed four-snapshot failure. A typed, bounded comparison is necessary for the actual emitted diagnostic. The payload is a fixed kind plus four nullable operands; path already belongs to the existing error. No source bytes, arbitrary error detail, capabilities or tokens are emitted.

## Application Gate

Send the exact planned paths to root now. Apply only after root confirms session 66076 terminal. Run the focused red test, implement schema-first, run green plus existing relevant churn/observation checks once. Preserve 19405 and 76837; no ordering fix follows merely from new diagnostic support.

## Prepared Fixture Validation

All five ticket input cases now carry exact expected byte lengths and SHA256 values. Existing Noble SHA256 and WebCrypto produced identical digests for every non-null operand. Strict Ajv validated the five proposed comparison rows and rejected an added source field. The bounded result is generated/physical-recheck-fixture-check.json. This validates prepared neutral data only; no production diagnostic red/green run is claimed.

The intended production scope is the two existing recheck refusal branches only: vanished/missing identity before transfer receives the retained claim plus null observed operands, and a completed mismatching read receives both exact existing operands. Cancellation, invalid authority, read-time filesystem errors and budget exceptions retain their original behavior; no extra read or catch-and-reclassification is introduced.
