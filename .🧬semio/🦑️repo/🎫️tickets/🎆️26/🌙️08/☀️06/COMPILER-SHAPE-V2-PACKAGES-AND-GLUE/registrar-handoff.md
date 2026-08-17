# Registrar Handoff — Compiler Shape V2

## Workspace members (`Cargo.toml` at repo root)

**Remove** these six entries:

- `🧰️framework/🔨️modules/📚️compiler/⚡️implementations/🦀️rust`
- `🧰️framework/🔨️modules/📚️compiler/📖️syntax/⚡️implementations/🦀️rust`
- `🧰️framework/🔨️modules/📚️compiler/🌍️world/⚡️implementations/🦀️rust`
- `🧰️framework/🔨️modules/📚️compiler/🔤️text/⚡️implementations/🦀️rust`
- `🧰️framework/🔨️modules/📚️compiler/🧮️math/⚡️implementations/🦀️rust`
- `🧰️framework/🔨️modules/📚️compiler/📤️svg/⚡️implementations/🦀️rust`

**Add** one entry:

- `🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust`

## Path dependency swaps (downstream `Cargo.toml`)

| Consumer | Key | Old path | New path |
|----------|-----|----------|----------|
| `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/⚡️implementations/🦀️rust/Cargo.toml` | `compiler` | `../../../../../../../🔨️modules/📚️compiler/⚡️implementations/🦀️rust` | `../../../../../../../🔨️modules/📚️compiler/📦️packages/🦀️rust` |

Package name stays `semio-framework-compiler`; lib name stays `compiler`.

## Removed crates (no longer workspace members)

- `semio-framework-compiler-syntax`
- `semio-framework-compiler-world`
- `semio-framework-compiler-text`
- `semio-framework-compiler-math`
- `semio-framework-compiler-svg`

Sub-slots are `compiler::syntax`, `compiler::world`, `compiler::text`, `compiler::math`, `compiler::svg` inside the unified crate.

## Nx / launch (if any target still pointed at per-slot `⚡️implementations/🦀️rust`)

Point compiler build/test targets at `📚️compiler/📦️packages/🦀️rust` only.

## Verification (ticket harness)

From `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/COMPILER-SHAPE-V2-PACKAGES-AND-GLUE/verify/`:

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-compiler
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test -p semio-framework-compiler
```

After registrar applies root workspace + infinite-canvas path:

```bash
DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo check -p semio-framework-compiler
```


## Registrar applied (2026-08-06)
- Root members: removed 6 implementations, added packages/rust.
- Retargeted Cargo.toml consumers off compiler implementations.
- Retargeted compiler package dsl deps to os-kernel packages path.


## Registrar completion (2026-08-06)
- Members swapped; canvas path retargeted; single kernel dep as dsl_core.
- Kernel aliases dsl_core/dsl_grammar/dsl_notation added.
- Closed; remaining kernel reds owned by OS-IMPLEMENTATIONS-FULL-ERADICATION.
