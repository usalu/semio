# Energy reference registry tests: report

## Result
No edit by this run. When the run started, the registry tests already passed (10 of 10, repeated twice).
The failures from the task description (about 290 designations, `DIN V 4108-6:2003-06, 6.4`) no longer reproduce.

Another writer changed the module and its concepts before this run:
- references `🐍️.py`, 09:01
- tests `🐍️.py` and `🔣️.cases.json`, 08:57 and 09:01
- concepts `📝️.md`, 09:43

## Runs (`SEMIO_TEST_MANIM=1`, uv, python 3.12)
- References registry tests: OK, 10 tests.
- Narration guard test: OK, 3 tests.
- Visuals integration test (`...visuals.tests.integration`): without `SEMIO_TEACHING_PROFILE` 13 errors, because no render profile is active (environment, not a regression). With `SEMIO_TEACHING_PROFILE=preview` and `SEMIO_TEACHING_LANGUAGE=en`: 8 of 9 pass, 1 failure.
  - `test_physics_scene_constructor_bounds`: radiator height 0.6812, expected 0.7384 (tolerance 0.0222).
  - This is scene geometry, unrelated to references. The physics scene sources are being changed concurrently, so I did not touch it.

## Files changed
None.

## Queued scene edits
None required by the reference rule.

## Remains
The radiator-height integration failure above belongs to the physics scene owner (scene or case-tolerance change). Not verified whether it predates the current scene edits.
