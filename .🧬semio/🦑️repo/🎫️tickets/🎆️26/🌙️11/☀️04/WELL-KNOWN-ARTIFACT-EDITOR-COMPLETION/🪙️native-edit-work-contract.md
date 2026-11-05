# Native Edit Work Contract

Permanent neutral schema, 43-row corpus, and Bun test live at stdio/tests/artifact-acceptance/work. Actual execution: 1 pass, 0 fail, 146 assertions, 256 ms. No production Rust or Cargo changed.

DOCX phases are snapshot measurement, mutation measurement, addressed apply, completed-post measurement and inverse measurement. ZIP phases are envelope inspection, one-row scan, addressed apply and completed-post validation. Mesh and Brep phases are one-row scan and addressed apply. Every phase has admitted success, admitted failure and denied-before-action rows. Each format also carries a two-attempt partial failure followed by a new denied call, proving preserved work2 on partial error and reset-to0 on the next call.

The independent SQLite oracle records an attempt before checking its failure, derives work from count of those actual records, and compares concrete ordered action trace and outcome to the fixture. It does not derive CPU work from copied_items, result.is_ok or an assumed constant success count. Existing prior-work values deliberately differ from expected current-call counts. These tests establish the neutral contract; actual native edit implementations and their partial-failure work ports remain subject to Rust regression execution.


## Four production port review

Read-only review after root implementation confirms reset before guards, reservation before the bounded phase, closure capture of fallible operations, commit before returning the captured Result, and getter of retained actual work. DOCX counters fit reserve2/3; ZIP counters fit1/2; Mesh/Brep fit1. No fresh control or ceiling is introduced. prepare_work checks cumulative overflow before admission; commit_work resets reservation and posts actual work, with no new cancellation callback. These synchronous local phases do not mutate the recipient binding, so valid counters within reservation avoid a later normal commit failure.

One concrete mismatch was found and sent root: ZIP scan validates an oversized entry name before incrementing last_work, returning a row-envelope refusal with actual work0 after an admitted scan. The neutral failed scan-row law expects1. Count the attempted rejecting row inspection, while retaining zero work on genuine grant denial before the admitted action. Other inspected malformed target checks are pre-action dispatch checks; completed-row scan/apply/validation calls increment before their fallible operations. Mesh/Brep old unit helper arity remains separately owned by root and is not native green evidence. No tests were repeated for this source review.
