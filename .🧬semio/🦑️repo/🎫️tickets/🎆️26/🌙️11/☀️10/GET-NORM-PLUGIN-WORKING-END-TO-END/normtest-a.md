# normtest-a

## Done and verified (host target tests)
- Cascade (14 tests): root cause was `properties::tests::a_multi_selection_shows_the_common_value...` (panels-inspection): `render_selection_inspector(...).unwrap().is_some()` dropped a built node without retirement, so the fixture close failed ("descendants remain borrowed") and, because the fixture forgot its lease on panic, the process-wide arena stayed owned for every later test. Fixed the test (project and retire the node). Harness fix in `🧰️framework/…/🌱️values/🧪️fixture/🦀️.rs`: on panic the fixture now still tries the normal close under `catch_unwind` (locals dropped first, so a failing test frees the arena) and only leaks when the close itself is refused.
  Result: panels-inspection lib 64 pass / 0 fail (was 54/10), panels lib 43 / 0 (was 37/6).

## Implemented, NOT yet verified (static_source, 8 tests)
- `🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/driven.rs`: new `session_step(bank, grant, work, |control, effect| ..)` (exported from `🦀️.rs`) on top of `Authorities::step_effect`, which now also binds the session Source witness (`with_source_witness`), because the first run showed the next error "native decoder has no original Source dispatch witness".
- `inspector-forms/paid/static-source-tests.rs`: `Owner::step_under` (session_step) used by `advance` when `preparation.phase` is `Phase::Read`; other phases keep `session_nested` (their `run(..)` opens its own effect).
- Run 1 (before the witness fix): 2 pass / 8 fail with the witness error. Run 2 (`T/🗑️generated/normtest-a-forms2.log`) was killed by the 1 h cargo limit while the capacity gate and the component build starved it: no result. Re-run needed: `T/normtest-a-run.sh forms3 semio-s-plugin-norm-inspector-forms paid::static_source`. Held back per coordinator until "TEST COMPILES RESUME" appears in coordination-plan.md.
- Possible remaining: the seeds "projection frame and answer must return before its inputs" close error was not reached yet; a live action inside `step_effect` may also conflict with `prepare_work` of the cursor (unknown until run).
