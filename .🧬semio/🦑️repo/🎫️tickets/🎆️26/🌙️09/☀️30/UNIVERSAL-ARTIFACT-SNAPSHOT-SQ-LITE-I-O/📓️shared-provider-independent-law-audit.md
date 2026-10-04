# Shared Provider Independent Law Audit

## Scope

Read-only inspection of all six Native laws, three Source laws and staging report. No Cargo, source edits or runtime execution. Root's genuine six-law RED remains valid repair authorization; this audit does not replace that receipt. Paths below are relative to `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧪️tests/🧮️allocation`.

## Concrete Coverage Deficiencies Before GREEN

1. **Nonempty copied bytes are never asserted.** Native `🦀️.rs:46–52` drops successful project_text/reconstruct_text/reconstruct_blob outputs. Exact requested byte totals and exhausted admission would still pass if a helper returned same-sized corrupted text/blob. The independent Source SQLite oracle validates fixture bytes through Bun SQLite, not these Rust helper results. Add exact returned text/octet equality for successful project and reconstruction before treating the law as exact semantic-copy proof.
2. **Late cancellation permits unbounded unpaid requests.** Native `🦀️.rs:57` uses requested >= total + final error.message.capacity. This establishes retained payload backing but cannot establish absence of unpaid canceled-path allocations; any extra uncharged allocation passes. Nested diagnostic allocations explain why simply equating the final message capacity is inappropriate, but the lower bound is an explicit evidence limitation. Use a separately attributed diagnostic observer or a narrow payload-allocation observation so actual canceled-path domain backing remains checked. Do not call the current lower bound complete cancellation allocation proof.
3. **Cancellation row-commit evidence is incomplete.** Numeric/IEEE low-allocation branches at42/75 assert no committed row. IEEE initial cancellation at76 checks returned Canceled, requests and retained allowance, but never checks row count; no staged row law cancels after a nonempty owned field is copied and before final row publication. Standalone copy cancellation is covered, but it cannot prove row publication atomicity at that boundary. Add an actual insert cancellation with a large owned text/blob cell and assert the row remains absent while admitted backing remains charged.

These are test evidence gaps, not observations of current implementation corruption or unauthorized publication.

## Narrowly Confirmed Conditions

The observer records actual current-thread alloc/alloc_zeroed/realloc requested sizes, counts full replacements and excludes fixture/setup/expectation allocation from measured intervals. Exact success comparisons for schema, numeric row, ordered frontier and IEEE insertion equate requested totals with cumulative admission. Native cost is measured from the actual producer first and reused for exact/minus-one ceilings. The neutral fixture contains no Rust/BTree ABI estimates. Minimum Vec bounds use size_of only in Native code.

Refusal checks for zero/tiny schema and numeric/ordered/IEEE allowance branches account for already admitted backing plus the returned typed message capacity. OwnershipLimit, Canceled and InvalidValue are asserted separately. Retired schema and string calls share a retained control and demonstrably cannot reset the authored ledger expectations. Both 100000-byte text and blob have genuine interior callback conditions; schema requires an interior SQL-byte callback and ordered frontier cancels inside300 rows. These conditions are materially stronger than initial cancellation alone.

The IEEE success minimum includes five Cell slots; this assumes the current explicit temporary expansion producer. It is a Native source/layout bound rather than a neutral ABI guess, but a future equally valid borrowed/direct emission algorithm may avoid that temporary vector. Observed requested/admitted equality is the durable accounting law; preserve semantic companion correctness rather than requiring incidental temporary storage forever.

## Independent Source Authority

Source `🟦️.ts` uses strict Ajv and the in-repo schema subset validator against the closed corpus and seven distinct hostile changes. Bun SQLite executes the authored two-table SQL, literal text/blob INSERTs, ordered relationships, FK/integrity checks, serialization/reopen and duplicate ordinal SQL. DataView independently reinterprets negative-zero and NaN words and compares signed INTEGER companions and numeric classes. Buffer/SQLite independently establish actual UTF8 and octet lengths, with a valid65536-byte interior frontier. No JSON carrier or external runtime dependency is introduced by these test-only oracles.

The Source arithmetic table for retired copies demonstrates the fixture arithmetic, not runtime Rust admission. Source does not execute Native helper output or cancellation; pair it with the Native returned-value assertions above. The corpus includes all six owner columns and four relation columns through INSERTs and explicit joins/queries, but it is a two-table shared primitive authority, not coverage of every artifact's independent handwritten schema or full public I/O dispatch.
