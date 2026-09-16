# Explore: energy model geometry + results, for a 3D model-tree inspector

Scope: what geometry/results the 🔋️energy model carries today, and the exact gaps to (a) render
walls/floors/roofs/ceilings, arbitrary window polygons, shading surfaces, zone volumes in 3D and
(b) colour surfaces by transmission/conduction loss or solar gain. All paths repo-relative under
`/Users/ueli/Documents/semio`.

## Headline finding

There is **no 3D viewport anywhere in the energy plugin today**. The only visual surface
(`👁️viewer/🎭️modes/👁️view/🪟️windows/🌳️structure/🦀️.rs`) renders a plain text `TreeView` of entity
*counts* per category ("materials: 4", "zones: 1", …) — no coordinates, no colour, no canvas. A 3D
model-tree inspector is a new capability, not an extension of an existing one.

---

## 1. Geometric fields, units, conventions, coordinate system

All geometry lives in `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs`.

- **Coordinate system**: right-handed, **z is up**. Confirmed by `⚙️engine/📐️geometry/🦀️.rs:76`
  (`tilt_deg = acos(n[2])`, i.e. tilt is measured off the world Z axis) and by
  `⚙️engine/🏛️bestest/🦀️.rs:203-204` docstring: *"x runs west→east, y south→north, z up; every
  polygon is wound counter-clockwise seen from OUTSIDE"*. Azimuth is clockwise from **north = +Y**,
  computed as `atan2(n[0], n[1])` then rotated by `north_axis_deg` (`📐️geometry/🦀️.rs:74-83`).
- **Winding/normal**: outward normal via Newell's method (`📐️geometry/🦀️.rs:55-69`,
  `⚡️epjson/…/🦀️.rs:174-187` reimplements the identical Newell sum for the exporter). Convention:
  every polygon is wound **counter-clockwise seen from outside the zone**, so the normal points
  away from the zone interior — this is also what epJSON's `GlobalGeometryRules` declares
  (`Counterclockwise`/`World`, exporter module docstring lines 29-33).
- **`Surface`** (`🔋️model/🦀️.rs:300-312`): `vertices_m: Vec<[f64; 3]>` (arbitrary planar polygon, ≥3
  verts, world meters), `class: SurfaceClass` (ExteriorWall/InteriorWall/Roof/Ceiling/Floor/
  Interzone/Adiabatic/Ground), `construction_id: EntityId`, `outside_boundary_condition:
  OutsideBoundary` (OutdoorAir/Ground/OtherSideTemperature/Adiabatic/Interzone(EntityId partner)),
  `sun_exposed: bool`, `wind_exposed: bool`, `multiplier: u32`, `zone_id: EntityId`.
- **`Fenestration`** (`🔋️model/🦀️.rs:389-407`): **no polygon field today**. Carries `surface_id`
  (host wall), `area_m2`, `height_m`, `sill_height_m` (all scalar), `u_value_w_m2k`, `shgc`, `vlt`,
  `frame_conductance_w_k`, `divider_conductance_w_k`, four shading-projection scalars
  (`overhang_depth_m`/`overhang_offset_m`/`fin_depth_m`/`fin_offset_m`), and optional
  `glazing_construction_id: Option<EntityId>`. It is positioned entirely by its host `Surface`'s
  polygon plus these scalars (see §2).
- **`ShadingSurface`** (`🔋️model/🦀️.rs:687-693`): already has `vertices_m: Vec<[f64; 3]>` — free
  polygon, no zone/class, plus optional `transmittance_schedule_id`. Fully 3D-render-ready today.
- **`Zone`** (`🔋️model/🦀️.rs:266-273`): `volume_m3: f64` (a **scalar**, not derived from surfaces —
  see below), `multiplier`, `conditioned`, `part_of_total_floor_area`. No geometric shape of its
  own; a zone is only implicitly a volume through the `Surface`s that reference its `zone_id`.
- **`Space`** (`🔋️model/🦀️.rs:277-282`): `zone_id`, `floor_area_m2: f64` — also a scalar, not
  derived, and carries no polygon/footprint at all.
- **`AdjacencyPair`** (`🔋️model/🦀️.rs:713-716`): just `{surface_a_id, surface_b_id}` — a bookkeeping
  pair alongside `OutsideBoundary::Interzone(partner_id)`; no separate geometry.
- **Zone volume / floor area vs. surfaces**: `Zone.volume_m3` and `Space.floor_area_m2` are
  **independent user-entered scalars**, never computed from the surface set at model-build time.
  The engine *can* derive true watertight volume from surfaces
  (`geometry::zone_volume_from_surfaces`, `📐️geometry/🦀️.rs:87-106`, pyramid-sum-to-centroid) and
  floor/exterior/roof area per zone (`precompute::ZoneGeometry`, accumulated in
  `🧠️precompute/🦀️.rs:492-500`), but these derived numbers are used only for *engine-internal
  physics bookkeeping* (`ZoneGeometry.floor_area_m2/exterior_area_m2/roof_area_m2` inside
  `PrecomputedModel`), not written back to `Zone`/`Space`, and not exposed to any UI today.
- **Planarity/area/normal helpers**: `surface_area_m2` (cross-sum, `📐️geometry/🦀️.rs:40-52`),
  `polygon_normal` (Newell, `:55-69`), `validate_polygon_planar` (tolerance check, `:136-155`),
  `transform_vertices`/`transform_direction` (4×4 building↔world, `:160-167`) — all generic over
  `&[[f64;3]]`, reusable as-is for fenestration polygons once they exist.

---

## 2. Fenestration polygon derivation today, and an optional `vertices_m` proposal

### 2a. Exact derivation used today (two independent, near-identical implementations)

**Engine** — `⚙️engine/🧠️precompute/🦀️.rs:777-802`, `place_window(host, normal, count, position,
area_m2, height_m, sill_m) -> Vec<[f64;3]>`:
1. `width = area_m2 / height_m.max(1e-3)`.
2. Builds an in-plane basis: `horizontal = normalize([-normal[1], normal[0], 0])` (falls back to
   world +X when the surface is near-horizontal, `abs(normal.z) > 0.99`); `upward = normal ×
   horizontal`.
3. Projects the **host wall's own vertices** onto `(horizontal, upward)` to get `u_min/u_max/v_min`
   (the wall's own footprint in its local frame).
4. Splits the host wall into `count` equal horizontal **bays** (`count` = number of sibling
   `Fenestration`s on the same `surface_id`, found by linear scan
   `🧠️precompute/🦀️.rs:534-536`); window `position` is centred in bay `position`.
5. `v0 = v_min + sill_m`, `v1 = v0 + height_m`; emits 4 corners in order
   `(u0,v0),(u1,v0),(u1,v1),(u0,v1)`.
   Called from `step_window` (`🧠️precompute/🦀️.rs:530-536`), once per `Fenestration`, at
   precompute time (before any timestep runs) — the resulting `WindowPrecompute.polygon` (line 59)
   is what solar/shading math (`⚙️engine/☀️solar`) actually operates on.

**Exporter (epJSON)** — `🚪️io/📤️export/🧵️serializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs:199-222`,
`aperture_rectangle(host, window, index, count) -> Option<[Vec3; 4]>`: same idea, small differences
— basis via `surface_basis` (`cross(world_up, normal)`, falls back to `cross(+Y, normal)`, lines
189-196) rather than the engine's `[-n.y, n.x, 0]` shortcut; and it fills in *defaults* when
`height_m`/`sill_height_m` are zero (`APERTURE_ASPECT = 1.5` aspect ratio, `APERTURE_SILL_M = 0.2`
m, lines 52-55, 213-215) — a fallback the engine's `place_window` does not have (it always uses
`height_m`/`sill_height_m` verbatim, so an all-zero `Fenestration` collapses to a degenerate strip
at the sill). Both derivations agree exactly on ASHRAE 140 case 600 (verified against
`🏛️bestest/🦀️.rs`'s literal `WINDOW_WIDTH_M=3.0`/`HEIGHT_M=2.0`/`SILL_M=0.2` constants, module
docstring line 27-28).

**Importer already extracts real polygons and throws them away** —
`🚪️io/📥️import/🧩️deserializers/🗿️artifacts/⚡️epjson/🔖️25.2/✳️any/🦀️.rs:458-511`,
`decode_apertures`: reads the real 4 vertices from an epJSON `FenestrationSurface:Detailed`
(`aperture_corners(fields)`, line 145, 472), computes `width`/`height` back out by `distance()`
between corners (477-478), computes `sill` by re-projecting corner 0 against the host wall's own
basis (479-496), and **rejects any aperture that isn't exactly 4 vertices** ("only four-vertex
rectangular apertures decode into a semio Fenestration", line 474) — i.e. an arbitrary window
polygon *can already be read from an epJSON file* but is discarded/rejected today because
`Fenestration` has nowhere to keep it.

### 2b. Proposal: optional `vertices_m: Vec<[f64; 3]>` on `Fenestration`

Add the field as `#[serde(default)]`-equivalent (the `ToValue`/`FromValue` derive already handles
`Vec<[f64;3]>` — `Surface.vertices_m` and `ShadingSurface.vertices_m` prove the derive macro needs
no hand-written impl for this shape) — empty `Vec` means "derive from area/height/sill as today",
non-empty means "use these vertices verbatim, area/height/sill become display-only / ignored by
geometry code but still used by the U-value/SHGC physics which don't need a shape at all". This
keeps every existing fixture/model byte-compatible (empty vec is the zero value).

Every place that would need to branch on `!vertices_m.is_empty()`:

| Site | File : line | What changes |
|---|---|---|
| Engine placement | `⚙️engine/🧠️precompute/🦀️.rs:536` (`step_window`) | `if fenestration.vertices_m.is_empty() { place_window(...) } else { rotate each vertex the same way host vertices are rotated by north_axis_deg, 🧠️precompute/🦀️.rs:373/461 }` |
| Engine solar/shading | `⚙️engine/☀️solar/🦀️.rs` (`sunlit_fraction`, `shadowed_area_m2`, `beam_overlap_m2`) | none — already generic over `&[[f64;3]]`, works unchanged once `WindowPrecompute.polygon` carries the real shape |
| Engine geometry helpers | `⚙️engine/📐️geometry/🦀️.rs` (`surface_area_m2`, `polygon_normal`) | none — generic |
| Exporter | `⚡️epjson/…/export/…/🦀️.rs:199-222` (`aperture_rectangle`) and its call site (`FenestrationSurface:Detailed` writer, search `apertures.push` near line 648) | branch: if `window.vertices_m` non-empty, emit those verbatim (matching `GlobalGeometryRules` winding, §1); else keep `aperture_rectangle` |
| Importer | `⚡️epjson/…/import/…/🦀️.rs:458-511` (`decode_apertures`) | drop the "only four-vertex rectangular" rejection (line 473-476); always set `vertices_m: corners`, and keep computing `area_m2`/`height_m`/`sill_height_m` as a best-effort display fallback for the scalar path |
| Validator | `⚙️engine/🔋️model/🦀️.rs:1177-1190` (`Model::validate`, fenestration loop) | if `vertices_m` non-empty, validate planarity (`geometry::validate_polygon_planar`) and ≥3 verts instead of / in addition to the `area_m2 <= 0`/`height_m <= 0` checks |
| Mutations | `🧬️mutations/` new leaf `replace-fenestration-vertices/🦀️.rs` modeled on `🔺️replace-surface-vertices/🦀️.rs` (`{id: EntityId, new_vertices_m: Vec<[f64;3]>}`); **append** one variant to the `EnergyModelMutation` enum and `KINDS` array in `🧬️schema/🧬️mutations/🦀️.rs` (order is binary-ordinal, see §5 — append-only) |
| Fixtures | `🧫️fixtures/🏛️bestest-*/…` JSON snapshots and `🧫️fixtures/🧬️mutations/…` | no change needed if the field defaults to empty-vec in the codec; only new fixtures exercising the polygon path need it |
| ToValue/FromValue | none — `#[derive(ToValueDerive, FromValueDerive)]` already on `Fenestration` (`🔋️model/🦀️.rs:389`) picks up a new field automatically, same as `Surface`/`ShadingSurface` |
| DSL text/binary codecs | `🧬️schema/🧬️mutations/📝️text/🦀️.rs` and `💾️binary/🦀️.rs` (both ~43 lines, fully generic over `dsl::Mutations`/`dsl::DslVariants` derive) — **no manual edit**, confirmed no per-kind code exists there today; the new leaf's own struct needs `dsl::DslField`-compatible types, which `Vec<[f64;3]>` already is (proved by `ReplaceSurfaceVertices`) |
| Snapshot codec | `🧬️schema/📸️snapshot/🛰️.proto` | none — the whole `Model` travels as opaque `bytes model = 2` (§ proto is only 11 lines, confirmed by read), so a new struct field never touches this file |
| Mutation proto/GraphQL/TS | `🧬️mutations/🛰️.proto` (new `message ReplaceFenestrationVertices { uint32 id = 1; repeated Vertex3 new_vertices_m = 2; }` + a new oneof arm, append after the highest existing ordinal ~211+), `🧬️mutations/🔗️.graphql`, `🧬️mutations/🟦️.ts` (mirror `ReplaceSurfaceVertices`, e.g. `🟦️.ts:227-230`) | new type + union member in each |

---

## 3. Per-surface / per-zone results after a run — and the "kWh map" gap

- **`Results`** (`⚙️engine/🧾️results/🦀️.rs:53-65`): `time_series: TimeSeriesTable`, `meters:
  MeterTable`, `summaries: SummaryTables`, `sizing: SizingTables`, `environmental`, `resilience`,
  `diagnostics`, `run_metadata`. All keyed by **string** (`TimeSeries.key: String`,
  `📤️output/🦀️.rs:66`) or by **zone-aggregate index**, never by surface.
- **`Meter`** (`⚙️engine/🧮️meters/🦀️.rs:46-53`): `{name, fuel: FuelType, end_use: EndUse, energy_j,
  peak_demand_w, peak_demand_hour}`. `MeterTable` is a `FixedTable` sized/admitted once
  (`🧪️sim/🦀️.rs:3535`) with **3 meters per zone** (heating/cooling/fan,
  `sim/🦀️.rs:2798/2816/2834`, index arithmetic `zone_index*3 + {0,1,2}`) plus 2 facility-level
  meters (heating, PV — `sim/🦀️.rs:2871/2882`). **No per-surface meter slot exists.**
- **Per-surface/per-window physics IS computed every timestep, but only as transient working
  arrays, never persisted**:
  - Opaque conduction: `unit_face` (`⚙️engine/🌰️kernel/🦀️.rs:1365-1406`) computes each surface's
    outside heat-balance coefficient/source (`h_convection+h_air+h_sky+h_ground`,
    `outside_solar_absorptance * solar_w_m2`, line 1394-1406) into `state.solver.*` arrays indexed
    by face — solved once per timestep and then **discarded** (overwritten next timestep).
  - Window solar: `unit_window_solar` (`kernel/🦀️.rs:1243-1285`) and `unit_beam_patch`
    (`kernel/🦀️.rs:1301-1345`) compute `state.solver.inside_absorbed_w_m2[back_index] += part *
    surface.inside_solar_absorptance / surface.area_m2` and `work.zone_diffuse_w += part * (1 -
    absorptance)` — again transient, accumulated into the **zone's** diffuse pool, not retained per
    source surface.
- **Grep confirms no per-surface output-variable keys exist anywhere in the engine**: no
  `"Surface Inside Face Conduction"`, `"Surface Window Transmitted Solar"`,
  `"SurfaceWindow…"`-style keys are ever built (searched all of `⚙️engine/**/*.rs`, zero hits
  outside the epJSON *export* module's own `CONTRACT_OUTPUT_VARIABLES` constant, `⚡️epjson/…/🦀️.rs:
  65`, which is just a list of variable *names to request from EnergyPlus* for the oracle
  comparison — it plays no role in semio's own kernel).
- **Published run payload is even coarser** —
  `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs:949-995`: the
  live progress cursor carries only `stage`, `tier`, `timestep` counters, and one scalar
  **`facility_electricity_kwh: f64`** (line 950, rounded to Wh at line 967/995). No zone breakdown,
  let alone per-surface, is on this wire at all; the full `Results` (with its per-zone meters) is
  only available from `Engine::run`'s return value (`🧪️sim/🦀️.rs:4734`), not from the session
  progress channel the shell subscribes to.

**What's needed to expose a per-surface "transmission loss kWh" / "solar gain kWh" map:**
1. A new persistent accumulator, parallel to `MeterTable` but keyed by `EntityId` (surface/window
   id) rather than zone index — e.g. `FixedTable<EntityId, SurfaceEnergyMeter>` with fields like
   `conduction_loss_j: f64`, `solar_transmitted_j: f64`, sized/admitted once like `meters.meters` is
   today (`🧪️sim/🦀️.rs:3535`).
2. Two new accumulate call sites: in `unit_face` (kernel.rs ~1394-1406) where the outside conductive
   flux per surface is already computed per timestep — accumulate `flux_w * dt_s` into that
   surface's meter instead of only using it to solve the shared node chain; and in
   `unit_window_solar`/`unit_beam_patch` (kernel.rs ~1243-1345) where `part`/`absorbed_w` (already
   per-window W) is computed before being folded into the zone pool — accumulate the *transmitted*
   (not just absorbed) solar there too (transmitted solar isn't currently separated from absorbed
   at all; would need its own term, since today's code only tracks what's absorbed by the back
   surface + what stays diffuse, never "transmitted through this window during this timestep" as a
   number).
3. Surface it in `Results` (e.g. add a `per_surface: FixedTable<EntityId, SurfaceEnergySummary>` to
   `⚙️engine/🧾️results/🦀️.rs`'s `Results` struct) and thread it through the simulation-session
   payload (`🧵️simulation-session/🦀️.rs`) if the UI needs it live rather than only at job
   completion.

---

## 4. Shading surfaces, adjacency, spaces — geometric contribution

- **`ShadingSurface`** (`🔋️model/🦀️.rs:687-693`): free-standing polygon (`vertices_m`, no zone),
  optional `transmittance_schedule_id`. Contributes as a **solar-shadow caster** only — collected
  into `PrecomputedModel`'s caster list (`precompute::push_caster`, referenced at
  `🧠️precompute/🦀️.rs:373`) and consumed by `solar::shadowed_area_m2`/`sky_diffuse_shading_ratios`
  (`☀️solar/🦀️.rs:279-366`). It never participates in the thermal envelope (no construction, no
  conduction) — purely an opaque-to-sun geometric obstruction. This is the one entity type already
  fully 3D-render-ready (arbitrary polygon, no reconstruction needed).
- **`AdjacencyPair`** (`🔋️model/🦀️.rs:713-716`): bookkeeping only — `{surface_a_id, surface_b_id}`.
  Combined with `OutsideBoundary::Interzone(partner_id)` on each `Surface`
  (`🔋️model/🦀️.rs:316-322`), this is how two zones' surfaces are declared thermally coupled
  (heat flows surface-to-surface rather than surface-to-outdoors). Validated in
  `Model::validate` (`🔋️model/🦀️.rs:1135-1139`, `1029-1033`) purely as an existence check on both
  surface ids — no separate geometry; for 3D purposes an interzone pair is just "two coincident (or
  near-coincident) polygons, one drawn per zone," nothing to derive additionally.
- **`Space`** (`🔋️model/🦀️.rs:277-282`): `{zone_id, floor_area_m2}` — a bookkeeping/reporting
  subdivision of a zone (e.g. for floor-area accounting in envelope compliance), **contributes zero
  geometry**: no footprint polygon, no height range. For 3D rendering, spaces cannot be drawn at all
  without a new geometric field (a footprint or a sub-volume) — today they're pure metadata.
- **`SpaceList`**/**`ThermalEnclosure`** (`🔋️model/🦀️.rs:696-709`): pure grouping (`Vec<EntityId>`
  of space/zone ids) — no geometry.

---

## 5. Mutation kinds relevant to inspector edits

Source: `🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` — one
`EnergyModelMutation` enum (line 307) generated via `#[derive(dsl::Mutations)]`; `KINDS: &[&str]`
(line 588) is the same roster in **binary-ordinal declaration order** — appending is safe,
reordering/inserting is not (breaks the wire format). Each variant is a re-export of a one-file
leaf under this directory (`🧬️mutations/<emoji-slug>/🦀️.rs`).

| Mutation fn | Payload fields (all in the leaf's struct) |
|---|---|
| `change_fenestration_u_value(id, new_u_value_w_m2k)` | `{id: EntityId, new_u_value_w_m2k: f64}` |
| `change_fenestration_shgc(id, new_shgc)` | `{id, new_shgc: f64}` |
| `change_fenestration_vlt(id, new_vlt)` | `{id, new_vlt: f64}` |
| `change_fenestration_area/height/sill_height(...)` | `{id, new_area_m2 / new_height_m / new_sill_height_m: f64}` |
| `change_fenestration_frame_conductance` / `_divider_conductance` | `{id, new_*_conductance_w_k: f64}` |
| `change_fenestration_overhang_depth/offset`, `_fin_depth/offset` | `{id, new_*_m: f64}` |
| `bind_fenestration_glazing_construction` / `clear_fenestration_glazing_construction` | `{id, new_glazing_construction_id: EntityId}` / `{id}` |
| `change_fenestration_surface(id, new_surface_id)` | `{id, new_surface_id: EntityId}` (re-host a window) |
| `rename_fenestration(id, new_name)` | `{id, new_name: String}` |
| `change_surface_class(id, new_class)` | `{id: EntityId, new_class: SurfaceClass}` |
| `change_surface_construction(id, new_construction_id)` | `{id, new_construction_id: EntityId}` |
| `change_surface_boundary_condition(id, new_boundary, new_interzone_surface_id)` | `{id, new_boundary: OutsideBoundaryKind, new_interzone_surface_id: Option<EntityId>}` |
| `replace_surface_vertices(id, new_vertices_m)` | `{id, new_vertices_m: Vec<[f64;3]>}` |
| `change_surface_zone/_multiplier/_sun_exposed/_wind_exposed` | `{id, new_zone_id: EntityId}` / `{id, new_multiplier: u32}` / `{id, new_sun_exposed / new_wind_exposed: bool}` |
| `rename_surface(id, new_name)` | `{id, new_name: String}` |
| `replace_shading_surface_vertices(id, new_vertices_m)` | `{id, new_vertices_m: Vec<[f64;3]>}` |
| `change_shading_surface_transmittance_schedule` | `{id, new_transmittance_schedule_id: Option<ScheduleId>}` |
| `rename_shading_surface(id, new_name)` | `{id, new_name: String}` |
| `change_material_conductivity/thickness/density/specific_heat` | `{id, new_conductivity_w_m_k / new_thickness_m / new_density_kg_m3 / new_specific_heat_j_kg_k: f64}` |
| `change_material_solar_absorptance/_thermal_absorptance/_visible_absorptance` | `{id, new_*_absorptance: f64}` |
| `rename_material(id, new_name)` | `{id, new_name: String}` |
| **glazing material fields** | **no mutation kind exists** — grepped the full re-export list (`🧬️mutations/🦀️.rs`), zero `change_glazing_material_*` or `change_gas_material_*` entries. `GlazingMaterial`'s 12 optical/thermal fields and `GasMaterial`'s `thickness_m`/`gas` are only settable by re-import, not by any inspector edit today. |
| `add_construction_layer` / `remove_construction_layer` / `reorder_construction_layers` | layer-list edits by material id; `rename_construction(id, new_name)` also exists — but no direct "change construction X's layer Y" field edit |
| `rename_zone(id, new_name)` | `{id, new_name: String}` |
| `change_zone_volume(id, new_volume_m3)` | `{id, new_volume_m3: f64}` |
| `change_zone_conditioned` / `_multiplier` / `_floor_area_participation` | `{id, new_conditioned: bool}` / `{id, new_multiplier: u32}` / `{id, new_part_of_total_floor_area: bool}` |
| `update_site(latitude_deg, longitude_deg, elevation_m, time_zone_hours, north_axis_deg)` | whole-`Site` replace, no id (singleton) |

**Gaps relevant to an inspector**: no `change_space_floor_area` (grep found none), no glazing/gas
material mutations at all, and — per §2 — no `replace_fenestration_vertices` yet since the field
doesn't exist.

---

## Summary of what's missing for the 3D + colour-by-result inspector

1. **No 3D viewport exists** in the energy plugin; `structure` window is text-only.
2. **Walls/floors/roofs/ceilings**: fully renderable today (`Surface.vertices_m`, real polygons,
   consistent CCW-from-outside winding, z-up).
3. **Arbitrary window polygons**: engine and exporter both derive a rectangle from
   `area_m2`/`height_m`/`sill_height_m`; the importer already parses real vertices from epJSON and
   throws them away. Needs the optional `Fenestration.vertices_m` field (§2) plus ~6 call-site
   updates (precompute, exporter, importer, validator, one new mutation kind, TS/proto mirrors) —
   no engine-geometry-helper changes needed since those are already polygon-generic.
4. **Shading surfaces**: fully renderable today, no gaps.
5. **Zones as volumes**: no zone geometry exists at all — only the surfaces that reference a
   `zone_id`; a 3D zone volume must be assembled client-side by grouping that zone's surfaces (the
   engine's own `zone_volume_from_surfaces` helper, §1, could be reused for a numeric volume badge,
   but there is no single "zone shape" to draw — it's implicitly the union of its surfaces).
   `Space` has even less: no footprint at all.
6. **Colour by simulation result**: no per-surface accumulator exists anywhere — conduction and
   solar fluxes are computed per surface/window every timestep but discarded after the zone-level
   heat balance solve; only 3 meters/zone + 2 facility meters persist, and the live session payload
   exposes just one scalar (`facility_electricity_kwh`). Needs a new `EntityId`-keyed accumulator
   table (parallel to `MeterTable`) fed from `unit_face`/`unit_window_solar`/`unit_beam_patch` in
   `⚙️engine/🌰️kernel/🦀️.rs`, plus a `Results.per_surface` field and session-payload wiring.
