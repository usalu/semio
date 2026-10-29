# Artifact Hygiene R17

Checked ticket JSON artifacts by filename and size. The large generated geometry-r11.json is a derived physical geometry export, not a copy of the checked-in semantic source. Primary-beams actual files contain traced fixtures/partition deltas. The model-r13-tests/model-r14-tests files are validation results. No checked-in model source copy was found among these candidates.

Runtime caches, extracted archival figures, QA sheets and result logs remain under the ticket generated directory while the goal is active. Preserve reusable authoring/validation scripts, input fixtures and Markdown audits. Remove generated outputs at actual ticket completion; do not remove retained inputs or audit reports.
