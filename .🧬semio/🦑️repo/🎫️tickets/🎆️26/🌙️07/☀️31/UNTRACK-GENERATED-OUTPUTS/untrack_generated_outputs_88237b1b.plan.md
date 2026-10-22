---
name: Untrack Generated Outputs
overview: Stop checking in toolchain outputs (wasm/js/d.ts bundles, plugin-modules, boot.js, and all 🤖️generated codegen). Harden .gitignore, fix stale generator paths, wire zero-touch regeneration into setup, and untrack everything already committed—lock files stay.
todos:
  - id: ticket
    content: Open UNTRACK-GENERATED-OUTPUTS ticket bound to AI-OPTIMIZED-REPO; keep audits/logs in ticket folder
    status: completed
  - id: gitignore
    content: Harden .gitignore for 🤖️generated, plugin-modules, wasm, boot.js, tokens.generated, pkg, maps; remove stale non-emoji paths
    status: completed
  - id: fix-generators
    content: Retarget schema/ui-asset/styling generators to current emoji output paths
    status: completed
  - id: setup-codegen
    content: Wire catalog generate into SetupScript + generate-if-missing in relevant Rust build.rs
    status: completed
  - id: untrack
    content: git rm --cached all tracked outputs; audit list in ticket folder
    status: completed
  - id: verify
    content: Confirm ignore/ls-files clean; regenerate; cargo/registry checks pass; hand-written configs still tracked
    status: completed
isProject: false
---

# Untrack All Generated Outputs

## Scope (option 2)

**Remove from git + ignore forever**
- Entire [`🔌️plugin-modules/`](🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/🔌️plugin-modules) tree (wasm, jco js/d.ts, `_vendor` preview2-shim copies, `.hot-swap`)
- Bundled [`🟨️boot.js`](🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/⚡️implementations/🦀️rust/🧑️‍🎨️engine/🧊️wgpu/🟦️typescript/🟨️boot.js) (built from [`🟦️boot.ts`](🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/⚡️implementations/🦀️rust/🧑️‍🎨️engine/🧊️wgpu/🟦️typescript/🟦️boot.ts))
- All `**/🤖️generated/**` dirs and single-file `**/🤖️generated.*` (icons, graph manifests, plugin registry, schema, styling)
- Other codegen products already tracked: e.g. [`🟦️tokens.generated.ts`](🧰️framework/🔨️modules/🖱️ui/⚡️implementations/🟦️typescript/🎨️styling/🟦️tokens.generated.ts), Next `🟦️next-env.d.ts`
- Global build leftovers: `*.wasm`, `*.map`, `**/pkg/**` (wasm-bindgen), keep existing `dist`/`out`/`target*` ignores; refresh stale non-emoji plugin-modules paths in [`.gitignore`](.gitignore)

**Keep tracked**
- Lock files only as the generated exception: `bun.lock`, `Cargo.lock`, `uv.lock`, `skills-lock.json`, etc.
- Hand-written source configs: `eslint.config.mjs`, `.dependency-cruiser.cjs`, `🟨️nx-plugin.mjs`, compose stubs, `vite-env.d.ts`, ticket workspace scripts under `.repo/🎫️/`
- Static presentation assets under `♻️mit-bestand/.../🌐️public/` (not toolchain codegen)

**Goal association:** [`AI-OPTIMIZED-REPO`](.repo/🎯️/AI-OPTIMIZED-REPO/goal.json)

**Ticket:** open new ticket `UNTRACK-GENERATED-OUTPUTS` (MCP if available; otherwise on-disk under `.repo/🎫️/26/07/31/` like the aggregator ticket). Put audit lists/logs in the ticket folder.

## Why current ignores fail

- Pattern `generated` matches a path segment named `generated`, **not** `🤖️generated` (verified with `git check-ignore`).
- Vite/plugin-modules ignores still use pre-emoji paths (`/framework/product/os/module/dev/js/plugin-modules/`), so live emoji paths are tracked.

## Implementation

### 1. Harden [`.gitignore`](.gitignore)

Add an explicit OUTPUTS section (keep lock files unignored):

```gitignore
#--------------------------------------OUTPUTS--------------------------------------
**/🤖️generated/
**/🤖️generated.*
**/🔌️plugin-modules/
**/*.wasm
**/*.map
**/pkg/
**/🟨️boot.js
**/🟦️tokens.generated.ts
**/🟦️next-env.d.ts
**/forge.env.d.ts
```

Replace stale non-emoji plugin-modules / boot.js entries with the emoji paths above. Do **not** add blanket `*.js` / `*.d.ts` (would kill hand-written source).

### 2. Fix generators that still write stale paths

These must emit into the ignored locations or regeneration after untrack will miss consumers:

- [`🧰️framework/🔨️modules/🧬️schema/.../📜️script.ts`](🧰️framework/🔨️modules/🧬️schema/⚡️implementations/🦀️rust/📜️script.ts) — still targets `framework/module/schema/...`; retarget to current emoji paths (`🔣️entity-kinds.json` input, `🤖️generated.rs` + TS/Go siblings beside the package).
- [`🧰️framework/🔨️modules/🖱️ui/🖼️assets/.../📜️script.ts`](🧰️framework/🔨️modules/🖱️ui/🖼️assets/⚡️implementations/🟦️typescript/📜️script.ts) — still uses `icon/generated`; retarget to `🔣️icons/` catalog + `🔣️icons/🤖️generated/`.
- [`🧰️framework/🔨️modules/🖱️ui/.../🎨️styling/📜️script.ts`](🧰️framework/🔨️modules/🖱️ui/⚡️implementations/🦀️rust/🎨️styling/📜️script.ts) — align `generatedDir` / JS token output with ignored paths (`🤖️generated.rs`, `🟦️tokens.generated.ts`, CSS under ignored dirs).

### 3. Zero-touch regeneration

[`📜️script.ts` `SetupScript.runFull`](📜️script.ts) currently does **not** run catalog generators. Add a codegen step after install tooling that runs, in dependency order:

1. `@semio-tech/framework-schema:generate`
2. ui styling `generate` (tokens → Rust/TS/Python/CSS)
3. `@semio-tech/ui-asset` build / `generate all`
4. `@semio-tech/semio-asset` build (metabolism icons)
5. `@semio-tech/graph-manifest:generate`
6. `@semio-tech/plugin-registry:generate`

Also teach Rust `build.rs` for packages that `include!` generated files ([graph-manifest](🧰️framework/🔨️modules/🧮️math/⚡️implementations/🦀️rust/🕸️graph/🛂️manifest/build.rs), schema if needed) to invoke the package `bun ./📜️script.ts generate` when the artifact is missing, so `cargo check` alone remains zero-touch.

Plugin wasm/`🔌️plugin-modules` stay **dev/build** outputs (produced by os-dev `📜️script.ts` on playground build), not setup-wide — only ignored + untracked.

`🟨️boot.js` continues to be produced by the existing wgpu [`📜️script.ts` bun build](🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/⚡️implementations/🦀️rust/🧑️‍🎨️engine/🧊️wgpu/📜️script.ts) before Trunk; ensure that path is gitignored.

### 4. Untrack currently committed outputs

From repo root (index-only; no commit):

```bash
git rm -r --cached -- '**/🤖️generated' '**/🔌️plugin-modules' ...
```

Plus explicit paths for single-file `🤖️generated.*`, `🟨️boot.js`, `🟦️tokens.generated.ts`, `🟦️next-env.d.ts`. Write the full path list into the ticket folder for audit.

Do **not** `git commit` (workspace rule + user rule). Staging the removals is enough for the ticket; the human bundles later.

### 5. Verify

- `git check-ignore -v` on representative wasm / `🤖️generated` / `🟨️boot.js` paths
- `git ls-files` shows **zero** matches for those patterns outside `.repo/🎫️` (except locks)
- Run the new setup codegen step; confirm key consumers resolve (`cargo check -p semio-framework-os-kernel-math-graph-manifest`, schema crate, plugin-registry `:check`)
- Confirm hand-written `eslint.config.mjs` / `🟨️nx-plugin.mjs` / `vite-env.d.ts` remain tracked

## Out of scope

- Rewriting hand-written `.mjs`/`.cjs`/`.d.ts` source to TypeScript
- Deleting mit-bestand scraped public JS
- Committing the untrack (leave staged/working tree for the existing micro-commit/bundle flow)
