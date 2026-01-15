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
