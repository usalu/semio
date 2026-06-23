# Ticket

## Todos

# Plan: Update VSCode Launch Configs

## Objective

Update VSCode launch.json and tasks.json with:

1. New naming convention: `@package/name script`
2. Reorder: specific packages first, general commands last
3. Lifecycle ordering: dev -> test -> build -> publish:test -> publish
4. Add missing test scripts: test, test:unit, test:e2e, test:coverage

## Package List

| Package               | Name              |
| --------------------- | ----------------- |
| go/compose              | compose/go          |
| ./repo/cli            | repo/go           |
| go/server             | repo/server       |
| rs/compose              | compose/rs          |
| py/compose              | compose/py          |
| py/engine             | compose/engine      |
| js/compose              | compose/js          |
| js/docs               | compose/docs        |
| js/play               | compose/play        |
| js/desktop            | compose/desktop     |
| js/vscode             | repo/vscode       |
| net/Compose             | compose/net         |
| net/Compose.Grasshopper | compose/grasshopper |
| assets                | compose/assets      |
| assets/logo           | compose/logo        |
| assets/icons          | compose/icons       |
| yak                   | compose/yak         |

## New Naming Convention

Format: `@scope/name script`

Examples:

- `repo/go build`
- `repo/vscode dev`
- `compose/js dev:storybook`
- `compose/js test:coverage`

## Ordering

1. **Package-specific** (grouped by package, ordered by lifecycle)
2. **Root/general commands** (at the end)

## Cleanup

- Remove all mcp references (go/mcp doesn't exist)
- Add go/server and rs/compose configurations

## Changes

## Log

# Log: Update VSCode Launch Configs

## Analysis

Analyzed existing launch.json (465 lines) and tasks.json (1153 lines) to understand current configuration patterns.

### Issues Found

- Old naming convention: `dev js`, `build go repo`, etc.
- No unified ordering
- References to non-existent `go/mcp`
- Missing configurations for `go/server` and `rs/compose`
- Missing test:unit, test:e2e, test:coverage for packages that support them

## Implementation

### New Naming Convention

Changed from `verb subject` to `@scope/package verb`:

- `dev js js storybook` → `compose/js dev:storybook`
- `build go repo` → `repo/go build`
- `dev vscode` → `repo/vscode dev`

### New Ordering

1. Package-specific commands (grouped by package)
2. Within each package: dev → test → test:unit → test:e2e → test:coverage → build → preflight → publish
3. Root commands at the end

### Packages Configured

- repo/go (dev, test, build, preflight)
- repo/server (dev, test, build, preflight) - NEW
- repo/vscode (dev, test, build, preflight, publish:vsix)
- compose/go (test, build, preflight)
- compose/rs (test, build, preflight) - NEW
- compose/py (test, build, preflight)
- compose/engine (dev, test, build, preflight)
- compose/js (dev, dev:storybook, dev:sketchpad, test, test:unit, test:e2e, test:coverage, build, preflight)
- compose/docs (dev, build, preflight)
- compose/play (dev, build, preflight)
- compose/desktop (dev, build, preflight)
- compose/net (test, build, preflight)
- compose/grasshopper (test, build, preflight)
- compose/assets (build, preflight)
- compose/logo (dev, build)
- compose/icons (build)
- compose/yak (build)

### Removed

- All `mcp` references (go/mcp doesn't exist)
- Old individual per-action tasks (analyze X, fix X, etc.)

### Root Commands

- dev, test, build, publish:test, publish
- analyze, fix, preflight, update
- benchmark, mcp inspector

## Summary

# Summary: Update VSCode Launch Configs

Rewrote VSCode launch.json and tasks.json with unified naming convention and lifecycle ordering.

## Changes Made

### New Naming Convention

Format: `@scope/package script`

- `dev js js storybook` → `compose/js dev:storybook`
- `build go repo` → `repo/go build`
- `test vscode` → `repo/vscode test`

### Ordering

1. Package-specific commands (grouped by package, lifecycle within)
2. Generic language debuggers (Python, Go)
3. Root commands at the end

### Lifecycle Order

dev → test → test:unit → test:e2e → test:coverage → build → preflight → publish

### New Packages Added

- repo/server (go/server)
- compose/rs (rs/compose)

### Removed

- All mcp references (go/mcp doesn't exist)
- Redundant per-action tasks (old analyze/fix/preflight/test/build/publish per package)

### Files Updated

- .vscode/launch.json (rewritten, ~550 lines)
- .vscode/tasks.json (rewritten, ~710 lines)
