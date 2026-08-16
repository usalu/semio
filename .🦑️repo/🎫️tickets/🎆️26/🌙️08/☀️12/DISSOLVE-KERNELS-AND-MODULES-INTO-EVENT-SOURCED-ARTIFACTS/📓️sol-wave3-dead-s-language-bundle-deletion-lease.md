# Dead Language Bundle Deletion Lease

## Status

Prepared on 2026-08-16 from read-only census, source-referrer, filesystem, and offline Cargo-metadata evidence. No source, manifest, registrar, lockfile, or generated output was changed while preparing this packet.

## Disposition

Delete the zero-consumer `✏️s/🔨️modules/🗣️lang` bundle. It is not a reusable module: no live production component imports its crate or its unique `session_for_uri` API, and Cargo does not include it in the workspace. Its four public re-exports are direct Framework OS language APIs; no production consumer requires a move, alias, or compatibility layer.

| Field | Evidence |
| --- | --- |
| Census component | `✏️s/🔨️modules/🗣️lang` |
| Census disposition | `delete` |
| Resolved terminal production consumers | `0` |
| Root package | `semio-s-language-bundle` |
| Cargo workspace membership | Absent from `cargo metadata --offline --no-deps --format-version 1`; observed workspace has 99 members. |
| Active source referrers | None for the exact source path, package name, crate identifier `s_language_bundle`, or unique `session_for_uri` API. |
| Runtime/registration/mount | None. There is no `📋️project.json`, package script, generator input, registry entry, root Cargo member, root workspace dependency, or launch entry for this package. |

Historical plans and ticket records describe the old package, but are excluded evidence consumers. They must not cause a source retention or replacement facade.

## Lease Boundary

| Role | Exact path |
| --- | --- |
| Delete | `✏️s/🔨️modules/🗣️lang/📦️packages/🦀️rust/Cargo.toml` |
| Delete | `✏️s/🔨️modules/🗣️lang/📦️packages/🦀️rust/📦️glue.rs` |
| Remove if empty | `✏️s/🔨️modules/🗣️lang/📦️packages/🦀️rust`, `✏️s/🔨️modules/🗣️lang/📦️packages`, `✏️s/🔨️modules/🗣️lang` |
| Retain | `✏️s/🔨️modules/AGENTS.md` and every sibling module. |

The package has no nested `AGENTS.md`. Its direct parent instruction applies and must be reread immediately before deletion.

## Prepared Hashes

```text
5123700f05c794152e1a9c748de9f14adb074b3ade7263dd427127d3f06d07ee  ✏️s/🔨️modules/🗣️lang/📦️packages/🦀️rust/Cargo.toml
816a0b8a43e098325e4baf29d25479bcfc9ee75761f92ba82299abcd1a6792a0  ✏️s/🔨️modules/🗣️lang/📦️packages/🦀️rust/📦️glue.rs
```

The executor must rehash both files and rerun the active-source referrer sweep before changing them. It must stop and return a new coordinator request if either hash or the zero-referrer set changes.

## Why Deletion Is Correct

The package is only a facade:

```rust
pub use dsl_lsp::{handle_json_rpc, LanguageSession};
pub use dsl::{language, language_for_extension, register_language, LanguageRole, LanguageSpec};
pub fn session_for_uri(...) -> Option<LanguageSession> { ... }
```

The facade has no clients. Retaining it violates the zero-production-consumer disposition; relocating it would preserve an unnecessary wrapper and falsely create a module. The Framework OS owner continues to own the underlying language contract unchanged.

## Explicit Exclusions

- Current repo CLI command lease: `🧰️framework/🛍️products/🦑️repo/🎮️commands/**` and CLI Rust glue.
- Current print SCC: `🧰️framework/🛍️products/📓️print/**`.
- Current stdio registry/runtime and its framework-plugin capability quarantine: `✏️s/🔌️plugins/🗄️stdio/**` and the protected framework-plugin pair.
- The separate repo `🔨️modules` collection frontier, root `Cargo.toml`, `Cargo.lock`, root `📜️script.ts`, taxonomy/discovery SSOT, package locks, and `.vscode/launch.json`.
- No Git state operation or external-HEAD remediation. The concurrent external advance to `dbcc4fa462` is handled only by the pre-change content hashes above.

## Required Verification

Run from the workspace root after the deletion:

```text
rg -n -F --hidden --glob '!node_modules/**' --glob '!target/**' --glob '!dist/**' --glob '!build/**' --glob '!Cargo.lock' --glob '!📦️bun.lock' '✏️s/🔨️modules/🗣️lang' 🧰️framework ✏️s 📜️script.ts Cargo.toml .vscode
rg -n -F --hidden --glob '!node_modules/**' --glob '!target/**' --glob '!dist/**' --glob '!build/**' --glob '!Cargo.lock' --glob '!📦️bun.lock' 'semio-s-language-bundle' 🧰️framework ✏️s 📜️script.ts Cargo.toml .vscode
rg -n --hidden --glob '!node_modules/**' --glob '!target/**' --glob '!dist/**' --glob '!build/**' --glob '!Cargo.lock' --glob '!📦️bun.lock' '\\bsession_for_uri\\b' 🧰️framework ✏️s 📜️script.ts Cargo.toml .vscode
cargo metadata --offline --no-deps --format-version 1
bun ./📜️script.ts verify taxonomy report --scope ✏️s/🔨️modules
git diff --check -- ✏️s/🔨️modules/🗣️lang
```

The first three commands must yield no active-source referrer. Cargo metadata must continue to omit `semio-s-language-bundle`. The scoped taxonomy report may retain unrelated collection-frontier findings until the separate `✏️s/🔨️modules` manifest lease runs; it must contain no remaining `🗣️lang` entry. There is no Nx or runtime target to invoke because the deleted crate is neither a workspace package nor a registered executable.

## Handoff

This is a conflict-free Terra deletion lease. It requires no central registrar request. On successful source-deletion and verification, record the exact command output in a unique completion Markdown file and release the paths. Do not create a replacement module, package, alias, adapter, migration, or manifest row.
