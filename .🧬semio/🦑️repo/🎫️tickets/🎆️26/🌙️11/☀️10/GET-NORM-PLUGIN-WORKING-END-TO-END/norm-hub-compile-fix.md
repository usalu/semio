# Norm Hub Compile Fix

The native norm component build (run `w7eNLWo`) compiled 1569/1570 units; `semio_hub_norm` (wasm32-wasip2, prototype) failed with 8 errors:

- `semio_framework` / `semio_framework_value` unresolved inside the `dyn_enum_close!` expansion of `PluginApp` (the reactor-app trait names those crates by path) → added both as direct dependencies of `semio-hub-norm`, as every sibling hub does.
- `semio_s_plugin_norm_app_surface` unresolved at `.shell_hooks(…NormShellHooks)` → it was only a dev-dependency; moved to `[dependencies]`.
- `NormApps: From<VcsArtifactApp<ViewerApp<Din18599Viewer>>>` unsatisfied → the enum variant (and the editor) use `SemioMembers`, but `Din18599Viewer` kept the default `Members`; added `type Members = semio_s_artifact_stdio_semio::SemioMembers;`.

Files:
- `🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs`
