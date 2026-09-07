# 🧱️ G2 — constructions & materials (`300`–`399`) + internal gains (`400`–`499`)

Lane G2 of `26/09/06/ENERGY-PLUGIN-END-TO-END`. Everything below is emitted from
`🐍️generate-mutation-leaves.py`'s `#region 🔖️G2`; no aggregate file, crate mount, oracle catalog
entry or feature table was hand-edited.

## 1. Reusable spec patterns (for W-D0, the generator custodian)

The 61 kinds are declared through five emitters that live inside the G2 region. Each one emits BOTH
the Rust leaf (`🔺️diff`, `↩️inverse`, payload, descriptor, payload schema) and the Python second
implementation's function + inverse arm from one description, so a collection-element kind is data
rather than prose. They are group-neutral: G1, G3 and G4 address the same kind of
`Vec<Element>`-on-`Model` collection and can lift them out of `#region 🔖️G2` unchanged.

| emitter | shape | refusals it generates |
|---|---|---|
| `g2_scalar` | one numeric field of one id-keyed element | `mutation.target-missing`, `mutation.invariant` (range), `mutation.no-op` |
| `g2_reference` | one reference field (`zone_id`, `schedule_id`, …) | `mutation.target-missing` for the element AND for the referent, `mutation.no-op` |
| `g2_rename` | the element's identity field | `mutation.target-missing`, `mutation.invariant` (blank), `mutation.duplicate-id` (name), `mutation.no-op` |
| `g2_create` | insert one element at a stated `index` | `mutation.duplicate-id`, `mutation.invariant` (index past the end), `mutation.target-missing` (each declared reference) |
| `g2_delete` | remove one element by id | `mutation.target-missing`, `mutation.invariant` (still referenced) |

Every code above is one of `POLICY_MUTATION_FROZEN_CODES`' seven (`📜️script.ts:29117`), so
`verify mutation-outcome-law` has nothing to say about these leaves — see §5.

Two decisions in there are load-bearing rather than stylistic:

- **The create payload carries the element's `index`.** Without it `delete` is not invertible:
  `delete`'s inverse is a `create`, and a `create` that appends restores the element at the wrong
  position for every element that was not last, so `fixtures::assert_inverse` would only pass by
  accident of fixture choice. With `index` the pair is exactly invertible for an element anywhere in
  the collection. The list position is real document data anyway — `Construction::layer_material_ids`
  is ordered outside-to-inside.
- **Every guard is JSON-expressible.** `🦠️mutation/🔣️.json` is a JSON document, so a refusal fixture
  can never carry a NaN or an infinity. That is why the four `ZoneInfiltration:DesignFlowRate`
  coefficients are guarded `nonnegative` (which EnergyPlus's own conventional coefficient sets —
  BLAST `0.606/0.03636/0.1177/0`, DOE-2 `0/0/0.224/0` — satisfy) rather than merely `finite`: a
  `finite` guard's only refusal vector would be a value JSON cannot hold.

`g2_delete` refuses rather than cascades wherever another entity still points at the element
(`delete-material` while a construction lists it, `delete-construction` while a surface names it),
matching the ledger §6 rule for `delete-zone`. A gain, an infiltration object and a mechanical
ventilation object are referenced by nothing, so those deletes have no in-use refusal and their
`⛔️` vector is an absent id instead.

## 2. Ledger delta

Five kinds beyond `📓️mutation-tag-ledger.md`'s reserved rows, all inside G2's own `400`–`499`
range. W-C's `Infiltration` reshape (status log, 05:20) landed after the ledger was written and left
five fields with no mutation able to reach them:

| # | kind | emoji | why |
|---|---|---|---|
| 440 | `change-infiltration-method` | 🔬️ | selects which of `air_exchange::InfiltrationMethod`'s four flow calculations runs |
| 441 | `change-infiltration-design-flow-ach` | 🔄️ | `ScheduledAch`'s only flow parameter (BESTEST's 0.5 ACH) |
| 442 | `change-infiltration-effective-leakage-area` | 🕳️ | `EffectiveLeakageArea`'s area |
| 443 | `change-infiltration-discharge-coefficient` | 🚰️ | `EffectiveLeakageArea`'s orifice coefficient |
| 444 | `change-infiltration-stack-height` | 🏭️ | `WindAndStack`'s height |

## 3. Two edits outside the G2 region (both additive, logged for their owners)

1. `🔨️modules/⚡️simulation/⚙️engine/🔄️air_exchange/🦀️.rs`: `InfiltrationMethod` gained
   `dsl::DslScalar`. `dsl::MutationLeaf` requires every payload field to implement `dsl::DslField`;
   without the derive no mutation payload can carry the enum at all, so `change-infiltration-method`
   and `create-infiltration` would both have been impossible.
2. The generator's four type maps (`JSON_TYPES`, `PROTO_TYPES`, `TS_TYPES`, `GRAPHQL_TYPES`) gained a
   `crate::air_exchange::InfiltrationMethod` row each, beside the `RoomAirModelType` rows W-D0
   pre-seeded for the other groups.

## 4. Kind roster

`300`–`315` constructions & materials (16), `400`–`444` internal gains (45). Two committed vectors per
kind — one `✅️applies` that moves the document, one `⛔️refuses` — 122 fixture cases.

| range | entity | kinds |
|---|---|---|
| 300–309 | `Material` | create, delete, rename, thickness, conductivity, density, specific-heat, thermal/solar/visible absorptance |
| 310–315 | `Construction` | create, delete, rename, add-layer, remove-layer, reorder-layers |
| 400–408 | `PeopleGain` | create, delete, zone, schedule, activity-schedule, people-per-area, sensible/latent/radiant fraction |
| 409–416 | `LightingGain` | create, delete, zone, schedule, watts-per-area, radiant/visible/return-air fraction |
| 417–423 | `EquipmentGain` | create, delete, zone, schedule, watts-per-area, radiant/latent fraction |
| 424–432, 440–444 | `Infiltration` | create, delete, zone, schedule, flow-per-exterior-area, the four A/B/C/D coefficients, method, design-flow-ach, ELA, discharge coefficient, stack height |
| 433–439 | `MechanicalVentilation` | create, delete, zone, schedule, design-flow, fan-total-efficiency, fan-delta-pressure |

Fixture seeds are real ANSI/ASHRAE 140 §5.2 numbers where the standard states them: the plasterboard
(`0.012 m`, `0.16 W/m·K`, `950 kg/m³`, `840 J/kg·K`) and fibreglass quilt (`0.066 m`, `0.04`, `12`,
`840`) layers of the case-600 lightweight wall, the 200 W / 48 m² = `4.1667 W/m²` equipment gain at
60 % radiative, and the 0.5 ACH `ScheduledAch` infiltration.

## 5. Verification

| gate | command | result |
|---|---|---|
| generator audit | `python3 T/🐍️generate-mutation-leaves.py` | **green** — `276 kinds, 552 fixture cases emitted`, no ledger drift in `300`–`499` |
| leaf payload schemas, third-party | `uv run --group test python3 T/🐍️validate-mutation-payload-schemas.py` | **green** — `jsonschema` 4.26 (Draft-07): 252 leaf schemas valid, every `title` a branch of the aggregate union. Payload validation waits on fixture materialization |
| Python second implementation | `uv run --group test python3 T/🐍️verify-g2-python-second-implementation.py` | **green** — `24 G2 python vectors: outcome + inverse law hold`, `24 G2 payloads validated against their leaf schema by third-party jsonschema` |
| aggregate union discriminator | same run | **red, and not G2's to fix** — see §7 |
| `mutation-outcome-law` | `bun nx run workspace:mutation-outcome-law` | **red repo-wide, 110 breaches, none of them energy's** — and that is itself a finding: the scan reads `🧬️mutations/<dir>/🦀️.rs` only, while every energy leaf keeps its diagnostics in `<dir>/🔺️diff/🦀️.rs`, so the gate cannot see this plugin at all |
| `verify-taxonomy-report` / `-enforce` | `bun nx run workspace:verify-taxonomy-report` | **blocked before it inspects anything** — `Normalization requires an explicit repository-boundary decision before authored classification: ♻️mit-bestand/🔎️recherche`, a nested git repository in the working tree that is not in `git ls-files`. Not energy's, not touched |
| Rust `cargo check --lib --tests` + `SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test` | see §6 | **queued** — the shared `target-energy-e2e` lock (host thrashing, see §6) |

The Python check is not a smoke test: it runs one `✅️` and one `⛔️` vector through EVERY emitter
(`g2_scalar`, `g2_reference`, `g2_rename`, `g2_create`, `g2_delete`) plus each of the three
hand-written construction-layer kinds and `change-infiltration-method`, asserting the exact refusal
code, that a refused step leaves the document untouched, that an applied step moves it, and that
`invert()` lands back on BASE with no undo step itself refused. Because all 61 kinds are emitted from
those emitters, a green run there is a statement about the emitters, not about one kind. It also
demonstrates the indexed create/delete pair concretely: `delete-material{id:1}` removes the element
at index 0 of a two-element list and its inverse puts it back at index 0, which an appending `create`
could not do.

## 6. Log

- Spec table landed: 61 G2 kinds, generator re-run clean, the generator's own `audit()` green
  (unique ledger numbers, unique emoji, exactly one U+FE0F each, NFC, `camel(slug) == variant`).
  With G1, G3 and G4 alongside, the shared table is now 276 kinds / 552 fixture cases.
- `📓️mutation-tag-ledger.md` reconciled for `300`–`499`: 56 reserved rows given their landed emoji
  and `LANDED (G2)`, five new rows (`440`–`444`) inserted. The generator's own `report_ledger_drift`
  now prints nothing for this range.
- A peer sweep (W-D0's) folded the whole vocabulary onto the seven frozen outcome codes while this
  lane was working: G2's `mutation.duplicate` → `mutation.duplicate-id`, and both
  `mutation.invalid-payload` and `mutation.target-in-use` → `mutation.invariant`. Because both the
  Rust leaf and the Python second implementation are emitted from the same spec row, the two sides
  moved together and cannot drift.

## 7. Finding: the aggregate union's `mutation` discriminator is the wrong spelling

`🧬️mutations/🦀️.rs` carries `#[value(tag = "mutation", rename_all = "camelCase")]`, so the wire
discriminator is the lowerCamel variant spelling. `🟦️.ts` agrees (`readonly mutation: "renameModel"`)
and so does the Python oracle's own `unwrap`/`wire_tag`, whose docstring says so in as many words.
`aggregate_schema_json()` disagrees: it emits `"mutation": {"const": "<kebab catalog id>"}`.

Measured, not inferred — all 24 G2 payloads pass their own leaf `🧬️.schema.json` under third-party
`jsonschema`, and all 24 fail the aggregate union on that `const` alone:

```
  ⚠️ aggregate union discriminator drift on 24 of 24 payloads, e.g.
     ChangeMaterialThickness: union says 'change-material-thickness', the `mutation` tag is 'changeMaterialThickness'
```

It affects all 276 kinds, not only G2's, and nothing has caught it because no `🦠️mutation/🔣️.json`
has been materialized yet — every one still holds the generator's `{}` seed. The moment
`SEMIO_ENERGY_WRITE_FIXTURES=1` writes real bytes, every union validation goes red at once. The fix
is one line in `aggregate_schema_json()`, which belongs to W-D0; G2 recorded it in `📓️status.md`
and did not touch it.
