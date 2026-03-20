# Ticket

## Todos

- [x] Remove bundle: prefix in VSCode tree view
- [x] Sort codebase tree items (bundles by name)
- [x] Fix section/definition list CLI commands to use --file flag

## Changes

### CodebaseBundleItem
- Changed from `bundle.id` to `bundle.name` to display without "bundle:" prefix
- Updated tooltip to show `{name} ({root})`

### Codebase Tree Sorting
- Bundles are now sorted alphabetically by name

### CLI Command Syntax
- Changed `section list "path"` to `section list --file "path"`
- Changed `definition list "path"` to `definition list --file "path"`
- Fixed in: `getSectionListForFile()`, `loadFileContent()`, and command handlers

## Log

## Summary

Fixed three issues in VSCode extension codebase tree: 1) Removed bundle: prefix by using bundle.name instead of bundle.id. 2) Added alphabetical sorting for bundles. 3) Fixed section/definition list commands to use --file flag as required by the CLI.
