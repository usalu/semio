# 📓️ W2-R stdio-b — Input-UI rollout for the stdio artifacts (second half)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-R-stdio-b, 2026-09-30. Contract: `📋️design.md` §6, `🧭️plan.md`
"W2-R brief". Scope: the stdio artifacts inventory, jpg, json, las, md, mp3, mp4, obj, pdf, ply, png, pptx, semio, step, stl, svg,
tiff, tsv, txt, wav, xlsx, xml and zip under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`. Every stdio artifact is in either this list or
stdio-a's list, and there are no stdio mutation leaves outside `🗿️artifacts`. The scope covers 688 catalogued mutation leaves.

## 1. Outcome

**Done and verified.** In scope, the strict lint reports **0 findings except 9 `refUnresolved`**, which are pre-existing (see §6).
There are no `malformed` findings in scope, because no in-scope leaf has a root union.

- 1,565 `x-semio-ui` annotations were written into 301 files:
  - 271 leaf payload schemas;
  - 29 shared snapshot/definition documents of in-scope artifacts, which are the `$ref` targets whose nested fields the reader walks;
  - 1 framework document, `🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json` (`ArtifactRef`/`ArtifactDialect`, which the semio kit
    leaves reference).
- The edits only added annotations. No payload structure, bound, `required`, enum or `$ref` changed, and every file keeps its
  original byte layout apart from the inserted member.
- I did not touch the glossary, the reader, the corpus, any Rust/TS code or the schema catalog, and I did not run cargo.

## 2. Census before and after (strict lint, in-scope diagnostics)

The lint command, run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`, was
`bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/🗄️stdio" --json`, filtered to the in-scope artifacts.

| artifact | before | of which refUnresolved | after |
|---|---|---|---|
| semio | 166 (151 labelMissing, 13 optionLabelMissing) | 2 | 3 (refUnresolved) |
| pdf | 53 (49 labelMissing, 1 optionLabelMissing) | 3 | 0 (a peer fixed pdf `diff.json` in the meantime) |
| step | 22 | 0 | 0 |
| jpg | 21 | 1 | 1 |
| png | 16 | 1 | 1 |
| tiff | 14 | 1 | 1 |
| las, svg | 12 each | 0 | 0 |
| obj | 11 | 0 | 0 |
| xml | 9 | 0 | 0 |
| mp4 | 7 | 1 | 1 |
| pptx | 7 | 0 | 0 |
| wav | 7 | 2 | 1 |
| xlsx | 6 | 0 | 0 |
| ply, tsv | 5 each | 0 | 0 |
| json | 4 | 1 | 1 |
| mp3, zip | 4 each | 0 | 0 |
| stl | 3 | 0 | 0 |
| md | 2 | 0 | 0 |
| txt | 1 (optionLabelMissing) | 0 | 0 |
| inventory | 0 | 0 | 0 |
| **total** | **391** (363 labelMissing, 16 optionLabelMissing, 12 refUnresolved) | 12 | **9 (all refUnresolved)** |

**Why far more was annotated than the lint first showed.** The lint reports only the first refusal per top-level input. My deep walk
(`🧪️w2-r-stdio-b-walk.py`) found **1,611 distinct site findings**:

- 1,530 missing labels, one of which (semio kit `representations`) only showed up after a walker fix;
- 81 enums with unlabelled options;
- 794 distinct (artifact, field) names among them.

Where a label and option labels belong to the same property, they share one annotation, so these findings became 1,565 annotations.
After the rollout the walk finds 0 of either, and 9 `refUnresolved`.

**Lint per artifact after the rollout** (`--under ✏️s/…/🗿️artifacts/<artifact>`):

- jpg, json, mp4, png, tiff and wav: 1 each;
- semio: 3;
- every other in-scope artifact: 0;
- all 9 findings are `refUnresolved`.

**Whole-repo census after the rollout:** the stdio row reads `988 leaves, 1740 inputs, 1730 declared, 10 missing, 10 refUnresolved`.
Of those 10, 9 are mine and 1 is stdio-a's.

## 3. How it was done

The scripts are in the ticket root; their outputs are in `🗑️generated/w2-r-stdio-b/`.

| Script | Role |
|---|---|
| `🧪️w2-r-stdio-b-walk.py` | Walks every in-scope leaf the way the TS reader `mutationInputDefs` does. It follows `$ref`, unwraps nullable unions, and handles `allOf`, nested objects, array items and the reference/vector short-circuits. It reports **every** unlabelled input or option set with the node that must carry the annotation (`site` = file + JSON path of the property object), the resolved value shape, the context (`subset`, `$defs` record, `leaf:<kind>` or `doc:<name>`), and recursion back-edges. `--all` mode (`findings(everything=True)`) lists labelled inputs too. |
| `🧪️w2-r-stdio-b-labels.py` | The hand-written tables: en/de labels, descriptions, widget overrides, `ref.kind`, `unit`/`displayUnit`/`displayFactor`/`precision`/`step`, and option labels. Lookup order is `subset:Parent.key`, `subset:key`, `Parent.key`, `key`, then the shared `COMMON` table (XML-family and OPC fields, snapshot `schema`, patch splices). |
| `🧪️w2-r-stdio-b-annotate.py` | Writes the annotations with a span-surgical JSON editor. The original text of each file is kept byte for byte; `x-semio-ui` is inserted as the last member of the property object in that object's own layout. An inline `{ "$ref": … }` object is expanded one member per line. Prettier-clean files are re-run through Prettier. Files in `json.dumps(indent=2)` layout are asserted to still round-trip. `--revise <backup.json>` re-derives, from the current tables, every annotation this script wrote (sites without `x-semio-ui` in the pre-edit bundle) and replaces them; it was verified idempotent (0 files change on a second run). |
| `🧪️w2-r-stdio-b-check.py` | Third-party Python `jsonschema` (Draft 7 + `referencing`) checks, see §5. |
| `🧪️w2-r-stdio-b-check.ts` | Checks with the strict Ajv oracle, `parseInputUi` and the full reader, see §5. |

The pre-edit state of the 301 files is `🗑️generated/w2-r-stdio-b/backup.json` (`{path: text}`). It was captured before the first
write and is the baseline for the verdict-drift and layout checks.

**Widget derivation**, unless a table entry overrides it:

- boolean → `toggle`;
- string enum → `segmented` (at most 3 values) or `select`;
- other strings → `text`, or `multiline` for PDF contents/rich text/XMP and semio run text;
- integer → `stepper`;
- number → `slider` when bounded on both sides, else `stepper`;
- a fixed 2-to-4 number array → `vector` (PDF boxes and rects, B-rep `prange`, border radii);
- entity ids → `widget: reference`, `role: target`, `ref: {kind}`;
- objects and arrays get a label only.

**Entity references** (`role: target`, `ref.kind`):

- svg `clipPath`;
- semio B-rep `vertex`, `edge`, `coedge`, `loop`, `face` and `shell`;
- semio `style`, `image`, `layout`, `master`, `material`, `mesh`, `brep`, `spatialNode`, `piece`, `type` and `artifact`.

`ref.domain` and `granularity` were left out on purpose. No stdio editor declares a selection domain; the only domains in the code
base are the canvas `vortex` and `tree`, and claiming either for stdio would be false.

**Suppressed false inference.** Some id-shaped string fields are free text in their format, not references: STEP
`definitionId`/`formationId` (the `id` attribute of the product definition entities) and LAS VLR `userId` (a registered owner such as
`LASF_Projection`). Without an explicit widget the reader would infer `<kind>Id` references for them. They now carry
`widget: text`, which stops the reader's `<kind>Id` inference.

**Bounds and snaps.** No hard bounds were added: every one I considered was either already in the schema (the integer widths) or not
certain for the stored snapshot. Keeping the schemas unchanged is also what makes the fixture verdicts provably identical. No snaps
were added: none of these format fields has a meaningful snap grid.

**Units and scaling** (each checked against the Rust or the format specification):

- WAV `Hz`/`B`/`B/s`/`bit`; LAS `s`/`B`/`deg`; mp4 times in `s` since 1904, `1/s` timescale, `bit/s`; PDF boxes and rects in `pt`,
  page rotation in `deg` with step 90, `Dur` in `s`, `UTC offset` in `min`; semio keyframe `t` in `s` (documented as float seconds
  in the animation serializers), frame delay in `ms`, arc `xRotation` in `deg` (imported unchanged from SVG); pptx `cx`/`cy` in EMU
  shown in cm.
- Fixed-point display factors: mp4 16.16 rate and resolution (`displayUnit dpi`), mp4 8.8 volume, PNG gAMA and cHRM × 100000.
- Two units were removed after checking the code: pptx `fontSize` is stored in **whole points** (the importer divides `sz` by
  100), and semio run `size` has no documented unit.

## 4. Domain terminology (German)

| Format | Terms used |
|---|---|
| PDF (Acrobat DE / ISO 32000) | Medienrahmen (MediaBox), Maskenrahmen (CropBox), Anschnittrahmen (BleedBox), Endformatrahmen (TrimBox), Objektrahmen (ArtBox); Anmerkung, Lesezeichen, Formularfeld/Interaktives Formular (AcroForm), QuickInfo (TU), Exportname (TM); Kennwort zum Öffnen des Dokuments / Berechtigungskennwort; Füllmethode, Linienende (Abgeschnitten/Rund/Überstehend quadratisch), Linienverbindung (Gehrung/Rund/Abgeflacht), Gehrungsgrenze, Flachheits- und Glättungstoleranz, Überdrucken, Aussparungsgruppe, Weiche Maske; Seitenlayout and Seitenmodus options (Einzelne Seite, Fortlaufend, Doppelseite …; Lesezeichenfenster, Seitenminiaturen, Ebenenfenster, Anlagenfenster); Output Intents (Ausgabebedingungen); Getaggtes PDF; Stichwörter, Thema, „Erstellt mit“ / „PDF erstellt mit“, Überfüllt |
| STEP (ISO 10303-21/-42) | Header-Abschnitt, Dateibeschreibung (FILE_DESCRIPTION), Dateiname (FILE_NAME), Dateischema (FILE_SCHEMA), Implementierungsebene, Freigabe, Ursprungssystem, Präprozessorversion, Entitätsinstanzen, Teilentitätsinstanzen, Produktdefinition, Produktversion (PRODUCT_DEFINITION_FORMATION), Darstellungskontext, Darstellungselemente, Entitätstyp |
| SVG / XML | Ansichtsrahmen (viewBox), Beschneidungspfad, Basisprofil, XML-Deklaration, Zeichenkodierung, Standalone-Dokument, Markup-Deklarationen, Externer Bezeichner, Parameter-Entity, Prolog/Epilog, Wurzelelement |
| XLSX / PPTX (OPC, DrawingML) | OPC-Paket, Paketteile, Inhaltstypen, Standard-Inhaltstypen, Inhaltstyp-Überschreibungen, Beziehungen, Arbeitsblatt-Teil, SpreadsheetML-Namensraum (Transitional / Strict), Textkörper, Textläufe, Folie, Form |
| Other formats | JPEG: DC/AC-Koeffizienten, Abtastfaktor, Quantisierungstabelle, Restart-Intervall, SOF-Marker. PNG: Farbtyp, Rendering Intent (Wahrnehmungsorientiert, Relativ/Absolut farbmetrisch, Sättigung), Chromatizität, Weißpunkt. TIFF: Bilddateiverzeichnis (IFD), Photometrische Interpretation, field types BYTE … DOUBLE. LAS: Echo/Echonummer, Flugstreifenrand, Datensätze variabler Länge (VLRs). mp4: Spur, Zeitskala, Schnittliste (Edit List), Kompositionszeitversatz, Sequenz- and Bildparametersätze. semio: Koante, Schleife, Schale, Hohlraumschale, Formatvorlage/Absatzformat, Folienmaster/Folienlayout, Grundstück/Gebäude/Geschoss/Raum, Wand/Decke/Stütze/Träger |

## 5. Verification (all foreground; no cargo)

| Check | Command | Result |
|---|---|---|
| Strict lint, whole stdio | `bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/🗄️stdio" --json` | in scope: 9, all `refUnresolved` |
| Strict lint, per in-scope artifact | the same, with `--under …/🗿️artifacts/<artifact>`, 23 runs | jpg, json, mp4, png, tiff, wav 1 each; semio 3; the rest 0; all `refUnresolved` |
| Deep walk | `python3 🧪️w2-r-stdio-b-walk.py` | 0 labelMissing, 0 optionLabelMissing, 9 refUnresolved |
| Reader, Ajv and InputUi (TS) | `bun 🧪️w2-r-stdio-b-check.ts` | 688 leaves, 1,102 inputs read in **one whole-payload read per leaf**, 0 failures, 9 `refUnresolved`, 1,056 leaf annotations parse as `InputUi` |
| Strict Ajv compile | same TS check | Every leaf that compiled with all `x-semio-ui` stripped still compiles annotated. 23 leaves already failed without the annotations: unresolvable `snapshot-patch`/`os/store/*` refs, strict `allowUnionTypes` in semio value, and the pdf `#/$defs/T` refs. |
| Meta-schema (Python `jsonschema`) | `.venv/bin/python 🧪️w2-r-stdio-b-check.py 🗑️generated/w2-r-stdio-b/backup.json` | all **1,565** annotations in the 301 edited files are valid against manifest `$defs/InputUi`: 0 invalid |
| Fixture payloads (Python `jsonschema`) | same | 281 committed payloads of in-scope leaves were checked: `🦠️mutation` files, `{mutation, payload}` envelopes, externally tagged `{Kind: {…}}` objects, and objects tagged with the leaf const. **Verdict drift: 0.** 10 are invalid both before and after (§6). |
| Layout | same | 301 files: `json.dumps` files still round-trip and Prettier-clean files are still Prettier-clean. Layout drift: 0. |
| Recursion (TS reader) | probe in `🗑️generated/w2-r-stdio-b/recursion-probe.ts` | Before W1-D's cycle guard: `RangeError: Maximum call stack size exceeded` on a fully labelled recursive schema. After it: the reader returns normally. |

The Python script exits 1 because of the 10 fixtures that were already invalid before any annotation; it reports no drift.

## 6. Findings outside this work package

1. **Reader recursion hazard, reported to the coordinator and fixed by W1-D.** A fully labelled recursive leaf used to overflow the
   reader's stack. The recursive leaves are pdf `set-outlines` and `set-acro-form`, semio value ×5 and semio drawing ×4. The lint
   survived only because the first unlabelled nested field threw first. W1-D landed a cycle guard, and every back-edge
   (`PdfOutlineItem.children`, `PdfFormField.children`, `PdfAction.next`, `DrawNode.children`, `SemioValue.items`/`entries`) is now
   labelled. All 11 leaves read without crashing.
2. **9 `refUnresolved`, left to the W1-D follow-up.**
   - Six `patch-snapshot` leaves (jpg, json, mp4, png, tiff, wav) `$ref` `…/s/stdio/snapshot-patch/schema.json`, which no catalogued
     document declares.
   - semio kit `bind-representation`, `change-representation-pin` and `set-snapshot` (`representations` items) `$ref`
     `…/os/store/link.json`, which is not catalogued.
3. **10 committed fixtures that do not match their leaf schema.** They were invalid before this work too; I changed no payload
   structure.
   - semio cad `🧪️tests/📐️mutate-semio-cad/*` ×7: snake_case keys `block_name`, `base_point`, `color_index`, and an `arc` entity
     with `start_angle`/`end_angle`.
   - semio document `📃️mutate-semio-document/*` ×2: block path kinds `listItem` and `quote`, which the schema does not know.
   - zip `set-snapshot` ×1: `commentUtf8` inside the snapshot, which `ZipSnapshot` does not declare.
4. **Schema catalog hashes.** `📚️library/🔣️schema-catalog.json` records sha256s of the 301 edited files and is now stale for them. I
   did not regenerate it, to avoid racing the parallel W2-R groups: run `schema generate` once after the W2-R wave. The lint reads
   the files directly, so its result does not depend on this.
5. **Framework document edited.** `🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json` gained `x-semio-ui` on the 5 `ArtifactRef` and
   `ArtifactDialect` fields; nothing else in it changed. These fields are nested inside in-scope semio kit leaves and could not be
   labelled anywhere else.

## 7. Files

**Scripts (ticket root):**

- `🧪️w2-r-stdio-b-walk.py`
- `🧪️w2-r-stdio-b-labels.py`
- `🧪️w2-r-stdio-b-annotate.py`
- `🧪️w2-r-stdio-b-check.py`
- `🧪️w2-r-stdio-b-check.ts`

**Leaf payload schemas (271):** `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>/…/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json`. The
count per artifact:

- semio 89, pdf 40, step 22, jpg 15, png 12;
- svg 11, obj 11, las 10, tiff 10, xml 8, pptx 7, mp4 6;
- zip 4, mp3 4, tsv 4, wav 4, ply 4, stl 3, json 3, md 2, xlsx 1, txt 1.

The full list is the key set of `🗑️generated/w2-r-stdio-b/backup.json` and `🗑️generated/w2-r-stdio-b/edited-files.txt`.

**Shared documents (30).** All but the last are under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`:

- `🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json`
- `📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json`
- `📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🧭️cell-address/🔣️.json`
- `📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json`
- `📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔣️.json`
- `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json`
- `📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔣️.json`
- `📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🔣️.json`
- `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🔣️.json`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🧮️geometry/🔣️.json`
- `🧿️semio/🏅️standards/🔖️v1/🪆️subsets/<subset>/🧬️schema/📸️snapshot/🔣️.json`, for 18 subsets: flow, animation, video, model, table,
  cad, document, object, presentation, audio, value, text, mesh, graph, drawing, image, brep, kit
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json`

**Generated scratch (`🗑️generated/w2-r-stdio-b/`):**

- lint and census before/after;
- walk before/after;
- `sites/` and `keys/` listings;
- `review.tsv`, `check.txt`, `check-ts.txt`, `lint-per-artifact.txt`;
- `backup.json`, `edited-files.txt`, `recursion-probe.ts`.
