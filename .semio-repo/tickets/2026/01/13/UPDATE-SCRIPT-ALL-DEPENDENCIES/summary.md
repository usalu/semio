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
   - Semio.csproj: FluentValidation, System.Collections.Immutable
   - Semio.Grasshopper: Grasshopper, System.Drawing.Common, System.Resources.Extensions
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
