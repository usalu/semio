# Incremental Dependency Evidence Decoding

[R197](result-R197.md) identified the dependency stage as the resident-size high-water mark of the 8 GiB readiness run. [R212](result-R212.md) placed ownership walks and receipt verification near that peak. [R220](result-R220.md) reduced ownership observations and generation receipt encoding. The current incremental decoder still built an array of parsed JSON rows, each with its nested arrays, then kept it while producing the emitted ledger or index records.

`verifyJavascriptDependencyInstalledReceiptV1` uses this decoder during sync, resume and donor verification. `readJavascriptDependencyOwnershipIndexV1` uses it before the index can seed an ownership walk. These consumers need the final records. They never need the intermediate collection of raw JSON arrays.

## Change

The incremental codec validates the evidence length and digest, decodes UTF-8, and projects each JSON line into its final record before parsing the next line. The buffer and decoded text remain bounded by the existing policy. The projected records, exact row count, byte sort, installed directory connectivity and per-kind counts still pass the same checks. The reader performs one digest, one UTF-8 decode and one JSON parse per accepted row, with no added filesystem operations. Installed byte ordering now encodes each path once, and the ownership decoder computes its constant prefix once.

## Verification

The test fixture was red before the patch. Both decoders parsed the entire fixture before projecting a row. After the patch, only one raw row awaited projection at a time, and SQLite independently produced identical projected records. The malformed-evidence test covers seal, size by digest reference, row count, invalid UTF-8 and disconnected paths. The test is included in the declared `javascript-incremental-sync` suite.

[The private probe](measure-dependency-evidence-decode.ts) decoded 24,000 rows of each kind in fresh Bun processes. Each input was constructed before a forced collection outside the measured decode. Each reported range contains two processes. These are process peak resident sizes on this macOS host, including process startup.

| Fixture | Before | After |
| --- | --- | --- |
| Ownership index, four children per row | 79.9–80.1 MiB | 75.0–75.6 MiB |
| Installed ledger | 80.2–80.5 MiB | 74.9–75.6 MiB |

The final retained projected heap after collection was within 0.1 MiB of baseline for both. Wall times varied as the host ran other work. No end-to-end time or 8 GiB physical-footprint inference follows from the probe.

The four-file focused run passed 17 tests, including the three new cases, installed mutation and receipt laws, Python and SQLite comparisons, and progress and repair tests. The existing ownership seeding test failed in its initial cold child at `run("cold")` with `Bun published workspace patterns drift: package.json`. That child imports the workspace facade and derives the tree before the test reaches either incremental decoder. The complete workspace stage and readiness lab were outside this verification. The scoped whitespace check and declared suite lookup passed.

The follow-up [ownership fixture audit](audit-dependency-ownership-fixture.md) records the corrected fixture and the green four-file rerun.
