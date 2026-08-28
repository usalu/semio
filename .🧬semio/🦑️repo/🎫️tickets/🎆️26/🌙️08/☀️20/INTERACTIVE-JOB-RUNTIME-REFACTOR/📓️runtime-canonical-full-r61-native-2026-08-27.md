# Full Runtime R61 — Original Inline Census Still RED

Actual exhaustive native run selected 109 tests. It completed **81 passed, 1 failed**, with 27 selected tests not run after the failure; 0.424s, session 78101 exited 1. No tests were excluded.

The sole executed failure was the original `surface_ownership_inline_fields_do_not_allocate_a_second_owner` acceptance law. The current census still double charges inline ownership; this is a real outstanding production correction, not a fixture to remove. Previously failing cursor layout and the current canonical root/comparison laws passed within this run.

The 27 not-run tests are not credited. In particular, this run does not prove the remaining transaction constructor/output path. The full physical old/candidate/output/retired census and paired Transacted output ownership remain the next main implementation work.

Canonical `SEMIO_COVERAGE=0 @semio-tech/ui-runtime-rs:test --args='exhaustive --lib'`, unchanged target/environment. Raw `🧪️member-runtime-canonical-full-r61-native-2026-08-27.txt`.
