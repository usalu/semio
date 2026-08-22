# P8yk Layout Resumable Exports

## Scope

Layout's four public export tools are now one explicitly registered Layout-owned cohort:

- `exportPng`
- `exportSvg`
- `exportPdf`
- `exportPackage`

The exact payload schema is `layout.layout.tool-command.v1`, the document schema is `layout.layout`, and the concrete factory owner is `EditorApp<LayoutPlayApp>`.

## Inventory and Reachability

- Registrations and typed command rows: `✏️editor/🦀️component.rs`.
- Public command handlers: `✏️editor/🎮️commands/🐚️export-{png,svg,pdf,package}/🦀️component.rs`.
- Engagement consumer: `✏️editor/🎮️commands/🐚️engagement-submit/🦀️component.rs`.
- Media and Wasm batch consumers: `✏️editor/🦀️component.rs` and `✏️editor/🌉️wasm/🦀️component.rs`.
- Former terminal scene implementation: `✏️editor/⚙️engine/🎬️scene/🦀️component.rs`.
- Resumable implementation and same-job batch adapters: `✏️editor/⚙️engine/📤️export/🦀️component.rs`.

The four direct command handlers now fail closed with `layout-export-job-only`. Runtime dispatch is admitted only through `LayoutExportJobFactory` and `ArtifactEditor::build_tool_job`. Engagement input redispatches the exact public export action. Media/Wasm compatibility entry points drive the same `LayoutExportJob`; none retains a separate exporter.

## State Machines and Bounds

The persistent stages are Validate, Plan, route Encode, Base64, and Complete. Validation walks pages, stories, and links incrementally. Planning traverses inherited and page frames incrementally. SVG emits one primitive per unit. PDF emits one object per unit. PNG keeps a scanline plus rectangle and 256-pixel tile cursors. Package output keeps JSON entity, ZIP entry, data-descriptor, and central-directory cursors. Base64 consumes at most 3,072 input bytes per unit.

The factory contract is one work unit, 2,000 microseconds, 2 MiB raw input, 5,184 decoded items, 32 MiB exact output, checkpoint every 64 units, and progress every step. Source caps additionally cover pages, frames/page, total frames, stories, links, 16 KiB package fragments/string bytes, dimensions, pixels, files, and checkpoint bytes.

Checkpoint state records kind, page, parent document, canonical base revision, operation, generation, completed units, byte length, and digest. Restore deterministically replays bounded units and verifies the recorded byte length and digest before exposing a preview. Completion is single-assignment and only handed to the framework as a commit candidate; the framework performs live revision/generation validation before applying the `DownloadMediaExport` effect.

The former whole display-list raster loops, `image`/PNG encoders, `ZipWriter`, whole package serialization, and direct `base64::STANDARD.encode` paths were removed. Layout's direct `image`, `png`, `zip`, and `base64` Cargo dependencies were removed.

## Focused Source Tests

`✏️editor/⚙️engine/📤️export/🦀️component.rs` contains focused tests for:

- deterministic bytes at one, two, four, and default-sized schedules for every route;
- dimension maximum plus one rejection;
- repeated one-unit yields and stale generation rejection;
- bounded, lossless, authority-qualified checkpoint restore.

The job implementation additionally has explicit cancellation polling, exact output checks, page/entity/file/string/pixel envelopes, checkpoint digest validation, and single-completion protection.

## Gates Run

- `rustfmt --edition 2021` on all touched Layout Rust sources: passed.
- Layout-scoped forbidden implementation scan for `image::`, `png::`, `zip::`, `ZipWriter`, `ImageBuffer`, direct base64 encoding, and whole document serialization in command handlers: no export-engine/handler hits.
- Layout-scoped registration/fail-closed reachability scan: exact factory/build hook and all four fail-closed handlers present.
- `git diff --check -- ✏️s/🔌️plugins/📏️layout`: passed.

## Not Run

Per P8 migration constraints, Cargo/native tests, Wasm compilation/runtime, plugin host/runtime dispatch, and download integration were not run. The source tests therefore exist but are not claimed passing. Descriptor generation was not rerun; the registration contract is runtime schema-surfaced by the accepted framework owner-qualified factory hook rather than a Layout descriptor file.
