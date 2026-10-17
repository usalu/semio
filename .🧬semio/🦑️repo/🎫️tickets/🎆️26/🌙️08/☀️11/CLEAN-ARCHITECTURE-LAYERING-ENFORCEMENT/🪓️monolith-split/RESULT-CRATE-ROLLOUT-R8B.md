# Result: CRATE-ROLLOUT-R8B (prefix `r8b`)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools: `F/r8b-tools/` (`step.sh <tag> <cmd...>`, `r8b-census.ts`, `status.sh`). Logs: `F/🗑️generated/r8b-*`. Sandboxes: `/Users/ueli/.cache/r8b/`. Started 2026-10-17 09:20.

## 0. Verdict (kept current)

| Package (`✏️s/🔌️plugins/…/🗿️artifacts/…`) | Verdict | Notes |
| --- | --- | --- |
| forms (`📋️forms/📋️forms`) | DONE (static, 11:55): 15 crates + facade, all inside the gate; `--verify --oracle` 0/0/0/0, `--check-applied` exit 0 with 0 extras | CENSUS4 PKG-7 DONE; run flags `--mutation-families=on --no-test-aggregate` |
| animate presentation (`🎞️animate/🎬️presentation`) | DONE (static, 12:50): 13 crates + facade inside the gate; verify+oracle 0/0/0/0, `--check-applied` exit 0 (0 extras) | CENSUS4 PKG-18 DONE; flags `--mutation-families=on --no-test-aggregate` |
| note (`🗒️note/🗒️note`) | DONE (static, 14:50): 14 crates + facade, all inside the gate; verify+oracle 0/0, `--check-applied` exit 0 (base 2042/4322 vs combined 2072/4318, onlyCombined 0/0) | flags `--mutation-families=on --no-test-aggregate --inline-root-modules`; backup `🗑️generated/r8b-backup-note` |
| shooting (`🎥️shooting/🎥️shooting`) | DONE (static, 13:30): 12 crates + facade inside the gate; `--check-applied` exit 0 (base 1755/3940 vs combined 1782/3976, onlyCombined 0/0) | CENSUS4 PKG-32 DONE; same flags; backup `🗑️generated/r8b-backup-shoot` |

## 1. Baseline census (cgc7 reader, `F/🗑️generated/r8b-census-base0.tsv`, 09:35)

| crate | lib files/bytes/lines | test closure |
| --- | --- | --- |
| semio-s-artifact-forms-forms | 201 / 718,945 / 10,479 | 358 / 1,240,998 / 15,959 |
| semio-s-artifact-animate-presentation | 170 / 722,436 / 14,096 | 292 / 1,077,720 / 19,157 |
| semio-s-artifact-shooting-shooting | 228 / 669,158 / 9,772 | 470 / 1,372,742 / 16,406 |
| semio-s-artifact-note-note | 248 / 820,105 / 12,501 | 507 / 1,708,461 / 20,129 |

(`note-oracle-codec` 6/61,180/1,224 and `note-note-test-oracle` are inside the gate already.)

## 2. Per package

### 2.1 forms (DONE, static)

Commands (flags `--mutation-families=on --no-test-aggregate`): `r8b-forms-verify4` (verify+oracle: oracle onlyCandidate native 0 / test 0, layout onlyCombined 0 / 0, module trees equal, visibility 0, orphan 0, glue 0, unresolved crates 0), apply `r8b-forms-apply` (196 files, 15 crates, owner rows; backup in `🗑️generated/r8b-backup-forms`), `r8b-forms-check4` (`--check-applied`: exit 0, base 1730/3225 vs combined 1755/3243, onlyCombined 0 and 0). Checks: both launch JSONs parse; owners scan has no forms row in `missing`; taxonomy `forms: missing Cargo manifest 📦️packages/🦀️rust/Cargo.toml` is an empty pre-existing plugin-root dir (not mine); `lib-wiring-budget` fails only on foreign roots (svg 605->615, tiff 526->595, flow brep mesh 1212, stdio graph stale pin); the forms root pin was lowered 984 -> 287.

Census after (cgc7 reader, `r8b-census-forms-after.tsv`, lib files/bytes/lines | test): facade 1/15,577/280 | 7/57,247/659; questions 2/12,719/266 | 5/16,083/328; config 7/11,846/252 | 7/11,846/252; config-tests 1/2,920/72 | 2/24,060/400; commands 31/78,702/1,534 | 31/78,702/1,559; forms (editor hub) 10/107,281/1,669 | 29/234,203/3,553; terminology-tests 2/1,248/41 | 3/1,901/50; tests 16/28,445/428 | 20/34,840/533; inferences 28/53,572/952 | same; definition 15/22,767/412 | 16/23,288/424; schema 24/90,853/453 | 28/100,877/619; tests-2 19/88,417/1,575 | 29/112,393/2,018; snapshot 19/143,062/1,336 | 20/145,567/1,384; diff 29/80,237/1,491 | same; tests-3 7/37,884/566 | 24/206,341/2,871; mutations 15/51,573/845 | 16/52,096/896. Before: 201/718,945/10,479 lib, 358/1,240,998/15,959 test. No crate over any limit (largest 31 files, 143 KB, 1,669 lines; test 31 files, 234 KB, 3,553 lines).

Cross-crate cfg(test) rule: `r8b-tools/cfgtest.py` found no module gated by `cfg(test)` reached from another produced crate; the only `cfg(test)` items outside test modules (`demo_mutation_cases`, the `ChunkAddressableJson` serde impls, `split_try_value_chunks`/`content_chunks`) are used inside their own file's test children (same crate).

Source changes (all minimal, no compat layers): `forms_artifact_steps` (dead) removed; `as_playbook_spec` moved to the artifact root next to `forms_steps`; `question_edit_mutations`/`update_block_operations` moved from the schema root to the mutations aggregate (7 editor callers re-pointed), dead `Construction` alias removed; schema root regions split (`SchemaExports`); response/validation made independent of the schema root (`super::`/alias paths replaced); unused glob imports removed from the text snapshot codec; the 4 inverse files import the payload type instead of the sibling module; editor root decomposed: new `support` module (UI admission helpers, interaction ids, `FORMS_PLAY_WINDOW_TRY`, `parse_value_json`) and `contributions` mounted as its own module, `try_values_map`/`effective_try_values` moved into the try wizard; `pub(crate)`/`pub(super)` items reached by tests widened to `pub`.

### 2.2 animate presentation (DONE, static)

Source changes: pure grid geometry (`TilePlay` region) moved out of the schema root into the new module `🧬️schema/🧮️tile-grid` (8 callers re-pointed); unused glob `schema::*` removed from the text snapshot codec; three inverse files import the sibling payload by its real path; editor root decomposed: new `🧰️support` module (action factory, UI admission helpers, interaction domain, `PresentationDispatchCtx`, `interaction_select_effect`, `new_tile_id`, `tile_roster_mutations`, `valid_tile_ids`, `tile_morph_prompt_effect`, `reset_presentation_document_effect`; 24 callers re-pointed); config/presence io files import through the public `config::*`/`presence::*` instead of the private `component` module; hoisted-test avoidance: the text mutations test imports `presentation_working_scene` instead of the qualified `crate::` path; sqlite test imports `crate::PresentationSnapshot`.

Commands: `r8b-anim-verify5` (oracle onlyCandidate 0/0, layout 0/0, trees equal, visibility/orphan/glue 0), apply `r8b-anim-apply` (132 files, 13 crates; backup `🗑️generated/r8b-backup-anim`), `r8b-anim-check` (`--check-applied`: exit 0, base 1494/2331 vs combined 1502/2346, onlyCombined 0/0). Lib-wiring pin of the root lowered 856 -> 394. Census after (lib | test): facade 1/24,195/388 | 5/56,364/618; v1 3/7,029/145 | 4/12,739/227; animation 10/78,282/2,500 | 12/81,928/2,603; video 8/48,938/1,077 | 14/64,756/1,422; animate (editor hub) 7/108,508/1,660 | 21/213,195/3,281; viewer 28/42,278/597 | 29/44,800/639; examples 17/90,881/2,536 | 27/115,909/3,267; inferences 22/100,871/1,592 | 28/110,417/1,841; tests 10/38,218/968 | 17/47,869/1,222; schema 25/92,697/732 | 28/97,634/818; mutations 14/80,876/1,376 | 29/201,482/3,078; resize-source-frame 26/41,896/874 | same; tile-grid 10/21,138/487 | 12/25,609/578; host 12/27,711/502 | 13/32,715/580. Before: 170/722,436/14,096 lib, 292/1,077,720/19,157 test. Nothing over any limit. cfg(test) rule: cfgtest scan 0; only a `#[cfg(test)] use` in the host crate. Kept-root-module check (blind spot): facade root file modules are `tests`, `hoisted_tests_0`, `hoisted_snapshot_1`; all first-segment `crate::` names exist in the facade, and the unfiltered rustc diagnostics of the combined layout (`r8b-tools/raworacle.py`) contain no unresolved `crate::` path in those files.

### 2.3 shooting (DONE, static)

Source changes: root `DocumentHelpers` region and the `MediaExport` region moved into the text snapshot codec (breaks the R-snapshot cycle); `Identified` impls moved into their types' region (orphan rule); new `✏️editor/🧰️ui-helpers-and-io` module (mounted `pub mod ui_helpers_and_io;`); hub tests with relative `use asset::{..}`/`use retained_commands::*` made absolute / `pub use`; root file modules inlined with `--inline-root-modules`. A first apply left schema defects and was reverted from the backup (`r8b-tools/revert.py`), the sources were fixed and re-applied.

Census after (cgc7 reader, `r8b-census-shoot-after.tsv`, lib files/bytes/lines | test): facade 1/41,023/603 | 3/73,535/787; shooting 8/97,099/1,362 | 25/214,687/3,291; config 28/50,106/920 | 28/50,106/921; inferences 28/99,418/1,348 | 29/102,987/1,407; schema 25/141,025/845 | 28/150,449/980; tests 22/58,172/1,174 | 29/79,453/1,583; mut-any 28/39,561/808 | 28/39,561/808; mut-any-2 28/37,509/755 | 28/37,509/755; mut-any-3 2/8,329/117 | 22/211,309/2,684; create-asset 25/28,101/535 | 27/31,312/582; mutations 10/53,314/730 | 29/178,251/2,423; tests-dims-scene-sun-to-half 14/89,959/1,627 | 20/134,146/2,210; set-active-asset 27/46,033/909 | 29/47,633/944. All 13 inside 32/262,144/4,000 (worst: 29 files, 214,687 bytes, 3,291 lines). Baseline was 228 / 669,158 / 9,772 lib and 470 / 1,372,742 / 16,406 test.

Lib-wiring pin of the root 808 -> 607 (`r8b-checks-shoot.log`).

### 2.4 note (DONE, static)

Source changes (all minimal, no compat layers): the editor hub root was a 42-group cycle (commands <-> `NoteDispatchCtx` <-> window <-> ink tool state <-> `NoteCommand` <-> retained). New modules under `✏️editor/`: `🧰️support` (`NOTE_PLAY_CONTROLLER_ID`, `note_action`, `note_internal_action`, `note_utility`, `note_active_utility`, `note_blocks_topology`, interaction constants, `reset_document_effect`, `ui_label`), `🎯️dispatch-ctx` (`NoteDispatchCtx`), `🎼️command-enum` (`NoteCommand` and the `args_bridge`); the root keeps `NotePlayApp`, `create_note_app` and the manifest. About 55 callers re-pointed. `NoteInkToolEntry`/`NoteInkToolState` moved from `ink-apply-events` into `🪟️window/🧬️schema` (window no longer needs a command). `window` takes `NOTE_PLAY_WINDOW_COMPOSITE` from the composite window directly. `note_label_number`/`note_setting_label` moved from the mutations aggregate into the schema root (setting leaves used them through the aggregate). The 12 `↩️inverse` companions import their payload types by the real leaf path instead of the aggregate's re-export. Presence codecs import `presence::*` instead of the private `presence::component::mutations`.

Commands: `r8b-note-plan0..3`, `r8b-note-verify3` (oracle onlyCandidate 0/0, one environment-only `--test` layout line, fixed by generator change 9), apply `r8b-note-apply` (206 files, 14 crates, backup `🗑️generated/r8b-backup-note`), `r8b-note-check1` (`--check-applied` exit 0: native 2042 -> 2072, `--test` 4322 -> 4318, onlyCombined 0/0). Blind spot: no `crate::host`/`crate::quick`/`crate::tests` path points at a moved item; `ctc-scan` 0 hits over all 17 note crates.

Census after (lib files/bytes/lines | test): v1 hub 20/174,370/2,598 | 26/201,587/3,070; schema 28/189,411/1,025 | same; diff 24/82,966/1,634 | 29/90,661/1,791; mutations 20/144,571/1,716 | 29/182,387/2,413; kinds-catalog 9/35,123/599 | 28/225,629/1,662; mut-family-4 7/25,243/430 | 28/221,914/2,987; tests-hides-the-grid 22/63,771/1,140 | 29/131,037/2,044; insert-table-row 26/33,477/656 | 27/34,718/685; mut-family 26/39,210/876 | 28/50,321/1,047; mut-family-2 28/44,629/976 | same; mut-family-3 28/41,550/939 | same; delete-blocks 28/62,200/1,334 | 29/62,944/1,352; nudge-selection 10/12,877/240 | 11/19,108/328; unit-tests 2/10,025/158 | 14/110,053/1,666; facade 1/43,304/658 | 7/83,782/1,286; oracle-codec 6/61,180/1,224; test-oracle 2/5,113/85 | 3/8,776/143. All 17 inside the gate (worst: 29 files, 225,629 bytes, 3,070 lines). Baseline was 248 / 820,105 / 12,501 lib and 507 / 1,708,461 / 20,129 test. Root lib-wiring pin 1530 -> 667.

## 3. Generator changes (all with tests in `L/🧪️tests/📦️crate-tree/🟦️.ts`, 97 pass with the mutation-leaf-generics suite)

1. `referencedItemNames` (module-tree): root-item closure and hosting no longer treat field accesses, `name:` bindings and non-leading path segments as references to root items (this had pulled the whole `Declaration` region, hence editor/viewer, into every crate that used a root helper).
2. Mutation family conversion finds companions (`diff`/`inverse`) that are siblings of the leaf in an inline wrapper (`holderOf`) and skips private aggregates; `absolutizeBeyond(..., plain=false)` keeps the private `component` segment for family leaves.
3. `queueEdit` de-duplicates identical edits (two hoisted tests widening the same host item produced `pub pub fn`).
4. CLI flags `--no-test-aggregate` (leaf tests stay per-leaf and detach into their own slices) and `--inline-root-modules` (`inlineRootFileModules`: root file modules are inlined so lower crates can reach them).
5. Sandbox: the unlinked-crate filter also drops `E0432 unresolved import semio_*`.
6. `cfgImpliesTest`: a module mount gated by `cfg(all(test, ...))` (the note `quick` mount) is test-only; before, moved root items were hosted inside a test module and re-exported from a native build.
7. Planner edge scan follows a path through a root `use` alias to the module it names (`crate::schema::diff::X` made only an edge to the schema crate, so leaf crates silently lost their edge to the diff/mutations crates).
8. `normalizeCombined` collapses `<sandbox>/../..` runs, so an unreadable manifest-relative `include_str!` compares by what it names and not by crate depth.
9. Debug traces (`[DEBUG] r8b`) removed.

## 4. cfg(test) cross-crate rule

- `F/ctc-tools/ctc-scan.ts` over all crates of the four packages: forms 0 hits, animate 0, shooting 2 `ambiguous` (name-only: `io::binary::mutations` is a real module, not the test module `mutations`), note 0.
- forms-forms-forms `unit_tests::context::dispatch` hand-off: the only consumer (`forms-forms-config-tests`, `windows/▶️try/🎚️config/🧪️tests/🔬️window`) declares its own local `async fn dispatch`; the name collision made the scanner report it. Renamed to `dispatch_try_window`, rescan 0 hits. No `testkit` feature was needed, so none was added (the canonical idiom, `cfg(any(test, feature = "testkit"))` + consumer `[dev-dependencies]`, applies if a real consumer appears).

## 5. Open items

- Foreign lib-wiring ratchet failures at the last run: sequence, writer, bim model, puzzle 2d/5d/brep mesh and the stdio object/kit roots (not R8B).
- Static gates only; no browser/runtime boot recipe was run for note/shooting/forms/animate.
