# Empty windows diagnosis (procedural 3d)

## Root cause
Runtime-installable extensions load from `/extensions/*`, but cargo/dev builds only wrote artifacts to `🔌️plugin-modules/`. `🔌️extension-modules/` stayed empty, so Vite SPA-fallback served HTML for extension JS URLs, program workers crashed, and procedural eval/preview emptied out after extension load attempts.

A second race made it worse under hot-swap: the `.hot-swap` SSE fired before artifacts were published into `/extensions`, so the browser reloaded a URL that did not exist yet.

## Fix
In `framework-os-dev` `📜️script.ts`:
1. `publishBuiltExtension` — after each `role: "extension"` build, atomically stage+rename into `extension-modules/` (with `install.json`).
2. `syncBuiltExtensionsToInstallRoot` — on prepare, seed `/extensions` from already-built `plugin-modules` crates (restart without full rebuild).
3. Publish **before** writing the `.hot-swap` marker so SSE clients never fetch a missing module.

## Verified (port 6018, settled after rebuild storm)
- `/extensions/flow-extension-brep/...js` serves `text/javascript`
- Fault probe t10s/t35s/after-contextmenu: `hasRenderError=false`, no fault boundaries
- Console: 0 `program load failed`, 0 worker crashes
