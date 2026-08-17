# Language-Tag Directories Verify Log

## Migration script

- `.repo/🎫️/26/07/02/INSERT-LANGUAGE-TAG-DIRECTORIES-UNDER-EVERY-BUNDLE/migrate-language-tags.ts`
- Ran multiple passes; final inventory shows no remaining root source files (except compliant bundles).

## Structural checks

- `cad/js/` dismantled → bundles under `cad/` with local `js/` folders
- `cad/renderer/renderer/` hoisted to `cad/renderer/`
- Compliant bundles unchanged (`draw/rs`, `kernel/2d/js`, etc.)

## Build verification

- `cargo check --workspace` — pass (warnings only)
- `note/core` vitest — 9/9 pass
- `draw/core` vitest — 4/4 pass
- `repo/client/cli/go` `go build` — pass (via `repo/go.work`)

## Manual fixes applied

- Cargo.toml workspace members updated with `/rs` suffixes
- Cargo path dependencies repaired (`repair-cargo-paths.ts`)
- `include_str!` paths for example/fixture data after rs/ moves
- `mathematical/graph/manifest/rs/build.rs` + `lib.rs` → `../generated/`
- `Monorepo.sln` csproj paths → `cs/` subfolder
- `go.work` + `repo/go.work` → `*/go` module paths
- Go module replace paths depth-corrected for nested `mcp/*/go` modules
