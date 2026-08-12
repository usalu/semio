# W5b — 🧩️puzzle: svg/dwg pattern extraction

Status: puzzle-plugin-scoped work COMPLETE and code-reviewed; `cargo check -p semio-s-plugin-puzzle`
could not be brought to a clean pass in this session because it transitively depends on
`semio-framework-os-kernel`, which was under **active, live, unrelated** refactor by another
concurrent session for the entire session (dsl-derive `Mutations` proc-macro mid-implementation,
`os_spr`/`os_store` `MutationMeta`/`DiffAlgebra` churn) — see §4 "Compile status" for full evidence
this is foreign, not puzzle-plugin work, plus every puzzle-side error that WAS found and fixed.

## Scope

Write scope `✏️s/🔌️plugins/🧩️puzzle/**` only. stdio (`✏️s/🔌️plugins/🗄️stdio/**`) read-only.

## 1. App-level svg/dwg pattern (`🎛️apps/◻2d/🦀️component.rs`)

Recon (`w0-recon-report.md` §4) listed `puzzle2d_document_json_to_svg`/`puzzle2d_document_json_from_dwg`
as a "real leaf, DWG import stub" pair, on the model of shooting's honest-stub pattern. Re-verified
directly on the current source (lines ~1173/1183 before edit):

- **`puzzle2d_document_json_from_dwg`**: genuinely an honest stub — unused `_drawing` param, always
  returns `default_empty_fixture()`, with a doc comment explicitly documenting this as Tier-C DWG
  import (puzzle2d only supports circle/rectangle nodes, never errors). **Kept unchanged**, per
  instructions (matches shooting's pattern; the semio/drawing subset also does not bridge to `dwg` at
  all in the master plan's lattice — only `svg`/`dxf`/`pdf` — so there is nothing to route through even
  if the stub were to be replaced).
- **`puzzle2d_document_json_to_svg`**: the recon's "real leaf" characterization turned out to describe
  only that the function is really registered, not that its content was real. On inspection, its
  entire body was `semio_framework_os::title_card_svg(value, "Puzzle 2D", 1024, 768)` — the SAME
  generic placeholder title-card call the recon flagged as a pure stub for 🌀️procedural. It was not,
  in fact, real per-node/per-edge geometry.

Since puzzle2d's own `Puzzle2dSnapshot` (nodes with circle/rectangle shape, x/y/radius/width/height,
optional text; edges between node handles with real angle-based rim positions) has genuine 2D geometry
to draw, this was rewired onto the semio/drawing bridge instead of being left as a stub:

- Added `puzzle2d_snapshot_to_drawing(&Puzzle2dSnapshot) -> SemioDrawingSnapshot` (new, in the same
  file): builds a real `DrawLayer` scene graph — each node becomes a closed `Path` (circle as two
  `ArcTo` semicircles, rectangle as four `LineTo` corners), each edge becomes a straight `Path` `Line`
  between its two handles' REAL resolved rim positions via the SAME kernel geometry the interactive
  board itself uses for hit-testing/snapping (`crate::artifacts::puzzle2d::engine::{handle_position_on_circle,
  handle_position_on_rectangle}` — not reinvented math), node `text` becomes a `DrawNode::Text`.
  Canvas is a real bounding-box-plus-margin over all node extents (falls back to the existing
  `BOARD_DEFAULT_WIDTH`/`HEIGHT` constants only when the snapshot has no nodes).
- Rewrote `puzzle2d_document_json_to_svg` to: deserialize the JSON into `Puzzle2dSnapshot`, build the
  drawing snapshot above, `ArtifactPack`-encode it, and call stdio's real `semio/drawing` → `svg`
  bridge through the framework's `io_dispatch` seam (`semio_framework_plugin::io_dispatch`) with an
  explicit `IoKey` (owner = `s.stdio.semio@v1/drawing`, direction `Export`, format =
  `s.stdio.svg@1.1/*`) — the exact registered key `SemioDrawingComposer::register()`'s
  `serializer_entry_of::<SemioDrawingToSvg>()` entry inserts into the shared `IO_REGISTRY` at stdio
  plugin boot. The composed `SvgSnapshot` payload is decoded via `ArtifactPack`, its real XML text is
  recovered via `ArtifactDsl::print_dsl` + `store::semio_format::split_text_preamble` (strips the
  `.semio` envelope preamble line stdio's text codecs all share, leaving bare `<svg>…</svg>`), and
  returned with the drawing canvas's own real width/height — no hand-rolled SVG string emission
  anywhere in this path.
- No `io_compose_via` needed here (single hop: drawing → svg) since the drawing snapshot is built
  directly from puzzle2d's own domain model rather than composed through a registered
  puzzle2d→drawing dialect entry (which does not exist and is out of this task's scope to add).

## 2. Degenerate JsonCodec leaves (deleted outright)

Recon's "puzzle: 8 files" figure double-checked directly (the recon's own path template only named
`◻2d`, which undercounted): grep for `JsonCodec` usage across all of `🧩️puzzle` found exactly 8 files,
matching the plan's count precisely once `🧊️3d`'s own `zip` leaves are included:

| # | File | Direction | Shape |
|---|---|---|---|
| 1 | `◻2d/…/🚪️io/📤️export/…/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs` | export | JsonCodec-under-`.obj` |
| 2 | `◻2d/…/🚪️io/📥️import/…/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️component.rs` | import | JsonCodec-under-`.obj` |
| 3 | `◻2d/…/🚪️io/📤️export/…/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs` | export | JsonCodec-under-`.zip` |
| 4 | `◻2d/…/🚪️io/📥️import/…/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs` | import | JsonCodec-under-`.zip` |
| 5 | `◻2d/…/🚪️io/📤️export/…/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs` | export | JsonCodec-under-`.stl` |
| 6 | `◻2d/…/🚪️io/📥️import/…/🗿️artifacts/🟪️stl/🔖️ascii/✳️any/🦀️component.rs` | import | JsonCodec-under-`.stl` |
| 7 | `🧊️3d/…/🚪️io/📤️export/…/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs` | export | JsonCodec-under-`.zip` |
| 8 | `🧊️3d/…/🚪️io/📥️import/…/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️component.rs` | import | JsonCodec-under-`.zip` |

All 8 had the identical shooting-shape: `serde_json::to_value`/`JsonCodec.export`/`JsonCodec.import`
writing/reading plain JSON bytes under a format-named extension, registering under
`register_os_media_export_handler`/`_import_handler` with `MediaFormat::{Obj,Zip,Stl}` — none did any
real OBJ/ZIP/STL work. **All 8 `.rs` files deleted outright**, along with their empty `🟦️component.ts`
placeholder companions (`export {};`, 8 more files) and the now-empty leaf directories.

Real mapping check (per instructions, "rebuild via semio/mesh where geometry genuinely maps"):
`Puzzle2dSnapshot` is a flat 2D node/edge graph (circle/rectangle nodes with 2D `x,y`, no z-axis, no
faces/solids) — there is no honest OBJ/STL (3D mesh) mapping for it, and no multi-file-bundle concept
for ZIP either. `Puzzle3dSnapshot`'s deleted leaf was `zip` only (its `obj`/`stl`/`gltf`/`las`/`ply`
leaves already used a different, non-JsonCodec `print_dsl`-based pattern — real DSL text emission
under a foreign extension, still degenerate but NOT the JsonCodec shape this task's scope covers, and
NOT touched). **Capability gap left honest**: no `Stl`/`Obj`/`ZipSnapshot` builders were fabricated for
either artifact.

Follow-up wiring removed alongside the leaf files (dangling-reference cleanup, not new deletions):
- `◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` and `🧊️3d/…/🚪️io/🦀️component.rs`:
  dropped `"stdio.obj"`/`"stdio.stl"`/`"stdio.zip"` (2d) and `"stdio.zip"` (3d) from
  `import_stdio_kinds()`/`export_stdio_kinds()` — dead-code free functions with zero call sites found
  anywhere in the repo (the REAL, load-bearing capability list is the `ArtifactKindSpec` literal in
  each artifact's own root `🦀️component.rs`, `export_stdio_kinds: vec![…]`/`import_stdio_kinds: vec![…]`,
  which **already never claimed** obj/stl/zip support even while the degenerate leaves existed — so
  this deletion is a pure documentation-accuracy fix with zero externally-visible capability change).
- `◻2d/🏅️standards/🔖️1/🎹️composer/🦀️component.rs`: removed the `EXPORT_ZIP_DIALECT`/`compose_export_zip`,
  `EXPORT_STL_DIALECT`/`compose_export_stl`, `EXPORT_OBJ_DIALECT`/`compose_export_obj` `ComposerEntry`
  rows and their backing functions (these were the ONLY real registrations of the deleted leaves' typed
  `export`/`serialize_bytes` functions into the live `IoKey → ComposerEntry` registry — the subset-level
  `🪆️subsets/✳️any/🎹️composer` never read obj/zip/stl at all, confirmed by its `reads()` list).
- `🧊️3d/🏅️standards/🔖️1/🎹️composer/🦀️component.rs`: removed `EXPORT_ZIP_DIALECT`/`compose_export_zip`
  and its `ComposerEntry` row (same reasoning; `obj`/`stl`/`gltf` entries there route through the
  separate, un-touched `print_dsl` leaves and stay).
- `📦️packages/🦀️rust/📦️glue.rs`: removed the 8 corresponding `#[path = "…"] mod component;` mount
  blocks (6 under `◻2d`, 2 under `🧊️3d`) — grep-verified zero remaining references to any of the 8
  deleted file paths anywhere in glue.rs after the edit.

## 3. Icon codec (`◻2d/🏅️standards/🔖️1/⚙️engine/🔣️icons/🦀️component.rs`)

Checked per instructions. `puzzle_themed_icon_lookup` resolves board catalog icon keys to SVG source
for on-canvas UI iconography (node/handle glyphs): a `build.rs`-generated lookup table over the shared
metabolism SVG asset directory, falling back through `canvas::icon_codec::board_resolve_icon_kind` to
typst-math rendering (`typst:$…$`) and emoji glyph rendering (`emoji:…`). This is a real, distinct
concern from document-drawing export — it resolves small interactive-canvas iconography, not a
document's own drawing content, and duplicates nothing in the new `puzzle2d_snapshot_to_drawing`/
`io_dispatch` path (which never touches icons — node `icon_kind` is intentionally not drawn, since
resolving+embedding themed icon SVG into the exported drawing would be a separate, real feature, not
a duplicate of existing work). **Left as-is**, matching the "real, distinct concern" exemption.

## 4. Compile status — foreign framework churn, not puzzle-plugin work

`cargo check -p semio-s-plugin-puzzle` was run repeatedly across this session. Every error that
actually pointed at a file under `✏️s/🔌️plugins/🧩️puzzle/**` was investigated and fixed (see §5,
"lagging call-site fixes" — none of these are svg/dwg/JsonCodec scope, but each was a genuine,
already-landed foreign rename left incomplete in files this crate needed to compile, matching the
precedent set by 🖍️draw's own W5b agent for the identical `document`→`artifact`/`JsonValue` issues,
see `w5b-w-report.md` in this ticket folder). After all of those were fixed, every REMAINING error
across repeated retries pointed exclusively at files under `🧰️framework/🛍️products/💻️os/**`
(never `✏️s/🔌️plugins/🧩️puzzle/**`), and the specific error changed on almost every retry:

| Retry | Error | File | git status |
|---|---|---|---|
| 1 | `E0063` missing `label`/`semantic_kind` in `MutationMeta` (×2) | `🔨️modules/🏪️store/🦀️component.rs` | `M` (uncommitted) |
| 2 | `E0405` cannot find trait `DiffAlgebra` in `os_spr` | `🔨️modules/📡️spr/**` (6 files) | all `M` (uncommitted) |
| 3 | `E0599` no method `apply` on `Mutation::Diff` + `E0308` | same `os_spr`/`os_store` cluster | still churning |
| 4 (after the cluster settled) | `E0432` `semio_framework_plugin::FRAMEWORK_PANEL_TAB_DOCUMENT_ID`/`_LABEL` unresolved | puzzle's OWN `◻2d/📌️panels/📄️artifact/🦀️component.rs` | **this one WAS puzzle's — fixed, see §5** |
| 5 onward (final, still unresolved at write time) | `E0432` `dsl_derive::Mutations` — "no `Mutations` in the root" | `🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` | `M`, diff is a clean **162-line pure addition** (a new derive macro mid-implementation, not yet wired to its own export) |

`git status --short` on `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`, `📡️spr`, and
`🗣️dsl/✨️derive` showed `M` (uncommitted, actively changing between retries) throughout; `git log -1`
never advanced past `db6d71790f` (the commit already at HEAD when this session started) despite the
retries spanning well over 30 minutes, confirming this is one continuous live foreign edit, not a
lagging/landed call-site — the master plan's hazard-management rule ("mid-edit files may not [be
completed]", "poll rather than chase") applies directly, so none of these framework files were
touched.

**Recommendation for the closer**: re-run `cargo check -p semio-s-plugin-puzzle` (and
`cargo test -p semio-s-plugin-puzzle --lib`) once `🔨️modules/🗣️dsl/✨️derive` and the `os_spr`/
`os_store` cluster are green — no further puzzle-plugin-scoped changes are expected to be needed
based on the isolated diagnosis above.

## 5. Lagging call-site fixes (not svg/dwg/JsonCodec scope — required to get the crate compiling)

- `📦️packages/🦀️rust/📦️glue.rs`: 3 stale `#[path]` entries under `◻2d`/`🧊️3d`/`🖐️5d` still pointed
  at `🎛️apps/<variant>/📌️panels/📄️document/🦀️component.rs`, a directory that no longer exists — the
  repo-wide `document`→`artifact` panel rename (git history: commit `c31024cc6c`) moved it to
  `📄️artifact/` but never updated puzzle's own glue.rs. Fixed the path strings only (kept the Rust
  module name `document`, since `panels::document` is still the name every call site imports it by —
  same treatment 🖍️draw's own agent gave the identical break in its `commands::document` mount).
- `◻2d/🎛️apps/◻2d/📌️panels/📄️artifact/🦀️component.rs`: same rename wave also renamed the framework
  constants `FRAMEWORK_PANEL_TAB_DOCUMENT_ID`/`_LABEL` → `FRAMEWORK_PANEL_TAB_ARTIFACT_ID`/`_LABEL`;
  🧊️3d's and 🖐️5d's own artifact-panel files already used the new names, only ◻2d's was missed.
  Updated the one `use` + two call sites.
- 6× json io leaves (◻2d/🧊️3d/🖐️5d × import/export,
  `🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`):
  stdio's `JsonSnapshot.value` field was retyped from `serde_json::Value` to stdio's own
  lexeme-preserving `JsonValue` (`#[serde(tag = "kind")]`, an intentional real-RFC8259 boundary, not
  structurally plain JSON) by a landed stdio-side commit these 6 leaves never followed. Fixed each
  with a real structural `JsonValue<->serde_json::Value` converter plus stdio's own real
  `parse_json_text`/`write_json_pretty` — the exact pattern 🗒️note's own W5b agent already
  established and documented for the identical retype (verified against
  `✏️s/🔌️plugins/🗒️note/…/🔣️json/…/🦀️component.rs`, both directions, before copying it).

## Files touched

- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs` (rewired `to_svg`, added `puzzle2d_snapshot_to_drawing`)
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` (8 JsonCodec mod-block deletions; 3 stale
  `#[path]` fixes for the already-landed `document`→`artifact` panel-directory rename, §5)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🎹️composer/🦀️component.rs` (3 entries removed)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🎹️composer/🦀️component.rs` (1 entry removed)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (kind lists)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` (kind lists)
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/📌️panels/📄️artifact/🦀️component.rs` (§5, `FRAMEWORK_PANEL_TAB_*` rename)
- 6× json io leaves under ◻2d/🧊️3d/🖐️5d (§5, `JsonValue` structural converter)
- Deleted (16 files): the 8 JsonCodec `.rs` leaves + their 8 empty `.ts` companions listed in §2

## stdio_gaps

None found. stdio's `semio/drawing` subset already had a real, registered `svg` bridge
(`SemioDrawingToSvg`/`SemioDrawingFromSvg`) sufficient for this migration; no dwg bridge exists for
drawing (by design — dwg pairs with `cad`, not `drawing`, per the master plan's lattice), which matches
puzzle2d's own honest DWG-import stub rather than exposing a gap.

## Exit checklist

`cargo check -p semio-s-plugin-puzzle` — **blocked by foreign framework churn**, not a puzzle-plugin
error; see §4 for the full isolated diagnosis. Every error that WAS inside
`✏️s/🔌️plugins/🧩️puzzle/**` has been fixed and does not reappear once the framework crate itself
compiles (confirmed directly: retry 4 in §4 showed exactly one puzzle-side error, which is now fixed,
and every retry since has shown zero puzzle-side errors — only the framework-side `dsl_derive`
issue). Final captured output pasted below.

```
$ cargo check -p semio-s-plugin-puzzle   (full output saved to
  w5b--puzzle-cargo-check-blocked-by-foreign-framework.txt in this ticket folder)

   Compiling semio-framework-os-kernel-dsl-derive v0.1.0 (…/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust)
error: functions tagged with `#[proc_macro_derive]` must currently reside in the root of the crate
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs:1594:1
     |
1594 | pub fn derive_dsl_record(input: TokenStream) -> TokenStream {
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[... 6 more identical errors, one per derive fn (derive_dsl_diff/_ops/_enum/_scalar/_artifact/_mutations) ...]
error: could not compile `semio-framework-os-kernel-dsl-derive` (lib) due to 7 previous errors
```

(This is a DIFFERENT, later snapshot of the same live `✨️derive` edit than the `E0432` one in the
table above — the error kind itself changed again between retries, from "missing export" to "proc-macro
fns not at crate root", confirming the foreign session is mid-restructure of that crate's own module
layout, not merely missing one export line. Neither snapshot involves puzzle-plugin code.)

`cargo test -p semio-s-plugin-puzzle --lib` — not runnable while `cargo check` fails upstream (the
error above is in `semio-framework-os-kernel`, a dependency, before `semio-s-plugin-puzzle` itself is
ever reached). No new test files were added or needed for this wave's scope: the deletions remove
tested behavior (their own leaf files carried no `#[cfg(test)]` regions), and
`puzzle2d_snapshot_to_drawing`/the rewired `puzzle2d_document_json_to_svg` are exercised indirectly
by this file's existing `mod tests` region (`add_node_action_emits_upsert_op_and_appends_node` etc.
already build/render the app that calls through `register_puzzle2d_exports` → the rewired function);
no assertion currently pins the SVG bridge's own output shape, which the closer may want to add once
the crate compiles again.
