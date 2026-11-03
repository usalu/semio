# Fidelity pass: EN 1992, EN 1995, EN 1996 scenes

Status: written, NOT compiled or run (build loop: "ready for rebuild: scene fidelity" is in `🗑️generated/build-en1992|1995|1996/latest.md`; requests in `📌️important/requests.md`). Python oracle for the EN 1992 thermal solver was run (outputs below). Roots: `A = ✏️s/🔌️plugins/📕️norm/🗿️artifacts/<emoji>enXXXX/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. Stations emitted by the checks (pure additions, law `utilization == max(stations)` held by construction)

| Artifact | Check | Field | Stations |
|---|---|---|---|
| EN 1992 | `en1992.6.1.flexure.*` | member `s` | 11, `[M_Ed(s), M_Rd, N_Ed]`; columns/walls/ties: end sections s=0,1 |
| EN 1992 | `en1992.6.2.shear.*`, `en1992.7.3.crack.*` | member `s` | 11 (shear `[V_Ed(s), V_Rd]`; crack `[w_k(s), w_max]` by re-running 7.3.4 at M(s)) |
| EN 1992 | NEW `en1992.7.4.deflection.*` | member `s` | 11, `[w(s), L/250]`: elastic line of the real loads and support, creep (Annex B.1, RH 50 %, 28 d, 50 a), cracking interpolation 7.18 with beta 0.5 |
| EN 1992 | `en1992.6.4.punching.*` | `punching-perimeter` (plate frame) | points on u1 (4 corner arcs, clipped at free edges); interior column: shear gradient 2-beta..beta along the span (mean 1, peak beta), edge/corner uniform beta |
| EN 1992 | `en1992-4.cone.*` | `anchor-cone` | one station per anchor (each anchor is its own subject) |
| EN 1995 | bending 6.1.6, shear 6.1.7, tension 6.2.3, combined 6.3.2.c | member `s` | 11 (+ extra peaks), demand/capacity recomputed with the combination scaled by the shape at `s` |
| EN 1995 | `winst`, `wfin`, `wfreq` | member `s` | 11 (+ field maxima of the two-span line), `[w(s), limit]`, `wfin` includes k_def |
| EN 1995 | `en1995.1-2.4.fire.*` | `fire-time` | 11 instants `t / t_fire`, `[M_Ed, M_Rd,fi(t), d_ef(t)]` |
| EN 1995 | Johansen `en1995.8.2.x.johansen.*` | `fastener-force` (2D: along grain, across) | one per fastener (<= 16 x 4); shear-lag shares, largest share = 1/n_ef so max station = F_Ed/F_v,Rd |
| EN 1996 | `en1996.6.1.2.compression.<wall>.<lc>` | `wall-phi` | 3 sections s = 0, 0.5, 1 (bottom, middle, top), `[N_Ed, N_Rd, Phi]` |
| EN 1996 | `en1996.6.3.flexure.*` | `wall-lateral` | 11 along the height, M_Ed(z) = M_Ed 4z(1-z) |

Where: `A(1992)/🧬️schema/💡️inferences/⚖️checks/📋️member-evaluation/{📉️effect-profile,🕳️control-perimeter,🏗️resistance-and-serviceability-checks,⚓️anchor-evaluation}`; `A(1995)/🧬️schema/⚖️timber/{📉️effect-profile,🧰️assessment-vocabulary,🏗️structure-and-members,🌉️bridge-and-connections}`; `A(1996)/🧬️schema/⚖️masonry/🧱️evaluate-wall/🏋️load-case-checks`.

Design decisions
- Shapes are the closed forms of the model's own statics (simple, cantilever, fixed, two-span; UDL and point load superposed with signed shapes, weights from the actual loads), normalised so the peak equals the value the check already assessed. No verdict changes by construction (stations only distribute the assessed value); a borderline difference can only come from floating point (the builder takes `max(stations)`).
- EN 1995 two-span: the check's `M = q l^2/14` is kept as the governing-section value; the exact two-span shape (support 1.0, field 0.57) is scaled to it. A request asks the norm agent to confirm that convention.
- EN 1996: only the three code sections are stations. The contract's 1/4 and 3/4 stations were deliberately not emitted because a continuous Phi/N profile could exceed the three assessed sections and silently change verdicts; the scene interpolates (see 2).
- Externally analysed effects (`source == "external"`, `use_fem`) have no known shape: no member stations (single governing colour as before).

## 2. Scenes read the stations

EN 1992 (`A/🎬️scene`)
- `🏛️members`: concrete body binding = envelope of all member-along stations (`envelope_binding`), rebar flexure colour = flexure stations (`projection_kit::station_values`), the approximation `ratio x moment_shape` is removed. Cracks: extent and each crack width come from the crack-width stations (`field_along`), the 0.4 Mmax threshold is removed. Deflected shape: the `7.4.deflection` stations (true line, not uncracked EI). Interaction design point and stress block use the governing flexure station (`mechanics::reported_effects`: M_Ed, N_Ed of the report) instead of the hard-coded 1.35/1.5/1.05 combination (kept only as fallback when the report has no station).
- `🕳️punching`: the u1 ring is tinted per vertex (`ColourChannel::Fields`, `FieldSamples::Vertex`, `field_at_points`) with its own legend; new `Spec::channel`, `Out::legends`, `field_legend` in the kit.
- `📈️diagrams`: deflection diagram from the check stations (title/EN+DE text updated), fire diagram adds the isotherm contour.
- NEW `🌡️thermal`: pure Rust 2D transient conduction, enthalpy formulation (exact through the 100-115 degC moisture plateau), explicit finite volumes, cell 5 mm, ISO 834 gas, EN 1991-1-2 boundaries (alpha_c 25 / 4, epsilon 0.7), EN 1992-1-2 3.3 siliceous properties (lambda upper limit, c_p with 1.5 % moisture peak, rho(theta)). Outputs: `Field::at`, `isotherm_ring` (72 rays, true contour with rounded corners), `reduced_rectangle` (500 degC method on the centre lines), `isotherm_depth`. Replaces `isotherm_depth`/`erfc`/`ISOTHERM_DIFFUSIVITY`/`reduced_section` (deleted, no compat). Entity `:isotherm` is now the real contour polygon.

EN 1995 (`A/🎬️scene`)
- `🎵️modes` (NEW): first vertical mode from the mechanics kernel (`Analysis::modal`, 16 consistent-mass elements, planar). Floor/footbridge mode overlays are eigen-solutions of the drawn supports (pinned, cantilever, two spans: antisymmetric about the middle support); frequency shown is the kernel's. `facts::{bending_stiffness, Dynamics::linear_mass}` added.
- `🪵️members`: deformed shape from the `wfin` stations (includes k_def); member binding = envelope (`kit::binding_of`).
- `🔩️connections`: fastener entity tinted per instance from `fastener-force` (`ColourChannel::Fields`, `FieldSamples::Instance`), legend; `Out::legends` + `field_legend` in the kit.
- Dependency: `semio-s-plugin-norm-mechanics` added to `🪵️en1995/Cargo.toml` (workspace dep) and the scene `Cargo.toml`.

EN 1996 (`A/🎬️scene`)
- `🧱️walls`: zone bindings equal the station values of `wall-phi` (the duplicate scene profile is only a fallback), each zone tinted per vertex by `field_along(stations, z / h)` with legend (continuous gradient over the height).
- `🧩️bond`: units are cut to the wall rectangles around openings (`solid_rects`), they were dropped before (audit "claimed vs actual" corrected). Perpend mortar next to an opening is still omitted.

## 3. Tests, oracles, features

- Rust: `A(1992)/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report` (stations law, 11 stations, profile not flat, punching field), `A(1992)/🎬️scene/🧪️tests/🔬️unit` (replaced the erfc/semi-infinite test by property laws + slab oracle; zero-minute, symmetry and time-monotone column tests; station-driven binding test), `A(1995)/🧬️schema/🧪️tests/⚖️compliance` (stations, row shares, fire/fastener fields), `A(1995)/🎬️scene/🧪️tests/🔬️unit` (kernel eigen-frequency vs pi/(2L^2) sqrt(EI/m) and the cantilever beta^2 formula, two-span antisymmetry, station-coloured member/fasteners; the old frequency test now allows 0.2 % against the closed form), `A(1996)/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report` and `A(1996)/🎬️scene/🧪️tests/🔬️unit` (Phi stations, wall-lateral parabola, zone field, cut units).
- Oracle (EN 1992, stdlib Python, run with `python3 -I`): implicit 1D conduction with the same EN 1992-1-2 properties; it validates itself against the closed-form erfc solution for constant properties (numeric 215.878 vs exact 215.924 degC at 50 mm after 30 min). 500 degC depth of a bottom-heated 0.4 m slab: R30 10.49 mm, R60 22.21 mm, R90 31.06 mm, R120 38.54 mm (old semi-infinite estimate: 19.4/32.4/42.8/51.7 mm, i.e. 34 to 85 % too deep). The Rust slab test compares the 2D solver to these with 2.5 mm tolerance. Embedded in the unit test and the `.feature` (`🧪️tests/🥒️.feature` of 1992, 1995, 1996 updated; scenarios added).
- Goldens: the scene goldens (`🧪️tests/🧫️fixtures/🏛️*/🔣️.json`) were 0-byte files and still are: run the scene test once with `SEMIO_BLESS=1` (1992) / `SEMIO_UPDATE_GOLDEN=1` (1995, 1996) after the first green build; the content changes (stations, fields, real contours, kernel modes) are all deterministic.

## 4. Honest limits and open data

- Not built or run: expect first-compile fixes in the new modules (`🌡️thermal`, `🎵️modes`, effect profiles) and in the touched closures of `🏗️structure-and-members` (EN 1995). Python oracle numbers above are real; Rust numbers are unverified.
- Thermal solver: siliceous aggregate, moisture 1.5 % (no snapshot field), lambda upper limit for both annexes (NA-DE NDP not verified), standard fire only, no spalling, reduced rectangle from the two centre lines (not the full zone method), symmetric faces from `exposed_faces(kind)`. Cost: about 1800 explicit steps for R120 on a 0.3 x 0.5 section (cached per member size/duration, eight entries).
- Real rebar and link positions: the snapshots have layer counts only (no `positions[]`, `stirrups.zones[]`, link outline, tendon control points): requests kept in `requests.md`; the cage keeps the rectangular outline of the schema pass.
- Masonry opening and load positions are still layout rules (no `WallOpening.x`, `ConcentratedLoad.position` in the snapshot); no plate solution, so `wall-lateral` is the 1D parabola, not a 2D panel field.
- EN 1995 vibration: the mode shape/frequency is a beam eigen-solution; the check's `floor_f1` ignores continuity (verdict and drawn frequency differ for two-span floors); no orthotropic plate mode (needs plate dimensions).
- EN 1992 flexure for `external`/FEM members and EN 1995 externally analysed internals have no stations; punching moment transfer is represented by the beta gradient, not by a real M_Ed; anchor stations are one per anchor.
- Concurrent edits: all changes were additions or surgical replacements of the quoted blocks; the EN 1995 and EN 1996 norm agents own the rest of the touched files.
