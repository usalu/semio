# 📓️ note-1/any and layout-1/any — carrier reachability, oracle registration, findings

Scope: put `s.note.note@1/✳️any` (33 mutations) and `s.layout.layout@1/✳️any` (25 mutations) under a
qualifying third-party oracle, per `📓️pilot-playbook.md`. Both subsets' vocabulary is domain-native
(note/layout blocks), not format-native, so the applicable template is the mesh/brep "carrier reader"
shape, not the dxf-any/svg-any/pdf-any "same-domain differential" shape.

## Step 0 — carrier reachability, read from every export serializer body

### `s.layout.layout@1/✳️any` — every one of its five carriers is a disqualifying stub

All five confirmed by **quoting the actual code**, then independently corroborated by the framework's
own `bun 🧰️framework/…/🧪️test/📜️script.ts contract`, which reports all five as `testing/contract`
breaches without this investigation asserting anything:

- **dxf** (`…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️component.rs`):
  ```rust
  pub async fn serialize(from: &LayoutSnapshot) -> Result<DxfSnapshot, store::PackError> {
      let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
      serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
  }
  ```
  `LayoutSnapshot`'s own JSON reinterpreted AS `DxfSnapshot` — a type-confusion stub, the JSON-shaped
  sibling of stub shape #2. `DxfSnapshot`'s five non-`schema` fields all carry `#[serde(default)]` and
  none of their names occur on `LayoutSnapshot`, so this **always "succeeds" with a permanently empty
  (0 entities) document**, regardless of layout content. Contract's own words: *"the dxf serializer
  coerces this artifact through serde into an empty dxf document."*
- **png** (`…/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️component.rs`): the identical pattern,
  but `PngSnapshot.width`/`height`/`bitDepth`/`colorType` carry no default, so it does not even reach
  "empty" — it **errors on every real document**. Contract: *"coerces this artifact through serde into
  an empty png document."*
- **svg** / **dwg** (`…/🎨️svg/🔖️1.1/✳️any` and `…/🖊️dwg/🔖️ac1018/✳️any`): both feed
  `<LayoutSnapshot as store::ArtifactDsl>::print_dsl(from)` — this subset's own `.dsl.semio` text,
  confirmed against the committed
  `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (`semio layout.layout.dsl v1` then
  `key=hex(json-bytes)` lines — **not XML**) — into an SVG XML parser and an SVG→DWG geometry
  flattener respectively. Both **fail to parse any real document**. Contract: *"emits the artifact's
  internal DSL text, not svg/dwg."*
- **pdf** (`…/📄️pdf/🔖️1.4/✳️any`): does not fail, but only because it dumps that same raw hex-DSL
  text into one PDF page's text field — an alternate encoding of our own snapshot, not an independent
  structural reading of it (the same "JSON does not count" principle this repository already applies
  to a JSON export of our own schema).

**Verdict: `s.layout.layout@1/✳️any` has no carrier a third-party reader could meaningfully verify
today.** This is recorded as an ADDENDUM to the existing `layout-mutation-semantics` `noOracleDecision`
in `…/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` (that decision already argued the
*vocabulary* side; this addendum closes the *carrier* side with the concrete serializer evidence
above) — **no oracle was registered for layout**, per the playbook's "say so and move on."

### `s.note.note@1/✳️any` — real, but narrow, per carrier

- **dxf** (`…/🖊️dxf/🔖️r12/✳️any/🦀️component.rs`) — REAL: builds actual `DxfEntity::Line` entries and
  calls `print_dxf_document`, the SAME already-oracle-green DXF R12 writer `s.stdio.dxf@r12/✳️any`
  registers against the `dxf` 0.6 crate. But only for `Ink` blocks, and only their raw `points`:
  ```rust
  for block in flatten_blocks(&from.blocks) {
      if let NoteBlockNode::Ink { points, .. } = block {
          for pair in points.windows(2) {
              entities.push(DxfEntity::Line { start: [pair[0][0], pair[0][1], 0.0], end: [...], layer: "0".into(), .. });
          }
      }
  }
  ```
  No `block.x`/`y`/`rotation` is ever added, no visibility filter is applied (unlike the SVG path
  below), and `Line` carries no width field.
- **svg** (`…/🎨️svg/🔖️1.1/✳️any` → `note_document_to_svg` → `note_document_to_drawing_snapshot`,
  `…/🚪️io/🦀️component.rs`) — REAL, and the richest carrier: a genuine bridge through stdio's
  `semio/drawing` subset (`io_dispatch`, never a hand-rolled string). Blocks are
  `.filter(|b| block_visible(b))` (visibility IS honoured here) and each survivor is wrapped in
  `DrawNode::Group { transform: note_block_transform(block), .. }`, where `note_block_transform` reads
  the block's own `x`/`y`/`rotation` into a real SVG `matrix(...)`. Per kind: Ink → real `<path>` +
  `stroke-width`; Text → real `<text>`, but `font_size` is wired to the glyph's `y` COORDINATE, never a
  font-size attribute; Image → real `<image>` with the asset's own bytes when the reference resolves,
  else a fallback outline; Table/Math/Group → ALWAYS a generic outline rectangle keyed only to
  `width`/`height`.
- **pdf** (`…/📄️pdf/🔖️1.4/✳️any`) — REAL but very lossy: `title` + every `Text` block's paragraphs,
  space-joined, onto ONE page. No position; no other block kind; same missing-visibility-filter bug as
  DXF (independently confirmed on a second serializer).
- **png** (`…/📷️png/🔖️1.2/✳️any`) — technically real (a valid PNG, real `encode_png`), but the pixel
  buffer is unconditionally opaque white sized to `note_document_bounds` — *"no block content is
  actually painted"* per its own doc comment. **Zero content fidelity; excluded from registration** —
  registering it would risk exactly the "empty result read as ok" anti-pattern the playbook warns
  against, for a signal (canvas bounds only) no mutation in this vocabulary specifically targets.
- **dwg** (`…/🖊️dwg/🔖️ac1018/✳️any`) — real geometry bridge (note→svg→`svg_to_dwg_bytes`, genuine
  usvg path-flattening into DWG polylines), but **no qualifying third-party DWG reader exists in this
  repository's approved dependency set** (`🔒️dependencies.json` has no `dwg` read crate; AutoCAD's
  binary format has no credible pure-Rust reader here). Left un-oracled and reported, per the task's
  own instruction, rather than invented.

## Step 1 — per-mutation witnessability (33 kinds)

**16 witnessable**, registered with real carriers: `rename-note` (pdf — title text), `create-asset` /
`replace-asset-payload` / `delete-asset` (svg — image payload, when referenced+visible),
`create-block` / `delete-block` / `delete-blocks` / `duplicate-block` / `duplicate-blocks` (dxf+svg+pdf
— existence, via whichever content capability the targeted block kind reaches), `drag-blocks` /
`move-block` / `resize-block` (svg — transform), `change-block-visible` (svg only — DXF/PDF never
filter by visibility, a confirmed cross-carrier bug in the SUBJECT, reported rather than
worked around), `edit-block-text` (pdf+svg), `change-block-ink-width` (svg only — DXF's `Line` carries
no width), `edit-block-ink-stroke` (dxf+svg).

**17 honestly un-oracled**, each carrying an `oracleRequirement` naming a capability NO registered
oracle provides (verified: `bun … contract` reports all 17 as `testing/oracle` HIGH breaches —
*"requires a third-party-library for capability X, and none is registered"* — the mechanism working
exactly as intended, not a gap I introduced):

| capability (no provider) | mutations | why |
| --- | --- | --- |
| `note.editor-only-setting` | 8× grid/snap/pencil/eraser | never written to any carrier — editor state, invisible by construction |
| `note.block.reparent` | move-block-to-container | its own diff removes+re-adds the SAME block clone under a new parent, never touching x/y |
| `note.block.name` | rename-block | no carrier renders `name` |
| `note.block.lock-state` | change-block-locked | no carrier's render path reads `locked` |
| `note.block.font-size` | change-block-font-size | SVG wires `font_size` to the text glyph's Y COORDINATE, never a size attribute — a real subject bug this investigation surfaces rather than one an oracle could quietly confirm |
| `note.block.math.content` | edit-block-math | Math always renders as a generic outline rect — TeX never reaches any carrier |
| `note.block.table.cell-content` | insert/remove-table-row, insert/remove-table-column | Table always renders as a generic outline rect keyed to width/height; confirmed by reading `insert-table-row`'s own diff — it mutates `rows`, never `width`/`height` |

## Oracles registered (`…/🗒️note/…/✳️any/🧪️oracle/🔣️.json`)

Three `third-party-library` entries, all reusing crates ALREADY approved in `🔒️dependencies.json` and
already linked into the shared `semio-s-plugin-stdio-test-oracle` crate — no new dependency:

- `dxf-crate-note-ink-reader` — `dxf` 0.6 (same crate/reader `dxf-crate-r12-mutate` already qualifies).
- `quick-xml-note-drawing-reader` — `quick-xml` 0.42 (same crate the `🎨️svg` subsets already qualify).
- `lopdf-note-text-reader` — `lopdf` 0.44 (already approved, capabilities `pdf-edit`/`pdf-parse`).

Plus three subset-scoped `comparisonProfiles` (`semantic-note-dxf-ink-v1`,
`semantic-note-svg-drawing-v1`, `semantic-note-pdf-text-v1`) and a full `mutationManifests` entry (33
mutations, `carriers` + honest `oracleRequirements` per mutation as above). The pre-existing
`note-python-independent` (`cross-semio-implementation`) entry is untouched — it remains a required
SUPPLEMENT, and this registration is what discharges the external-oracle requirement it explicitly
said was still owed.

**Two pre-existing repo gaps fixed because they blocked the manifest generator, not invented for this
ticket**: `➕️create-block`, `🎯️duplicate-block`, `👥️duplicate-blocks` (note) and `➕create-frame`,
`🌱create-page` (layout) had no `🔣️payload.schema.json` at all — `bun … manifest scaffold` refused to
derive a descriptor without one. Added, following the exact convention their sibling leaves already
use (a minimal `NoteBlockNode`/`Frame`/`Page` `$defs` shape, `additionalProperties: true`, mirroring
the equally-loose shape already committed in each subset's own `📸️snapshot/🔣️component.json`).

## Rust oracle implementation — real, delegating, `cargo check`-verified

`…/🗒️note/…/✳️any/🧪️oracle/🦀️component.rs` (new) exposes `project_note_dxf` / `project_note_svg` /
`project_note_pdf`, each **delegating to an already-registered projector this crate carries for
another subset** rather than re-implementing a reader a third time:

- `project_note_dxf` → `crate::artifacts::dxf::standards::v_r12::subsets::any::project_dxf_r12` (the
  SAME function `s.stdio.dxf@r12/✳️any` is oracle-green under).
- `project_note_svg` → `crate::markup::live::{parse_markup, project_markup}` (this crate's shared
  `quick-xml` tree reader; SVG is XML, nothing note-specific needed).
- `project_note_pdf` → `crate::document::project_pdf` (this crate's shared `lopdf` reader).

Wired into `…/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs` under a new `note::artifacts::note::standards::v1::subsets::any`
module (a "🔖️Plugins" region, sibling to the existing stdio-only "🔖️Artifacts" region — this crate's
own `📦️lib.rs` doc comment and note's artifact-root `🧪️oracle/🔣️.json` already anticipated a
non-stdio owner reaching it this way).

**Verified, not merely written:**
- `cargo check --features oracles` on `semio-s-plugin-stdio-test-oracle` compiles clean (only 3
  pre-existing, unrelated warnings) — confirms the module path, the delegation, and the crate wiring
  are all correct.
- `cargo test` for the SAME crate currently fails to even build its test tree — but for reasons
  independently confirmed to be **pre-existing and unrelated**: `git status` shows another session
  mid-rename (`🦀️component.rs`→`🦀️.rs`, `component.feature`→`🥒️.feature`) across the STEP
  `✳️cc1`–`✳️cc6` and `pptx` subsets, whose `#[cfg(test)]` modules `include_str!` a `🔣️component.json`
  that has already been renamed to `🔣️.json` on disk. Confirmed by `git log`/`git status`, not
  inferred.
- Runtime proof was still obtained, isolated from that breakage: a standalone scratch crate
  (`…/🧪️v1-adversarial-probe`-adjacent scratch, not committed — see `📜️note-oracle-verify-main.rs`
  below) links `dxf` 0.6 / `quick-xml` 0.42 / `lopdf` 0.44 directly and, at runtime:
  - parses the EXACT minimal DXF text `NoteIntoDxf` would emit for one Ink `points.windows(2)` pair →
    recovers `start=(0,0) end=(10,20)`.
  - parses the EXACT `<g transform="matrix(1,0,0,1,5,10)"><path d="…"/></g>` shape
    `svg_element_from_draw_node` writes → walks `["svg","g","g","path"]` and recovers the transform
    attribute string verbatim.
  - builds a PDF with `lopdf` itself (`Tj "hello from note"`), saves it, reads it back, and recovers
    the exact text.
  - Full stdout: `[dxf] OK: parsed 1 LINE entity start=(0,0) end=(10,20)` /
    `[svg] OK: walked ["svg", "g", "g", "path"], inner transform=Some("matrix(1,0,0,1,5,10)")` /
    `[pdf] OK: round-tripped Tj text ["hello from note"]` / `ALL OK`. Source kept at
    `🔬️note-oracle-verify/` in this ticket folder (`Cargo.toml` pins `dxf = "0.6"`,
    `quick-xml = "0.42"`, `lopdf = "0.44"`; `cargo run` reproduces the run).
- The FULL end-to-end pipeline (actually running note's own mutation host, generating
  `fixtureManifests` from real note-serializer output, `matrix`, `fixture reproduce --subset note-1-any`)
  is **blocked repo-wide**, independently of this ticket: `cargo check` on `semio-s-plugin-stdio` (the
  PRODUCTION crate note's own DXF/PDF/PNG serializers depend on) fails with **4620 errors** right now.
  This matches `📌️status.md`'s own already-recorded finding — a peer's in-flight `Mutations` derive
  change landed without the aggregate-file renames it now requires. Confirmed independently in this
  session, not merely cited.

## What is NOT done, honestly

- `fixtureManifests` (third-party-generated fixture files under `🧫️fixtures/`) were **not** produced —
  building them means running note's own serializer to get real DXF/SVG/PDF bytes for chosen
  before/after `NoteSnapshot`s, which needs `semio-s-plugin-stdio` to build. Blocked, see above.
- `matrix` / `fixture reproduce --subset note-1-any` were **not** run for the same reason.
- No raster/perceptual gate was built or validated both ways — note's only raster carrier (png) was
  excluded at Step 0 for zero content fidelity, so there is nothing for a raster gate to compare yet.

## Files touched

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — 3 oracle registrations, 3 comparisonProfiles, full `mutationManifests` (33 mutations, carriers+oracleRequirements).
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — new, the three delegating projectors + runtime smoke tests.
- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/📦️lib.rs` — new `note::artifacts::note::standards::v1::subsets::any` module wiring.
- `✏️s/🔌️plugins/🗒️note/…/🧬️mutations/{➕️create-block,🎯️duplicate-block,👥️duplicate-blocks}/🔣️payload.schema.json` — new, unblocked the manifest scaffold.
- `✏️s/🔌️plugins/🗒️note/…/🧬️mutations/*/🔣️.json` (33 files) — new, auto-scaffolded leaf descriptors.
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json` — noOracleDecision addendum with the concrete serializer evidence.
- `✏️s/🔌️plugins/📏️layout/…/🧬️mutations/{➕create-frame,🌱create-page}/🔣️payload.schema.json` — new, unblocked the manifest scaffold (leaf descriptors were also scaffolded for the other 23 layout leaves, but layout's manifest was deliberately NOT written — no qualifying oracle exists to discharge it).
