# DIN 4108 target (envelope thermal protection): clause list and status

Subject: DIN 4108-2 (minimum thermal insulation, summer heat protection), DIN 4108-3 (moisture protection, Glaser), DIN 4108 Beiblatt 2 (thermal bridges), with the supporting parts 4, 6, 7, 10 and EN ISO 6946 as used by the code. Check ids are `<family>.<kind>.<element|zone id>` and every check names the row it belongs to as `subject.entity_id`, so the scene draws it on that row (`route_of`). Status: `done` = implemented and covered by a fixture or oracle, `new` = added by this lane (compile-untested), `open` = not implemented.

Every row needs a language-agnostic vector (JSON input, expected values) and a third-party or independently implemented oracle (`🔮️oracles/⚖️compliance/🐍️.py` with numpy, `🎬️scene/🔮️oracles/🐍️.py` with a numpy nodal solve and the DIN EN ISO 13788 saturation formula).

## DIN 4108-2 Minimum thermal insulation

| Id | Clause | Content | Check id | Status |
|---|---|---|---|---|
| 2.1 | Table 3 | Minimum thermal resistance R_min of opaque elements by kind (wall, roof, floor, adjacency) and surface mass, heated at 19 °C and above or at 12 to 19 °C | `din4108-2.table3.<element>` | done |
| 2.2 | 5.2.1 | Surface mass m' of the element against the Table 3 row | `din4108-2.mass.<element>` | done |
| 2.3 | 6.2 | Mould criterion: temperature factor f_Rsi = 1 - R_si/R_T >= 0.70 at the design external temperature of the climate region | `din4108-2.frsi.<element>` | done |
| 2.4 | 5.3 | Heat transmission loss of the zone H_T against the allowance by usage | `din4108-2.zone-ht.<zone>` | done |
| 2.5 | 8 | Summer heat protection by the solar input value: S_vorh = sum(A_w g F_C F_w F_i)/A_G <= S_zul, S_zul = sum(S_x) by climate region, construction weight, night ventilation, solar-control glazing, passive cooling | `din4108-2.summer.<zone>` | done |
| 2.6 | 8 | Zones missing: summer heat protection cannot be demonstrated | `din4108-2.summer.none` | done |
| 2.7 | Table 3 | Elements missing: R_min cannot be demonstrated | `din4108-2.table3.none` | done |

## DIN 4108-3 Moisture protection (Glaser)

Boundary conditions (DIN 4108-3 climate, EN ISO 13788): winter -5 °C / 80 % (shifted by facade orientation, see `glaser_boundary`), interior from the document (default 20 °C / 50 %), summer from the climate region design temperature (minus 16 K for walls and floors, minus 8 K for roofs, plus orientation shift) at 70 %.

| Id | Clause | Content | Check id | Status |
|---|---|---|---|---|
| 3.1 | 4.3 | Interface temperatures from the series resistance network; vapour pressure linear in the equivalent air layer thickness s_d = mu d; saturation pressure at the interface temperature; condensation where p > p_sat | `din4108-3.glaser.<element>` | done (profile diagrams `temperature:` and `glaser:`, oracle in the scene crate) |
| 3.2 | 4.3 | Condensed mass in the winter period <= 1.0 kg/m2 (0.5 kg/m2 for wood and light elements up to 100 kg/m2) | `din4108-3.glaser.<element>` | done |
| 3.3 | 4.3 | Drying reserve: the mass that can evaporate in the summer period at the condensation interfaces must be at least the condensed mass of the winter period | `din4108-3.drying.<element>` | new (`part_3::check_drying_reserve`, oracle) |
| 3.4 | 4.3 | Minimum vapour diffusion resistance when no condensation occurs (s_d >= requirement) | `din4108-3.glaser.<element>` | done |
| 3.5 | 4.2 | Refined hourly or dynamic methods (EN 15026) | none | open, out of scope of the steady-state subset |

## DIN 4108 Beiblatt 2 Thermal bridges

| Id | Clause | Content | Check id | Status |
|---|---|---|---|---|
| 4.1 | 5.1 | Equivalence category of each linear bridge (A, B or detailed psi) | `din4108-bb2.bridge.<bridge>` | done |
| 4.2 | 5.1 | Flat surcharge Delta U_WB = sum(psi l)/A_envelope against the admissible value of the category mix (0.03, 0.05, 0.10 W/(m2 K)) | `din4108-bb2.delta-u` | done |
| 4.3 | 5.1 | Delta U_WB enters the U' of every element | `din4108-6.u-prime.<element>` | done |
| 4.4 | 5.1 | Document-level conformity flag of the Beiblatt 2 details | `bb2DetailsConform` input | done |

## Supporting parts

| Id | Part | Content | Check id | Status |
|---|---|---|---|---|
| 5.1 | 4 | Design lambda of each layer and segment from the catalogue | `din4108-4.lambda.<element>.<layer>` | done |
| 5.2 | 6 / EN ISO 6946 | U-value (including inhomogeneous layers by upper and lower bound, Delta U corrections) against the limit | `din4108-6.u.<element>` | done |
| 5.3 | 7 | Airtightness n50 against 1.5 (with ventilation) or 3.0 per hour | `din4108-7.n50` | done |
| 5.4 | 10 | Application type, compressive, water and tensile class of each layer | `din4108-10.app.<element>.<layer>` | done |
| 5.5 | 1 | Reference integrity: unique ids, zone references, catalogue materials | `din4108.integrity.*` | done |

## Visual and editing obligations

- Scene: element = layer section (real thicknesses, interior first), zone = floor plate, bridge = line; diagrams `temperature:`, `glaser:`, `u-values:building`; grips write `layers[id].thicknessM`, `thermalBridges[id].lengthM|psi` (EN+DE field meta).
- Every check routes to its row or to the building badge (`DOCUMENT_CHECKS`).
- Placements (`placements[id].x|y|yaw`) are open: the layout derives positions from the document order.
