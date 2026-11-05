# Lowpoly Validation Scope Audit

## Finding

The current test-runtime router selects semio-hub-lowpoly, semio-s-artifact-lowpoly-lowpoly-editor, semio-s-artifact-lowpoly-lowpoly-engine and semio-s-artifact-lowpoly-lowpoly-io. It omits semio-s-artifact-lowpoly-lowpoly-mut-objects. The new canonical_create_object_schema_preserves_complete_managed_mesh_ownership regression is mounted by the lowpoly_objects_mutations integration target in that omitted package. Compiling the package as a dependency does not execute its tests. Add that exact package to the runtime selection before treating the runtime target as complete regression coverage.

## Confirmed Source Wiring

The editor manifest explicitly declares lowpoly_editor_mounted, lowpoly_commands_object and lowpoly_commands_media, so the managed-history, duplication and current-geometry assertions are selected by the editor package. Engine and IO unit modules are cfg(test) mounted in the selected package libraries. The composition activation regression is cfg(test) mounted by the composition library.

Checked all include_str fixture paths in the composition activation, mounted, object duplication, IO unit and CreateObject schema regression files. All resolved to existing files, including the repaired descriptor and framework schema references. The object group manifest supplies semio-framework-schema-validator and serde_json as dev dependencies. IO supplies gltf with import and utils features as a dev dependency. This is source verification, not execution of schema compilation or Rust tests.

## Command And Admission

Canonical coordinated command: repo run @semio-tech/lowpoly-plugin:test-runtime --parallel=1. The target forwards arguments, hashes composition default inputs plus ^default and the entire Lowpoly plugin owner, and calls runRepositoryCargoTests. That helper partitions selected package manifests by Cargo workspace, prepares each exact selection, obtains repository test policy and enters the issued compiler lane with crates set to request.packages. The native test planner selects all Cargo default test targets for those packages; it does not add --lib or a named test filter. Nextest selection and bounded build/assertion budgets come from the owning policy; compiler arguments add --locked and --offline. No extra raw Cargo command or manually invented capability is indicated.

No native command, build, infrastructure mutation, or browser run was performed by this audit. Budget/cap readiness and shared preparation remain runtime facts requiring the coordinator. The current status correctly avoids claiming native execution.

## Read Boundaries

Reads were confined to status/verification/active-runs, Lowpoly composition router/project/crate/surface files, the Lowpoly owner tree, and exact Cargo admission/test planner entry points. Only this audit Markdown file was written.
