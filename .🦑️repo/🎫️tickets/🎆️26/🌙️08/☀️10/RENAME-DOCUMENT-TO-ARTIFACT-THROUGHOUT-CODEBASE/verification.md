# Rename Verification

## Completed

- Replaced the remaining Rust core re-export and asset references for the artifact panel.
- Resolved `AppDefinition`'s ambiguous legacy path field as `breadcrumb`, preserving `label` for the existing localized visible label.
- Updated the generated manifest mirror and Rust/TypeScript renderer call sites to use `breadcrumb` and `terminologyBreadcrumbs`.
- Renamed the renderer breadcrumb helpers to `appBreadcrumb`, `resolveAppBreadcrumb`, and `appWindowLabel`, avoiding a collision with the existing plugin-overlay `resolveAppLabel` helper.
- Migrated the public TypeScript kernel and OS sync protocol types from `Document*` to `Artifact*`, including the `ArtifactCommand` wire variant while retaining its established numeric tag.
- Changed the framework-owned panel's visible label, translation key, and tree identifiers from document to artifact.
- Moved the generated WGPU icon assets to `artifact_jack.svg` and `artifact_report.svg` so the renamed Rust icon enum resolves them.

## Validation

- `bun nx run @semio-tech/framework-rs:generate` completed successfully and refreshed the generated TypeScript manifest mirror.
- `bun nx run @semio-tech/framework-rs:check` completed successfully after the Rust rename fixes.
- `bun nx run @semio-tech/framework-renderer-react:test-quick` completed after the renderer rename fixes; the previous duplicate `resolveAppLabel` transform failure no longer occurs.
- `git diff --check` reported no whitespace errors.

## Existing Verification Limits

- `bun nx run @semio-tech/framework-renderer-react:test` exceeds the repository's 15-second test budget before reporting results.
- `bun nx run @semio-tech/ui-react:typecheck` remains blocked by broad pre-existing workspace type errors (missing generated symbols, duplicate existing imports, and unrelated UI type mismatches). The run did identify and the work fixed the rename-specific incomplete AppFrame branch and duplicate manifest field it exposed.
- `bun nx run @semio-tech/framework:test` remains blocked by existing duplicate/missing test fixture failures.
- `bun nx run @semio-tech/framework-os-kernel:check` remains blocked by its existing broken `📜️script.ts` relative import.
