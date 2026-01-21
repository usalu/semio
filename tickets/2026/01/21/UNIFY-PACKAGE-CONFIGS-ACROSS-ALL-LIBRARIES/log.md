# Log: Unify Package Configs Across All Libraries

## Analysis Phase

Analyzed existing package.json files across the monorepo:

### Existing package.json files found:
- `/package.json` - root workspace
- `assets/package.json` - @semio/assets
- `assets/logo/package.json` - @semio/logo
- `assets/icons/package.json` - @semio/icons
- `js/semio/package.json` - @semio/js
- `js/docs/package.json` - @semio/docs
- `js/play/package.json` - @semio/play
- `js/desktop/package.json` - @semio/desktop
- `js/vscode/package.json` - semio-repo (VSCode extension)
- `py/semio/package.json` - @semio/py
- `py/engine/package.json` - @semio/engine
- `go/semio/package.json` - @semio/go
- `go/repo/package.json` - @semio-repo/go
- `net/Semio/package.json` - @semio/net
- `net/Semio.Grasshopper/package.json` - @semio/grasshopper
- `yak/package.json` - @semio/yak

### Missing package.json files:
- `go/server/package.json` - created
- `rs/semio/package.json` - created

### Issues found and fixed:
1. `go/mcp` was in workspaces but doesn't exist - removed
2. `js/vscode/package.json` had "package" script - renamed to "publish:vsix"
3. `go/repo/package.json` had incomplete build script - fixed
4. `go/semio/package.json` was missing build script - added
5. `py/semio/package.json` was missing all scripts - added build, test, preflight
6. `.NET` packages were missing test scripts - added

## Implementation

### Created files:
1. `go/server/package.json` - with dev, build, test, preflight scripts
2. `rs/semio/package.json` - with build, test, preflight scripts

### Updated files:
1. `go/repo/package.json` - fixed build script, added dev script
2. `go/semio/package.json` - added build script
3. `py/semio/package.json` - added build, test, preflight scripts, changed projectType to library
4. `net/Semio/package.json` - added test script, updated preflight
5. `net/Semio.Grasshopper/package.json` - added test script, updated preflight
6. `js/vscode/package.json` - renamed "package" to "publish:vsix"
7. `package.json` (root) - updated workspaces list

## Unified Script Naming Convention

| Script | Purpose |
|--------|---------|
| `dev` | Start development mode/watch |
| `build` | Build production artifacts |
| `test` | Run tests |
| `preflight` | Linting, type checking, formatting |
| `publish` | Publish to package registry |
| `publish:vsix` | Package VSCode extension |
