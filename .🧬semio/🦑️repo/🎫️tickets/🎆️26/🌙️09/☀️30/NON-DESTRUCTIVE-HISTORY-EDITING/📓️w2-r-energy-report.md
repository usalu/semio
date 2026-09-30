# 📓️ W2-R energy — input-UI rollout for `✏️s/🔌️plugins/🔋️energy`

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor W2-R-energy, 2026-09-30. Contract: `📋️design.md` §6, `🧭️plan.md`
"W2-R brief", W1-D meta-schema `$defs/InputUi`.

## 1. Outcome

**DONE and VERIFIED.** The strict lint for the energy scope reports **0 findings**: all 737 inputs of the 291 leaves are
declared. The group had no root unions (`malformed` 0) and no cross-document `$ref`s (`refUnresolved` 0).

| Census (`schema mutation-inputs --census --under ✏️s/🔌️plugins/🔋️energy`) | leaves | inputs | declared | missing | labelMissing |
|---|---|---|---|---|---|
| before | 291 | 737 | 411 | 326 | 326 |
| after | 291 | 737 | **737** | **0** | **0** |

The 411 inputs that were "declared" before only resolved through the glossary: generic labels such as `id` → "ID", with no
units, widgets or German domain terms. So every input was annotated, not only the flagged ones:

- 743 `x-semio-ui` annotations: 737 top-level inputs plus the 3 nested `camera` fields in each of the 2 set-camera leaves.
- 463 structural hard bounds (§3.2).
- 289 of 291 leaf schemas changed. The other 2 (`disconnect-referenced-model`, `unbind-weather-file`) have no inputs.

Every annotation has:

- `widget`, `role`, `label {en, de}`, `group` and `order`;
- `description {en, de}` wherever the name is not self-explanatory;
- `unit`, plus `displayUnit` and `displayFactor` where a domain display unit exists;
- `step` and `precision`;
- soft ranges and snaps only where they are meaningful;
- `options {en, de}` for every enum.

Widget mix, as `argControl` derives it from the whole-leaf reader output:

| stepper | text | slider | select | toggle | segmented | dial |
|---|---|---|---|---|---|---|
| 579 | 64 | 48 | 16 | 17 | 8 | 5 |

The 64 text controls include the arrays, which the framework currently renders as text.

## 2. How

- `🧪️w2-r-energy-annotate.py` writes the annotations.
  - It covers the 289 standard-format leaves.
  - It is idempotent: `--check` reports `changed=0`.
  - Output is `json.dumps(indent=2, ensure_ascii=False)`. Before any edit it was verified to round-trip the existing bytes of
    every one of those 289 files.
  - It reads each leaf's Rust payload struct (`🦀️.rs`) for the field types and the `SemanticDescriptor` verb and entity.
  - It refuses to run if any input lacks an annotation or a Rust field.
- The two compact-format `🎥️set-camera` schemas (edit window and viewer window) were edited in place. Their inline brace style
  was kept.
- Nothing else was touched: no glossary, reader, corpus, Rust/TS code, payload fields, fixtures or catalog.

## 3. Decisions

### 3.1 Entity ids are numeric steppers, not references (reader gap)

Every energy id is a Rust `u32` newtype (`EntityId`, `ScheduleId`) serialised as a JSON **integer**. The W1-D reader (Rust
and TS) refuses `role: target`, `widget: reference` and `ref` on anything that is not a string id: it returns
`widgetIncompatible: a reference is a string id or an array of string ids`. So the design §6 `role: target` + `ref` cannot be
expressed today without a lint finding.

Ids are therefore annotated as follows:

| Input | Label | Group | Description |
|---|---|---|---|
| Addressed entity (`id` of change/delete/rename/add/remove/…) | The entity noun ("Material" / "Material", "Ideal loads system" / "Ideallastsystem") | `target` | "The material this mutation changes, by id." / "Material, das diese Mutation ändert, per ID." |
| Minted `id` of create-* leaves | "ID" | `identity` | Later mutations address the entity by this id. |
| Reference fields (`zoneId`, `scheduleId`, `constructionId`, `surfaceId`, …) | Hand-written en/de label per field | per field | Hand-written en/de description per field |
| Id arrays | Label and description only | per field | — |

Each single id is a `stepper` with step 1 and precision 0. The id arrays carry no widget because the reader has none for
integer lists.

**Follow-up for W1-D (reader).** Admit integer-keyed references. The energy interaction domain already maps them one-to-one:
target ids are `id.0.to_string()` in domain `energyModel` (`✏️editor/🕹️interaction/🦀️.rs`). The flip is then mechanical
(swap the stepper for `role: target` plus a `ref`):

| Input | `ref` |
|---|---|
| addressed `id` | `{kind: <granularity>, domain: "energyModel", granularity: <granularity>}` |
| `zoneId`, `zoneIds`, `terminalZoneIds` | `zone` |
| `spaceId`, `spaceIds` | `space` |
| `surfaceId`, `surfaceAId`, `surfaceBId`, `interzoneSurfaceId` | `surface` |
| `constructionId`, `glazingConstructionId` | `construction` |
| `layerMaterialIds` | `[material, glazingMaterial, gasMaterial]` |
| every `*ScheduleId` | `schedule` |
| `targetEquipmentId` | `hvac` |

Granularities available in the domain: zone, space, surface, fenestration, shading, material, glazingMaterial, gasMaterial,
construction, thermostat, load, hvac, schedule. The plain node numbers `supplyNodeId`, `returnNodeId`, `outdoorNodeId`,
`nodeIds` and `linkIds` are not entities and stay numeric.

### 3.2 Hard bounds: structural only

- **Added.** Hard bounds added are the Rust type domain of every unsigned payload field:
  - `minimum: 0` on 432 integer fields: ids, indices, counts, multipliers, node numbers;
  - `items.minimum: 0` on 13 id arrays;
  - `maximum: 255` on the 15 `u8` fields and `maximum: 65535` on the 3 `u16` fields (18 maxima in total).

  No payload that decodes can violate these bounds.
- **Withheld.** Semantic ranges are **not** added as hard bounds. These are the ranges the leaf diffs enforce as
  `mutation.invariant`: fractions in [0, 1], efficiencies in (0, 1], positive conductivity, thickness, U-value or capacity,
  month 1–12, weekday 0–6, priority ≥ 1, and so on. The committed refusal vectors carry exactly those out-of-range payloads by
  design, for example `newDensityKgM3: 0.0`, `month: 13`, `newInverterEfficiency: 1.5` and `kind: "Sunny"`. A hard bound
  would make them schema-invalid. Adding bounds per family, depending on whether a vector happens to exist, would make
  create- and change-leaves of the same quantity disagree.
- **Where the ranges appear instead.** They reach the UI as soft slider ranges (`softMin`/`softMax`: 0–1, 0–90°, 0–360°,
  1–12, 1–31, 0–6, ±90, ±180, −12…14) or as a description. A history edit outside the range therefore replays into the leaf's
  own `mutation.invariant` Error, which blocks finalize.
- **Existing bounds kept.** The config leaves keep their bounds: timesteps 1–60 and warm-up 0–365.

### 3.3 Units and display

- Stored units are the SI units of the payload, written as unit strings: °C, K, W, W/m², W/(m²·K), W/(m·K), W/K,
  J/(kg·K), kg/m³, kg/s, m, m², m³, m³/s, m³/(s·m²), 1/h, 1/m², L/s, Pa, lx, kWh, s, min, d, h.
- Angles use `deg`, the fleet convention from W1-F and its peers.
- Conventional display units:

  | Stored | Displayed |
  |---|---|
  | W | kW (kWp for PV DC power) |
  | m³/s | m³/h |
  | m³/(s·m²) | m³/(h·m²) |
  | m (layer thickness, gas gap) | mm |
  | m³ (storage volume) | L |
  | m² (effective leakage area) | cm² |

  Steps are chosen so the displayed steps are round: 10 m³/h, 1 m³/h per person, 0.1 m³/(h·m²), 100 W and 1 mm.
- Shares and efficiencies are shown in % (`displayUnit %`, `displayFactor 100`, slider 0–1). This covers the sensible,
  latent, radiant, visible and return-air fractions, severity, and the module, inverter, collector, fan and round-trip
  efficiencies.
- Radiation properties stay dimensionless 0–1, as they are tabulated in DIN EN 410/673 and DIN 4108-4: SHGC (g-value),
  τ_v, α_sol, α_vis, α_th, ε_IR and window τ.
- Dials are used for azimuth and north axis (0–360°, cardinal snaps). Tilt is a slider (0–90°, 15° grid).
- Zone and system timesteps snap to the divisors of 60.

### 3.4 German terminology

Labels reuse the plugin's own German vocabulary, taken from every leaf's `label()`. That way the editor and the history rows
say the same thing. Examples:

- Luftkreislauf, Anlagenkreislauf, Anlagenkomponente;
- Feuchteregler, Sollwertmanager, Ideallastsystem, Zonengerät, Stromverteiler;
- Trinkwarmwasseranlage, Solarthermieanlage, Kälteanlage, Wasseranlage;
- Personen-, Beleuchtungs- and Gerätewärmegewinn;
- Tages-, Wochen-, Jahres- and Zeitreihenzeitplan;
- Verglasungsmaterial, Gasfüllung, Verglasungsaufbau.

Physical quantities use the DIN/VDI terms:

- Wärmeleitfähigkeit, Rohdichte, spezifische Wärmekapazität;
- U-Wert, Gesamtenergiedurchlassgrad (g-Wert), Lichttransmissionsgrad, solarer, sichtbarer and thermischer Absorptionsgrad;
- Rahmen- and Sprossen-Wärmeleitwert, Brüstungshöhe, Überstands- and Seitenblendentiefe;
- Heiz- and Kühl-Proportionalbereich, Bemessungszuluftvolumenstrom, Bemessungsluftwechsel;
- Durchflussbeiwert, effektive Leckagefläche, Vorlauf- and Rücklauftemperatur, Bemessungsmassenstrom;
- Ventilatordruckerhöhung, Soll-Beleuchtungsstärke, Blendungsgrenzwert, Einschwingtage;
- Außenrandbedingung, erdberührtes Bauteil.

The German target descriptions avoid adjective declension: they are noun-first with an accusative relative pronoun chosen
by grammatical gender.

## 4. Verification (all run, foreground)

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/🔋️energy"` (repo test module, strict) | **exit 0**, 737/737 declared, **0 findings**. Before: exit 1, 326 `labelMissing`. |
| `… --census` | energy 291 / 737 / 737 / missing 0 |
| `.venv/bin/python 🧪️w2-r-energy-check.py` (Python `jsonschema` Draft7 + `referencing`) | Validates 580 fixture payloads, 6 config-corpus verdicts and all 743 annotations against the manifest `$defs/InputUi`. Result: 0 InputUi errors, 6/6 corpus verdicts kept, 578/580 fixtures valid. The **2 failures are pre-existing and identical before and after** (baseline run `--baseline`, see §5.1). |
| `bun 🧪️w2-r-energy-check-inputs.ts` | Whole-leaf `mutationInputDefs` + `argControl`: 737 inputs, 0 reader errors. `parseInputUi` accepted all 743 annotations. The strict Ajv oracle (`semioSchemaAjvV1`, `x-semio-ui` registered) compiled all 291 leaves. Ajv validated 578/580 fixtures (the same 2 pre-existing failures). |
| `bun ./📜️script.ts schema --under "✏️s/🔌️plugins/🔋️energy" --json` | 14 diagnostics before and 14 after, **0 new**. All 14 are pre-existing: export parser missing, owner ineligible, uncatalogued window-config modules, placement. |
| Rust | Not run (the brief forbids cargo). The Rust reader is the byte-equal twin of the TS reader per W1-D's corpus. The leaves `include_str!` their schema, so the next energy build will recompile them. |

## 5. Findings for the coordinator

1. **`🧱️create-material` leaf schema lacks `roughness` (pre-existing, blocks history edits).**
   - The Rust payload `CreateMaterial` has `roughness: SurfaceRoughness`, and both committed fixtures carry it.
   - The JSON Schema, the TS twin `🧬️mutations/🟦️.ts` and the proto omit it, while the schema says `additionalProperties: false`.
   - Result: both fixtures fail the leaf schema, and a `historyEditInput` on a create-material (`payload_value` → validate)
     will be refused.
   - Fix: add `roughness` (enum VeryRough … VerySmooth) to the leaf schema, the TS twin, proto and graphql. This is outside
     the W2-R brief, because it changes payload structure. Once `roughness` exists in the schema, the annotation
     vocabulary to use is already in the annotate script as `ROUGHNESS`, and it is used today by `change-material-roughness`.
2. **Integer references (W1-D reader).** See §3.1; the mapping table makes the flip mechanical.
3. **Catalog hashes are stale.** `📚️library/🔣️schema-catalog.json` stores a sha256 per leaf schema, and 289 energy entries now
   differ. `schema check` does not gate hash drift (0 new diagnostics). Regenerate the catalog centrally once all W2-R groups
   have landed (`bun ./📜️script.ts schema generate`); it was deliberately not regenerated here, to avoid racing the other
   rollout agents.
4. **Engine weekday slot bug (out of scope).**
   - `Date::day_of_week()` returns 1 = Mon … 7 = Sun.
   - Both `schedule::weekly_value` and the kernel (`🌰️kernel/🦀️.rs:1998`) index with `min(dow, 6)`.
   - As a result slot 0 of a weekly schedule is never read, and Saturday and Sunday share slot 6.
   - The `dayIndex` description therefore only says "slot 0 to 6" and does not name weekdays.
5. **Unit ambiguities, left without a unit.**
   - The humidistat throttle ranges are compared against RH as a 0–1 fraction in the engine (setpoints 0.4 and 0.6), but the
     fixtures use 5.0 and 10.0.
   - The glare limit is a simplified 0–1 index in the engine (0.4), but a DGI-like 22 in the fixtures.
6. **Setpoint-manager `kind` is a free string.**
   - The diff accepts only Scheduled, OutdoorAirReset, WarmestZone and ColdestZone.
   - An `enum` would allow a `select`, but it would invalidate the `⛔️refuses-a-bad-kind` vector (`"Sunny"`).
   - It is a `text` input with the four values in its description.
7. **Terminology notes.** Kept for consistency with the plugin's mutation labels; a plugin-wide rename is suggested.
   - "Thermische Hülle" names a zone enclosure, but in DIN usage it means the building envelope.
   - "Sollwertmanager" would be "Sollwertführung" in German HVAC usage.

## 6. Files

- Modified: 289 leaf payload schemas `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/**/🧬️mutations/<leaf>/🧬️schema/🔣️.json`:
  - 285 model leaves under `🧬️schema/🧬️mutations/` (287 minus the 2 without inputs);
  - 2 config leaves under `✏️editor/🎚️config/…`;
  - 2 set-camera leaves under `✏️editor/…/🧊️model/🎚️config/…` and `👁️viewer/…/🧊️model/🎚️config/…`.
- Created in the ticket root (inputs, kept):
  - `🧪️w2-r-energy-annotate.py`: annotation writer, idempotent, `--check`;
  - `🧪️w2-r-energy-check.py`: Python `jsonschema` fixtures, corpus and InputUi; `--baseline`;
  - `🧪️w2-r-energy-check-inputs.ts`: TS reader, `argControl`, `parseInputUi`, strict Ajv and Ajv fixtures.
- Scratch in `🗑️generated/w2r-energy/`: lint and census JSON/TXT before and after, check outputs, the leaf, fixture and
  invariant dumps, and the `schema --json` before/after.

## 7. Follow-up (coordinator request, 2026-09-30)

### 7.1 `create-material` roughness — fixed

The schema was the wrong party: the Rust payload `CreateMaterial` has `roughness: SurfaceRoughness` between `name` and
`thicknessM`, and so do both fixtures. Every schema-first format now matches it, with Rust field order:

| Format | Change |
|---|---|
| Leaf JSON Schema `🧱️create-material/🧬️schema/🔣️.json` | New property `roughness: {enum: [VeryRough, Rough, MediumRough, MediumSmooth, Smooth, VerySmooth]}`, the same enum as `change-material-roughness`, added to `required` after `name`. Its `x-semio-ui` is written by the annotator: `select`, "Roughness"/"Rauigkeit", localized options Sehr rau … Sehr glatt, group `thermal`. |
| TS twin `🧬️mutations/🟦️.ts` | `readonly roughness: "VeryRough" \| … \| "VerySmooth";` |
| Proto `🧬️mutations/🛰️.proto` | `string roughness = 4;`, with the later fields renumbered 5–11 to keep Rust order (greenfield, no wire compatibility). The string shape matches `ChangeMaterialRoughness.new_roughness`. |
| GraphQL `🧬️mutations/🔗️.graphql` | `roughness: String!` |
| Normative text grammar `🧬️mutations/📖️.grammar.semio` | `"roughness" "=" TEXT` in `create-material` |

No other TS code constructs a `CreateMaterial`.

### 7.2 German terms — corrected in the history labels and in `x-semio-ui`

Only German label strings were edited, and each keeps its placeholder count. The history row and the editor now use the
same term.

| Before | After | Why |
|---|---|---|
| Sollwertmanager | **Sollwertführung** | German HVAC control term for setpoint scheduling/reset (e.g. witterungsgeführte Sollwertführung) |
| Thermische Hülle / thermischer Hülle | **Zonenverbund** — **not** "Gebäudehülle" | See below |

**Deliberate deviation.** The entity is `ThermalEnclosure { name, zone_ids }`, a named group of zones (the EnergyPlus radiant
and solar "enclosure"). It is not the building envelope. My earlier finding meant that the old term wrongly suggested the
envelope, because in DIN usage "(thermische) Hülle" means "Gebäudehülle". Renaming it to "Gebäudehülle" would state that
error outright. "Zonenverbund" says what the object is. If you still want a different word, it is one entry in the annotator
vocabulary (`ENTITY["thermal-enclosure"]`) plus the 5 label strings.

- **Leaf labels** (`MutationKind::label`, German string only):
  - `create-`, `delete-` and `rename-thermal-enclosure`;
  - `add-` and `remove-thermal-enclosure-zone`;
  - `create-`, `delete-` and `rename-setpoint-manager`;
  - `change-setpoint-manager-schedule` and `replace-setpoint-manager-kind`.
- **`x-semio-ui`**: 9 leaves were re-annotated. These are the 8 addressed-id labels and descriptions (for example
  "Zonenverbund, den diese Mutation löscht, per ID." and "Sollwertführung, die diese Mutation ändert, per ID."), plus the
  `zoneIds` description of `create-thermal-enclosure`.
- **Leftovers**: `/usr/bin/grep -rl "Thermische Hülle|thermischer Hülle|Sollwertmanager" ✏️s/🔌️plugins/🔋️energy` finds 0 files.
  No test or fixture asserted the old strings.

### 7.3 Verification after the follow-up (all run)

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/🔋️energy"` | exit 0, **738/738** declared (the new `roughness` input), 0 findings |
| `.venv/bin/python 🧪️w2-r-energy-check.py` | exit 0, **580/580 fixtures valid** (both create-material fixtures now pass), 6/6 corpus verdicts, 744 annotations valid against `InputUi`, **0 failures** |
| `bun 🧪️w2-r-energy-check-inputs.ts` | exit 0: 738 inputs through the whole-leaf reader, 744 annotations pass `parseInputUi`, strict Ajv compiles all 291 leaves, Ajv 580/580 fixtures valid, 0 failures |
| `cargo check -p semio-s-artifact-energy-model --message-format=short` (gated, once) | **exit 101, blocked by a peer**. See the note below the table. |

**Rust compile: WRITTEN BUT UNVERIFIED.** The cargo failure is not in my files:

- The error is `couldn't read …/🧬️mutations/🏝️change-humidistat-dehumidifying-setpoint/🦀️.rs`. It comes from a peer's
  **staged** edit (01:04 today) to the crate mount `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs`.
- That edit points 4 leaves at shortened directory names: `🏝️change-humidistat-dehumidifying-setpoint`,
  `🧻️change-humidistat-dehumidifying-throttle`, `🔴️change-ideal-loads-system-max-heating-supply` and
  `🔵️change-ideal-loads-system-min-cooling-supply`.
- Only the matching **fixture** directories were renamed (staged `R`). The leaf directories under `🧬️schema/🧬️mutations/`
  still carry their full names.
- So the crate stops at module resolution, before type checking. The mount file is not mine, so it was not touched.
- Fix: rename those 4 leaf directories, or revert those 20 mount lines.
- My Rust edits are string-literal-only, so they carry no type risk. The `create-material` `include_str!` path is unchanged.
- Re-run `cargo check -p semio-s-artifact-energy-model` after the mount is consistent.

### 7.4 Integer references — prepared, not applied

W1-D's integer-id `Reference` support is visible in flight in the working tree:

- TS `valueSchema` now says "a reference is a string or integer id or an array of them", and adds `REFERENCE_ID_INTEGER_MAX`
  and `referenceIdValue`.
- The Rust message changed as well.

No landing message has arrived, so nothing was switched. The annotator already carries the mapping behind a flag. With
`--references` (dry run `--check --references`: 268 leaves would change):

- Every addressed `id` of a non-create leaf and every entity reference field becomes `widget: reference`, `role: target`.
  The numeric `step` and `precision` are dropped, because the new reader refuses number keys on a reference.
- `ref` values:

  | Input | `ref` |
  |---|---|
  | addressed `id` | `{kind: <entity camelCase>, domain: "energyModel", granularity: <domain granularity>}` |
  | `zoneId(s)`, `terminalZoneIds` | `zone` |
  | `spaceId(s)` | `space` |
  | surface ids | `surface` |
  | construction ids | `construction` |
  | `materialId` | `material` |
  | `layerMaterialIds` | `[material, glazingMaterial, gasMaterial]` in the domain |
  | every `*ScheduleId` and `dailyScheduleIds` | `schedule` |
  | `targetEquipmentId` | `idealLoadsSystem`/`hvac` |

- Entities with no granularity in the `energyModel` interaction domain get `{kind}` only: air loop, PV, battery, plant
  component, generator, fault, setpoint control, and so on.
- Created ids and plain node numbers stay numeric.

To apply once you confirm the landing:

```
.venv/bin/python 🧪️w2-r-energy-annotate.py --references
```

Then re-run the strict lint and both checkers.

## 8. Integer references applied (after W1-D landed `ReferenceIdType::Integer`)

This supersedes the stepper fallback of §3.1 and the "prepared" status of §7.4.

**Annotator.** `🧪️w2-r-energy-annotate.py` now emits integer references unconditionally. The `--references` flag and the
stepper path for ids were removed, and a default run is a no-op (`--check`: changed=0). 268 leaves changed.

**What became a reference.** 340 inputs are now `widget: reference`, `role: target`, with the reader output
`{kind: "reference", kinds, domain?, granularity?, many?, idType: "integer"}`. The number keys (`step`, `precision`) were
dropped, as the reader requires. Breakdown by `ref`:

| `ref` kind | Granularity in `energyModel` | Inputs |
|---|---|---|
| zone | zone | 42 |
| schedule (every `*ScheduleId`, `dailyScheduleIds`) | schedule | 39 |
| fenestration | fenestration | 18 |
| surface | surface | 18 |
| material | material | 11 |
| idealLoadsSystem (incl. the fault target) | hvac | 10 |
| construction | construction | 9 |
| space | space | 7 |
| thermostat | thermostat | 6 |
| glazingMaterial | glazingMaterial | 6 |
| shadingSurface | shading | 4 |
| gasMaterial | gasMaterial | 3 |
| people, lighting, equipment gains and infiltration | load | 34 |
| annual, daily, weekly, time-series and constant schedules (addressed ids) | schedule | 19 |
| layer lists `[material, glazingMaterial, gasMaterial]` | domain only | 2 |
| `{kind}` only, no granularity in the domain (air loop, plant loop, PV, battery, humidistat, zone equipment, fault, setpoint control, zone group, …) | — | 112 |

**What stayed numeric.** Created ids and plain node numbers (`supplyNodeId`, `returnNodeId`, `outdoorNodeId`, `nodeIds`,
`linkIds`) remain numeric.

**Verification (all run).**

| Command | Result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under "✏️s/🔌️plugins/🔋️energy"` (strict) | **exit 0**, 738/738 declared, **0 findings** |
| `.venv/bin/python 🧪️w2-r-energy-check.py` | exit 0: 580/580 fixtures valid, 6/6 corpus verdicts, 744 annotations valid against `InputUi`, 0 failures |
| `bun 🧪️w2-r-energy-check-inputs.ts` | exit 0: 738 inputs through the whole-leaf reader. Control mix: reference 340, stepper 250, text 53, slider 48, select 17, toggle 17, segmented 8, dial 5. `parseInputUi` accepts all 744 annotations, strict Ajv compiles all 291 leaves, Ajv 580/580 fixtures, 0 failures. |
| `bun ./📜️script.ts schema --under "✏️s/🔌️plugins/🔋️energy" --json` | 14 diagnostics before and after, **0 new** |

**Still open.**

- The energy crate `cargo check` remains blocked by the peer's staged mount-path edit (§7.3). Re-run it once that is
  consistent.
- The schema-catalog hashes need a central `schema generate` (§5.3).
