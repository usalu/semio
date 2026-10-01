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

## 9. Follow-up: invariant refusals are negative witnesses (W2-S F16)

Starting point: `schema mutation-payloads --census --under ✏️s/🔌️plugins/🔋️energy` reported **149 `negative` findings**.
These were fixtures the Rust diff refuses with `mutation.invariant` while the leaf schema accepts them.

| Lint | Before | After |
|---|---|---|
| `schema mutation-payloads` | 431/580 clean, 149 findings | **580/580, 0 findings**: 134 negative witnesses, of which 130 are rejected by the schema itself and 4 are covered by a declared `x-semio-invariant` |
| `schema mutation-inputs` (strict) | 0 findings | **0 findings**, 738/738 |

### 9.1 Kind 1: ranges the leaf diff enforces are now hard bounds

**Principle.** The annotator (`🧪️w2-r-energy-annotate.py`, tables `RULES`, `CONDITIONS` and `INVARIANTS`) now states in each
leaf schema exactly what that leaf's own diff refuses as a payload-intrinsic `mutation.invariant`. This is per-leaf parity with
the Rust code, and it reverses the §3.2 decision now that the refusal fixtures count as negative witnesses. Coverage is
~140 leaves.

**Rule shapes.**

| Rule | Schema |
|---|---|
| positive | `exclusiveMinimum: 0` |
| non-negative | `minimum: 0` |
| fraction | 0–1 |
| efficiency | (0, 1] |
| temperature ranges | −100…300 °C (plant loop), −100…200 °C (ideal loads), 0…100 °C (hot-water setpoint) |
| site | latitude ±90, longitude ±180, time zone −12…14 |
| angles | tilt 0–90, azimuth 0–360 |
| calendar | month 1–12, day 1–31; weekday slot 0–6 |
| non-zero integers | multipliers, counts, priority, node ids, timestep, plant component ids: `minimum: 1` |
| exact list lengths | 12 monthly ground temperatures, 24 hourly values, 7 weekly days; time series ≥ 1 value; polygons ≥ 3 vertices |
| fenestration polygon | empty or ≥ 3 vertices (`anyOf`) |
| names | non-blank via `pattern` |
| artifact reference URIs | `pattern ^[^!]+![^@]+@[\s\S]+/[^/]+$`, the exact `ArtifactRef::parse_uri` grammar |
| setpoint control law | `enum [Scheduled, OutdoorAirReset, WarmestZone, ColdestZone]`, now a `select` with de/en options; the TS twin became the same union |

The name pattern is the exact complement of Rust's `char::is_whitespace`, so it matches `str::trim().is_empty()`.

**Cross-field rules draft-07 can state.** These go in the root `allOf` as `if/then/else`:

- ideal-loads capacity present ⇔ capacity > 0, absent ⇔ 0;
- setpoint limits are 0 unless the law is OutdoorAirReset;
- no setpoint schedule ⇒ schedule id 0;
- Interzone boundary ⇔ integer partner surface;
- schedule limits: both or neither;
- airflow network absent ⇒ empty lists.

The W1-D reader ignores `if/then`, and the strict Ajv oracle compiles them because every branch declares its `type`.

**UI coherence.**

- Soft ranges stay inside the hard bounds. Efficiency sliders start at 0.001 because an exclusive 0 cannot be a soft minimum.
- Glare limit: slider 0.01–1, as a simplified 0–1 index. **Settled by the Rust code**: `daylight::simplified_glare_index` is
  0–1.
- Humidistat throttle ranges: an RH fraction shown in %. **Settled by the Rust code**: `controls::HumidistatSpec` compares
  0–1 RH.

  The committed humidistat fixtures still carry 5.0 and 10.0. These are valid (> 0) but act as on/off bands; this is a realism
  note, not a defect.

**Leaves without bounds.** Create leaves whose diff does not check a value get no bound. Examples: `create-material`,
`create-people-gain`, `create-pv-system`, `create-battery`, `create-fault`, and `name` in `create-construction`,
`create-space-list`, `create-thermal-enclosure` and `create-electrical-load-center`. **Rust follow-up:** these creates should
enforce the same invariants as their change siblings. Once they do, the rows go into `RULES`.

### 9.2 Kind 2: state-dependent refusals recoded

`🧪️w2-r-energy-recode.py` is idempotent. It updated three places:

- **Rust diffs:** 46 files.
- **Python second implementation:** 55 edits in `🏛️mutate-energy-model-1/🐍️.py`.
- **Committed outcomes:** 15 of the fixtures that witness these refusals.

| Refusal (depends on the base document) | Was | Now |
|---|---|---|
| delete while still referenced (13 leaves: zone, space, surface, material, construction, air loop, PV, battery, 5 schedule kinds) | invariant | `mutation.target-referenced` (fem precedent) |
| index past the end: 27 create/add/insert leaves, `remove-construction-layer`, `reorder-annual-schedule-rules`, holiday/rule index | invariant | `mutation.target-missing` |
| `reorder-construction-layers` list is not a permutation of the held layers | invariant | `mutation.id-mismatch` |
| `replace-fenestration-vertices` polygon off the host surface's plane | invariant | `mutation.target-mismatch` |

The `delete-zone` docstrings (Rust and Python) were updated to name the new code.

### 9.3 Kind 3: payload-intrinsic rules draft-07 cannot state

These are declared in the leaf root's `x-semio-invariant`, which uses the manifest `$defs/SchemaInvariants` shape
`[{id, description{en,de}}]`:

| Id | Leaves |
|---|---|
| `equipment-ids-ascending` | create-plant-loop |
| `terminal-zone-ids-ascending` | create-air-loop |
| `distinct-surfaces` | connect-surfaces |
| `zone-node-pairs` | replace-airflow-network |
| `outdoor-reset-range` | create-setpoint-manager, replace-setpoint-manager-kind |
| `limits-ordered` | create-daily-schedule, change-daily-schedule-limits |

The 4 witnesses (jumbled plant loop, jumbled air loop, self-pair, unpaired nodes) name their id in the committed outcome
(`"invariant": "<id>"`).

- **Python:** `rejected(code, path, invariant=None)` produces the id at all 8 sites.
- **Rust:** a produced `MutationOutcome` has no slot for the id. The energy fixture harness
  (`🧬️mutations/🧪️tests/🔬️fixtures/🦀️.rs`) therefore handles it three ways:
  - `assert_outcome` compares the outcome without `invariant`;
  - it asserts that the produced code is `mutation.invariant` and that the leaf's own `Mutation::input_schema` declares the id;
  - `write_when_requested` preserves a committed `invariant`.

### 9.4 Pre-existing blockers fixed on the way

- **Peer mount rename completed.**
  - A staged peer change had pointed the crate mount (`🗿️artifacts/🔋️model/🦀️.rs`), the leaf tests and the descriptors at
    shortened directory names. The 4 over-long leaf directories themselves had not moved; their paths were 256–261 bytes,
    above the 240-byte limit.
  - They are now moved (`mv`) to the short names, which match the already-renamed fixture directories:
    - `🏝️change-humidistat-dehumidifying-setpoint`
    - `🧻️change-humidistat-dehumidifying-throttle`
    - `🔴️change-ideal-loads-system-max-heating-supply`
    - `🔵️change-ideal-loads-system-min-cooling-supply`
  - The remaining long-name references in `DIRECTORIES`, the oracle catalog, the feature file and the leaf `Case.directory`
    strings were updated.
  - The schema catalog was regenerated at the repo root (`bun ./📜️script.ts schema generate`, 3650 scopes), so the lints
    find the moved leaves.
- **No-op outcome class.** The harness `outcome_document` emitted only `applied`/`rejected`, while two committed vectors say
  `no-op` (committed 09-25). It now emits `no-op` for the `mutation.no-op` warning, which is the `applied | no-op | rejected`
  vocabulary the Python oracle already used.
- **Two "applies" vectors that applied nothing.** `change-zone-volume/✅️resizes-zone-one` (129.6 m³, equal to the base) and
  `change-glazing-material-conductivity/✅️applies` (1.06, equal to the base) violated the mutate case's observability law.
  - Their scenarios now really change the document: 158.4 m³, and 1.4 W/(m·K).
  - Both quintets were regenerated by the fixture writer (`SEMIO_ENERGY_WRITE_FIXTURES=1`).
  - That writer run regenerated **all 580 vectors**, and only these 8 files changed. This proves that every other committed
    quintet — including the 19 recoded/renamed outcomes — is exactly what the Rust implementation and harness produce.

### 9.5 Verification (all run, foreground, gated; `cargo test` with `CARGO_INCREMENTAL=0` and a private `target-nde-w2r-energy`)

| Command | Result |
|---|---|
| `schema mutation-payloads --under ✏️s/🔌️plugins/🔋️energy` (strict) | **exit 0**: 580/580 fixtures meet their leaf schema, 134 negative witnesses (4 via `x-semio-invariant`), 291/291 leaves witnessed, **0 findings** |
| `schema mutation-inputs --under ✏️s/🔌️plugins/🔋️energy` (strict) | **exit 0**, 738/738, **0 findings** |
| `.venv/bin/python 🧪️w2-r-energy-check.py` | Now witness-direction aware. 580 fixtures, 134 negatives rejected, 6/6 corpus verdicts, 744 annotations valid against `InputUi`, **0 failures** |
| `bun 🧪️w2-r-energy-check-inputs.ts` | Strict Ajv compiles all 291 leaves, including the `if/then` rules. Ajv judges 580 fixtures, 134 negatives, 0 failures. 738 reader inputs, `parseInputUi` accepts all 744 annotations. |
| `cargo check -p semio-s-artifact-energy-model` | **ok**: 2 warnings, both in the peer-owned `⚡️simulation` window. The mount blocker of §7.3 is gone. |
| `cargo test -p semio-s-artifact-energy-model --lib` | **6298 passed, 2 failed, 1 ignored**. See the notes below the table. |
| `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test … writes_the_committed_vector_when_requested …` | 592 passed. It rewrote every vector from its scenario, and only the 8 files of the two repaired vectors changed. |
| `bun ./📜️script.ts parity exhaustive --owner …/✳️any --case 🏛️mutate-energy-model-1` (test domain) | **2297/2298 passed, parity 1148/1149**: every mutate and inverse scenario agrees between Python and Rust. See the notes below the table. |

**Crate tests.**

- The 2 failures are `editor|viewer::…::windows::zones::tests::render_lists_one_row_per_zone`
  (`assert!(table.children.is_empty())`). They are UI render tests in files I did not touch, unrelated to schemas, codes or
  fixtures.
- Every leaf fixture law passes, as does the structural correspondence test with the moved directories.
- The earlier run (before the no-op/observability repair) had 2 more failures: the two no-op vectors.

**Parity.** The one remaining gap is `identity-round-trip`, a `@mode-round-trip` scenario that the Python adapter by design
does not register (`adapter has no oracle registration`). Its scenario and the adapter registration are unchanged.

**Files (this follow-up).**

- **Leaf payload schemas:** 158 were re-rendered by the annotator. The ones that gained schema rules are listed in `RULES`,
  `CONDITIONS` and `INVARIANTS` of `🧪️w2-r-energy-annotate.py`.
- **Rust diffs:** 46 files (`🧬️mutations/*/🔺️diff/🦀️.rs`, codes only).
- **Leaf descriptions:** `🏚️delete-zone/🦀️.rs` (docstring).
- **Energy fixture harness:** `🧬️mutations/🧪️tests/🔬️fixtures/🦀️.rs`.
- **Leaf test scenarios:** `📦️change-zone-volume/🧪️tests/✅️resizes-zone-one/🦀️.rs` and
  `🟠️change-glazing-material-conductivity/🧪️tests/✅️applies/🦀️.rs`.
- **Fixture quintets and outcomes:**
  - 8 regenerated quintet files;
  - 19 outcome files (15 recoded codes, 4 named invariants).
- **Python second implementation:** `🧪️tests/🏛️mutate-energy-model-1/🐍️.py`.
- **Renamed leaf directories (4), and the files that referenced them:**
  - `🧬️mutations/🦀️.rs` (`DIRECTORIES`);
  - `🔮️oracles/🔣️.json`;
  - `🥒️.feature`;
  - the 8 leaf test `Case.directory` strings.
- **TS twin:** `🧬️mutations/🟦️.ts` (setpoint control-law unions).
- **Schema catalog:** `📚️library/🔣️schema-catalog.json`, regenerated.
- **Ticket scripts:** `🧪️w2-r-energy-recode.py` (new); annotate and both checkers updated.

**Checked by reading, not run.** The framework's Rust validator
(`🧬️schema/✅️validator/🦀️.rs::PatternMatcher`, used when a history edit is validated) runs its own regex subset with
unanchored search. That subset parses every construct the two new patterns use: `[^…]` with `\t \n \f \r \uXXXX` escapes and
`\u`-bounded ranges, `[\s\S]`, `^ $` and `+`.

No Rust test validates a payload against these patterns, so this was verified from the parser source only. Python `re`, Ajv
(`u` flag) and npm/Python `jsonschema` did run the patterns, above.

## 10. Outcome-code vocabulary extension (coordinator decision, 2026-09-30)

§9 introduced three new codes. The vocabulary is frozen and enforced at persistence: `🏪️store` `expected_mutation_message_level`
refuses unknown codes, and a persisted message must carry exactly its code's level. Decision, implemented framework-wide in
one change:

### 10.1 Design note (this replaces the ticket 26/08/16 contract-freeze §C2 table)

| Code | Level | Meaning |
|---|---|---|
| `mutation.target-missing` | Error | the addressed target (or position) does not exist in this base |
| **`mutation.target-referenced`** | **Error** | deleting or replacing a target that something else still references |
| **`mutation.target-mismatch`** | **Error** | the payload is inconsistent with the target's current state: a reorder that is not a permutation of the held ids, a polygon off its host plane, a `replace-` whose record renames the target it selects |
| `mutation.no-op` · `mutation.partial` · `mutation.clamped` | Warning | unchanged |
| `mutation.duplicate-id` · `mutation.invariant` | Fatal | unchanged. The payload itself is wrong; the payload-intrinsic ones are stated in the leaf schema or in `x-semio-invariant`. |
| `mutation.cascade` | Info | unchanged |

- The vocabulary is **nine codes, no per-plugin codes**.
- `mutation.id-mismatch` (fem only, Fatal) is gone. It collapsed into `mutation.target-mismatch` at `Error`, per the
  coordinator's decision.
- The two new codes are *state-dependent*: a different base can admit the same payload. That is why they are `Error`, never
  `Fatal`.

### 10.2 Surfaces changed

| Surface | Change |
|---|---|
| Store persistence (`🏪️store/🦀️.rs` `expected_mutation_message_level`) | the two codes at `Error` |
| Store test (`🏪️store/🧪️tests/🔬️unit/🦀️.rs`) | new `spr_round_trip_preserves_state_dependent_error_codes`: each new code survives the `.spr` history round trip (conflict messages), and the same codes at `Fatal` are refused ("malformed mutation message") |
| Replication | `⚔️conflict/🧬️schema/🔣️replay-report/🔣️.json` code enum (+2); `🎮️mutation` `MutationMessage` doc (nine codes) |
| Store replay schema | `🏪️store/🧬️schema/🔣️supersede-replay/🔣️.json` code enum (+2) |
| Gate | `📜️script.ts` `POLICY_MUTATION_FROZEN_CODES` (+2), plus the rule docs and breach texts (7 → 9) |
| Rust labels | `🔌️plugin/⏪️time-travel/🦀️.rs` `history_code_text`: "Target still referenced / Ziel wird noch referenziert", "Inconsistent with the target / Widerspricht dem Ziel" |
| React | `ShellHost` `mutationCodeLabelKey` (+2 cases); `📚️I18n` `UiTranslationSchema.mutation.code` (+`targetReferenced`, `targetMismatch`); de and en bundles in `⚛️react/🌐️i18n/🟦️.ts` (normal and beginner texts) |
| TS twin | `🎠️kernel/🟦️.ts` `MutationMessage` doc (nine codes; `code` is typed `string`, so there is no union to extend) |
| Docs mentioning the count | spr history, db artifact, workflow, I18n, store fixture docs |

**fem, id-mismatch → target-mismatch.** `🧪️w2-r-energy-fem-target-mismatch.py` (idempotent) changed 61 files plus the helper
rename:

- the Rust guard helpers in 2d and 3d: `fatal` → `error`, and the 3d helper `id_mismatch` → `target_mismatch`;
- the level-discipline docs and the leaf diff docs;
- 16 leaf tests and 4 unit tests: level `Fatal` → `Error`, and the 2d `…_rename_is_id_mismatch` tests renamed to
  `…_rename_is_target_mismatch`;
- the 3d `replace_node_rename_is_fatal` test, renamed to `replace_node_rename_is_a_target_mismatch_error`;
- 9 committed outcomes (code, and message level `fatal` → `error`);
- 10 Python second implementations (`TARGET_MISMATCH`, `error(...)`).

**energy.**

- `reorder-construction-layers` → `mutation.target-mismatch` in the Rust diff, the Python second implementation and the
  committed outcome.
- Every energy `mutation.invariant` (197) and `mutation.duplicate-id` (59) refusal was raised at `Error`. These are now `Fatal`,
  the level the persistence table demands; otherwise a replayed or degraded conflict carrying them would fail
  `validate_persisted_message`.
- The committed outcomes are unaffected, because rejections carry no level.
- `🧪️w2-r-energy-recode.py` now also does the level pass.

**Not changed.** No Python oracle enumerates the code set: the fem and energy second implementations use constants per
refusal, and those were updated above.

### 10.3 Verification (all run, foreground, gated; private `CARGO_TARGET_DIR=…/target-nde-w2r-energy`)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-os-kernel --features sync` | ok. The first two attempts failed on a peer's in-flight store edit (`trunk_alternative_id` not yet re-exported from `os_spr`, and the `♻️retirement` macro); the retry once the peer landed was green. |
| `cargo test -p semio-framework-os-kernel --lib --features sync -- spr_round_trip_preserves` | **3/3 passed**, including the new `spr_round_trip_preserves_state_dependent_error_codes` |
| `cargo check -p semio-framework-plugin` (time-travel labels) | ok |
| `cargo test -p semio-s-artifact-fem-2d -p semio-s-artifact-fem-3d --lib` | **fem-2d 1039/1039, fem-3d 957/957**. The first run had 1 failure, 3d `replace_node_rename_is_fatal`, which asserted Fatal; it was renamed and now asserts the Error + target-mismatch contract. |
| fem Python oracles (`bun ./📜️script.ts oracle exhaustive --owner <subset> --case <case>`, 15 fem cases) | **389/389 passed** |
| `cargo test -p semio-s-artifact-energy-model --lib` | **6297 passed, 3 failed**. See the notes below the table. |
| `parity exhaustive … 🏛️mutate-energy-model-1` | **2297/2298, parity 1148/1149**. The only gap is the pre-existing Python `identity-round-trip` registration. |
| `schema mutation-payloads` / `schema mutation-inputs` under energy | **0 / 0 findings**: 580/580 fixtures, 134 negatives; 738/738 inputs |
| `bunx tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json` | 26 errors, **none in the files changed here** (`ShellHost`, `📚️I18n`, the i18n bundles, kernel twin). All 26 are in peer files: engine-contract, extension-retirement, frame-worker, browser-bundle worker, hub-edit-durability. |
| `bun ./📜️script.ts verify mutation-outcome-law` | **Crashes before any verdict, pre-existing.** `policySeverityInfoBreaches` refuses the repo-root `.tmp-ticket` symlink, a peer's since 2026-09-23. Running the 7 rules one by one (`🧪️w2-r-energy-outcome-law.ts`) did not finish rule 1 within 50 min under fleet load, so it was stopped. |
| Code-rule equivalent: rule 2's builder regex over all 23,248 tracked `.rs` files, against the nine-code set | **Only 2 out-of-vocabulary codes, both in `🗄️stdio`** (`stdio.png.patch-pixels.invalid-range`, `stdio.wav.patch-data.invalid-range`), for W3-CODES. energy, fem and the framework show 0. |

**Energy crate failures.**

- 2 are the unrelated zones-window render tests (§9.5).
- 1 is the timing law `sim::tests::energy_job_previews_checkpoints_and_commits_bounded_steps` ("worst energy step was 13 ms"). Rerun alone it **passes** (1/1), so this is load jitter.
- Every fixture law passes with the Fatal levels.

**Open items.**

- The schema catalog hashes of the two changed framework schemas are stale; they need a central `schema generate`.
- `.tmp-ticket` blocks the outcome-law gate for everyone.
- W3-CODES was messaged: vocabulary landed, fem remap already done, energy code edits done.

## 11. The outcome-law gate walks once, and the energy reds (coordinator follow-up, 2026-09-30 → 10-01)

### 11.1 Where the gate stood

W3-CODES had already rewritten rule 2 (outcome codes) onto `git ls-files` with a fixture-backed vocabulary. I coordinated
with it before touching the gate; it approved the plan, including rule 2's file source. Profiling each rule
(`🧪️w2-r-energy-outcome-law.ts`, wall time per rule) showed four problems:

- **Rule 1 never finished.** `policyFindAllMutationsDirs` runs the full taxonomy source admission (`inventoryTaxonomySources`)
  just to find `🧬️mutations` directories: more than 10 min under fleet load. It now also crashes outright on a peer's
  in-flight taxonomy parse (`semanticPathProjectionContracts.artifact-editor-command-bundle-v1`).
- **Rules 3, 4 and 5 each re-walked the repo.** Rules 3/4 use `policyAllRustFiles` (an fs walk into gitignored trees), rule 5
  uses `policyWalkRelFiles`, at 30–54 s per rule. The `.tmp-ticket` crash itself was already gone: a peer's `🚶️file-walk`
  change skips dot-named symlinks. That skip is name-based, though, not a rule.
- **Rule 6 checked stale surfaces.** `📡️spr/🧾️wire/🦀️.rs` no longer exists (the enum lives in `📡️replication/🧾️wire`). The host
  codec `💻️os/🟦️.ts` only carries the ordinal through the kernel's `mergePolicyAsU8`. The React `📦️packages/🟦️typescript/🟦️.tsx`
  is a 3-line re-export; the bundles moved to `⚛️react/🌐️i18n`. Result: 3 false breaches.
- **Rule 7 checked an impossible law.** It required byte identity between the derive owner and its package glue. Since 09-02
  the glue is `#[path = "../../🦀️.rs"] mod component;` (88 packages use that pattern), so this was 1 permanent false breach.

### 11.2 Changes (📜️script.ts, region `🔧️PolicyRuleMutationOutcomeMergePolicy` only)

- **One shared walker, `policyMutationLawInventory(repoRoot)`** (exported, memoized per root):
  - ONE `git ls-files -co --exclude-standard -z` over the positions the rules scan (`*.rs *.ts *.tsx *.py *.feature
    *🎯️outcome/🔣️.json`). git never enters ignored trees and never follows a symlink: it lists a link as one path and
    nothing beneath it.
  - It drops taxonomy `pathExclusions` (`taxonomyRelativePathIsExcluded`), `POLICY_SKIP_DIRS` segments and the router itself
    before any filesystem access.
  - `lstat` keeps regular files only, so a symlink (the root `.tmp-ticket`) or a deleted tracked path is skipped, never read.
  - It returns `{ files, mutationsDirs }`; the `🧬️mutations` directories are read off the files' ancestors.
- **All seven rules read it.**
  - Rule 1 iterates `mutationsDirs`.
  - Rule 2 changed only its file source; W3-CODES' vocabulary and regexes are untouched.
  - Rules 3 and 4 use `policyMutationLawRustFiles`; rule 5 uses `policyMutationLawSourceFiles`, now a slice of the inventory.
  - `policyAllRustFiles` and `policyFindAllMutationsDirs` stay for the other policy rules.
- **Rule 1 false positive fixed.** The return check accepted only the qualified spelling `protocol::MutationOutcome<`, so the
  30 en1990 diffs that `use protocol::MutationOutcome;` and return `-> MutationOutcome<` failed it.
  `policyDiffReturnsOutcome` now accepts the bare spelling after that import.
- **Rule 6 surfaces** are now the 4 files that spell the variants:
  - Rust spine: `📡️replication/🧾️wire`;
  - TS kernel types;
  - the `📚️I18n` label contract;
  - the en+de `🌐️i18n` bundles.

  A surface that no longer exists is reported as a stale path instead of as "all variants missing".
- **Rule 7 is `policyDeriveGlueMountBreaches`.** The package glue must mount the owner with `#[path = "../../🦀️.rs"] mod
  component;` and must not define its own `expand_*`. The macro body then exists once and no build shape can run a stale copy.

### 11.3 Gate verification (all run)

| Command | Result |
|---|---|
| `bun 🧪️w2-r-energy-mutations-dirs-parity.ts` | The admission side crashes on the peer's taxonomy parse, so the inventory was checked against an lstat `find` walk instead (`POLICY_SKIP_DIRS` pruned): **identical, 1950 `🔺️diff/🦀️.rs` leaves in 117 `🧬️mutations` dirs**, 0 only on either side. The inventory took 24.6 s. |
| `bun 🧪️w3-codes-gate-negatives.ts` (W3-CODES' proof, scratch git repo) | **13 reported, missing [], unexpected []** |
| `bun 🧪️w3-codes-outcome-law.ts` (W3-CODES' proof) | **0 breaches** in 15 s |
| `bun 🧪️w2-r-energy-outcome-law.ts` (per rule, before the rule-1 import fix) | Under load average 120: rule 1 55.7 s (the shared walk is paid here), rule 2 54.3 s, rules 3/4/5 17.7/20.1/22.0 s, rules 6/7 0.0 s. 0 crashes. |
| `bun ./📜️script.ts verify mutation-outcome-law` | **A verdict in 95 s: 47 breaches.** All are rule 1 `mutation-migration/outcome` ("never references one of the 9 frozen codes"), all in 📕️norm; rules 2–7 report 0. |

**The 47 findings were sent to W3-CODES**, who owns the code remaps. The W2-W norm groups edit 📕️norm concurrently. Every one
of these diffs refuses nothing:

- remove-* filters without `mutation.target-missing`;
- change-* never says `mutation.no-op`;
- insert-* never refuses a duplicate id and silently clamps its index.

The findings:

- **en1990 (6):** insert-permanent, insert-seismic, insert-accidental, insert-effect, insert-variable, insert-member
- **en1997 (3):** insert-footing, insert-layer, insert-pile
- **din16798 (4):** insert-zone, remove-zone, insert-vent-system, remove-vent-system
- **en1992 (2):** insert-anchor, remove-anchor
- **en1993 (16):** insert-bridge-fatigue, insert-cold-formed-member, insert-crane-runway, insert-fatigue-detail,
  insert-fire-exposure, insert-joint, insert-load-case, insert-material, insert-member, insert-member-action, insert-pile,
  insert-plated-panel, insert-section, insert-silo-shell, insert-tension-component, insert-tower-leg
- **en1996 (4):** change-annex, change-design-situation, change-storeys, change-masonry-class
- **en1995 (2):** insert-connection, insert-member
- **en1999 (1):** add-member
- **en1998 (9):** insert-building, insert-bridge, update-site, insert-assessment, insert-tower, insert-tank,
  insert-retaining-wall, insert-foundation, insert-silo

### 11.4 Energy reds

**Zones-window render tests: pre-existing, not caused by this ticket; the stale part was in energy and is fixed.**

- `editor|viewer::…::windows::zones::tests::render_lists_one_row_per_zone` asserted `table.children.is_empty()` for an empty
  document.
- Since `6b8089dcb21` (2026-09-27), the framework `TableScene` always pages `columnsJson`/`rowsJson` into two lane carriers:
  `split_lanes`, `framework.scene.table.{columns,rows}`. So every table surface has exactly 2 children. That change is
  intended (`table_kit_pages_large_tables_without_losing_rows`).
- The energy tests were last touched 09-09/09-16 and asserted the old shape. Their name said "one row per zone" while they
  checked "no children".
- Both tests now read the rows the way the hosts read them: `artifact_app_laws::built_surface_scene::<TableScene>` merges
  the lanes back. They assert an empty model gives 0 rows, and a two-zone model gives rows `1 Ground floor` and `2 Attic` in
  order.
- Files: `✏️editor/🎭️modes/✏️edit/🪟️windows/📊️zones/🧪️tests/🔬️unit/🦀️.rs` and the `👁️viewer` twin.

**`identity-round-trip` now has a Python oracle.** `🐍️.py` gained a region `🔖️TextCarrier`, written from the carrier text:

- a tokenizer;
- a reader for records, lists, strings, atoms and `null`, refusing repeated member names;
- the inline child handle (`child_id=… target="…"`), expanded to the vector shape, which requires the target's artifact id to
  be the child id;
- a printer.

The handler reads `asset://🎬️demo/🗣️.dsl.semio` and requires all of the following in role:

- print → read → print is a fixpoint;
- the printer reproduces the committed carrier's token stream, numeric atoms compared by kind and value;
- the model carries zones and surfaces.

It then answers the document. It refuses a present `referenced_model`/`weather_link` link slot instead of guessing its
spelling, because no committed asset spells one. The Rust registration doc and the feature description now name this oracle.

Checks on the reader:

- All 15 committed energy carriers (`🎬️demo` and 14 BESTEST cases) round-trip and reproduce their token streams.
- The parsed demo has exactly the committed JSON vector's key set and value types.
- Python oracle phase (`bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts oracle exhaustive --owner …/✳️any --case
  🏛️mutate-energy-model-1`): **1149/1149 passed**, up from 1148 registered.

### 11.5 Rust verification

Pending a buildable workspace. While this section was written, peers had the dependency closure mid-edit:

- the workspace manifest (`🌎️hub/🧩️compositions/🗄️stdio`);
- `store::ArtifactCodec::of` now demanding `ArtifactSqliteSnapshot` from every stdio snapshot;
- `semio-framework-plugin`'s `🛠️tool-machine` (`tool_machines` field).
