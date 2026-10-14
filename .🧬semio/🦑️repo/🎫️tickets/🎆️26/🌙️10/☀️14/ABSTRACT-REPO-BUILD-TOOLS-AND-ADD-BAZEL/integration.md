# Build Tool Integration

The repository build contract owns project identity, task selection, execution recipes, dependencies, named inputs, configuration, progress and cancellation. Nx remains the configured main tool; Bazel is explicitly selectable and inherits through nested orchestration. The shared checkout is preserved without modifying Git commands or worktrees.

## Coordination

Read-only exploration used two GPT 6.1 Sol low agents (the available setting corresponding to Light). Implementation uses three GPT 6.1 Sol high agents at the maximum concurrent fleet capacity: build core, Bazel backend, dashboard and launch projection. The main chat coordinates integration.

## Test-Driven Surface Changes

The portable fixture and owned schema were written before implementation. The first run failed three tests for package commands, workspace context, and root/editor/CI Nx coupling. After migration these three pass. The authored project schema test failed on the existing Nx executor and passes after neutral executor/schema projection. Latest targeted run: 4 passing tests, 21 assertions, zero failures; Bun 1.3.14, Ajv 8 oracle, TypeScript JSONC parsing. Full backend and runtime qualification remains in progress.

## Integration Scope

Package scripts and direct build command callers use bun repo. Generic execution environment publishes SEMIO_BUILD_* policy and SEMIO_WORKSPACE_ROOT; backend adapters translate their own environment. Generator APIs use generatorBuildCommand and corresponding preview/check names. Authored project manifests refer to the repository schema and repo:script/repo:noop execution kinds. Setup provisions both pinned tools. Agent instruction aliases remain unedited because they link to AGENTS.md; neutral command discovery is supplied through repository metadata, MCP consumers, generated editor commands and README.
