# W5a — 🎞️animate: ad-hoc codec extraction

Agent: W5a, plugin 🎞️animate. Write scope: `✏️s/🔌️plugins/🎞️animate/**` only. stdio was read-only
throughout (no edits made to `✏️s/🔌️plugins/🗄️stdio/**`).

Crate: `semio-s-plugin-animate` (confirmed from `📦️packages/🦀️rust/Cargo.toml`).

## Summary of what changed

| File | What |
|---|---|
| `🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs` | **Deleted outright** the FFmpeg subprocess path (`run_ffmpeg`, `concat_partials`, `mux_audio_track`, `Command::new("ffmpeg")`, `VideoError::FfmpegStatus`). **Rewired** mp4 assembly onto stdio's real `encode_mp4`/`decode_mp4` engine and the gif sidecar onto stdio's real `encode_gif` (89a) engine, both in-process. |
| `🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` | **Rewired** the raw HTML site emitter (`index_html`) to build a real `HtmlSnapshot` and serialize via stdio's real HTML5 `write_html_document`. **Rewired** the SVG/DWG title-card + DWG-import codec sites to round-trip their SVG output through stdio's real SVG codec (`parse_svg_xml`/`write_svg_xml`). |
| `🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🔤️text/🦀️component.rs` | **Isolated** (not deleted) Typst behind a local `TextRenderer` trait + `TypstTextRenderer` impl; output now also feeds a real `SvgSnapshot` via stdio's SVG codec. |
| 4× present `✳️any` io leaf files (md/json import+export) | **Foreign-lag fixes** (not part of this wave's assigned scope) — see "Foreign breakage" below. |
| `🎛️apps/🎬️present/🦀️component.rs`, `📦️packages/🦀️rust/📦️glue.rs` | **Foreign-lag fix** — stale `📌️panels/📄️document` → `📄️artifact` rename call sites. |

## 1. Deleted outright: FFmpeg subprocess path

File: `⚙️engine/🎥️video/🦀️component.rs`, `pub mod writer`.

Deleted: `fn run_ffmpeg(args: &[&str])` (spawned `Command::new("ffmpeg")`), `fn concat_partials`
(ffmpeg `-f concat`), `fn mux_audio_track` (ffmpeg `-c:a aac`), the per-frame PNG-to-partial-dir
staging (`write_frame_png` writing into a filesystem directory for ffmpeg to later encode), the
`VideoError::FfmpegStatus` error variant, and the `std::process::Command` import.

**Proof of deletion**: `grep -c "ffmpeg\|Command::new"` on the file: **7 → 0 real call sites** (3
remaining matches are doc-comment prose explaining what was deleted/mirrored, verified individually
— zero `Command::new(` invocations anywhere in the crate: `grep -rn "Command::new" ✏️s/🔌️plugins/🎞️animate --include="*.rs"` returns nothing outside a doc comment).

**LOC**: `git diff --numstat` on this file: **71 lines removed, 251 added** (file: 1187 → 1367
lines net). The file grew despite deleting FFmpeg because real mp4/gif domain codec logic (raw-frame
Mp4Snapshot encode, decode-and-merge concat, own color quantizer, own spatial scaler) replaced it —
none of that growth is hand-rolled *container-format* byte encoding; all ISO-BMFF/GIF89a byte
structure is built by stdio's real `encode_mp4`/`encode_gif`.

## 2. Rewired: mp4

`SceneFileWriter` now buffers captured RGBA8 frames in memory per partial segment. `finalize_partial`
builds a real `Mp4Snapshot` (one video track, `Mp4Codec::Other` escape-hatch fourcc `"rgb8"`, a real
ISO/IEC 14496-12 §12.1.3 `VisualSampleEntry` box built with stdio's own `write_box`) and encodes it
via stdio's real `encode_mp4`. Concatenation (`concat_raw_partials`) decodes each partial via stdio's
`decode_mp4`, merges the samples, and re-encodes via `encode_mp4` — no FFmpeg, no hand-rolled
ISO-BMFF box assembly anywhere in this plugin; stdio owns 100% of the container byte structure.

**Honest boundary / stdio_gap**: frames are stored **uncompressed** (`Mp4Sample.data` = raw RGBA8
bytes), not H.264. See stdio_gaps §1 below — the task brief's premise that stdio's mp4 engine
"includes a real baseline H.264 encoder" is not accurate as shipped; only NAL/SPS bitstream framing
and `avcC` box construction utilities exist (confirmed by reading `⚙️engine/🎥️h264/🦀️component.rs`'s
own doc comment: the full macroblock/pixel-encode pipeline was deliberately not moved from remodel).
Building a compressed-video encoder inside animate would have meant inventing container-codec logic
myself, which the task explicitly forbids — reported, not fabricated.

## 3. Rewired: gif

Own domain code (not duplicated into stdio, per the task's explicit instruction that stdio's gif
engine should not grow scaling logic):
- `nearest_neighbor_scale` — spatial downscale mirroring the deleted `scale=640:-1` ffmpeg filter.
- Frame decimation mirroring the deleted `fps=15` ffmpeg filter (own `step_by` over captured frames).
- `gif_palette`/`nearest_cube_index` — a fixed 6×6×6 uniform color cube (216 colors, padded to a
  valid power-of-two 256-entry GCT per the schema's own documented allowance for padding entries),
  direct-arithmetic nearest-index quantization (no external crate, no brute-force search).

Output is a real `GifSnapshot` (89a) encoded via stdio's real `encode_gif`.

## 4. Rewired: HTML site emitter

`compile_present_site`'s `🌐️index.html` generation (`index_html`, previously a hand-rolled
`format!("<!DOCTYPE html>...")` string with manual `&`/`<` escaping) is now `index_html_snapshot`: a
real typed `HtmlSnapshot` tree (`Element`/`Text`/`RawText` nodes, `RawTextKind::Script` for the deck
JSON `<script>` and the two module `<script>` tags) serialized via stdio's real
`write_html_document`. This is also **more spec-correct** than the deleted emitter: HTML5's RAWTEXT
content model never entity-decodes `<script>` content, so the old `&`/`<` string-replace would have
literally corrupted any deck JSON string containing those characters once a real browser's
`textContent` read it back — the new code embeds the JSON verbatim, matching real parser behavior.

`styles.css`/`player.js`/`manifest.json`/`deck.json` are plain CSS/JS/JSON sidecars, not HTML — there
was no ad-hoc HTML codec logic at those specific `fs::write` sites, so they are unchanged.

## 5. Isolated (not deleted): Typst

`⚙️engine/🔤️text/🦀️component.rs`: added a local `pub trait TextRenderer { fn render_svg(&self, markup:
&str) -> Option<String>; }` + `pub struct TypstTextRenderer` implementing it (wraps the pre-existing
`typst_markup_to_svg`). All three construction sites (`Text::new`, `Code::new`, `MathText::new`) now
go through `typst_markup_to_validated_svg(&default_text_renderer(), markup)`, which additionally
parses the renderer's SVG output through stdio's real `parse_svg_xml` into a genuine `SvgSnapshot`
and re-serializes via `write_svg_xml` before handing the (now stdio-validated) text to the existing
`usvg`-based geometry extractor (`svg_to_vobject`) that feeds the Vello renderer.

This satisfies both halves of the instruction: the external Typst library is isolated behind an
interface (CLAUDE.md), and the renderer's output feeds a real `SvgSnapshot` encoded via stdio's SVG
engine. `svg_to_vobject`'s own `usvg`-based path-geometry extraction was deliberately left unchanged
— `usvg` does full `<use>`/`<defs>`/CSS resolution that stdio's structural SVG codec does not attempt
(a rendering concern, not a duplicated codec), and reimplementing that resolution inside animate to
avoid `usvg` entirely would be far outside this task's explicit boundary and duplicate real,
non-trivial rendering logic that already exists and works.

## 6. Rewired: SVG/DWG MediaCodec sites

`animate_present_document_json_to_svg` (title-card export) and `animate_present_document_json_from_dwg`
(DWG import): the framework helpers they call (`semio_framework_os::title_card_svg`,
`dwg_drawing_to_svg`, `rasterize_svg_to_png_base64`) are **shared, non-duplicative framework
utilities**, not local ad-hoc codec code written by animate — there was no hand-rolled SVG/DWG byte
encoding inside this plugin to delete. Both sites now round-trip their SVG string through stdio's
real SVG codec (`parse_svg_xml` → `write_svg_xml`) before use, validating it as genuinely
spec-conformant SVG and exercising the real stdio engine. See stdio_gaps §3 for why a deeper
`DwgDrawing → semio/drawing` rewrite was not attempted.

## stdio_gaps (for the orchestrator — genuine gaps, not workarounds)

1. **mp4 has no real pixel encoder.** `⚙️engine/🎥️h264`'s own doc comment confirms only NAL/SPS
   bitstream framing + `avcC` box construction were moved from remodel; the macroblock/pixel-encode
   pipeline was deliberately not moved. The task brief's "includes a real baseline H.264 encoder —
   use it" does not match what's shipped. Worked around honestly via `Mp4Codec::Other` with an
   uncompressed `"rgb8"` escape-hatch codec (real container, no compression, clearly documented as
   such) rather than fabricating H.264 frames.
2. **`Mp4Track`/`Mp4Snapshot` only model video-handler (`vide`) tracks.** `decode_trak` requires
   `hdlr[8..12] == "vide"`, else the whole `trak` is retained as an opaque `unknown_boxes` blob —
   there is no schema slot for a real audio track. Audio muxing (`config.audio_track`) was dropped
   from `encode_outputs` rather than hand-rolling an ISO-BMFF `'soun'` trak myself (that would
   duplicate container-format work stdio should own). `AnimateConfig.audio_track`/`with_audio_track`
   (in `⚙️engine/🎛️config/🦀️component.rs`) were left untouched — plumbing, not codec logic, and not
   named in this wave's scope.
3. **No bridge exists from the legacy `semio_framework::DwgDrawing` (11 geometry variants: Line/
   Point/Circle/Arc/Ellipse/LwPolyline/Spline/Text/Face3d/Polyline3d/PolyfaceMesh) to semio's
   `SemioDrawingSnapshot`/`DrawNode` tree.** Writing one inside animate would duplicate
   `semio_framework_os::dwg_drawing_to_svg`'s existing correct geometry logic for a type W6 deletes
   outright, and would be the 1st of ~9 near-identical reimplementations across the W5a/W5b svg/dwg
   pattern plugins. Recommend a single shared converter in a future wave instead.
4. **`JsonSnapshot.value` (`JsonValue`) has no built-in structural bridge to `serde_json::Value`** —
   intentional per that schema's own doc comment ("No `serde_json::Value` anywhere in this file"),
   but it means every caller needing ordinary serde interop must hand-roll one (done locally in the
   two json leaf files touched below; not fixed at the source, out of this wave's write scope).

## Foreign breakage found + fixed as lagging call-site completions

These blocked `cargo check -p semio-s-plugin-animate` for the WHOLE crate (including code this wave
never touched) and were fixed only for that reason — confirmed foreign via `git log`/`git status`
before touching anything, per the hazard-management protocol ("lagging call-sites of landed foreign
refactors may be completed").

1. **`📌️panels/📄️document` → `📄️artifact` rename never propagated to this plugin's own glue.rs.**
   `git log` shows the directory rename landed in commit `c31024cc6c` (2026-08-10 23:04, a full day
   before this ticket opened) as part of a repo-wide "document → artifact" terminology migration, but
   `📦️packages/🦀️rust/📦️glue.rs:485` (`pub mod document`) and 3 call sites in
   `🎛️apps/🎬️present/🦀️component.rs` were never updated, and the actual `📄️document` directory no
   longer exists (renamed to `📄️artifact`). `git status` on both files was clean (not mid-edit by
   anyone). Fixed: `pub mod document` → `pub mod artifact` in `glue.rs` (path + module name), and the
   3 call sites (`use ... panels::{catalogue, document, inspection}`, `document::render(...)`,
   `document::definition()`, `pub use document::PRESENT_PLAY_BODY_DOCUMENT`) renamed to `artifact`.
   **Note for the orchestrator**: the identical stale `📌️panels/📄️document` path exists in ~18
   *other* plugins' `glue.rs` files (raster/remodel/flow/process/cad/block/dag/sequence/writer/
   reasoning/vcs/imperative/forms/shooting/layout/puzzle/lowpoly/animate) — only animate's was fixed
   here (in scope); the rest are out of this wave's write scope and flagged for a dedicated pass.
2. **Present's own pre-existing `✳️any`-subset io leaves (md + json, both directions — 4 files)
   broke against a concurrent stdio schema change** that landed mid-session: `MdSnapshot.body: String`
   → `blocks: Vec<MdBlock>`, `JsonSnapshot.value: serde_json::Value` → stdio's own `JsonValue` (commit
   `ad0fc0019b`, one of the 5 most-recent repo commits at session start). `git status` on all 4 files
   was clean before I touched them. Fixed as minimal lagging-call-site updates, preserving the exact
   same degenerate placeholder semantics as before (not a real present↔markdown/json mapping — that
   was never in scope for these leaves):
   - md export/import: wrap/unwrap the printed DSL text in a single `MdBlock::Paragraph{inlines:
     [MdInline::Text]}` instead of a bare `body` string.
   - json export/import: a real structural `serde_json::Value ↔ JsonValue` converter (stdio provides
     none — stdio_gap §4 above), plus switched `serialize_bytes`/`deserialize_bytes` onto stdio's real
     `write_json_pretty`/`parse_json_text` text codec — the previous
     `serde_json::to_vec_pretty(&tagged_JsonValue)` would have serialized `JsonValue`'s
     internally-tagged shape verbatim (not real JSON text), a latent bug this fix also corrects.
3. **`semio-framework-os-kernel` (an upstream dependency of this crate) was actively being edited by
   another concurrent session** during this wave's verification — confirmed via `git status` showing
   live unstaged changes to `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
   and `.../🏪️store/🦀️component.rs`, and by watching the exact error set change across 4 consecutive
   `cargo check` runs (2 errors → 3 errors → 1 error → 0 errors) as that session iterated. Not touched
   (framework/os-kernel is outside this plugin's write scope) — simply re-ran the check until the
   dependency stabilized, per "poll rather than chase."

## Exit checklist

`cargo check -p semio-s-plugin-animate 2>&1 | tail -40` (full output: `w5a--animate-cargo-check.txt`):

```
warning: unnecessary parentheses around block return value
   --> ✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/./././././../../🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎬️scene/🦀️component.rs:598:9
    (pre-existing, unrelated file, not touched by this wave)

warning: unused import: `PresentSnapshot`
  --> .../🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs:13:90
    (pre-existing, unrelated file, not touched by this wave)

warning: unused import: `ArtifactBuilder`
 --> .../🪆️subsets/✳️any/🎹️composer/🦀️component.rs:4:48
    (pre-existing, unrelated file, not touched by this wave)

warning: hidden lifetime parameters in types are deprecated
  --> .../🪆️subsets/✳️any/🎹️composer/🦀️component.rs:29:27
    (pre-existing, unrelated file, not touched by this wave)

warning: field `artifact` is never read
   --> .../⚙️engine/🦀️component.rs:772:5 (PresentEngine.artifact — pre-existing dead field, unrelated
    to any of this wave's edits in the same file)

warning: `semio-s-plugin-animate` (lib) generated 5 warnings (run `cargo fix --lib -p semio-s-plugin-animate` to apply 3 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 45.23s
```

**0 errors. 5 pre-existing warnings, none in any file this wave modified with new logic.**

`cargo test -p semio-s-plugin-animate --lib 2>&1 | tail -30` (full output: `w5a--animate-cargo-test.txt`):

```
test artifacts::present::standards::v1::engine::video::writer::tests::nearest_neighbor_scale_downsizes_dimensions ... ok
test artifacts::present::standards::v1::engine::video::writer::tests::build_gif_snapshot_quantizes_and_downscales ... ok
test artifacts::present::standards::v1::engine::video::writer::tests::writer_writes_srt_from_sections ... ok
test artifacts::present::standards::v1::engine::video::writer::tests::writer_writes_png_sequence_frame ... ok
test artifacts::present::standards::v1::engine::video::writer::tests::writer_buffers_frame_and_finalizes_a_real_decodable_mp4 ... ok
test artifacts::present::standards::v1::engine::video::writer::tests::concat_raw_partials_merges_sample_counts_and_stays_decodable ... ok
test artifacts::present::standards::v1::engine::video::render::tests::render_scene_writes_last_frame ... ok
test artifacts::present::standards::v1::engine::component::compiler::tests::compile_scene_to_assets_writes_mp4 ... ok
test artifacts::present::standards::v1::engine::component::compiler::tests::compile_present_site_writes_static_bundle ... ok
test artifacts::present::standards::v1::engine::component::tests::animate_present_document_json_to_svg_embeds_title ... ok
test artifacts::present::standards::v1::engine::component::tests::animate_present_document_json_to_svg_falls_back_to_app_label_without_title ... ok
test artifacts::present::standards::v1::engine::component::tests::from_dwg_builds_single_slide_deck_from_entity ... ok
test artifacts::present::standards::v1::engine::component::tests::from_dwg_never_errors_on_empty_drawing ... ok
test artifacts::present::standards::v1::engine::text::text::tests::typst_plain_text_compiles ... ok

test result: ok. 208 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.75s
```

**208 passed, 0 failed.** New tests added (extended the existing `//#region`-marked test modules —
no new test files): `writer_buffers_frame_and_finalizes_a_real_decodable_mp4`,
`writer_writes_png_sequence_frame` (replaces the old `writer_writes_png_frame`),
`concat_raw_partials_merges_sample_counts_and_stays_decodable`,
`build_gif_snapshot_quantizes_and_downscales`, `nearest_neighbor_scale_downsizes_dimensions`.
Pre-existing tests that now exercise the rewired real codecs end-to-end and still pass:
`compile_scene_to_assets_writes_mp4` (full `render_scene` → real mp4 pipeline),
`compile_present_site_writes_static_bundle` (full real `HtmlSnapshot` site build),
`animate_present_document_json_to_svg_embeds_title`/`_falls_back_to_app_label_without_title`,
`from_dwg_builds_single_slide_deck_from_entity`/`from_dwg_never_errors_on_empty_drawing`,
`typst_plain_text_compiles`.

## LOC deleted (grep/diff proof, before → after)

| File | Metric | Before | After |
|---|---|---|---|
| `⚙️engine/🎥️video/🦀️component.rs` | `ffmpeg\|Command::new` occurrences | 7 | 0 real calls (3 doc-comment mentions only) |
| `⚙️engine/🎥️video/🦀️component.rs` | total lines | 1187 | 1367 |
| `⚙️engine/🎥️video/🦀️component.rs` | `git diff --numstat` | — | -71 / +251 |
| `⚙️engine/🦀️component.rs` | total lines | 753 | 794 |
| `⚙️engine/🔤️text/🦀️component.rs` | total lines | 550 | 605 |

The net LOC growth reflects real domain codec work (raw-frame mp4 muxing, gif quantization/scaling,
typed HTML tree construction, the `TextRenderer` trait) replacing the deleted FFmpeg calls and raw
string emitters — not a like-for-like shrink, since the FFmpeg path was doing real work (encoding)
that now has to happen in-process via stdio instead.

## Not touched (explicitly out of this wave's scope)

- `🖼️images` engine / PLY/LAS/OBJ exporters — that extraction belongs to 📸️remodel, not animate.
- `write_png_file` / `OutputFormat::LastFrame` / `OutputFormat::PngSequence` PNG writes (still via the
  `image` crate) — a real library call, not a subprocess, not named in the delete/rewire/isolate list
  for this plugin; left unchanged.
- `AnimateConfig.audio_track` field/builder/test in `⚙️engine/🎛️config/🦀️component.rs` — plumbing,
  not codec logic (see stdio_gap §2).

## Files touched (created none; edited only)

- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🎥️video/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/⚙️engine/🔤️text/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/📦️glue.rs`

## Naming-collision note for the orchestrator

This ticket folder already contained generic `w5a--report.md`/`w5a--cargo-check.txt`/
`w5a--cargo-test.txt` from another W5a agent (🔋️energy) using the exact same unqualified filenames
this wave's prompt template specifies. No collision occurred here (verified content before touching
anything; used `w5a--animate-*` filenames instead), but the generic `w5a--<description>.txt` naming
convention in the wave prompt template is unsafe for parallel same-wave agents and should be
plugin-qualified (e.g. `w5a--<plugin>-<description>.txt`) in future dispatches.
