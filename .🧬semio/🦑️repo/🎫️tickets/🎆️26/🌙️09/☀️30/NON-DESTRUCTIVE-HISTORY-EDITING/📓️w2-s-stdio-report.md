# 📓️ W2-S-B2 — stdio (non-glTF) schema/payload parity (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-S-B2, 2026-09-30.

- **Contract:** `🧭️plan.md` "W2-S parity brief", plus the approved repo-wide rule: the leaf root is the Rust `payload_value()` wire
  in camelCase, Rust is fixed when it is the outlier, and aggregates take the internal, external or adjacent form.
- **Scope:** every artifact under `✏️s/🔌️plugins/🗄️stdio/` except `🧊️gltf`.

## 1. Outcome

| lint, `--under ✏️s/🔌️plugins/🗄️stdio`, non-glTF | before (05:27) | after (08:40) |
|---|---|---|
| `schema mutation-payloads`, classes of the brief (aggregate, layout, invalid fixtures, opaque, undescribed fixtures, unmapped fixtures, unresolved) | 112 | **0** |
| `schema mutation-inputs` | 10 `refUnresolved` | **0**, over all 1,658 inputs of all 988 stdio leaves |
| strict Ajv oracle (`🧪️w2-s-stdio-check.ts`) | 19 leaves did not compile, and aggregates were not checked | 867 leaves and 87 aggregate documents compile; 0 failures |
| committed stdio fixtures that match their leaf schema | 266 / 358 | 477 / 477 |

The findings the lint reports now come only from its two newer classes, F9 (feature rows) and `unwitnessed`. W2-S added both
while this WP ran, and they belong to the separate F10 conversion (§5.1).

**Rust:**

- The gated `cargo check` of all 10 changed crates is green.
- The lib tests pass: semio 2591, avi 37, bcf 38, dwg 71, dxf 36, html 43, ply 46, step 148, svg 94 and xml 69.
- The harness cases pass in parity against their independent Python oracles:

  | case | result |
  |---|---|
  | animation | 80/80 |
  | video | 56/56 |
  | audio | 62/62 |
  | cad | 98/98 |
  | model | 68/68 |
  | document | 110/110 |
  | flow | 80/80 |
  | avi `mutate-avi-1-0` | 15/15 (subject only) |

  presentation fails 2 rows, which predate this work (§5.3).

## 2. What was wrong and what changed

### 2.1 Aggregate forms

- **Coordinator rule.** I ran `🧪️w2-s-aggregate-rule.py --apply` over the 86 non-glTF stdio aggregates. It rewrote 546 leaves and
  85 aggregate documents; the list is in `🗑️generated/w2s-stdio/rule-changed.txt`.
  - Internally tagged leaves now declare `mutation: {const}` first in `properties` and first in `required`, and their aggregate is
    the `oneOf` of leaf `$ref`s.
  - The external and adjacent aggregates got the wrapped branch form.
  - This closed bcf, csv and mp3 `set-snapshot` (the missing `mutation` discriminator) and the bare `oneOf` of png, jpg and tiff.
- **Manual items the rule reported:**
  - The 4 `patch-snapshot` leaves (jpg, tiff, png, json) declared the adjacent tag inside the payload. I removed it.
  - The 18 semio base `apply-*` leaves are a false positive. Their own field is literally named `mutation`, so they stay as they
    are.
- **dwg ac1018** re-exports the AC1024 vocabulary (`pub use`), but its aggregate document pointed at ac1018 leaf `$id`s that do not
  exist. It is now `allOf` → the ac1024 aggregate (the coordinator's pdf/dwg note). The pdf aggregates already compiled at the time.
- **svg, xml and json wrapped leaves.** W1-D marked 20 `phase`/`value` leaves `payload = Apply`. Script `🧪️w2-s-stdio-wrapped.py`
  followed the glTF precedent:
  - each wrapped leaf keeps its flat root (the `Apply` content) and gains `$defs/<LeafEnum>`, which is
    `{phase: apply, value: {$ref: "#"}} | {phase: restore, value: <artifact diff.json>}`;
  - the adjacent aggregate branch references `<leaf $id>#/$defs/<LeafEnum>`, which is the union the lint derives from Rust.
- The svg `diff.json` pointed at the xml doctype through a relative file path, which does not resolve. It now uses the `$id`.

### 2.2 Rust was the outlier — `rename_all_fields = "camelCase"` on 15 enums

`#[value(rename_all)]` on an enum only renames the variant tags. Named variant fields stayed snake_case on the wire, while every
schema, TS twin, graphql file and feature spelled them camelCase. The attribute follows the brep precedent. Enums changed:

| artifact | enums |
|---|---|
| bcf | `BcfCamera` |
| avi | `AviStreamFormat` |
| dxf | `DxfEntity` |
| svg | `PathCommand`, `SvgElement` |
| semio drawing | `PathSegment` |
| semio cad | `CadEntity` |
| semio document | `DocPathSegment` |
| semio model | `GeometryRef` |
| html | `HtmlNode`, `HtmlNodeDiff` |
| step | `StepValue` |
| xml | `XmlExternalId` |
| dwg | `DwgXRecordValue` |
| ply | `PlyProperty` |

Two more Rust outliers were fixed the same way:

- **Audio format.** `SemioAudioFormat::Float32/Float64` emitted `float32`/`float64`, while the grammar, TS twin, snapshot schema and
  oracle all say `f32`/`f64`. It now uses `#[value(rename = "f32"/"f64")]`. The audio `set-format` and base `set-snapshot` schemas
  had copied the Rust spelling, so they were corrected.
- **bcf `set-comment`.** The field `viewpoint_ref: Option<Option<String>>` broke the new derive payload law, because a present `null`
  decoded as "untouched". It now uses `deserialize_with = "deserialize_double_option"`, following the semio model precedent.

The snake_case consumers were renamed with `🧪️w2-s-stdio-casing.py`:

- the avi hdrl vectors;
- every `mutate-semio-cad` vector, plus the cad feature and oracle. This includes the leaf-struct keys `blockName`, `basePoint` and
  `colorIndex`; those structs were already camelCase in Rust;
- the model vectors, the base `apply-model-applied`, and the model feature, oracle and Rust adapter;
- the drawing vectors and oracle;
- the document vectors, feature and oracle. This `blockIndex` casing was the brief's "document block kinds missing".

A scan of the whole repository finds nothing else that depends on the old keys.

### 2.3 `noMutation` sentinel fixtures (10 unmapped)

These are not mutations, so they left the mutation fixture tree:

- **The 8 spec vectors that have a before-snapshot** (flow, cad, document, animation, video, audio, model `⏸️`, presentation `⏸️`)
  now carry the real identity mutation `{"mutation":"setSnapshot","snapshot":<before>}`. I removed the cad and document adapters'
  `kind == "no-mutation"` special case.
- **The 2 payload-only sentinels** (presentation `🪞️no-mutation` and image `⏸️no-mutation`) were deleted. Their baseline scenarios
  now carry `{"mutation": "noMutation"}` as a doc string, which the Rust and Python adapters read when the scenario names no
  payload fixture.

### 2.4 `{kind, params}` vectors (29 layout)

Script `🧪️w2-s-stdio-vectors.py` rewrote the animation, video and audio vectors and feature rows to the aggregate wire value
`{"mutation":"<camelCaseVariant>", …}`. This is the form the cad, flow, model, document, presentation and image features already
use.

- For video, the wire uses stream kind names and byte arrays. The case projection keeps the grammar letters and hex.
- **Production:** new `decode_semio_{animation,video,audio}_mutation_json` bridges.
- **Rust adapters:** they decode through those bridges, and the hand-written decoders are gone.
- **Python oracles:** `parts()` reads the wire and `wire()` emits it; video converts at that boundary.
- **Tests:** the audio and video `demo_mutation_cases` were `cfg(all(test, feature = …))`. That broke the derive's payload-law test,
  so they are now `cfg(test)`, like every sibling.

### 2.5 Opaque, restated and stale snapshot contracts (`🧪️w2-s-stdio-snapshots.py`)

- **xml, svg and pdf 1.7 `artifact.json`** restated part of the snapshot, with opaque `doc`/`info`. They now use the zip form
  (`allOf` → `snapshot.json`), and their `x-semio-ui` moved onto `snapshot.json`.
- **The xml and pdf `set-snapshot` leaves** reference `snapshot.json`.
- **svg `doc`** is `$ref` → xml `XmlDocument`.
- **zip base and iso21320 `set-snapshot`** inlined a stale `ZipSnapshot` without `metadata` or `commentUtf8`. They now reference the
  base `snapshot.json`.
- **semio table `cells.items`** → `$ref` → `SemioValue`.
- **semio value `SemioValue`** was a loose object that typed `value` as `["boolean","string"]`. It dropped `bytes` and failed strict
  Ajv. It is now the precise `kind`-tagged `oneOf` of the Rust enum, and every label is kept.
- **pdf 1.7 `diff.json`** had generic `PdfIndexedItemT`/`PdfKeyedItemT` that pointed at the nonexistent `snapshot.json#/$defs/T`.
  They were monomorphised into 24 concrete definitions.

### 2.6 Labels (design §6)

Written with `🧪️w2-s-stdio-labels.py`, for the members that the re-pointed leaves now reach:

- the 24 `PdfSnapshot` catalog members;
- the ZIP header metadata;
- `commentUtf8`.

I also added a `pin` label in `os/store/link/schema.json`. This is a W1-D/W1-G file; the edit adds one member and nothing else. It
labels the kit `representations` that W1-D's link split made reachable.

## 3. Verification (all run, results seen)

| check | result |
|---|---|
| `bun ./📜️script.ts schema mutation-payloads --under ✏️s/🔌️plugins/🗄️stdio --json` | non-glTF, brief classes: 0. F9/F10 classes: see §5.1. |
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/🗄️stdio` | 1658/1658 inputs, 0 findings, exit 0 |
| `bun 🧪️w2-s-stdio-check.ts` | 867 leaves and 87 aggregates compile under strict Ajv. 2,289 leaf and 2,930 document `x-semio-ui` annotations parse as `InputUi`. The reader reads 1,430 inputs. 0 failures. |
| `cargo check -p semio-s-artifact-stdio-{semio,bcf,avi,dxf,svg,html,step,xml,dwg,ply}` | ok. The warnings are all in peers' files. |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2s-stdio cargo test --no-fail-fast -p <9 crates> --lib` | avi 37, bcf 38, dwg 71, dxf 36, html 43, ply 46, step 148, svg 94, xml 69: all pass |
| same, for `-p semio-s-artifact-stdio-semio --lib` | 2591 passed, 0 failed. This includes every derive-emitted `semio_payload_law_*` test. |
| `bun ./📜️script.ts parity exhaustive --case <case>` | animation 80/80, video 56/56, audio 62/62, cad 98/98, model 68/68, document 110/110, flow 80/80; presentation fails 2 (§5.3) |
| `bun ./📜️script.ts subject exhaustive --case 🎛️mutate-avi-1-0` | 15/15 |
| `bun ./📜️script.ts oracle exhaustive --case …` (Python only) | presentation 46/46 as well |

## 4. Rust files changed

- **Enum attributes:** the 15 enums of §2.2.
- **Audio:** `SemioAudioFormat`.
- **bcf:** `set-comment`, plus the `deserialize_double_option` helper in the bcf mutations module.
- **Production decoders:** `decode_semio_{animation,video,audio}_mutation_json`.
- **Demo cases:** the audio and video `demo_mutation_cases` cfg.
- **Case adapters (Rust and Python):** animation, video, audio, cad, document, model, image and presentation.
- **brep tessellation unit test:** the peer change `HashMap`→`BTreeMap` of 04:00 had left it uncompilable. I made the one-line fix
  there so the semio tests could run.

## 5. Open items

1. **F9/F10 (W2-S follow-up 3), not part of this brief.**
   - **What the lint counts:** stdio `{"kind","params"}` feature rows, and leaves that no wire value witnesses.
   - **Non-glTF numbers now:** 469 `invalid` and 90 `undescribed` rows (hand-mapped `params`), 106 `unmapped` rows
     (no-mutation / identity-round-trip), and 98 `unwitnessed` leaves.
   - **Where they are:** 34 artifacts, table in `🗑️generated/w2s-stdio/f10-inventory.tsv`.
     - Rows are concentrated in ifc (145), step (42), svg (32), json (30) and pdf (26).
     - `unwitnessed` is mostly pdf 1.7 base (45) and semio (32: base `apply-*`, value, and 8 subset `set-snapshot`s).
   - **Converting them per the F10 recipe** means rewriting rows to the Rust wire and adapters to `from_payload_value`, which W1-D
     now provides. It also needs wire fixtures for the 98 leaves. That is ~34 artifacts of Rust adapters plus oracles, and it needs
     its own split.
   - **Form note:** semio features, now including animation, video and audio, use aggregate-wire docstrings
     (`{"mutation":…}`), which F9 does not read. Their `{kind, mutation, before, after}` vectors already witness every leaf.
     `{kind, params}` for semio is a coordinator decision.
2. **A staged repo-wide rename sweep (988 index `R`s, not mine) shortened spec-vector slugs.** The drawing and image `🥒️.feature`
   files still name the old slugs, so `mutate-semio-drawing` and `mutate-semio-image` stop at fixture resolution. My image and
   presentation adapter edits are the same pattern, and presentation compiled and passed. The sweep owner has to update the URIs.
3. **presentation `mutate-set-snapshot` / `inverse-set-snapshot` parity: 6 differences.**
   - Rust keeps the base slide order; the Python oracle takes the snapshot's order.
   - The keyed slide diff cannot express a reorder.
   - This predates my work: that row's fixture, the Rust code and the Python code are untouched.
4. **Catalog.** `schema --under stdio` shows the same pre-existing contract families as before, plus stale-catalog entries for files
   edited by W2-R, W2-S and W1-D (e.g. pdf `diff.json` exports). Run `schema generate` once, centrally.
5. **Not drift, so left alone:**
   - 123 non-glTF leaves with snake_case root keys that are consistent everywhere, e.g. bcf `topic_guid`, presentation
     `slide_index` and document `DocBlock.style_id`;
   - 55 stdio `artifact.json` that still restate their snapshot. No leaf reaches them, so the lint cannot see them. Latent examples:
     mp4 `ftyp`, avi `mainHeader`, pptx `opc`, dxf `tables`, semio value `root`.

## 6. Files

- **Ticket scripts (kept):**
  - `🧪️w2-s-stdio-casing.py`
  - `🧪️w2-s-stdio-vectors.py`
  - `🧪️w2-s-stdio-snapshots.py`
  - `🧪️w2-s-stdio-labels.py`
  - `🧪️w2-s-stdio-wrapped.py`
  - `🧪️w2-s-stdio-check.ts`
- **Schemas and fixtures:**
  - `🗑️generated/w2s-stdio/rule-changed.txt` (631 files);
  - the files each script prints;
  - xml, svg and pdf `artifact.json` and `snapshot.json`;
  - pdf and svg `diff.json`;
  - value and table `snapshot.json`;
  - zip `snapshot.json` and 2 zip leaves;
  - dwg ac1018 `mutations.json`;
  - the audio `set-format` and base `set-snapshot` leaves;
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🔣️.json` (the `pin` label).
- **Rust:** see §4.
- **Deleted:**
  - presentation `🧫️fixtures/📽️mutate-semio-presentation/🪞️no-mutation/`;
  - image `🧫️fixtures/🖼️mutate-semio-image/⏸️no-mutation/`.
- **Scratch:** `🗑️generated/w2s-stdio/`.
