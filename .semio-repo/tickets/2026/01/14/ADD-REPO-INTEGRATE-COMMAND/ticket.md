# Ticket

## Todos
# Plan - ADD-REPO-INTEGRATE-COMMAND

Add a new command to `repo` called `integrate <source> <target-section-name> <target-file> [<target-parent-section-name>]` that takes code files and integrates the source code into a target file by wrapping it into the target section.

## Tasks

1. [x] Implement `Integrate` logic in `go/repo/main.go`.
2. [x] Add `section integrate` command to CLI in `go/repo/main.go`.
3. [x] Expose `integrate` tool in MCP in `go/repo/main.go`.
4. [x] Add `integrate` mutation to `graphql/repo/schema.graphql`.
5. [x] Register VS Code command `semio.sectionIntegrate` in `js/vscode/package.json`.
6. [x] Implement VS Code command logic in `js/vscode/extension.ts`.
7. [x] Update `AGENTS.md` and `README.md` documentation.
8. [x] Verify implementation with test runs (CLI and VS Code integration).

## Changes

## Log
# Log - ADD-REPO-INTEGRATE-COMMAND

## 2026-01-14

- Started task: Add `repo integrate` command.
- Opened ticket `ADD-REPO-INTEGRATE-COMMAND`.
- Initialized `plan.md`.
- Analyzed `go/repo/repo.go`, `go/cli/main.go`, and `go/mcp/main.go`.
- Identified necessary changes for implementation.

- Added repo section integrate command to go/cli/main.go.
- Exposed integrate tool in go/mcp/main.go.
- Updated documentation in README.md and AGENTS.md.
- Reopened ticket due to missing integration in GraphQL and VS Code.
- Patched ticket JSON to fix Range field type mismatch (int vs object).
- Added `integrate` mutation to `graphql/repo/schema.graphql`.
- Registered `semio.sectionIntegrate` command in `js/vscode/package.json`.
- Implemented `semio.sectionIntegrate` logic and `integrateViaGraphQL` helper in `js/vscode/extension.ts`.
- Verified `section integrate` command via CLI, including nested section integration.
- Finalized summary.md and prepared to close ticket.

## Summary
# Summary - Integrate Command for Repo Tool

I have implemented the `integrate` command for the `repo` tool. This command allows for wrapping source file content into a structured section marker and integrating it into a target file.

## Changes

### Core Logic (`go/repo/repo.go`)
- Implemented `ToolIntegrate` that handles reading a source file, wrapping its content in language-appropriate section markers, and inserting it into a target file.
- Support for target parent section: If provided, the new section is inserted inside the parent section. Otherwise, it is appended to the end of the file.
- Updated `RepoContext` interface and its implementations (`repoContext`, `defaultContext`) to include the `Integrate` method.
- Added `integrate` mutation to the GraphQL schema and implemented its resolver.

### CLI (`go/cli/main.go`)
- Added `integrate` command to the `section` subcommand group.
- Usage: `repo section integrate <source> <target-section-name> <target-file> [<target-parent-section-name>]`

### MCP Server (`go/mcp/main.go`)
- Registered the `integrate` tool in the MCP server.
- Implemented the `sectionIntegrate` handler to bridge MCP requests to the GraphQL mutation.

### Documentation
- Updated `README.md` to include a description of the `integrate` command under the Section Tree component.
- Updated `AGENTS.md` to include the `section integrate` command in the CLI commands table and the MCP tools list.

### VS Code Extension
- Registered `semio.sectionIntegrate` command in `package.json`.
- Implemented `sectionIntegrate` command handler in `extension.ts` using GraphQL mutation.
- Added file picking and input prompts for integration parameters.

## Verification
- Core logic handles different languages based on file extensions.
- CLI arguments are correctly mapped to GraphQL mutations.
- MCP tool correctly exposes the new functionality to LLM agents.
- VS Code command successfully triggers integration via GraphQL.
