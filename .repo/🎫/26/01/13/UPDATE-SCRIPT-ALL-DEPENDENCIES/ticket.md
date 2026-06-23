# Ticket

## Todos

# Plan: Update Script for All Dependencies

## Overview

Create a comprehensive update script (`update.ts`) that updates all dependencies across the monorepo for all package managers while respecting pinned/excluded dependencies and preserving local package references.

## Package Managers Detected

1. **npm (package.json)**: Root + workspaces (js/compose, js/docs, js/play, js/desktop, js/vscode, etc.)
2. **uv (pyproject.toml)**: py/engine
3. **cargo (Cargo.toml)**: rs/compose
4. **go (go.mod)**: go/cli, go/mcp, ./repo/cli, go/compose
5. **C# (.csproj)**: net/Compose, net/Compose.Grasshopper, net/Compose.Tests, net/Compose.Grasshopper.Tests

## 💯Requirements

1. Update dependencies in manifest files (not just lock files)
2. Support excluding specific packages from updates (pinned dependencies)
3. Preserve local workspace references (e.g., `"compose/js": "*"`)
4. Preserve local Go module replace directives

## Implementation

### 1. Create Configuration File (`update.config.json`)

Define excluded dependencies per project:

```json
{
 "exclude": {
  "net/Compose.Grasshopper/Compose.Grasshopper.csproj": ["Grasshopper", "System.Drawing.Common", "System.Resources.Extensions"]
 },
 "preserveLocalVersions": {
  "npm": ["*"],
  "go": ["github.com/usalu/semio/*"]
 }
}
```

### 2. Create Update Script (`update.ts`)

Script structure:

- Parse configuration
- Update npm packages (root and workspaces)
- Update uv/Python packages
- Update cargo/Rust packages
- Update go modules
- Update .NET/C# packages
- Restore local package versions that were overwritten

### 3. Update Commands per Package Manager

| Manager | Command                               | Notes                           |
| ------- | ------------------------------------- | ------------------------------- |
| npm     | `npm update -S`                       | Updates package.json versions   |
| uv      | `uv lock --upgrade`                   | Then sync pyproject.toml        |
| cargo   | `cargo update`                        | Then update Cargo.toml versions |
| go      | `go get -u ./...` then `go mod tidy`  | Per module directory            |
| dotnet  | `dotnet outdated --upgrade` or manual | Respecting excludes             |

### 4. Post-Update Restoration

After running npm update:

- Scan all package.json files in workspaces
- Restore `"*"` versions for local packages like `compose/js`, `compose/assets`

## Files to Create/Modify

1. **Create**: `update.config.json` - Configuration for excluded deps
2. **Create**: `update.ts` - Main update script
3. **Modify**: `package.json` - Add update script reference

## Execution Flow

```
npm run update
  └─> npx tsx update.ts
        ├─> Load update.config.json
        ├─> npm update -S (root)
        ├─> Restore "*" versions in workspaces
        ├─> cd py/engine && uv lock --upgrade
        ├─> cd rs/compose && cargo update
        ├─> For each go.mod: go get -u && go mod tidy
        └─> For each .csproj: dotnet outdated --upgrade (skip excluded)
```

## Risk Mitigation

- Always run `git diff` after update to review changes
- Excluded packages are never touched
- Local package versions are restored after npm update
- Go replace directives are preserved (they're not modified by `go get`)

## Changes

## Log

# Log: Update Script for All Dependencies

## 2026-01-13

### Initial Analysis

Explored the monorepo structure and identified all package managers in use:

- **npm**: Root package.json with workspaces at js/compose, js/docs, js/play, js/desktop, js/vscode, assets/logo, assets/icons, assets, py/engine, net/Compose, net/Compose.Grasshopper, go/compose, ./repo/cli, go/mcp, yak
- **uv (Python)**: py/engine/pyproject.toml
- **cargo (Rust)**: rs/compose/Cargo.toml
- **go**: go/cli/go.mod, go/mcp/go.mod, ./repo/cli/go.mod, go/compose/go.mod
- **C# (.csproj)**: net/Compose/Compose.csproj, net/Compose.Grasshopper/Compose.Grasshopper.csproj, net/Compose.Tests/Compose.Tests.csproj, net/Compose.Grasshopper.Tests/Compose.Grasshopper.Tests.csproj

### Key Findings

1. Local npm packages use `"*"` for workspace dependencies (e.g., `"compose/js": "*"` in js/docs)
2. Compose.Grasshopper has pinned dependencies: Grasshopper, System.Drawing.Common, System.Resources.Extensions
3. Go modules use `replace` directives for local packages

### Implementation v1

Created initial version with basic support for all package managers.

### Issues Found After Testing v1

1. **Local compose/\* packages affected**: Hardcoded package list missed packages like `compose/logo`
2. **Cargo only updates lock file**: `cargo update` doesn't update Cargo.toml versions
3. **Compose.csproj exclusions missing**: FluentValidation and System.Collections.Immutable needed exclusion
4. **uv only updates lock file**: `uv lock --upgrade` doesn't update pyproject.toml

### Fixes Applied v2

1. **Dynamic workspace detection**: Now reads all workspace package.json files to detect names automatically
   - Detected 15 packages: compose/logo, compose/icons, compose/assets, compose/engine, compose/js, compose/net, compose/grasshopper, etc.

2. **Cargo.toml direct updates**: Added parser for Cargo.toml that fetches latest versions from crates.io API and updates the file directly

3. **Added Compose.csproj exclusions**: FluentValidation, System.Collections.Immutable added to exclude list

4. **pyproject.toml direct updates**: Added parser that fetches latest versions from PyPI API and updates the file directly

### Issues Found After Testing v2

- **Version conflicts**: Updating all packages to latest can cause dependency conflicts (e.g., starlette 0.51.0 conflicts with fastapi's requirement of starlette<0.51.0)

### Fixes Applied v3

1. **Rollback on failure**: Both Python and Rust updates now backup the original file and rollback if lock/update fails
   - Python: If `uv lock` fails due to incompatible versions, pyproject.toml is restored
   - Rust: If `cargo update` fails, Cargo.toml is restored

### Final Test Results

**NPM** - All 15 workspace packages detected:

```
  Detected 15 workspace packages: compose/logo, compose/icons, compose/assets, compose/engine, compose/js, compose/docs, compose/play, compose/desktop, repo/vscode, compose/net, compose/grasshopper, compose/go, repo/go, repo/mcp, compose/yak
  Will preserve local package versions:
    assets/icons/package.json: devDependencies.compose/logo = "*"
    net/Compose.Grasshopper/package.json: devDependencies.compose/net = "*"
    yak/package.json: devDependencies.compose/grasshopper = "*"
```

**Cargo.toml** - Successfully updated:

```diff
-serde = { version = "1.0", features = ["derive"] }
+serde = { version = "1.0.228", features = ["derive"] }
-thiserror = "1.0"
+thiserror = "2.0.17"
```

**pyproject.toml** - Updates attempted but rolled back due to conflicts:

```
    Lock failed! Rolling back pyproject.toml...
    Rolled back to original versions.
```

### Known Limitations

- Python updates may fail if packages have conflicting version constraints. In this case, manual intervention is needed or a smarter algorithm to resolve compatible versions.

## Summary

# Summary: Update Script for All Dependencies

Created a comprehensive dependency update system for the monorepo that handles all package managers (npm, uv, cargo, go, dotnet) with support for excluding specific packages, preserving local workspace references, and automatic rollback on failure.

## Files Created/Modified

- **update.config.json**: Configuration file defining excluded packages and local version preservation rules
- **update.ts**: Main update script handling all package managers
- **package.json**: Updated `update` script to use `npx tsx update.ts`

## Key Features

1. **Multi-package manager support**: npm, uv (Python), cargo (Rust), go, dotnet (C#)
2. **Dynamic workspace detection**: Automatically detects all 15 workspace packages
3. **Local version preservation**: Automatically restores `"*"` versions for all workspace packages after npm update
4. **Direct manifest updates**:
   - Updates pyproject.toml versions directly (not just lock file)
   - Updates Cargo.toml versions directly (not just lock file)
5. **Excluded packages**:
   - Compose.csproj: FluentValidation, System.Collections.Immutable
   - Compose.Grasshopper: Grasshopper, System.Drawing.Common, System.Resources.Extensions
6. **Rollback on failure**: If `uv lock` or `cargo update` fails, the manifest file is restored to original
7. **Dry-run mode**: Test what would be updated without making changes
8. **Target-specific updates**: Update only specific package managers

## Usage

```bash
# Update all dependencies
npm run update

# Dry run to preview changes
npm run update -- --dry-run

# Update specific package manager
npm run update npm
npm run update python
npm run update rust
npm run update go
npm run update dotnet
```

## How It Works

- **npm**: Runs `npm update -S`, then restores `"*"` versions for workspace packages
- **Python**: Fetches latest versions from PyPI API, updates pyproject.toml, runs `uv lock` (rollback on failure)
- **Rust**: Fetches latest versions from crates.io API, updates Cargo.toml, runs `cargo update` (rollback on failure)
- **Go**: Runs `go get -u ./...` and `go mod tidy`
- **dotnet**: Uses `dotnet list package --outdated` to find updates, then modifies .csproj files directly

## Known Limitations

- Python updates may fail if packages have conflicting version constraints (e.g., fastapi requires starlette<0.51.0 but latest starlette is 0.51.0). In such cases, the file is rolled back and manual intervention is needed.
