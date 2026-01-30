# Ticket

## Todos

## Changes

## Log

## Summary
# Fix VS Code Extension Packaging and Runtime

## Summary

The `semio-repo` VS Code extension was broken due to:
1.  **Schema Mismatch**: The TypeScript extension code queried `autofix` on `Violation` objects, but the Go backend schema only exposed `autofixable`. This caused code generation and runtime errors.
2.  **Packaging Failures**: The `vsce package` process failed because of bundling issues with `dependencies` vs `devDependencies` and missing exclusions in `.vscodeignore`.
3.  **Runtime UI Failure**: The extension installed but showed no icon and no views. This was caused by the `package.json` referencing `../../assets/icons/semio_codeicon.svg` (outside the extension root), which is invalid for VSIX packages. The View Container failed to register, hiding all associated views.

## Changes

### 1. Schema & Code Refactoring
- **`js/vscode/extension.ts`**:
    - Updated `AnalyzeDocument` GraphQL query to remove the invalid `autofix` field.
    - Refactored `createRepoCodeAction` to check `violation.autofixable` and invoke the `semio.fixViolation` command instead of applying edits directly ( aligning with the CLI-driven architecture).
    - Verified `fixViolation` implementation calls `repo fix`.

### 2. Packaging Pipeline
- **`js/vscode/package.json`**:
    - Moved all runtime `dependencies` (except `jsonc-parser` which is bundled) to `devDependencies` to support `vsce package --no-dependencies`.
    - Updated `vscode:prepublish` script to run the full build pipeline.
- **`js/vscode/vite.config.ts`**:
    - configured to bundle `jsonc-parser`.
- **`js/vscode/.vscodeignore`**:
    - Added exclusions for `node_modules`, `src`, `codegen`, and config files to ensure a lean VSIX.

### 3. Runtime & Assets
- **Assets**:
    - Copied `assets/icons/semio_codeicon.svg` to `js/vscode/semio_codeicon.svg` for the View Container icon.
    - Copied `assets/icons/semio_512x512.png` to `js/vscode/semio_icon.png` for the Extension Marketplace icon.
- **Manifest (`package.json`)**:
    - Updated `viewsContainers` to reference the local `semio_codeicon.svg`.
    - Added `"icon": "semio_icon.png"` to the extension metadata.
    - Verified activation events are correct (`onStartupFinished`).

## Verification
- Ran `npm run package` successfully, producing a valid `semio-repo.vsix`.
- Verified the VSIX contains the necessary icons and the bundled `extension.js`.
