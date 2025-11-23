# Semio VS Code Extension

Validation and linting for Semio kit JSON files.

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

### GUID Uniqueness (`guid-unique`)

**Severity:** Error

Ensures all entity GUIDs are unique across the entire kit.

**Quick Fix:** Regenerates a new GUID and updates all references.

### Design Sibling Name Uniqueness (`design-sibling-name-unique`)

**Severity:** Error

Ensures designs with the same parent have unique names.

**Quick Fix:** Renames the design with a unique suffix (e.g., "Wall 2", "Wall 3").

### Piece Name Uniqueness (`piece-name-unique-in-design`)

**Severity:** Error

Ensures pieces within a design have unique names.

**Quick Fix:** Renames the piece with a unique suffix.

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
npm run build
```

Or use the **"dev vscode"** launch configuration from the Run and Debug panel in VS Code (F5).

## Testing

The extension validates any JSON file matching:

- `kit_*.json`
- `*_kit.json`  
- `kit.json`

Create test files with intentional errors to verify validation works.

## Future Enhancements

- More validation rules (port references, type references, etc.)
- Batch fixes (fix all issues at once)
- Performance optimizations (incremental validation)
- Configuration options (enable/disable specific rules)
- Custom rule plugins

