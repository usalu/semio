# WP-20 energy bridge: study (agent `w2-wp20-energy-bridge`)

T = ticket folder, S = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, E = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model`.

## 1. What the energy artifact is

* Artifact `s.energy.model@1/*`. Its authored snapshot is `EnergyModelSnapshot { schema = "energy.model", model: Model, structure, zones, referenced_model, weather_link }`.
  `model` is the typed building energy model (`Model`, engine module `🔨️modules/⚡️simulation/⚙️engine/🔋️model`, SI units): `site`, `zones`, `spaces`, `surfaces`, `fenestrations`, `materials`, `glazing_materials`, `gas_materials`, `constructions`, `people`, `lighting`, `equipment`, `thermostats`, `ideal_loads`, `infiltrations`, `mechanical_ventilations`, `thermal_enclosures`, `adjacency_pairs`, `ground_temperature`, `run_period`, `schedules` ... (45 members, all always present).
* `structure` / `zones` are two *derived* child handles (`energy-value`, `energy-table`), re-derived from `model` on every whole-document load (`energy_children_from_model`, `energy_genesis_child_pack`). Nothing of the model lives in them.
* Snapshot carriers: native DSL text `.dsl.semio` (`semio energy.model.dsl v1`, a `DslRecord` print of `EnergyModelPackRecord { schema, model: DslValue, structure, zones, referenced_model, weather_link }`), native pack (binary), `s.stdio.json@rfc8259` (whole snapshot as `ToValue`, camelCase members `schema model structure zones referencedModel weatherLink`), `s.stdio.zip`, epJSON export (EnergyPlus 25.2).
* The JSON Schema facet (`E/.../🧬️schema/📸️snapshot/🔣️.json`, draft-07, `additionalProperties: false`, required `schema model structure zones`) types `model` as a bare `object`; the field layout of `model` is only given by the Rust types and the TS facet `🧬️schema/📸️snapshot/⚡️model/🟦️.ts`. Examples: `🖼️assets/🏛️bestest-600/🗣️.dsl.semio` and the serde form `🧫️fixtures/🏛️bestest-600/🔋️model.json`.
* Conventions needed for a legal model: ids are `u32` (`EntityId`, `ScheduleId`), zone names unique, every surface references a zone and a construction, an `Interzone(partner)` partner must exist, every `ScheduleId` must resolve in `schedules` (constants -> annual -> weekly -> daily lookup order), construction layers are listed **outside first**, a fenestration's polygon must lie in the plane of its host (1 mm), polygons are counter-clockwise seen from outside (floor normal down), SHGC in 0..1, U > 0.
* Engine facts used for the mapping: boundary `OutdoorAir | Ground | OtherSideTemperature | Adiabatic | Interzone(partner)`; `Floor`/`Ground` classes are treated as floors by the radiant distribution, `Roof`/`Ceiling` as ceilings; ground boundary uses the monthly `ground_temperature` (no ground factor); gains are per floor area of the zone (taken from its floor surfaces).

## 2. How other plugins bridge artifacts (precedent)

* Domain artifacts export only into stdio dialects (checked: every `const INTO` outside `🗄️stdio` is `s.stdio.*`).
* Cross-artifact crate links exist but are rare and mostly optional (`📕️norm/*` -> `🏗️fem` optional feature, `📏️layout` -> `📋️forms`, `🏭️process` extensions -> `process3d`, `📐️cad` extension -> `cad`, `🌎️hub` compositions link plugins). Energy itself pins `serde`/`serde_json` at runtime ("architecturally pinned") and compiles the whole engine tree.
* The registry key of an io hop is `(from, into)` and a second entry with the same key is a hard `Duplicate` error. BIM already owns `s.stdio.json@rfc8259/*` (diagnostics), `s.stdio.csv`, `ifc2x3`, `ifc4`, `gltf`, `svg`, `pdf`. Stdio json subsets are declared profiles (`base`, `geojson`, `i-json`), so inventing an `energy` subset of stdio json would mean editing the stdio artifact.

## 3. Decision

1. **No runtime dependency of the BIM crate on the energy crate** (AGENTS.md: no runtime dependencies on external libraries; energy drags in `serde`, `serde_json`, the engine tree, epw/xlsx/zip). The bridge owns a typed *target mirror* of the subset of `Model` it writes (`EnergyModel` with the same field names and derives, so the `ToValue`/DSL value tree is the same), schema-first: the target contract is a JSON Schema (`S/🚪️io/📤️export/🔋️energy/🔣️.json`) plus the energy snapshot facet.
2. **Hop**: `ModelIntoEnergy` (`Serializer<ModelSnapshot>`), `INTO = s.energy.model@1/*` (the free and semantically right key: BIM -> energy). Payload is the artifact's own text carrier (DSL text, the `.energy` file the energy editor opens) built with `semio_framework_dsl_record` (`EnergyDocument` mirrors `EnergyModelPackRecord`, envelope `energy.model.dsl v1`), fidelity `Lossy` (drops listed below). The same value also renders as the **snapshot JSON** (`energy_json`, members as the energy `s.stdio.json` export writes them) which is what the third-party oracle validates and what the energy artifact's JSON import reads.
3. **Reads only** `ModelInference.energy_envelopes` and `energy_totals` (surfaces, areas, azimuth, tilt, boundaries, adjacency, U-values, polygons) plus the snapshot for what the inference does not carry: layer stacks and material data, window/door thermal data, `space_conditions`, zones, site, project, building rotation. No geometry is recomputed. The layer stack of a floor/ceiling is found by matching the ISO 6946 transmittance of the candidate slab/roof types against `EnvelopeSurface::u_value` (the inference does not name the type: need for the inference owner, see report).
4. **Progress and cancellation**: staged writer `StagedEnergy` (`STAGES` table like the IFC `StagedExport`: `fraction()`, `stage()`, `step(model, inferred)`), a cancelled job simply drops it between two stages. The editor owns the `exportModel` command (not mine): needs written into the report.
5. **Verification**: Rust unit tests; dev-dependency test with the real energy crate only if it compiles in the BIM workspace (parse the DSL text with `EnergyModelSnapshot::parse_dsl`, deserialize the JSON, `Model::validate()`); python oracle: `jsonschema` (draft-07) against both schemas, shapely/numpy recomputation of floor areas, volumes (signed tetrahedra), areas by sector, ISO 6946 U-values; `.feature` scenario.

## 4. Mapping (BIM -> energy `Model`)

| BIM | energy | loss / closest construct |
|---|---|---|
| project name | `name`, `version = "semio-bim-1"` | |
| site (first exported building) | `site` lat/lon/elevation, `time_zone_hours = round(lon/15)`, `north_axis_deg = (true_north - rotation) in degrees mod 360` | time zone is not authored; one site per model, buildings with another bearing are not exported (note) |
| space (each, conditioned or not) | `Zone` (one per space: set points are per space), `Space` row (floor area) | a BIM zone is a grouping, not a thermal node |
| BIM zone | `ThermalEnclosure { zone_ids }` | |
| `EnvelopeSurface` wall/floor/ceiling | `Surface` (class + `OutsideBoundary`), polygon as inferred | curtain walls and surfaces without U-value are not exported (note per element) |
| boundary Exterior / Ground / Adjacent / Adiabatic | `OutdoorAir` / `Ground` / `Interzone(partner)` + `AdjacencyPair` / `Adiabatic` | ground factor 0.6 has no home (the engine uses `ground_temperature`, 18 C default); unmatched adjacent surface -> `Adiabatic` (note) |
| window | `Fenestration` (u, shgc = g*(1-frame), vlt = shgc, polygon) | frame fraction folded into SHGC; VLT is not authored |
| door | `Fenestration` with shgc 0, vlt 0, door U | no thermal mass of the leaf |
| layers | `Material` per (material, thickness), `Construction` outside first | surface absorptances fixed (0.9 / 0.7 / 0.7), roughness MediumRough |
| occupancy density (space, else its BIM zone) | `PeopleGain` + activity constant | 120 W per person assumed |
| lighting / equipment density | `LightingGain` / `EquipmentGain` | radiant fractions fixed |
| set points | `Thermostat` + constant schedules + `IdealLoadsSystem` | missing side set to a non-binding bound |
| ventilation rate (L/s per m2) | ideal loads outdoor air per area | not exported for unconditioned spaces |
| schedule name | built-in constant / weekday-window schedules (`... HH-HH`) | other names: always on (note) |
| infiltration | none authored | not exported |
| weather | `weather_link` stays empty | the epw link is chosen in the energy app |

## 5. Needs for other owners
Inference: `EnvelopeSurface` should name the type that supplies its layers (kind + id) so the bridge need not match U-values. Editor: `exportModel` needs an `energy` format using `StagedEnergy`. See the report.
