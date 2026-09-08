# Workflow Run Artifact Package

The framework-owned workflow run artifact is included in the complete 93-artifact scope. Its package is `semio-framework-artifact-workflow-run`, with Nx project `@semio-tech/workflow-run-rs`.

The language-neutral package fixture was added before the implementation. AJV validated both event/projection cases, then the package existence assertion failed as expected because the standalone Cargo declaration did not exist. The cases cover sealed-run admission and duplicate-start admission. The Rust test compares domain JSON decoding/encoding to serde_json and checks the projected event results.

The package declaration points to the artifact's taxonomy root. Snapshot types/codecs, diff composition, and mutation application live in schema facets outside the package directory. Existing run laws moved alongside the fixture. The workflow graph module no longer owns or reexports these types, and the runner library and executable consume the artifact directly. Runtime dependencies are only the repository kernel and value derive crate; serde_json is a test oracle.

Cargo and Nx verification is pending workspace registration and execution. This report does not claim runtime success yet.

The independent package router check passed (exit 0): `bun ./📜️script.ts check`, using the ticket Cargo target. Cargo compiled only the new artifact plus its kernel dependency closure; elapsed 4m29s includes waiting for the shared build lock. The Nx test target is running; runtime results remain pending.

## Workflow Graph Ownership

The adjacent `os.workflow` document was previously source-mounted by the framework monolith. Its complete domain implementation, mutation taxonomy and retirement facets now belong to `🗿️artifacts/🔁️workflow`, with package `semio-framework-artifact-workflow-workflow` and Nx `@semio-tech/workflow-workflow-rs`. The framework no longer mounts or reexports the document. The OS host and runner consume the artifact directly. This dependency direction permits the artifact's media/app contracts to depend on the framework without introducing a reverse edge.

A language-neutral empty-workflow serialization fixture and AJV schema were added first. AJV validation passed, then the independent Cargo-declaration assertion failed as expected. Existing workflow laws moved with the implementation, and a JSON third-party oracle plus DSL/pack equivalence test checks the fixture. The independent compile/test gates are pending.

Nx run verification passed: `bun nx run @semio-tech/workflow-run-rs:test --excludeTaskDependencies --output-style=static`, 19 unit tests passed and 0 failed; doc tests contained 0 tests. The Nx target completed in 6m02s including graph preparation/build-lock time.

Focused run-fixture runtime verification also passed (1 test, 18 filtered): both sealed-run and duplicate-start cases printed their `[DEBUG]` pass messages. Workflow fixture field spelling was checked against the value derive macro: unannotated field names retain snake_case, so the fixture preserves existing wire behavior.

The first workflow Nx test compile rejected moved mutation metadata: 18 descriptors still named their original source owner. The descriptor paths were corrected to the new artifact taxonomy, preserving source-authority enforcement. A second Nx test run is in progress. The framework self-alias used only by the removed workflow mount was also removed after Cargo reported it unused.

The isolated Nx workflow-workflow test target completed successfully: **37 tests passed**, including the language-neutral JSON oracle with its `[DEBUG]` console confirmation. Cargo elapsed 17m38s includes the shared build-lock wait; it is not a cold-build benchmark. The shared Playbook target remains in the run-many queue.
