# FEM actions, clipboard, gumball rotate and resolve (F5, F11, F2, F3), 3D and 2D twins

`$M25` = `🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE`. Scripts: `$M25/fc-law.sh` (3D test target), `$M25/fc-pkg.sh` (any package, private dir `C:\t\<tag>`). Logs and `C:\t\fc*` dirs are deleted.

## Status

| Item | Code | Law | Native result |
| --- | --- | --- | --- |
| F5 addCombination syntax, addSolid/addRegion refusals, notices EN/DE | done (3D + 2D twin) | done | pure laws green (below); mounted laws compile, blocked by the framework store-initializer hang |
| Every Model Actions action acts or refuses | done | `declared_verbs` laws 3D + 2D (framework `assert_declared_verbs_honour_their_declarations`) | compiles, blocked by the hang |
| F11 Copy / Cut / Paste | done 3D, 2D regions added | 4 laws 3D, 4 laws 2D | green (3D lib 4/4, 2D lib 4/4) |
| F2 rotate | verified, law widened | engine 361 degrees + editor `a_ring_turn_of_a_solid_keeps_it_and_its_analysis_alive` (45 degree angles added) | engine laws green in the earlier report; editor law blocked by the hang |
| F3 re-solve | laws added (3D sequence + 2D settle twins) | 3D: superseded, burst, example switch, add solid, edit+undo+redo+addSolid; 2D: superseded, sequence, example switch | compile, NOT run: blocked by the hang |

Green runs of this session: `semio-s-artifact-fem-2d-model --test structure_laws` 4/4 (terms syntax fixture), 3D lib `clipboard` 4/4, 2D lib `clipboard` 4/4, 3D `editor_app` pure laws (`add_combination_refusals_have_complete_localized_notices_3d`, `add_combination_refuses_terms_it_cannot_honour_with_a_fault`, `add_solid_refusals_reach_the_shell_as_localized_notices_3d`, `every_command_keeps_its_current_bytes`, `every_command_round_trips_through_text_and_binary`, `command_ids_are_unique...`, `every_mutation_action_declares_args_covering_its_payload`, `manifest_labels_resolve_german_3d`, `fem3d_io_declares...`, `every_tree_row_icon_id_is_registered_3d`). `validate_fault_notices` passes on the whole 3D table.

## BLOCKER for every mounted law: store initializer does not finish

Any test that mounts an app (`fem3d_app()`, `fem2d_app()`, every dispatch, every declared-verb probe, every settle law) does not finish within 110 s, 2D and 3D alike. A single test alone (`add_node_action_emits_op_3d`, `--test-threads=1`) spins; the log shows `[DEBUG] plan ready=true wanted=OriginalReturnEscrowAmounts { copy_bytes: 3600 .. } quantum=RetainedCloneGrant { maximum_copy_bytes: 2368 .. }` thousands of times and `[DEBUG] store initializer call=99969 ..`. The instrumentation is a peer's in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/..` `🧵️job/⏱️context/🛬️owned/🦀️.rs` (`original_receiving_outcome`, edited 01:27 and 04:21 today, the "hang" agent: `$T22/🗑️generated/hang/law-l*.log`). The wanted escrow exceeds the per-turn quantum, so `original_receiving_outcome` returns `Ok(false)` forever. Not related to FEM code (not touched by me). Run the laws below once that lands. Exception: `editor_live_cleanup` finished in 6.8 s in the hang agent's run `law-l4.log`.

Earlier in the session the same laws failed differently, and this one I fixed (framework, stale probe): `🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/📬️typed-operation/🦀️.rs` `TypedOperationResultPage::close_step` carried a leftover `if std::env::var_os("L3D_TRACE").is_some() { eprintln!("[DEBUG] page close ..") }`. On Windows `var_os` allocates the 20 byte wide key, which broke the exact heap witness of `close_fixture_result_page` (`left: (20, 20) right: (0, 0)`, `artifact-app-laws` line 70) at the first dispatch of EVERY mounted law (editor_app, editor_app_2 and the results laws; this is also the `(20, 20)` seen in `ls/law-w4.log`, `tq/law-add3.log`). Removed. Still present from peers (not touched): `[DEBUG] worker page reservation zero` in `🧵️job/📬️outcome/👷️worker/🦀️.rs`, `[DEBUG] plan ready` in `🛬️owned`, `[DEBUG] store initializer ..`.

## F5 actions

Root causes
- `addCombination`: `terms` was declared with `ActionArgDef::json_text` (`x-semio-format: json`), so a form validates or sends JSON and `dead:1,live:1` never reaches the command; the handler (earlier today) already admitted the natural spellings. The arg is now plain text with the worked example description (law `the_combination_terms_argument_is_declared_as_plain_text_with_an_example` replaces the JSON one).
- 2D `addCombination` had no `terms` arg at all (`terms: Vec::new()` from the wire): it authored an EMPTY combination. 2D payload is now `terms: String` with the same syntax.
- Refusals were `Fault::from("fem3d.patch....")` and code-with-text faults: shown as a bare code. All now have EN and DE notices.
- Silent no-ops found by reading all handlers: `setSelfWeight` with an unknown case, `setAnalysisSettings` with no setting, `removeSelection` with no ids, copy with nothing selected and paste without a fragment (framework), `setCamera` without a camera (bare code).
- The two stale editor_app imports (`commands::add_solid`, `add_combination`) were already `semio_s_artifact_fem_3d_editor_commands::...`; editor_app compiles (3D and 2D).

Changes (3D under `🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor`, 2D under `◻️2d/...`)
- Shared parser, one place: `◻️2d/.../🧬️schema/🏗️structure/🦀️.rs` `parse_combination_terms` -> `CombinationTermsError::{Unreadable, Empty}` (`dead:1.35, live:1.5`, `dead=1.35`, `1.35 dead + 1.5 live`, `1.35*dead;`, line breaks, JSON pairs/records/object). 3D `🔗️add-combination` and 2D `🔗️add-combination` resolve case ids or names (any letter case), refuse unknown, repeated, non-finite with `fem3d|fem2d.combination.terms-*`.
- 3D `📋️manifest` terms text arg; 2D `🚀️ui-helpers-and-app-assembly` terms arg (required, described).
- 2D `🗺️add-region`: unknown material and non-positive extent refused (`fem2d.region.material-unknown`, `fem2d.region.extent`), twin of `addSolid`.
- `⚖️set-self-weight` (`*.self-weight.case-unknown`), `🧮️set-analysis-settings` (`*.analysis.nothing|value|field`), `🗂️remove-selection` (`*.remove-selection.empty`, only for an empty id list; named-but-missing ids stay a no-op), 3D `🎥️set-camera` (`fem3d.camera.orbit-required|orbit-invalid`).
- Notices: `🖥️play-app/🦀️.rs` (3D) and `🎛️play-app-editor/🦀️.rs` (2D) tables; generator `fem_editor_fault_notices(prefix, kinds)` in `◻️2d/.../🎮️commands/🧭️gumball/🦀️.rs` yields the `patch*-missing|field|value` family, `focusEntity`, `setCamera` and results codes for both editors; the 2D `fem_fault_notices` table gained the result animation codes.
- Framework `🧰️framework/.../🔌️plugin/📜️declaration/📋️clipboard/🦀️.rs`: copy with nothing selected -> `clipboard.copy.empty-selection`, paste without a readable fragment -> `clipboard.paste.empty` (both with EN/DE notices in the FEM tables), instead of an empty emit.
- Left as designed: `patch*` with an unchanged value is an idempotent no-op (inspector commits every blur); `setResultAnimation`, `resultAnimationTick`, `focusEntity`, `setTransformGumballFlag` are not in the pane (`in_palette: false`).

Laws
- Language-agnostic: `◻️2d/.../🏗️structure/🧪️tests/🧫️fixtures/🔤️combination-terms/🔣️.json` (9 readable spellings, 9 refusals) read by `combination_terms_read_every_documented_spelling`, JSON spellings cross-checked with `serde_json`. GREEN.
- `⚖️declared-verbs` (3D `🧪️tests/⚖️declared-verbs`, 2D `🧪️tests/⚖️declared-verbs`, fixtures `🧫️fixtures/⚖️declared-verb-examples/🔣️.json`, in `editor_app`): every declared verb on the demo example acts or is refused by name (no `SilentMutation`, `IgnoredArgument`, `Unreachable`), and `addCombination/addSolid|addRegion/addBar/addFrame|addBeam/addSupport/addNodalLoad/addMemberUdl/addAreaLoad/setSelfWeight` write the document. Compiles; first findings will be reported by the run once the hang is gone.
- Updated: `set_self_weight_unknown_case_is_refused_by_name` (3D, 2D), 2D wire bytes of `addCombination` (`010b0203554c531c...`), `retained_route_dispositions_are_exact_and_exhaustive` (looks the action up in any window kind: `setResultDisplay` moved to the results window, the law was red before this work).

## F11 clipboard

Root causes (all three evidenced)
1. Browser `UnsupportedOwner ... natural-file import`: the served wasm predates framework `build_reserved_tool_job`; needs the coordinator's component rebuild.
2. Native `sealed source requires original granted handback`: the laws leaked the parsed snapshot (fixed earlier with `Retiring`), not clipboard code.
3. The heap witness probe above, which failed every mounted law.
Fix: 3D solids copy/cut/paste (earlier), now 2D copies, cuts and pastes regions (`◻️2d/.../📋️clipboard/🦀️.rs`: fresh `r` ids, outline and holes shifted to the placement, unknown material falls back to the first, cut removes selected regions).
Laws (green): 3D `clipboard::tests` 4/4; 2D `clipboard::tests` 4/4 (`copy_then_paste_adds_the_selected_member_under_fresh_ids_2d`, `paste_centers_on_the_placement_position_and_an_empty_selection_copies_nothing_2d`, `paste_moves_a_region_and_keeps_its_fields_2d`, `cut_then_paste_recreates_the_member_and_removing_the_paste_reverts_it_2d`).

## F2 gumball rotate

`gumball-rotate.md` holds: root cause `triangulate` failing at about 45 degrees (`could not recover boundary edge`), fixed with lattice retries (shared by 2D regions), partial meshing with a visible error marker, tolerant quaternion composition. Verification here: the editor law now also turns the slab by 44.5, 45 and 45.5 degrees (the failing band), 135, 225, 315 degrees and checks `build_model` and `fem3d_solve` on each; the engine laws cover all 361 whole degrees (3D) and `a_turned_region_always_meshes` (2D). Editor law needs the hang fix to run.

## F3 re-solve

What landed before (`f3-root-cause.md`, `turn-cost-batching.md`, `results-laws-initialize.md`): session `initialize()` in every law manifest, assembly/CSR/PCG/LDLT/subspace batching (slab 37,025 -> 232 turns). Added here: 3D `edit_undo_redo_add_solid_sequence_settles_3d` (8 edits, 6 history verbs, addSolid, undo, redo, 900 rounds bound, no job left), the `[DEBUG]` trace removed from `pump_results`; 2D twins `🧪️tests/🔁️results-settle` (`superseded_solve_settles_on_the_latest_revision_2d`, `edit_undo_redo_add_region_sequence_settles_2d`, `example_switch_during_a_running_solve_settles_on_the_new_example_2d`, bound 600 rounds, settled = `Converged`, no `Pending`, no `Previous Results`, no job left; `context::view` made public). `superseded_solve_settles_on_the_latest_revision_3d` is unchanged. None could be run (hang), so F3 stays UNVERIFIED natively; the other open lead from `resolve-wedge.md` (`StagedActionSession` Execute gating behind the queued `source.query`) is still not tested.

## Run when the hang is fixed

```
bash $M25/fc-law.sh 3d editor_app "declared_verbs,add_,remove_selection,gumball,set_camera" a
bash $M25/fc-law.sh 3d editor_app_2 "results_window" b
bash $M25/fc-pkg.sh 2d fcm app -p semio-s-artifact-fem-2d-editor --test editor_app -- --nocapture declared_verbs add_nodal_load remove_selection
bash $M25/fc-pkg.sh 2d fcm app2 -p semio-s-artifact-fem-2d-editor --test editor_app_2 -- --nocapture results_settle
```
Build 2D `editor_app` and `editor_app_2` in separate cargo invocations (together they unify features and `semio-s-fem-app-surface` fails with `grow` not in scope, `HasStackLayout`). Expect the declared-verb laws to name further silent verbs if any remain; fix those by refusing with a coded fault plus EN/DE notice.

## Housekeeping

- `C:\t\$D` (a literal directory from a first quoting mistake in `fc-pkg.sh`) could not be removed (the delete guard refused); please delete it.
- Edited framework files outside FEM: `📋️clipboard/🦀️.rs` (declaration) and `📬️typed-operation/🦀️.rs` (probe removed).
