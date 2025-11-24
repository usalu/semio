# Semio VS Code Extension

Validation and linting for Semio kit JSON files.

> **📖 For complete validation documentation, see [`VALIDATION.md`](../../VALIDATION.md) in the repository root.**

## Features

### Automatic Validation

The extension automatically validates Semio kit JSON files (files named `kit_*.json`, `*_kit.json`, or `kit.json`) and shows:

- **Errors** for critical issues (duplicate GUIDs, duplicate names, etc.)
- **Warnings** for non-critical issues

### Quick Fixes

Every validation issue comes with one or more **Quick Fix** options that:

1. Apply a `KitDiff` to fix the issue
2. Re-serialize the entire kit
3. Replace the document with the fixed JSON

All fixes are **diff-based**, meaning they use the same `KitDiff` system as the core Semio application, ensuring consistency across all platforms.

## Validation Rules

The extension implements **11 validation rules**:

1. **GUID Uniqueness** - All GUIDs must be unique
2. **Type Name Uniqueness** - Sibling types must have unique names
3. **Design Name Uniqueness** - Sibling designs must have unique names
4. **Piece Name Uniqueness** - Pieces in a design must have unique names
5. **Quality Name Uniqueness** - All qualities must have unique names
6. **Interface Name Uniqueness** - All interfaces must have unique names
7. **File Name Uniqueness** - All files must have unique names
8. **Folder Name Uniqueness** - Sibling folders must have unique names
9. **Port Name Uniqueness** - Ports in a type must have unique names
10. **Model Name Uniqueness** - Models in a type must have unique names
11. **Layer Path Uniqueness** - Layers in a design must have unique paths

See [`VALIDATION.md`](../../VALIDATION.md) for detailed rule descriptions and scope information.

## Architecture

This extension is a **minimal JSON linter** that:

1. Parses JSON → `Kit` using `deserializeKit` from `@semio/js`
2. Runs `validateSemioKit` (pure domain logic)
3. Maps domain locations → JSON ranges using `jsonc-parser`
4. Applies fixes via `applyKitDiff` + full document replacement

**Key principle:** All validation logic lives in `@semio/js/semio.ts` as pure domain functions. This extension is just a thin JSON-aware wrapper.

## Usage

1. Open a Semio kit JSON file
2. Validation happens automatically
3. Hover over errors/warnings to see details
4. Click the lightbulb 💡 or press `Ctrl+.` to apply Quick Fixes

## Development

```bash
npm install
npm run compile
```

Press `F5` in VS Code to launch an Extension Development Host window with the extension loaded.

## Testing

The extension validates any JSON file matching:

- `kit_*.json`
- `*_kit.json`
- `kit.json`

Create test files with intentional errors to verify validation works.

Example test cases:

- Duplicate GUIDs
- Duplicate names (types, designs, pieces, etc.)
- Invalid references

## Files

- `src/extension.ts` - Main extension logic
- `package.json` - Extension manifest and dependencies

## Dependencies

- `@semio/js` - Core Semio domain logic (validation, diffs, serialization)
- `jsonc-parser` - JSON parsing with comment support
- `vscode` - VS Code extension API

## Related Documentation

- [**VALIDATION.md**](../../VALIDATION.md) - Complete validation system documentation
- [**AGENTS.md**](../../AGENTS.md#validation) - Validation specs and architecture
- [`agents/2025-11-23_validation-system.md`](../../agents/2025-11-23_validation-system.md) - Implementation details
