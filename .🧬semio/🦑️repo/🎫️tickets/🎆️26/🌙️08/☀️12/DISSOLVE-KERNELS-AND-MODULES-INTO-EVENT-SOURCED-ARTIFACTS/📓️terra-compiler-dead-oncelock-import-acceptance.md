# Compiler Dead OnceLock Import Acceptance

## Change

Removed only the unused top-level `use std::sync::OnceLock;` from `🧰️framework/🔨️modules/📚️compiler/🦀️component.rs`.

The live nested import and `static FONTS: OnceLock<Fonts>` remain in `fonts()`.

## Preflight

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- `shasum -a 256` for the compiler component source: `0be70e2393330cb88d6bf77599e080d9ef42c7c008cc845d86fa2160cf34bff2`.
- Scoped status and ordinary/cached numstat for `🧰️framework/🔨️modules/📚️compiler` were empty before the change.
- Final compiler component source SHA-256: `bb50a90ecbe3bff739435090dad438ae6201246ba2d999ad4eb47d23d25d0182`.

## Validation

- Static check finds no top-level `OnceLock` import.
- Static check confirms the nested `OnceLock` import and live `FONTS` static remain.
- Scoped ordinary diff contains exactly the two deleted top-level-import lines; cached diff is empty.
- Structural exception: the compiler component and its Rust package have no `project.json`, and no package script references `semio-framework-compiler`; validation therefore used direct package Cargo.

## Final Validation

- Final `shasum -a 256 🧰️framework/🔨️modules/📚️compiler/🦀️component.rs`: `bb50a90ecbe3bff739435090dad438ae6201246ba2d999ad4eb47d23d25d0182`.
- Final ordinary compiler diff is exactly the two-line top-level-import deletion; final cached compiler diff is empty.
- `bun nx show projects | rg -i compiler` found no registered compiler project (`nx` exit `0`, filter exit `1`), so no Nx gate is available.
- Exact fallback command: `cargo check --manifest-path 🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/Cargo.toml`.
- Result: blocked while compiling external `semio-framework-os-kernel` by moving SPR/store work. The first relevant blockers are `HistoryLog` missing `conflicts` at store component line `2521` and `reconcile_with_last` not found at store component line `4598`.
