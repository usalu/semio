# WP-20 EN 1998 Visual Vertical

Status: phase 1 (scene) and phase 2 (editor and viewer windows) WRITTEN. Build status: see "Verification" (the `check` run was queued behind `show projects` jobs for the whole session; update this line when it exits).

Root: `NORM/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any` (`SUBSET`). Rollout checklist followed: `📌️important/artifact-rollout-checklist.md`.

## 1. Scene crate `semio-s-artifact-norm-en1998-scene` (`SUBSET/🎬️scene`)

Registered: member in `🫨️en1998/Cargo.toml`, `[workspace.dependencies.semio-s-artifact-norm-en1998-scene]`, `[workspace.dependencies.semio-s-plugin-norm-structural-scene]`, package name in `📦️packages/🦀️rust/📜️script.ts`.
Deps: `-model`, `-schema` (for `part_1`, `na_de`, `AnnexParams`), `norm-compliance`, `norm-structural-scene`; dev: `-inferences` (report for tests), `serde_json`.
Public API (same shape as EN 1993): `project(&SceneInput) -> StructuralScene`, `diagrams(document, report, focus: Option<&str>)`, `handles(document, placements, selection)`, `Layout`, `Placement`, `ENTITY_KINDS`, `entity_kind_of`, `entity_dependencies`, `response::{site, respond, joint_demand}`.

| Module | Content |
|---|---|
| `🧰️kit` | entity builder, utilisation binding from the report, layers, legends, entity-kind registry (copied from EN 1993, domain registry replaced) |
| `📍️layout` | default rule: buildings along +X (Lx + 12 m gap) at y = 0, walls y = -30, bridges y = -70, silos y = 50, tanks y = 90, towers y = 130, 30 m slots; persisted placements win |
| `🧮️response` | masses, T1 (ct / given / rayleigh), q per system, Sd(T1), lambda, F_b, F_i = F_b m_i z_i / sum, torsion factor delta, first shear-building mode: all through the same `part_1` functions as `check_full_seismic` |
| `🏢️buildings` | plan footprint with bound Lx/Ly dimensions; storey slabs (utilisation colour from storeyForces / drift / pdelta checks, soft-storey highlight with error hatch and failure glyph, bound height dimension, clause badge); column grid (spacing <= 6 m, size from the RC column member); capacity-design joint boxes at every column head; mass glyph (`Inertial` with seismic mass) per storey; centres of mass and stiffness markers, eccentricity dimension e0, accidental eccentricity labels and bound dimension e_ai; lateral force arrows per system (F_i times delta, shifted by e_ai); fundamental mode shape as `DeformationOverlay::Mode` on slabs and columns; drift bars (length = d_r nu / (theta h), coloured by the drift check) per storey and system; seismic joints between neighbours (gap vs sqrt(d1^2 + d2^2), error style when too small); assessment labels |
| `🏭️vessels-and-towers` | silos and tanks as revolved shells with filled content cylinder, seismic base shear arrow from the settled check, bound height dimension; towers and chimneys as vertical CHS members with fixed base, overturning shear arrow |
| `🌉️ground-and-bridges` | footing bodies under the supported building, retaining walls with the seismic thrust line load, bridge deck on two piers with roller bearings and the bound bearing displacement capacity |
| `📈️diagrams` | `spectra` (Se, Sd, TB/TC/TD, 2TC/4TC, T1 marker with Sd, F_b, bound check and bound `t1GivenS` when method is `given`), `spectrum-vertical`, `drift:<building>` (profile with limit 1.0 and per-storey markers bound to `driftXM`/`driftYM`), `zone-de` (a_gR over zones 0..3 with site marker bound to `site.seismicZone`); every diagram has a table twin |
| `🕹️handles` | plan width and length (face offsets), storey height, storey permanent action (mass), centre of mass x/y, centre of stiffness x/y, tower height, silo/tank height and radius, wall height; all with snap increments, limits and EN+DE accessible names; ids listed on the owning scene entities |

## 2. Tests (language-agnostic first)

- `SUBSET/🎬️scene/🧪️tests/🥒️.feature`: spectra, force distribution, mode shape, handle round trip, golden.
- `🧪️tests/🔮️oracle/🐍️.py` (numpy, independent): elastic and design spectra at every sampled period, F_b with gamma_I / q / lambda, F_i = F_b m z / sum(m z), first eigenmode with `numpy.linalg.eigh`. Spawned through `norm::python_oracle::run_ok`.
- `🧪️tests/🔬️unit/🦀️.rs`: valid scene for all eight fixtures, content hashes, determinism, golden digests (`🧪️tests/🧫️fixtures/<fixture>.txt`, created on first run, `SEMIO_UPDATE_GOLDEN=1` to refresh), every binding names a report check, every listed handle exists and writes the current value at s = 0, failing fixtures show error styles, per-storey entity set, drift bar length law, seismic joint colour law, placements/layout, registry and dependencies, entity coverage of silo/tank/tower/foundation/wall/bridge, diagrams validate and carry exact corner markers, numpy oracle comparison.

## 3. Editor and viewer windows (phase 2)

- Editor: deleted `🪟️windows/{📥️inputs,📊️results}` (Rust, TS twins, tests); added `{🧊️viewport,📈️diagrams,🧾️properties,✅️checks}` (Rust, tests, TS twins) from the EN 1993 reference; edit mode layout (properties 22 %, model 50 %, checks above diagrams 28 %); `✏️editor/🦀️.rs`: imports, `ENTITY_KIND_SCENE_KINDS`, `ChecksWindowConfigOwner`, render arms (focus = entity of the selected check, diagrams fall back to the worst entity), `viewport_interaction_definition`, four window kind definitions; `Cargo.toml` deps; `🟦️.ts` twin exports; root unit test body keys.
- Viewer: deleted `📊️report`; added read-only `{🧊️viewport,📈️diagrams,✅️checks}`; `view` mode layout 50/25/25; `👁️viewer/🦀️.rs` render arms and window kinds; `Cargo.toml` deps; TS twin exports.
- Field meta: all handle and marker write paths already have `FieldSpec` rows (norm-completion agent).
- Commands stay on `EDIT_RULES.resolve` (EN 1998 mutations were completed with 100+ `change-*` leaves, including storey height, centres, plan dimensions, tower/silo/tank/wall dimensions).

## 4. Deviations and honest gaps

- Capacity design column/beam ratio: the snapshot has no `M_Rc`, `M_Rb` inputs, so joints are drawn at every column head and coloured by the governing member detailing check; the ratio (`part_1::strong_column_ratio`) is not displayed. Request logged.
- NA-DE zone "map": a geographic map needs the licensed municipality dataset; delivered the zone chart (a_gR over zone, site marker with `Choice[0..3]` binding) and said so in the diagram description.
- Torsional radius ellipse and the pushover capacity curve (V98-8), q chart: not implemented (not required by the task list).
- Vertical spectrum uses a_vg/a_g = 0.9 and type 1 corner periods for DE and EN (NA-DE value unverified, matches `part_1::vertical_elastic_spectrum`).
- Spectrum floor: diagram follows EN 3.2.2.5(4) (beta times gamma_I times a_gR); `check_full_seismic` floors with beta times a_gR. Logged for the norm agent.
- Placement: scene takes `&[Placement]`; the editor passes `&[]` until the snapshot carries placements (request logged).
- Duplicated helper code between the EN 1993 and EN 1998 `🧰️kit` modules (builder, binding helpers): a shared scene-kit in `🎬️structural-scene` would remove it; left to the infrastructure owner.
- Hub generated component manifests (`🌎️hub/🧩️compositions/📕️norm/🧩️component/🔣️.json`, fixtures) still list `norm-en1998-inputs/results`; regenerate in integration. The shared fixture `🪟️results/🎚️config/🧪️tests/🧫️fixtures/🔬️window-ownership/🔣️.json` still lists `norm-en1998-results` as a results window kind (EN 1993 has the same stale entry).

## 5. Requests

Appended to `📌️important/requests.md` under "WP-20 EN 1998 visual vertical": Placement snapshot/mutations, capacity-design joint inputs and checks, `site.seismicZone` numeric binding, spectrum floor alignment, `resolve_annex` visibility, NA-DE vertical spectrum value.

## 6. Verification

- `repo run '@semio-tech/norm-en1998-rs:check'` started once (log `🗑️generated/vis-en1998/check.txt`); the capacity queue held it behind `projects` jobs for the whole session, no compiler output yet. Nothing compiled: expect first-build fixes (names of re-exports, trait bounds on `Violation`/`Handle` Debug usage in tests). Test command: `repo run '@semio-tech/norm-en1998-rs:test'` after `check` is green.
