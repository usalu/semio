# Norm app-surface test helpers ported to the current plugin API (2026-11-22)

## Causes
`semio-hub-norm` tests failed with 53 errors, all in `🌎️hub/🧩️compositions/📕️norm/🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs`:

- `PluginApp::render` now takes a `&mut StepContext` and answers `Result<Option<()>, OriginalTurnFailure>`; the tree stays owned by the app. `project_and_retire_fixture_tree(ComponentTree)` no longer fits.
- `Plugin::create_app` now takes `MountedOwnerPolicyV1` and an `EntityIdentityAuthority` and answers `Result<Option<App>, Fault>`.
- `close_registered_fixture_app` takes the mounted policy and the caller context and answers a `Result`.
- `assert_viewer_never_mutates` requires `NoTransient`/`NoTransientMutation` and `Command: Default`. Norm viewers own `NormLaneTransient` and non-default view commands, so the law does not apply to them.

## Fixes
- Shared helpers use one `OriginalFixtureScope` per app (`scope.cx()`, `scope.retire()`), `fixture_identity()` and `INTERACTIVE_COMPONENT`.
- Surface rendering goes through `render_registered_fixture_app`, the framework's bounded render, projection and release loop.
- Laws crate: `render_registered_fixture_app`, `render_registered_fixture_app_over` and `release_fixture_render` were generalised from `VcsArtifactApp<A, M>` to any `PluginApp` (they only used `PluginApp` methods), so Norm's `NormApps` enum can be rendered. Existing callers are unaffected.
- `assert_viewer_never_mutates` was dropped from `assert_norm_pair`. The editor/viewer dialect-sharing law stays; Norm viewers' non-mutation remains covered by their viewer unit tests and the ephemeral transient lane.

## Changed files
- `🌎️hub/🧩️compositions/📕️norm/🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🔎️declared-verb-probing/🦀️.rs`
