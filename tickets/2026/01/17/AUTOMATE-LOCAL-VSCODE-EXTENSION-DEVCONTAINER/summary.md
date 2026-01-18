# Summary

Automated the installation of the local semio VSCode extension in devcontainers.

## Changes

- Added `@vscode/vsce` dependency and `package` script to `js/vscode/package.json`
- Updated `.devcontainer/post-create.sh` to build, package, and install the extension
- Added `*.vsix` to `.gitignore`

## Result

When a new devcontainer is created, the semio VSCode extension will be automatically built and installed, making it immediately available for development.
