# Live Wiring: Immediate Re-evaluation and Synchronized Selection (all ten Eurocode editors)

Status: code written for all ten artifacts and the shared crates, NOT compiled or tested. Per the build loop, the artifact norm agents build; every `🗑️generated/build-enXXXX/latest.md` has a "ready for rebuild: live wiring" line, and compile concerns for the shared crates are in `📌️important/requests.md` (addressed to `integrator`). Script used for the mechanical per-artifact edits: `🧰️scripts/live-wiring-rollout.py` (idempotent).

NORM = `✏️s/🔌️plugins/📕️norm`, SUBSET = `NORM/🗿️artifacts/<emoji>enXXXX/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. Live evaluation (one retained session per artifact)

- `NORM/⚖️compliance/⚡️incremental-evaluation/🛰️live`: macros `live_fold_source!(Source, Document, evaluate_fn)` (one unit reading `*`, the full fold for families without check units) and `live_report!(pub mod live(Document) = Source)`. The generated module holds a `OnceLock<SharedEvaluation<Source>>` and exposes `report(&doc)`, `progress()`, `cancel()`. `SharedEvaluation` gained `progress()` and `revision()`.
- `SUBSET/🧬️schema/💡️inferences/🦀️.rs` (all ten): `live::report` is the only evaluation path of editor and viewer. EN 1993 uses `En1993Units` (per-entity CheckUnits, incremental); EN 1990-1992 and 1994-1999 use the fold source (memoised by read hash, full fold on change; each gains incrementality when its `Units` land, by swapping the source type only).
- `NormFamily` gained `live_report` (default = `evaluate`), `live_progress` (default idle) and `cancel_live` (default no-op); the ten editor families override them with `inferences::live::{report, progress, cancel}`.
- `NormHost::from_artifact`, the global report cache (`cached_report_for`, `store_cached_report_for`, `invalidate_cached_report_for`, `report_cache`) and `document_revision_key` are deleted (no compatibility layer). New: `NormHost::live(document)` (settled live report). `NormHost::apply/replace_document/evaluate` re-settle through the same session. All `from_artifact` call sites, including the five non-Eurocode norm apps and every test, now call `live`.
- Editor render: `render_body` (free fn per editor root) builds `LiveView::<Family>` (host with settled report, resolved selection, focus, worst entity, progress). No render path calls `evaluate`; the `evaluate` command stays as an explicit "re-run now" job (progress stages, cooperative cancel now calls `cancel_live`), it is no longer a dependency of any paint. Viewer windows (`checks`, `viewport`, `diagrams`) call `inferences::live::report` (EN 1993 viewer/editor `thread_local` copies removed, so editor and viewer share the single session and read the same result).
- App-surface consumers (`export_media` report:out, remedy application, remedy preview, inspector helper, evaluate job) use `F::live_report`.
- Progress and cancellation: `live::progress()` -> `LiveView.progress` (`EvalProgress{running, done, total, cancelled}`), `live::cancel()` supersedes the run in flight (generation token, latest edit wins), the retained `evaluate` job cancels it. Paint-time settling is synchronous (fast path: equal document = no work), so the in-flight window is only observable for T2 units on a worker (`run_async`); no worker is spawned yet.

## 2. Selection (per artifact) and the mirror

Decision: the transient lane type (`NormTransient`) is not wired as the editors' `ArtifactEditor::Transient` (it would need a complete artifact schema: ArtifactDsl, pack, mutations, io). The framework already owns an ephemeral/persisted-local selection store, the interaction domain `norm.<variant>` (viewport picks, checks rows and tree rows write it), and `render_with_request_context` hands it to a render. So each editor now overrides `render_with_request_context`, reads `interaction.selection(interaction_domain_id(VARIANT)).ids` and passes it to `render_body`; `render` (no interaction) passes `&[]`.

- `NORM/📇️registry/🧬️contract/🖥️app-surface/🪄️selection-mirror`: `NormSelection::resolve(report, stored_check_index, picked)` accepts scoped addresses (`member/b1`, `check/<id>`, `#part` suffixes) and raw entity ids; picks come first, the selected check's entity is added, an unselected check falls back to the governing check of the first picked entity. Outputs: `entities`, `check`, `paths` (the check's `subject.path`, the field path), `primary()`, `outliner_state()`, `checks_selected()`, `highlights(scene entities)`. `LiveView<F>` bundles it with the host.
- Mirrors: outliner (`OutlinerState.selected` = entities + field paths), properties (`InspectorOptions.outliner`), viewport highlight (all selected entities and their parts, multi-select), checks (selected row = governing/selected check), diagrams (focus = primary entity, else worst entity).
- Cross-window write: `setSelectedCheckIndex` from the outliner or properties window used to fail (`norm-results-window-kind-required`, the addressed window is not the checks window). `norm_results_config::addressed` now targets the addressed or focused window when it is a Results window, else the first Results window instance.
- Limitation: stored check index (window config) and picks (interaction store) carry no timestamps; the mirror shows their union and focuses the check's entity. Clearing a pick does not clear the stored check.

## 3. Outliner as its own window role

`SUBSET/✏️editor/🎭️modes/✏️edit/🪟️windows/🌲️outliner` created for the nine remaining artifacts (EN 1993 reworked), `outliner::definition()` and the body arm, layout: outliner above properties (22 % column), mode test lists five windows, `window_kinds.len() == 5`. `render_outliner_document` lists the scene `ENTITY_KINDS` first and appends every other entity list of the document labelled from the field meta (no hand-kept support table); `render_outliner` was removed (no other caller).

## 4. Tests added

- compliance: unchanged unit suite for the session; the fold source is covered by the per-artifact equality assertion below.
- app-surface `🪄️selection-mirror/🧪️tests`: stored check -> entity + field path; pick -> governing check; check row + pick union; stale index; highlights incl. parts. `🌲️outliner/🧪️tests`: document outliner orders kinds before support tables and marks the selection.
- per artifact (editor `🧪️tests/🔬️unit`): `render_right_after_an_edit_shows_updated_checks_without_evaluate` (SetField then render the checks body, no evaluate dispatch; asserts the retained session equals the full `inferences::evaluate`, "No checks computed" absent, the worst check of the edited report is shown), `a_picked_entity_selects_its_governing_check_in_every_window`; outliner window tests; EN 1993 `incremental` tests now read `inferences::live::report`.
- Language-agnostic/third-party oracle: the equality oracle is the pure full fold (`evaluate`), as in WP-05; no new numeric domain logic was added, so no new Python oracle.

## 5. Not done / open

1. Nothing compiled or run (queue); expect first-compile fixes listed in `requests.md`.
2. Viewer windows read the live report but do not mirror a selection (the viewer has no interaction domain wiring in `render`); only the editors do.
3. Fold sources are not incremental per entity for EN 1990-1992 and 1994-1999 (EN 1993 is). Each needs its `Scope`/`Units` split (checklist 8b) and then only the `live_report!` source type changes.
4. No async worker/cancel button for T2 units yet; progress and cancel exist as session API plus the `evaluate` job cancel.
5. `NormTransient.eval` (live-lanes) is not fed from `live::progress()`; needs the transient lane wired as the editors' `Transient`.
6. The five non-Eurocode norm apps now evaluate fully on every paint (family default); give them `live_report!` if that matters.
7. Rollout checklist updated (8b, new 8d, 8c).
