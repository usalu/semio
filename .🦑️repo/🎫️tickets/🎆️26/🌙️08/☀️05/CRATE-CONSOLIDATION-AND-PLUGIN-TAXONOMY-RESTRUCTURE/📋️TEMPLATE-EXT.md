# 📋️ TEMPLATE-EXT — de-sandwiching a `🧩️extensions/*` crate/package in place

Written by the W6 pilot (🌊️flow, ticket `26/08/06/FLOW-PLUGIN-TS-MODULES-AND-EXTENSIONS-CONSOLIDATION`) from
what actually happened de-sandwiching flow's `🏗️bim` wasm-bindgen extension. Read the master doc's "Plugin
residuals" bullet (Rule B) first; this file is the how.

**Rule B, restated:** extensions stay SEPARATE packages — never fold into the plugin's own
`📦️packages/🦀️rust` or `-js` package — because they are dynamically loaded/looked-up by string at runtime
(the plugin loader resolves an extension by its published crate/package name). De-sandwiching only moves
the manifest+entry file into `📦️packages/<lang>/`; it must never rename anything that's a lookup key.

---

## 0. Before you touch anything

1. Inventory: `find ✏️s/🔌️plugins/<p>/🧩️extensions/<e> -type f`. Note every manifest file's CURRENT
   directory depth (count path segments from repo root) — you'll need it twice, once to recompute the new
   depth in §2 and once to sanity-check whether it changed at all (it might not — see §2's aside).
2. Read `Cargo.toml`'s `[package] name` (Rust) and/or `package.json`'s `"name"` (TS/wasm wrapper) —
   whichever exists. **This string is frozen. Do not touch it, even if it looks inconsistent with the
   plugin's own new naming (e.g. `semio-s-plugin-flow-extension-bim` staying exactly that, not becoming
   `semio-s-plugin-flow-bim` or anything shorter).** Grep the whole repo for the name to see every runtime
   lookup site before you start, so you recognize if you accidentally need to touch one (you almost never
   do — de-sandwiching is a pure relocation, the name never appears inside the moved files themselves).
3. Determine which case you're in — **Rust-only**, **TS-only**, or **hybrid** (a Rust wasm-bindgen crate
   with a hand-authored npm wrapper around its own generated wasm-pack output, flow's bim case). The hybrid
   case has a real technical trap (§3) that the other two don't.

---

## 1. Rust case (or the Rust half of a hybrid)

```
✏️s/🔌️plugins/<p>/🧩️extensions/<e>/📦️packages/🦀️rust/{Cargo.toml, 📦️lib.rs, 📋️project.json, 📜️script.ts}
```

Same rules as a full plugin's own package, just scoped to the extension's own small tree:

- `[lib] path = "📦️lib.rs"` (same-dir, matching the plugin convention's literal filename — keep the `📦️`
  emoji prefix, the ticket-level shorthand "`lib.rs`" in task prompts means "the entry file," not literally
  the bare ASCII name).
- **Recompute every dependency's relative-path depth — do not assume it stays the same.** An extension
  living at `🧩️extensions/<e>/⚡️implementations/🦀️rust` and moving to
  `🧩️extensions/<e>/📦️packages/🦀️rust` happens to be the SAME depth (swapping one 6-character-ish segment
  for another single segment), so flow's bim needed zero dependency-path edits — but this is a coincidence
  of bim's specific old layout, not a general guarantee. Count segments from repo root to both the old and
  new manifest location; if they differ, every `path = "../../…"` dependency needs the delta applied, same
  as any other relocation.
- If `lib.rs` has any `#[path]` submodule tree (most extensions don't — they're often one flat file, like
  bim), apply the SAME leaf-vs-`"."` prefixing rule as `📋️TEMPLATE.md` §14 step 3. Check for
  `include_str!`/`include_bytes!` too (§14 step 3b) — easy to miss, only a real `cargo test` catches a wrong
  target.
- Add the plugin-package-shape `test`/`test-quick`/`test-long`/`test-exhaustive` leveled targets to
  `📋️project.json` even though this is "just" an extension — `checkLeveledTestTargets` doesn't carve out an
  exception for extension crates, and there's no reason to skip real per-level budgeting here either.
- If the extension ALSO has a wasm-pack "wasm" build step (most wasm-bindgen extensions do — see §3), add a
  `wasm` target to the SAME `📋️project.json` and register a second command on the SAME `📜️script.ts`
  (`.register("test", TestScript).register("wasm", WasmScript)`) rather than splitting into two script
  files — CLAUDE.md forbids extra script files per directory, and there's no reason to want two anyway.

---

## 2. TS-only case

If the extension is pure TypeScript (no Rust at all — check `📐️cad`'s several TS-only extensions for real
examples once one is retrofitted), it's just Rule A's recipe (`TEMPLATE-TS.md`) applied at the extension's
own root instead of the plugin's root: `🧩️extensions/<e>/📦️packages/🟦️typescript/{package.json,
📋️project.json, 📜️script.ts, 📦️index.ts}`, published npm name frozen exactly like a Rust crate's name is
frozen. No wasm-pack trap (§3) applies here since there's no generated `pkg/` output to reach.

---

## 3. Hybrid case — the `pkg/` output escape trap (read this before creating a TS package for a wasm crate)

A `wasm-bindgen`/`wasm-pack` extension crate's build step (`runWasmPackWebBuild` in the shared repo-lib
script helpers) **always** writes its generated JS/wasm/`.d.ts` output to `<rsDir>/pkg/` — `rsDir` being
wherever the crate's OWN `Cargo.toml` lives, no override exists for the output location. This collides
directly with Shape V2's instinct to put a hand-authored npm wrapper package.json in a SIBLING
`📦️packages/🟦️typescript/` directory: **Node's (and Bun's) `package.json` `"exports"` field hard-rejects any
target path containing `../`** (`ERR_INVALID_PACKAGE_TARGET`) — a wrapper package.json living in
`📦️packages/🟦️typescript/` cannot legally `export` a file that only exists in the sibling
`📦️packages/🦀️rust/pkg/`.

**Ruling: do NOT create a `📦️packages/🟦️typescript/` directory for a wasm-bindgen extension's generated
wrapper.** Instead, register the crate's own generated `📦️packages/🦀️rust/pkg` directory directly as the
root `package.json` workspace member — this is the ALREADY-ESTABLISHED pattern elsewhere in this exact
codebase (`🧰️framework/…/🌊️flow/🫀️core/⚡️implementations/🦀️rust/pkg` is a literal root workspaces entry
today, not a hand-authored wrapper). `runWasmPackWebBuild`'s `pkg: { name, files, main, module, types }`
option writes a real `package.json` directly into the generated `pkg/` dir on every build — that generated
file IS the published npm identity (frozen `name`, same as the Rust crate name is frozen), no separate
hand-maintained manifest needed. Concretely:

- Delete any pre-existing hand-authored `package.json` at the extension root that only existed to redirect
  `exports` into the nested wasm-pack output (flow's bim had exactly this — a legacy artifact whose
  `exports` happened to still resolve under the OLD sandwich shape only because the wrapper and the `pkg/`
  it pointed into were both nested under the SAME extension-root directory at the time).
- Add ONE literal line to root `package.json` workspaces: `"✏️s/🔌️plugins/<p>/🧩️extensions/<e>/📦️packages/🦀️rust/pkg",`
  — a literal path, not a glob (the dir may not exist on disk yet until the first `wasm` build runs; verify
  it resolves — if you're unsure whether bun errors on a not-yet-existing workspace path the way Cargo hard-errors
  on a zero-match member glob, run the `wasm` build once BEFORE adding the workspaces line, or flag it for the
  registrar to verify in the same pass it applies the edit).
- The `test`/`wasm` targets both live on the Rust package's own `📋️project.json`/`📜️script.ts` (§1's last
  bullet) — there is no separate TS project to register targets on.

If you ever encounter a wasm-bindgen extension whose npm consumers need something the generated `pkg/`
output genuinely can't provide (extra hand-written TS glue beyond the raw bindings), that additional glue
is a real authored TS module and belongs in a real `📦️packages/🟦️typescript/📦️index.ts` that **imports**
from the generated `pkg` package by its published name (`import … from "@semio-tech/<wrapper-name>"`), not
by a relative path — that sidesteps the `../` escape entirely, since package-name imports resolve through
`node_modules`/bun's workspace symlinks rather than the filesystem-relative `exports` map.

---

## 4. Registrar handoff — root Cargo.toml member-line path change

An extension crate almost always has its own root `Cargo.toml` member line (it's a real, independently
compiled crate). De-sandwiching moves that line's target, nothing else:

```
Change this root Cargo.toml member line:
    "✏️s/🔌️plugins/<p>/🧩️extensions/<e>/⚡️implementations/🦀️rust",
to:
    "✏️s/🔌️plugins/<p>/🧩️extensions/<e>/📦️packages/🦀️rust",
```

The package NAME inside that manifest is unchanged (§0.2), so this is a pure path swap — no
`[workspace.dependencies]` alias rename, no dependent repoint needed anywhere else in the workspace (nothing
depends on an extension crate by workspace-alias the way plugins depend on framework crates; extensions are
themselves the dependency-graph leaves, loaded dynamically, not `path =`-referenced by anything else in
`Cargo.toml`). If your extension is a hybrid (§3) and you also touched root `package.json` workspaces
yourself (per your dispatch's exception, same caveat as `TEMPLATE-TS.md` §5), the Cargo.toml line above is
still registrar-only — the two files have independent ownership even when the same agent is allowed to
touch one and not the other.

---

## 5. Verification sequence

Use `📋️TEMPLATE.md` §3's temporary `[workspace]` overlay trick to check/test the relocated crate in
isolation before the registrar's member-line swap lands (copy `[workspace.package]`,
`[workspace.dependencies]` rows your extension actually uses, lints, and profile blocks verbatim from root
`Cargo.toml`, plus the `cargo-features = ["trim-paths"]` line above `[workspace]`). Then:

| # | Command | Notes |
|---|---|---|
| 1 | `cargo check --manifest-path <new>/Cargo.toml` | no `DEVELOPER_DIR` needed |
| 2 | `cargo clippy --manifest-path … --all-targets -- -D warnings` | |
| 3 | `DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --manifest-path …` | must match the extension's pre-move test count exactly (pure relocation) |
| 4 | `cargo check --manifest-path … --target wasm32-wasip2` | if the extension builds for the component-guest feature |
| 5 | *(hybrid only)* `cd <new rust package dir> && bun ./📜️script.ts wasm` | confirms the wasm-pack build still runs and writes `pkg/` with the frozen name; harmless to run pre-registrar since it only writes inside the new crate's own dir |
| 6 | delete the overlay + nested `target/`/`Cargo.lock` | same as `📋️TEMPLATE.md` §8.3, do this LAST |

Report the registrar handoff (§4) and, if applicable, the workspaces-array addition (§3) in your final
message; do not apply the Cargo.toml edit yourself.
