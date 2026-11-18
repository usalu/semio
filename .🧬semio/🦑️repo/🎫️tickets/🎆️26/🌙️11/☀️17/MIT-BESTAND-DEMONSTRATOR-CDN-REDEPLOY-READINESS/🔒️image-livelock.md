# Image lease livelock on `native-fixed-image:.cache/b/l`

## Evidence
- Lease registry (`~/Library/Caches/semio/lease-registry/<hash of .cache/b/c>`) showed two `repo mcp` servers (started 12:20 and 12:21) and the two component drivers (about 12:24) in `WAIT native-fixed-image:.../b/l exclusive`, `queued:false` (raw lease, `maximumLeaseDatabaseBytes` set), for 3,300+ s. All started while `l` was unmounted after the 11:36 unclean stop.
- Another process (service host, pid 65840, since 12:24) then activated `l` (exclusive, mount, drop to shared) and holds it shared for good, as do later drivers and cargo children.
- The wait line `remains active on native-fixed-image:... (exclusive, N s)` is the raw wait loop's 1 s progress, not work.

## Root cause
`fixedPersistentState` returns `inactive` for a reader; the opener releases its shared hold and waits in `openPreparedNativeFixedPlatformV1` (`prepare`, `acquire("exclusive")`, `storage/🧱️fixed/🟦️.ts`) for the exclusive writer. The raw lease grants readers first and never re-evaluates why it waits. Once a peer activated the image and stays as a permanent reader, the exclusive request can never be granted, and the reason for it is gone. Every opener that arrived during the inactive window starves; a fresh process sees `mounted` and passes (why one driver alone worked).

## Fix
`storage/🧱️fixed/🟦️.ts`: the exclusive writer wait now passes `onWait` (top-level receivers only) that re-reads `fixedPersistentState`; when it reports `mounted` it throws `FixedPeerActivationV1`. `openNativeFixedPlatformV1` retries the open (`retryNativeFixedOpenAfterPeerActivationV1`) from the shared reader step.

## Verification
`repo capacity run --memory=2048 -- repo tools exec bun -- test ./.../🧱️fixed/🧪️tests/🟦️.ts`: new test passes (5 pass); 1 pre-existing failure needs `SEMIO_TEST_ARTIFACT_DIR`.
Running stalled `repo mcp` processes (pids 61563, 61693) keep old code and must be restarted.
