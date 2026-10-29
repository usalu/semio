STATUS: IN PROGRESS (executor birthresult, wave 1029) — all edits applied, verification checks queued.

## Design
- `ReferenceArtifactAppBirth<P>` (`plugin/📜️declaration/🧩️app/🦀️.rs`) is now `fn()->Result<P, semio_framework::Fault>`; `ArtifactApp/ArtifactEditor/ArtifactViewer::reference_birth()` keep `Option<ReferenceArtifactAppBirth<Snapshot>>` (None = no explicit producer).
- New `reference_birth_fault(step, cause)` (same file; re-exported as `semio_framework_plugin::reference_birth_fault` + `ReferenceArtifactAppBirth`): Fault `app.reference-birth.refused`.
