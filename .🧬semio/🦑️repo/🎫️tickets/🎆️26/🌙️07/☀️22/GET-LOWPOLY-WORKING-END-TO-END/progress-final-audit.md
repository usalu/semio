# Lowpoly Progress Final Audit

Read-only source review on 2026-11-04; no build, serve or native evidence.

## Result

Latest wiring uses canonical surface_app_id derived from LowpolyPlayApp::DIALECT and ::ROLE, replacing the initial lowpoly-play kit controller. Latest test compares fixture canonical s.lowpoly.lowpoly@1/*#editor against create_lowpoly_app().id/controller_id and EditorApp::surface_id, then checks rendered binding scope against the actual assembled controller. This independently addresses the wrong-literal risk.

Model and UV both wrap their existing node only while doc.operations is nonempty; exact same two window bodies appear in Partial dirty scope. Panels and all other scope flags remain excluded. Fixture checks both windows, existing original node, explicit English/German, polite status, exact operation/generation, cancelling disable and empty-snapshot removal. These are presentation assertions; no live cancellation claim follows.

## Compile Source Review

Public ArtifactOperationProgress import through plugin::app is valid (plugin root app reexports app_views; mounts wp2-declaration exports type). ArtifactView::with_operations accepts a borrowed slice. ArtifactEditor trait render signature matches the static test call. LowpolyPlayApp publicly exports through editor play_app; create_lowpoly_app publicly exports through manifest. EditorApp::surface_id is a public inherent async fn returning &str; resolve_ready can consume its immediately ready future, without extra trait imports. EditorApp default derives surface_app_id and its ArtifactApp::instance_id returns surface_id. ArtifactEditor::ROLE defaults Editor. Cargo registers lowpoly_editor_progress with correct relative path, existing semio-framework-async dev dependency, serde_json and plugin artifact-app-testing feature.

## Host Admission Evidence

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🔌️plugin-app-impl/📨️action-dispatch/🦀️.rs:7-9 rejects address.app_id differing from self.app.instance_id before reserved-action handling.
- Same plugin /🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:352-354 repeats that owner check for preview.
- Same plugin /📜️declaration/🪟️surface/🖊️editor-app/🦀️.rs:10 derives canonical ID; :17 surface_id exposes it; :40 instance_id delegates to it.
- Lowpoly editor /🕹️play-app/🦀️.rs:70 derives canonical scope; :94 operation_progress_scope targets Model and UV.
- Lowpoly editor /🧪️tests/📶️progress/🦀️.rs:36-41 checks assembled definition and actual EditorApp identity.

## Remaining Notes

No actionable wiring defect found in the latest source. Suggested explicit action.version == 1 assertion to runtime agent as a minor completeness improvement. Current locale cases couple English/running and German/cancelling; both states are checked but full locale-state Cartesian coverage is absent. Native cancellation transport and compiled test execution remain unverified by this audit.
