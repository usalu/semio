# Summary: Unify Package Configs Across All Libraries

Unified package.json configurations across all libraries in the monorepo (Go, Rust, Python, JavaScript, C#).

## Changes Made

### New Files Created
- `go/server/package.json` - @semio-repo/server with dev, build, test, preflight scripts
- `rs/semio/package.json` - @semio/rs with build, test, preflight scripts

### Files Updated
- `package.json` (root) - Updated workspaces: removed non-existent `go/mcp`, added `go/server`, `rs/semio`, `py/semio`
- `go/repo/package.json` - Fixed incomplete build script, added dev script
- `go/semio/package.json` - Added build script
- `py/semio/package.json` - Added build, test, preflight scripts
- `net/Semio/package.json` - Added test script, updated preflight to use dotnet build
- `net/Semio.Grasshopper/package.json` - Added test script, updated preflight
- `js/vscode/package.json` - Renamed "package" script to "publish:vsix"

## Script Naming Convention

All libraries now follow consistent script naming:
- `dev` - Development mode/watch
- `build` - Production build
- `test` - Run tests
- `preflight` - Linting, type checking, formatting
- `publish` - Publish to registry
- `publish:vsix` - Package VSCode extension
