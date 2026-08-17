# 📓️ Research and Fix Plan: EditorApp / ViewerApp APP_ID Placeholder

## Problem Analysis

In `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`:
- `EditorApp<E>` and `ViewerApp<V>` implement `ArtifactApp` with `const APP_ID: &'static str = "surface";`.
- `ArtifactApp::instance_id(&self) -> &str` was introduced with a default of `Self::APP_ID`, and overridden in `EditorApp<E>` and `ViewerApp<V>` to return `self.surface_id()` (the canonical derived runtime id).
- While `handle_action_invocation`, `dispatch_command`, and `PluginApp::app_id` were updated to read `self.app.instance_id()`, `VcsArtifactApp::with_registry` was still constructing document envelopes and subsidiary store ids using `A::APP_ID`:
  - `create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, A::APP_ID, ...)`
  - `config_id = format!("{}-config", A::APP_ID)`
  - `draft_id = format!("{}-draft", A::APP_ID)`
  - `interaction_id = format!("{}-interaction", A::APP_ID)`
- As a result, every `EditorApp` and `ViewerApp` instance initialized internal stores keyed with the `"surface"` placeholder rather than the canonical surface app id (`s.space.home@1/*#editor` etc.).

## Proposed Fix

1. In `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` at `VcsArtifactApp::with_registry`:
   - Read `let app_id = app.instance_id();`
   - Use `app_id` in `create_document_envelope`, `config_id`, `draft_id`, and `interaction_id`.
2. Update the doc comments in `EditorApp` and `ViewerApp` to reflect that all internal envelope and store ids now read `instance_id()`.
3. Add unit test coverage in `SurfaceTestkit` asserting that `EditorApp` and `ViewerApp` instances construct envelopes carrying their canonical `surface_app_id` for document, config, draft, and interaction stores.
4. Verify by running `cargo test -p semio-framework-plugin --lib`.
