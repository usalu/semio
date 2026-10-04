# Current Owner Cohort Gates Audit

Read-only source audit, 2026-10-03. No Cargo, runtime execution, source writes, or Git mutations occurred in this lane. Fresh runtime authority belongs to Root's current owning executions.

## Exact Source Census

- TIFF selector `sqlite_snapshot_tiff`: 19 tests, eight in snapshot SQLite tests and eleven in its mounted `🚦️cohort` child module. The similarly named production `sqlite_snapshot_codec` function is not a test and must not inflate the count.
- CommonMark selector must union `sqlite_snapshot_md` and `sqlite_snapshot_commonmark`: 25 tests, 24 in snapshot SQLite tests plus the allocator-refusal retirement test in owned pack. Selecting only `sqlite_snapshot_md` omits CommonMark-named tests. Production `sqlite_snapshot_codec` is not a test.
- JSON selector `sqlite_snapshot_json`: 19 tests in the mounted base snapshot SQLite tests. The artifact physical directory is `🧾️json`, not `🔣️json` (the latter also appears in serializer paths).

## Concrete Remaining Native Budget Gate

JSON's mounted snapshot SQLite owner still creates its direct input `NativeDecodeControl` with `limits.max_value_bytes` at line 53. This is the same semantic/native ownership ceiling conflation repaired in the shared Store bridge. The direct owner decodes the logical record and then reconstructs it with that native control. Shared bridge repair alone does not change this code. This is a source-level concern requiring the fresh 19-test receipt; no new runtime failure is claimed here.

The nineteenth JSON law computes the exact semantic SQL bytes with an independent Bun SQLite oracle and exercises public SQL import and native output through `io_run_with_snapshot_control`. It rejects output with `max_allocation_bytes: 1`, but does not call the direct `decode_sqlite_snapshot_native` hook with independent allocation and semantic ceilings. Existing direct-hook law at line 171 instead lowers `max_value_bytes`. Thus a green current JSON19 receipt would not by itself disprove the remaining direct-input source concern.

## Typed Compile Prerequisites and Fidelity

CommonMark SQL private helpers (`ordinal`, `checkpoint`, `project`, entities, subtype selection, relationships, boolean/start/text option, tail reconstruction) now return `ValueError`; String conversion happens at the current public trait boundaries. The previous helper typed-error prerequisite is therefore absent from these inspected helpers.

JSON's SQL measure and reconstruction closures likewise use `ValueError` with explicit public boundary conversion. Its tests independently compare authored record identity through blake3 and definition binding, edit relational entities with Bun SQLite, and preserve number lexemes, schema strings, duplicate members, and intermediate typed states. These are stronger than carrier-only round trips.

TIFF tests compare all twelve owned scalar variants, duplicate tags, IEEE bit states, independent edits, exact baseline diagnostics and full public owned IO. Its construction cohort separates semantic payload from actual typed/container backing allocation. The mounted count includes the secondary-strip row ceiling and cumulative native construction laws.

CommonMark tests include independent surrogate renumbering, a different permitted tree guard, all literal variants and optional states, deep trees, empty-list frontier cancellation, owned decode reconstruction cancellation, final SQL cancellation, and allocation-refused linear retirement. Prior report `📓️md-native-retirement-runtime-correction.md` records 24/25 passing before the shared bridge correction: the allocation law was the remaining actual failure. That receipt is historical; current whole-cohort authority awaits Root's fresh execution.

No semantic full-owned fidelity defect was established in the inspected tests or named paths. This is source evidence, not a claim that all three current cohorts compile or pass.
