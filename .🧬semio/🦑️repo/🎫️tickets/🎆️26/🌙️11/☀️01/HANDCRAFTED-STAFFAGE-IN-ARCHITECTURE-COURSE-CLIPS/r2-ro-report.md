# R2 ro: optimization clip verification

## 1. Render review (quality l, 15 fps, en, one scene at a time)
Rendered and inspected frame sheets plus full-size frames of:
- Beat1_LocalGlobalExtrema (287 s): radiator (red) and cooler (light blue) beside the Heating/Cooling/Lighting formula rows and the lamp read correctly; occupant in the two section rooms stands at about 60 % of room height (1.75 m in 3 m), placed on the floor, no overlap with labels.
- Beat4_NormalizedIndex (295 s): thermal orange occupant in the room, correct scale and floor placement; room cooler/radiator not obstructed by labels.
- Beat3_FactorialExperiments (315 s): the four mini rooms show thermal occupant, cooler at the rear wall top right and lamps at consistent scale; labels do not collide.
No layout, scale or colour issue found; no scene code changed. Sampling, pareto and algorithms beats show no staffage and were not rendered.

## 2. Tests
Runner: `🧰️ro/tests.sh` (uses `run-g3-tests.py`, SEMIO_TEACHING_OUTPUT_ROOT set, SEMIO_TEST_MANIM=1). Modules: `🧪️tests/🐍️.py`, algorithms comparison and widgets tests, experiments consumed test.
- Full run: 75 tests, 1782 s, 74 OK, 1 failure, 0 errors. `test_all_clauses_fit_both_languages_and_audiences` completed and passed (about 30 min total suite on a loaded host).
- The one failure: `SchemaFirstStudy.test_schema_file_exists_and_is_a_draft07_document_forbidding_extras` asserted `$schema` == draft-07.
- Diagnosis: genuine stale test, not a schema bug. `🧮️model/🧬️schema/🔣️.schema.json:2` declares draft 2020-12, which is the convention in every other schema of the repo (visuals, icons, clip, audience, output). Fixed the test (`🧪️tests/🐍️.py` around line 1186): renamed to `..._is_a_draft_2020_12_document_...` and asserts `https://json-schema.org/draft/2020-12/schema`.
- After the fix, `SchemaFirstStudy` re-run: 2 tests OK. Expected full-suite result now: 75 tests, 0 failures, 0 errors (the other 74 passed in the full run).

## 3. Open
Nothing. Generated output in `🗑️generated/ro` removed; scripts kept in `🧰️ro/`.
