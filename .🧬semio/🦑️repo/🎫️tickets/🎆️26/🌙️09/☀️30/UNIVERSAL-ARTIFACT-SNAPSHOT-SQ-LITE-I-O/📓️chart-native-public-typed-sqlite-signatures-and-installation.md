# Chart Native Public Typed SQLite Signatures And Installation

Read-only actual current APIs, no execution. Framework IO `🚪️io/🦀️.rs` exposes these under `io_mechanism`, also imported by existing genuine Process public tests through `store::io::io_mechanism`:

```rust
pub async fn io_export_sqlite_snapshot<P: store::ArtifactSqliteSnapshot + 'static>(
    dialect: &ArtifactDialect, snapshot: &P, encoding: SnapshotEncoding,
    limits: SqliteDatabaseLimits, progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool,
) -> IoResult<Vec<u8>>;
pub async fn io_import_sqlite_snapshot<P: store::ArtifactSqliteSnapshot + 'static>(
    dialect: &ArtifactDialect, bytes: &[u8], limits: SqliteDatabaseLimits,
    progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool,
) -> IoResult<P>;
pub async fn io_route(from: &ArtifactDialect, into: &ArtifactDialect, max_hops: u8) -> IoResult<IoRoute>;
pub async fn io_run_with_snapshot_control(
    route: &IoRoute, payload: IoPayload, limits: SqliteDatabaseLimits,
    progress: &mut dyn FnMut(SqliteSnapshotProgress) -> bool,
) -> IoResult<IoPayload>;
```

Definitions are2522,2547,2559,2630. IoResult's successful value is an IoOutcome (`.value` plus diagnostics), not the direct Vec/P/payload. Typed helper2533 resolves an actual registered dialect codec and verifies its snapshot TypeId, so standalone codec creation is insufficient.

Typed export projects actual SQL tables, validates exact P::SQLITE_SCHEMA/subset, attaches physical snapshot metadata and writes pages. Typed import reads pages, removes metadata, requires exact requested dialect, validates schema, reconstructs actual P and applies subset validation. These test direct typed relational behavior; the separate io_route→io_run path additionally tests real native Binary/Text owner decode/encode.

Chart has no editor app and already uses NoPluginApp. Its genuine declaration can bind `.document_codec_bare::<ChartSnapshot,ActualChartMutation>(actual_schema,actual_dialect)`; the actual method is OS Plugin `🦀️.rs:3429–3446`, retaining DocumentCodecSpec's actual bare factory. The declaration is supplied by normal Print print_plugin53–59, which already calls Plugin builder try_build. Builder736–737 obtains assembly plan,785–786 commits; Plugin plan4310 collects real snapshot capability,4378 publishes to Framework IO assembly commit1920–1943. Supply actual explicit ArtifactPack Some override plus the real typed schema/dialect; no test-only side registration is needed after the declaration is coherent.

Existing Process public law publication uses `Plugin::<ActualApps>::builder(...).artifact(crate::declaration().unwrap()).try_build()` (Process snapshot public test9). For Chart use actual `crate::print_plugin()` to exercise its normal initialization, then typed helper/public route with Chart's exact corpus dialect and the framework SQLITE_SNAPSHOT endpoint. The future provider must define actual native ArtifactDsl/ArtifactPack and mutation authority; neither current generic DslValue nor a manufactured dummy mutation earns a public codec claim.
