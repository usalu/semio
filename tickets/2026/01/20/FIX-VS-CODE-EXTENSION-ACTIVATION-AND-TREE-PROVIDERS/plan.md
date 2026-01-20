# Plan - Fix VS Code Extension Activation and Tree Providers

1.  **Diagnose activation issues**:
    *   Verify `engines.vscode` version matches the container's VS Code.
    *   Ensure TreeView providers are registered as early as possible in `activate()`.
    *   Add comprehensive logging to the "semio" output channel.
2.  **Fix binary path resolution**:
    *   Update `getRepoBinaryPath()` to correctly find the binary in `/workspaces/semio/go/repo/repo`.
3.  **Improve GraphQL fetching**:
    *   Refactor URQL `fetch` to use robust shell escaping for Linux.
    *   Return a `Response`-compatible object to handle environments without the global Web API.
    *   Unify all GraphQL loads (including `loadCodebase`) to use the improved URQL-based fetch.
4.  **Fix VSIX packaging**:
    *   Include `semio.png` and `LICENSE.md` to satisfy `vsce` requirements.
    *   Optimize `.vscodeignore` to exclude irrelevant folders and reduce VSIX size.
5.  **Validate**:
    *   Verify binary execution and GraphQL query results in the terminal.
    *   Rebuild and reinstall the extension.
