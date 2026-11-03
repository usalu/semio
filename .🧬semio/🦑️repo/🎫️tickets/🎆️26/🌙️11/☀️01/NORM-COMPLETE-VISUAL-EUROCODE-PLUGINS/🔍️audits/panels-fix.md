# Panels fix (app-surface-panels)

## State found
All listed errors were already fixed in the working tree when inspected (by an earlier edit, not by me):
- properties `meta.choices()` called; CheckStatus arms (UnverifiedBasis -> Warning band/slot/tone, `?` glyph; Info -> neutral, `ℹ`) present in utilisation-bar, checks (`admits_status`, `status_slot`), field-feedback (`status_glyph`, `severity`, `tone_of`) and ids-chrome-and-labels (EN/DE "Unverified basis"/"Ungeprüfte Grundlage").
- remedy-chips: `chip_id.clone()` / `&chip_id` used; `UiFixedList<_>` annotated (default const N).
- viewport3d uses structural-scene's own 4-variant `CheckStatus` (exhaustive, unaffected).
No file edits were made by this agent.

## Validation attempts (norm-en1995-rs:build)
- panels/1.txt: Nx daemon could not compute the project graph (daemon terminated), no rustc reached.
- panels/2.txt: "NX Funded Nx graph cache capacity exhausted" in graph admission, no rustc reached.
Both are infrastructure failures, not compile errors. Shared crates are therefore NOT artifact-validated; a retry is needed once the Nx graph cache capacity is freed (framework/cache owner).
