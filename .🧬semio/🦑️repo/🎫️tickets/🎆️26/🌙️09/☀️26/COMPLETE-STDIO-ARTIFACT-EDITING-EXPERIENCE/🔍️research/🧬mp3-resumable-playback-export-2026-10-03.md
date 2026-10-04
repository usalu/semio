# MP3 Resumable Playback Export

## Scope

This checkpoint implements the first genuine Stdio media playback exporter for the MPEG-1 Layer III `any` subset. It covers the shared app-owned media job route, MP3 editor and viewer registration, resource-bound window readiness, exact byte streaming, progress, cancellation, and bounded retirement. AVI now also has a true incremental mux cursor; its live app export route and the MP4 incremental mux remain separate follow-on work.

## Revalidated Authorities

- `semio_framework_plugin::app::ArtifactMediaExportResult` carries a semantic `MediaType`, an artifact schema string, a MIME string, and sealed segmented chunks. MP3 therefore declares semantic `Presentation/Sequence`, schema `stdio.mp3`, and MIME `audio/mpeg` separately.
- `ArtifactMediaExportCredit` and `ArtifactOutputChunks` are the only output ownership authorities. A producer must reserve every emitted byte, seal the queue before completion, and retain the same queue identity in the result.
- `ArtifactMediaExportHandle` binds app instance, parent document, operation, base revision, and generation. Submission and completion validate that exact authority without narrowing `u64` values.
- `ArtifactSnapshotCloseLease` and the app's exact `ArtifactSnapshotDisposer` jointly govern retirement. The job releases its snapshot reference only after the lease witnesses it, and the disposer retires nested snapshot allocation under explicit item and byte grants.
- Editor and viewer apps both pass through `ArtifactApp::build_media_export_job`; `ViewerApp<V>` also needs to forward reserved jobs and snapshot disposers because the live host calls the wrapper type.
- `framework.media.transport@1` names the output port `playback:out` and invokes the registered tool `export-media:playback:out`. The TypeScript contract schema is co-located with its parser test.
- The committed MP3 fixture includes actual ID3 and MPEG frame bytes. The isolated oracle uses the third-party `id3` crate plus a separately written MPEG frame walker. It stays a separately registered cross-implementation project, not a nested Cargo dependency or workspace root.

## Incremental Encoder

`Mp3EncodeCursor` owns only scalar phase, index, offset, measured ID3v2 size, and emitted-byte counters. One `advance` call performs one bounded action:

1. measure at most one ID3v2 frame,
2. emit a bounded slice of the ID3v2 header,
3. emit a bounded slice of one ID3v2 frame header or payload,
4. emit a bounded slice of one MPEG frame header or opaque payload,
5. emit a bounded slice of the ID3v1 trailer,
6. report completion.

The cursor never calls the batch encoder and never constructs the complete MP3 in memory. ID3v2 frame sizes and the tag's syncsafe envelope are computed before their headers are emitted, so an existing ID3 envelope is byte-identical. MPEG payloads remain opaque and retain their exact bytes.

The playback job coalesces cursor slices into fixed 4,096-byte pages. Each worker step consumes one fuel unit and publishes a monotone `u64` progress checkpoint. Completion seals the original queue and publishes the exact MIME/schema result. Encoder or ownership failures publish a bounded UTF-8 job-fault page containing the concrete fault code and message; media polling returns that detail to the caller while retaining the original outcome for bounded close.

## App Plumbing

- The MP3 editor and independent MP3 viewer register their own exact `Mp3MediaExportJobFactory` specializations.
- Both expose the same `playback:out` port and build the same genuine producer against their own app type.
- Both supply the MP3 snapshot disposer.
- The main media window reports `Ready` only for a revision-bound resource and declares `audio/mpeg`. An absent resource reports localized `Unsupported`.
- `ArtifactViewer` forwards media export construction and snapshot disposal through `ViewerApp<V>`.

## Retirement

The job retires cursor, current page capacity, output queue reference, completion authority, credit authority, snapshot reference, and close lease in separate bounded steps. The snapshot disposer stages schema bytes, ID3 frame identifiers and payloads, MPEG payloads, the ID3v1 allocation, and outer vectors. A zero grant releases nothing, and every positive step reports the exact released item and byte debt.

## Validation

- `media-window-typescript-4.log`: six TypeScript files and twelve tests passed after correcting the schema-relative parser fixture.
- `mp3-playback-component-check-2026-10-03-3.log`: the registered MP3 Nx component check completed successfully with `component-app-assembly`.
- `mp3-playback-component-test-2026-10-03-4.log`: the registered MP3 native suite passed 58 of 58 tests.
- `mp3-playback-route-tests-2026-10-03-3.log`: the two focused live route tests passed through the registered MP3 target.
- `incremental_cursor_yields_bounded_pages_matching_the_real_fixture` drains the 193,275-byte committed LAME fixture with 257-byte cursor grants, observes repeated progress and chunks, and requires exact equality with both the fixture and the existing batch encoder.
- `playback_snapshot_retirement_obeys_each_byte_and_item_grant` verifies zero-grant behavior and complete bounded retirement with one item and 4,096 bytes per close step.
- `playback_export_route_is_registered_cancellable_and_fully_retired` exercises live registration, cancellation, transfer to close, and terminal retirement.
- `playback_export_route_streams_the_real_fixture_with_exact_mime_and_bytes` loads the real fixture into the registered editor app, polls the live producer to completion, validates `audio/mpeg`, drains every segmented page, requires exact fixture bytes, and closes every retained owner.
- The registered `identity-round-trip` cross-implementation scenario drains the incremental cursor at 257 bytes per call, requires the exact carrier, and submits the result to the isolated third-party decoder oracle.
- `avi-pack-json-incremental-test-2026-10-03-5.log`: the registered AVI native suite passed 52 of 52 tests, including exact 732-byte parity while draining `AviEncodeCursor` with 257-byte grants.
- `mp3-independent-oracle-2026-10-03.log` is retained as an inconclusive zero-test run. The oracle manifest now default-enables `oracles`, and an unconditional registration guard fails if that feature is disabled.
- `mp3-independent-oracle-2026-10-03-3.log`: the fresh zero-touch registered target passed 8 of 8 tests. The suite includes third-party ID3 decoding, an independent MPEG frame walk over all 462 frames, incremental carrier parity, mutation/inverse checks, and the feature-registration guard.
- The dedicated zero-touch oracle command is registered in both `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json` as `🧪️test🗄️stdio🎵️mp3🔮️oracle🦀️native`; keeping the seed authoritative prevents launch regeneration from deleting it.
- `mp4-pack-json-test-2026-10-03-4.log`: the first post-space-recovery run compiled all 65 tests, passed 58, and stopped after the exact binary `max_file_bytes - 1` case returned `WorkLimit`; six tests were not run due to fail-fast. The cause was Pack's physical `max_file_len` append ceiling entering the generic work-limit branch. The writer now emits a typed `OwnershipLimit` refusal at that exact physical byte boundary. `mp4-pack-json-test-2026-10-03-5.log` is the registered confirmation run.
- The MP4 confirmation remains in progress under local execution session `63842`; its durable log is `🗑️generated/mp4-pack-json-test-2026-10-03-5.log`. At handoff it had reached the Cargo build phase and was waiting on the shared artifact lock, with no compiler or runtime result yet.
