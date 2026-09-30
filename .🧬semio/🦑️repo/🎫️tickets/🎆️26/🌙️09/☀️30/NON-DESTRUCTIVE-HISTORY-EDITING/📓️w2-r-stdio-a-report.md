# 📓️ W2-R stdio-a — `x-semio-ui` rollout for the first stdio group (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-R-stdio-a, 2026-09-30. Contract: `📋️design.md` §6, the W2-R brief in
`🧭️plan.md`, manifest `$defs/InputUi`. Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{avi, bcf, binary, bmp, commands, contract, csv,
deflate, docx, dwg, dxf, epw, gif, gltf, graph, html, ifc}`.

## 1. Outcome

**Done and verified.** Every mutation input in scope now resolves to a UI descriptor. The strict lint shows **0 findings** in every
artifact except one allowed `refUnresolved`: csv `patch-snapshot` `/patch`, whose target `stdio/snapshot-patch/schema.json` is in no
catalogued scope.

- `commands`, `contract` and `graph` have no catalogued mutation leaves, so there was nothing to annotate there.
- This covers 300 leaves and 626 top-level inputs. The count includes the fields of the 51 glTF root unions, which the reader now reads.
- In total there are 14,730 `x-semio-ui` annotations:

| Annotation | Count | What it carries |
|---|---|---|
| Top-level inputs | 545 | widget, label, description, role/ref, unit, precision, group, order |
| Nested record fields | 13,618 | label, plus a description where the name is not self-explanatory |
| Enum option tables | 312 | option labels |
| Root-union variant branches | 102 | variant label (`apply` → Anwenden, `restore` → Wiederherstellen) |
| Root-union discriminators | 102 | `phase` as `role: discriminator` |
| glTF `restore` payloads | 51 | `widget: hidden` |

- Following the coordinator's note about the new `$ref` cycle guard, the recursive IFC `Part21Value` (`values: Part21Value[]`) is now
  fully labelled, including its `kind` options. It reads cleanly and produces no `readerFault`.

## 2. Census before / after

Commands, run from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

- before: `bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>" --json`
- after: the same command in strict mode

"Before" was captured at 04:35 with the reader of that time, which refused root unions as `malformed`. The reader in use now reads
them, so the 51 glTF unions contributed 102 more inputs.

| artifact | leaves | inputs before | declared before | findings before | inputs after | declared after | findings after |
|---|---|---|---|---|---|---|---|
| 📼️avi | 12 | 21 | 12 | labelMissing 9 | 21 | 21 | 0 |
| 💬️bcf | 13 | 35 | 23 | labelMissing 12 | 35 | 35 | 0 |
| 💾️binary | 4 | 6 | 3 | labelMissing 3 | 6 | 6 | 0 |
| 🪟️bmp | 6 | 19 | 6 | labelMissing 13 | 19 | 19 | 0 |
| 🏃️commands | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🛂️contract | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 📊️csv | 6 | 10 | 3 | labelMissing 6, refUnresolved 1 | 10 | 9 | refUnresolved 1 |
| 🗜️deflate | 4 | 6 | 0 | labelMissing 6 | 6 | 6 | 0 |
| 📜️docx | 32 | 51 | 28 | labelMissing 23 | 51 | 51 | 0 |
| 🖊️dwg | 2 | 4 | 1 | labelMissing 3 | 4 | 4 | 0 |
| 🖋️dxf | 18 | 29 | 19 | labelMissing 10 | 29 | 29 | 0 |
| 🌦️epw | 12 | 15 | 9 | labelMissing 6 | 15 | 15 | 0 |
| 🎞️gif | 31 | 55 | 41 | labelMissing 14 | 55 | 55 | 0 |
| 🧊️gltf | 121 | 208 | 145 | labelMissing 12, malformed 51 | 310 | 310 | 0 |
| 🕸️graph | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🌐️html | 9 | 18 | 17 | labelMissing 1 | 18 | 18 | 0 |
| 🏗️ifc | 30 | 47 | 28 | labelMissing 19 | 47 | 47 | 0 |
| **total** | 300 | 524 | 335 | labelMissing 137, malformed 51, refUnresolved 1 | 626 | 625 | refUnresolved 1 |

The lint reports only the first refusal per top-level input. The real number of unlabelled nested fields was much larger: 16,408 input
occurrences, at 11,242 distinct schema nodes for glTF alone.

## 3. Method

1. `🧪️w2-r-stdio-a-walk.py` is a Python mirror of `mutationInputDefs` (`🛂️manifest/🟦️.ts`). It covers:
   - the `$ref` chain and `x-semio-ui` merge;
   - nullable unwrap and `allOf`;
   - root-union variants;
   - recursion into objects and array items, with a cycle guard.

   It enumerates every input, the node that carries its annotation, and its enum node.
2. `🧪️w2-r-stdio-a-annotate.py` holds the domain label tables, with en and de labels, descriptions and options. Lookup order is:
   leaf-specific `leaf:field`, then `$defs`-record `Record.field`, then the parent collection `^parent.field`, then enum-specific
   `field#value`, then the field name, then common diff terms. It writes:
   - labels and widgets on the property node;
   - option labels on the enum node;
   - variant labels on the union branch;
   - `role: discriminator` on `phase`.

   For top-level inputs, including a root union's `apply` payload fields, it also sets:
   - the widget: toggle, stepper with `precision 0` for integers, select or segmented, text or multiline, or reference;
   - `role: target` with `ref {kind, domain}` for string entity ids:
     - bcf: topic, comment, viewpoint;
     - docx: style, part;
     - dxf: layer, textStyle, linetype, headerVariable;
   - units (`px`, `B`, `px/m`, `bit`, and `cs` shown as `s` via `displayFactor 0.01`);
   - `group` (`target` or `value`) and `order`.

   A nested string whose name would be *inferred* as a reference, such as `installId` or `localeId`, gets `widget: text` so that it is
   not misread as a reference. glTF indices are integers, so they are steppers with an "Index of the …" description, not reference
   pickers. The script refuses to write if any input would stay unlabelled, or if two leaves disagree on one shared node.
3. Formatting:
   - Files that round-trip byte-identically through `json.dumps(indent=2)` were rewritten that way; the diffs only add lines.
   - The two prettier-style files were edited by hand in their own compact style: docx `🧭️xml-address/🔣️.json` and bmp
     `📸️set-snapshot/🧬️schema/🔣️.json`.
   - A re-run of the script writes 0 files, so it is idempotent.
4. Shared documents in scope were annotated once:
   - the ifc 2x3 `artifact.json` (Part21 records) and `snapshot.json`;
   - the docx `snapshot.json` and `xml-address.json`;
   - the glTF `snapshot.json`;
   - the bmp `snapshot.json`.

   The xml artifact's `snapshot.json`, which docx references, belongs to stdio-b and was already annotated there. It was not touched
   (`--shared` would write it).

Terminology follows each format's own vocabulary:

| Format | German terms used |
|---|---|
| DWG | AutoCAD system-variable names in brackets, e.g. "Pfeilgröße (DIMASZ)", "Überstand der Maßhilfslinie (DIMEXE)", "BKS-Ursprung (UCSORG)", "Limiten Minimum (LIMMIN)", "Referenz (Handle)" |
| DXF | Layer, Linientyp, Textstil, Systemvariable, Gruppencode, Objekt |
| IFC | Entität, Entitätstyp, Attributwert, STEP-Kopfteil, Geschoss, Höhenkote, Eigentümerhistorie, Lastgruppe, Einwirkungsart / Einwirkungsursache (EN 1990), Berechnungsmodell, Modellansichtsdefinition (MVD) |
| BCF | Thema, Kommentar, Ansichtspunkt, Bauteile, Einfärbung |
| glTF | Netz, Primitiv, Morph-Ziel, Accessor, Pufferansicht, Alphamodus, Metallizitätsfaktor, Rauheitsfaktor, Okklusionsstärke, Texturkoordinatensatz |
| EPW (EnergyPlus / meteorology) | Trockenkugeltemperatur, Taupunkttemperatur, Globalstrahlung (horizontal), Direktnormalstrahlung, Atmosphärische Gegenstrahlung, Gesamtbedeckungsgrad, Wolkenuntergrenze |
| OOXML | Paketteil, Formatvorlage, Absatzformatvorlage, Inhaltstyp, Knotenadresse |
| RIFF/AVI | Datenstrom, Chunk, Schlüsselbild |
| zlib | Kompressionsverfahren (CM), Fenstergröße, Kompressionsstufe (FLEVEL), Vorgabewörterbuch (DICTID) |

Where the framework glossary is wrong for a domain, a local label overrides it:

| Field | Glossary label | Local label |
|---|---|---|
| glTF `scale` | "Maßstab" | "Skalierung" |
| DWG `handle` | "Anschluss" | "Referenz (Handle)" |
| GIF `frame` | "Rahmen" | "Einzelbild" |
| glTF `before` | "Vor" | "Name vorher" |

The glossary itself was not edited.

## 4. Verification (all run, results seen)

| Check | Result |
|---|---|
| Strict lint per artifact: `bun ./📜️script.ts schema mutation-inputs --under ✏️s/…/<artifact>` | 0 findings for all 16 artifacts; csv has 1 `refUnresolved` (allowed) |
| `.venv/bin/python 🧪️w2-r-stdio-a-check-fixtures.py`: Python `jsonschema` 4.26, Draft7 with a `referencing` registry, each fixture validated against the annotated schema **and** against the git-HEAD schema | 140 committed fixture payloads: 121 valid, 18 invalid, 1 without a leaf schema. **All 139 verdicts are identical to HEAD**, so the rollout changed no validation outcome. The 18 invalid ones are pre-existing (§5). |
| Same script: every `x-semio-ui` validated against manifest `$defs/InputUi` | 14,730 annotations, 0 violations |
| `bun 🧪️w2-r-stdio-a-check-inputs.ts`: strict Ajv oracle `semioSchemaAjvV1` with `x-semio-ui` registered | Every leaf compiled with and without `x-semio-ui`. 0 new failures; 1 pre-existing (csv unresolved `$ref`). glTF only compiles once the undeclared legacy keywords `x-semio`/`x-semio-mutation` are registered (§5.6). |
| Same script: `parseInputUi` on every annotation | 14,730 parsed, 0 faults |
| Same script: `mutationInputDefs` on every leaf | 625 inputs; 1 fault (the csv `refUnresolved`) |
| `bun test 🛍️products/💻️os/🧪️tests/🏷️schema-vocabulary/🟦️.ts` | 2 of 3 pass. The failure is the pre-existing hub `x-semio-fixture`; this test does not scan `✏️s`. |

No cargo was run, as the brief requires. The Rust derive `include_str!`s these files, so the next build picks up the new schemas.

## 5. Schema problems found (pre-existing, not fixed: the brief forbids payload-structure edits)

1. **69 of 121 glTF leaf schemas describe the flat `apply` payload**, while the Rust leaf is
   `#[value(tag = "phase", content = "value")] enum {Apply(..), Restore(Box<GltfDiff>)}`. Examples: `node/transform`, `scene-root/*`,
   `primitive/*`, `asset/*`. Their wire value `{phase, value}` does not validate against their schema, so
   `mutation_input_instance`/`with_payload_value` round trips will be refused for them. The other 52 are root unions with the correct
   shape.
2. **The glTF `material/change-alpha` and `material/change-sides` schemas describe the Rejection struct `{code, path, detail}`** (titles
   `…Rejection`) instead of `{material, alphaMode|doubleSided}`. They are annotated honestly as "Ablehnungscode …". Their fixtures are
   invalid against them.
3. **glTF fixture/schema drift:**
   - the `change-extras` and `change-extensions` operation fixtures use `data: {state, value}`, which the schema union rejects
     (10 fixtures);
   - `accessor/create` uses `componentType: 5125` and `kind: "SCALAR"`, while the schema has the string enums `UnsignedInt…` and
     `Scalar…`.
4. **The bcf and csv `set-snapshot` leaf schemas lack the `mutation` discriminator property.** Their `additionalProperties: false`
   therefore rejects their own fixtures `{mutation: "setSnapshot", …}`.
5. **The avi `set-snapshot` fixture's `strf` uses snake_case keys** (`bit_count`, `size_image`) that the schema does not admit.
6. **The glTF leaves carry the undeclared vendor keywords `x-semio` and `x-semio-mutation`.** The strict Ajv oracle refuses all 121
   leaves until they are added to the vocabulary or removed.
7. **dwg `4️⃣ac1018/…/set-snapshot` has a fixture but no catalogued leaf schema.**
8. **csv `patch-snapshot` → `https://…/s/stdio/snapshot-patch/schema.json` is in no catalogued scope** (`refUnresolved`, W1-D
   follow-up).
9. **Size and performance:**
   - The glTF leaf schemas grew from 4.05 MB to 6.66 MB (+2.6 MB); the whole group grew from 4.65 MB to 7.54 MB. Each glTF root union
     inlines a full `GltfDiff` in its `restore` branch, and the reader demands a label on every nested field even under
     `widget: hidden`.
   - Recommendations, for the W1-D follow-up / coordinator:
     - make `hidden` (and discriminated `restore` branches) opaque in the reader, so it does not recurse into them;
     - add the generic diff names (`added`, `removed`, `modified`, `diff`, `extensions`, `extras`) to the glossary;
     - reference one shared `GltfDiff` document instead of 51 inlined copies.

     Any of these makes most of the added bytes removable.
10. **The schema catalog now has 301 stale `🔣️.json` hashes** in this scope. `bun ./📜️script.ts schema generate` should run once
    centrally, after all W2-R groups land. It was not run here, to avoid racing the other rollout groups on the catalog.

## 6. Files

- Ticket scripts (kept):
  - `🧪️w2-r-stdio-a-walk.py` (reader mirror / inventory)
  - `🧪️w2-r-stdio-a-annotate.py` (tables and writer)
  - `🧪️w2-r-stdio-a-check-fixtures.py` (Python jsonschema: fixtures and InputUi)
  - `🧪️w2-r-stdio-a-check-inputs.ts` (strict Ajv, `parseInputUi`, reader)
- Scratch, in `🗑️generated/w2-r-stdio-a/`:
  - `before-*.json` / `after-*.json` (lint output)
  - `inventory.jsonl`, `names.txt`, `toplevel.txt`
  - `plan_dump.py`, `plan.jsonl`
  - `schema-vocabulary.txt`, `leaf-schemas.json`
- Modified schemas, all under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`:
  - 296 leaf `🧬️schema/🔣️.json` files. Four catalogued leaves are unchanged: csv `patch-snapshot` (unresolved `$ref`) and three leaves
    without inputs (docx strict and transitional `remove-conformance-attribute`, glTF `default-scene/unbind`).
  - `🏗️ifc/…/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/{🔣️.json, 📸️snapshot/🔣️.json}`
  - `📜️docx/…/🧱️base/🧬️schema/📸️snapshot/🔣️.json`
  - `📜️docx/…/🧱️base/🧬️schema/🧬️mutations/🧭️xml-address/🔣️.json`
  - `🧊️gltf/…/♾️any/🧬️schema/📸️snapshot/🔣️.json`
  - `🪟️bmp/…/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- Not touched: the glossary, the reader, the corpus, Rust/TS code, payload structure, and the stdio-b-owned xml `snapshot.json`.
