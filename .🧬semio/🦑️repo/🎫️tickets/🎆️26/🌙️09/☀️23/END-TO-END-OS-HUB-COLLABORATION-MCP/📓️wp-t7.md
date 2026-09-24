# WP-T7: GIS GeoJSON, generation2d Example Export, Non-io Crate Failures, Raster DWG, DWG Oracle

Slice: T7 (session 10). Captures: `.tmp-ticket/wp-t7/generated/`. Kept inputs: `wp-t7/run-batch.sh`
(now takes `crate[:features]` lines), `wp-t7/crates-{1,2,3}.txt`. Private target: `wp-t7/target`.
W1 request: `.tmp-ticket/wp-w1/requests/t7.txt`. All results below are measured runs (native).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. gismap GeoJSON import/export | landed | stdio-json 86/86 (`stdio-json-1.txt`, `batch-semio-s-artifact-stdio-json.txt`), gismap 158/158 (`gismap-1.txt`) |
| 2. generation2d export on the bundled example | landed (3 real bugs fixed) | lib 274/274 + `bundled-example-export` 1/1 with `component-app-assembly` (`generation2d-12.txt`), 176/176 without (`generation2d-13.txt`) |
| 3a. flow | fixed (root cause: process-global built-node pool probe sharing a binary) | lib 255/255 + `tree-projection-arena` 1/1, six consecutive runs (`flow-s1..6.txt`) |
| 3b. fem-2d timing laws | fixed (tests now measure the product law) | 1038/1038 (`batch-semio-s-artifact-fem-2d.txt`) |
| 3c. draw | fixed (3 root causes) | 282/282 (`draw-3.txt`) |
| 3d. raster | green incl. rewritten dwg import | 228/228 (`batch-semio-s-artifact-raster-raster.txt`) |
| 3e. pptx | fixed (committed fixture) | 76/76 (`batch-semio-s-artifact-stdio-pptx.txt`) |
| 4. raster dwg import history | repaired | raster 228/228; stdio-semio 2657/2657 (`batch-semio-s-artifact-stdio-semio.txt`) |
| 5. DWG third-party oracle | implemented (`acadrust`); exposes two codec gaps (spawned task) | stdio-dwg 67/67 + 3 ignored (`dwg-4.txt`); ignored laws fail as stated (`dwg-5.txt`) |

## 1. GeoJSON (RFC 7946)

Schema-first home: a new subset **`s.stdio.json@rfc8259/geojson`** (GeoJSON is a constraint profile of JSON,
exactly like the existing 🛜️i-json subset), not a gismap-private format.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/`
  - `🧬️schema/🔣️.json` — JSON Schema of the model (Feature/FeatureCollection/7 geometries, linear ring, GJ2008 named `crs`).
  - `🧬️schema/🦀️.rs` — typed model (`GeoJsonGeometry/Feature/Id`), reader `read_geojson[_text]` over the base
    subset's lexeme-preserving `JsonValue` (exact decimal → f64, no intermediate parser), writer `write_geojson`
    (FeatureCollection, closed right-handed rings, never `crs`), `check_geojson_conformance`, builder (base
    mutations + RFC 7946 build gate), analyzer, facets.
  - `🚪️io/🦀️.rs` — composer (stamps `geojson` only when conformant) + `SubsetValidator`; registered in the
    rfc8259 composer aggregator and the artifact root's `subset_validators`; `📜️artifact-definition.json`
    gained the composer and subset-validator runtime rows; subsets `🔣️.json` gained `geojson`.
- **CRS policy** (stated in the schema doc and the JSON Schema): no `crs` → WGS 84 lon/lat (RFC 7946 §4);
  GJ2008 named `crs` accepted only for CRS84/EPSG:4326 (read lon/lat) and spherical Web Mercator
  EPSG:3857/900913/102100/102113 (inverse-projected in our code, heights kept); every other named CRS and every
  linked CRS is refused with a pointer to `/crs…`; after projection every position must be inside
  lon ∈ [−180,180], lat ∈ [−90,90] (projected metres without `crs` are refused, not guessed). The writer only
  ever writes WGS 84 (the map already stores lon/lat) and refuses out-of-range positions.
- gismap leaves `🚪️io/{📤️export/🧵️serializers,📥️import/🧩️deserializers}/🗿️artifacts/🔣️json/🔖️rfc8259/🌍️geojson/`,
  composer read + export rows, capability `s.gis.gismap.composer.geojson`; `artifact_kind()` now derives its
  stdio kind lists from `🚪️io` (it still declared pdf/png/svg imports that T3 removed).
  - **Fidelity export `Semantic`**: every feature, id, payload member and coordinate survives the round trip;
    rings come back without the closing position and right-hand wound, a region's `points` comes back as `ring`.
  - **Fidelity import `Lossy`**: null geometry dropped; multi-part/GeometryCollection split into `<id>#<n>`;
    numeric ids become text; missing ids `feature-<n>`; repeated ids in a family get `#<n>`; `bbox`/foreign
    members dropped; properties named like geometry members (`id lon lat alt points ring holes`) dropped.
  - Holes are preserved in the opaque payload (`holes`), 3D positions as `alt` / third coordinate.
- **Language-agnostic fixtures**: `🌍️geojson/🧫️fixtures/🌍️read-write/🔣️.json` (12 read→write/refusal vectors incl.
  Web Mercator, CRS84, left-handed rings, open rings, unknown CRS) and
  `gismap/…/🧫️fixtures/🌍️geojson-io/🔣️.json` (exact export + import vectors).
- **Third-party oracle** (test-only dev-dependency, added to `🔒️dependencies.json` by hand — my two entries only,
  the 9 other new peer deps were left for their owners): the `geojson` 1.0.0 crate
  - reads every document our writer emits into the same ids/properties/coordinates (≤ 2 ulp: the workspace
    `serde_json` lacks `float_roundtrip`, the oracle's decimal reader is not correctly rounded — documented in the test);
  - writes documents our reader accepts exactly;
  - reads the export of the bundled Liège map (152 points, 149 routes) with the map's coordinates, and our
    import restores the map; polygon + hole areas (`geo` crate) survive the round trip.
- **GeoPackage / Shapefile — not added**: GeoPackage is an SQLite database; reading/writing SQLite pages in our
  own code is a codec project of its own and no runtime dependency is allowed. Shapefile is a multi-file set
  (`.shp/.shx/.dbf/.prj`) with ONE geometry type per file, DBF attributes limited to 10-character names and
  fixed types, while the io leaves produce one byte payload and the map carries three families with opaque
  payloads — only a zip of three shapefile sets with lossy attributes could carry it. Neither is honest today.
- Not done: no TS reader oracle (d3-geo is available; the Rust `geojson` crate covers the oracle requirement);
  no `🔮️oracles/🔣️.json` manifest for the geojson subset (no mutation vocabulary of its own; T3's gltf/csv/zip
  codec oracles are likewise unregistered); no .geojson representation row (format kinds are per artifact,
  `stdio.json` already lists it).

## 2. generation2d export on the bundled example

Measured on the live example (flow host + packaged `draw` extension linked in an own test binary):
1. **The bundled demo produced no drawing at all**: it was `slider → math.add → preview` (a number).
   Replaced by `slider(Width 30) → draw.shape.rect(width) → draw.style.stroke → preview`, neuron input ports
   declared in the document (the bundled-document convention, so the bare host resolves endpoints).
2. **Drawing handles were never found** (framework + generation2d): both collectors kept only handles starting
   `drawing-`, but the kernel's handles are hex content addresses. Now identified by the `draw.drawing` schema:
   `🧰️framework/…/🌊️flow/🌉️bridge/🦀️.rs` (this also fed `retain_drawing_handles` an empty live set on every
   evaluation) and generation2d `collect_drawing_handles_from_eval`.
3. **Every intermediate drawing was exported** (rect AND its stroked copy): new `output_drawing_handles`
   (drawings a synapse delivers into an output widget) used by the page exports and `drawing:out`
   (`generation_output_layers`); the generate-preview window keeps showing every node's drawing.
4. **The shared SVG writer dropped a layer root group's transform** (the page margin): fixed in
   `🧿️semio/…/🖊️drawing/…/🎨️svg/…/🦀️.rs`.
- Test `🚪️io/🧪️tests/🖼️bundled-example-export/🦀️.rs` (`[[test]] bundled-example-export`): canvas 62×42,
  one path, svg margin transform, pdf header, png outline pixels inside the 16-unit margin, dxf polyline.
- Pinned lib laws updated to the new demo: `⏯️preview-eval-run.json` demoRun (`outline` failed too, entity key
  added), three tests remove `rect` and `outline`, flow-window operator assertion, node-graph-edit law no longer
  contributes a fixture extension (the contributed stub made the process-wide registry park the demo laws).
- `procedural` `depends-on` gained `flow-extension-draw` (the served guest must stage it for the demo).
- The plugin descriptor `✏️s/🔌️plugins/🌀️procedural/🔣️.json` (generated, holds the old demo text) → W1 describe.

## 3. Five crates

- **flow** (255/256 flaky): the fixture-tree projection law SATURATES the process-global built-node page pool
  and asserts terminal emptiness; any sibling test building UI in the same binary made it (and them) fail. The
  partial RwLock scheme did not cover dispatch/settle builds. Root fix: the law moved to its own test binary
  (`✏️editor/🧪️tests/🌳️tree-projection-arena/🦀️.rs`, `[[test]] tree-projection-arena`); the lock machinery
  and its 7 call sites were removed. Six consecutive full runs green.
- **fem-2d** (two "< 8 ms per step" laws measured 14–40 ms under peer load): the product law is
  `semio_framework_trace::StepOverrunLedger` — one step over `INTERACTIVE_STEP_CEILING_US` is recorded, only
  `SUSTAINED_OVERRUN_QUARANTINE_STEPS` (4) consecutive overruns attribute latency to the step. The tests measured
  the wrong quantity (max single wall-clock sample). They now apply the same law (`StepLatency` in
  `✏️s/🔨️modules/🏗️fem/⚙️engine/{🧮️analyses,🕸️mesh}/🧪️tests/🔬️unit/🦀️.rs`) using the job crate's own
  constants; ceiling and budget unchanged.
- **draw** (3 failures, 3 root causes):
  1. editor/viewer never declared `child_restore_projection` → every live envelope load faulted (added, same
     shape as raster/flow);
  2. `🔌️plugin` `ActiveArtifactEnvelopeDecode::poll` answered `Ready` for a decode cancelled after its worker
     finished, so `advance_artifact_envelope_load` tried to admit it and faulted
     `replacement-output-not-ready`; it now polls `Cancelled` (maintenance closes the record). Plugin envelope
     laws 9/9 (`plugin-envelope.txt`);
  3. the store initializer failed the whole load when the process mutation-arena pool was `NotReady` (still
     bootstrapping) or `Contended` (another thread's bounded borrow); both now yield and retry
     (`DrawingMutationCandidateAuthority::try_new` returns the typed error); the laws borrow the same way.
- **raster**: green once item 4 landed (the earlier arena/poison failures do not reproduce: 228/228).
- **pptx**: the exact-source law read the gitignored `temp/…pptx` (absent). It now runs on the committed
  PowerPoint-written `🧱️base/🧫️fixtures/📽️.pptx` (7 slides, 55 parts, 22 rels); byte-exactness is held to the
  export of its import plus a logical re-import check (the fixture's ZIP framing is not the writer's).

## 4. Raster DWG import

History (read-only): the leaf last changed in `9869c6e99b`/`6152f9ca6a` (path renames only); nothing of a peer
was lost — the restored leaf equals HEAD and T3's io diff only removed pdf/dwg EXPORT. But the leaf was
inconsistent with the codec: it called `decode_dwg` then `dwg_from_bytes`, which accepts only `AC1015`, so
every R2004-family file the dialect claims was rejected, and its own geometry converter (LINE/LWPOLYLINE/3D
polyline only) duplicated the shared bridge; its tests bypassed the byte path. Now: `decode_drawing(bytes, Dwg)`
(stdio's one drawing←dwg bridge) → `raster_document_from_dwg_drawing` (fit world drawing to page, y flip,
render, canonical PNG; the silent raw-PNG fallback is gone). The shared `dwg_geometry_to_path_segments`
gained LINE, ARC, ELLIPSE, POLYLINE3D and the −Z extrusion mirror (tilted planes → none), with a unit law.
Tests now go through real DWG bytes (`🔬️dwg-import`: page = world bounds, empty file, non-DWG refusal).

## 5. DWG third-party oracle

Options surveyed: LibreDWG (GPL-3.0 C, a system install — `dwgread`/`dwg2dxf` exist on this Mac via Homebrew
but are not zero-touch or cross-platform, so used only for manual cross-checks); ODA File Converter (proprietary,
not installed); a DWG→DXF path (none of ours is independent); **`acadrust` 0.5.5** (MPL-2.0, pure Rust, reads
AND writes DWG AC1012–AC1032) — implemented as the test-only dev-dependency oracle, no network at test time:
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/…/🔟ac1024/…/🚪️io/🧪️tests/🔮️acadrust-oracle/🦀️.rs`.
- **Holds**: both readers agree entity for entity on the committed AutoCAD AC1024 drawing (68 model-space
  LINE/LWPOLYLINE/ARC/CIRCLE/TEXT).
- **Ignored acceptance laws (real gaps, spawned task "Make the DWG codec read and write real DWG")**:
  - our `dwg_to_bytes` writes a private container under an `AC1015` sentinel: acadrust reads it as empty and
    LibreDWG refuses it (`ERROR 0x800`) → every DWG export in the repo is unreadable by CAD tools;
  - our reader fails every acadrust-written version (AC1018 R2010-layout frames, AC1021 header, AC1024 classes
    marker, AC1027/32 LINE bitstream) while LibreDWG reads each with the 3 entities.
- The recorded no-oracle decision claimed "no permissively licensed Rust DWG reader exists"; corrected in
  `🔮️oracles/{🔣️.json,🦀️.rs}` (it still stands for the 3-kind mutation vocabulary).
- Also fixed: stdio-dwg's unit tests `include_bytes!` a gitignored `temp/architectural_example.dwg` (crate did
  not compile on a clean tree) → committed asset; the stale demo `🎒️.pack.semio` regenerated with the crate's own
  sanctioned `zzz_write_demo_fixtures` (decode equality held; only the byte encoding had moved).

## Not attributable / left open

- `semio-framework-os-flow` lib: 208/249, 41 failures (mostly "ordered-map root must be explicitly retired
  before drop", vcs/wasm_session/registry laws) — not in this slice's crates; the drawing-handle collector law
  still passes (`batch-semio-framework-os-flow.txt`).
- Other stdio tests still read gitignored `temp/` files (mp4 ×4, deflate ×2) → spawned task.
- Descriptors/guests: W1 request filed (stdio, gis, procedural, raster, draw; both framework edits touch every guest).

## Files changed

- stdio json: `🌍️geojson/{🧬️schema/{🦀️.rs,🔣️.json,🟦️.ts,🧪️tests/🔬️unit/🦀️.rs},🚪️io/{🦀️.rs,🟦️.ts},🧫️fixtures/🌍️read-write/🔣️.json}`,
  `🪆️subsets/🔣️.json`, `🧱️base/🚪️io/🦀️.rs`, root `🦀️.rs`, `📜️artifact-definition.json`, Cargo.toml.
- gismap: io leaves (2 new + `.ts`), `🚪️io/🦀️.rs`, `🚪️io/🧪️tests/🔬️unit/🦀️.rs`, `🧫️fixtures/🌍️geojson-io/🔣️.json`, root `🦀️.rs`, Cargo.toml.
- generation2d: demo `🗣️.dsl.semio`, `🧬️schema/🦀️.rs`, `🚪️io/🦀️.rs`, `✏️editor/🦀️.rs`, `🚪️io/🧪️tests/🖼️bundled-example-export/🦀️.rs`,
  `🧫️fixtures/⏯️preview-eval-run.json`, 4 editor test files, Cargo.toml; procedural plugin Cargo.toml.
- framework: `🌊️flow/🌉️bridge/🦀️.rs`, `🔌️plugin/🦀️.rs` (poll).
- stdio semio: drawing svg export leaf, drawing←dwg import leaf doc.
- stdio dwg: `🚪️io/🦀️.rs`, `🚪️io/🧪️tests/{🔬️unit,🔮️acadrust-oracle}/🦀️.rs`, `🔮️oracles/{🔣️.json,🦀️.rs}`, demo `🎒️.pack.semio`, Cargo.toml.
- raster: `🚪️io/🦀️.rs`, dwg leaf, `🧪️tests/🔬️dwg-import/🦀️.rs`, svg leaf doc, schema doc, root doc.
- draw: editor + viewer `🦀️.rs`, `🧬️schema/🧰️owned/🦀️.rs`, `…/🔬️retained-mutation-authority/🦀️.rs`.
- flow: unit tests, panels/interactive-job/window-ownership tests, new `🌳️tree-projection-arena` test, Cargo.toml.
- fem: `⚙️engine/{🧮️analyses,🕸️mesh}/🧪️tests/🔬️unit/🦀️.rs`. pptx: base schema unit tests.
- `Cargo.lock` (geojson, acadrust), `🔒️dependencies.json` (+2 entries).
