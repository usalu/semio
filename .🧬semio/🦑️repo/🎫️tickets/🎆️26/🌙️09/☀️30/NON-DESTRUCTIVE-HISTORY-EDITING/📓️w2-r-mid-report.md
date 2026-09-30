# 📓️ W2-R-mid — Input-UI rollout: wfc, block, fem, layout, os-owned leaves (report)

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, rollout executor W2-R-mid, 2026-09-30. Contract: `📋️design.md` §6, the W2-R
brief in `🧭️plan.md`, the manifest `$defs/InputUi` meta-schema and W1-D's reader. Status: **DONE and VERIFIED**. The strict lint
reports **0 findings** for every owner in this group.

## 1. Census (`schema mutation-inputs --census`)

| owner | leaves | inputs before | declared before | findings before | inputs after | declared after | findings after |
|---|---|---|---|---|---|---|---|
| wfc | 72 | 133 | 98 | 35 (33 labelMissing, 2 optionLabelMissing) | 133 | 133 | **0** |
| block | 105 | 162 | 129 | 33 (32 labelMissing, 1 refUnresolved) | 162 | 162 | **0** |
| fem | 59 | 85 | 52 | 33 labelMissing | 85 | 85 | **0** |
| layout | 31 | 89 | 61 | 28 labelMissing | 89 | 89 | **0** |
| os | 18 | 34 | 2 | 32 (28 labelMissing, 1 malformed, 3 optionLabelMissing) | 49 | 49 | **0** |
| window (gis map window lane) | 6 | 8 | 7 | 1 labelMissing | 8 | 8 | **0** |

Notes on the table:
- **os input count.** It grew from 34 to 49 because W1-D's root-union reader landed at 04:36. `🎨️ui-preferences` now yields a
  variant selector and the fields of each variant. The selector and variant labels are annotated, so its former `malformed`
  finding is gone.
- **block refUnresolved.** A peer changed `👁️set-brush-preview` to use a local `$defs/Block3dBrushPreview` while this WP ran. That
  removed the one `refUnresolved`, and its fields are annotated now.
- **Repo-wide.** The total went from 1,842 findings (3,276 of 5,118 inputs declared) to 752 (4,385 of 5,137). That figure includes
  other groups' concurrent work.

## 2. What was annotated

The tool is `🧪️w2-r-mid-annotate-inputs.py` in the ticket root. It is rule-driven, per-artifact vocabulary tables, and strict:
an input without a rule aborts the run. It annotated **348 leaf schemas** with **1,371 `x-semio-ui` annotations**:

| family | leaf schemas | annotations |
|---|---|---|
| fem2d | 29 | 213 |
| fem3d | 30 | 202 |
| block2d | 26 | 58 |
| block3d | 38 | 92 |
| block5d | 41 | 102 |
| wfc graph (2d/3d) | 38 | 154 |
| wfc grid2d | 14 | 105 |
| wfc grid3d | 14 | 41 |
| wfc bitmap | 11 | 37 |
| layout | 31 | 113 |
| os config (incl. the `ui-preferences` root union) | 17 | 115 |
| gis map window lane | 6 | 8 |
| dag | 14 | 31 |
| flow | 10 | 16 |
| workflow | 17 | 23 |
| workflow run | 5 | 36 |
| store | 6 | 21 |
| interaction | 1 | 4 |

**What is covered.** Every top-level input carries an explicit label. So do every nested record field (inline or `$defs`), every
array-item record field, every root-union variant (a `label` on the `$defs` variant) and every nested-union branch (FEM
loads/elements, wfc 3d media, run trigger), together with the fields of those branches. Enum items get `options`. The `mutation`
discriminators and nested `kind` constants are `{widget: hidden, role: discriminator}`.

The os-owned scope goes beyond the linted config/window lanes. It also covers the uncatalogued (`product-module` level) os artifact
leaves: dag, flow, workflow, run, store and interaction.

### 2.1 Rules

- **Targets.** Entity ids are `{widget: reference, role: target, ref: {kind, domain, granularity}}`.
  - The kinds come from each artifact's own vocabulary.
  - `domain`/`granularity` are set only where the artifact's `InteractionDefinition` actually selects that kind:

    | artifact | domain | granularities |
    |---|---|---|
    | fem2d | `fem2d` | node, element, region, support, load, material, section, loadCase, combination |
    | fem3d | `fem3d` | the same, with `solid` instead of `region` |
    | block2d | `handle` | handle, handleKind |
    | block3d | `vortex` | vortex |
    | block5d | `grip` | grip, gripKind |
    | wfc 2d/3d | `slot` | slot |
    | layout | `elements` | frame (granularity `element`) |

    Every other target kind carries `{kind}` only: tile, rule, edge, page, layer, story, link, styles, vortexKind, author,
    compatibilityRule, representation, and the dag/flow/workflow/store entities.
  - A top-level `id` is resolved to the entity the leaf verb addresses, e.g. `change-handle-handle-kind` → handle,
    `disconnect-slots` → edge, `disconnect-nodes` → edge. `id`s inside created records are `text`.
- **No validation keyword was added or changed.** The fem leaf descriptions state that the payload schemas are "structural only",
  with plausibility bounds living in the snapshot schema and the guards. The same policy was applied to the whole group, so
  `x-semio-ui` holds UI facts only (soft ranges, steps, units).
- **Widgets.**

  | value | widget |
  |---|---|
  | booleans | toggle |
  | integers | stepper |
  | enums | segmented (≤ 3 options) or select, with localized `options` |
  | fixed-length numeric arrays (positions, directions, quaternion, RGBA 0–1) | vector |
  | angles (FEM roll, block handle/grip angle, layout frame rotation) | dial: `unit rad`, `displayUnit deg`, 1° step, soft ±π, snaps at multiples of π/2 |
  | zoom factors | log slider 0.1–10 with snaps |
  | 0–255 colour channels | slider |
  | base64 pixel data, proxy data URLs | hidden |
  | free text | text, or multiline for descriptions, story text and JSON |

### 2.2 Units and terminology

- **FEM**, SI storage with engineering display units:

  | quantity | label (en / de) | storage | display |
  |---|---|---|---|
  | E | Young's modulus / Elastizitätsmodul | Pa | GPa, as the plugin inspector shows it |
  | G | Shear modulus / Schubmodul | Pa | GPa |
  | ν | Poisson's ratio / Querdehnzahl | — | soft range 0–0.5 |
  | ρ | Density / Dichte | kg/m³ | kg/m³ |
  | A | Querschnittsfläche | m² | cm² |
  | I_y, I_z | Flächenträgheitsmoment | m⁴ | cm⁴ |
  | I_T | Torsionsflächenmoment (Saint-Venant) | m⁴ | cm⁴ |
  | w | Streckenlast | N/m | kN/m |
  | p | Flächenlast | Pa | kN/m² |
  | thickness | Dicke | m | mm |
  | mesh size | Netzweite | m | m |

  Other terms:
  - Lastfall, Lastfallkombination, Faktor (described as "Teilsicherheits- bzw. Kombinationsbeiwert"), Eigengewicht.
  - Lager and Gesperrte Freiheitsgrade, with options Verschiebung x/y/z and Verdrehung um x/y/z.
  - Anfangs-/Endknoten, Fachwerkstab / Balken / Rahmenstab, Verdrehwinkel.
  - Scheibenbereich, Volumenkörper, Extrusionsachse/-höhe, Basishöhe, Elementlagen.
  - Anzahl Eigenformen / Knickformen, Verformungsmaßstab.
  - Knotenlast / Gleichstreckenlast / Flächenlast.
  - 2D member loads are described as local (wx along, wy across) and 3D member loads as global components, matching the engine.
- **Layout**, DTP terms:
  - Absatzformat, Zeichenformat, Textfluss, Textumfluss, Schriftgrad (pt), Schriftstärke, Zeilenabstand (pt), Laufweite.
  - Grundlinienraster / Grundlinienversatz (pt), Spaltenabstand, Rand oben/unten/links/rechts (mm), Innenabstand (mm).
  - Mustervorlage, Druckziel, Verknüpfung, Prüfsumme, Farbprofil, Auflösung (dpi).
  - Page and frame geometry is in mm (A4 = 210 × 297 in the fixtures). Frame rotation is in rad.
- **block**: the plugin's own terms. Griff/Griffart for block2d handles and block5d grips, Wirbel/Wirbelart, Objektart, Bauteil,
  Darstellung, Detailstufe, Kompatibilitätsregel, Standard-Leitungs-/Kabel-/Seilart.
- **wfc**: Kachel, Slot, Nachbarschaftsregel, Relation, Gewicht, Startwert, Periodisch in X/Y/Z, Spalte/Zeile/Lage for grid cells,
  Mustergröße, Symmetrie, Boden, Palettenindex.

### 2.3 Formatting

Every file keeps its own layout:
- 323 pretty (indent 2) files and 23 minified files are rewritten with the identical serializer.
- The one-line file keeps its default separators.
- 17 hand-formatted files (inline property objects) are edited in place by a JSON span editor that keeps their layout: inline
  objects get an inline `x-semio-ui`, and expanded objects get the member on its own line.
- Key order follows W1-F.

## 3. Files

- **Leaf payload schemas.** 348 `…/🧬️mutations/<leaf>/🧬️schema/🔣️.json` under:
  - `✏️s/🔌️plugins/{🀄️wfc,🧱️block,🏗️fem,📏️layout}`
  - `🧰️framework/🛍️products/💻️os/🎚️config`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/{♾️infinite/🗿️artifacts/🕸️dag,🌊️flow,🔁️workflow (workflow+run),🏪️store,🔌️plugin/🕹️interaction}`
  - `✏️s/🔌️plugins/🌍️gis/…/🪟️windows/🗺️map/🎚️config`

  The exact list is `🗑️generated/w2r-mid/before-manifest.json`, minus the 16 leaves that have no properties. Pre-edit copies of
  every file are in `🗑️generated/w2r-mid/before/`.
- **Outside the group, annotation-only.**
  - `🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json` `$defs/AppRef`: labels for `pluginId` and `appId`, needed by
    `set-default-app`. Edited with the Edit tool on a unique anchor.
  - The io `$defs/ArtifactDialect` fields were already annotated by a peer while I worked, so that file was not touched.
- **Ticket inputs, kept.**
  - `🧪️w2-r-mid-annotate-inputs.py` writes the annotations. `--check` writes nothing; `--preview <dir>` writes copies to `<dir>`.
  - `🧪️w2-r-mid-check-inputs.ts` checks the reader and the `InputUi` meta-schema over every in-scope leaf, catalogued or not.
  - `🧪️w2-r-mid-validate-fixtures.py` compares the Python `jsonschema` fixture verdicts before and after.
- **Scratch.** `🗑️generated/w2r-mid/`: inventories, trees, lint/census/check logs and the before snapshot.

## 4. Verification (all run, foreground)

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under <path>` (test module), for each of the 6 scope paths | **0 findings** each: wfc 133/133, block 162/162, fem 85/85, layout 89/89, os 49/49, gis window 8/8 |
| `bun ./📜️script.ts schema mutation-inputs --census` | wfc, block, fem, layout, os, window: missing 0 |
| `bun 🧪️w2-r-mid-check-inputs.ts`: W1-D TS reader over **all 364** in-scope leaves (incl. uncatalogued), each `x-semio-ui` against manifest `$defs/InputUi` (npm `jsonschema`), explicit label present on every top-level input | 629 inputs, 1,371 annotations, **0 meta-schema errors, 0 unannotated**. One remaining finding outside the lint: see §5.1 |
| Annotation-only diff: every edited file equals its snapshot once `x-semio-ui` is stripped | 348 edited, 16 untouched, 0 structural diffs of mine. The 1 structural diff is the peer's concurrent `$ref` localisation in block3d `👁️set-brush-preview`. |
| `.venv/bin/python 🧪️w2-r-mid-validate-fixtures.py 🗑️generated/w2r-mid/before`: all 447 committed `🦠️mutation/🔣️.json` fixture payloads, Draft 7 + `referencing` | **0 verdict changes** (exit 0). 423 valid, 24 invalid both before and after, pre-existing (§5.3). |
| `bun test 🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️mutation-inputs/🟦️.ts` (after the manifest AppRef edit) | 33 pass, 0 fail |

No cargo was run, per the brief. The leaf derive `include_str!`s these schemas, so the next build of the affected crates recompiles.
That changes content only and cannot affect types.

## 5. Open items (not in this WP's power)

1. **flow `♻️replace-flow-host-snapshot`** (uncatalogued, so not linted) is refused as `labelMissing` at `/hostSnapshot/schema`.
   The field lives in the foreign flow `snapshot.json` `$defs/HostDocument`, a document schema. Its owner, or the glossary
   (`schema`), must label it.
2. **Stub payload schemas: 14 layout leaves.** `create-layer`, `set-story-runs`, `update-link`, `set-page-guides`, the three
   character-style leaves, `update-spread`, `set-page-parent`, `set-page-overrides`, `set-drawing-text`, `set-frame-layer`,
   `reorder-frame` and `update-parent-page` have schemas of the form `{"type":"object"}` with no properties. The workflow leaves
   have the same problem: `update-node-ports` has empty properties, and the `node`/`binding`/`input`/`edge`/`parameter` records are
   bare objects. All of these are opaque to the history editor until their payload schemas describe the real Rust payloads, which
   is a payload-structure change outside the brief.
3. **Pre-existing fixture/schema divergence (24).** The history editor edits the wire form, so these schema gaps matter:
   - 21 layout and 1 wfc 3d fixture payloads are in the serde externally-tagged snake_case form (`{"ChangePageWidth":{"new_width":…}}`),
     not the camelCase wire form the leaf schemas describe.
   - 2 block fixtures (`update-presentation`, `update-part2d` `circle-to-rectangle`) carry `newRadius: null`, but the schemas type
     `Option<f64>` fields as a bare `number`.
4. **Schema catalog.** `📚️library/🔣️schema-catalog.json` stores per-file sha256 hashes. They are stale for the 348 leaves and the
   manifest schema until a central `bun ./📜️script.ts schema generate`, which W1-F ran for puzzle 2d. I left this to the
   coordinator because the other W2-R groups need the same regeneration. The lint does not read the hashes.
5. **Test-fixture leaves untouched.** Leaves under `🧫️fixtures` (the plugin/spr/store test aggregates) are test inputs, and some
   tests assert their roster rows, so no annotations were added to them.
