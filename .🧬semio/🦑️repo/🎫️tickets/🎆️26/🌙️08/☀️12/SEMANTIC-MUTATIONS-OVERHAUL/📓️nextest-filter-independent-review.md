# Nextest Filter Independent Review

FND-NEXTEST-FILTER-09 is accepted for positional test-filter routing in the metadata-backed runner. Required Cargo option values remain build arguments; ordered name filters reach execution; post-separator libtest arguments remain intact. The Node parser oracle independently checks both option values and positional arguments.

Coordinator review found that bare `--timings` incorrectly consumed the next positional filter. The installed Nextest grammar requires an optional value to be equals-joined. The executor added a failing neutral case and corrected it; bare `--timings first_filter second_filter` now preserves both filters, while `--timings=html` remains a build argument.

The final coordinator Nx gate exited0 with2 tests and68 assertions,291 filtered,0 failures. Evidence: `🧪️nextest-filter-root-final.log`. Earlier root evidence before the final timing case is `🧪️nextest-filter-root.log`. Executor grammar/fixture/lint details are in `📓️fnd-nextest-filter-09.md` and `🧪️nextest-list-help-2026-08-27.md`.

The real nine-law PDF invocation is running separately using the corrected positional route. These parser tests do not claim that PDF laws pass. The reported repo-library lint errors remain outside this bounded packet; no full lint pass is claimed.
