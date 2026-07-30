---
name: Untrack compiled artifacts
overview: Stop version-controlling OS-dev WASM/JS/native build outputs (~3700 files) by fixing stale `.gitignore` paths after the product-tree move, then removing those paths from the git index while keeping local build trees and handwritten source.
todos:
  - id: open-ticket
    content: Auth repo MCP if needed; open ticket under AI-OPTIMIZED-REPO
    status: completed
  - id: fix-gitignore
    content: Update .gitignore paths for framework/os/dev build outs, boot.js, coverage
    status: completed
  - id: untrack-index
    content: git rm -r --cached on plugin-modules, native, renderer-modules, public plugins, coverage, boot.js
    status: completed
  - id: verify-and-close
    content: Verify ignore + ls-files; close ticket with file list
    status: completed
isProject: false
---

# Untrack Compiled JS and WASM

## Diagnosis

After apps moved out of `framework/product/`, [`.gitignore`](.gitignore) still ignores the **old** paths:

```81:86:.gitignore
/framework/product/os/dev/plugin-modules/
/framework/product/os/dev/plugin-modules-native/
/framework/product/os/dev/public/plugin-modules/
/framework/product/os/dev/renderer-modules/
/framework/product/os/dev/public/plugins/
/public/plugin-modules/
```

Build outputs now land at `framework/os/dev/...` (see [`framework/os/dev/script.ts`](framework/os/dev/script.ts) `pluginOutRoot`) and are **tracked**:

| Path | Tracked files | Contents |
|------|---------------|----------|
| `framework/os/dev/plugin-modules/` | ~895 | WASM, jco/bindgen JS, `.d.ts`, vendored preview2-shim |
| `framework/os/dev/renderer-modules/` | ~2769 | staged wgpu renderer + copied plugin WASM/JS/assets |
| `framework/os/dev/plugin-modules-native/` | 25 | `.dylib` plugin binaries |
| `public/plugin-modules/` | 4 | lowpoly WASM/JS (already listed in ignore, still tracked) |
| `framework/os/kernel/.../js/coverage/` | 9 | coverage report (not ignored) |
| `framework/os/renderer/wgpu/js/boot.js` | 1 | bun-built from `boot.ts` |

**Keep tracked:** handwritten source/config (`*.ts`, `eslint.config.mjs`, compose stubs, ticket scripts under `.repo/🎫/`, mit-bestand presentation snapshot assets, codegen token files like `tokens.generated.ts` — those are a separate concern).

Goal association on execute: `AI-OPTIMIZED-REPO` / consistent repo hygiene (open ticket via repo MCP once auth works).

## Approach

1. **Open ticket** with repo MCP (`ticket_open`, goal `AI-OPTIMIZED-REPO`), stash inventory/logs under the ticket folder.
2. **Fix ignore rules** in [`.gitignore`](.gitignore) VITE section:
   - Replace stale `/framework/product/os/dev/...` entries with current `/framework/os/dev/plugin-modules/`, `plugin-modules-native/`, `public/plugin-modules/`, `renderer-modules/`, `public/plugins/`.
   - Keep `/public/plugin-modules/`.
   - Add `/framework/os/renderer/wgpu/js/boot.js`.
   - Add `**/coverage/` (coverage reports are not source).
   - Add native binary globs for plugin builds: `*.dylib`, and ensure `*.so` / `*.dll` plugin outputs cannot sneak back in under those dirs (dir ignores already cover the native out root).
3. **Untrack without deleting working trees** (so local builds keep working):
   ```bash
   git rm -r --cached -- \
     framework/os/dev/plugin-modules \
     framework/os/dev/plugin-modules-native \
     framework/os/dev/renderer-modules \
     public/plugin-modules \
     framework/os/kernel/math/graph/dsl/core/js/coverage \
     framework/os/renderer/wgpu/js/boot.js
   ```
   Do **not** commit (per repo rules / unless you ask). Staging the removals + ignore fix is the change set.
4. **Verify**:
   - `git check-ignore -v` on a sample `.wasm` / `.js` / `.dylib` under the new paths.
   - `git ls-files '*.wasm'` is empty (or only non-build exceptions if any remain — expect none).
   - Handwritten JS/TS configs still tracked.
5. **Close ticket** with summary + touched paths.

## Out of scope (unless you expand)

- `tokens.generated.ts` / `generated.rs` / `generated.py` styling/schema codegen (generated, but not WASM/JS compile artifacts).
- mit-bestand scraped presentation vendor JS (external snapshot, not built from this repo).
- Deleting local build directories from disk.