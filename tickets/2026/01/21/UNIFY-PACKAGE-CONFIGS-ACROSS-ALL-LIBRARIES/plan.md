# Plan: Unify Package Configs Across All Libraries

## Problem Statement

Workspace and package configs are outdated. Every library (Go module, Rust crate, Python package, JavaScript package, C# project) needs a package.json with unified naming conventions (e.g., rename "package" to "build" or "publish").

## Current State Analysis

### Existing package.json files:
1. `/package.json` - root workspace
2. `assets/package.json` - @semio/assets
3. `assets/logo/package.json` - @semio/logo
4. `assets/icons/package.json` - @semio/icons
5. `js/semio/package.json` - @semio/js
6. `js/docs/package.json` - @semio/docs
7. `js/play/package.json` - @semio/play
8. `js/desktop/package.json` - @semio/desktop
9. `js/vscode/package.json` - semio-repo (VSCode extension)
10. `py/semio/package.json` - @semio/py
11. `py/engine/package.json` - @semio/engine
12. `go/semio/package.json` - @semio/go
13. `go/repo/package.json` - @semio-repo/go
14. `net/Semio/package.json` - @semio/net
15. `net/Semio.Grasshopper/package.json` - @semio/grasshopper
16. `yak/package.json` - @semio/yak

### Missing package.json files:
1. `go/server/package.json` - needs to be created
2. `rs/semio/package.json` - needs to be created

### Issues Found:
1. `js/vscode/package.json` has `"package"` script - should be `"publish:vsix"` for clarity
2. Some packages lack `test` scripts
3. Some packages lack `build` scripts
4. `go/mcp` is listed in root workspaces but doesn't exist (needs removal)
5. `go/server` and `rs/semio` are not in root workspaces (needs addition)

## Changes to Implement

### 1. Create Missing package.json Files

#### go/server/package.json
- Name: `@semio-repo/server`
- Scripts: `dev`, `build`, `test`, `preflight`

#### rs/semio/package.json
- Name: `@semio/rs`
- Scripts: `build`, `test`, `preflight`

### 2. Update Existing package.json Files

#### js/vscode/package.json
- Rename `package` to `publish:vsix`

#### go/semio/package.json
- Add `build` script: `go build ./...`

#### go/repo/package.json
- Fix incomplete `build` script

#### py/semio/package.json
- Add `test`, `build`, `preflight` scripts

#### net/Semio/package.json
- Add `test` script

#### net/Semio.Grasshopper/package.json
- Add `test` script

### 3. Update Root package.json

- Remove `go/mcp` from workspaces (doesn't exist)
- Add `go/server` to workspaces
- Add `rs/semio` to workspaces

## Standard Script Naming Convention

| Script | Purpose |
|--------|---------|
| `dev` | Start development mode/watch |
| `build` | Build production artifacts |
| `test` | Run tests |
| `preflight` | Linting, type checking, formatting |
| `publish` | Publish to package registry |
| `publish:vsix` | Package VSCode extension |
