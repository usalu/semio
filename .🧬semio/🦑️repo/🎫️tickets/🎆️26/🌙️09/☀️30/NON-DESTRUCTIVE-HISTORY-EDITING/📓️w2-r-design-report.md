# 📓️ W2-R design — Input-UI rollout for puzzle 3d/5d, shooting, cad, procedural, note, forms, raster and draw

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, rollout executor W2-R-design, 2026-09-30. Contract: `📋️design.md` §6,
`🧭️plan.md` "W2-R brief", manifest meta-schema `$defs/InputUi`. Style reference: W1-F's puzzle 2d pass.

## 1. Outcome

**DONE and VERIFIED.** The strict lint `schema mutation-inputs` reports **0 findings** in every path of the group and exits 0.
Every input in scope carries an explicit `x-semio-ui`, including inputs the glossary already labelled. That covers every
top-level input, every field of every record a leaf reaches through `$ref`, and every union variant:

- **1,133 annotations** in **265 schema documents**: 258 leaf payload schemas plus 7 record documents that leaves `$ref`.
- **34 hard bounds** added, each backed by a Fatal invariant in the leaf's diff or by physical meaning (§3.4).
- **0 new fixture failures**. Every annotation validates against `InputUi` (Python `jsonschema`), every leaf compiles in the
  strict Ajv oracle, and W1-D's TypeScript reader accepts all 262 leaves.

## 2. Census

Before is the census at 04:35. After is the census of the final run (`🗑️generated/w2-r-design/census-after.tsv`).

| path | leaves | inputs | declared before | findings before (labelMissing / optionLabelMissing / refUnresolved) | declared after | findings after | strict exit |
|---|---|---|---|---|---|---|---|
| puzzle `🧊️3d` | 35 | 85 | 69 | 16 (14 / 2 / 0) | 85 | 0 | 0 |
| puzzle `🖐️5d` | 35 | 89 | 76 | 13 (11 / 2 / 0) | 89 | 0 | 0 |
| `🎥️shooting` | 39 | 67 | 40 | 27 (27 / 0 / 0) | 67 | 0 | 0 |
| `📐️cad` | 25 | 47 | 33 | 14 (9 / 5 / 0) | 47 | 0 | 0 |
| `🌀️procedural` | 45 | 57 | 43 | 14 (12 / 0 / 2) | 57 | 0 | 0 |
| `🗒️note` | 34 | 60 | 47 | 13 (13 / 0 / 0) | 60 | 0 | 0 |
| `📋️forms` | 14 | 26 | 15 | 11 (11 / 0 / 0) | 26 | 0 | 0 |
| `🖨️raster` | 17 | 43 | 32 | 11 (11 / 0 / 0) | 43 | 0 | 0 |
| `🖍️draw` | 17 | 35 | 25 | 10 (10 / 0 / 0) | 35 | 0 | 0 |
| **total** | **261** | **509** | **380** | **129** | **509** | **0** | |

Notes on the census:

- No path in scope had a root-union `malformed` finding.
- The two procedural `refUnresolved` findings were the viewer `set-preview-camera` leaves. They held relative-file `$ref`s. A
  peer (the W1-D follow-up) fixed them during this run: one leaf got a local `$defs`, the other now refs it by `$id`.
- The walker already annotated that local `$defs` record, so the fix and the annotations fit together.

Reader census over all 262 leaves, including the uncatalogued `update-path-geometry` and counting nested fields (1,074 inputs):

| widget | count |
|---|---|
| string (text) | 299 |
| reference | 193 |
| stepper | 188 |
| object | 76 |
| vector | 75 |
| boolean (toggle) | 66 |
| array | 45 |
| slider | 39 |
| multiline | 33 |
| any | 26 |
| dial | 19 |
| segmented | 15 |

## 3. Decisions

### 3.1 Targets

Every entity id is `role: target` with `ref {kind, domain, granularity}`. The domain and granularity are the plugin's own
declared interaction domain, so W2-A's `historyEditUseSelection` can fill the input from the live selection.

| plugin | domain | granularities used |
|---|---|---|
| puzzle 3d | `vortex` | object, reference, targetVolume, attraction, vortex |
| puzzle 5d | `vortex` | part, grip, fastener, targetVolume |
| shooting | `assets` | asset |
| cad | `cad` | object |
| procedural | `graph` | node (widget), edge (synapse) |
| note | `blocks` | block |
| forms | `fields` | field (question), section (step) |
| raster | `layers` | layer |
| draw | `strokes` | stroke (layer) |

Some ids get `ref {kind}` and **no domain**, because no selection domain covers them or the selection would carry a different id:

- shots, saved cameras, generations, responses, cad nodes, cad and puzzle references-by-model, and asset keys;
- the object-local `vortexId` and `gripId`, because the selection carries the full id `object:vortex`.

The full-id inputs `attracting`, `attracted` and the 5d fastener `source`/`target` do carry the `vortex` domain.

### 3.2 Gesture widgets

| input | widget and facets |
|---|---|
| board offsets and positions, 5d `part2d` x/y, `move-part2d` | `stepper`, step 1, precision 2, `snapSource {config: gridFactor}` (the 5d board config key, as in 2d) |
| note `drag-blocks` dx/dy, `move-block`, resize, ink bounds | `stepper`, `snapSource {snapshot: "/snapGridSpacing"}` (the document's own snap spacing) |
| 3D origins, positions, directions, extents | `vector`; `unit: m` for puzzle 3d/5d, whose world is metres (see below) |
| angles in radians: shooting `rotate-assets.angle`, draw `transform.rotation`, 5d `grip2d.angle` | `dial`, `unit rad`, `displayUnit deg`, `displayFactor 57.29…`, step 1°, soft ±π, snaps at multiples of π/2 |
| angles in degrees: 3d/5d connection rotation/turn/tilt, shooting sun azimuth and elevation, draw arc rotation | `dial`, `unit deg` |
| factors: shooting `scale-assets` sx/sy/sz, cad reference scale, camera zooms | log `slider`, soft 0.1–10, snaps 0.25/0.5/1/2/4 (zoom snaps 0.5/1/2) |
| opacities: cad, raster, draw, note grid | `slider` 0–1 with `displayUnit %`, `displayFactor 100`, snaps at quarters |
| roughness, trace threshold | 0–1 slider without percent display |
| raster adjustment `expected`/`value` | slider inside their existing −1..1 bounds |
| camera field of view (shooting, procedural preview) | slider, unit deg, soft 10–120, snaps 30/45/60/90 |
| colors: draw fills, strokes and gradient stops | `vector` with an RGBA 0..1 description |

Why the vectors get no snaps, and where the units come from:

- The reader rejects number facets on arrays, so 3D vectors carry only `step` and `unit`, never snaps.
- Puzzle 3d/5d is in metres by its own terminology ("Contact tolerance (m)") and `width_world` (`#[dsl(unit = "m")]`).
- The connection parameters gap/shift/rise are metres, and rotation/turn/tilt are degrees. This follows the 3d inspector
  labels (`rotation_deg` "Rotation (°)"), so these carry richer units than 2d's plain steppers.
- The InputUi vocabulary has no `color` widget. RGBA arrays are therefore vectors, and hex strings (handle and vortex colors,
  shot background) are `text` with a "CSS color" description.

### 3.3 Enumerations, records and discriminators

- Enums get localized `options`: cad `pane` (segmented), 3d/5d `specificity` and `anchor`, raster `parameter`, draw blend
  modes (16, with the standard German Photoshop terms), fill rule, line cap and line join.
- Several values are free strings with a closed value set in code: shot format and shape, show and LOD mode, blend mode,
  adjustment kind, `booleanOperation` and 5d `shape`. They got a `text` widget whose description lists the values, because
  adding `enum` would change the payload contract.
- Records, arrays of records and untyped values carry label, description, group and order but no widget. Examples are
  3d/5d `scale`, which is a number or a vector, forms `default`, and procedural `value`.
- Union variants have `x-semio-ui.label` on each branch, and their `kind` const is `{widget: hidden, role: discriminator}`.
  This covers forms `Expression`, draw `PathSegment` and fill, procedural widget kinds and raster layer kinds.
- The reader reads nested unions as `any`. The branch labels are ready for when it descends into them.
- `order` is the property position ×10 unless set; every input has a `group`.

### 3.4 Hard bounds (standard keywords; all checked against committed fixtures)

- **Backed by a Fatal invariant in the leaf's diff:**
  - shooting: sun elevation −90..90, sun and ambient intensity ≥ 0, roughness 0..1, scale factors > 0;
  - note: grid opacity 0..1, grid and snap spacing > 0, subdivisions ≥ 1, eraser radius > 0, pencil width > 0, resize
    width/height > 0;
  - cad: reference width > 0.
- **Physically certain:**
  - puzzle 3d reference width > 0;
  - 5d board radius, width and height > 0, mirroring 2d;
  - catalog template `t` 0..1, rank ≥ 0 and order ≥ 0, mirroring 2d;
  - note font size > 0;
  - draw radial gradient `r` ≥ 0 and trace simplification ≥ 0.
- **Deliberately soft only:**
  - draw and raster layer opacity, because their diffs accept any finite value;
  - stroke widths and font sizes elsewhere;
  - field of view and zoom.
- Existing bounds were left untouched, for example the raster sizes' `minimum: 0` and the shot size.

## 4. Tooling (ticket root, kept)

- `🧪️w2-r-design-annotate-inputs.py` holds the annotation tables and the walker.
  - Tables are keyed by catalog scope id plus payload path, or by record name (`title` or `$defs` key), plus a variant table.
  - The walker follows `$ref`s into local `$defs`, sibling record documents (`$id`) and relative files. It refuses a reachable
    field without a table entry, and a table entry that nothing reaches.
  - Standard-formatted files are rewritten with `json.dumps(indent=2)`. The 20 hand-formatted files (compact procedural,
    draw and raster leaves) get byte-span edits, so their layout survives.
  - Re-running it changes 0 documents.
  - Modes: `--report` lists missing and unused entries, `--dry-run` writes nothing.
- `🧪️w2-r-design-check-inputs.ts` runs W1-D's `mutationInputDefs` over every leaf in scope, catalogued or not, and compiles
  every leaf with the strict Ajv oracle `semioSchemaAjvV1`. It resolves refs across the plugins and the framework, and prints
  the widget census.
- `🧪️w2-r-design-check-schemas.py` uses Python `jsonschema` (Draft 7):
  - it validates every `x-semio-ui` against manifest `$defs/InputUi`;
  - it validates every committed `🦠️mutation/🔣️.json` fixture payload against its leaf, after unwrapping external tags and
    dropping the aggregate `mutation` tag;
  - `--json` writes a failure list for a before/after diff.

## 5. Verification (all run, foreground)

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under <path>` (test module), once per path of the group | **0 findings, exit 0** for all nine paths (table in §2) |
| `… --census --under <path>` | 509/509 declared (`census-after.tsv`) |
| `python3 🧪️w2-r-design-annotate-inputs.py --report` | 0 missing, 0 unused |
| `python3 🧪️w2-r-design-annotate-inputs.py --dry-run` after the write | 0 of 269 documents would change (idempotent) |
| `bun 🧪️w2-r-design-check-inputs.ts` | 262 leaves, 1,074 inputs, **0 failures** (reader + strict Ajv) |
| `.venv/bin/python 🧪️w2-r-design-check-schemas.py` before vs after | **1,133/1,133 annotations valid against `InputUi`**. Fixture failures went from 106 to 104: **0 new**, and 2 fixed by the peer's ref repair. All remaining failures are pre-existing drift (§6.1). |
| `bun test ./🧪️tests/🏷️schema-vocabulary/🟦️.ts` (os) | 2/3. The one failure is the pre-existing hub `x-semio-fixture`, unchanged. |
| schema catalog hashes of the touched files | 261 pinned, 0 stale. A peer ran `schema generate` after this write, so no catalog regeneration is owed for this group. |

I did not run cargo, as the brief requires. The puzzle 3d/5d leaf derives `include_str!` the edited schemas. Only
`x-semio-ui` and standard bound keywords changed, so no Rust source is affected, but the next build of those crates recompiles.

## 6. Findings for follow-up (not fixed: payload structure and fixtures are outside this brief)

### 6.1 Pre-existing fixture and schema drift

These are 104 validation failures over 61 payloads, all present before this pass. By plugin (failing payloads):

| plugin | failing payloads |
|---|---|
| shooting | 28 |
| procedural | 9 |
| 5d | 9 |
| cad | 6 |
| 3d | 5 |
| raster | 3 |
| note | 1 |

By cause (failures):

- **Renamed required keys (37) and snake_case payload keys (29).** Mostly shooting, whose fixtures use `new_label`,
  `asset_ids`, `to_index` and `new_intensity` against the camelCase leaves.
- **null for optional fields the schema does not make nullable (21).** For example 3d/5d `index` and 5d part fields.
- **Leaf schemas that are lossy against the real payload:**
  - The procedural widget union pins `kind` to PascalCase `"InputNote"` with no other fields. The payload is
    `{kind: "inputNote", id, text, …}`.
  - The raster `create-layer` union pins `"Pixel"`. The payload is `{kind: "pixel", id, name, …}`.
  - The cad `create-*-model` leaves omit `target`, which the Rust payload carries.
  - The 3d `change-object-anchor` enum is `Fixed`/`Derived`, while fixtures use `derived`.
  - The 5d `create-part` payload has `2d`/`3d` keys, while the schema has `part2d`/`part3d`.

  Consequence: the variant labels on those nested unions label consts that do not match the real payloads. The schemas need
  a structural fix first.

### 6.2 Other findings

- **Uncatalogued leaf.** `🖍️draw/…/🔀️transform/🧬️schema/🧬️mutations/✏️update-path-geometry/🧬️schema/🔣️.json` carries
  `$id: https://semio.tech/s/draw/update-path-geometry`. That id is outside the catalog `$id` scheme, so neither the lint nor
  the census sees the leaf. It is annotated anyway and passes the reader.
- **Snaps on 3D offsets.** Vectors cannot take `snapSource` or `snaps`, because the reader rejects number facets on arrays. A
  grid snap for 3D origins needs a reader/vocabulary extension (W1-D).
- **No color widget.** InputUi has no `color` widget, so RGBA arrays and hex colors fall back to `vector`/`text`. A `color`
  widget would be the natural W1-D/W1-E follow-up.

## 7. Files

- **Created:** the three ticket-root tools `🧪️w2-r-design-annotate-inputs.py`, `🧪️w2-r-design-check-inputs.ts` and
  `🧪️w2-r-design-check-schemas.py`, and this report.
- **Scratch** in `🗑️generated/w2-r-design/`: dumps, slot inventory, before/after census and checks, `touched-files.txt`.
- **Modified, leaf payload schemas** `…/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json` (258 leaves; full list in
  `🗑️generated/w2-r-design/touched-files.txt`):

  | plugin | leaves |
  |---|---|
  | puzzle 3d | 35 |
  | puzzle 5d | 35 |
  | shooting | 39 |
  | cad | 21 |
  | procedural | 45 |
  | note | 34 |
  | forms | 14 |
  | raster | 17 |
  | draw | 18 |

- **Modified, record documents reached by `$ref`** (annotations only):
  - `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` (`CameraJson`)
  - `…/🧊️generation3d/…/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json` (`Generation3dConfig`, `Generation3dPreviewCamera`)
  - `…/🧊️generation3d/…/✳️any/👁️viewer/🎚️config/🧬️schema/🔣️.json` (`Generation3dViewCamera`)
  - `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📝️definition/🔣️.json` (`Question`, `Step`, `Expression`)
  - `…/📋️forms/…/✳️any/🧬️schema/📨️response/🔣️.json` (`FormsResponse`)
  - `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` (`RasterLayerMask`, `RasterTransform`)
  - `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` (`DrawingLayerNode`, `StrokeStyle`, `PathSegment`)
