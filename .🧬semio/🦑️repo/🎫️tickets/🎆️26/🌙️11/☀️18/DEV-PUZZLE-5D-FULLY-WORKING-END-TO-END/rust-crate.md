# puzzle-5d-rs final status

## Fixed
- 5d `🧪️tests/🔬️unit/🦀️.rs`: serde on schema types replaced by the schema-owned codec (`semio_framework_pack_json::from_json_str` / `to_json_string`) through `decode`/`encode` helpers; assertions unchanged. Coordinator confirmed it compiles.
- 5d binary snapshot unit test (`🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`): `puzzle5d_store` now imported from `crate::host::owned`. Compiles.

## Not run to a result
- `repo run @semio-tech/puzzle-5d-rs:<target>` for build, check, test, graph-wire-check and the verify-* targets: first wave stopped by the coordinator (stale, pre-fix, starving host memory). Build and four first-run targets also hit graph-service failures ("Error: closed", `readPhysicalSource`). No target has a pass/fail result.
- Full `--lib` test run: owned by the sqlite agent once its file compiles.

## Next
Rerun the targets once the host is quiet, at most 3 at a time. Logs: generated/rs-*.log.
