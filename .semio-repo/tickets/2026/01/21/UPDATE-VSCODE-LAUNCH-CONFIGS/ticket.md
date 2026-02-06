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
| go/semio              | semio/go          |
| ./semio-repo/cli      | semio-repo/go     |
| go/server             | semio-repo/server |
| rs/semio              | semio/rs          |
| py/semio              | semio/py          |
| py/engine             | semio/engine      |
| js/semio              | semio/js          |
| js/docs               | semio/docs        |
| js/play               | semio/play        |
| js/desktop            | semio/desktop     |
| js/vscode             | semio-repo/vscode |
| net/Semio             | semio/net         |
| net/Semio.Grasshopper | semio/grasshopper |
| assets                | semio/assets      |
| assets/logo           | semio/logo        |
| assets/icons          | semio/icons       |
| yak                   | semio/yak         |

## New Naming Convention

Format: `@scope/name script`

Examples:

- `semio-repo/go build`
- `semio-repo/vscode dev`
- `semio/js dev:storybook`
- `semio/js test:coverage`

## Ordering

1. **Package-specific** (grouped by package, ordered by lifecycle)
2. **Root/general commands** (at the end)

## Cleanup

- Remove all mcp references (go/mcp doesn't exist)
- Add go/server and rs/semio configurations

## Changes

## Log

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

- `dev js js storybook` → `semio/js dev:storybook`
- `build go repo` → `semio-repo/go build`
- `dev vscode` → `semio-repo/vscode dev`

### New Ordering

1. Package-specific commands (grouped by package)
2. Within each package: dev → test → test:unit → test:e2e → test:coverage → build → preflight → publish
3. Root commands at the end

### Packages Configured

- semio-repo/go (dev, test, build, preflight)
- semio-repo/server (dev, test, build, preflight) - NEW
- semio-repo/vscode (dev, test, build, preflight, publish:vsix)
- semio/go (test, build, preflight)
- semio/rs (test, build, preflight) - NEW
- semio/py (test, build, preflight)
- semio/engine (dev, test, build, preflight)
- semio/js (dev, dev:storybook, dev:sketchpad, test, test:unit, test:e2e, test:coverage, build, preflight)
- semio/docs (dev, build, preflight)
- semio/play (dev, build, preflight)
- semio/desktop (dev, build, preflight)
- semio/net (test, build, preflight)
- semio/grasshopper (test, build, preflight)
- semio/assets (build, preflight)
- semio/logo (dev, build)
- semio/icons (build)
- semio/yak (build)

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

- `dev js js storybook` → `semio/js dev:storybook`
- `build go repo` → `semio-repo/go build`
- `test vscode` → `semio-repo/vscode test`

### Ordering

1. Package-specific commands (grouped by package, lifecycle within)
2. Generic language debuggers (Python, Go)
3. Root commands at the end

### Lifecycle Order

dev → test → test:unit → test:e2e → test:coverage → build → preflight → publish

### New Packages Added

- semio-repo/server (go/server)
- semio/rs (rs/semio)

### Removed

- All mcp references (go/mcp doesn't exist)
- Redundant per-action tasks (old analyze/fix/preflight/test/build/publish per package)

### Files Updated

- .vscode/launch.json (rewritten, ~550 lines)
- .vscode/tasks.json (rewritten, ~710 lines)
