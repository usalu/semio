# Ticket

## Todos

# Plan: Unify Package Configs Across All Libraries

## Problem Statement

Workspace and package configs are outdated. Every library (Go module, Rust crate, Python package, JavaScript package, C# project) needs a package.json with unified naming conventions (e.g., rename "package" to "build" or "publish").

## Current State Analysis

### Existing package.json files:

1. `/package.json` - root workspace
2. `assets/package.json` - compose/assets
3. `assets/logo/package.json` - compose/logo
4. `assets/icons/package.json` - compose/icons
5. `js/compose/package.json` - compose/js
6. `js/docs/package.json` - compose/docs
7. `js/play/package.json` - compose/play
8. `js/desktop/package.json` - compose/desktop
9. `js/vscode/package.json` - repo (VSCode extension)
10. `py/compose/package.json` - compose/py
11. `py/engine/package.json` - compose/engine
12. `go/compose/package.json` - compose/go
13. `./repo/cli/package.json` - repo/go
14. `net/Compose/package.json` - compose/net
15. `net/Compose.Grasshopper/package.json` - compose/grasshopper
16. `yak/package.json` - compose/yak

### Missing package.json files:

1. `go/server/package.json` - needs to be created
2. `rs/compose/package.json` - needs to be created

### Issues Found:

1. `js/vscode/package.json` has `"package"` script - should be `"publish:vsix"` for clarity
2. Some packages lack `test` scripts
3. Some packages lack `build` scripts
4. `go/mcp` is listed in root workspaces but doesn't exist (needs removal)
5. `go/server` and `rs/compose` are not in root workspaces (needs addition)

## Changes to Implement

### 1. Create Missing package.json Files

#### go/server/package.json

- Name: `repo/server`
- Scripts: `dev`, `build`, `test`, `preflight`

#### rs/compose/package.json

- Name: `compose/rs`
- Scripts: `build`, `test`, `preflight`

### 2. Update Existing package.json Files

#### js/vscode/package.json

- Rename `package` to `publish:vsix`

#### go/compose/package.json

- Add `build` script: `go build ./...`

#### ./repo/cli/package.json

- Fix incomplete `build` script

#### py/compose/package.json

- Add `test`, `build`, `preflight` scripts

#### net/Compose/package.json

- Add `test` script

#### net/Compose.Grasshopper/package.json

- Add `test` script

### 3. Update Root package.json

- Remove `go/mcp` from workspaces (doesn't exist)
- Add `go/server` to workspaces
- Add `rs/compose` to workspaces

## Standard Script Naming Convention

| Script         | Purpose                            |
| -------------- | ---------------------------------- |
| `dev`          | Start development mode/watch       |
| `build`        | Build production artifacts         |
| `test`         | Run tests                          |
| `preflight`    | Linting, type checking, formatting |
| `publish`      | Publish to package registry        |
| `publish:vsix` | Package VSCode extension           |

## Changes

## Log

# Log: Unify Package Configs Across All Libraries

## Analysis Phase

Analyzed existing package.json files across the monorepo:

### Existing package.json files found:

- `/package.json` - root workspace
- `assets/package.json` - compose/assets
- `assets/logo/package.json` - compose/logo
- `assets/icons/package.json` - compose/icons
- `js/compose/package.json` - compose/js
- `js/docs/package.json` - compose/docs
- `js/play/package.json` - compose/play
- `js/desktop/package.json` - compose/desktop
- `js/vscode/package.json` - repo (VSCode extension)
- `py/compose/package.json` - compose/py
- `py/engine/package.json` - compose/engine
- `go/compose/package.json` - compose/go
- `./repo/cli/package.json` - repo/go
- `net/Compose/package.json` - compose/net
- `net/Compose.Grasshopper/package.json` - compose/grasshopper
- `yak/package.json` - compose/yak

### Missing package.json files:

- `go/server/package.json` - created
- `rs/compose/package.json` - created

### Issues found and fixed:

1. `go/mcp` was in workspaces but doesn't exist - removed
2. `js/vscode/package.json` had "package" script - renamed to "publish:vsix"
3. `./repo/cli/package.json` had incomplete build script - fixed
4. `go/compose/package.json` was missing build script - added
5. `py/compose/package.json` was missing all scripts - added build, test, preflight
6. `.NET` packages were missing test scripts - added

## Implementation

### Created files:

1. `go/server/package.json` - with dev, build, test, preflight scripts
2. `rs/compose/package.json` - with build, test, preflight scripts

### Updated files:

1. `./repo/cli/package.json` - fixed build script, added dev script
2. `go/compose/package.json` - added build script
3. `py/compose/package.json` - added build, test, preflight scripts, changed projectType to library
4. `net/Compose/package.json` - added test script, updated preflight
5. `net/Compose.Grasshopper/package.json` - added test script, updated preflight
6. `js/vscode/package.json` - renamed "package" to "publish:vsix"
7. `package.json` (root) - updated workspaces list

## Unified Script Naming Convention

| Script         | Purpose                            |
| -------------- | ---------------------------------- |
| `dev`          | Start development mode/watch       |
| `build`        | Build production artifacts         |
| `test`         | Run tests                          |
| `preflight`    | Linting, type checking, formatting |
| `publish`      | Publish to package registry        |
| `publish:vsix` | Package VSCode extension           |

## Summary

# Summary: Unify Package Configs Across All Libraries

Unified package.json configurations across all libraries in the monorepo (Go, Rust, Python, JavaScript, C#).

## Changes Made

### New Files Created

- `go/server/package.json` - repo/server with dev, build, test, preflight scripts
- `rs/compose/package.json` - compose/rs with build, test, preflight scripts

### Files Updated

- `package.json` (root) - Updated workspaces: removed non-existent `go/mcp`, added `go/server`, `rs/compose`, `py/compose`
- `./repo/cli/package.json` - Fixed incomplete build script, added dev script
- `go/compose/package.json` - Added build script
- `py/compose/package.json` - Added build, test, preflight scripts
- `net/Compose/package.json` - Added test script, updated preflight to use dotnet build
- `net/Compose.Grasshopper/package.json` - Added test script, updated preflight
- `js/vscode/package.json` - Renamed "package" script to "publish:vsix"

## Script Naming Convention

All libraries now follow consistent script naming:

- `dev` - Development mode/watch
- `build` - Production build
- `test` - Run tests
- `preflight` - Linting, type checking, formatting
- `publish` - Publish to registry
- `publish:vsix` - Package VSCode extension
