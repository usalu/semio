# P8c Release Catalog Dispatch Gate

## Outcome

The plugin SDK now treats interactive-job classification as a release and dispatch invariant.

- `AppBuilder::try_build_definition` collects app/window actions and app/mode commands and calls `semio_framework::validate_interactive_job_classification`.
- Unclassified declarations fail with `app-definition.interactive-job-classification`.
- Classified but non-UI-safe declarations (`BatchOnlyPendingRewrite`, `ForbiddenFromUi`, `Deleted`) fail the same release boundary. Only `Migrated` can enter an activated UI catalog.
- `VcsArtifactApp` independently checks action, addressed command, and typed-command dispatch against `AppActionRegistry`. A malformed or externally decoded catalog therefore cannot bypass release validation; it fails with `interactive-job.not-ui-safe` before the app handler runs.

## Coverage

Two focused contract tests cover all four rejected dispositions for both actions and commands:

- `release_catalog_rejects_every_non_ui_safe_action_and_command_classification`
- `ui_dispatch_backstop_rejects_every_non_migrated_action_and_command`

The dispatch test additionally asserts that the document snapshot is unchanged, proving rejected declarations never reach an app handler.

## Exact inventory

- Release construction validates four declaration domains: app actions, window actions, app commands, and mode commands.
- Release construction first calls `validate_interactive_job_classification`, then independently rejects every classification other than `Migrated` before building `AppDefinition`.
- The dispatch backstop covers direct action dispatch, addressed command dispatch, and typed-command dispatch through `AppActionRegistry`.
- Rejected classifications exercised by both focused tests: `Unclassified`, `BatchOnlyPendingRewrite`, `ForbiddenFromUi`, and `Deleted`.
- Release error contract: `app-definition.interactive-job-classification`.
- Dispatch error contract: `interactive-job.not-ui-safe`.
- UI-reachable accepted classification: exactly `Migrated`.

## Gates

- Native: `cargo test -p semio-framework-plugin --lib` — 304 passed, 0 failed.
  - Log: `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/📝️r18-p8-release-catalog-native-3.txt`
- Wasm: `cargo check -p semio-framework-plugin --target wasm32-wasip2` — exit 0.
  - Log: `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/📝️r18-p8-release-catalog-wasm-1.txt`

## Adjacent Compiler Boundary

- `cargo check -p semio-s-plugin-stdio --lib` — exit 0 after removing the 14 compiler-exact stale `Emit::mutations(...).await` sites exposed by the pure constructor seam.
  - Log: `PHASE-1-5-DE-ASYNC-REPAIR-SWEEP/📝️r18-stdio-central-sync-seams-check-1.txt`
