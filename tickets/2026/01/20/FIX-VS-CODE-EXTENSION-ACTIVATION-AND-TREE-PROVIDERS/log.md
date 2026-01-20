# Log - Fix VS Code Extension Activation and Tree Providers

-   Verified `engines.vscode` in `js/vscode/package.json` and updated to `^1.106.0`.
-   Fixed `js/vscode/extension.ts` registration order: moved `registerSidebarViews` to the top of `activate()`.
-   Improved `urql` fetch in `extension.ts` with robust bash escaping and `Response` mock.
-   Unified `loadCodebase` to use `fetchRepoViaGraphQL`.
-   Reduced VSIX size and fixed packaging by updating `.vscodeignore` and adding `semio.png`.
-   Rebuilt and reinstalled the extension using `code --install-extension`.
-   Verified GraphQL queries via `repo` CLI in the terminal.
