# Ticket

## Todos

- Fix VS Code extension manifest name so vsce packaging succeeds.
- Update devcontainer post-attach expectations and documentation for extension packaging rules.
- Record changes in ticket log and finalize summary.

## Changes

## Log

- Added TicketOpen input fields (noIssue, planPath) to repo GraphQL schema and resolver in ./semio-repo/cli/main.go to align with CLI payload.
- Updated graphql/repo/schema.graphql TicketOpenInput to match repo schema fields.
- Rebuilt ./semio-repo/cli/cli binary to apply schema changes.
- Opened ticket in repo root and removed mislocated ./semio-repo/cli/tickets folder via repo folder delete.
- Updated VS Code extension ticket commands to match current repo CLI ticket workflows.
- Updated .devcontainer/post-attach.sh to build the VS Code extension when the vsix is missing and then install it automatically.
- Updated README.md and AGENTS.md to document repo tooling schema source-of-truth, ticket input requirements, devcontainer extension install flow, and updated codebase entries.
- Closed ticket with updated files and summary.
- Received devcontainer post-attach failure: vsce packaging failed because package.json name is invalid (scoped name not allowed for VS Code extensions).
- Updated js/vscode/package.json name to a VS Code extension-compatible unscoped value for vsce packaging.
- Documented VS Code extension manifest name requirement in README.md and AGENTS.md, and added Codebase entry for js/vscode/package.json.
- Closed ticket after updating manifest name and documentation.

## Summary

Fixed VS Code extension packaging by switching the extension manifest name to an unscoped value, and documented the vsce naming requirement in README/AGENTS alongside the codebase entry for js/vscode/package.json.
