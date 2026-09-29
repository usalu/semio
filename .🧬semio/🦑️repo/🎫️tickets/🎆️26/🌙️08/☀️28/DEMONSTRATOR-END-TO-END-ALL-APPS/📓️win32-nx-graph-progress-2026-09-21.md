# Win32 Nx graph progress (2026-09-21)

Goal: demonstrator end-to-end for all eight plugin panes (`♻️mit-bestand/🧺️demonstrator`, acceptance suite `bun nx run @semio-tech/mit-bestand-demonstrator:test-e2e`).

## Blockers found this session

1. **Nx project graph** failed on Windows because `@repo/test-cases` and `@repo/emoji-project-json` emitted `createDependencies` edges whose `sourceFile` paths did not match Nx’s indexed workspace paths (emoji paths corrupted in the native walker).

2. **Root symlink** `.generation3d-crate-link` duplicated test-case discovery under a second tree.

## Changes applied (in working tree)

- `🧰️framework/.../library/🟨️.mjs`: `nxTrackedSourceFile`, `walkCargoToml`, realpath-based `📋️project.json` / `Cargo.toml` roots, skip non-existent dependency source files, **on `win32` skip native `createDependencies` edges** (keep bun.lock edges only).
- `🧰️framework/.../test/🟨️.mjs`: discover cases only via `discoverCaseDirs` (not Nx glob), skip symlink directories, skip `.generation*-link` paths, **on `win32` return no test `createDependencies`**.

## Current state after fixes

- `bun nx reset` + `bun nx run @semio-tech/mit-bestand-demonstrator:test-e2e` gets past project-graph **dependency** validation.
- Run then fails in the **native task hasher**: `Value is non of these types ... on Target.inputs` (needs `--verbose` bisect to which project/target has an invalid `inputs` entry).

## Next steps

1. Bisect invalid `inputs` on the graph (likely a generated `type:test` project or a demonstrator dependency).
2. Run full `test-e2e` (cold wasm build + Playwright acceptance for all eight panes).
3. Fix any pane failures per `🧪️tests/🎭️acceptance/🟦️.ts`.

## Evidence commands

```powershell
$env:NX_DAEMON = "false"
bun nx reset
bun nx run @semio-tech/mit-bestand-demonstrator:test-e2e --verbose
```
