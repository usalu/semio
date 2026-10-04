# AVI and MP4 Incremental Live Export

## Scope

This cut completes the Stdio application route from `export-media:playback:out` to paged playback bytes for AVI and MP4. The port remains `playback:out`, the tool remains `export-media:playback:out`, and the shared media type remains Presentation/Sequence. MP4 reports `video/mp4`; AVI reports `video/x-msvideo`.

## Domain-neutral lifecycle

`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🎬️media-export/🦀️.rs` owns the shared live-export job and lifecycle:

- one format cursor advance per interactive job step;
- output pages no larger than 4096 bytes;
- retained output credit, progress checkpoints, cancellation, typed fault detail, and exact close witnesses;
- snapshot retirement releases an operation's shared `Arc` as one bounded item and enters deep `RetireOwned` disposal only when the operation is the final owner.

The earlier use of `shared_retirement` was invalid for operation-local clones because it waited for the valid application cache owner. The focused MP4 cancellation law reproduced this as `Blocked { reason: "media snapshot still has an external owner" }`. `Arc::into_inner` now distinguishes the shared and final-owner cases without losing retirement authority.

## MP4 incremental mux

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs` adds `Mp4EncodeCursor`. It does not materialize the encoded file, `moov`, sample tables, or `mdat`:

- measurement advances retained chunk counts, samples, `stsc`, AVC parameter sets, and HEVC arrays one retained element at a time;
- emission streams box headers, fixed fields, metadata slices, edit entries, AVC/HEVC NAL slices, `stts`/`ctts` runs, sync entries, chunk maps, sample sizes, offsets, and sample payload slices;
- a 257-byte-grant law drains the real fixture in more than 100 advances and asserts byte equality with the committed valid MP4 fixture.

Editor and viewer both register the app-owned factory, declare playback IO, build the live export job, and provide bounded snapshot disposal. MP4 snapshot owners, including `Mp4VisualSampleEntry` and `Mp4Bitrate`, derive `RetireOwned`.

## AVI live route

AVI uses its existing genuine `AviEncodeCursor`; the editor and viewer now expose it through the same shared live-export lifecycle. The focused law loads the real AVI fixture, polls the registered app route, validates MIME/schema/media type, drains bounded pages, and requires exact fixture bytes. A separate cancellation law requires terminal-empty close.

## Independent oracle

The isolated `semio-s-artifact-stdio-mp4-test-oracle` package uses external `mp4` 0.14 and has no subject dependency. Its default feature now enables the oracle, and `independent_reader_accepts_the_exact_playback_fixture` parses the exact fixture independently into typed tracks. The production cursor and live route both assert their exported bytes are exactly that fixture, so the external parse result applies to the emitted artifact rather than a self-round-trip.

The zero-touch target is registered in `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json` as `🧪️test🗄️stdio🎥️mp4🔮️oracle🦀️native`. Focused MP4 and AVI live route targets are registered beside it.

## Validation

- MP4 check: green. Log: `🗑️generated/mp4-live-export-check-2026-10-03-1.log`.
- MP4 default native: 66/66 green, including the new exact incremental mux law. Log: `🗑️generated/mp4-live-export-native-2026-10-03-1.log`.
- MP4 component live route: 2/2 green, 78 skipped. Log: `🗑️generated/mp4-live-export-route-2026-10-03-2.log`.
- AVI default native baseline and exact cursor: 52/52 green from the prior current run.
- AVI component live route: 2/2 green, 62 skipped. Log: `🗑️generated/avi-live-export-route-2026-10-03-1.log`.
- MP4 external parser oracle: 1/1 green with the default `oracles` feature, plus 0 doc tests. The selected test count is nonzero and the external `mp4` 0.14 reader parsed the exact playback fixture. Log: `🗑️generated/mp4-independent-oracle-2026-10-03-2.log`.
