---
goal: R26-02
---

# Ticket

## Summary

Refactored all artifact kind derivation from hardcoded values to source-driven logic:

- **BundleKind**: Reads `bundleKind` from `package.json`/`project.json` at bundle root, falls back to `library`. Changed from emoji values to string values. Renamed `BundleKindClient` to `BundleKindUI`.
- **FolderKind**: Derives from folder name (`.`-prefixed and manifest-containing folders are `required`, others `organization`).
- **FileKind**: Comprehensive pattern matching (~60+ extensions) for `code`, `test`, `config`, `docs`, `resource`, `script`, `license`.
- **DefinitionKind**: Maps language keywords via `extractDefinitionKeyword` (word-before-name priority, modifier skipping) and `refineDefinitionKind` (const arrow function detection).
- Fixed GraphQL enum bugs (`"Client"` → `"UI"`, `"REQClientRED"` → `"REQUIRED"`).
- Renamed CLI flags `no-client`/`only-client` → `no-ui`/`only-ui`.
- Updated all 15 bundle `package.json` files with `bundleKind` field.
- Created 8 `project.json` files for schema bundles without `package.json`.
- Updated AGENTS.md and README.md documentation.
- All tests pass.

## Changes

- `semio-repo/cli/cli.go`: BundleKind type refactor, DeriveBundleKind, DeriveFolderKind, DeriveFileKind, DeriveDefinitionKind, extractDefinitionKeyword, refineDefinitionKind, ParseDefinitions, bundleKindEmoji, GraphQL enum fixes, CLI flag renames
- `semio-repo/cli/cli_test.go`: Updated test data keys and expectations
- `AGENTS.md`: Added Artifact Kind Derivation SRS section, updated codebase docs, removed stale `go/repo/` references
- `README.md`: Added Artifact Kind Derivation section, updated file paths
- 15 `package.json` files: Added `bundleKind` field
- 8 `project.json` files: Created with `bundleKind: "schema"`

## Log

- Explored codebase to understand current kind derivation
- Implemented BundleKind string values and DeriveBundleKind
- Implemented DeriveFolderKind with manifest detection
- Implemented DeriveFileKind with comprehensive pattern matching
- Implemented DeriveDefinitionKind with keyword extraction
- Added refineDefinitionKind for const arrow function detection
- Fixed extractDefinitionKeyword with word-before-name priority
- Fixed GraphQL enum bugs and CLI flag names
- Added bundleKind to all package.json/project.json files
- Fixed test failures and verified all tests pass
- Updated AGENTS.md and README.md documentation

## Todos

- [x] Refactor BundleKind from emoji to string values
- [x] Implement DeriveBundleKind from package.json/project.json
- [x] Implement DeriveFolderKind from folder name patterns
- [x] Implement DeriveFileKind with comprehensive pattern matching
- [x] Implement DeriveDefinitionKind with keyword extraction
- [x] Add refineDefinitionKind for const arrow functions
- [x] Fix GraphQL enum bugs
- [x] Rename CLI flags
- [x] Add bundleKind to all bundle manifests
- [x] Fix tests
- [x] Update documentation
- [x] Close ticket
