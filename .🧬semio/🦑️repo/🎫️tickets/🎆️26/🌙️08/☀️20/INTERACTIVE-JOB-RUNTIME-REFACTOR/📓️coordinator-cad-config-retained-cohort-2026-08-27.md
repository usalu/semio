# CAD Config Retained Cohort

## Outcome

- Migrated 19 bounded CAD config routes plus the framework-injected `setActiveUtility` through `CadRetainedCommandJobFactory`.
- Kept `loadRawRequest` on its exact `HostOnly` lane and added `addNode`, `renameNode`, `patchCadPlayReference`, and `focusModelDefinition` with exact Artifact preparation, yielding 24 of 40 app-owned fixture routes and 25 admitted factory keys.
- Added exact `Config` publication contracts and a bounded, reversible `CadConfigStorePreparationFactory` sealed through `ArtifactStoreOneItemLiveAuthority::prepare_one_item`.
- Admission caps dynamic config state at 256 selected-node items and 65,536 retained bytes; oversized pre/post roots and mutations fail closed.
- Left CAD's 16 remaining document, child, decode, and serialization routes honestly `BatchOnlyPendingRewrite`; they require separate resumable algorithms.

## Evidence

- `bun ./📜️script.ts retained-audit` in the CAD TypeScript package: passed, 40 fixture routes / 24 migrated / 16 batch-only / 25 admitted factory keys, with Ajv schema and hostile-source checks.
- `git diff --check -- '✏️s/🔌️plugins/📐️cad'`: passed.
- Official verifier output: `📊️coordinator-official-tool-jobs-live-r16-cad-artifact-2026-08-27.json`. CAD has no remaining/unclassified, scan-then-monolith, or process-global payload row. Explicit batch-only routes remain incomplete work, not completed features.
- Native `cargo check -p semio-s-plugin-cad --lib --message-format short` with the ticket-owned `🧱️cargo-target-cad` passed on 2026-08-27 in 2m 27s after correcting the retained reducer's selection field lookup. The full dependency graph, including stdio, compiled.
- Nx, Wasm, runtime tests, and rustfmt remain pending.

## Files

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️retained-jobs/🔣️component.json`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️retained-jobs/🔣️schema.json`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts`
