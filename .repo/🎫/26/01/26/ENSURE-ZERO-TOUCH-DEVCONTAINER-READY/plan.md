# Plan - Ensure Zero Touch Devcontainer Ready

- [x] Fix permission issues in devcontainer lifecycle scripts by enforcing `vscode:vscode` ownership on `$WORKSPACE`.
- [x] Wrap `sql-wasm.wasm` copy in `postinstall` script to prevent `npm install` failure.
- [x] Ensure `postAttachCommand` successfully builds and installs the VS Code extension.
- [ ] Verify that Nx module resolution works for `@semio/play` and other apps.
- [ ] Verify that all repositories (semio and metabolism) are in a ready state.
- [ ] Finalize by ensuring the devcontainer setup is robust and automated.
