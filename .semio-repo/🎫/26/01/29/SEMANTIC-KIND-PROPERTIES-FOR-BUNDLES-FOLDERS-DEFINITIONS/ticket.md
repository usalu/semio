# Ticket

## Todos

- [x] Add BundleKind enum and kind property to Bundle (library, schema, binary, ui, site, assets)
- [x] Add FolderKind enum and kind property to Folder (organization, required)
- [x] Add semantic DefinitionKind categories (implementation, interface, constant)
- [x] Improve file kind detection beyond just file names
- [x] Add generated detection for folders like js/vscode/generated
- [x] Add filter flags for bundle kinds (--no-library, --only-library, etc.)
- [x] Add filter flags for folder kinds (--no-organization, --only-organization, etc.)
- [x] Add filter flags for definition kinds (--no-implementation, --only-implementation, etc.)
- [x] Add time dimension filters (--no-YEAR, --only-YEAR, etc.)
- [x] Add contributor dimension filters (--no-CONTRIBUTOR, --only-CONTRIBUTOR)
- [x] Add policy dimension filters (--no-POLICY, --only-POLICY)
- [x] Add breach-kind dimension filters
- [x] Update VS Code extension filter tree with semantic kinds
- [x] Update GraphQL schema for new kinds
- [x] Test and verify everything works

## Changes

1. ./semio-repo/cli/main.go - Added BundleKind, FolderKind, and semantic DefinitionKind enums with classification functions
2. ./semio-repo/cli/main.go - Updated Bundle, Folder structs with Kind field
3. ./semio-repo/cli/main.go - Added filter flags and streaming options for all new dimensions
4. ./semio-repo/cli/main.go - Added IsGeneratedFolder function for detecting generated folders
5. js/vscode/extension.ts - Updated FilterProvider with hierarchical filter tree
6. js/vscode/extension.ts - Added bundle, folder, definition kind toggles
7. graphql/repo/schema.graphql - Updated schema with new enums and fields

## Log

- 2026-01-29: Started implementation of semantic kind properties
- 2026-01-29: Added BundleKind (library, schema, binary, ui, site, assets), FolderKind (organization, required), updated DefinitionKind
- 2026-01-29: Updated streaming functions with dimension filters
- 2026-01-29: Updated VS Code filter tree with hierarchical structure

## Summary

Added semantic kind properties to Bundle (library, schema, binary, ui, site, assets), Folder (organization, required), and updated Definition kinds (implementation, interface, constant). Implemented filtering by bundle kind, folder kind, definition kind, time, contributor, policy, and breach-kind dimensions. Updated VS Code extension filter tree with hierarchical toggle structure.
