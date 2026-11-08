# fleet-1108 shared-surface

Status: edits made, NOT yet compiled; gated run `@semio-tech/norm-artifact-contract-rs:test` log `🗑️generated/shared-surface/1.txt` (see end of file for its outcome).

## 1. visual-chrome E0425 (3)
No `error[E0425]` text survives in any current log (`audit-build.md` row: locations elided, names verified to resolve). Static check of the crate root imports against the facade found nothing unresolved; the run above is the verification.

## 2. Audit #44 (blocking live report)
- `LiveView::with_lanes/within/over_lanes` already paint through `NormHost::live_within` (16 ms `PAINT_BUDGET`, 4 ms drag budget, cached drag report, progress and cancel); only tests use `LiveView::new`.
- The remaining synchronous paint-path reads were in `🫧️live-lanes/🎛️editor-wiring`: context menu, view-step resolution, interaction topology, check/view focus effects, `live_scene`, `live_diagrams`, `evaluate_units`. All now call new `paint_report::<F>` (= `F::live_responsive(doc, PAINT_BUDGET).0`). Test: `the_editor_wiring_paints_only_from_the_responsive_report`.
- The 65 `inferences::live::report` hits are artifact files (5 per artifact, 13 artifacts): requests filed under `## fleet-1108 → <artifact>`.
- Kept synchronous on purpose (need the exact settled report): apply remedy, preview remedy, report export.

## 3. Audit #87 (English reasons)
Already satisfied by `⚖️compliance/🗣️reasons` (rules `input.missing`, `input.missing-any`, ... EN+DE; evaluators call `reason_copy`; `german_leaks` census). No app-surface change needed; the evaluator `format!("missing input {key}")` strings are keys of the rule table, not displayed text.

## 4. Audit #13 (verification-case catalogue)
`🗂️design-case-surface`: `verification_catalogue_table::<C: CaseCatalogue>(family, annex, query, locale)` evaluates each entry once on its seeded example inputs and renders id, clause, part, title, worst status, utilization, governing explanation (EN/DE). It is a `CatalogueTable`, so artifacts mount it in `render_catalogue`. Test `verification_table_evaluates_each_entry_once_and_speaks_both_languages`. Mount requests filed for en1990-en1998.

## 5. DIN 4108 window ids (relayed by din-a)
- `🌎️hub/🧩️compositions/📕️norm/🖥️app-surface/🧪️tests/🧫️fixtures/🔣️.json`: every editor/viewer row re-derived from the `BODY_*` constants of each artifact source (all variants except din16798 and vdi3805, which still own `inputs/results`). Viewer rows now list `view.{viewport,diagrams,checks}`; unverified by a hub run.
- `🪟️results/🎚️config/🧪️tests/🧫️fixtures/🔬️window-ownership/🔣️.json`: `norm-{din4108,en1991,en1995,en1997,iso16757}-results` -> `-checks`.
- NOT done: hub `🧩️component` twins (`🔣️.json`, `🛂️.descriptor.semio`) must be regenerated with `repo run workspace:components describe norm`, not hand edited.

## 6. Family-free paint for viewers (en1994 request)
`paint_report_of(|budget| <inferences>::live::responsive(document, budget))` in `🎛️editor-wiring` (viewers have no `NormFamily`); `paint_report::<F>` is implemented on top of it. Test `a_family_free_paint_reads_the_partial_report_within_the_paint_budget`. Requests updated: editors use `paint_report::<Family>`, viewers use `paint_report_of`.

## 7. Layout titles (iso-vdi request)
The framework `WindowLayoutWindowNode.title` is a single `Option<String>`, so it cannot be bilingual. The shared layout builders (`🪟️manifest-and-media`) no longer set it (`role_title`/`titled` removed); the shell shows each window kind's `LocalizedLabel::native(en, de)` (already EN+DE in every `window_definition`). Test `layout_windows_carry_no_single_language_title_so_the_localized_kind_label_governs`. No call-site change in artifacts (the helpers were private). Hub twins change with the next `describe norm`.

## 8. insert_edit on nested collections (iso16757 request)
`➕insert-tool/🦀️.rs`: `insert_edit` resolves `spec.collection` with `get_value_at_path` (dotted, e.g. `catalogue.products`) and accepts an array or an object map keyed by entity id (template = last value, `NormEdit::InsertItem { path, index: len }`). Test `a_dotted_collection_path_reaches_nested_arrays_and_id_keyed_maps`. Applying a map insert remains the artifact's `resolve_edit` (ISO resolves `geometry.objects` itself). Uncompiled.
