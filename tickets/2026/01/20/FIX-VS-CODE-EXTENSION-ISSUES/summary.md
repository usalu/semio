# Summary

The VS Code extension installation issues in the devcontainer were fixed by addressing both a script bug and an engine version mismatch.

## Changes

### 📦 VS Code Extension
- Updated `js/vscode/package.json` to require `vscode` version `^1.106.0` instead of `^1.108.0`.
- Updated `@types/vscode` to `^1.106.0` for compatibility with Windsurf.

### 🛠️ Devcontainer
- Refactored `.devcontainer/post-attach.sh` to include a robust IDE CLI detection mechanism (`find_working_cli`).
- The new script:
    - Checks for `windsurf`, `cursor`, `code-insiders`, and `code`.
    - Verifies that the detected binary actually works by running `--version`.
    - Searches directly in known remote-cli locations for Windsurf and VS Code to bypass broken PATH wrappers.
    - Added strict error checking and exit codes for the installation process.
    - Added logic to rebuild the VSIX package if it is missing or older than the source code.

## Verification Results
- **Installation**: Successfully verified by running `.devcontainer/post-attach.sh` manually. It detected the VS Code remote CLI and installed the extension without errors.
- **List Extensions**: `code --list-extensions` confirms `usalu.semio-repo` is installed.
- **Unit Tests**: Ran `npm run test` in `js/vscode`; all 22 tests passed, confirming functionality across commands, diagnostics, and views.
