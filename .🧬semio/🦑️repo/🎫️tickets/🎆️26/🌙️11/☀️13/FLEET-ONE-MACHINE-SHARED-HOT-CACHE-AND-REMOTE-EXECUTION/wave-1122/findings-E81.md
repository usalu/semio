# E81 findings (17:15)
1. Row recording is not the cause of the 13/53 re-inferences (measured on the 422 stored records: zero source rows of scheduling domains; the only scheduling-prefix rows are one Cargo.toml). Dependency-crate Cargo.toml `field`/`exists` rows of cargo plugin owners are the likely mover; confirm with `[graph-stale]` of the next read before changing any recording.
2. Test `walk-views` "a target's own closure holds ..." depends on esbuild's service and times out at load 33-40 (120 s): wrap esbuild use with the stretched deadline (E69 `stretchedMillisecondsV1`) or run it with a retry.
3. Observation main test roster fixture lacks peer exports finishOnLoadedImplementationV1 / loadedImplementationRecordedFactV1 (fixtures/🔣️.json `semantic.declarationExports`).
