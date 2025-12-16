---
slug: SQL-WASM-SKETCHPAD-BUILD
summary: Fix sql-wasm.wasm not being included in sketchpad production build
---

# Previously

Production deployment of `js/sketchpad` to `https://sketchpad.semio-tech.com` failed with:

```
Failed to auto-import kit from /metabolism.zip: TypeError: Failed to fetch dynamically imported module: https://sketchpad.semio-tech.com/assets/sql-wasm-B36ncQqm.js
```

The error did not appear in `vite preview`.

# Plan

1. Investigate how sql.js WASM loading is configured
2. Compare what files exist in `js/js/public` vs `js/sketchpad/public`
3. Verify dist output after build

# Changes

## Root Cause

The `sql.js` library requires a `sql-wasm.wasm` file to be available at runtime. The `@semio/js` package had a `postinstall` script that copied this file to `js/js/public/sql-wasm.wasm`, but `@semio/sketchpad` had its own `public` folder without the wasm file.

During Vite build, only files from the app's own `public` folder are copied to `dist`, so the wasm file was missing in production.

The error message about `.js` (not `.wasm`) was misleading - the actual issue was that the JavaScript module was trying to fetch the wasm file which didn't exist.

## Fix

Added `postinstall` script to `js/sketchpad/package.json`:

```json
"postinstall": "node -e \"require('fs').copyFileSync('../../node_modules/sql.js/dist/sql-wasm.wasm', 'public/sql-wasm.wasm')\""
```

This ensures `sql-wasm.wasm` is copied to `js/sketchpad/public/` which then gets included in the production build at `dist/sql-wasm.wasm`.

## Second Issue: GitHub Pages Jekyll Processing

After deploying with the wasm file, a new error appeared:

```
Laden des Moduls von "https://sketchpad.semio-tech.com/assets/__vite-browser-external-D7Ct-6yo.js" wurde auf Grund eines nicht freigegebenen MIME-Typs ("text/html") blockiert.
```

The `__vite-browser-external-*.js` file existed in dist but GitHub Pages returned 404 (HTML page).

### Root Cause

GitHub Pages uses Jekyll by default, which **ignores files and folders starting with underscore** (`_`). Vite generates `__vite-browser-external-*.js` files as stubs for Node.js modules externalized for browser compatibility.

### Fix

Created `.nojekyll` file in `js/sketchpad/public/` to disable Jekyll processing on GitHub Pages. This file is copied to dist during build and tells GitHub Pages to serve all files as-is.
