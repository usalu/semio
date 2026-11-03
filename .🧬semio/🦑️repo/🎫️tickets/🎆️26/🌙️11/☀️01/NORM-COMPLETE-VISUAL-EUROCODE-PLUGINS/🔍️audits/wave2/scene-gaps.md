# 🧊 Wave 2 Scene Gaps (EN 1990–1999)

Read-only audit against `📐️specs/visual-editor-architecture.md` §3.6 (per-artifact entity → visual map), §3.7 (placement), §4.1–4.6 (windows, viewer), §6.2 (handles), and `🔍️audits/scene-coverage.md`.

Scene root: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/`
Legend: ✓ present in scene code, ◐ partial or unverified by grep, ✗ missing. "3D" counts are `EntityKind::` uses in non-test scene code.

## Summary

- All ten scenes project 3D entities, 2D diagrams, handles and a clause-badged overlay set. No scene is empty.
- Deformation or mode shapes exist in 1990 (member elastic line), 1992 (deflection overlay), 1993 (buckling and LTB mode, deflection), 1997 (settlement dish track), 1998 (eigenmode track and mode profile), 1999 (flexural buckling mode, deflection line, local buckling mode). None exists in 1991, 1994 (2D statics only), 1995 (2D vibration and deflection only) or 1996.
- Most §3.6 rows are covered. The main gaps are listed per artifact below, with 1996 and 1991 the weakest.
- Golden scene fixtures are empty in all ten artifacts (0 bytes). Four exist at the expected path (1990, 1993, 1996, 1999). 1992 has a 0-byte fixture under the wrong folder name. The other five have no fixture folder.
- Editor mode mounts viewport, diagrams, properties, checks and outliner in all ten artifacts. Viewer mode mounts viewport, checks and diagrams only, as §4.6 requires, in all ten.
- Two cross-cutting defects: persisted placements (§3.7) are never passed to the projection, in either editor or viewer. The 1995 viewer still has an unmounted `📊️report` directory that §4.6 says should be replaced.

## Cross-cutting findings

1. **Placements not wired (§3.7).** Every artifact's editor viewport calls `project(SceneInput { placements: &[], revision: SceneRevision { document_seq: 0, report_seq: 0 }, .. })` (for 1990: `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs` line 29, `scene_of`). The viewer viewport does the same (`👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️viewport/🦀️.rs`, `render_with`). So a dragged `Placement` never changes the scene, and viewer and editor cannot diverge in the way the spec requires once placements exist. The `document_seq` and `report_seq` stamps are also always zero, so the incremental scene diff (§5.4) cannot key on revisions.
2. **Viewer combination fixed to governing.** The viewer passes `combination: None`. §4.6 allows only camera, layer and colour-map changes in the viewer, so this is consistent with the spec. Recorded for information only.
3. **Golden fixtures empty.** See the fixture table below. No test can currently assert the golden scene.
4. **Viewer leftover (1995).** `👁️viewer/🎭️modes/👁️view/🪟️windows/📊️report/` exists on disk. The view mode mounts only viewport, checks and diagrams, so the report window is not mounted. §4.6 says it should be replaced by the three shims, so the directory should be removed.
5. **Editor mount (1995).** `✏️edit/🪟️windows/📥️inputs/` and `📊️results/` exist on disk. The edit mode mounts only the five standard windows. Confirm whether inputs and results are intended as extra windows; if so, they must be mounted or removed.

## Golden scene fixtures (`🧪️tests/🧫️fixtures/🏛️scene/🔣️.json`)

| Artifact | Path status | Size |
|---|---|---|
| en1990 | exists | 0 bytes (empty) |
| en1991 | no `🧫️fixtures` folder | n/a |
| en1992 | `🧫️fixtures/🏛️office-frame/🔣️.json` (wrong folder, no `scene` fixture) | 0 bytes |
| en1993 | exists | 0 bytes (empty) |
| en1994 | no `🧫️fixtures` folder | n/a |
| en1995 | no `🧫️fixtures` folder | n/a |
| en1996 | exists | 0 bytes (empty) |
| en1997 | no `🧫️fixtures` folder (oracle test under `🔮️oracle`) | n/a |
| en1998 | no `🧫️fixtures` folder (oracle test under `🔮️oracle`) | n/a |
| en1999 | exists | 0 bytes (empty) |

## Editor and viewer mode layout

| Artifact | Editor edit mode mounts | Viewer view mode mounts | Viewer shims |
|---|---|---|---|
| en1990–1994, 1996–1999 | viewport, diagrams, properties, checks, outliner | viewport, checks, diagrams | render shared app-surface functions; checks uses no controller |
| en1995 | viewport, diagrams, properties, checks, outliner (inputs and results dirs present, not mounted) | viewport, checks, diagrams (report dir present, not mounted) | as above |

Read-only status of the viewer rests on the shared render functions not dispatching commands. The viewer tree contains no explicit `ReadOnly` marker (grep count 0). The checks shim passes `ChecksViewState::default()` and no controller, which is correct. The viewport shim uses `CONTROLLER = "s.norm.en1990@1/*#viewer"`, which should be checked against the shared handler to confirm it cannot emit edits.

## Per-artifact findings

### en1990 (⚖️)

- 3D: Member (7), Load (6), Plate (3), Group (4), Support (1). Combination load glyphs (Point, Line, Impact) carry ψ via `LoadCaseRef`. Seismic loads take the Warning tone. Governing combination shown.
- Handles: member length and axes (ScalarAlong), action magnitudes (ScalarAlong), combination stepper (CountStepper).
- Deformation: SLS member elastic line (`🏗️members`, exaggerated between ×1 and ×200). Generic member, not bridge-specific.
- 2D: combination matrix ✓ (heat table, not a bar chart), ψ-factor plot ✓, reliability index and FORM ✓, k_n ✓, fatigue S–N ✓.

§3.6 gaps:
- ✗ SLS limit lines (no SLS limit series; only the limit-state line in FORM).
- ◐ γ·ψ bar chart (matrix is a table, not bars).
- ◐ Bridge SLS deflected shape (generic elastic line, no bridge-specific output).

### en1991 (🏋️)

- 3D: Solid (20, floors, roofs, self-weight stack, silos, crane), Load (20 incl. Area, Pressure, Thermal, Impact, Explosion), Surface (8), Shell (3), Support (2), Member (2).
  - Floors area loads ✓ (`floor`), roof snow zones ✓ (`roof_zones`), wind c_pe zones A–E as pressure zones ✓ (`cpe_candidates`), q_p arrows ✓ (LoadGlyph::Pressure), self-weight by unit weight ✓ (`self_weight` colour by γ), accidental impact and explosion glyphs ✓.
- Handles: building height, width, depth, storeys, altitude, wind zone, terrain (all grips); roof pitch, parapet, obstruction; floor area; self-weight thickness; face z and area; bridge span, lane width, lane count; crane hoisting speed; silo height.
- Deformation or mode shapes: none. Only the gravity and fire scenes and the bridge influence line are 2D.
- 2D: q_p(z) ✓, c_pe over loaded area ✓, snow μ₁/μ₂ and μ₃ ✓ (μ vs pitch ◐ — pitch axis not verified), fire curves ✓, bridge influence line ✓, silo pressure ✓.

§3.6 gaps:
- ✗ Load-duration / combination ψ chart (no ψ or duration diagram; ψ only in combination data).
- ◐ Snow shape factors vs pitch (μ shown per pitch class; pitch axis unverified).
- ◐ "Volume colour by γ": colour is by unit weight, no volume term.

### en1992 (🏛️)

- 3D: Solid (5, concrete cover shell, punching columns), Fastener (9, anchors headed, group and cone), Member (4, beams, columns, walls, slabs), Surface (6), Plate (2), Load (3), Support (1). Rebar entities (`MaterialRole::Rebar`, link cages, longitudinal bars).
- Handles: longitudinal count stepper, bar diameter (Radial), anchor grips, FaceOffset for cover and punching perimeter.
- Deformation: member deflection overlay (`deflection_overlay` in `🏛️members`, id `en1992.7.4.deflection.*`) ✓.
- 2D: M–N interaction ✓, stress block ✓, tendon ✓, deflection ✓, fire ✓, materials ✓, crack width vs stress (`wmax`) ✓ (◐ crack pattern overlay not confirmed).
- 3D overlays: punching u0 and u1 rings ✓ (`🕳️punching`).

§3.6 gaps:
- ✗ Shear truss (cot θ): no diagram or 3D output (`cot|truss` 0 hits).
- ✗ Strut-and-tie: no output (0 hits).
- ✗ Shear plane overlay.
- ◐ Crack pattern overlay (crack width exists; no crack pattern in 3D confirmed).

### en1993 (🔩️)

- 3D: Member (18), Group (7), Load (5), Fastener (4, bolt groups), Plate (5, plates and panels), Shell (4, silo and tank), Support (2). Welds (`MaterialRole::Weld`). Tension cables and rods, tower legs, piles, crane runways present (`🌀️eigen-modes`, `🔗️joints`, `🧱️plates-and-shells`).
- Handles: member length and offsets, weld length and throat (DimensionLine, FaceOffset), bolt pitch, gauge, end and edge distance, rows and per-row (DimensionLine, CountStepper), panel a, b and thickness, silo radius, depth and thickness, pile embedded length.
- Deformation or mode shapes: ✓ eigen modes (flexural buckling, LTB mode shape with exact load factor) and static deflection (`🌀️eigen-modes`).
- 2D: buckling curves ✓, M–N ✓, S–N ✓ (wohler), damage fraction ✓, fire k_y,θ and steel at rating ✓, cross-section class chart ✓, LTB ✓, shape ray ✓ (`ray`, `ray-ratio`), spectrum ✓.
- Overlays: LTB marker ✓ (`marker-ltb`), spacing marker ✓.

§3.6 gaps:
- ✗ Haunches (0 hits).
- ✗ Gussets (no output).
- ✗ Effective-width sketch (0 hits).
- ◐ End plates (one reference only).
- ◐ Plated panel stiffeners (2 hits).
- ◐ Plate buckling modes overlay (not confirmed as a 3D output).
- ◐ Block tearing overlay (not checked).

### en1994 (🧩)

- 3D: Member (16, composite beams and columns), Plate (8, deck and slab), Fastener (4, studs), Load (4), Support (4), Surface (2). Stud pattern (101 hits), deck ribs (`🧱️slabs`), encased and filled columns (`🏛️columns`).
- Handles: beam span, depth and offsets (ScalarAlong, FaceOffset), column height, slab span and depth, orientation (CountStepper).
- Deformation: 2D only. `🌀️statics` gives a normalised deflection line (diagram `deflection`). No 3D deformed shape.
- 2D: plastic stress blocks ✓ (full and partial), M–N ✓ (columns), shear connection degree η ✓, stud load–slip ✓, shear flow ✓, slab moment ✓, column buckling ✓.
- Overlays: slip (`stud_slip`) ✓ as 2D; deck sheeting failure ✓ (`en1994.9.3.` failure tag in slabs).

§3.6 gaps:
- ◐ Slip arrows at interface in 3D (2D stud slip present; 3D arrows not confirmed).
- ◐ M–N polygon for beams (only columns).

### en1995 (🪵)

- 3D: Timber (4 roles, glulam 18 hits), Fastener (4, nails, screws, dowels, bolts), Solid (3), Member (6), Group (4), Plate (1), Support (1), Load (1), Surface (2). Steel plates only one.
- Handles: member length, widths and faces, connection spacing (DimensionLine), end distance and edge distance (ScalarAlong), number, rows, diameter (Radial), t1, t2, planes.
- Deformation: none in 3D. 2D only: deflection diagram, vibration diagrams (`🎵️modes`).
- 2D: k_mod ✓, embedment ✓, Johansen ✓, spacing ✓, kcrit ✓, buckling ✓, fire ✓.

§3.6 gaps:
- ✗ Effective number n_ef diagram (only referenced in a feature station, no output).
- ✗ Splitting-plane overlay (no split output in scene code).
- ✗ Row shear block overlay.
- ◐ Steel plates in 3D (one Plate entity).
- Cleanup: remove or mount `👁️viewer/.../📊️report` (see cross-cutting 4). Confirm `📥️inputs` and `📊️results` on the editor side (cross-cutting 5).

### en1996 (🪨)

- 3D: Solid (7, walls and openings), Plate (7, lintels and slabs), Load (5, concentrated patches, uniform), Group (3), Fastener (2), Surface (3), Support (1). Bond pattern (`🧩️bond`, 28 hits), openings (147 hits), lintels (22 hits), concentrated loads with 45° spreading (27 hits).
- Handles: wall length, height, thickness, sides; opening width, height, sill; slab bearing depth and span; resultant eccentricity; concentrated bearing length.
- Deformation: none.
- 2D: eccentricity diagram (top, mid, bottom) ✓, slenderness reduction Φ(λ) ✓ (`slenderness`, `reduction`, `phi`), strength–eccentricity envelope ✓ (`strength`, `mn`), capacity ✓, shear ✓, lateral ✓, fire ✓, interaction ✓.
- Overlay: failure glyph ✓ (1).

§3.6 gaps:
- ✗ Flanges (0 hits).
- ✗ Cracked zones overlay.
- ✗ Stress block under concentrated load overlay.
- Golden fixture: 0 bytes.

### en1997 (🌍)

- 3D: Solid (23, strata, footings, piles, walls), Surface (9, wedges and slip surfaces), Plate (3), Load (7), Group (4). Strata columns (`🧱️ground` and layers 215 hits), footings with effective area (`🦶️footings`), piles (`🏗️piles`), retaining walls with active and passive wedges (`🧱️walls`), slopes with slip circle (`⛰️slopes`), uplift cases (`💧uplift`), ground settlement track (`📉️settlement`).
- Handles: layer boundaries and groundwater (ScalarAlong), footing width, length, embedment, load-h (FaceOffset, ScalarAlong), wall height, embedment, base width, stem, slope height and angle (Angular), pile length, diameter (Radial), count, translate for positions.
- Deformation: ground settlement dish as a ghost track (`🎞️tracks`, Terzaghi consolidation time). No structural deformed shape.
- 2D: earth-pressure vs depth ✓, CPT and SPT profiles with layer overlay ✓ (`cpt-profile`), bearing capacity factors ✓ (N-factors, not q against B), load–settlement ✓.
- Overlays: slip surface ✓ (slopes), Prandtl mechanism wedges ✓ (kernel `🧮️mechanics`, applied in footings and walls), heave zone ✓ (`Uplift and heave` layer).

§3.6 gaps:
- ◐ Bearing capacity chart q vs B: the diagram shows N-factors, not a q–B curve.
- ◐ SPT profile: only the generic `profile` function is confirmed; a separate SPT output was not found.

### en1998 (🫨)

- 3D: Plate (11, slabs, foundations), Solid (14, walls, storeys), Shell (6, silos, tanks, towers), Member (6), Load (7, storey mass glyphs and inertial), Group (5), Support (2). Bridges and foundations (`🌉️ground-and-bridges`), buildings and storeys (`🏢️buildings`), vessels and towers (`🏭️vessels-and-towers`).
- Handles: plate lx and ly (FaceOffset), slab h, mass g, solid x and y (Translate), member h, shell h and r (Radial), wall h.
- Deformation: eigenmode ghost track (`🎞️tracks`, one mode per layer) ✓ and mode profile diagram ✓. Modes computed from stiffness and mass matrices (`🧮️response`).
- 2D: Type 1 and Type 2 horizontal spectra ✓ (`spectrum_type1` switch), vertical spectrum ✓, DE-NA zone chart ✓ with T1 marker, drift profile ✓, eigenmode profile ✓.
- Overlays: mode shape overlay ✓, soft-storey highlight ✓ (`soft_storey`).

§3.6 gaps:
- ✗ Capacity curve (pushover): no output (0 hits).
- ✗ Behaviour-factor q chart: q is computed (`🧮️response`) but has no diagram.
- ◐ Storey force profile: only drift profile found.
- ◐ Equivalent lateral forces in 3D: only three `lateral` hits, not verified as glyph output.

### en1999 (🪶)

- 3D: Member (17, aluminium extrusions with HAZ bands), Shell (5), Group (6), Fastener (4), Plate (2), Load (5), Support (2). Extruded Polygon sections (44 hits), cold-formed and sheets (`🥫️sheets-and-shells`), connections (`🔗️connections`), fire scenario (`🔥️fire-and-fatigue`).
- Handles: member length, buckling and LTB lengths, section depth and width and thickness (tied to plate elements), weld positions, HAZ extent (DimensionLine), bolt pitch, gauge, distance.
- Deformation: flexural buckling mode and deflection line (`🌀️eigen-modes`) ✓, local buckling mode (`local_mode` in `🏗️members`) ✓.
- 2D: alu buckling curves classes A and B ✓, HAZ ρ chart ✓ (`haz`, `rho-o-haz`, `rho-u-haz`), S–N alu ✓ (`wohler`), fatigue ✓, fire ✓, shell ✓, LTB ✓.
- Overlays: HAZ bands ✓ (translucent strips in 3D, feature test), HAZ extent handle ✓, local buckling mode ✓.

§3.6 gaps:
- ◐ Section effective thickness diagram: effective bands computed (`🧮️facts` `local_bands`) but no 2D output with that id.

## Full coverage matrix (§3.6)

| Artifact | 3D entities | 2D diagrams | Overlays | Status |
|---|---|---|---|---|
| en1990 | ✓ | ◐ (γ·ψ bars, SLS lines missing) | ✓ governing | 2 gaps, 1 partial |
| en1991 | ✓ | ◐ (ψ/duration chart missing) | ✓ clause badges | 1 gap, 2 partial |
| en1992 | ✓ | ◐ (cot θ truss missing) | ◐ (punching ✓, shear plane and strut-and-tie missing) | 3 gaps |
| en1993 | ✓ | ✓ (effective width missing) | ◐ (block tearing, plate buckling unverified) | 4 gaps, 4 partial |
| en1994 | ✓ | ✓ | ◐ (3D slip arrows unverified) | 2 partial |
| en1995 | ✓ | ◐ (n_ef missing) | ✗ (splitting, row shear) | 3 gaps |
| en1996 | ✓ | ✓ | ✗ (cracks, stress block, flanges) | 3 gaps |
| en1997 | ✓ | ◐ (q–B chart missing as q–B) | ✓ | 1 partial |
| en1998 | ✓ | ✗ (capacity curve, q chart missing; force profile partial) | ✓ | 2 gaps, 2 partial |
| en1999 | ✓ | ◐ (effective thickness output) | ✓ | 1 partial |

## Open items

- Placements (§3.7) are unwired in every artifact's editor and viewer. This blocks direct manipulation (§6) from persisting visibly.
- All ten golden scene fixtures are empty; no golden test can assert the scene.
- The 1995 viewer `📊️report` directory should be removed, and the 1995 editor `📥️inputs` and `📊️results` should be confirmed or removed.
- Unverified by this audit: 3D rendering of slip arrows (1994), steel-plate stiffeners (1993), tendon geometry (1992), and whether the viewer's `CONTROLLER` string is wired to a non-dispatching handler.
