# WP-21 EN 1999 visual vertical

Status: phase 1 (scene) and phase 2 (editor and viewer windows) WRITTEN. Build status: see "Verification" at the end (updated when the queued `check` and `test` exit).

Root: `NORM/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any` (`SUBSET`). Rollout checklist followed: `📌️important/artifact-rollout-checklist.md`.

## 1. Scene crate `semio-s-artifact-norm-en1999-scene` (`SUBSET/🎬️scene`)

Registered: member in `🪶️en1999/Cargo.toml`, `[workspace.dependencies.semio-s-artifact-norm-en1999-scene]`, `[workspace.dependencies.semio-s-plugin-norm-structural-scene]`, package name in `📦️packages/🦀️rust/📜️script.ts`.
Deps: `-checks`, `-model`, `norm-compliance`, `norm-structural-scene`; dev: `serde_json`.
Public API (same shape as EN 1993): `project(&SceneInput) -> StructuralScene`, `diagrams(document, report, focus)`, `handles(document, placements, selection)`, `Layout`, `Placement`, `ENTITY_KINDS`, `entity_kind_of`, `entity_dependencies`.

| Module | Content |
|---|---|
| `🧰️kit` | entity builder (`Spec` with an optional style override), utilisation binding, layers (extrusions, HAZ, effective sections, connections, sheets, shells, actions and fire, supports, annotations), legends, entity-kind registry (member, connection, coldFormedSheet, shell, fireScenario, fatigueDetail), `section_shape` (extruded I, channel, angle and round tube as parametric profiles, box/RHS as a polygon with a hole, fallback `Rect`) |
| `📍️layout` | default rule: members along +X with a 1.5 m gap, cantilever posts upright (fixed base), sheets at y = -6 (span + 1 m slots), shells at y = -12 (3 m slots); persisted placements win |
| `🧮️facts` | numbers drawn from the very clause functions the checks use: alloy band at the thickest wall, plate strips (`part_1_1::strips`) under three contexts (HAZ only, local buckling only, both) converted to outline coordinates, η recovered exactly from the reported β/ε of the first internal plate, element classification and ρ_c |
| `🏗️members` | extrusion along the frame; HAZ bands (one translucent, dotted-hatch Weld-role extrusion per strip reduced by ρ_o,haz, inflated by 0.5 mm against z-fighting, leader label with b_haz and ρ_o,haz); effective-section ghost (composite of the thinned strips, Ghost style, local buckling half-wave overlay on the lateral axis, leaders with ρ_c and t_eff per reduced strip); buckling and LTB mode overlay (governing of flexural y, flexural z, LTB when utilisation >= 0.5); failure glyphs (flexural, LTB, yielding); supports (pinned/roller, intermediate, fixed base), lateral restraints at L_LT or at quarter points when fully restrained; design-action glyphs (N, V_z, M_y, resultant of the line load); dimensions of length and section depth with field bindings; clause badge |
| `🔗️connections` | connected plate, bolt grid (rows x bolts per row, pitch, gauge, hole clearance from the EN 1999-1-8 rule) or rivet grid (Dowel with round head), fillet weld, HAZ extent slab (b_haz from the document or from the table, with dimension bound to `welds.hazExtent`), selectable group, action glyphs |
| `🥫️sheets-and-shells` | cold-formed sheet as a ThinWalled top-hat profile extruded along the span, supports, span dimension; shell as a revolved cylinder, r/t label, failure glyph |
| `🔥️fire-and-fatigue` | fire scenario as a Thermal load glyph at the member temperature of the same lumped-capacity integration the check uses; fatigue detail as a selectable group with Δσ_C label, clause badge and fatigue glyph |
| `📈️diagrams` | `buckling` (classes A and B, Euler hyperbola, y and z markers bound to the buckling lengths through the exact linear slenderness law), `ltb` (classes 1/2 and 3/4, marker), `ltb-spacing` (M_b,Rd over L_LT, design moment limit, marker bound to `ltbLength`), `mn` (capacity polygon, linear for class 3/4 or welded and plastic exponents otherwise, utilisation ray, demand marker of the combination with the largest ray ratio), `class` (β/ε per plate against the three class limits, class and ρ_c in the table twin), `effective` (ρ_c over β/ε for the four support/weld types with the plates as markers), `haz` (ρ_o,haz, ρ_u,haz over the plate thickness, markers bound to the plate thickness in mm), `hazwidth` (b_haz over the thickness), `sn` (Δσ_R of the detail and seven neighbouring detail categories on log-log axes with m_1/m_2 of the detail, Δσ_D and Δσ_L limits, demand marker bound to `nCycles`), `fire` (ISO 834 gas and aluminium temperature against θ_cr, marker bound to `durationS` in minutes), `fire-k` (k_0.2proof,θ and k_E,θ), `shell` (χ_x over λ̄_x). Every diagram has a table twin and `bound_check` = report check id |
| `🕹️handles` | member: length, buckling lengths y and z, L_LT, depth, width, flange and web thickness (round tube: diameter and wall), one weld-position grip per welded plate; plate elements stay tied to the section by value match (web width = h - 2 t_f, flange width by kind, thickness of tied flanges/webs) so one drag writes the section leaf and all tied plate leaves as one commit; connection: pitch, gauge, edge distance, rows, per row, bolt diameter, plate thickness, weld length, throat, HAZ width; sheet: span, flat width, thickness; shell: radius, length, thickness; fire: duration and start temperature |

## 2. Tests, oracle, feature

- `🧪️tests/🔬️unit/🦀️.rs`: 31 tests. Validity of both fixtures, unique ids and layers, determinism and locale-free names, closed-form area of the I extrusion, rhs polygon with hole, default layout and placement, utilisation binding, failing member (error style, hatch, glyph, mode shape), HAZ band count and exact inflated area, effective ghost with the reported ρ_c, combined connection (grid, fillet, HAZ length = b_haz, 4 children), rivets and pure welded/bolted variants, sheet/shell/fire/fatigue geometry, supports and restraints, action glyphs, handle existence, snapping and tied plate writes, derived ids, dependencies, diagram validity with table twins, buckling marker on its class curve with the length binding, M-N demand inside the polygon, HAZ marker thickness binding, focus narrowing, S-N axes and category curves, golden scene, Python oracle for both fixtures, 1000-member timing.
- `🧪️tests/🥒️.feature`: five scenarios (extrusion + HAZ, class 4 effective section, connection, tied handles, diagrams + oracle).
- `🔮️oracles/🐍️.py` (third-party check, shapely when installed, shoelace otherwise): closed-form I area vs polygon and document sections, net area of polygon sections (outer minus holes), HAZ rectangles inside the section, buckling classes A/B and LTB classes 1/2 and 3/4 (alu reduction law) for every curve and marker, M-N ring area and demand-in-polygon, S-N law for the detail and the neighbouring categories (all points) and Δσ_D/Δσ_L, b_haz rule (20/30/35/40 mm), class table (class from β/ε and limits, ρ_c = 1 below class 4), ρ_c(β/ε) from the Table 6.3 constants for both buckling classes, ISO 834 gas curve and an independent explicit integration of the aluminium temperature (EN 1999-1-2 4.2.5, dt = 5 s), k_θ monotone from 1, EN 1999-1-5 shell reduction with an independent α_x. Spawned through `norm::python_oracle::run_ok`.
- Golden `🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` is empty until the first green run with `SEMIO_UPDATE_GOLDEN=1`.

## 3. Editor and viewer (phase 2)

- Editor: deleted `📥️inputs` and `📊️results`; new `🧊️viewport`, `📈️diagrams`, `🧾️properties` (typed inspector + outliner), `✅️checks`; four-window layout (properties 22 %, model 50 %, checks above diagrams 28 %); `.interaction(viewport_interaction_definition(...))` with the seven scene kinds; `render` arms with the focus entity of the selected check (diagrams fall back to the worst entity); `ChecksWindowConfigOwner` replaces the results owner. Typed TS twin lines of the deleted windows removed.
- Viewer: deleted `📊️report`; read-only `🧊️viewport`, `📈️diagrams`, `✅️checks` shims (no editor import), view layout 50/25/25.
- `mutations::resolve` (`🧬️mutations/🧭️edit-rules`): dedicated kinds keep priority (annex, loads, buckling length, plate thickness, weld throat, bolt rows); every other `collection[id=ID]...leaf` raises the whole-list `Change*` mutation with the leaf set (materials, sections, members, connections, fireScenarios, fatigueDetails, coldFormed, shells). `set-field`, `insert-item`, `remove-item`, `apply-remedy` use it.
- Field meta: the existing table already contains a row for every handle path and diagram marker binding (lengths, buckling lengths, `sections[].…`, element thickness/width/weldPosition, `connections[].bolts.*`/`welds.*`, `fireScenarios[].durationS/thetaA`, `fatigueDetails[].nCycles`, `coldFormed[].…`, `shells[].…`); no row added.
- Editor tests: manifest declares exactly the four windows, every body key renders without a JSON dump; mode tests list the four window kinds.

## 4. Known approximations and open points

1. Mode shapes are sinusoidal indications (L/25 peak for member modes, 0.4 % of L for the local plate half-waves), not eigenmodes; the LTB mode has no twist component.
2. The section outline comes from `kind` plus plate elements; there is no free polygon in the snapshot (request filed: `outline`/`holes`). Box and RHS sections do use a polygon with a hole.
3. One weld position per plate; both flange outstands of an I section share it (handle drags the right one, the checks mirror it).
4. Per-station utilisation along members is not produced (entity-level colour); `Placement` snapshot fields and `update-placement` do not exist yet (the scene accepts them).
5. M-N polygon ignores M_z and the class 4 shift e_N (demand lies inside the drawn polygon whenever the check passes); the demand marker is the ULS combination with the largest ray ratio.
6. Drags commit through `setField` (one leaf per write); the `setFields` lane and transient previews are not available in the command macros yet (same open point as EN 1993).
7. Tied plate widths are matched by value; a plate whose width was edited away from the formula is no longer dragged with the section.
8. Requests filed in `📌️important/requests.md` (Placement, section polygon, weld runs and transverse HAZ, shared member resistances, CheckUnits).

## Verification

Never compiled by this agent: my queued build was cancelled per the one-build-per-artifact rule; "ready for rebuild" is in `🗑️generated/build-en1999/latest.md`. The norm agent runs check then test (first test run with SEMIO_UPDATE_GOLDEN=1).
