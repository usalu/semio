# Current Prepared Pair Boundary Audit

Read-only source review resumed after the app restart. This is an interim assessment; Native preparation owner confirms the repairs below are in progress. No tests, builds, producer jobs, caches, locks, or source files were changed by this lane.

The current custody implementation still selects scanned Unicode import text through Latin1-to-UTF8 decoding plus a whole-source substring check. That check does not establish the spelling of the particular import literal. A genuine mojibake literal with decoded emoji text in a comment or unrelated string is a bounded ambiguity; escaped Unicode imports also need independently decoded literal evidence. The neutral oracle must cover these alongside ordinary emoji and Latin1 imports.

Current custody rechecks hashes of selected sources and resolver configuration but records no resolution edges. A newly appearing `program.ts` can outrank unchanged `program.js`; currentness must re-resolve the actual source/specifier pair or bind candidate absence. The existing neutral resolution mutation is present in the test owner, but no settled meaningful RED/GREEN receipt was supplied yet.

Current owner preparation appends dependencies to an insertion-ordered pending set and immediately runs the parent recipe. Manifest and member inventories remain cached. It therefore does not yet prove dependency-first preparation or fresh discovery of dependencies/members introduced by recipes. Native owner confirms planned DFS and post-recipe refresh; no completed credit is assigned.

The consumed-byte repair is present: custody accepts the actual returned bytes and Hub composition passes the read buffer. The existing read-race mutation changes the file between read and observation and expects refusal before Cargo calls. This source observation is not a runtime pass claim.

Custody schema currently uses canonical `/component.json`, title `CargoPreparationCustody`, and two variable observation/input definitions outside fixture collections. The test owner checks actual inventory scope `repo.library.workspaces.cargo.preparation.custody`, all exported definitions, exact schema hash and own-path diagnostics. Its settled terminal and fresh six-scope root diagnostic/catalog receipts remain pending.

The preceding Unicode dependency-order RED failed during import resolution before its intended dependency-order assertion. It remains unsuitable as meaningful dependency-order RED. Native dev/release, four guests, normal publication and mounted HTTP remain unproven.
