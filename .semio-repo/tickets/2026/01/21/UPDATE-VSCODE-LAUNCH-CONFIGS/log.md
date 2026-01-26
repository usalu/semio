# Log: Update VSCode Launch Configs

## Analysis

Analyzed existing launch.json (465 lines) and tasks.json (1153 lines) to understand current configuration patterns.

### Issues Found
- Old naming convention: `dev js`, `build go repo`, etc.
- No unified ordering
- References to non-existent `go/mcp`
- Missing configurations for `go/server` and `rs/semio`
- Missing test:unit, test:e2e, test:coverage for packages that support them

## Implementation

### New Naming Convention
Changed from `verb subject` to `@scope/package verb`:
- `dev js js storybook` → `@semio/js dev:storybook`
- `build go repo` → `@semio-repo/go build`
- `dev vscode` → `@semio-repo/vscode dev`

### New Ordering
1. Package-specific commands (grouped by package)
2. Within each package: dev → test → test:unit → test:e2e → test:coverage → build → preflight → publish
3. Root commands at the end

### Packages Configured
- @semio-repo/go (dev, test, build, preflight)
- @semio-repo/server (dev, test, build, preflight) - NEW
- @semio-repo/vscode (dev, test, build, preflight, publish:vsix)
- @semio/go (test, build, preflight)
- @semio/rs (test, build, preflight) - NEW
- @semio/py (test, build, preflight)
- @semio/engine (dev, test, build, preflight)
- @semio/js (dev, dev:storybook, dev:sketchpad, test, test:unit, test:e2e, test:coverage, build, preflight)
- @semio/docs (dev, build, preflight)
- @semio/play (dev, build, preflight)
- @semio/desktop (dev, build, preflight)
- @semio/net (test, build, preflight)
- @semio/grasshopper (test, build, preflight)
- @semio/assets (build, preflight)
- @semio/logo (dev, build)
- @semio/icons (build)
- @semio/yak (build)

### Removed
- All `mcp` references (go/mcp doesn't exist)
- Old individual per-action tasks (analyze X, fix X, etc.)

### Root Commands
- dev, test, build, publish:test, publish
- analyze, fix, preflight, update
- benchmark, mcp inspector
