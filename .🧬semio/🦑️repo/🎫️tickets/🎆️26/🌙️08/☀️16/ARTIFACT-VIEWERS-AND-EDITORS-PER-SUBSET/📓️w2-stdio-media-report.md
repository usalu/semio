# W2 Packet P1-stdio-media Report

Lane: W2 packet P1-stdio-media, plugin `🗄️stdio`, 17 subsets across 12 kinds:
`📷️png`(1) `📷️jpg`(2) `🖼️bmp`(1) `🖼️tiff`(2) `🎞️gif`(2) `🎨️svg`(3) `🎥️mp4`(1) `🎵️mp3`(1) `🔊️wav`(1)
`📼️avi`(1) `🌐️html`(1) `📝️md`(1). None of these kinds ever had a `🎛️apps/` app to migrate (stdio is a
zero-app "schema-owned library" plugin, confirmed: `✏️s/🔌️plugins/🗄️stdio/🎛️apps/` holds only an
orphaned stub `🦀️component.rs`, no per-kind subdirectory) — every surface here is authored fresh from
the artifact's own schema/io, following `📓️w2-cad-report.md`'s recipe adapted for "author, don't
migrate."

## What landed

Every one of the 17 subsets now has a real, thin `✏️editor` and `👁️viewer` pair: root
`🦀️component.rs`/`🟦️component.ts`, one mode (`✏️edit` / `👁️view`), one window (`🪟️main`) with both
`🦀️component.rs` and `🟦️component.ts` — 10 files × 17 = **170 files**, all real content, zero SCAFFOLD
markers remaining anywhere in these trees (verified by grep, see Verification).

### Window-kit assignment (contract §2.6)

| kit | kinds | rationale |
|---|---|---|
| `ImageWindowKit` | png, jpg×2, bmp, tiff×2, gif×2, svg×3 (11 subsets) | all render as a 2D image |
| `MediaWindowKit` | mp4, mp3, wav, avi (4 subsets) | audio/video transport |
| `TextWindowKit` | html, md (2 subsets) | text-shaped documents |

Every window's `definition()` returns the kit's own `window_kind()` (viewer) / `editable_window_kind()`
(editor) **verbatim** — never a bespoke `WindowKindDefinition` — and every window id/body-key is the
kit's own frozen `KIND_ID` (`"framework.window.image"` / `"framework.window.media"` /
`"framework.window.text"`), matching the precedent already set by the concurrently-landing
`🖨️raster`/`✒️writer` packets (`ImageWindowKit::window_kind()` reused as-is, real pixels through
`ImageWindowKit::render(&ImageView{..})`, no per-app id namespacing needed since each surface is its
own `AppDefinition`).

### Editor command → real mutation mapping (no invented mutations)

Every editor's typed `Command` enum has exactly **one** variant, driving the kit's one frozen editable
action onto the artifact's own most-fitting existing mutation — never a mutation the schema doesn't
declare:

| kind(s) | action id (from kit) | Command variant | dispatched mutation |
|---|---|---|---|
| png | `set-pixel-region` | `SetPixelRegion{pixels}` | `PngMutation::SetPixels{pixels}` |
| jpg (any+baseline) | `set-pixel-region` | `SetPixelRegion{pixels}` | `JpgMutation::SetPixels{pixels}` |
| bmp | `set-pixel-region` | `SetPixelRegion{pixels}` | `BmpMutation::SetPixelData{pixels}` |
| tiff (any+baseline) | `set-pixel-region` | `SetPixelRegion{pixels}` | `TiffMutation::SetPixels{pixels}` |
| gif 87a | `set-pixel-region` | `SetPixelRegion{indices}` | `GifMutation::SetImagePixels{index:0,indices}` |
| gif 89a | `set-pixel-region` | `SetPixelRegion{indices}` | `GifMutation::SetFramePixels{index:0,indices}` |
| svg (any+basic+tiny) | `set-pixel-region` | `SetPixelRegion{source}` | `SvgMutation::SetSnapshot{snapshot}` via `parse_dsl(source)` |
| html, md | `replace-text` | `ReplaceText{text}` | `<Kind>Mutation::SetSnapshot{snapshot}` via `parse_dsl(text)` |
| mp4, mp3, wav, avi | `seek-media` | `SeekMedia{position_ms}` | **none** — documented no-op, see below |

Notes on the two non-literal mappings, called out per the brief's instruction to say so rather than
invent:
- **gif**: the format's schema has no whole-raster replace, only per-frame/per-image pixel-index
  replace (`SetFramePixels`/`SetImagePixels`, each keyed by `index`). "Region" is simplified to "the
  whole first frame/image" (`index: 0`) — the closest real mutation, documented in the file's own doc
  comment.
- **svg**: SVG has no pixel buffer at all — it is XML. `set-pixel-region` is mapped to the artifact's
  own DSL text round-trip (`<SvgSnapshot as store::ArtifactDsl>::parse_dsl` → `SetSnapshot`), the
  closest real mutation this format declares. The *viewer's* image rendering for svg is likewise not a
  rasterization: the window's `ImageView.base64` is the SVG's own XML source
  (`write_svg_xml(&snapshot.doc)`, from `🧬️schema/📸️snapshot`) wrapped as an `image/svg+xml` data URI
  — `ui_image`/`ImageWindowKit::render` displays it natively, no rasterizer dependency needed.
- **html/md**: both declare a real `SetSnapshot{snapshot}` mutation, so `replace-text` round-trips
  through the artifact's own DSL text envelope (`print_dsl`/`parse_dsl`) — the editable `TextView.text`
  is that DSL text, not literal `<html>`/markdown markup (html/md snapshots are typed DOM/AST trees,
  not flat strings; the DSL text is the closest artifact-native "text" a generic `TextWindowKit`
  editor can offer without a bespoke per-format text serializer). Documented in the window file.
- **mp4/mp3/wav/avi**: `seek-media` is declared (the frozen action from
  `MediaWindowKit::editable_window_kind()`) so the editable surface exists and advertises the action,
  but `handle` intentionally performs no document mutation and no ephemeral-transient mutation either.
  Playback position is host-side transport state, not something any of these four formats' decoded
  snapshots model as persisted content — inventing a `position_ms` field on the document schema (which
  is `🧬️schema/**`, outside this packet's lease and owned by the FULL-STDIO peer ticket) was not an
  option. `MediaView.duration_ms`/`.position_ms` stay at the kit's zero defaults; documented as a v1
  thin simplification, not silently wrong.
- Parse failures on svg/html/md's `parse_dsl` fail soft to `Emit::default()` (no-op), mirroring the
  fail-soft precedent `🖨️raster`'s own viewer window sets for a failed composite.

Every failure path returns a real, structurally-typed empty `Emit`/`ViewEmit` — never a panic, never an
`unwrap()` on user-controlled input.

### `<KIND>_DIALECT` consts (artifact root, `artifact_kind` verified against the schema descriptor)

Added to each of the 12 artifact-root `🦀️component.rs` files (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<kind>/🦀️component.rs`
— NOT under `🧬️schema/**`/`🚪️io/**`, so this is additive surface-support code, not a schema edit):

- `PNG_DIALECT`, `BMP_DIALECT`, `MP4_DIALECT`, `MP3_DIALECT`, `WAV_DIALECT`, `AVI_DIALECT`,
  `HTML_DIALECT`, `MD_DIALECT` — one const each.
- `JPG_ANY_DIALECT`/`JPG_BASELINE_DIALECT`, `TIFF_ANY_DIALECT`/`TIFF_BASELINE_DIALECT`,
  `GIF_87A_DIALECT`/`GIF_89A_DIALECT`, `SVG_ANY_DIALECT`/`SVG_BASIC_DIALECT`/`SVG_TINY_DIALECT` — one
  const per subset (multi-subset kinds).

Every `artifact_kind` string was copied verbatim from that artifact's own pre-existing
`schema::…::png_artifact_schema_descriptor()`/`Dialect{artifact_kind: "s.stdio.<kind>", ...}` row
already used by its `<Kind>AnalyzerAnalysis` (e.g. `"s.stdio.png"`, `"s.stdio.jpg"`, `"s.stdio.gif"` —
**shared** across gif's two standards, matching that artifact's own pre-existing convention of one
document-schema id for both 87a/89a) — never guessed. `standard`/`subset` match each file's own
`🏅️standards/🔖️.../🪆️subsets/✳️...` location.

### Baseline/basic/tiny subsets (jpg, tiff, svg)

Confirmed via each subset's own `🧬️schema/🦀️component.rs` doc comment ("D4 Tier-1: same snapshot type,
subset moves") that `baseline`/`basic`/`tiny` reuse their `any` sibling's `Snapshot`/`Mutation` Rust
type **verbatim** (`pub use …any::schema::*;`) — only the `Dialect.subset` differs. Their editor/viewer
surfaces are therefore genuinely independent `AppDefinition`s (own `DIALECT`, own window/mode ids) but
share the identical render/mutation-mapping code shape as `any`. One exception: `jpg`/`tiff`'s
`baseline` subset's own `🚪️io` module carries only its `JpgBaselineValidator`/`TiffBaselineValidator`,
not the `encode_jpg`/`encode_tiff` codec functions — those two subsets' editor/viewer windows import
the encoder from the sibling `✳️any` subset's `🚪️io` instead (documented in the generator, not a
purity violation: `🚪️io` is read-only-imported, never edited).

## Method

Given 17 near-identical thin surfaces, I built a one-off Python generator
(`🐍️w2-stdio-media-gen-surfaces.py`, this ticket folder — not a permanent script, not committed as
repo tooling; every subset's real render/command-mapping decision was hand-designed first — see the
mapping table above — then mechanically applied, matching this exact packet's own warning that
hand-typing ~200 emoji paths guarantees the ENGINELESS-ticket defect class). Every `#[path]` target and
Cargo-relative path the generator emitted was verified against the real filesystem programmatically
(`os.path.isdir`/`os.path.isfile`) before being written — no path segment was hand-retyped from memory.
`🐍️w2-stdio-media-add-dialects.py` patched the 12 artifact-root files; `🐍️w2-stdio-media-gen-glue-blocks.py`
built the `📦️glue.rs` mount text; `🐍️w2-stdio-media-categorize-errors.py` split cargo output into
mine-vs-foreign for verification. All four scripts are in this ticket folder per this ticket's "no
migration scripts, but permanent-repo-tooling and one-off authoring aids are different things" framing
(§7.8 of the contract; the mechanical, per-subset-identical shape here is exactly the scaffolder's own
justification for existing).

## `📦️glue.rs` (shared with two concurrently-running sibling packets)

Re-read the file's exact tail immediately before every edit; appended (never rewrote) two new
top-level regions after the pre-existing `//#endregion Artifacts`:

- `//#region ✏️Editor` / `pub mod editor { pub mod png { ... } pub mod jpg_any { ... } ... }` — 17 flat
  submodules (one per subset, e.g. `jpg_any`/`jpg_baseline` distinct siblings, not nested under a
  shared `jpg`), each `#[path]`-mounting its own `✏️editor/{🦀️component.rs, 🎭️modes/✏️edit/{🦀️component.rs,
  🪟️windows/🪟️main/🦀️component.rs}}`.
- `//#region 👁️Viewer` — identical shape under `👁️viewer/`.

**Real trap hit and fixed**: my first append used `pub mod editor { pub mod png { #[path="."] ... } }`
— i.e. `#[path="."]` on the inner subset module but **not** on the outer `editor`/`viewer` module
itself. This compiles (no path-resolution error at the attribute level) but resolves every nested
`#[path="../../🗿️artifacts/..."]` one directory too shallow (`editor/` becomes a phantom directory
level baked into the base, so two `..`s cancel `editor/` plus one real level instead of two), producing
`error: couldn't read ".../editor/./../../🗿️artifacts/..."`. Root-caused by comparing against
`📐️cad`'s own working glue.rs (`#[path="."]` on **both** `pub mod editor {` *and* its nested `pub mod
cad {`, confirmed at `📐️cad/📦️packages/🦀️rust/📦️glue.rs:552-555`) — added the missing `#[path="."]`
to my own `pub mod editor {`/`pub mod viewer {` lines, re-ran the disk-resolution check (0 missing,
3571 total `#[path]` attrs across the whole file), and the path errors disappeared. Documented here
because it's exactly the class of bug `📓️w2-cad-report.md` item 10's "verify with a script" warns
about — the script had verified my paths as *syntactically present on disk*, which is necessary but
not sufficient; the actual rustc module-resolution semantics of `#[path]` on an *inline* (body-given,
no-file) module needed hand-tracing against a known-working precedent.

A sibling packet (`P3-stdio-geometry`) landed its own `//#region` block in the same file mid-session,
appended cleanly after mine with no overlap — confirmed via `git status`/diff review, not reverted or
touched.

## Plugin builder (`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`)

stdio is a **library** plugin (`Plugin::builder("stdio")...try_library()`, not a document-app plugin)
— its `plugin()` function had no `.editor()`/`.viewer()` calls of any kind before this packet (0 prior
surfaces registered anywhere in stdio). Added a `//#region 👁️✏️SurfacesP1StdioMedia` block, 34 calls
(`.editor::<X>(create_x_editor())` / `.viewer::<X>(create_x_viewer())`, one pair per subset), inserted
before `builder.try_library()`. Re-read the file fresh immediately before editing (confirmed unchanged,
15 lines, matching what W0-F's report described as still-untouched). A sibling packet
(`P3-stdio-geometry`) appended its own 40-subset region immediately after mine, cleanly, no conflict.

## `Cargo.toml` (shared)

Two dependencies were missing and needed by every raster/svg window (`base64::engine::general_purpose`)
and by the tests region (`semio_framework::AppRole`) — neither had ever been added to stdio's
`Cargo.toml` before (stdio had `semio-framework-plugin` and four `semio-framework-*` module crates, but
never bare `semio-framework` or `base64`). Added, matching `📐️cad`/`🖨️raster`'s own exact dependency
lines:
```toml
base64 = "0.22"
semio-framework = { path = "../../../../../🧰️framework/📦️packages/🦀️rust", package = "semio-framework" }
```
Also needed (and initially missed) `use base64::Engine as _;` in every image-kit window file — base64
0.22's `GeneralPurpose::encode` is a trait method (`base64::Engine`), not an inherent method; matches
`🖨️raster`'s own `use base64::Engine as _;` precedent.

## Verification

### `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --all-targets --keep-going`

Five full runs while iterating (137→22→0 errors in my own files); full output of the **last** run in
`🧪️w2-stdio-media-cargo.txt` (34763 lines). Final run: **0 errors, 0 warnings anchored in any of my 34
surface trees** (`grep`-verified: every error/warning block's own `--> file:line` path was checked
against `✏️editor`/`👁️viewer` + one of my 12 kind directory names — the same categorization script is
in the ticket folder as `categorize_errors.py`).

- 1351 errors remain, **all** in files this packet never touched: other artifacts' `🧬️schema`/`🚪️io`
  (owned by the live peer ticket `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`)
  and other W2 packets' own editor/viewer trees (`🧊️gltf`, `🧿️semio`, etc. — confirmed by kind-name,
  not just directory-role). Spot-checked two representative foreign files with `git status
  --porcelain`/`git log --date=iso`: both `📝️md/…/🧬️schema/🦀️component.rs` and
  `🎨️svg/…/🧬️schema/🧬️mutations/🦀️component.rs` show `M` (modified, uncommitted) right now, last real
  commit `2026-08-15 15:21:02` — actively mid-edit by the peer schema ticket, not by me.
  `git status --porcelain` over the whole `🗄️stdio/🗿️artifacts/` tree (excluding my own
  `✏️editor`/`👁️viewer` paths) shows **530** modified files at time of writing — the crate genuinely
  cannot finish a clean `cargo check` right now for reasons entirely outside this packet's lease.
- Representative foreign error classes seen across the three fix iterations: `MutationDiff::apply`
  signature now returning `Result` (a third peer ticket,
  `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`, mid-sweep across the whole
  crate) and assorted `E0432`/`E0308` inside other kinds' own `🧬️schema/🧬️mutations` — none inside a
  `✏️editor`/`👁️viewer` path belonging to any of my 12 kinds.

### Live-filesystem policy checks (my 17 subsets only — `bun ./📜️script.ts policy`'s own cache is stale
per this packet's brief, so these were run by direct filesystem grep, not the cached command)

- **Scaffold residue**: `grep -rl SCAFFOLD` across all 34 surface trees (17 × {editor, viewer}) →
  **0 matches**.
- **Viewer purity**: `grep -rn "::editor::"` and `grep -rln '\.mutation(\|Emit::mutations\|artifact_mutations'`
  across all 17 `👁️viewer` trees → **0 matches** both.
- **Surface completeness**: scripted existence check of all 170 expected files (root `.rs`+`.ts`, mode
  `.rs`, window `.rs`+`.ts`, × 2 roles × 17 subsets) → **all 170 present**, none scaffold-shaped
  (content verified non-trivial by the SCAFFOLD grep above).

### Not run / not applicable

- `cargo test -p semio-s-plugin-stdio` — blocked by the same 530-file peer-ticket churn (crate doesn't
  finish `--all-targets` cleanly yet); the two `#[cfg(test)]` assertions per surface (68 tests total:
  role/dialect-match pairs, `WindowKindDefinition` id checks, a default-document render smoke test)
  compile cleanly (0 errors in `lib test` target anchored in my files) but were not executed to a
  pass/fail result for the same reason W0-F's own cad tests couldn't be.
- WASM/`tsc`/vitest — not named in this packet's required verification list; TS twin files were
  hand-reviewed (simple `as const` literal exports, no runtime logic) rather than compiled.
- `assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect` (contract §2.5) — real
  versions now exist in `semio_framework_plugin::testkit` per `📓️w0-f-report.md`/`📓️w2-fix-report.md`;
  not wired into this packet's own test modules (kept to the simpler local `def.role`/`def.dialect`
  assertions already used) — a thin, cheap follow-up for whoever next touches these 17 files, not left
  broken.

## Files touched

Created (170, 10 per subset × 17): every subset's
`🗿️artifacts/<kind>/🏅️standards/<std>/🪆️subsets/<subset>/{✏️editor,👁️viewer}/{🦀️component.rs,
🟦️component.ts, 🎭️modes/{✏️edit,👁️view}/🦀️component.rs, 🎭️modes/{✏️edit,👁️view}/🪟️windows/🪟️main/{🦀️component.rs,
🟦️component.ts}}` — replacing the scaffolder's SCAFFOLD placeholders at the same paths (no new
directories beyond what the scaffolder already created).

Edited:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📷️png,📷️jpg,🖼️bmp,🖼️tiff,🎞️gif,🎨️svg,🎥️mp4,🎵️mp3,🔊️wav,📼️avi,🌐️html,📝️md}/🦀️component.rs`
  (12 files: `Dialect`/`StandardId`/`SubsetId` added to the existing `use` line, one `//#region 🔖️Dialect`
  block each with 1–3 `<KIND>_DIALECT` consts).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (append-only: two new top-level regions).
- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` (34 `.editor()`/`.viewer()` registration calls, one new region).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml` (`base64`, `semio-framework` dependencies added).

Not touched (outside this packet's lease, confirmed by `git status`): anything under any kind's
`🧬️schema/**` or `🚪️io/**`; every other stdio kind not in this packet's 12; `🎛️apps/` (already empty of
per-kind content).

Scratch (ticket folder): `🧪️w2-stdio-media-cargo.txt` (final cargo output), `🐍️w2-stdio-media-gen-surfaces.py`,
`🐍️w2-stdio-media-add-dialects.py`, `🐍️w2-stdio-media-gen-glue-blocks.py`,
`🐍️w2-stdio-media-categorize-errors.py` (one-off authoring/verification aids, described above).

## Outstanding / follow-ups (not this packet's to fix)

1. `mp4`/`mp3`/`wav`/`avi`'s `seek-media` is a declared no-op (see mapping table) — a real transport
   position would need either a new persisted-config field (this packet's own `🎚️config` facet, which
   I could have added) or an ephemeral-transient one; skipped for v1 thinness, flagged for whoever
   revisits media playback UX.
2. `gif`'s `set-pixel-region` always targets frame/image `index: 0` — a real per-region (or
   per-selected-frame) patch would need either a UI-selected index threaded through the command or a
   `set-pixel-region`-shaped mutation this format's schema doesn't currently declare.
3. Contract §2.5's real `testkit::assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect`
   exist now (landed after this packet started) but aren't wired into these 17 surfaces' own test
   modules — cheap follow-up, not done here to avoid re-touching all 17 files a second time under time
   pressure.
4. `cargo test`/full `cargo check` for `semio-s-plugin-stdio` should be re-run once the FULL-STDIO peer
   ticket's schema/io sweep and the MUTATION-OUTCOMES peer ticket's `MutationDiff` signature change both
   land — expected clean for all 12 of my kinds based on every run this packet saw (zero errors ever
   attributed to a file under my lease).
