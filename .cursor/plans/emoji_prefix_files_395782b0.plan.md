---
name: Emoji Prefix Files
overview: Prefix every renamable basename under the four emoji product roots with a vocabulary emoji, keep tool-/constitution-mandated names unchanged, and reconfigure tools that already support alternate paths (Cargo `build`/`[[bench]]`/`[lib]`, `runVitest` config arg, explicit imports).
todos:
  - id: ticket-goals
    content: Auth repo MCP, read repo://goals, open ticket, bind plan id
    status: completed
  - id: file-vocab
    content: Build FILE_VOCAB + EXT_EMOJI + NEVER_RENAME_FILES; fail on gaps; write file-rename-map.json
    status: completed
  - id: apply-renames
    content: Apply file (and leftover dir) renames deepest-first under the four emoji roots
    status: completed
  - id: rewrite-refs
    content: Rewrite Cargo/TS/asset/policy refs; reconfigure build/bench/vitest paths
    status: completed
  - id: verify
    content: Verify cargo metadata, nx graph, sample vitest/asset paths; close ticket with summary + files
    status: completed
isProject: false
---

# Emoji-Prefix All Renamable Filenames

## Decision

Rename **every file basename that tools and repo constitution allow** under `✏️s`, `🧰️framework`, `🌎️hub`, and `♻️mit-bestand`. Keep hard-required names. Where a tool supports an alternate path, rename and reconfigure (same pattern as today’s `[lib] path = "…/📦️lib.rs"`).

`compose/` is out of this pass (still pre-emoji directory layout; separate structural ticket). `.repo/` ticket artifacts stay as-is. Dotfiles stay as-is.

Continue via a **new ticket** (distinct from the open dir-migration ticket), after reading `repo://goals` and authenticating repo MCP. Bind this plan id on open/close.

## Locked basenames (never rename)

These break discovery or repo law if renamed:

- **Package / workspace:** `Cargo.toml`, `Cargo.lock`, `package.json`, `bun.lock`, `bunfig.toml`, `go.mod`, `go.sum`, `go.work`, `go.work.sum`, `pyproject.toml`, `uv.lock`
- **Nx / TS defaults:** `project.json`, `nx.json`, `tsconfig.json`
- **Repo constitution:** `script.ts`, `AGENTS.md`, `CLAUDE.md` (policy and AGENTS require these exact names; AGENTS.md must not be edited)
- **Forge / legal display:** `README.md`, `LICENSE.md`, `CHANGELOG.md`, `CITATION.cff`
- **Toolchain discovery:** `rust-toolchain.toml`, `rustfmt.toml`, `nextest.toml`, `eslint.config.mjs`, `.prettierrc.json`, `.prettierignore`, `CMakeLists.txt`, `CMakePresets.json`, `*.csproj`, `*.sln`, `Trunk.toml` (Trunk default), `conftest.py`
- **Next.js app-router contracts:** `page.tsx` / `layout.tsx` / `route.ts` / `middleware.ts` / `globals.css` under Next `app/` trees; the `app/` dirname itself (already in `NEVER_RENAME` / exclude list)
- **Git hook install names:** `post-commit`, `prepare-commit-msg`, `post-checkout`, `post-rewrite`, `post-merge`, …
- **Cargo auto-discover dirnames** (already excluded): `benches`, `tests`, `examples`, `src` — **files inside them are renamable** via explicit `[[bench]]` / `[[test]]` `path =`

## Renamable with reconfiguration

| Class | Example | Emoji rule | Reconfigure |
| --- | --- | --- | --- |
| Cargo entry/build | `build.rs` | `📦️build.rs` | `[package] build = "📦️build.rs"` |
| Bench/test sources | `benches/kernel.rs` | `📦️kernel.rs` | explicit `[[bench]] path = "benches/📦️kernel.rs"` |
| Multi-file Rust modules | `📂️src/*.rs` (wfc) | `📦️<name>.rs` | `mod` / `#[path]` / `include!` |
| Generated Rust | `generated.rs`, `🤖️generated/*.rs` | `🤖️<name>.rs` | codegen output path + consumers |
| Vitest/Vite config | `vitest.config.ts`, `vite.config.ts` | `🧪️vitest.config.ts`, `⚙️vite.config.ts` | change `runVitest` default + every caller; Vite `--config` |
| Content / data | `index.md`, `typology.json`, `*.dsl`, fixtures | `FILE_VOCAB` or word emoji + name | rewrite imports/refs |
| Assets | `*.png/svg/glb/3dm/ttf/…` | **extension emoji** + original name (e.g. `🖼️foo.png`, `🔣️foo.svg`, `🔤️foo.ttf`, `🧊️foo.glb`) | rewrite URL/import/asset refs |
| Other TS/TSX/CSS/Go sources | `brand.ts`, `footer.tsx`, `main.go` (non-Next) | vocab or role emoji | rewrite imports; Go: `main.go` stays if `go run .` package-main convention requires it — keep `main.go` / `main_test.go` locked |

## Implementation (ticket workspace only for temp scripts)

Extend the existing migration helpers under [`.repo/🎫️/26/07/29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/`](.repo/🎫️/26/07/29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/) pattern — new scripts live in the **new ticket folder**:

1. **`FILE_VOCAB` + `EXT_EMOJI` + `NEVER_RENAME_FILES`** in a ticket-local `build-file-vocab.py` (mirror [build-vocab.py](.repo/🎫️/26/07/29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/build-vocab.py)).
   - Shared basenames first (`vitest.config.ts` → `🧪️`, `build.rs` → `📦️`, `index.md` → `📄️`, …).
   - Fallback: `EXT_EMOJI[ext] + basename` for unique assets.
   - Fail closed on any basename that is not locked, not in vocab, and has no ext fallback.

2. **Generate `file-rename-map.json`** (deepest paths first): `~5700` files in scope, `~3900` unique basenames.

3. **Apply renames** with filesystem/`git mv` (no stash/checkout). Also finish remaining **non-locked plain directories** from the dir vocab (Next `app/`, `benches/`, excluded recherche/vendor trees stay).

4. **Rewrite references** (extend [rewrite-refs-v2.py](.repo/🎫️/26/07/29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/rewrite-refs-v2.py)):
   - Cargo.toml `build` / `[[bench]]` / `[[bin]]` / `path =`
   - TS/JS imports, `package.json` `main`/`exports`/`types`
   - `runVitest(..., "vitest.config.ts")` → new default in [`📦️index.ts` `runVitest`](🧰️framework/🛍️product/🦑️repo/🔨️module/📚️lib/⚡️implementation/🟦️typescript/📦️index.ts) and all call sites
   - Asset URLs, Storybook, manifests, codegen emitters that write `generated.rs`
   - Policy scanners that match exact filenames (e.g. vitest config detection in repo lib tests) — update matchers, do not weaken rules

5. **Verify** (logs in ticket folder):
   - `cargo metadata` / targeted `cargo check` on crates that gained `📦️build.rs` or bench path changes
   - Nx project graph still loads (`project.json` unchanged)
   - Sample `bun ./📜️script.ts test` on a TS package using renamed vitest config
   - Spot-check asset-heavy plugin (cad/puzzle) for broken relative paths

## Explicit non-goals

- Renaming `script.ts` / `AGENTS.md` / `project.json` / `Cargo.toml` / `package.json`
- Emoji-izing `compose/` or root tooling manifests in this ticket
- Editing `AGENTS.md`
- Leaving adapter shims or dual-name aliases

## Scale note

Roughly **5.7k files** under the four roots. Mechanical rename + ref rewrite; hand-fix only vocab gaps and tool-path edge cases (wfc multi-file crate, 6 `build.rs`, benches, codegen outputs).
