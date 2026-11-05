# Canonical Staffage Fitting Audit

The bridge maps canonical figure metres uniformly into Manim scene units. Transport figures use figureBoundsM for placement and sizing, excluding air layers. Explicit crop windows determine their own fit extent. Mirror transforms preserve width and height.

## Confirmed Defect

fitted_staffage probes the static first frame even when it returns a moving clip. The returned motion carries whole-clip bounds, so fitting a narrow first pose can exceed the requested box immediately and later frames can exceed it further. Existing fixture first-frame extent is 1 by 4 metres; whole-clip bounds are 5 by 5 metres. A width limit of 2 currently selects unit 2, yielding a motion width of 10 instead of 2.

## Regression

Language-agnostic animatedCases vectors specify width-only, height-only and combined limits. The bridge test checks the unit, every frame extent and requested limits against a separately constructed Manim Rectangle scaled by the fixture expected unit. Existing static fit acceptance remains unchanged.

## Validation Status

Regression added before implementation. Actual Python bridge execution is pending runtime provisioning; registered python currently cannot resolve python3. No fixture/source copies were placed in the ticket.

The initial targeted test invocation through repo tools exec --light python failed before imports: tool-missing (no registered python3). The implementation now selects a whole-clip motion probe whenever rate is provided. It keeps static first-frame fitting, windows, mirror, canonical geometry, provenance recording and final physical unit application unchanged. Runtime execution remains pending environment recovery.

## Executed Red/Green Verification

After provisioning Python 3.12 with Manim 0.22.0, NumPy 2.5.4 and svgelements 1.9.6, the exact original single probe line was restored temporarily. Running Fit.test_motion_fits_the_whole_clip_and_matches_an_independent_rectangle through the registered repo Python tool failed as intended: actual unit 2.0 versus expected 0.4 for the width-only 2-unit limit.

The exact fixed line was restored without replacing any other content. Running the complete Fit test class with Python -X utf8 passed all 3 tests in 0.362 seconds. This includes static first-frame fitting verified by the independent svgelements parser, every-frame animated fitting against the independent Manim Rectangle oracle, and rate/loop dispatch. No pending fit validation remains.

## Full Bridge Acceptance Run

Executed the entire bridge unittest module through repo tools exec --light python -- -X utf8 -m unittest, using the ticket Python venv and repo launcher on PATH. Result: 61 tests in 83.732 seconds; 57 passed, 1 failure, 3 errors. All fitting, placement, geometry, motion, frame swap, rate, playback, colour and provenance/reference acceptance checks passed.

The three Schema errors are environment-only: jsonschema was not installed in the temporary venv. The existing Cache.test_each_input_changes_the_key test fails its catalog-file case because it explicitly clears _source_digest and _catalog_bytes caches, but omits the separate memoized asset_revision cache. Its before/after content key is therefore identical. This does not arise from whole-clip fitting or probing motion twice; no unrelated cache behavior was changed. Root agent was notified for scope/validation handling.

## Full Acceptance Follow-Up

After jsonschema was installed, rerunning the entire Schema class passed all 3 tests in 3.864 seconds. Coordinated the cache failure with the course coverage agent: the extra asset_revision memoization duplicated its already-memoized catalog bytes and obscured the intended snapshot refresh. That agent removed the redundant decorator and its checkpoint cache-clear call. The unchanged test Cache.test_each_input_changes_the_key then passed in 0.279 seconds.

Composite acceptance result: all 61 full-module tests now have passing execution evidence (57 initial passes plus 4 targeted passing reruns). The whole-clip fit probe introduces no observed geometry, provenance or playback regression. No failed case was omitted or weakened.
