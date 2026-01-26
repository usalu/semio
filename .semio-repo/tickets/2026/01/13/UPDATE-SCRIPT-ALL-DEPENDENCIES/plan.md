# Plan: Update Script for All Dependencies

## Overview

Create a comprehensive update script (`update.ts`) that updates all dependencies across the monorepo for all package managers while respecting pinned/excluded dependencies and preserving local package references.

## Package Managers Detected

1. **npm (package.json)**: Root + workspaces (js/semio, js/docs, js/play, js/desktop, js/vscode, etc.)
2. **uv (pyproject.toml)**: py/engine
3. **cargo (Cargo.toml)**: rs/semio
4. **go (go.mod)**: go/cli, go/mcp, go/repo, go/semio
5. **C# (.csproj)**: net/Semio, net/Semio.Grasshopper, net/Semio.Tests, net/Semio.Grasshopper.Tests

## Requirements

1. Update dependencies in manifest files (not just lock files)
2. Support excluding specific packages from updates (pinned dependencies)
3. Preserve local workspace references (e.g., `"@semio/js": "*"`)
4. Preserve local Go module replace directives

## Implementation

### 1. Create Configuration File (`update.config.json`)

Define excluded dependencies per project:

```json
{
  "exclude": {
    "net/Semio.Grasshopper/Semio.Grasshopper.csproj": [
      "Grasshopper",
      "System.Drawing.Common",
      "System.Resources.Extensions"
    ]
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

| Manager | Command | Notes |
|---------|---------|-------|
| npm | `npm update -S` | Updates package.json versions |
| uv | `uv lock --upgrade` | Then sync pyproject.toml |
| cargo | `cargo update` | Then update Cargo.toml versions |
| go | `go get -u ./...` then `go mod tidy` | Per module directory |
| dotnet | `dotnet outdated --upgrade` or manual | Respecting excludes |

### 4. Post-Update Restoration

After running npm update:
- Scan all package.json files in workspaces
- Restore `"*"` versions for local packages like `@semio/js`, `@semio/assets`

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
        ├─> cd rs/semio && cargo update
        ├─> For each go.mod: go get -u && go mod tidy
        └─> For each .csproj: dotnet outdated --upgrade (skip excluded)
```

## Risk Mitigation

- Always run `git diff` after update to review changes
- Excluded packages are never touched
- Local package versions are restored after npm update
- Go replace directives are preserved (they're not modified by `go get`)
