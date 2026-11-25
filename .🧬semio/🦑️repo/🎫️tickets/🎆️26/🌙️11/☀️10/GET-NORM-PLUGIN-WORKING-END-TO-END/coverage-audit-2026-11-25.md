# Norm Coverage Audit, 2026-11-25

This is a read-only review of existing reports, not a fresh acceptance run. The shared tree changes continuously; old green runs do not establish the present state.

## Product coverage

`norm-end-to-end-2026-11-22.md` reports all 15 Standards routes booted from verified signed components, drew a model and completed alternate-example round trips. The earlier DIN V 18599 transfer stall was reported resolved. That run predates later source changes; signed components must be rebuilt and the browser workflow rerun to establish current coverage. EN 1992 was notably slower (117 seconds for the recorded route).

`repo-run-end-to-end.md` reports `repo run @semio-tech/norm-js:test --output=static` passed 11,991 tests, while broader artifact and registry work was interrupted by graph admission memory limits. `en1990-editor-en1993-fieldmeta-2026-11-24.md` reports 12,050 native JS tests passing once, then a later run aborting on an EN 1999 Ajv keyword error amid concurrent edits. The same report records three EN 1990 editor integration history failures, a release latency benchmark not run, and a guest not rebuilt after Rust changes. These are historical observations and need reproduction.

`mutation-vectors-2026-11-25.md` records wire twins 11,956 passing, but marks most Rust family mutation-vector runs and a schema crate sweep outstanding. It describes an EN 1992 schema versus Rust diff key-order disagreement and JSON-inexpressible non-finite rejection vectors, plus EN 1994 and DIN 18599 changes awaiting compilation. Its status and fixes should be rechecked before use, not assumed current.

`build-hazards-2026-11-25.md` identifies missing debug-ledger modules, missing test paths and debug instrumentation. A current text search found no `debug-ledger.rs` references in the cited value and store open trees, so that specific finding is stale; other entries were not independently rechecked. Template and manifest-path findings were explicitly reachability-unverified in that report.

## Acceptance sequence

1. Use repository-owned test targets and current graph, with a small selection under contention. Start with `repo run @semio-tech/norm-js:test --output=static`; then run the registered per-family check/test targets and mutation-vector law across all families, including EN 1992, EN 1994, DIN 18599 and EN 1997. Confirm actual project/target names using repo's project catalogue rather than guessing. Run the wire-twins target and independent third-party oracle alongside Rust tests.
2. Reproduce the three EN 1990 editor integration history cases and run the release latency benchmark. Validate schema order against Rust serialization and ensure rejection classifications are reachable in the actual wire domain.
3. Rebuild all five signed norm components after source changes, verify their published signatures and run the strict Play workflow across all 15 routes. The prior driver is `probe-norm-base-companions.py` with `PLAY_TEST_PICKER_ROUNDTRIP`, `PLAY_TEST_OBSERVE_ARCHIVES`, `PLAY_TEST_REQUIRE_DRAW`, and `VERIFY_SIGNED_MODULES`; verify current invocation and service configuration first. Require shell readiness, initial archive acknowledgment, model draw, alternate archive acknowledgment, return to original and redraw, and no faults or failed requests.
4. Run registered catalog, registry and composition verification once graph admission permits; record resource cancellations as incomplete rather than passes. Preserve the ticket reports, remove generated output at completion.
