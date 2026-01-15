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
