# WP-P3 — Plugin Test Gate Batch B

Slice: P3. Scratch: `.tmp-ticket/wp-p3/`. Private cargo target: `.tmp-ticket/wp-p3/target`.
Budget mechanism: reuse P1 schema-first `test-level-budgets` + `nextest.toml` package overrides (batch B floors added).

## Status table (17 plugins)

| Plugin | test-quick | Notes |
|--------|------------|-------|
| layout | PASS 3/3 | measured |
| cad | PASS 5/5 | measured; extensions pending |
| norm | FAIL 32/33 | only `descriptor_is_fresh` (logic PASS after framework fan-out + presence retirement) |
| playbook | FAIL 2/3 | only `descriptor_is_fresh` |
| imperative | PASS (audit) | extensions pending |
| remodel | PASS (audit) | |
| energy | PASS (audit) | |
| trinity | PASS (audit) | |
| dag | FAIL 2/3 | only `descriptor_is_fresh` |
| draw | FAIL 2/3 | only `descriptor_is_fresh`; host-unit double landed |
| raster | FAIL 2/3 | only `descriptor_is_fresh` |
| stdio | FAIL 13/14 | only `descriptor_is_fresh` (+1 skipped) |
| note | FAIL 3/4 | only `descriptor_is_fresh` |
| puzzle | FAIL 6/7 | only `descriptor_is_fresh` (fix-only; no 5d refactor) |
| block | FAIL 7/8 | only `descriptor_is_fresh` after dsl_text helpers restored |
| space | FAIL 21/26 of 88 | connect fixed; delete_selection + set_active_example + set_app_registrations + descriptor still fail |
| sourcing | FAIL 2/3 | only `descriptor_is_fresh` |

## Extensions

| Extension | parent | test | loads with parent |
|-----------|--------|------|-------------------|
| (pending describe + extension runs) | | | |

## Fixes landed

| Item | status |
|------|--------|
| draw FSM host-unit test double | DONE — static `host_double` MachineDefinition |
| batch-B budget floors (reuse P1) | DONE — fixture + library-index + nextest |
| AppBuilder fan-out of top-level actions onto windows | DONE — fixes norm setSnapshot window roster |
| ArtifactEditor/Viewer presence local retirement default | DONE — `Some(bounded_presence_root_retirement_factory)` |
| block txt `from_dsl_text`/`dsl_text` helpers | DONE — restored for zip/unit after codec refactor |
| SpaceApp `build_document_store_owners` | DONE — unblocks batched fold; some fixture Drop/stall remain |
| connect_media_ports test isolation | DONE — empty workflow instead of demo |

## Honest gaps

- Most FAIL rows are solely stale `descriptor_is_fresh` pending wasm-mutex `describe` (queued behind s14/tc5).
- space: delete_selection spawnApp publication-stalled; set_active_example ArtifactEnvelope Drop; set_app_registrations wasm registry; descriptor.
- Extensions not yet measured.
- describe batch in flight via fleet-mutex wasm p3.

## Files changed

- draw host-unit FSM test
- library test-level-budgets fixture + PACKAGE_TEST_BUDGET_MS + nextest.toml
- framework plugin: action fan-out; Editor/Viewer presence retirement defaults
- block 2d/3d/5d txt utf-8 import/export helpers
- space engine SpaceApp document store owners; connect-media-ports unit test
