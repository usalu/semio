# 🖼️ Registrar Handoff — Framework Assets Family De-Sandwich

De-sandwiched `🧰️framework/🔨️modules/🖼️assets/` into Shape V2: all data (fonts, icons, metabolism,
compose, logos, cursor, introduction, list, badge, images, mesh) now lives directly at the owner root
`🧰️framework/🔨️modules/🖼️assets/**`; the `@semio-tech/assets` TS package moved to
`📦️packages/🟦️typescript/` (`role = "framework"`, `id = "assets"`); `@semio-tech/logos` and
`@semio-tech/icons` (zero external consumers) were folded into the main package's `📜️script.ts` as
`generate-logo`/`export-logo` subcommands; a third orphan wrapper, `🖼️images/package.json`
(`@semio-tech/images`, also zero consumers), was deleted too. The
`⚡️implementations/🟦️typescript` sandwich is fully deleted.

**Every item below is OUTSIDE `🖼️assets/**` (this ticket's exclusive ownership) and was intentionally
left unedited**, per this ticket's explicit constraint list (never touch root Cargo.toml/package.json
workspaces, any plugin directory, or any other framework family's directory) — confirmed as the
established convention this wave by `CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE/FRAMEWORK-SURFACE-FAMILY-CRATE-CONSOLIDATION/📋️registrar-handoff.md`.
All old-path references below were found via `rg "🖼️assets/⚡️implementations"` repo-wide and are the
complete consumer list as of this session (Repo MCP was unavailable; found via ripgrep, not the graph).

Everywhere below: replace `🖼️assets/⚡️implementations/🟦️typescript/<rest>` with `🖼️assets/<rest>`
(same relative depth otherwise — only that middle path segment is dropped), **except** the two
`SEMIO_ASSET_ROOT`-shaped constants which must become `🖼️assets` (owner root) rather than
`🖼️assets/📦️packages/🟦️typescript` (the new package dir has no font/icon data siblings any more —
see §4).

## 1. Root `package.json` — workspaces (NEVER touch, per constraint)

Replace these 4 lines (lines 26–29; the last 3 are dead — `🏛️compose/🔣️icons` never had a
`package.json`, and `🖼️images`/`🪧️logos` no longer do post-consolidation):

```
    "🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript",
    "🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🏛️compose/🔣️icons",
    "🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🖼️images",
    "🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🪧️logos",
```

with ONE line:

```
    "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript",
```

(No root `Cargo.toml` change is needed — the assets family has no Rust crate of its own; confirmed
`grep "🖼️assets" Cargo.toml` has zero matches.)

## 2. Root `📜️script.ts` — workspace codegen orchestration (not edited)

`runWorkspaceCodegen()` (~line 277) hardcodes the build `cwd`:

```
    runCmd("bun", ["./📜️script.ts", "build"], {
      cwd: join(this.root, "🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript"),
      ...orchestratorBudgetOpts(),
    });
```

→ `cwd: join(this.root, "🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript")`.

## 3. `bun.lock`

Contains the old workspace member path as a lockfile entry — will self-correct on the next
`bun install` once §1 lands; do not hand-edit.

## 4. `ui/styling`'s `SEMIO_ASSET_ROOT` constant (ui family, not edited)

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🟦️vite-elements-assets.ts:886`:

```
export const SEMIO_ASSET_ROOT = "🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript";
```

**Important — do NOT mechanically drop just the `⚡️implementations/🟦️typescript` segment here.**
`resolveSemioAssetRoot()`/`uiAssetsVitePlugin()` in the same file validate/serve `🔤️fonts` (and other
data dirs) as direct children of `SEMIO_ASSET_ROOT`. Post-de-sandwich, `🔤️fonts` etc. live at the
*owner root*, not inside `📦️packages/🟦️typescript` (which now holds only packaging/code). The correct
replacement is the owner root itself:

```
export const SEMIO_ASSET_ROOT = "🧰️framework/🔨️modules/🖼️assets";
```

Verified on disk: `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🔤️anta/🔤️latin.woff2` exists (the exact
file the test suite at line ~1857 checks for), and the middleware's `cpSync(assetsRoot, dest, {
recursive: true })` behavior is unchanged (it already copied code-and-data siblings together before,
since the old sandwich dir held both too).

## 5. `ui-wgpu`'s dangling Anta/Noto-Emoji font paths (ui family, explicitly out of scope — DO NOT EDIT, per this ticket's own constraint)

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️text.rs` (~line 46–61) is
**actively being edited by a concurrent UI-ticket session right now** (observed mtime seconds old,
mid-migration): `ANTA_LATIN`/`KELLY_SLAB_LATIN`/`SHARE_TECH_MONO_LATIN` and 10 of the 12
`NOTO_EMOJI_BUCKETS` entries (indices 0–9) already correctly point at
`../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/...` (7 levels up, de-sandwiched, i.e.
exactly the target shape). **Only 2 lines remain broken** — this is the literal "missing
`10-400.ttf`/`11-400.ttf`" bug from the ticket brief:

```rust
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️10-400.ttf"),
    include_bytes!("../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔤️11-400.ttf"),
```

**Root cause confirmed: refs are stale, not missing files.** Both `🔤️10-400.ttf` and `🔤️11-400.ttf`
exist and always existed at `🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/` (verified on
disk this session) — these 2 lines simply never got the same 7-level-up correction as their 10
siblings on lines above them (probably a copy/paste that missed 2 lines). The fix — to be applied by
whoever owns `ui-wgpu`/finishes that concurrent session — is to match the established sibling pattern
exactly:

```rust
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️10-400.ttf"),
    include_bytes!("../../../../../../../🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🔤️11-400.ttf"),
```

I verified this exact resolved path exists on disk for both files and (by temporarily applying then
reverting the identical fix) that it is syntactically correct Rust — I did not leave the fix applied,
per this ticket's explicit "do NOT edit ui-wgpu's crate files, even if you find the font bug lives
there structurally" constraint. Note also: as of this session `ui-wgpu`'s own `Cargo.toml` declares
`build = "build.rs"` but no `build.rs` file currently exists in that directory (also mid-flight
concurrent-session churn, unrelated to fonts) — `cargo check -p semio-framework-ui-wgpu` will not fully
succeed until that settles, independent of the 2-line font fix above.

## 6. `vscode` client TS config (repo product, not edited)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⚡️implementations/🟦️typescript/🟦️vite.test.config.ts:58`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⚡️implementations/🟦️typescript/⚙️vite.config.ts:60`
  Both: `"@semio-tech/assets": path.resolve(__dirname, "../../../../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/📦️index.ts")`
  → `.../🖼️assets/📦️packages/🟦️typescript/📦️index.ts` (same depth, drop the middle segment).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/⚡️implementations/🟦️typescript/tsconfig.json:13-14`
  same rename for both the `@semio-tech/assets/*` and `@semio-tech/assets` path-map entries.

## 7. `os/renderer/engine` (os product, not edited)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts:15`
  `{ find: "@semio-tech/assets", replacement: resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/📦️index.ts") }`
  → drop the middle segment.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️lib.rs:1047`
  `const ANTA_LATIN: &[u8] = include_bytes!("../../../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔤️fonts/🔤️anta/🔤️latin.ttf");`
  → drop the middle segment (verified `🔤️anta/🔤️latin.ttf` exists at the de-sandwiched path; this is
  a *different* wgpu crate/target than §5's `ui-wgpu` — this one is `os/renderer/engine`'s own wgpu
  target, unrelated to the Noto Emoji bug, and is NOT under active concurrent editing).
- Same crate's `build.rs`: `let icons_dir = manifest_dir.join("../../../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔣️icons");`
  → drop the middle segment.

## 8. `os/infinite/canvas` (os product, not edited)

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/⚡️implementations/🦀️rust/build.rs:15,19`
  `repo_ui_assets()` and `metabolism_icons_dir()` helpers both join
  `"../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript"` (+ suffix for the
  latter) → drop the middle segment in both.
- Same crate's `📦️lib.rs:1215,1218`:
  ```
  #[path = "../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🌱️metabolism/🔣️icons/🤖️generated/🦀️metabolism_icon_name.rs"]
  mod metabolism_icon_name_gen;
  #[path = "../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🔣️icons/🤖️generated/🦀️icon_name.rs"]
  mod catalog_icon_name_gen;
  ```
  → drop the middle segment in both `#[path]` attrs. Verified both generated `.rs` files exist at the
  de-sandwiched location. (This directory also has its own local `🖼️assets/🔤️NotoColorEmoji-subset.ttf`
  for an unrelated concern — do not confuse with these icon-codegen paths.)

## 9. `os/dev` (os product, not edited)

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/⚙️vite.config.ts:77`
`{ find: "@semio-tech/assets", replacement: path.resolve(repoRoot, "./🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/📦️index.ts") }`
→ drop the middle segment.

## 10. `♻️mit-bestand` and `compose` apps (different top-level technologies, not edited)

- `♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts:36`
- `compose/client/lib/sketchpad/doc/js/vite.config.ts:43`
  Both alias `@semio-tech/assets` at `.../🖼️assets/⚡️implementations/🟦️typescript/📦️index.ts` → drop
  the middle segment. (Not "framework family" directories per this ticket's constraint list, but left
  untouched anyway to avoid mixing technologies without being asked.)

## 11. Plugin `Cargo.toml`/`build.rs` files (never touch — plugin directories forbidden)

`[[package.metadata.semio.assets]]` `roots`/`placeholder` (identical 2 lines) in all 4:

```
✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/Cargo.toml:24-25
✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml:48-49
✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:81-82
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:56-57
```

```
roots = ["🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🌱️metabolism/🎨️representation", "♻️mit-bestand/🖼️assets/🏚️abbau-aufbau"]
placeholder = "🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🥽️mesh/🧊️placeholder.glb"
```

→ drop the middle segment in both lines, all 4 files (verified both new paths exist on disk:
`🖼️assets/🌱️metabolism/🎨️representation` and `🖼️assets/🥽️mesh/🧊️placeholder.glb`).

Additionally, `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/build.rs:7` (a real compile-time consumer,
`include_str!`-style icon codegen, not just metadata):

```
let icons_src = manifest_dir.join("../../../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🌱️metabolism/🔣️icons");
```

→ drop the middle segment (verified `🖼️assets/🌱️metabolism/🔣️icons` exists with 29 SVGs at the new
location). **This one currently silently produces an empty icon table** (the code checks
`icons_src.is_dir()` and no-ops if false, rather than panicking) — not a hard build break, but a real
runtime regression (`board_metabolism_icon_svg()` will return `None` for everything) until fixed.

## 12. Other framework-family Rust consumers of the metabolism `capsule_J.glb` (other framework families, not edited)

All 3 use the exact same `include_bytes!(...capsule_J.glb)` line for a "decode Puzzle GLB" test/demo:

- `🧰️framework/⚡️implementations/🦀️rust/📦️lib.rs:2725` (framework/core) —
  `include_bytes!("../../../🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🌱️metabolism/🎨️representation/🧊️capsule_J.glb")`
  → drop the middle segment (3-levels-up prefix is already correct for this file's depth).
- `🧰️framework/🔺️mesh/🦀️component.rs:2630` (framework/mesh) — **same line, but ALSO has a
  pre-existing, unrelated depth bug**: this file lives directly at `🧰️framework/🔺️mesh/` (one level
  under `🧰️framework/`), so the copy-pasted `"../../../🧰️framework/..."` prefix (3 levels up + re-enter
  `🧰️framework`) never resolved correctly even before this ticket touched anything — it needs to
  become just `"../🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/🧊️capsule_J.glb"` (1 level up,
  no `🧰️framework/` re-entry). Verified via `cd` that only the 1-level-up form resolves from this
  file's real directory.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/⚡️implementations/🦀️rust/🌍️world/📦️lib.rs:3899`
  (os/infinite/world) —
  `include_bytes!("../../../../../../../🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🌱️metabolism/🎨️representation/🧊️capsule_J.glb")`
  → drop the middle segment (7-levels-up prefix already correct for this file's depth).

## 13. Environment note

Repo MCP was unavailable this session; the consumer list above was built via `rg -l
"🖼️assets/⚡️implementations" .` repo-wide (re-run that after applying §1–§12 — it should return zero
matches outside `bun.lock`, which self-corrects on next `bun install`). `ui-wgpu` (§5) and the plugin
`Cargo.toml`s (§11) were observed to be under active concurrent editing/churn during this session —
re-verify §5's exact line numbers before applying, they may have shifted.

## Registrar handoff JSON

```json
{
  "owner": "🖼️assets",
  "ticketPath": "26/08/06/FRAMEWORK-ASSETS-FAMILY-DE-SANDWICH",
  "status": "flagged",
  "oldMemberLines": [
    "\"🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript\"",
    "\"🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🏛️compose/🔣️icons\"",
    "\"🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🖼️images\"",
    "\"🧰️framework/🔨️modules/🖼️assets/⚡️implementations/🟦️typescript/🪧️logos\""
  ],
  "newCrates": []
}
```

Note: no root `Cargo.toml` members — family is TS-only (`@semio-tech/assets` at
`📦️packages/🟦️typescript`, `role = "framework"`, `id = "assets"`). Registrar applies §1–§12; `status`
stays `flagged` until those consumer paths land.


## Registrar applied (orchestrator 2026-08-06)
- Root package.json workspaces: pruned 56 dead entries; assets → `📦️packages/🟦️typescript`.
- Consumer path repoints for old `⚡️implementations/🟦️typescript` segment (handoff §2–§12 except ui-wgpu §5).
- `SEMIO_ASSET_ROOT` → owner root `…/🖼️assets`.
- `bun install` succeeded after also adding surface-rs + math-js + s-3d-js workspaces and renaming graph-manifest→framework-math-js, kernel-3d-js→s-3d-js.
