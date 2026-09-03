# Artifact Opening and Shared Document Report

## Outcome

The React shell's replay seam now accepts plain `os.open-artifact` relays without requiring `pluginId`/`appId`. It resolves the selected editor/viewer from the live `AppRouter` plus the event-sourced `OpeningPreferences` projection. `os.open-artifact-with` remains an exact registered-app selection. A relay attaches the actual `documentId` and `schema`, with optional `spaceId`, to the session created for the resolved app rather than to the previous React session.

The space index's open, open-with, and create-and-open effects now all carry the row's real schema. The native WGPU relay parser mirrors role normalization and relay validation and no longer substitutes the document id or dialect coordinate as a schema.

## Contract

- `artifactRef` is a dialect coordinate or a canonical role-suffixed surface id.
- `role` accepts `"viewer"`, `"editor"`, `0`, or `1`; omission defaults to editor. A suffix and explicit role must agree.
- Plain open resolves through live catalog ordering and opening preferences unless a complete explicit app ref is supplied.
- Open-with requires a complete explicit `pluginId`/`appId` registered for the dialect and role.
- Document attachment requires the complete `documentId`/`schema` pair. `spaceId` is optional for local-only documents.
- The app-channel notification uses the canonical role-suffixed surface id.

The language-neutral schema and vectors cover six valid paths and seven rejected paths. The TypeScript suite validates the vectors with Ajv before testing the in-repo resolver. The same JSON vectors are consumed by the Rust WGPU unit test.

## Implementation

- Added `resolveArtifactOpeningRelay` to the OS protocol surface.
- Replaced the React replay handler's `pluginId && appId` gate with canonical resolution.
- Made app opening return the newly-created session/plugin pair and passed it into `openDocument`, closing the React dispatch race.
- Added the pure `resolveDocumentOpeningTarget` selector and a focused regression test proving the new session wins over the still-current previous session.
- Added real schema fields to all three space artifact-opening effects and assertions.
- Expanded the WGPU relay parser and shared-vector coverage; document open uses the exact supplied schema.

## Verification

Passed:

```text
bun nx run @semio-tech/framework-renderer-react:test-long -- '🧪️opening.test.ts'
Test Files  1 passed (1)
Tests       2 passed (2)
exit 0
```

The two tests cover the Ajv-validated shared contract vectors and the ShellHost new-session attachment race.

Typecheck was executed and failed on concurrent baseline diagnostics outside this lane:

```text
bun nx run @semio-tech/framework-renderer-react:typecheck
exit 1
```

The reported diagnostics are existing demonstrator/tutorial/icon/resident/actor-store issues. There were no diagnostics in `🧪️opening.test.ts`, `ShellHost/🧭️opening/🟦️.ts`, the new OS resolver, or this lane's changed ShellHost lines.

Rust tests executed: zero. The canonical target fails before Cargo because Nx has inferred a stale moved cwd:

```text
bun nx run @semio-tech/framework-renderer-wgpu:test-native -- open_artifact_relay_vectors_match_the_typescript_contract
error: Module not found "./📜️script.ts"
exit 1
```

`nx show project` reports the project root under `engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`, but its generated target cwd is the obsolete `engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu`. A direct invocation of the checked-in router reached Cargo/nextest discovery but produced no test output while the shared Cargo workspace was heavily contended, so it was cancelled rather than reported as a pass. The space Rust suite was not started under the same contention. Collaboration E2E steps 3–5 were not attempted because their native build prerequisite was not green.

Residual verification after the shared Rust/Nx infrastructure is repaired:

```text
bun nx run @semio-tech/framework-renderer-wgpu:test-native -- open_artifact_relay_vectors_match_the_typescript_contract
bun nx run @semio-tech/framework-renderer-wgpu:test-native -- open_artifact_relay_target_parses_document_and_space_ids
bun nx run @semio-tech/space-plugin:test-long
bun nx run @semio-tech/framework-renderer-react:typecheck
```

Then run collaboration E2E steps 3–5 unchanged.

## Files

- `🧰️framework/🛍️products/💻️os/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🧭️opening/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️opening.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🧪️fixtures/📂️open-artifact/🧬️schema.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🧪️fixtures/📂️open-artifact/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/{📂open-artifact,🗃️open-artifact-with,🌱create-artifact}/🦀️.rs`
