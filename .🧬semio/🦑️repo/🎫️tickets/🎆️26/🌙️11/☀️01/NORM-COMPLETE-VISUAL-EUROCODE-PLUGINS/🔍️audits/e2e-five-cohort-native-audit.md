# Five Non-Eurocode Norm Cohort Gate Map

Read-only source audit, 2026-11-04. No native build or test launched. Source presence is not a compile pass.

All five have native artifact, model, checks, inference, editor and viewer package owners and explicit artifact workspace manifests. Their registered Rust project targets all include build/check/test plus test-snapshot-sqlite, test-snapshot-sqlite-native and test-snapshot-sqlite-source. Their scripts delegate to the owned runArtifactRustPackageMain with an explicit package roster.

| Cohort | Registered native project | Artifact workspace manifest |
| --- | --- | --- |
| 🧱️din4108 | @semio-tech/norm-din4108-rs | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/Cargo.toml |
| 🌬️din16798 | @semio-tech/norm-din16798-rs | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/Cargo.toml |
| ⚡️din18599 | @semio-tech/norm-din18599-rs | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/Cargo.toml |
| 📇️iso16757 | @semio-tech/norm-iso16757-rs | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/Cargo.toml |
| 🏭️vdi3805 | @semio-tech/norm-vdi3805-rs | ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml |

For each project use repo run <project>:build, :check, :test. Exact scoped native smoke commands from repo root (replace FAMILY/MANIFEST with one row below) retain all ordinary repo gates:

    repo --service=off cargo --store=shared --configuration=prototype -- check --manifest-path MANIFEST -p semio-s-artifact-norm-FAMILY-editor -p semio-s-artifact-norm-FAMILY-viewer -p semio-s-artifact-norm-FAMILY-inferences
    repo --service=off cargo --store=shared --configuration=prototype -- test --manifest-path MANIFEST -p semio-s-artifact-norm-FAMILY-inferences --lib

- din4108: repo run @semio-tech/norm-din4108-rs:check; scoped manifest ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/Cargo.toml; packages semio-s-artifact-norm-din4108, semio-s-artifact-norm-din4108-model, semio-s-artifact-norm-din4108-checks, semio-s-artifact-norm-din4108-inferences, semio-s-artifact-norm-din4108-editor, semio-s-artifact-norm-din4108-viewer.
- din16798: repo run @semio-tech/norm-din16798-rs:check; scoped manifest ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/Cargo.toml; packages semio-s-artifact-norm-din16798, semio-s-artifact-norm-din16798-model, semio-s-artifact-norm-din16798-checks, semio-s-artifact-norm-din16798-inferences, semio-s-artifact-norm-din16798-editor, semio-s-artifact-norm-din16798-viewer.
- din18599: repo run @semio-tech/norm-din18599-rs:check; scoped manifest ✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/Cargo.toml; packages semio-s-artifact-norm-din18599, semio-s-artifact-norm-din18599-model, semio-s-artifact-norm-din18599-checks, semio-s-artifact-norm-din18599-inferences, semio-s-artifact-norm-din18599-editor, semio-s-artifact-norm-din18599-viewer.
- iso16757: repo run @semio-tech/norm-iso16757-rs:check; scoped manifest ✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/Cargo.toml; packages semio-s-artifact-norm-iso16757, semio-s-artifact-norm-iso16757-model, semio-s-artifact-norm-iso16757-checks, semio-s-artifact-norm-iso16757-inferences, semio-s-artifact-norm-iso16757-editor, semio-s-artifact-norm-iso16757-viewer.
- vdi3805: repo run @semio-tech/norm-vdi3805-rs:check; scoped manifest ✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.toml; packages semio-s-artifact-norm-vdi3805, semio-s-artifact-norm-vdi3805-model, semio-s-artifact-norm-vdi3805-checks, semio-s-artifact-norm-vdi3805-inferences, semio-s-artifact-norm-vdi3805-editor, semio-s-artifact-norm-vdi3805-viewer.

## Actual Supported UI Shape

Each of these editors implements ArtifactEditor and registers an edit mode with inputs/results windows. Each viewer implements ArtifactViewer and registers a view mode with a report window. The application dialect/document constants are family-owned; each editor documents semio.norm.<family>/v1. Hub NormApps registers their separate editor/viewer pair. These are deliberate existing surfaces, not the ten-Eurocode outliner/model/properties/checks/diagrams layout.

The inspected DIN4108 editor uses NoPresence and NoTransient, routes current retained tool jobs through build_norm_tool_job and registers migrated setSelectedCheckIndex/setField/insertItem/removeItem/applyRemedy actions. Its viewer uses NoPresence/NoTransient and read-only ViewEmit with no lane-based selection surface. The analogous five editor/viewer main sources mount inputs/results versus report. Therefore the ten-Eurocode NormViewerFrame/view-focus event rollout does not establish their UI behavior. Extend cohort-specific witnesses over their actual input/result/report controls rather than assuming a 3D/check-step UI exists.

## Static Findings and Limits

No missing editor/viewer/inference package manifest or absent artifact Cargo workspace was found. No definite native compile error was established by this limited source audit. Strict fifteen-family twins prove wire schema/readers and byte contracts, not these native editor render paths, retained commands or inference evaluation. Full registered targets remain the authoritative way to expose API drift and behavior failures. Root Cargo excludes these artifact workspaces, so an ad hoc root-cwd -p selector without explicit manifest is incorrect.

Next native extension: build/check/test each of the five registered projects, including their sqlite native/source suites; then create/open a cohort-specific committed example, edit one real typed field and observe a dependent input/result/report quantity plus undo/reload. Confirm viewer read-only behavior separately. Progress/cancellation, EN/DE accessibility and multiuser requirements remain actual execution gates rather than inferred from Eurocode source.

## Concurrent Browser Selector Update

Parent has now scoped the EN1990 witness owner selector using schema-owned ownerWindow: React reads closest data-slot=window, wgpu uses mirror window. This supersedes the earlier shortest-owner-row ambiguity in e2e-browser-scenario-review.md. No further browser result is inferred from that source correction.
