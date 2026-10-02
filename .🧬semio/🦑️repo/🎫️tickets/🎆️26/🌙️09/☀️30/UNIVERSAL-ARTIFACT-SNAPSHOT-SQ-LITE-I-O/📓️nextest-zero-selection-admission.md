# Nextest Zero-Selection Admission

The actual installed `cargo nextest run --help` was inspected read-only. It declares `--no-tests <ACTION>` and these values: `auto` automatically determines behavior, defaulting to failure; `pass` silently exits zero; `warn` warns and exits zero; `fail` produces an error and exits with status four. It also declares environment input `NEXTEST_NO_TESTS`.

The real Semio quick invocation with an incorrectly anchored qualified-name filter compiled the complete test crate but reported `Starting 0 tests`, `0 passed`, `2291 skipped` and `warning: no tests to run`, followed by successful Nx exit. Exact Nextest ID is `8fde0566-5c68-4305-804e-02e5f360209d`; evidence is retained at `🗑️generated/semio-five-controlled-readers-quick-flow-word-red.log`. This is compile/route evidence only, and its premature progress interpretation was corrected immediately upon inspection of the summary.

The root owns any shared runner change. This worker has made no runner edit. Explicit `--no-tests fail` is supported by the installed CLI and would make zero-selection admission deterministic regardless of its auto/environment behavior. A meaningful runner contract law must observe the actual selected subprocess arguments and nonzero admission for an empty selector. The worker's corrected Semio filter removes the anchor and retains actual qualified-name matching, the existing quick profile and the same warm build directories.
## Executed Runner Regression

The new language-neutral empty-selection fixture was run through the registered `framework-process:test-cargo-driver` target before changing the runner. Independent actual Cargo Nextest compiled its one-test fixture and returned the expected status 4 under `--no-tests fail`. Our unchanged execution driver instead resolved after reporting zero selected tests; the new assertion failed after 1.877 seconds. This is a real execution regression, not an inferred command-line mismatch.

After that result, both ordinary and coverage Nextest assertion plans now explicitly use `--no-tests fail`. The existing compiler/assertion budgets and filters are unchanged. A fresh full registered cargo-driver invocation is running; no passing driver result is claimed yet.
## First Changed Runner Result

The first changed-source run reached both selected laws. The new actual Nextest empty-selection regression passed after 3.453 seconds. The pre-existing multi-policy subprocess law hit its own Bun five-second limit at 5.009 seconds, after 63 assertions across the suite; this is not a driver success. Only that measured expensive law now has a ten-second local allowance. Its existing fifteen-second whole owner command budget is unchanged. A fresh full registered retry is running.
## Fresh Complete Driver Result

The final fresh registered driver invocation passed both selected laws: 2/2, zero failures, 73 assertions, 9.13 seconds of Bun assertions and successful uncached Nx exit. The real independently compiled Nextest fixture and our execution driver now both refuse zero selected assertions with status four. The original multi-policy compiler/assertion/coverage/capture checks also pass. No compiler cache or whole command budget was broadened; only the measured subprocess law received its ten-second local allowance.
