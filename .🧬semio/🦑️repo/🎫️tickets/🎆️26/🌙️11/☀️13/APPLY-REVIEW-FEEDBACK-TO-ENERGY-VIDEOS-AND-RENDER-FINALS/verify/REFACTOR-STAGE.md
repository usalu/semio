# Refactor: stage.transition removes individually added source parts

## Change
- `🎓️teaching/🏛️architecture/🔀️morph/🎭️stage/🐍️.py`: new `Stage._consume(source)`; called in `transition` and `cut` right after the subjects resolve. Scene entries that are members of the source group's family (not the group itself) are removed and the group is added, exactly what the old `_regroup` did. No-op when the group itself is in the scene. Unlike `_regroup` it does not touch scene entries that merely contain the group (ancestors).
- `🎓️teaching/🏛️architecture/🔀️morph/🧪️tests/🎭️stage/🐍️.py`: `run(..., individually=False)` option and test `test_source_group_added_part_by_part_is_fully_gone_after_transition_and_cut` (preview and final, transition and cut). Red before (2 failures), green after.
- `📊️demand/🎬️clip/♨️system-losses/🐍️.py` and `📊️demand/🎬️clip/🏷️certificate/🐍️.py`: `_regroup` helper and its single call each (`_regroup(self, eq)`, `_regroup(self, big_sheet)`) deleted. Transition ids, sources, targets and run_times untouched.
- Ticket tooling: `tests-final/run-refactor-stage.sh`, `tests-final/refactor_stage_equiv.py`.

## Tests (final profile, en; logs in `🗑️generated/refactor-stage/`)
- morph stage tests: 9 OK (before the fix: new test failed)
- demand `🧪️tests/🔗️chain`: 8 OK
- demand `🧪️tests/🛡️guard`: 8 OK
- The two scenes have no own test modules; the other demand test modules (regulations sun, primary-energy chain/scale, recap balance) are unrelated to the change and were not run.

## Picture equality
- Not rendered: no cheap single-scene last-frame path was found; rendering was also off limits for this task.
- Evidence instead: `refactor_stage_equiv.py` builds a scene with sibling mobjects and parts added individually (flat and nested) and shows the old `_regroup` and `Stage._consume` leave the identical `scene.mobjects` order (`other, tail, group`), so z-order is identical. Call timing is identical too: in both scenes no scene add happens between the old `_regroup` call and the `stage.transition` call, and `transition` builds the animation after `_consume` without changing durations.
