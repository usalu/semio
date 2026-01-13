# Log: Update Script for All Dependencies

## 2026-01-13

### Initial Analysis

Explored the monorepo structure and identified all package managers in use:

- **npm**: Root package.json with workspaces at js/semio, js/docs, js/play, js/desktop, js/vscode, assets/logo, assets/icons, assets, py/engine, net/Semio, net/Semio.Grasshopper, go/semio, go/repo, go/mcp, yak
- **uv (Python)**: py/engine/pyproject.toml
- **cargo (Rust)**: rs/semio/Cargo.toml
- **go**: go/cli/go.mod, go/mcp/go.mod, go/repo/go.mod, go/semio/go.mod
- **C# (.csproj)**: net/Semio/Semio.csproj, net/Semio.Grasshopper/Semio.Grasshopper.csproj, net/Semio.Tests/Semio.Tests.csproj, net/Semio.Grasshopper.Tests/Semio.Grasshopper.Tests.csproj

### Key Findings

1. Local npm packages use `"*"` for workspace dependencies (e.g., `"@semio/js": "*"` in js/docs)
2. Semio.Grasshopper has pinned dependencies: Grasshopper, System.Drawing.Common, System.Resources.Extensions
3. Go modules use `replace` directives for local packages

### Implementation v1

Created initial version with basic support for all package managers.

### Issues Found After Testing v1

1. **Local @semio/* packages affected**: Hardcoded package list missed packages like `@semio/logo`
2. **Cargo only updates lock file**: `cargo update` doesn't update Cargo.toml versions
3. **Semio.csproj exclusions missing**: FluentValidation and System.Collections.Immutable needed exclusion
4. **uv only updates lock file**: `uv lock --upgrade` doesn't update pyproject.toml

### Fixes Applied v2

1. **Dynamic workspace detection**: Now reads all workspace package.json files to detect names automatically
   - Detected 15 packages: @semio/logo, @semio/icons, @semio/assets, @semio/engine, @semio/js, @semio/net, @semio/grasshopper, etc.

2. **Cargo.toml direct updates**: Added parser for Cargo.toml that fetches latest versions from crates.io API and updates the file directly

3. **Added Semio.csproj exclusions**: FluentValidation, System.Collections.Immutable added to exclude list

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
  Detected 15 workspace packages: @semio/logo, @semio/icons, @semio/assets, @semio/engine, @semio/js, @semio/docs, @semio/play, @semio/desktop, @semio/vscode, @semio/net, @semio/grasshopper, @semio/go, @semio-repo/go, @semio-repo/mcp, @semio/yak
  Will preserve local package versions:
    assets/icons/package.json: devDependencies.@semio/logo = "*"
    net/Semio.Grasshopper/package.json: devDependencies.@semio/net = "*"
    yak/package.json: devDependencies.@semio/grasshopper = "*"
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
