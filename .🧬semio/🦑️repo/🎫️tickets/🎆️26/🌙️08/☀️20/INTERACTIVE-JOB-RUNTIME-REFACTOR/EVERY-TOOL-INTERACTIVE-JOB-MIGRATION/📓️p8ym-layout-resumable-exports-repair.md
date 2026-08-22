# P8ym Layout Resumable Export Repair

## Status

Layout-owned repair is complete against the P8yl rejection, including integration with the repaired
shared submit/poll/cancel and terminal-credit seam. Cargo, native, release, Wasm, runtime-host,
downloader, and timing gates were not run because this packet explicitly reserves disk and forbids
Cargo.

## Owned Routes

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🦀️component.rs`
  owns the single export state machine and the exact `exportPng`, `exportSvg`, `exportPdf`,
  `exportPackage`, and `export-media:layout:out` factories.
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
  registers both factories, builds them only from framework-provided operation/revision/generation
  and `Arc` snapshots, and makes direct `layout:out` reducer reachability fail closed.
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
  returns a `LayoutExportOperation` from all four public export methods. The handle exposes one-slice
  `step`, progress/checkpoint/preview status, external `cancel`, and move-only terminal data take.
  It uses `semio_framework_async::process_worker_pool`; it does not create a private scheduler. Each
  step compares its submitted generation with the live session generation and cancels/fails stale
  work before a result becomes observable.
- `run_to_completion` remains only in the crate-private, explicitly named headless batch adapter and
  a checkpoint test. The export module is crate-private and the scene compatibility re-export is
  test-only. No Layout UI/Wasm/command/media reducer calls a batch adapter.

## Persistent Stages And Bounds

The job persists Validate, Plan, Encode, Base64, PackageCommit, and Complete state. Route state
includes a display-list cursor, PDF catalog/pages/page-kids/rect/xref-entry/trailer cursors, PNG
scanline/rectangle/256-pixel tile state, incremental Adler and CRC state, package collection,
preflight byte, missing-link scan, manifest, central-directory-entry cursors, base64 3,072-byte input
credit, and final 4,096-byte commit cursor. Final candidate capacity is reserved while empty, then
filled only by bounded chunks, so no terminal flatten/copy remains.

Validation is staged and fail-closed before encoding. The source constants bind 64 pages, 64 parent
pages, 64 spreads, 512 stories, 512 links, 256 paragraph and 256 character styles, 8 frames and
overrides per page, 16 layers and layer IDs per page, 64 guides and spread page IDs, 1,024 total
frames, 2,048 JSON nodes, 8 KiB strings, 64 KiB serialized package values/preflight, 2,048 dimensions,
4,194,304 pixels, three package files, 2 MiB raw payload, 32 MiB output, and 4 KiB checkpoints.
Frames, overrides, margins, columns, guides, layers/object IDs, colors, rotations, insets, link
dimensions/DPI, grids, styles, parent pages, and arbitrary JSON scalars/keys/nodes are finite and
bounded. Composed background/reference values use the same aborting bounded JSON writer, so
serialization stops at the declared fragment credit.

The externally supplied preflight value is not discarded. Its explicit contract is
`layout.preflight-report.array.v1`: one JSON array, at most 64 KiB, 2,048 nodes, depth 64, and 8 KiB
per key/string. The package writer copies its exact source bytes into `preflight-report.json` in
4 KiB units. The focused parity test searches the decoded stored ZIP bytes for the exact supplied
byte sequence.

## Focused Source Tests Added

- exact ActionBus factory dispatch through real WorkerPool instances with 1/2/4/default workers for
  PNG/SVG/PDF/package deterministic bytes;
- exact `export-media:layout:out` reserved-factory dispatch through a real WorkerPool;
- real worker cancellation and stale generation;
- multi-step one-unit yields, preview/checkpoint emission, lossless replay, authority mismatch;
- dimension, top-level collection, nested collection, string, JSON byte/node/schema, and output
  max/max+1 envelopes;
- supplied preflight byte parity;
- standard and split-input incremental CRC-32.

These tests are source-present but unrun, per the no-Cargo constraint.

## Static Gates Run

- `rustfmt --edition 2021 --check` over every modified Layout Rust file plus the new export source:
  passed.
- `git diff --check -- ✏️s/🔌️plugins/📏️layout`: passed.
- Bun TOML parse of Layout Cargo.toml: passed.
- Layout scan for `ImageBuffer`, `ZipWriter`, external base64 engines, whole `.flatten()`, and
  whole `.concat()`: zero matches.
- Production caller scan for the crate-private headless adapters outside export/scene tests: zero
  matches.
- Wasm scan confirms `process_worker_pool`, `LayoutExportOperation`, `step`, `progress`,
  `cancel`, and `takeResultData`; `WorkerPool::new` is absent from the Wasm bridge.
- `run_to_completion` scan has exactly two matches: the named headless adapter and checkpoint test.

## Shared Runtime Integration

Live inspection confirms the repaired submit/poll/cancel seam no longer applies `resolve_ready` to a
pending worker result. The runtime clones a stable per-instance `Arc<RuntimeAppCell>`, releases the
collection lock, awaits only that instance cell, and `poll_owned_media_export` uses
`pending_step.try_recv()`, returning Running on Empty and submitting one next slice only after an
outcome.

The former terminal `serde_json::to_vec(&media)` scan/allocation is absent. The framework request
now supplies one owner-bound `ArtifactMediaExportCredit`. Layout threads that exact credit into its
media job, credits every final candidate chunk during the existing 4 KiB commit cursor, and credits
the fixed `2d.layout` schema bytes once before completing. A credit overflow becomes a job Fault.
The shared fresh terminal poll reads the accumulated credit in O(1), checked-adds the actual public
Structured/Binary payload string lengths in O(1), requires exact equality with producer credit, and
requires the structural sum to remain within the contract cap. Its source regressions accept the
exact maximum, reject credited max+1, and reject an under-credited structural max+1 without
serializing Media.
