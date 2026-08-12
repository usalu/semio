# W6 Migrate — `✏️s/🔌️plugins/🏭️process` — MediaFormat Retirement

## Scope
Write scope: `✏️s/🔌️plugins/🏭️process/**` only. Did not touch `🧰️framework/**` or `✏️s/🔌️plugins/🗄️stdio/**`.

## Files with literal `MediaFormat` references (the census's "2 files") — both now grep-clean (0)

1. `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs`
   - Dropped `MediaFormat` from the `semio_framework_plugin::{...}` import list.
   - Rewrote a code comment that named `MediaFormat::Step::is_binary()` to describe the "stdio.step" format instead, no type reference.
   - `ArtifactKindSpec` literal in `create_process3d_app()`: `export_formats`/`import_formats` (`Vec<MediaFormat>`) emptied to `vec![]`; the real format list moved onto the already-present (previously-empty) `export_stdio_kinds`/`import_stdio_kinds` string peers: `["stdio.step", "stdio.obj", "stdio.stl", "stdio.gltf"]` / `["stdio.step", "stdio.obj", "stdio.stl"]` (GLB has no dedicated stdio artifact kind — it is the binary `sourceForm` of `stdio.gltf`, confirmed against `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/**`). Also fixed pre-existing indentation drift on this literal.
   - This exactly mirrors the pattern already established by W5 in the sibling file `🗿️artifacts/🧊️process3d/🦀️component.rs`'s `artifact_kind()` (same plugin), which already carries `export_stdio_kinds: vec!["stdio.dwg", "stdio.gltf", "stdio.ifc", "stdio.json", "stdio.obj", "stdio.png", "stdio.step", "stdio.stl"]` — I intentionally kept my list to only the formats this app actually declared (step/obj/stl/glb export, step/obj/stl import) rather than expanding to that sibling's broader set, to avoid changing behavior beyond a type/representation swap.

2. `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
   - `process3d_io()`'s `AppIo.export_formats`/`import_formats` (`Vec<MediaFormat>`) emptied to `vec![]`. Unlike `ArtifactKindSpec`, `AppIo` has **no** `export_stdio_kinds`/`import_stdio_kinds` string peer in the framework yet. Verified these two `AppIo` fields are dead weight at runtime — `register_app_io`/`OsAppRegistration` (`🧰️framework/🛍️products/💻️os/🦀️component.rs`) never reads `AppIo.export_formats`/`import_formats` at all (grepped `.io.export_formats` repo-wide: zero hits), so emptying them is a no-op functionally, not a regression. This mirrors the sibling `artifact_kind()` (already empty) per the fn's own doc comment ("mirrors `artifact_kind()`'s literal ... copied verbatim").
   - `export_process3d_model`'s GLB branch: replaced `let media_format = semio_framework_plugin::MediaFormat::Glb;` (+ `.as_str()`/`.mime_type()` calls) with the equivalent literal constants `"process3d.glb"` filename and `"model/gltf-binary"` mime type — byte-for-byte the same values `MediaFormat::Glb` produced, just no longer routed through the enum.
   - Note: `exporter.format()` (used a few lines below, from `semio_framework_3d::brep::kernel::SolidExporter`) still returns `semio_framework::MediaFormat` — that trait lives in `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/`, framework-owned and out of my write scope, and my file never spells the type name for it (only `.as_str()`/`.is_binary()`/`.mime_type()` method calls), so it doesn't affect the grep count and I left it untouched — flagging it here since it's a remaining real `MediaFormat`-typed surface for whichever wave migrates `framework/3d`.

## Stale-test fallout fixed (same plugin, no new `MediaFormat` text, but blocking `cargo test`)
Both were pre-existing assertions from W5's incomplete pass, in files this task's write scope covers:
- `⚙️engine/🦀️component.rs` test `process3d_io_mirrors_the_declared_artifact_kind`: asserted `export_formats.len()==4`/`import_formats.len()==3`; now asserts both are empty (with a doc comment explaining why), matching the code above.
- `🗿️artifacts/🧊️process3d/🦀️component.rs` test `artifact_kind_declares_the_expected_media_surface`: same stale-count assertions (already broken before I touched anything — `export_formats`/`import_formats` were already `vec![]` there from W5, but the test still asserted `len()==4`/`len()==3`); now asserts both empty and additionally asserts `export_stdio_kinds == import_stdio_kinds` with `len()==8`, covering the real data that replaced them.

## Unrelated pre-existing breakage fixed (blocking compilation entirely, discovered via baseline check)
`✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs` had two stale `#[path=...]` attributes pointing at a `📄️document` directory that no longer exists — both `🎛️apps/🧊️3d/🎮️commands/📄️document` and `🎛️apps/🧊️3d/📌️panels/📄️document` were renamed to `📄️artifact` on disk (per `git log`) without updating glue.rs. Nothing else references the old paths (only the `document`/`document_panel` Rust identifiers in the apps file, which I left alone — only the `#[path]` targets were stale). Fixed both to point at `📄️artifact`. Unrelated to MediaFormat; without this fix `cargo check -p semio-s-plugin-process` fails immediately with "No such file or directory" before reaching any MediaFormat-adjacent code.

## Exit checklist
- `grep -c "MediaFormat"` on both census files: **0** and **0**.
- Plugin-wide `grep -rl "MediaFormat" ✏️s/🔌️plugins/🏭️process/`: only remaining hit is the doc comment I added in `🗿️artifacts/🧊️process3d/🦀️component.rs` explaining the retirement (prose, not a type reference).
- `cargo check -p semio-s-plugin-process` / `cargo test -p semio-s-plugin-process --lib`: **currently fail**, but not because of anything in this migration. The failure (3× `E0308`/`E0308`-family "expected `Value`, found `JsonValue`") is entirely inside `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`, which consumes `semio_s_plugin_stdio::artifacts::json::JsonSnapshot` — a struct whose `value` field type is being migrated live, right now, inside the (off-limits) `🗄️stdio` plugin: `git status` shows hundreds of concurrently-modified files under `✏️s/🔌️plugins/🗄️stdio/**`, and repeated `cargo check` runs a minute apart returned *different* error sets (18 stdio errors → 1 stdio error → this stable 3-error `JsonValue` mismatch in my plugin), confirming an in-progress refactor elsewhere, not a fixed state. This file has zero relation to `MediaFormat` and zero local git diff — I left it untouched, per the ticket boundary forbidding `🗄️stdio` edits and per the "concurrent workspace churn" pattern (poll, don't chase). Full logs: `w6-migrate--process-cargo-check.txt`, `w6-migrate--process-cargo-test.txt` in this folder.
- Both target files individually confirmed free of the 3 blocking errors — none of the 3 `cargo check` errors trace to either MediaFormat-migrated file.

## Files touched
- `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️component.rs` (stale-test fix only, no MediaFormat text added except one doc comment)
- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs` (unrelated stale-path fix, required to compile at all)

## Not fixed / flagged, not fabricated
- The `JsonValue`/`Value` mismatch in the JSON deserializer (see above) — out of scope (root cause is in `🗄️stdio`, currently being edited by another session) and unrelated to `MediaFormat`.
- `semio_framework_3d::brep::kernel::SolidExporter`/`SolidImporter::format() -> MediaFormat` (framework trait) — still real `MediaFormat` usage, but it's a framework-owned signature outside this plugin's write scope; my file only calls it, never names the type.
