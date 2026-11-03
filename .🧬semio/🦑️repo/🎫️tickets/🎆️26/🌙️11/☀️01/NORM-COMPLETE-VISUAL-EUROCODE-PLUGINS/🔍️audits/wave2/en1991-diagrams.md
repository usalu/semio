# EN 1991 diagrams, grips and showcase (wave 2)
Nothing compiled (no cargo, no `repo run`). `freeze.txt` was absent at every batch. No git writes.

## Verification of the audit first
The audit (gap 4 and 17) was partly stale: tie, helicopter, impact force, gas explosion, thermal section profile, bridge wind, stage timeline, lane LM1 load and accidentalCases grips (nested impact and explosion) already existed. Verified missing and now added: wind pressure per face, roof snow per roof, self-weight breakdown, construction stage loads, accidental impact and explosion rows, impact force-time pulse, falling load, execution imperfection, derailment, lane plan, grips roofs.multiSpan and dynamicElements.massPerLength, and the whole showcase.

## Diagrams (`🎬️scene/📈️diagrams/🦀️.rs`, pure projections of document and report, wired into `diagrams()` by focus group)
| id | Group (focus owners) | Content |
|---|---|---|
| `wind-face-pressure` | Wind | net pressure q_p(z_e)(c_pe(A) - c_pi) c_s c_d per face and zone vs assumed w_p, marker bound to `assumedWp` |
| `roof-snow-loads` | Snow | mu C_e C_t s_k per roof (shape_mu: pitch, parapet, drift, multi-span) vs assumed, bound `assumedSk` |
| `self-weight-breakdown` | Gravity (floor, self-weight) | gamma t per element vs assumed g_k |
| `construction-stage-loads` | Stages (stage and construction load) | psi0 sum Q_c,i per stage vs assumed |
| `accidental-impacts`, `accidental-explosions` | Accidental (new) | per nested row: v sqrt(km), Mills blast pressure vs assumed, bound to the nested `impact[j]`/`explosion[j]` paths |
| `impact-pulse` | Impact | triangular force-time pulse, peak v sqrt(km), duration 2 m v / F (impulse = momentum) |
| `falling-load-force` | Falling (new) | m g (h+s)/s over s, required and assumed per row |
| `execution-imperfection-force` | Imperfection (new) | N Delta / h over Delta, required and assumed per row |
| `derailment-forces` | Derailment (new) | entered F_dx, F_dy and assumed resistance over the distance |
| `bridge-lane-plan` | Bridge | LM1 UDL per lane across the carriageway at offset and width, governing (heaviest tandem) lane in warning tone, marker bound to `offset` |
Shared helper `compare` (required vs assumed pairs). Not added (documented): placements, specialVehicles (position is gripped; influence line exists), v12 collections. Thermal gradient exists already as `section-temperature-profile`.
Shared formula fns moved to `checks` `part_1_6` (`falling_arrest_force_n`, `imperfection_force_n`, `combined_construction_load_pa`); `hazard-checks` now calls them (no duplicate formula).

## Grips (`🎬️scene-members/🕹️handles/🦀️.rs`)
roofs.multiSpan: choice stepper `multi-span` (tokens `false`/`true`, the edit rules map them onto the Bool leaf, `edit-rules` line 433). dynamicElements.massPerLength: grip `mass` (100 kg/m per metre). `handle-paths-check` en1991: 0 uncovered.

## Showcase
New builder `🧰️scripts/showcase/en1991.py` (pattern of showcase-refresh.md): bridge-kind document populating all 28 collections (the six hazard collections plus footbridges, railDynamics, loadDispersals, bulkSolids, rotatingMachines, craneHoists, a multi-span roof, two lanes, nested impact and explosion rows). Output `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, `--check` unchanged. `common.py` now resolves `definitions` and `$defs` together (the en1991 schema carries both after the v12 generator; reported, not restructured). Rust: `En1991Snapshot::full_showcase()` and palette example `full-showcase` (EN "Full showcase example", DE "Vollstaendiges Gesamtbeispiel") in the editor.

## Tests added (unit, scene crate)
`the_full_showcase_populates_every_collection_and_every_new_view_has_a_diagram` (28 non-empty, scene laws, diagram laws incl. German-leak and table-twin, each new id in the list and under its focus), `the_hazard_diagrams_carry_the_hand_computed_required_values` (falling 20.594 kN, imperfection 5 kN, derailment 1200 kN, pulse impulse = m v), `the_new_grips_write_the_span_arrangement_and_the_mass_per_length`. Diagrams are pure functions of (document, report); the incremental-equals-full test (`incremental_equals_full_on_random_edit_sequences`) is unchanged and covers the scene path, the diagram path has no incremental state.

## Gates
handle-paths: 0 uncovered. Showcase `--check`: unchanged. Fixture checker `verify_fixtures.py en1991 roof` via heavy.sh: queued behind the heavy lock when this report was written (no fixture of a mutation kind changed; only the new showcase file was added); result lands in `🗑️generated/en1991-diagrams/fx.txt`.

## Needs a run
Compile of the new diagram code (likeliest: `compare` array/Vec axes, `DerailmentCase` closure coercion, `choice_handle` import path), `full_showcase` include path, `looks_english` on new German labels, ChoiceStepper current-token read for a Bool leaf, scene golden bless for the roof handle (new grip changes the roof golden). DSL assets (`🖼️assets`) and packs untouched.
