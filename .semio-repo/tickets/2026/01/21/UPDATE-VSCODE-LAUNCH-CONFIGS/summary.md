# Summary: Update VSCode Launch Configs

Rewrote VSCode launch.json and tasks.json with unified naming convention and lifecycle ordering.

## Changes Made

### New Naming Convention
Format: `@scope/package script`
- `dev js js storybook` → `@semio/js dev:storybook`
- `build go repo` → `@semio-repo/go build`
- `test vscode` → `@semio-repo/vscode test`

### Ordering
1. Package-specific commands (grouped by package, lifecycle within)
2. Generic language debuggers (Python, Go)
3. Root commands at the end

### Lifecycle Order
dev → test → test:unit → test:e2e → test:coverage → build → preflight → publish

### New Packages Added
- @semio-repo/server (go/server)
- @semio/rs (rs/semio)

### Removed
- All mcp references (go/mcp doesn't exist)
- Redundant per-action tasks (old analyze/fix/preflight/test/build/publish per package)

### Files Updated
- .vscode/launch.json (rewritten, ~550 lines)
- .vscode/tasks.json (rewritten, ~710 lines)
