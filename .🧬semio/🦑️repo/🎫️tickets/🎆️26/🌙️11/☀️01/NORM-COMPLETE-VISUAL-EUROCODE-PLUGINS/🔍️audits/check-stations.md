# Check stations: spatial utilisation in the shared result contract

Ticket: `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`. Not compiled by the author (shared crates belong to the integrator; "ready for rebuild" filed in `📌️important/requests.md`).

## Contract

- `Station { at: Vec<f64>, utilization: f64, values: Vec<Quantity> }`: `at` is `[s]` (member-local, `[0,1]`), `[x, y]` (plate) or `[x, y, z]` (solid); `values` are the governing quantities (typically demand and capacity).
- `CheckResult.stations: Vec<Station>` and `CheckResult.station_field: String` (empty = along member length; otherwise a field name such as `soil-pressure`, `wall-phi`, `punching-perimeter`, `storey-drift`, `section-temperature`). Wire names `stations`, `stationField`; required arrays/strings in all twins (empty when absent).
- Law: `utilization == max(stations)` (at least 1.0 for a Fail), enforced in `CheckBuilder::build` (a station above 1.0 turns Pass/Warning into Fail, so the usual remedy requirement applies); not applicable checks drop stations. `CheckResult::stations_consistent()` verifies the law and station well-formedness; `max_station_utilization()`.
- Builder: `Station::{new, along, point, point3, ratio_of, with, is_well_formed}`, `CheckBuilder::{stations, field, sampled(n, |s| (u, values))}`. Station utilisation is capped at `UTILIZATION_CAP`; NaN becomes the cap.

## Files

- Kernel: `⚖️compliance/✅️check-results/🦀️.rs`; twins `⚖️compliance/🧬️schema/{🛰️.proto,🔗️.graphql,🟦️.ts,🔣️.json}` (new `Station` type; JSON schema bounds `at` to 1..3 items and utilisation >= 0).
- Fixture `⚖️compliance/🧪️tests/🧫️fixtures/🔬️check-report/🔣️.json` (all checks gained `stations`/`stationField`, new check `din4108.2.psi.stations`; summary total 4, pass 2) with the Rust sample in `🧪️tests/🔬️unit/🦀️.rs`.
- Kit: `🎬️structural-scene/🧰️projection-kit/🌈️station-fields/🦀️.rs` (`station_values`, `field_along`, `segment_values`, `field_at_points`); `binding_of_check` fills `per_station`.

## Tests

- Unit (kernel): `stations_make_the_maximum_the_governing_utilisation` (law, fail promotion, 2D field, N/A drop, Value round trip) and the fixture round trip.
- Unit (kit): `member_stations_become_fields_segments_and_binding_values_like_the_oracle`, `plate_stations_interpolate_by_inverse_distance_like_the_oracle`, `duplicate_station_positions_keep_the_largest_value`.
- Language-agnostic: `⚖️compliance/🧪️tests/📍️check-stations/🥒️.feature` + Python law oracle `🐍️.py` (verified: passes on the fixture); kit feature scenario "Check stations become scene fields" with oracle (numpy `np.interp` cross-check when installed, closed-form values hard-coded in the Rust test); TS: third-party Ajv test in `🔬️check-report-fixture/🟦️.ts` (law, 4-coordinate and negative-utilisation rejection).

## Rollout (announced in `requests.md`)

Member checks along length (sampled 10), wall Phi per height (EN 1996), slab punching per perimeter point (EN 1992 6.4), soil pressure per footing point (EN 1997), drift per storey (EN 1998/1990), section temperature (fire parts), fatigue per position. Visual agents: `binding_of_check().per_station`, `field_along`, `segment_values`, `field_at_points`; fall back to the single governing colour when `stations` is empty.

## Notes

- EN 1992 `members` scene already builds its own `per_station`; it can switch to the kit helper once EN 1992 checks emit stations.
- The kit currently depends on `StationValue` of the scene schema (member stations only); 2D/3D fields are served through `field_at_points`, no scene schema change.
