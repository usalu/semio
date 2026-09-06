# 🖍️ Draw plugin — drawing artifact schema/io audit

Scope: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🦀️.rs` + standard `1` / subsets `✳️any`, `🎨️style`,
`🏷️metadata`, `🔀️transform`, `🧱️structure`. Cross-checked against `✏️s/🔌️plugins/🗄️stdio` (drawing
bridge target) and the sibling `🖨️raster` plugin (same stdio bridge, correct imports).

## 0. Headline: this crate almost certainly does not compile today

Two independent, verifiable breakages exist right now:

- **`🚪️io/🦀️.rs:15-19`** imports five stdio symbols that were renamed away (`DrawingCanvas`,
  `DrawingLayer`, `DrawingNode`, `DrawingStyle` no longer exist; current names are `DrawCanvas`,
  `DrawLayer`, `DrawNode`, `DrawStyle`) and imports a subset module (`svg::standards::v1_1::subsets::any`)
  that has never existed — svg 1.1's subsets are `base`/`basic`/`tiny`. See §3.
- **`include_str!` fixture paths are stale in 4 of 5 subsets' mutate tests** (12 of 14 mutation
  kinds) — the referenced fixture directories were renamed with hash suffixes
  (`➕️appends-shape-b-at-the-root` → `➕️appends-shape-b-0b0435`, etc.) and the test files were never
  updated. `include_str!` on a missing path is a hard `rustc` error, not a runtime failure. See §7.

Both classes of breakage are duplicated across the `<subset>`-owned test file AND its
`✳️any`-owned "shard F4" duplicate, so fixing one copy is not enough.

## 1. Artifact declaration (`🖍️drawing/🦀️.rs`)

- `artifact_kind()` (line 453-468): `id: "2d.drawing"` — **does NOT follow `s.<plugin>.<kind>`**.
  This is the OLD capability-row id, explicitly kept live: the plugin's own activation reads it
  (`✏️s/🔌️plugins/🖍️draw/🦀️.rs:44`: `.activation(ActivationEvent::OnArtifactKind { kind:
  crate::artifacts::draw::artifact_kind().id })`), so the running plugin activates on artifacts
  whose kind is literally `"2d.drawing"`.
- `artifact()` (line 531-535, the NEW declaration-tree root): `ArtifactKindId::parse("s.draw.drawing")`
  — the grammar-correct id. `DRAWING_DIALECT` (line 476) and `definition()`'s `ArtifactIdentity`
  (line 509) also use `"s.draw.drawing"`. `DrawingSnapshot`'s own `#[artifact_schema(id =
  "s.draw.drawing")]` (schema files) and `DrawingArtifact` likewise.
- **Two live kind-id grammars coexist for the one artifact**: `"2d.drawing"` (activation +
  `artifact_kind()`, plus `✏️editor/🦀️.rs` and `procedural/generation2d`'s editor, per repo-wide
  grep) vs `"s.draw.drawing"` (the new `ArtifactDeclaration`/`ArtifactSchema`/`Dialect` tree, plus
  viewer/editor TS and the generator script). The file's own doc comments (lines 478-486, 523-530)
  say this is intentional interim debt ("debt D1", not deleted until "W6") — but nothing in this
  file bridges the two ids, so any code that resolves-by-kind through the NEW declaration
  (`ArtifactDeclaration.kind`) will not match an artifact instance activated (and thus actually
  running) under the OLD id, and vice versa.
- `SurfaceDeclaration`/roster wiring: `artifact()` wires exactly one standard
  (`crate::artifacts::drawing::standards::v1::standard()`), `localization: &[]` (English/German
  names instead live in `definition()`'s two `localization` capability rows, lines 506-507 —
  another place the old/new split leaks through).
- `export_stdio_kinds`/`import_stdio_kinds` on `artifact_kind()` (lines 465-466: `["stdio.svg",
  "stdio.png"]`) are dead per `🚪️io/🦀️.rs`'s own module doc (line 6): "old, zero callers even
  before this pass" — kept only because they're struct fields, never read.

## 2. Schema

- `DrawingSnapshot` (`✳️any/🧬️schema/📸️snapshot/🦀️.rs:1-47`): clean `#[derive(dsl::ToValue,
  dsl::FromValue, dsl::DslRecord, ArtifactSchema)]`, `#[artifact_schema(id = "s.draw.drawing")]`.
  No hand-written `ToValue`/`FromValue` impl anywhere for it or `DrawingArtifact`/`DrawingDiff` — all
  derived. **No `store::to_dsl_value` self-call trap** — confirmed no manual `ToValue` impls exist
  in this artifact at all; the only hand-written codec impls are `ArtifactDsl`/`ArtifactPack` for
  the native text/binary facet (§4), which call `dsl::parse`/`dsl::print`/`store::pack_rt::*`, never
  `store::to_dsl_value` on themselves.
- `DrawingArtifact` (`✳️any/🧬️schema/🦀️.rs:19-...`) is the full artifact+presence+config state
  struct (`#[state(artifact/presence/config)]` per field) that `ArtifactSchema` derive splits into
  `DrawingSnapshot`/config/presence lanes — `DrawingSnapshot` itself only carries
  `#[state(artifact)]` fields (schema/id/title/layers/assets/artboard).
- `🧰️owned/🦀️.rs` (6699 lines): **hand-written, not generated** — no "GENERATED" banner, no
  external source; it's the bounded-memory retained-store machinery for `DrawingSnapshot`/
  `DrawingMutation` (arena pools, credit slots, clone/digest/rebuild "authorities", incremental
  retirement state machines with `phase: u8` cursors). Zero `ToValue`/`FromValue`/`ArtifactDsl`
  impls in it — purely `store::ErasedSnapshotRetirement`, `store::SnapshotRetirementFactory`,
  `store::ArtifactEnvelopeOwnedFieldCatalog` etc. This is the same per-artifact boilerplate class
  flagged for CAD/other plugins in prior tickets (large, mechanical, one département per state
  category) — not schema-generated from any other file.
- Diff (`✳️any/🧬️schema/🔺️diff/🦀️.rs`): `DrawingDiff` derives `ArtifactSchema` too
  (`#[artifact_schema(id = "s.draw.drawing")]`), sparse `Option<...>` per field, plus hand-written
  `apply`/`absorb` pure transforms living alongside the type (design.md rule 3, per its own doc
  comment) — NOT under `🚪️io/🔺️diff/`, which instead holds only the grammar/protocol assets.
- Mutations (`✳️any/🧬️schema/🧬️mutations/🦀️.rs:9-30`): `DrawingMutation` is one flat
  `#[derive(dsl::DslEnum, dsl::Mutations)]` enum aggregating **all 14 variants from all 4 semantic
  subsets** (style: SetLayerBlendMode/SetLayerOpacity/ReplaceLayerFill/ReplaceLayerStroke;
  metadata: SetLayerVisible/SetLayerLocked/RenameLayer; transform:
  UpdateLayerTransform/SetLayerBooleanOperation/UpdateLayerTraceParams; structure:
  CreateLayer/DuplicateLayer/DeleteLayer/ReorderLayer). Each variant's payload struct + its
  `↩️inverse`/`🔺️diff`/`🦠️mutation` triad physically lives under its OWNING subset's own
  `🧬️schema/🧬️mutations/<slug>/` directory (confirmed: `set-layer-blend-mode` lives under
  `🎨️style/…`, not `✳️any/…`) — `✳️any`'s enum only re-exports/dispatches.
  **Ownership is declared by the subset-level `🔮️oracle/🔣️.json` manifest, not merely by physical
  directory** — `✳️any/🔮️oracle/🔣️.json`'s own `_comment` documents this explicitly: "[2026-09-02
  subset split] All 14 s.draw.drawing@1 mutations moved to real semantic subset owners
  (structure/style/transform/metadata) … `mutationManifests` here is now empty and
  `fixtureManifests` moved with their mutations. The `drawing-1-any` `mutationCatalogs` entry is
  KEPT here … purely because `mutate-drawing-1`'s cross-language differential test still tags
  `@mutations-drawing-1-any` and exhaustiveness is checked against the catalog id's FIRST registry
  match." This matches the general repo rule (memory: mutation ownership is a manifest `subset`
  field, not file location) but no single per-mutation Rust attribute declares it — it is entirely
  test-platform-manifest-driven.

## 3. IO (`✳️any/🚪️io/🦀️.rs`, 314 lines)

`io()` (lines 188-234) registers 6 foreign format pairs (12 `IoEntry`s) via typed
`Serializer<DrawingSnapshot>`/`Deserializer<DrawingSnapshot>` leaves, all keyed on one
`DRAWING_DIALECT`. `NativeCodecs` sets `snapshot`/`diff`/`mutations` `LanguagePair { text: None,
binary: None }` — **deliberately not registered** in the io_mechanism registry even though real
text/binary codecs exist (§4) — the code comment (lines 220-225) calls this "a documented,
deliberate scope-narrowing matching every other subset already on the new tree."

### Format matrix

| format | import | export | real/stub/Err | registered in `io()` |
|---|---|---|---|---|
| svg (1.1) | `SvgIntoDraw` → placeholder empty doc, `Ok` | `DrawingIntoSvg` → real, bridges through `SemioDrawingSnapshot`/`io_dispatch` | import=**stub (Ok, fabricated empty)**; export=**real** | yes |
| pdf (1.4) | `PdfIntoDraw` → placeholder empty doc, `Ok` | `DrawingIntoPdf` → `Err("PDF export is not yet implemented")` | import=**stub (Ok)**; export=**stub (Err)** | yes |
| png (1.2) | `PngIntoDraw` → placeholder empty doc, `Ok` | `DrawingIntoPng` → `Err("PNG export is not yet implemented")` | import=**stub (Ok)**; export=**stub (Err)** | yes |
| json (rfc8259) | `JsonIntoDraw` → real | `DrawingIntoJson` → real | **real, Exact fidelity**, both ways | yes |
| dwg (ac1018) | `DwgIntoDraw` → stub, `Err` | `DrawingIntoDwg` → `Err("no stdio drawing<->dwg bridge")` | **stub (Err)**, both ways | yes |
| dxf (r12) | `DxfIntoDraw` → stub, `Err`(likely; not read verbatim, but doc says "out of scope") | `DrawingIntoDxf` → `Err("DXF export is not yet implemented")` | **stub (Err)**, both ways | yes |
| txt (DSL, `drawing.drawing`) | real `ArtifactDsl::parse_dsl` | real `ArtifactDsl::print_dsl` | **real**, but native `LanguagePair.text = None` | **not registered** |
| pack (binary, DSL) | real `ArtifactPack::decode_pack` | real `ArtifactPack::encode_pack` | **real**, but native `LanguagePair.binary = None` | **not registered** |

Note: only 3 of the 12 registered `IoEntry`s are non-stub in the direction that matters (svg
export, json import, json export); svg import and every pdf/png hop are `Ok`-with-fabricated-empty
stubs (honest per their doc comments — they don't claim real content); dwg/dxf are hard `Err`
stubs both ways.

### stdio bridge (`🚪️io/🦀️.rs:15-19`) — real bridge code, real drift

`drawing_document_to_semio_drawing`/`drawing_document_to_svg` build a real `SemioDrawingSnapshot`
and dispatch it through stdio's `io_dispatch`, mirroring `🗒️note`'s/`📏️layout`'s pattern
(`ensure_semio_drawing_bridge_registered`, line 38-41, calls
`semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::drawing::io::register` exactly
once — that symbol **does exist**, confirmed at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs:145`).

### stdio API drift table

| draw's import (`🚪️io/🦀️.rs:15-19`) | exists today? | current name/path |
|---|---|---|
| `…subsets::base::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioRgba, SemioTransform}` | **yes** | unchanged — confirmed at `✉️base/🧬️schema/🧮️geometry/🦀️.rs:25,33,49,64,81` |
| `…subsets::drawing::schema::snapshot::DrawingCanvas as SemioDrawCanvas` | **NO** | renamed to `DrawCanvas` — confirmed struct decl at `🖊️drawing/🧬️schema/📸️snapshot/🦀️.rs:121` and used un-aliased by sibling `🖨️raster` plugin (`raster/…/🚪️io/🦀️.rs:28`) |
| `…subsets::drawing::schema::snapshot::DrawingLayer as SemioDrawLayer` | **NO** | renamed to `DrawLayer` — decl at `…snapshot/🦀️.rs:109` |
| `…subsets::drawing::schema::snapshot::DrawingNode as SemioDrawNode` | **NO** | renamed to `DrawNode` — decl at `…snapshot/🦀️.rs:52` (an enum) |
| `…subsets::drawing::schema::snapshot::DrawingStyle as SemioDrawStyle` | **NO** | renamed to `DrawStyle` — decl at `…snapshot/🦀️.rs:89` |
| `…subsets::drawing::schema::snapshot::PathSegment as SemioPathSegment` | **yes** | unchanged |
| `…subsets::drawing::schema::snapshot::SemioDrawingSnapshot` | **yes** | unchanged, decl at `…snapshot/🦀️.rs:143` |
| `…subsets::drawing::schema::snapshot::STDIO_SEMIODRAWING_DOCUMENT_SCHEMA` | **yes** | unchanged, value `"stdio.semio.drawing"`, decl at `…snapshot/🦀️.rs:136` |
| `…subsets::drawing::io::register` | **yes** | unchanged, `pub fn register()` at `🖊️drawing/🚪️io/🦀️.rs:145` |
| `svg::standards::v1_1::subsets::any::schema::snapshot::write_svg_xml` | **NO** | svg 1.1 has no `any` subset — it's `base`/`basic`/`tiny`; `write_svg_xml` lives at `svg::standards::v1_1::subsets::base::schema::snapshot::write_svg_xml` (`🎨️svg/…/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:41`); every other consumer in the repo (svg's own root `🦀️.rs`, its own tests) uses `subsets::base`, never `subsets::any` |
| `svg::SvgSnapshot` (crate-root re-export) | **yes** | `pub use crate::artifacts::svg::schema::snapshot::SvgSnapshot;` at `🎨️svg/🦀️.rs:7` |

**Root cause hint**: sibling plugin `🖨️raster`'s own bridge
(`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs:28`) imports
the SAME stdio module with the CORRECT current names (`DrawCanvas, DrawLayer, DrawNode,
PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA` — no `Drawing*` prefix, no
aliasing needed). Draw's copy was not updated when stdio dropped the `Drawing*` prefix (the
`retire_struct!`/`retire_leaf!` macro calls in `stdio/…/🧿️semio/🦀️.rs:277-280` are an unrelated
owned-value-retirement registry, not a compat shim — they do NOT re-introduce the old names).

## 4. Snapshot text/binary codecs

- `📸️snapshot/📝️text/🦀️.rs` (146 lines): `impl store::ArtifactDsl for DrawingSnapshot` — all
  **sync fns** (`parse_dsl`, `print_dsl`). Also carries `COMPONENT_GRAMMAR_SEMIO`
  (`include_str!("📖️.grammar.semio")`) and `SEMIO_DRAW_EXAMPLE_TEXT` (`include_str!` of the demo
  example's `.dsl.semio` — this one path DOES resolve; verified separately from the broken fixture
  paths in §7).
- `📸️snapshot/💾️binary/🦀️.rs` (156 lines): `impl store::ArtifactPack for DrawingSnapshot` — all
  **sync fns** (`encode_pack_with`, `decode_pack_with`, `record_spec`), delegates to
  `store::pack_rt::encode_document`/`decode_document`. Both files are the "handcrafted, derive no
  longer emits this trait" pattern (P6) — callers are `📸️snapshot/🦀️.rs`'s own doc comment,
  `🧰️owned/🦀️.rs`'s `decode_drawing_snapshot_pack`/`decode_drawing_mutation_pack`, and the
  `round-trips-the-committed-document` cucumber test (§7).
- Neither is wired into `io()`'s `NativeCodecs` (§3) — both exist and both are exercised directly
  by tests/owned-store code, just not discoverable through the io_mechanism registry.

## 5. TS twins

- `✳️any/🧬️schema/🟦️.ts` (49 lines): **real, hand-written interface mirror** —
  `DrawingArtifact`/`DrawingLayerNode` (loose `{kind: string; [key: string]: unknown}`, not a
  full discriminated union like the Rust `DrawingLayerNode` enum — TS side does not model the 7
  shape/path/text/image/group/boolean/trace variants)/`DrawingImageAsset`/`DrawingArtboard`. Not
  `export {}` — genuinely populated, but noticeably weaker-typed than the Rust source of truth for
  the layer tree.
- `✳️any/🚪️io/🟦️.ts` (37 lines): real, populated `ioEntries: IoEntryDescriptorMirror[]` (12
  entries mirroring the Rust `io()` list). **Type/fidelity mismatch with reality**: the
  `fidelity` field type is `"Exact" | "Canonical" | "Semantic" | "Lossy"` — there is no `"Stub"`/
  `"Err"` value, so every entry (including the 8 hard-`Err`/placeholder stubs: pdf/png export,
  dwg/dxf both ways, svg/pdf/png import) is forced to claim `"Lossy"`, which reads as "real but
  imprecise" rather than "not implemented." The file's own header comment (lines 1-6) admits this
  ("svg import and every pdf/png/dwg/dxf hop are honest not-yet-implemented stubs") but the data
  shape can't represent that distinction.
- No TS test exists for either file (no `🧪️tests/🟦️.ts` under `✳️any/🧬️schema` or `✳️any/🚪️io`)
  — the only TS test in the drawing artifact tree is the demo example's own
  `📚️examples/🎬️demo/🧪️tests/🟦️.ts`.
- `@semio-tech/draw-js` package (`🖍️draw/📦️packages/🟦️typescript/🟦️.ts`) was not traced import-by-
  import in this pass; flagged for a follow-up if the TS surface needs verifying end to end.
- **Cross-language fixture oracle**: the plugin-root `🖍️draw/🧪️oracle/🔣️.json` explicitly declares
  `"oracles": []`, `"noOracleDecisions": []` and only an `oracleHostPackages` pointer to
  `semio-s-plugin-stdio-test-oracle` (rust), with the comment "This plugin's contribution … It
  registers NO reference implementation: every artifact `🖍️draw` owns is a semio-NATIVE document
  … and no third party reads or writes that envelope." The real oracle work lives per-subset
  (`🪆️subsets/*/🔮️oracle/🔣️.json`) — see §7's `✳️any` manifest, which registers `quick-xml` as
  the third-party SVG reader. There is no Rust↔TS fixture oracle for drawing's own DSL/pack — only
  the SVG bridge is independently checked (via `quick-xml`), and only from the Rust side
  (`oracle-probe`, §7); the TS twins are not exercised by any oracle.

## 6. Examples

- `✳️any/📚️examples/🎬️demo/` — real, not a placeholder: `🦀️.rs` exposes
  `ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)` with `PRIMARY_TEXT =
  include_str!("🖼️assets/🗣️.dsl.semio")` (this include_str! path DOES resolve — verified
  separately from §7's broken ones); `🟦️.ts` exposes matching `id`/`label`/`icon` constants (no
  `primaryText` constant in the TS twin — it would need its own `include`/fetch of the same
  `.dsl.semio` asset, not confirmed present). Has its own `🧪️tests/{🦀️.rs,🟦️.ts}`.
  `🛂️manifest.json` present alongside the `.dsl.semio` asset.
- This shape (`ID`/`label()`/`ICON`/`source()` in Rust, `id`/`label`/`icon` in TS, both reading the
  same `🗣️.dsl.semio` asset) matches what
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`'s
  `registryExampleCatalog` (per the task brief, recently extended to scan
  `🪆️subsets/<s>/📚️examples/`) would expect to find — this pass did not re-read that discovery
  file to confirm the scan glob matches `✳️any/📚️examples/🎬️demo/` exactly; flagged as a quick
  follow-up if example-catalog completeness needs proving.
- No `📚️examples/` directories exist under the other 4 subsets (style/metadata/transform/
  structure) — only `✳️any` has one.

## 7. Tests

Five `🧪️tests/mutate-drawing-1[-any]-*` hosts, one per subset:

| subset | test dir | kinds covered | fixture `include_str!` paths |
|---|---|---|---|
| `✳️any` | `🎨️mutate-drawing-1-any-style` | replace-layer-fill, replace-layer-stroke, set-layer-blend-mode, set-layer-opacity | **4/4 BROKEN** (see below) |
| `✳️any` | `🔀️mutate-drawing-1-any-transform` | set-layer-boolean-operation, update-layer-trace-params, update-layer-transform | **3/3 BROKEN** |
| `✳️any` | `🧱️mutate-drawing-1-any-structure` | create-layer, delete-layer, duplicate-layer, reorder-layer | **4/4 BROKEN** |
| `✳️any` | `🪪️mutate-drawing-1-any-metadata` | rename-layer, set-layer-locked, set-layer-visible | **1/3 BROKEN** (rename-layer only; locked/visible resolve) |
| `✳️any` | `🔁️round-trips-the-committed-document` | native DSL round-trip (`ArtifactDsl`) | not fixture-`include_str!`-based; feature-gated (`#[cfg(feature = "sut")]`) cucumber/`sut` adapter |
| `🎨️style` | `🎨️mutate-drawing-1-style` | same 4 as above, subset-owned original | **4/4 BROKEN — identical stale paths as the `✳️any` duplicate** |
| `🏷️metadata` | `🪪️mutate-drawing-1-metadata` | rename-layer only referenced via `include_str!` | **1/1 BROKEN** |
| `🔀️transform` | `🔀️mutate-drawing-1-transform` | same 3 as above | **3/3 BROKEN** |
| `🧱️structure` | `🧱️mutate-drawing-1-structure` | same 4 as above | **4/4 BROKEN** |

**The break, concretely**: each broken `include_str!` references a "friendly-slug" fixture
directory name (e.g. `🌈️solid-to-linear-gradient`, `➕️appends-shape-b-at-the-root`,
`✖️normal-to-multiply`, `🌫️dims-shape-a-to-half`, `🖊️adds-a-dashed-stroke`, `➖️union-to-subtract`,
`🔍️sharpens-the-trace`, `📐️translates-and-scales-shape-a`, `🚫️removes-group-a-with-its-child`,
`🚫️rejects-a-missing-source-layer`, `⬆️moves-shape-a-above-shape-b`,
`✏️renames-shape-a-without-touching-its-id`). The actual on-disk directories all carry a 6-hex-digit
suffix instead (e.g. `🌈️solid-to-linear-9bdbe8`, `➕️appends-shape-b-0b0435`, `✖️normal-to-b12530`,
`🌫️dims-shape-a-to-c25ad9`, `🖊️adds-a-dashed-92bad7`, `➖️union-to-subtract-845e1f`,
`🔍️sharpens-the-117131`, `📐️translates-and-f4316d`, `🚫️removes-group-a-41e1e0`,
`🚫️rejects-a-c88127`, `⬆️moves-shape-a-d7c515`, `✏️renames-shape-a-ba5eed`). Only
`🔒️locks-shape-a`/`🙈️hides-shape-a` (metadata's `set-layer-locked`/`set-layer-visible`) happen to
still match. `include_str!` on a nonexistent path is a `rustc` compile error (not a test failure),
so **12 of the 14 mutation kinds' worth of fixtures block compilation of both the owning-subset
test crate and the `✳️any` duplicate** — i.e. this plugin's test target (and anything that depends
on it building) is broken until every one of these paths is corrected to the hashed name.

`🔬️probes/🦀️oracle-probe/🦀️.rs` (656 lines): real, substantial third-party-carrier reader —
parses exported SVG via `quick-xml` (an approved oracle dependency), offers `svg-structure`,
`svg-compare` (geometry/color/transform delta measurement, not byte-diff), `gate-inputs`, and
`fixtures` subcommands (writes its own SVG directly with `quick-xml::Writer`, never calling this
repo's `write_svg_xml` — keeps the oracle independent of the subject). Its own doc comment (lines
19-27) is itself a useful fidelity map: layer `id`/`name`, `blend_mode`, `fill_rule`, `locked` are
NOT witnessable through the SVG bridge (dropped before reaching `SemioDrawNode`), and Image-layer
fill/stroke/opacity mutations are invisible too (image leaves never reference an interned style).

`#[async_test]` count: **33 files** under `🖍️draw` use `#[async_test]` repo-wide (only
`✳️any/🚪️io/🦀️.rs`'s own `drawing_document_to_svg_bridges_shape_text_image_and_gradient_nodes_
through_semio_drawing` test, confirmed above, is one of them); the five `mutate-drawing-1[-any]-*`
hosts and the demo example's own tests are cucumber/`sut`-adapter-driven (0 `async_test` each) and
mount `semio_repo_test_host::{Adapter, Context, Json, Outcome}` + `semio_s_plugin_stdio_test_oracle::law`
as their shared assertion module, per the `🎨️mutate-drawing-1-any-style` header comment.

## Recommended fix lanes

1. **Compile-blocking, do first**: fix the 5 stdio import names/paths in `✳️any/🚪️io/🦀️.rs:15-19`
   (`DrawingCanvas→DrawCanvas`, `DrawingLayer→DrawLayer`, `DrawingNode→DrawNode`,
   `DrawingStyle→DrawStyle`, `subsets::any→subsets::base` for svg 1.1) — copy the already-correct
   import line verbatim from `🖨️raster/…/🚪️io/🦀️.rs:28` as the reference.
2. **Compile-blocking, do first**: fix all 12 broken `include_str!` fixture paths across the 8
   affected test files (4 subset-owned + 4 `✳️any`-duplicated) to the current hash-suffixed
   directory names listed in §7 — both copies of each duplicated pair need the same fix.
3. Resolve the `"2d.drawing"` vs `"s.draw.drawing"` kind-id split (§1) — either bridge activation
   to resolve through the new `s.draw.drawing` id, or explicitly document/verify that both ids are
   simultaneously live and mutually consistent at runtime; currently nothing in this file proves
   that.
4. Give the TS `IoEntryDescriptorMirror.fidelity` type (`✳️any/🚪️io/🟦️.ts`) a way to represent
   "not implemented" (a `"Stub"`/`"Unsupported"` variant) instead of overloading `"Lossy"` for both
   real-but-imprecise and outright-`Err` hops.
5. Decide whether `txt`/`pack` native DSL codecs (§3/§4 — fully implemented, real) should be
   registered into `io()`'s `NativeCodecs` after all, or whether the "deliberate scope-narrowing"
   comment should instead be promoted into the subset's `🔮️oracle/🔣️.json` as an explicit decision
   record (it currently lives only as a Rust doc comment).
6. Strengthen the TS `DrawingLayerNode` interface (`✳️any/🧬️schema/🟦️.ts`) from the untyped
   `{kind: string; [key: string]: unknown}` catch-all to a discriminated union mirroring the Rust
   `DrawingLayerNode` enum's 7 variants, if TS consumers need to branch on layer kind safely.
