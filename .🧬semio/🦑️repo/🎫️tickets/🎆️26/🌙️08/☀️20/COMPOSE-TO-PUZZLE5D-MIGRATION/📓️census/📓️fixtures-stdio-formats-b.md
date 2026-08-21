# 🧪️ Handcrafted mutation fixtures — stdio formats slice B (24 trees, 24 mutations)

24 single-mutation `🗄️stdio` trees, one handcrafted `📄set-snapshot` case each. Every `before`,
`after`, mutation, diff and outcome was authored from a direct read of that tree's own
`🔺️diff/🦀️component.rs` (`between` / `diff_set_snapshot`) and its snapshot/diff serde attributes —
never from a leaf's name or docstring.

## 1. Cases

| tree | case |
| --- | --- |
| `📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any` | `retags-the-catalog-revision-and-rewrites-an-item-label` |
| `📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any` | `records-jfif-print-density-and-a-restart-interval` |
| `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any` | `retunes-gamma-and-repaints-the-second-pixel` |
| `📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any` | `promotes-the-second-movi-chunk-to-a-keyframe` |
| `🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any` | `resamples-to-16-khz-and-doubles-the-pcm16-amplitude` |
| `🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any` | `bumps-the-version-lexeme-and-appends-a-tag` |
| `🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any` | `bumps-the-auxiliary-save-counter` |
| `🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any` | `retitles-the-summary-and-records-the-last-editor` |
| `🖊️dxf/🏅️standards/🔖️r12/🪆️subsets/✳️any` | `widens-the-circle-entity-radius` |
| `🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any` | `recolors-the-second-palette-slot-to-magenta` |
| `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any` | `stamps-a-software-tag-and-adds-an-image-description` |
| `🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any` | `raises-the-flevel-hint-and-extends-the-payload` |
| `🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️any` | `lifts-the-third-vertex-and-gives-it-an-explicit-w` |
| `🟪️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any` | `renames-the-solid-and-closes-the-wedge-with-a-third-facet` |
| `🧿️semio/…/🪆️subsets/✳️any` | `replaces-the-envelope-wrapping-a-value-subset` |
| `🧿️semio/…/🪆️subsets/✳️value` | `retypes-a-map-member-and-repoints-a-graph-node` |
| `🧿️semio/…/🪆️subsets/✳️video` | `retimes-the-track-and-promotes-a-sample-to-a-keyframe` |
| `🧿️semio/…/🪆️subsets/✳️audio` | `rerates-to-48-khz-and-rewrites-the-right-channel` |
| `🧿️semio/…/🪆️subsets/✳️flow` | `relabels-and-repositions-the-transform-node` |
| `🧿️semio/…/🪆️subsets/✳️model` | `slides-the-wall-and-attaches-a-fire-rating-pset` |
| `🧿️semio/…/🪆️subsets/✳️animation` | `steps-the-spin-channel-and-appends-a-keyframe` |
| `🧿️semio/…/🪆️subsets/✳️document` | `bolds-the-body-paragraph-and-finalizes-its-copy` |
| `🧿️semio/…/🪆️subsets/✳️cad` | `dims-the-walls-layer-and-widens-the-circle` |
| `🧿️semio/…/🪆️subsets/✳️presentation` | `rewrites-the-second-slides-textbox-and-adds-a-speaker-note` |

Each case carries the mandatory six: `📸️snapshot/⬅️before`, `📸️snapshot/➡️after`,
`🦠️mutation/🔣️component.json`, `🔺️diff/🔣️component.json`, `🎯️outcome/🔣️component.json`,
`🦀️component.rs`. All 24 are `applied` (a genuine change, so no `mutation.no-op` warning), so no
`🔺️diff/🚫️component.absent` is involved anywhere in this slice.

## 2. What each committed diff actually pins

The diff — not the end state — is what distinguishes these cases. Per artifact:

- **xml** — a recursive `root` chain (`attributes.modified` at the top, one `children.modified` step
  into `item`, one `Text` leaf), never a `XmlNodeDiff::Replace` of the root subtree; `prolog`,
  `declaration`, `doctype` all absent.
- **jpg** — three APP0 density scalars + `restartInterval`; the id-keyed `quantTables`/
  `huffmanTables` and the index-keyed `otherSegments` triples stay absent.
- **png** — `gama` (tri-state `Some(Some(45455))`) + whole-buffer `pixels`; `textChunks`/
  `chunkOrder`/`plte` absent even though the snapshot carries all three.
- **avi** — nested `streams.modified[0] → chunks.modified[1]`; the whole-value `mainHeader`, the
  `idx1Present` flag and the `unknownChunks` triple absent.
- **wav** — whole-value `fmt` + whole-value `data`; `otherChunks` absent.
- **json** — a structural `Object` delta (never a root `Replace`) reporting the two changed members
  in BASE member order, with the `tags` array delta as a pure `added[2]`.
- **dwg ac1018 / ac1024** — `DwgDiff` is a per-top-level-field WHOLE-VALUE delta: exactly one key
  (`auxiliaryHeader` / `summary`). Notably `revisionHistory` must NOT follow the save counter.
- **dxf** — `entities.modified[1]` carrying a kind-preserving `Circle` patch with `radius` as its
  only set field; `headerVars`/`tables`/`blocks` absent. (`DxfDiff` has no `otherTables` slot at
  all, so no fixture may place a change there.)
- **bmp** — one `palette.modified[1]` + the replacement `pixels`; none of the twelve
  BITMAPINFOHEADER scalars.
- **tiff** — a TAG-ID-keyed triple with one `modified` (305) and one `added` (270); `byteOrder` and
  `pixels` absent, the five untouched baseline tags not re-listed.
- **deflate** — `compressionLevelHint` + `payload`; CMF nibbles and the tri-state `dictId` absent.
- **obj** — `vertices.modified[2]` with `z` + tri-state `w` only; the other nine top-level slots
  absent.
- **stl** — `solidName` + a pure `triangles.added[2]`; `modified`/`removed` both empty.
- **semio ✳️any** — deliberately a whole `SemioDiff::Replace`, NOT a delegated `SemioDiff::Value`.
  Set-snapshot is the only way the envelope's subset KIND can change, so a per-subset delta would
  reach the same end state and still be wrong. The test asserts the `Replace` shape explicitly.
- **semio ✳️value** — a `Map` root delta whose one modified member is a `Replace` (Int→Float is a
  KIND change), plus an id-keyed `nodes.modified` entry.
- **semio ✳️video** — `streams.modified[0] → samples.modified[1]`; the subtitle stream and the
  stream's identity fields absent, the sample's unmoved `pts` absent.
- **semio ✳️audio** — `sampleRate` + `channels.modified[1]` (whole `samples` vector); `format` and
  `tags` absent.
- **semio ✳️flow** — ID-keyed `nodes.modified["b"]` with `label`/`position` only; `edges` absent.
- **semio ✳️model** — ID-keyed `elements.modified["w-1"]` with `placement` + whole-vector `psets`;
  `spatial`, `relations`, `class`, `geometry`, tri-state `spatialId` absent.
- **semio ✳️animation** — three nested triples down to a pure `keyframes.added[2]`, plus the
  channel's `interpolation`; the sibling scale channel and the tri-state timeline `name` absent.
- **semio ✳️document** — `blocks.modified[1]` as a kind-preserving `Paragraph` patch → one run →
  `style.bold` alone (not the six other `RunStyle` fields the snapshot spells out).
- **semio ✳️cad** — name-keyed `layers.modified["WALLS"]` (`colorIndex`/`visible` only, not
  `lineType`) + handle-keyed `entities.modified["h-1"]` (`entity` only, not `layer`); `blocks`
  absent.
- **semio ✳️presentation** — `slides.modified[1]` → a `TextBox` shape patch with `frame` unset
  (never `Replace`) → one whole-block replacement, plus a pure `notes.added[0]`; `masters` and
  `layouts` absent.

## 3. serde traps found and pinned

1. **A container-level `rename_all` on an ENUM renames only the VARIANTS.** Struct-variant FIELDS
   follow the variant's own `rename_all`, falling back to the container's `rename_all_fields` —
   never to the container's `rename_all`. Verified directly against
   `serde_derive-1.0.228/src/internals/ast.rs:88-94`. This is load-bearing for four of this slice's
   artifacts and the committed JSON follows the RUST behaviour, not the sibling `🟦️component.ts`
   facet:
   - `AviStreamFormat::BitmapInfo` → `bit_count`, `size_image`, `x_pels_per_meter`,
     `y_pels_per_meter`, `colors_used`, `colors_important` stay snake_case (the committed `.ts`
     facet says `bitCount` etc — a real facet/Rust divergence, flagged below).
   - `GeometryRef::Mesh` → `mesh_id` (semio ✳️model).
   - `DocBlock::Paragraph`/`Heading`/`Image` → `style_id`, `image_id` (semio ✳️document and, through
     reuse, ✳️presentation) — while the sibling `DocParagraphDiff` is a plain STRUCT and therefore
     writes the very same field as `styleId`.
   - `DxfEntity::Arc`/`Insert` → `start_angle`, `end_angle`, `block_name` (not exercised by the
     chosen dxf case, but documented in its test so a future case does not get it wrong).
2. **`Option<Option<T>>` does not survive a JSON round trip.** `Some(None)` serializes as bare
   `null` and deserializes back as `None` (= unchanged), so a "cleared" delta is inexpressible in a
   committed fixture. Every tri-state field in this slice is therefore either left ABSENT or driven
   in the round-trippable `Some(Some(v))` direction, and each affected test says so:
   `DeflateDiff::dict_id`, `ObjDiff::mtllib`, `ObjVertexDiff::w` (exercised as `Some(Some(1.0))`),
   `XmlDiff::declaration`/`doctype`, `PngDiff::gama` (exercised as `Some(Some(45455))`),
   `JpgDiff::restart_interval` (`Some(Some(4))`), `SemioModelElementDiff::spatial_id`,
   `AnimTimelineDiff::name`, `SlideDiff::layout_id`, `DocParagraphDiff::style_id`.
3. **Binary payloads are arrays of numbers, never base64.** Every `Vec<u8>` in this slice
   (`PngSnapshot::pixels`, `AviChunk::data`, `JpgSegment::data`, `BmpSnapshot::pixels`,
   `DeflateSnapshot::payload`, `SemioVideoSample::data`, `DwgIndexedPreview::pixel_indices`, …)
   serializes as a JSON number array. `deflate`'s `#[dsl(base64)]` attribute belongs to the
   SEPARATE `ArtifactDsl` grammar and does not touch serde — pinned in that test's docstring.
4. **Adjacent vs internal tagging differs per artifact and is asserted:** `WavData` and `TiffValues`
   are adjacently tagged (`kind` + `value`) because their variants wrap `Vec`/`String`;
   `SemioMutation` is adjacently tagged (`mutation` + `payload`) to avoid a key collision with a
   wrapped subset's own `"mutation"` key; `SlideShape` is tagged `shapeKind` (not `kind`) because
   `Placeholder` owns a field called `kind`.
5. **`skip_serializing_if` is per-field, not per-artifact.** `DwgSnapshot` has none anywhere, so its
   two committed snapshots spell out every zeroed dimension scalar and every `null` UCS relation
   handle (~430 lines each); `DxfSnapshot` skips empty `unknown_group_codes` vectors entirely;
   `ObjSnapshot` skips an omitted `w`/`texcoord`/`mtllib` but still writes `"texcoords": []`.

## 4. Wiring

Each case is wired from its OWN tree's mutations-root `🦀️component.rs`, in an appended
`//#region 🧪️FixtureCases` block — `📦️glue.rs` was not touched:

```rust
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/<case>/🦀️component.rs"]
mod set_snapshot_<case_with_underscores>;
```

A `#[path]` on a module declaration that is NOT inside an inline module block resolves relative to
the directory of the source file it appears in (Rust Reference, "Module source filenames"). The
repo already relies on exactly this: `🧿️semio/…/✳️audio/🚪️io/🦀️component.rs` and
`✳️animation/🚪️io/🦀️component.rs` both say so in their own doc comments and compile today. A
`#[path = "."]`-wrapped inline module would NOT work here, because for a non-mod-rs file the base
for inline-module children is `<dir>/<file-stem>/` — i.e. `…/🧬️mutations/🦀️component/`.

## 5. Verification

`cargo` was not run (forbidden this session, and a peer's de-async sweep has the workspace mid-flight
— see §6). Validation was structural:

- **Repo-wide gate** — `cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust && bun ./📜️script.ts fixtures
  lint --by-tree`: `1558 mutations · 1483 covered`; none of this slice's 24 trees appears in the
  uncovered list any more (they were all listed as `1/1` uncovered at the start).
- **Scoped re-run of the lint's own rules** (`📓️census/scoped-lint-stdio-formats-b.py`, the rules
  transcribed from `📜️script.ts`) — needed because the CLI truncates its error list at 40 rows
  repo-wide: **24/24 trees `1/1` covered, 0 errors, 192 derived-encoding warnings (8 per tree, the
  expected `fixtures generate` gap).**
- **`include_str!`** — every one of the 120 targets across the 24 test files resolves on disk.
- **`#[path]`** — every path attribute in all 24 mutations roots resolves, and each case's test file
  is the one the root actually names.
- **`rustfmt --edition 2021 --emit stdout`** parses all 48 touched Rust files (24 new test files +
  24 mutations roots) with exit 0.
- **JSON integrity** — all 120 committed JSON files parse, have no duplicate keys and end with a
  newline; for every case `before != after`, the mutation payload is byte-identical to `after`, the
  mutation tag is `setSnapshot`, the outcome is `{"status":"applied"}` and the diff is non-empty.
- **Test shape** — every file has exactly the seven required assertions
  (`applies_to_committed_after`, `inverse_restores_before`, `committed_json_is_canonical`,
  `declared_outcome_holds`, `produces_committed_diff`, `committed_diff_is_canonical`,
  `committed_diff_applies_to_after`), reaches the artifact through its own
  `apply_*_mutation` / `<M as protocol::Mutation<S>>::{diff,inverse}` /
  `<D as protocol::MutationDiff<S>>::apply` entry points, and contains no `.await` (de-async target
  style, matching the sweep now landing through this plugin).

No test is claimed to PASS — none was executed.

## 6. Out-of-scope observations (not fixed)

- **Pre-existing lint errors on `🧿️semio/…/✳️any`** — 18 `enum variant has no mutation directory`
  errors (`Brep`, `Mesh`, … `Kit`). They come from `declaredMutations()` reading `SemioMutation`'s
  eighteen tuple wrapper variants and expecting a mutation leaf per variant, while the tree has a
  single `📄set-snapshot` leaf. Present in the baseline run before this work, unaffected by it, and
  not fixable by adding fixtures.
- **Peer de-async sweep is live in this tree.** `✳️any/…/🧬️mutations/🦀️component.rs` changed under
  me during the session (`.await` removals). All edits here were strictly additive appends and the
  peer's changes are intact; the wiring block is still present in all 24 roots. Several files in
  the plugin are currently half-migrated and will not compile as-is (e.g. `📰xml`'s
  `apply_xml_mutation` is a non-`async fn` that still calls `outcome.await`) — deliberately left
  alone.
- **`📼️avi` facet/Rust divergence.** `📸️snapshot/🟦️component.ts` and `🔗️component.graphql` declare
  `AviBitmapInfo` fields as `bitCount`/`sizeImage`/`xPelsPerMeter`/…, but the Rust enum's variant
  fields serialize snake_case (see §3.1). The committed fixtures follow Rust. Either the enum needs
  `rename_all_fields = "camelCase"` or the `.ts`/`.graphql` facets need correcting — an owner
  decision, out of this slice's scope.

## 7. Files

- 24 × `<tree>/📄set-snapshot/🧪️tests/<case>/{📸️snapshot/⬅️before,📸️snapshot/➡️after,🦠️mutation,🔺️diff,🎯️outcome}/🔣️component.json`
- 24 × `<tree>/📄set-snapshot/🧪️tests/<case>/🦀️component.rs`
- 24 × `<tree>/🦀️component.rs` (appended `//#region 🧪️FixtureCases` wiring block only)
- `📓️census/serde-default-skeleton.py` — reads a snapshot schema's Rust source and prints the exact
  serde JSON a default-shaped value produces, used to derive the mechanical skeleton for the large
  `dwg`/`dxf` snapshots (validated against the hand-authored `png` skeleton first).
- `📓️census/scoped-lint-stdio-formats-b.py` — the scoped lint of §5.
