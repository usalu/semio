# MD25 and HTML23 Current Runtime Receipts

Root used the existing Bun/Nx Native snapshot routes, the quick Nextest profile, and the paired private Cargo target/build directories for this ticket. Counts describe selected snapshot laws, not complete artifact libraries.

## CommonMark

Route: `bun nx run @semio-tech/stdio-md-rs:test-snapshot-sqlite-native --skip-nx-cache`.

Nextest run `cf89b1c9-3b33-4c12-b438-40410e58db11`: 25 laws executed, 24 passed, one failed, 47 outside the selection. Runtime 750 ms; Nx critical path 18.8 seconds. All original 24 laws passed, including the linear retirement test with denied allocation and the isolated late final-SQL cancellation test. The newly added public allocation law failed because native backing ownership still uses the semantic value ceiling. Shared Store consumer changes remain subject to their own actual baseline before mounting.

## HTML

The first run stopped before assertions at three typed invariant terminal compiler errors. The responsible owner repaired those prerequisites without mounting the new behavior.

Route: `bun nx run @semio-tech/stdio-html-rs:test-snapshot-sqlite-native --skip-nx-cache`.

Nextest run `bb4bbadf-2870-46d1-a56a-5da486b588e3`: 23 laws executed, 18 passed, five failed, 43 outside the selection. Runtime 516 ms; Nx critical path 24.3 seconds. All original 18 laws passed. Real failures covered first-body normalization, exact normalized returned-row limits, cumulative native owned admission, allocation-denied deep/wide retirement, and late metadata cancellation after a complete deep tree exists. The two lifecycle laws each ran an isolated child. These five failures authorized only the corresponding previously staged production repairs; fresh Native23 verification remains required.

## Limits

Neither receipt proves complete artifact-library coverage, universal dispatcher lifetime admission, or all persisted artifact owners. The goal remains active.
