# Dependency Storage Reading Verification

The starting broad reading run had 81 passing tests, 14 failures, and 7 skipped workers. The first failing assertion closed the issued Original, so later refusals in that run did not identify independent failures.

## Fixture repairs

- The declared-provider fixture now creates providers according to the neutral case. The missing optional case puts an incompatible provider in a physically present unowned ancestor and asserts null results on both owned sides and no retained observation of the unowned path. The required-missing case retains its refusal. The one-sided optional, escaping source alias, and unknown-generation cases retain their refusals. An undeclared package import stays outside observation, while both declared dependency edges remain visible.
- The borrowed normalization fixture uses the stage's physical provider directories and checks the Native alias without creating an alias to itself. The extracted observer and normalization bodies receive their actual hoisted policy, stamp, and runtime ports.
- The extracted Store materializer uses the actual exported Store work control. A failed accepted case surfaces the original error. The isolated Store settlement worker supplies the gate, two pins, and build lease and verifies release order, including when the gate release throws. The child receipt still verifies original invocation identity, preserved primary values, abort state, and process status across all seven corpus modes.

- The Store lock fixture exercises the actual incremental lock helper, Store work control, immutable input parent walk, and current file and directory identity checks. The fixture still verifies the accepted and refused publication paths, including incomplete prospective output, drift, ready collision, foreign aliases, original publication errors, and retained outputs across a subset or entire Store.

The materializer's independent Python filesystem and digest oracle remains active. The settlement parent checks receipts using its SQLite oracle. The changes are confined to the reading test and language-agnostic case data.

## Final verification

The full reading suite exited zero with **94 passes, 7 skipped workers, and 0 failures**. It ran 141,594 assertions in 44.62 seconds. The skipped top-level workers are launched in isolated processes by the parent tests, which check their expected exit statuses and receipts. This run used the native repository tool, a 180,000 ms test timeout, and scratch owned by this ticket.
