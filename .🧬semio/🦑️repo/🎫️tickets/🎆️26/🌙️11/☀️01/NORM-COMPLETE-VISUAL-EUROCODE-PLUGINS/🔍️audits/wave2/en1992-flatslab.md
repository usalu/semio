# EN 1992 flat slab with punching reinforcement (wave 2)

Nothing compiled or run on the Rust side. Python run (niced): both oracles on the new fixtures, check-outcome `--self-check`, jsonschema validation. Rebuild requested ("- ready for rebuild: en1992 flat slab" in `build-en1992/latest.md`).

## 1. Example document
`En1992Snapshot::flat_slab_punching()` (`🧬️schema/📸️snapshot/🦀️.rs`, built from the office frame's flat slab template): DE annex, C30/37, B500B, three members (1 m strip, h 0.30 m, d 0.255 m, span 6 m, Ø14 x 8 bottom), one per column of 300 x 300 mm, each with four persisted stud rings (Ø12), characteristic punching loads (G / Q kN) by tributary area:

| member | position | G / Q | rings (radius m x studs) | stud layout | punching (oracle) |
|---|---|---|---|---|---|
| slab-FS-interior | Interior | 300 / 110 | 0.10 0.24 0.38 0.52 x 12 | pass 0.784 | 0.476, crushing 0.723 |
| slab-FS-edge | Edge | 160 / 60 | same radii x 8 | pass 0.784 | 0.515, crushing 0.869 |
| slab-FS-corner | Corner | 85 / 30 | 0.10 0.24 0.38 0.62 x 5 | FAIL 1.255 (radial gap 0.24 m > 0.75 d) | 0.477, crushing 0.868 |

Palette entry `flat-slab-punching` ("Flat slab with punching reinforcement" / "Flachdecke mit Durchstanzbewehrung", derived DSL like "Full catalogue example"). Loads and geometry are plausible demo inputs; the layout limits stay the unverified ones of v10.

## 2. Wiring
- Editor palette: `✏️editor/🦀️.rs` (`flat_slab_punching_example`). Palette tests (titles, byte-identical DSL round trip) cover it automatically; new test in `✏️editor/🧪️tests/📚️assets/🦀️.rs` (both titles, equals constructor, equals the committed oracle JSON fixture, live and full scene under the projection laws).
- `assert_traced`: via `report_ids()` fixtures of `⚖️checks/🧪️tests/📍️reach/🦀️.rs`; remedies fixtures; scene `fixtures()` (all scene-law, handle-law, track tests); `incremental_laws!` documents; compliance tests (explanations EN/DE, jsonschema).
- Scene path read: `🎬️scene/🕳️punching/🦀️.rs` already emits column solid, `u0`/`u1` surfaces (u1 carries the `punching-perimeter` field coloured per station from check `en1992.6.4.punching.<id>`), stud fasteners per ring, a radius grip on stud 0 and a count grip on stud 1 of each ring (`studRings[id=..].radius|count`, rules exist).
- Scene fixes (found by reading): (a) `points_along` distributed studs over the whole clipped ring including the closing segment on the free slab edge, so edge/corner studs sat on the slab edge and differed from the layout check's perimeter; now only the open arc path is used (`free` predicate). (b) Studs were bound to `en1992.6.4.` only, so a failing layout never coloured them; now bound to the worst of `en1992.6.4.` and `en1992.9.4.3.studs.`.
- New scene test `the_flat_slab_example_shows_perimeters_stud_rings_utilisation_colours_and_bound_stud_grips`: u0/u1/column per member, u1 field channel bound to the punching check, one fastener per stud, no overlapping studs, count and radius grips exist, are bound to the document value and attached to studs, corner studs bound to the failing stud check (Fail), interior studs not failing.

## 3. Remedies (so the failure shows a fix)
`⚖️checks/📋️member-evaluation/🩹️minimal-remedies/🦀️.rs`: new fields `StudRingRadius`, `StudRingDiameter`. Plan: `en1992.9.4.3.studs.*` lowers the radius of the outer ring of the widest radial gap (corner: ring r4 to the largest value with gap <= 0.75 d, about 0.571 m); `en1992.6.4.punching` with stud rings raises the diameter of the innermost ring (before, the `punching.asw` remedy was ineffective once rings exist because `stud_asw()` overrides it). Tests in `🧪️tests/🩹️remedies/🦀️.rs` (minimal and one step wider fails; punching failure repaired through the stud diameter); `apply`/`plan_paths` know the ring paths; flat slab added to the remedy fixtures.

## 4. Oracle rows
- `🔮️oracles/🚦️check-outcome/🐍️.py`: new `punching_rows` (v_Ed = beta V/(u1 d) of the governing ULS combination, v_Rd,cs = 0.75 v_Rd,c + 1.5 d/s_r A_sw f_ywd,ef/(u1 d), s_r = 0.75 d, innermost ring as A_sw; crushing limit 1.4 v_Rd,c for DE only), `control_perimeter` by shapely (buffered column cut by the slab, free-edge part removed; edge and corner), `stud_layout_row` now for all column positions with shapely ring perimeters; `--self-check` also compares the closed-form interior/edge/corner perimeters to the shapely cut buffer (green).
- `🧬️schema/🧪️tests/⚖️compliance/🐍️.py`: stud rows for all positions (closed-form perimeters); its punching rows stay filtered (`MOVED_TO_CATALOGUE_ORACLE`). New Rust test `the_stud_layout_of_every_column_position_matches_the_python_oracle_and_the_hand_values` (hand values, Rust, Python equal to 1e-9).
- Fixtures `🧪️tests/🧫️fixtures/🚦️check-outcome/flat-slab-punching/` (the document) and `flat-slab-punching-bare/` (same, no rings; generator `🧰️scripts/en1992-flatslab-fixture.py`).
- Feature rows `🧪️tests/🚦️check-outcome-en1992-1/🥒️.feature` pass-39..45, fail-46..48: punching (3 pass), crushing (2 pass), stud layout (2 pass, corner FAIL 1.2549), bare edge and corner punching FAIL (1.2172, 1.2159). Values are from the Python oracle; the Rust side is unverified until the first run.

## 5. Unverified / open
- Rust vs oracle agreement (nothing compiled); the DE NA reduction of C_Rd,c for u0/d < 4 is not applied by Rust nor the oracle (the corner column has u0/d < 4: NA text not checked, unverified).
- The stud area A_sw uses the innermost ring only (production formula, mirrored by the oracle); layout limits remain the unverified v10 constants.
- Fire and l/d rows of the new document (l/d 0.85 by oracle) are not asserted; fire is not covered by the oracle.
- Interior bare punching is 1.012 (near limit), so no feature row.
