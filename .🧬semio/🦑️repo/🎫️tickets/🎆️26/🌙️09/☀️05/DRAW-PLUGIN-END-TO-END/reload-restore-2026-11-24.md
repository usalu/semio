# Reload restores the folder (2026-11-24)

Run r35 (en and de) passes boot, blank, example, attach, rectangle, undo, redo, persisted, export (12,372 changed pixels) and fails at `reload`: `initialization requires its original installed native semantic producer`.

Cause: the restore folds the archived edits through `DocumentStoreOwners::replay_preparation`. `ArtifactEditor`/`ArtifactViewer` default to `bounded_artifact_store_owners`, which installs none. Puzzle, sourcing, process and others override `build_document_store_owners`; Draw did not, although `DrawingArtifactStorePreparationFactory` already implements the replay factory.

Fix: `drawing_document_store_owners` (editor, one shared issuer for publication and replay) and `cold_replay_document_store_owners` (viewer). Law: `drawing_document_catalog_replays_persisted_history`. r36 contains the fix: `reload` passes in en and de (13 of 13 runs). The law itself has not been run.

Not changed on purpose: the operation wire. Replayed time-travel replacements (`InputReplacement::Input`) need an original receiving, which Draw's document wire does not install; undo and redo do not record such replacements (read from code). Reloading a document with a time-travel edit is therefore unverified.

The cancel fix from `job-cancellation-2026-11-24.md` is confirmed in r35: a running job is cancelled, ends as `original job was canceled`, and neither run shows `cancel-refused` or `start-stalled`.
