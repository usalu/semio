# Selection and Result Actions (FEM 3D, 2D twin)

## Root causes

1. **Select All no-op.** `fem3d`/`fem2d` interaction domains declared `HierarchyProvider::Flat` and the apps never implemented `interaction_topology`. `selectAll` walks that topology, so it selected nothing (Granularity had the same single consumer, so Set Granularity had no visible effect either).
2. **Set Selection Mode "refused ... viewport" and session poisoning.** Browser repro: `fem3d` is accepted on a clean page; `viewport` is legitimately undeclared. The defect is that any refused reserved verb poisons the session. `step_framework_reserved_commit` returned the plain `Fault` as a failed turn (commit left at the queue head), and the browser bridge (`component-bridge-source`, poll catch) holds that as a permanent native refusal, so every later dispatch replays the first refusal text (which made typed `fem3d` look ignored). Cut/Paste (`UnsupportedOwner`) share the path.
3. **Set Result Display.** Dispatched from the Model-window Actions palette it addressed the wrong window kind; the framework only publishes a window config for the window the command was dispatched from, so a cross-window fallback is rejected (`original window publication address differs from its captured authority`, seen in a law run and reverted). Its free-text Source also needed a canonical identity (`loadCase:...`).

## Fixes (files)

- 3D `✏️editor/🕹️interaction/🦀️.rs` (`fem3d_interaction_topology`, `HierarchyProvider::Topology`, Element first), `🖥️play-app/🦀️.rs` (`interaction_topology`); 2D `🕹️interaction/🦀️.rs`, `🎛️play-app-editor/🦀️.rs`.
- Framework `🔨️modules/🧵️job/⚠️failure/🦀️.rs` (`plain_refusal`), `🔌️plugin/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs` (a plain `Fault` seals the commit as that command's refusal instead of failing the turn).
- Result display: `👁️set-result-display/🦀️.rs` `resolved`/`canonical_source` (a case id or name resolves to its canonical source identity), routed in 3D `🗃️retained-commands` and 2D `🧾️retained-command-contracts-and-work` + `🎛️play-app-editor`; the action is claimed by the Results window kind only (`window_kind_action_refs` in 3D `📋️manifest` and 2D `🚀️ui-helpers-and-app-assembly`), so the Model window's palette never lists it. `addressed_window_id` keeps refusing a wrong-kind window, now saying to run it from the Results window. The panel/`remember` shadow was not touched.
- Bridge hold (`holdNativeRefusal` for semantic poll failures) left unchanged: with the framework fix a semantic failure no longer fails the poll. Needs wasm rebuild and a browser re-check.

## Laws

| Law | Status |
|---|---|
| `select_all_topology_lists_every_selectable_entity_3d` (interaction crate) | PASS |
| `select_all_topology_lists_every_selectable_entity_2d` | PASS |
| `a_source_name_resolves_to_its_canonical_identity_3d` (editor_app_2) | PASS |
| `a_refused_reserved_verb_does_not_poison_the_session_3d` (editor_app_2, `🛡️refused-verbs`: refused `setSelectionMode viewport`, then selectAll selects every element, clearSelection, typed domain, a typed command commits) | PASS |
| `result_display_from_the_results_window_palette_accepts_a_case_name_3d` | NOT RUN: the 3D editor build is currently broken by a peer's in-flight framework change (`semio-plugin-instance-stores`: `CommitPolicy`/`validate_commit_with`), and the disk filled (os error 112) |
| `set_result_display_is_offered_only_by_the_results_window_3d` / `_2d` (Model window palette does not list it) | WRITTEN, NOT RUN (same causes) |
| `a_refused_interaction_verb_does_not_poison_the_next_verb` (framework part-5) | WRITTEN, cannot compile: `semio-plugin-tests-09-2` is broken by a peer refactor (`framework-test-crates.md`) |

Note: the app-level refusal law passing in the 3D editor means the framework change in `reserved-routes` works end to end (before it, the queue head stayed stuck).

Not done: running the three NOT RUN laws above once the peer build is green; wasm rebuild and a browser re-check of Select All, Set Selection Mode (refusal then pick), Set Granularity and Set Result Display (the bridge `holdNativeRefusal` was left unchanged and should no longer be reached by a refused verb). Set Granularity has no fix of its own: its only consumer was Select All.
