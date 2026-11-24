# en1990 editor and scene, en1993 field-meta choices (2026-11-24)

Norm root: `✏️s/🔌️plugins/📕️norm/`. en1990 root: `🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/` (`E` below). Runner scripts in this folder: `run-en1990-editor-tests.sh` (bless, then lib and integration), `run-en1990-editor-full.sh` (editor, scene, inferences), `run-en1990-scene.sh`, `run-norm-js-native.sh`.

## Result

| target | before | after |
| --- | --- | --- |
| editor lib | 6 failures | `test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out` (the filtered one is `latency_bench`) |
| editor integration | `42 passed; 9 failed` | `test result: FAILED. 48 passed; 3 failed` (the 3 are framework history tests, below) |
| scene lib | `73 passed; 1 failed` (2.5 s gate) | `test result: ok. 74 passed; 0 failed` |
| inferences lib | 6 passed | `test result: ok. 6 passed; 0 failed` |
| native JS (en1993 field-meta) | `12049 pass / 1 fail` | `12050 pass / 0 fail` (run right after the en1993 fix) |

Final JS rerun at the end of this work: `136 pass / 2 fail`, aborted by an unhandled Ajv error `strict mode: unknown keyword: "x-semio-active"` that comes from `🗿️artifacts/🪶️en1999/.../🧬️schema/📸️snapshot/🔣️.json` (another agent is editing the en1999 schema; staged `M` in git). Not touched here; `field-meta choices` passes in that run too.

Still failing in en1990 integration, all three fail identically in every family log of this ticket (never green anywhere) and are framework-owned:
- `unit_tests::undo_redo_round_trips_through_the_wrapper`: `WorkLimit ... original fixture settlement stopped after 4098 turns`.
- `unit_tests::context::history_edits_end_to_end`: `original report requires its installed semantic and retirement issuers (unsupportedOwner)`.
- `unit_tests::context::conflict_history_edits_end_to_end`: `the archive load did not settle ... DecodeForward: Replay: truncated at offset 33`.

## Causes and fixes (en1990)

1. NDP read-log leak (the three incremental laws: retained session, `every_unit_declares_all_it_reads`, `render_right_after_an_edit`). Same cause as en1998 and en1999: a check cites every registry read since the last drain, so reads of a previous row leaked into the first check of the next. Fix: `isolate()` (drain) at the start of `evaluate_gated` and before every member, reliability-analysis, test-series and case row (`E/🧬️schema/💡️inferences/📋️compliance-evaluation/🦀️.rs`). Bonus: the `members.N.*` "undeclared reads" were this leak (a perturbed member changed the global unit through the log).
2. Remedy law `DSL3` / `IL3`: the remedies of `en1990.annex-b.dsl` and `.il` offered the legacy key (`as_str()`, `DSL3`) while the wire enum is `dsl3`. Fix: the option is the wire token (`E/🧬️schema/💡️inferences/🌍️working-life-and-bridge-annexes/🦀️.rs`).
3. DSL asset drift (`print(decode(asset))` not byte-identical, stale `.dsl.semio` with equal sizes): the `FactorKind` change of the previous note turned the column `factor:TEXT` into `factor:ENUM`; the 9 committed DSL assets still carried `TEXT` (header only, no rows). Rewritten by hand (`E/🖼️assets/*/*/🗣️.dsl.semio`).
4. Stale packs (`expected Enum, found Text("building")`): the two packs (`default-office`, `high-consequence-office`) predate the enum schema. The shared bless helper only looked for the pack beside the DSL, but every family keeps the pack in the example directory above the nested DSL directory, so packs were never blessed. Fix in `⚖️compliance/🧪️tests/🧬️regen/🦀️.rs`: the pack is the one beside the DSL or the one in the example directory above. Then `SEMIO_UPDATE_GOLDEN=1` regenerated both packs through the codecs (701 and 752 bytes; the stale-pack message named the expected 701).
5. Palette kinds:
   - `combination` is a derived selector without a collection in the document (`the document has no collection 'combinations'`), `effect` has no row id of its own (`rows carry an id`). Both leave the insert palette (`E/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`, with `case`).
   - Empty collections (`accidentals.0 expected an object, found Null`, `insertItem 'bridgeSls' needs a value`): the insert tool of an empty collection produced `value: None`, which the edit rules refuse and the test oracle stored as a Null row. Fix in the engine: `resolve_edit` places a typed starter row for an empty collection (`accidentals`, `seismics`, `bridgeSls`, `fatigueSpectra`, `reliabilityAnalyses`, `testSeries`, `factorOverrides`, `trafficGroups`, `placements`; `E/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`). The shared live-acceptance helper now places the first row through the editor's own `live_resolve` instead of the Null-storing `apply_to_tree` (`📇️registry/🧬️contract/🖥️app-surface/🧪️tests/🔬️live-acceptance/🦀️.rs`: `with_first_row`, `step`).
6. Grips (`every_grip_of_every_entity_kind_...`): the maps were relative to a local origin (soffit, bar foot, action base), the framework reads and writes the absolute rest parameter. Fix in `E/🎬️scene/🕹️handles/🦀️.rs`: `from_origin` shifts map and limits by `origin · axis` (span, pre-camber, deflections, resistances, action magnitude, overrides, bridge deflection, spread whiskers, cycles); every grip rests inside its limits (local `handle` = kit handle + `with_rest_in_limits`); the deflection maps subtract the cumulative tip of the previous components (the old map wrote the cumulative sum into `w2`/`w3`); the gauges are `rebased` (`🎚️gauges/🦀️.rs`). Scene tests that fed a distance from the old origin now feed `rest + delta` (pre-camber, magnitude within half a snap step, `w3`, gauge laws incl. rest in limits).
7. `a_picked_entity_selects_its_governing_check_in_every_window` picked the first check with an entity id, which is the project id of an Annex B check, not an entity. The test now picks a check whose subject is an entity of the document (`entity_kind_of`).
8. `render_right_after_an_edit_...`: the retained session files document-level checks under their owners (entity id cleared); the raw `evaluate` keeps the project id. The expected value is now the full fold filed under its owners (`incremental_evaluation::owned`), the form the framework laws use. An attempt to file inside `evaluate` changed the committed scene golden (`golden_scene_matches_committed_bytes`) and was reverted.
9. Touch-menu test: the first scene entity is the project plate (`project-site`), not a row. The shared test now takes the first entity that is a document row.
10. Scene 2.5 s gate (`projecting_a_thousand_members_stays_interactive`, 2.5 s against 2 s in debug): the framework rule is that wall-clock thresholds assert in release builds only (`latency-bench` doc: "The thresholds assert in release builds only"). The number 2000 is unchanged; the assertion is `cfg!(debug_assertions) || elapsed < 2000 ms`, the figure is still printed. The shared `release_only` arm of `norm_latency_bench!` (ignore in debug, the same as en1997 and en1999) is now used by en1990 too (`E/✏️editor/🦀️.rs`), which is what kept the 15 minute debug `latency_bench` from running.

## Native JS: en1993 `field-meta choices`

Cause: the two tables `MOMENT_DIAGRAM` and `FIRE_RATING` in `🔩️en1993/.../✏️editor/🏷️field-meta/🦀️.rs` carried hand-written labels (`Linear (straight line)`, `R30 (resistance 30 min)`) that differ from the `x-semio-ui.options` of the schema enums. Fix: ran the project generator `python3 -I 🧪️tests/🏷️field-meta-choices/🐍️generate.py --artifact en1993` (it regenerated exactly those two tables; tokens and order already matched the enums), then `🐍️check.py`: `choices=286 mismatches=0`. Native run: `12050 pass / 0 fail` (`🗑️generated/norm-js-native.log`).

## Changed files

- en1993: `🔩️en1993/.../✏️editor/🏷️field-meta/🦀️.rs`.
- en1990 inferences: `🧬️schema/💡️inferences/📋️compliance-evaluation/🦀️.rs`, `🧬️schema/💡️inferences/🌍️working-life-and-bridge-annexes/🦀️.rs`.
- en1990 edit rules: `🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`.
- en1990 editor: `✏️editor/🦀️.rs`, `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs`.
- en1990 scene: `🎬️scene/🕹️handles/🦀️.rs`, `🎬️scene/🕹️handles/🎚️gauges/🦀️.rs`, `🎬️scene/🧪️tests/🔬️unit/🦀️.rs`.
- en1990 assets: the nine `🖼️assets/*/*/🗣️.dsl.semio`, `🖼️assets/🏢️default-office/🎒️.pack.semio`, `🖼️assets/🏢️high-consequence-office/🎒️.pack.semio`.
- shared: `⚖️compliance/🧪️tests/🧬️regen/🦀️.rs`, `📇️registry/🧬️contract/🖥️app-surface/🧪️tests/🔬️live-acceptance/🦀️.rs`.
- ticket scripts: `run-en1990-editor-tests.sh`, `run-en1990-editor-full.sh`, `run-en1990-scene.sh`.

No `[DEBUG]` of mine is left (the temporary diff dumps in the unit test and the live-acceptance test were removed). The `[DEBUG] ladder ...` / `follow unit` lines in the logs and the `echo "[DEBUG] ..."` in the pre-existing runner scripts are not mine.

## Not done, not verified

- The three framework history tests above.
- `latency_bench` was never run (needs a release build); en1990 now ignores it in debug.
- The `norm-eurocode` guest was not rebuilt after these Rust changes (see the previous note).
- Other families with empty collections in their default document (en1998 note) can use the same starter-row pattern; only en1990 was changed.
