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

## 6. Follow-up — invariant refusals are negative witnesses (fem, remodel) and the fem3d retained-limits twin

Coordinator decision: a fixture whose outcome is `mutation.invariant` must fail its leaf schema (`📋️design.md` §11, W2-S F16).
Status: **DONE and VERIFIED** — both lints report 0 findings for fem and remodel. Tool: `🧪️w2-r-mid-invariant-bounds.py`
(ticket root) for every JSON change; the Rust, TS, Python and feature changes were made by hand.

### 6.1 Value ranges → hard bounds

These are exactly what the Rust guards refuse. In the table, "> 0" means `exclusiveMinimum: 0`.

| Leaf | Bounds |
|---|---|
| fem2d `create-material`/`replace-material` | `e`, `rho` > 0; `nu` in (−1, 0.5) |
| fem3d `create-material`/`replace-material` | the same, plus `g` > 0 |
| fem2d `create-section`/`replace-section` | `area`, `iy` > 0 |
| fem3d `create-section`/`replace-section` | `area`, `iy`, `iz`, `j` > 0 |
| fem2d `create-region`/`replace-region` | `outline` minItems 3; every hole minItems 3; `thickness`, `meshSize` > 0 |
| fem3d `create-solid`/`replace-solid` | `outline` minItems 3; every hole minItems 3; `height`, `meshSize` > 0; `layers` minimum 1 |
| fem2d/3d `update-analysis-settings` | `modalCount`, `bucklingCount` minimum 1; `deformationScale` > 0 |
| remodel `update-geo-params` | `gsdM`, `dsmCellM`, `dtmFilterRadiusM` > 0; `orthoMaxPx` minimum 1 |
| remodel `update-feature-params` | `targetCount` minimum 1; `edgeThreshold` minimum 0 |
| remodel `update-ingest-params` | `frameSampleStride`, `maxFrames` minimum 1; `minSharpness` minimum 0 |
| remodel `update-match-params` | `ratioTest` in (0, 1] |

**`x-semio-ui` stays coherent.** The reader refuses a soft range outside the hard bounds, so `nu` `softMax` was lowered from 0.5 to
0.49. The same change was made in `🧪️w2-r-mid-annotate-inputs.py`.

**Descriptions.** The 29 fem2d leaf descriptions no longer say "Structural only". They now say that the hard bounds and
`x-semio-invariant` state exactly the payload-intrinsic breaches `mutations::guards` refuses, and that refusals depending on the
base belong to the diff alone.

### 6.2 Cross-field invariants → root `x-semio-invariant`

The fixture outcome names the invariant with `"invariant"`.

| Leaves | Invariant ids | Fixture that names one |
|---|---|---|
| fem2d `create-region`/`replace-region` | `region-outline-encloses-area`, `region-hole-encloses-area`, `region-holes-inside-outline` | `denies-loose-hole-d9efa1` → `region-holes-inside-outline` |
| fem3d `create-solid`/`replace-solid` | `solid-outline-encloses-area`, `solid-hole-encloses-area`, `solid-holes-inside-outline` | `sliver-outline-316a7c` → `solid-outline-encloses-area` |
| fem2d `create-combination`/`replace-combination` | `combination-not-self-weighted` | `self-term-0f54d1` |

fem3d combinations are not affected. A 3D term can only name a load case, so a self-term there is already `target-missing`.

### 6.3 State-dependent refusals → `mutation.target-missing` (Error)

All four remodel guards below were recoded identically in the Rust diff and its doc comment, the TS twin
(`🧬️mutations/🟦️.ts`) and the Python reference (`🐍️.py`, docstring included).

| Leaf | Refusal | Changes beyond the recode |
|---|---|---|
| `create-rig-extrinsic` | unknown camera | fixture outcome, `🥒️.feature` rows (both scenarios), fixture test (code and `Severity::Error`) |
| `create-stream` | unknown camera | same as above; the target is now the **missing camera id**, and the fixture `path` changed from `orbit-quaternary` to `orbit-cam-absent` |
| `add-stream-frame` | owner stream of another media kind | same as the first row |
| `add-gcp-observation` | unknown stream | none: it has no fixture, and it was recoded only for consistency |

The editor comment and test comment in `add-stream` no longer cite a "FATAL invariant".

### 6.4 fem window-config TS twin (`✏️s/🔌️plugins/🏗️fem/🧪️tests/🪟️window-config-contract/🟦️.ts`)

The Rust side is right. `FEM3D_RETAINED_TOOL_IDS` has 36 entries, the retained-route law asserts 36, and the fixture carries 36
routes. The schema `✏️editor/🎮️commands/🧬️schema/🚧️retained-limits/🔣️.json` was stale: its `routes` tuple had 18 entries and
`min/maxItems` 18.

I extended the tuple to 36 const rows (`id`, `disposition`, `lanes`, `blocker`) from the Rust-verified fixture and set
`min/maxItems` to 36. The first 18 rows were compared first and all matched.

### 6.5 Verification (all run, foreground, rustc gate)

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-payloads --under ✏️s/🔌️plugins/🏗️fem` | **0 findings**; 236/236 fixtures; 16 negatives, 3 of them via `x-semio-invariant` (before: 16 `negative`) |
| `… --under ✏️s/🔌️plugins/📸️remodel` | **0 findings**; 136/136; 4 negatives (before: 7 `negative`; 3 became positive) |
| `bun ./📜️script.ts schema mutation-inputs --under` fem / remodel | 0 / 0 findings (85/85, 56/56) |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-w2rmid cargo test -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d --lib -- standards::` | **818 + 923 passed, 0 failed**, including `semio_payload_law_fem2d_mutation` and `semio_payload_law_fem3d_mutation` and all the negative fixture tests |
| `… cargo test -p semio-s-artifact-remodel-remodeling --lib` | 1296 passed, 7 failed; see the note below |
| `bun test …/📸️remodeling/…/🧬️schema/🧪️tests/🧩️suite/🟦️.ts` (TS twin against every fixture outcome, codes and paths) | 1368 pass, 0 fail |
| `bun ./📜️script.ts oracle exhaustive --owner …/📸️remodeling/…/✳️any --case 📸️mutate-remodeling-1` (Python reference) | 275/275 |
| `bun ✏️s/🔌️plugins/🏗️fem/🧪️tests/🪟️window-config-contract/🟦️.ts` | exit 0 |
| `tsc --strict` over the remodel mutations twin | 0 errors in the edited file; siblings have their own errors under this ad-hoc config |

**The 7 remodel failures.** All the recoded and schema-related tests in the remodel run are green:
- the three refusal fixture suites (`declared_refusal_holds`, etc.);
- `semio_payload_law_remodeling_mutation`.

The 7 failures, re-run in isolation (1 of them then passed), are in untouched editor code that never reaches the four recoded
guards:
- 4 of them are the 8 ms worker-ceiling timing laws, which were run with 15–22 concurrent rustc;
- `export_qc_report_is_a_no_op_without_a_report` fails with `remodeling.qc-report.missing`;
- the report component and window-ownership tests fail with "Observations column" / "report render did not consume isolated
  table selection".

I could not run them against the pre-change tree, so "pre-existing" is inferred rather than proven.

**Not run:** the Rust subject and parity phases of `📸️mutate-remodeling-1`, which would need another adapter build. The Python
reference (275/275) and the Rust fixture tests assert the same codes.

**Files.**
- fem schemas: 20 leaf schemas with bounds or invariants (fem2d material, section, region, analysis and combination; fem3d
  material, section, solid and analysis), plus the fem2d description-only leaves.
- fem outcomes: 3 outcome files.
- fem3d `🚧️retained-limits/🔣️.json`.
- remodel schemas: 4 leaf schemas.
- remodel code: 4 `🔺️diff/🦀️.rs`, `🧬️mutations/🟦️.ts`, `🧪️tests/📸️mutate-remodeling-1/{🐍️.py,🥒️.feature}`.
- remodel fixtures: 3 fixture tests and 3 outcomes.
- remodel editor: `✏️editor/🎮️commands/🌱️add-stream/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}` and the `🛠️edit-calibration` unit-test comment, which still cited
  "FATAL" for an unknown stream.
- Logs: `🗑️generated/w2r-mid/{payloads-after-*.txt,cargo-fem.txt,cargo-remodel*.txt}`.
