# W3-PERF-2: report handles, precise global reads, shared row gate, one value tree (nothing compiled or run)

Implements design notes 2, 5, 6 and 3 of `perf.md` (as numbered there: `Arc<CheckReport>`, global reads, per-scope gates, one value tree). Not done on purpose: `Arc<SceneEntity>` (about 120 files, after the first green build) and the `Arc<Document>` session change (needs a `UnitSource::Document: Sync` trait change; not contained). Builds: `- ready for rebuild: perf-2` appended to `build-en1990 .. build-en1999/latest.md`. `freeze.txt` was empty when the edits ran (it named en1995 and en1996 earlier, those artifacts were edited only after it cleared). The repo MCP did not connect, so no ticket call was made.

## 1. `Arc<CheckReport>` through session, family, host, clock

| Where | Change |
| --- | --- |
| `compliance/incremental-evaluation/session` | `EvaluationSession.report: Arc<CheckReport>`, `shared_report()`, `report()` still returns `&CheckReport`. `SharedEvaluation::{report_for, commit, report_now}` return `Arc<CheckReport>`. `finish` only merges when a unit ran since the last merge (`merged` revision), so a settled paint no longer rebuilds and clones the whole report. |
| `live_report!` macro | `report`, `report_now`, `responsive` return `Arc<CheckReport>`. |
| `NormFamily` | `live_report -> Arc<CheckReport>`, `live_responsive -> (Arc<CheckReport>, bool)`, default `Arc::new(Self::evaluate(..))`. The ten editors' overrides changed (type only). |
| `NormHost` | `report: Arc<CheckReport>`, `shared_report()`; `report()` still `&CheckReport`. The `ToValue`/`FromValue`/serde derives were dropped: nothing converts a host (grep over the tree), and `Arc` has no value codec. |
| `LaneClock` | stores `Arc<CheckReport>`; `cached_report` returns the handle, `store_report` takes one; `LiveView::over_lanes` stores `host.shared_report()` (a drag tick between two due reports copies nothing). |
| viewer surface | `NormViewReport = Arc<CheckReport>`; trait, `view_lane_mutations` and the test follow; the ten viewer macro lines compile unchanged. |
| other call sites | `export_media` (`&*report`), the two scene/diagram macro calls in editor wiring (`&*report`), `incremental_laws!` (`*` derefs), compliance and editor tests (`assert_eq!(*..)`). All other readers (`&report` into `&CheckReport` parameters) rely on deref coercion. `revision_of<D, R: Debug>` is generic: `Arc`'s `Debug` is transparent, so the stamp is byte-identical. |

Pin tests: `a_settled_session_hands_every_reader_the_one_retained_report_without_copying`, `a_catch_up_that_re_runs_no_unit_does_not_merge_the_report_again` (compliance unit), `the_cached_preview_report_is_the_stored_handle_not_a_copy` (lane clock), `a_host_and_the_lane_clock_share_the_live_report_by_handle_and_equal_the_family_report` (live-acceptance macro, all ten editors). Latency: `a_repeated_read_of_a_settled_thousand_member_session_copies_and_merges_nothing` prints `[latency] settled-report-read-1000-members/paint` (2 ms release, 100 ms debug).

## 2. Global and part units with precise read sets

New builders `Rows::global_sharing(extra, run)` and `Rows::part_sharing(name, extra, run)` (shared fields of the row units plus the declared collections) next to the existing `global_reading`/`part_reading`, which are no longer dead. Reads were derived by reading the scoped evaluators; the existing law `every_unit_declares_all_it_reads` (perturbs every leaf outside the read set) is the check, and a new law `an_edit_re_runs_exactly_the_units_whose_declared_reads_changed` (macro `incremental_laws!`, all ten editors) compares a session's re-runs after 40 random leaf edits with the units whose read hash changed.

| Artifact | Unit | Reads |
| --- | --- | --- |
| en1990 | global | shared fields + `members/reliabilityAnalyses/testSeries/cases/placements .*.id` (was `*`) |
| en1991 | integrity | shared scalars + the id column of each of the 32 row collections + `bridgeLanes`, `roofSurfaces` (lane numbers, widths, faces, extents) |
| en1991 | parts `floors-none`, `self-weight-none`, `snow`, `wind`, `accidental-none` | shared scalars + the id column of the one collection whose emptiness they state |
| en1991 | parts `fire`, `thermal`, `construction` | shared scalars only; `traffic`: shared scalars + `bridgeLanes.*.width` |
| en1992 | global | `annex`, id columns of members/anchors/placements/cases, `concreteGrades`, `reinforcementGrades`, `prestressSteels` |
| en1994 | verdict units | same reads as before, now declared through `global_reading`/`part_reading` instead of post-editing the unit list (`declare_verdict_reads` removed) |
| en1995 | global | `annex` + id columns of every collection incl. member/connection action ids and case number/text ids |
| en1996 | global, `detailing` | `annex, walls`; `annex, walls, chases, movementJoints` |
| en1999 | integrity | `annex`, id columns, `materials sections placements outlineVertices`, member `sectionId/materialId` |
| en1999 | six `*-empty` parts | `annex` + the id column of the collection |

Pin tests (declared reads, per editor `incremental` tests): `a_row_field_edit_leaves_the_non_row_units_clean` in en1990, en1991, en1992, en1995, en1996, en1999.

Left at `*` (not verified, each hides a long reference walk): en1993 `check_integrity` (about 40 id lists plus cross references), en1997 `integrity` and part units, en1998 `integrity` (four reference walkers) and the ten `*-na` parts (they depend on `foundations`/`soilLayers` through `effective_v_s30`). Documented risk: a wrong narrow set means a stale check; the two laws above fail loudly on the next build if one of the derived sets is short.

## 3. Per-scope gates computed once per revision

`Rows` unit lists get `Gate<G>` (lazy `OnceLock` shared by all row closures of one `units()` call). EN 1990 was the only family with a heavy per-row gate: `evaluate_scoped` re-ran input validation and the whole integrity pass in every member, analysis, test-series and case unit. Now `evaluate_gated(document, scope, &Gate<RowGate>)` takes `(admitted, broken members)` from the shared gate; the global scopes still emit the gate checks themselves and never read it; `evaluate_scoped` keeps its signature (fresh gate). Pin tests: `a_shared_row_gate_answers_every_row_like_a_gate_computed_per_row` (valid, invalid input, unwired member, no members), `the_row_gate_reads_nothing_the_member_units_do_not_declare` (the audit machinery over the gate, because the cached gate makes the per-unit audit blind to gate inputs), compliance `a_gate_is_computed_once_for_all_units_of_a_list_and_every_list_has_its_own`. Latency: `a_thousand_member_rows_share_one_gate_evaluation_within_the_cold_budget` prints `[latency] en1990-gate-1000-rows-1000-members/cold` (100 ms release, 5000 ms debug; without sharing it is about 1000 gates). The other families' scoped evaluators have no comparable gate (checked en1991, en1992, en1994..en1999 by reading).

## 4. One value tree per document revision

- `Rows::except` lists the field names with `value_key_at_path(&[], index)` (answered by the derive without converting any value): no whole tree per `units()` call.
- New `incremental_evaluation::shared_value_tree(&D) -> Arc<DslValue>`: most recent four revisions per document type, served by equality, converted outside the lock. The session's `rebuild` and the scene kit's `row_texts` (every fingerprint pass) read it, so a drag tick converts the preview document once instead of twice. `UnitSource::Document` gained `PartialEq + 'static`, `SceneKit::Snapshot` gained `Clone + PartialEq + Send` (all families already satisfy them).
- The overlay base (`overlay_preview`) already converts once per drag; unchanged.
- Pin tests: `the_shared_reads_come_from_the_field_names_without_converting_the_document`, `the_session_and_every_other_reader_of_one_revision_convert_the_document_once` (counting `ToValue`), `global_and_part_units_read_the_shared_paths_plus_only_the_collections_they_declare`. Latency: `a_further_reader_of_a_thousand_member_revision_pays_a_compare_not_a_conversion` prints `[latency] shared-value-tree-1000-members/paint`.
- Honest gain: a hit costs a typed compare and a miss costs the conversion plus one document clone, so the saving is the second conversion (roughly 1 to 2 ms at 1000 rows) minus a clone; the clone is cheaper than the conversion by a factor that was not measured.

## Compile risks to look at first (nothing was compiled)

1. `NormHost` lost its value/serde derives; any `NormHost: ToValue` bound elsewhere would break (none found by grep).
2. `[T; N]` read-set consts have hand-counted lengths (en1990 5, en1991 35, en1992 8, en1994 5, en1995 18, en1999 19).
3. `UnitSource::Document: PartialEq + 'static` and `SceneKit::Snapshot: Clone + PartialEq + Send` are new bounds on every implementor.
4. En1991 agent was editing `🧮️units/🦀️.rs` concurrently (35 row collections, dust explosions); my `INTEGRITY_READS` was generated from the current `push_referential_integrity` tags, a later collection needs adding there (the audit law will say which).
5. The deref-coercion call sites listed in section 1 (`&report` into non-generic parameters) are the places a type error would show up.
6. The `incremental_laws!` macro grew one law; it needs `read_hash_of` (new, `read-tracker`) and `SessionEvent` from the public module.

## Files

Shared: `compliance/⚡️incremental-evaluation/{🧵️session,🛰️live,🧱️rows,🧪️laws,🔍️read-tracker,⚙️units,🌳️value-tree (new),🦀️}`, their `🧪️tests/{🔬️unit,⏱️latency}`, `compliance/🏛️norm-error-and-family`; `structural-scene/🎯️projection/🧲️scene-kit`; `app-surface/{🫧️live-lanes/⏱️lane-clock (+tests),🫧️live-lanes/🎛️editor-wiring,🪄️selection-mirror,🪟️manifest-and-media,👁️viewer-surface (+tests),🧪️tests/🔬️live-acceptance}`. Artifacts: all ten `✏️editor/🦀️.rs` and `✏️editor/🧪️tests/🔬️unit`, the `🧮️units` of en1990, en1991, en1992, en1994, en1995, en1996, en1999, en1990 `📋️compliance-evaluation` and its compliance-report tests, and the `🧮️incremental` tests of en1990, en1991, en1992, en1995, en1996, en1999.
