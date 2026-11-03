# SHOWCASE-B (wave 2): en1992, en1996, en1997 showcases
Nothing compiled (no cargo, no `repo run`). No git writes. freeze.txt named only en1994/en1995 during the edits. Python gates niced.

## Builders (`🧰️scripts/showcase/`)
- `en1992.py`, `en1996.py`, `en1997.py` (`--write` default, `--check`): explicit annex `De`, `reliabilityClass` where the schema has it, jsonschema Draft 2020-12 of every output plus the `common.finish` structural validation (every collection non-empty; en1997 borehole `readings` stay empty on purpose). `--check` unchanged for all three.
- New shared helpers: `dslfmt.py` (schema-driven DSL emitter: kebab keys, enum kebab values, nested `[ {..} ]` tables, `_` null, quantity units per field, ENUM header repair from the schema) and `oracle_feature_rows.py` (idempotent pass/fail rows from oracle JSON). DSL assets equal the output of `🛠️asset-laws/patch.py` (patch diff empty for all three; the other en1997 assets show header-only drift owned by the asset-laws pass, not touched).
- Existing fixtures win on rebuild (`merge`); to change a value, delete the generated fixture folders first.

## Content
- en1992 `rc-building-showcase`: 12 members (beam, slab, ribbed slab, column, shear wall, tension member, bridge girder, prestressed tank wall with 4 section points, 6 bars, 3 tendon points; flat slabs interior/edge/corner with 12/8/5 stud rows and UnbalancedMoments beta method; flat slab with opening), 2 concrete, 3 reinforcement and 2 prestress grades, 2 anchors through the fastening kernel (one cracked, one uncracked with head), 12 placements, the 115 catalogue cases of the demo document (FA-01..14 fastenings included). Values from committed fixtures; derived members use documented inputs.
- en1996 `masonry-showcase`: 5 walls (load-bearing wall with two concentrated loads and a seismic design situation; slender braced wall; shear walls with opening, lintel and shear bar; confined wall with piers, ties, prestress, confined element, flange and diaphragm stiffness/plan positions), chase, movement joint, placements, catalogue cases.
- en1997 `ground-showcase`: sand/clay/gravel layers (clay with C_u and N_kt), 2 footings, bored and driven pile (dynamic test data, negative friction, test profiles), 2 retaining walls (second with seismic coefficients and reinforcement), 2 slopes (second with slip circle), uplift plus heave case, anchor with tests, free-earth and cantilever embedded walls, 2 pile groups, direct-shear/oedometer/triaxial tests, 3 layer waters (incl. well), CPT x2, SPT and borehole investigations with readings, characteristic bases (derived and declared), 71 catalogue cases.
- Fixtures: `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json`, `🚦️check-outcome/{rc-building,masonry,ground}-showcase-{pass,fail}`; DSL assets `🖼️assets/{🏢️rc-building,🏘️masonry,🏘️ground}-showcase`.

## Oracles and feature rows (tolerance 1e-2, pass <= 0.95, fail >= 1.05)
- en1992: 107 rows (feature dry run 194 ok, 0 failed). Excluded on purpose, oracle gives sentinel/near-limit values: `column-C1` and `wall-SW1` (compression members, flexure sentinel 56250), `studs.slab-FS-corner` (1.255 in the pass document, stud spacing rule). Anchor checks (`en1992-4.*`) have no Python oracle and no rows.
- en1996: 101 rows (144 ok, 0 failed); pass document has 0 checks above 1.0, fail has 8 above 1.05.
- en1997: 70 rows (148 ok, 0 failed); pass document 0 above 1.0 (earth-pressure rows sit at exactly 1.0 by construction and are not used). The oracle has no investigation/characteristic-basis checks; those collections are populated and visible but unchecked by the oracle.

## Rust (unbuilt)
- `full_showcase()` in each `🧬️schema/📸️snapshot/🦀️.rs` (`include_str!` of the showcase fixture, pack-json decode as en1991), palette entry `full-showcase` ("Full showcase example" / "Vollständiges Gesamtbeispiel") in each editor, one unit test per editor (`the_full_showcase_populates_every_collection_and_is_in_the_palette`).
- Likeliest first-build issues: `examples()` trait in scope in the unit-test file, en1996 test paths `semio_s_artifact_norm_en1996_model::{MasonryWall, LoadCaseSituation}`, field names of the JSON decode (`Reject` policy) against the generated serde names, German `looks_english` on none (no new labels).

## Gates
`--check` x3 unchanged; jsonschema x3 ok; check-outcome feature dry runs ok (above); fixture checker `verify_fixtures.py`: en1992 0 failures, en1996 0 failures, en1997 2 failures that exist independently of this work (`insert-characteristic-basis` "addresses None, not an ordered collection", `remove-characteristic-basis` "collection this document does not carry": the characteristicBases kinds need the mutation owner); binary-tags ok (2531 leaves); i18n-mutations fails only on existing `change-pier-*` kinds ('pier' English word in German labels, not en1992/1996/1997). Heavy tests were niced single runs (heavy.sh not needed: no `repo` script). Packs (`🎒️.pack.semio`) not generated for the new assets.
- "- ready for rebuild: showcase-b" appended to `build-en1992/1996/1997/latest.md`.
