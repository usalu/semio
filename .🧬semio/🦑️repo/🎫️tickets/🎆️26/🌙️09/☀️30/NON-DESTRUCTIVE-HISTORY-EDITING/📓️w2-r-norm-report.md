# 📓️ W2-R norm — Input-UI rollout for `✏️s/🔌️plugins/📕️norm`

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-R-norm, 2026-09-30. Contract: `📋️design.md` §6, `🧭️plan.md`
"W2-R brief", manifest `$defs/InputUi`. Scope: DIN 4108, DIN V 18599, DIN EN 16798, EN 1990–1999 (German NA), ISO 16757,
VDI 3805 and the shared norm results config.

## 1. Outcome

DONE and VERIFIED. The strict lint for the norm scope went from 370 findings to **0**, with 895 of 895 inputs declared. That
includes zero root-union `malformed` and zero `refUnresolved` findings; the norm scope had none to begin with.

| Census (`schema mutation-inputs --census --under ✏️s/🔌️plugins/📕️norm`) | leaves | inputs | declared | missing | labelMissing | optionLabelMissing |
|---|---|---|---|---|---|---|
| before | 484 | 895 | 525 | 370 | 367 | 3 |
| after | 484 | 895 | **895** | **0** | 0 | 0 |

What was annotated:
- **All 518 norm leaf payload schemas that have inputs.** That is 484 catalogued leaves plus 34 uncatalogued leaves (see §5.2).
  - 950 top-level inputs, each with a complete `x-semio-ui`, not only the 370 the lint flagged. The 525 inputs that already passed
    on generic glossary labels now carry domain labels, units and references. For example, `newValue` "Neuer Wert" is now
    "Stützweite l" / "Span l".
  - 186 nested record fields (ISO 16757, VDI 3805 and the DIN V 18599 monthly climate), including option labels for every
    nested enum.
  - The fixture validator counts 1,136 `x-semio-ui` annotations over the whole scope (§3), a different base from the 950 + 186
    above.
- **What the annotations contain.** Widgets used: stepper 561, reference 232, text 137, record without a widget 163, toggle 17,
  slider 6, segmented 6, select 5, dial 5, vector 4. There are 232 `ref`s and 11 option tables.
- **The 30 EN 1990 leaf schemas are stubs** (`{"title","type":"object"}`) and have no inputs to annotate (§5.1).

## 2. Method and conventions

- **Sources.** Terminology comes from each artifact's editor field metadata (`✏️editor/🏷️field-meta/🦀️.rs`, which gives en/de
  labels, SI units and choice lists), the leaf's own `MutationKind::label` (en/de), and the diff code (which snapshot field the
  input writes). Stored units come from the snapshot `#[dsl(unit)]` attributes. Labels use the official German terms of each
  standard, for example:
  - Bemessungswert, charakteristischer Wert, Nennmaß der Betondeckung c_nom, Vorhaltemaß Δc_dev, statische Nutzhöhe d;
  - Wärmedurchgangskoeffizient U, längenbezogener Wärmedurchgangskoeffizient ψ, Bemessungswert der Wärmeleitfähigkeit λ,
    Gesamtenergiedurchlassgrad g;
  - Böengeschwindigkeitsdruck q_p, Topografiebeiwert c_o, rechnerischer Fahrstreifen;
  - Wichte des Schüttguts γ, Horizontallastverhältnis K, Wärmeeindringzahl b;
  - Nutzungsklasse, Klasse der Lasteinwirkungsdauer, Endkriechzahl φ_∞, Steifemodul E_s, Nachweisverfahren;
  - Temperaturänderungsgrad der Wärmerückgewinnung η_t, Kategorie des Innenraumklimas.
- **Units.**
  - `unit` is the stored unit: SI base where the snapshot is SI (m, Pa, N, N·m, K, s, J/m², N/m³), or the unit named in the field
    where the schema says so (en1993 `…Mm`, `…Mpa`, `…Kn`).
  - `displayUnit`/`displayFactor` give the engineering display unit (display = stored × factor):
    - Pa → N/mm² (strengths) or kN/m² (area loads);
    - N → kN; N·m → kN·m; N/m → kN/m; N/m³ → kN/m³;
    - m → mm (cross-section dimensions, covers, diameters);
    - m² → cm² (reinforcement areas), m²/m → cm²/m;
    - s → min; J/m² → MJ/m²; W/m² → kW/m²;
    - ratios 1 → %.
  - `step` is expressed in the stored unit and `precision` in the display unit.
- **References.**
  - Every entity id has `widget: reference`, `role: target` and `ref.kind` drawn from the artifact's own vocabulary. Kinds used
    include zone, ventSystem, element, window, thermalBridge, member, action, anchor, barLayer, concreteGrade,
    reinforcementGrade, connection, footing, soilLayer, pile, slope, retainingWall, material, section, product, geometry, curve,
    subject, productClass, productSeries, propertyDefinition, and others.
  - Value-side ids (for example `newVentSystemId`, `newClassId`, `functionRefs`) use `role: value` with a `ref`.
  - No `domain` or `granularity` is set, because norm reports have no interaction selection domain (see the app-surface test
    "no interaction domain, no pick granularity").
- **List positions.** Integer list positions are steppers with `minimum: 0`. Each is labelled with the entity it selects: in
  EN 1996, `index` means the wall, the load case, the opening or the concentrated load depending on the leaf.
- **Enums and codes.**
  - Real schema enums get localized `options`: EN 1993 annex, EN 1995 role/support, the EN 1991 fire method, the VDI 3805 edition
    and unit kinds, and the ISO 16757 exchange process, subject kind, property kind and operator.
  - String code fields without an enum stay `text`. Their description lists the admissible codes using the wire spelling (for
    example `Xc1–Xc4`, `Class1–Class5`, `bsP/bsT/bsA`). No enum was added, because that would change the payload structure.
- **Hard bounds.** Bounds were added only where the standard or the Rust diff makes them certain: 363 `minimum`, 38
  `exclusiveMinimum` and 22 `maximum` keywords.
  - Non-negative physical magnitudes: lengths, areas, volumes, densities, flows, strengths.
  - `exclusiveMinimum: 0` where the Rust diff refuses ≤ 0: GEG factor, A_NGF, V_e, stud spacing/diameter/f_u.
  - Ratios in 0..1 (g, F_C, η_t, φ_i, χ, ζ) and percentages in 0..100.
  - Codified ranges: wind zone 1–4, terrain category 0–4, deck type 1–3, geotechnical category 1–3, service class 1–3,
    cross-section class 1–4, supported edges 2–4, SFP class ≤ 7, pile count ≥ 1 (Rust refuses 0), tilt 0–180°, slope and φ′ 0–90°.
  - Three existing `minimum: 0` were tightened to 1 (HSS cross-section class, and the two EN 1995 service classes).
  - Deliberately unbounded, because they are signed or the convention is uncertain: actions and forces, eccentricities,
    temperatures in °C, ψ, c_pe, assumed wind pressure, assumed ΔT_u, net heat flux ḣ_net, interstorey drift, ΔU_WB, azimuths
    (which get a soft range 0–360 instead), and groundwater level.

## 3. Verification (all run in the foreground; nothing is claimed that was not run)

| Check | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/📕️norm` (cwd: repo test module), strict | **0 findings, exit 0**, 895/895 inputs of 484 leaves |
| `… --census` | norm 484 / 895 / 895 / missing 0 |
| `bun 🧪️w2-r-norm-check-inputs.ts --before 🗑️generated/w2r-norm/leaf-schemas-before.json`: W1-D TS reader `mutationInputDefs` over all 548 leaf schemas on disk, one input at a time | **leaves=548 inputs=950 failures=0** (before: 404 failures) |
| same script: every leaf compiled in the strict Ajv oracle `semioSchemaAjvV1` (third party, `x-semio-ui` registered), now vs before | **455 compile now = 455 before, 0 regressed**. The 93 that fail do so before and after (§5.4). |
| `.venv/bin/python 🧪️w2-r-norm-check.py --before …`: Python `jsonschema` validation of every `x-semio-ui` against manifest `$defs/InputUi` | **1136 annotations, 0 errors** |
| same script: every committed fixture mutation payload in scope validated against its leaf schema, now vs before | 372 fixtures: 132 name no existing leaf, **130 valid now = 130 valid before, 0 regressed** |
| `python3 🧪️w2-r-norm-annotate-inputs.py --check` (idempotence) | 0 to write, 518 unchanged, 0 problems |
| Structural diff before → after, with `x-semio-ui` stripped | Only the bound keywords listed in §2 were added; no other schema key changed |

No cargo run was needed; this rollout changed no Rust or TS code.

## 4. Files

- **Changed:** 518 leaf payload schemas `✏️s/🔌️plugins/📕️norm/**/🧬️schema/🧬️mutations/<leaf>/🧬️schema/🔣️.json`.

  | artifact | leaf schemas |
  |---|---|
  | din16798 | 41 |
  | din18599 | 19 |
  | din4108 | 43 |
  | en1991 | 80 |
  | en1992 | 28 |
  | en1993 | 49 |
  | en1994 | 25 |
  | en1995 | 66 |
  | en1996 | 58 |
  | en1997 | 20 |
  | en1998 | 22 |
  | en1999 | 18 |
  | iso16757 | 29 |
  | vdi3805 | 19 |
  | 🪟️results `change-selected-check-index` | 1 |

  Formatting is preserved: 2-space JSON, `ensure_ascii=False`, and the trailing newline as it was.
- **Ticket inputs (ticket root):**
  - `🧪️w2-r-norm-annotate-inputs.py` holds the annotation tables and writer. Options:
    - `--check` is a dry run;
    - `--baseline <snapshot>` rebuilds from the pre-rollout text, so a bound removed from a table is removed from the schema.
  - `🧪️w2-r-norm-check.py` runs the InputUi meta-schema check and the fixture before/after validation.
  - `🧪️w2-r-norm-check-inputs.ts` runs the reader over every leaf and the strict Ajv before/after compile.
- **Scratch (`🗑️generated/w2r-norm/`):**
  - `leaf-schemas-before.json` is the pre-rollout snapshot needed by `--baseline` and `--before`;
  - census/strict JSON from before the rollout;
  - exploration dumps (`dump.txt`, `leafdump.*`, `rust-fields.txt`, `fieldmeta.*`, `old-hashes.txt`).

## 5. Findings outside this WP (pre-existing; not changed, per brief: no payload-structure edits)

1. **EN 1990 has 30 stub leaf schemas.** Each is `{"title": …, "type": "object"}` with no `$id` and no properties, although the
   Rust leaves carry payload fields. These leaves are uncatalogued and have no inputs. Their payload schemas need to be authored.
2. **34 leaves have no `$id` and are not catalogued.**
   - 19 are EN 1991 fire/thermal/bridge leaves: snake_case keys such as `new_fire_duration`, the 2020-12 dialect, and
     inconsistent with their camelCase siblings.
   - 15 are EN 1996 leaves, such as `change-slab-span` and `change-density`.

   They are annotated and reader-clean, but the lint cannot see them until they get an `$id` and a catalog row.
3. **The schema and the Rust payload have drifted apart.**
   - EN 1993 `update-*-inputs`: the schema lists flat `new…` fields, while the Rust payload is a whole record
     (`section: SteelSection`, `joint: SteelJoint`, …).
   - EN 1999 `change-member-my-ed`/`-n-ed`: the schema uses `loadCaseId`/`newMYEd`/`newNEd`, the Rust payload
     `action_id`/`new_m_y_k`/`new_n_k`. The labels follow the Rust semantics (characteristic M_y,k, N_k).
   - EN 1991 `change-structure-kind`: the schema says boolean, the Rust type is `StructureKind` (Building|Bridge).
   - DIN V 18599 and DIN 4108: enums (`AutomationClass`, `BuildingCategory`, `UseClass`, `CalculationMethod`, `Attachment`,
     `ClimateZoneDe`) are typed `object` in the schema.
   - EN 1991 `new_fire_mode`: the schema enum is lower-case, the Rust variants are `None|Nominal|Parametric`.
   - VDI 3805 `newChoice` is `Legacy|Current` in the schema but `current` in the fixture, and `VdiUnit.kind` is PascalCase in the
     schema but `dimensionless` in the fixture.
4. **Some schemas are polluted.**
   - EN 1993 `insert-joint`, `insert-load-case` and `insert-pile` property nodes carry example values as unknown keywords
     (`category`, `frictionMu`, `embeddedLength`, …).
   - The VDI 3805 `configuration.attributes` node is an example object, not a schema.
   - EN 1995 uses `format: double`.
   - These defects, together with the 2020-12 leaves and a few others, make 93 of 548 leaves fail strict Ajv. That count is the
     same before and after this rollout.
5. **Fixtures.**
   - 132 of the 372 norm mutation fixtures name leaves that no longer exist (stale vocabulary in din16798 ×61, en1998 ×47,
     en1996 ×16 and en1990 ×8).
   - 110 of the 240 mapped payloads do not validate against their leaf schema. They are externally tagged or snake_case, and
     most are din4108 ×64, iso16757 ×19 and en1993 ×17.
   - None of these counts changed.
6. **The catalog is stale.** The `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` hashes for the 484
   annotated norm leaves need a central `schema generate` after the W2-R wave. It was not regenerated here, to avoid racing
   the parallel rollout groups. The lint does not depend on the hashes; it was verified green without the regeneration.
