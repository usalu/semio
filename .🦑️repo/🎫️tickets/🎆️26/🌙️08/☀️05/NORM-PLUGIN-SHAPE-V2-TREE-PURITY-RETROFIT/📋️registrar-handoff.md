# 📋️ Registrar handoff — 📕️norm

The migrating agent never edits root `Cargo.toml`/`Cargo.lock`, the registry generator, `launch.json` or
any other plugin. Everything below is for the serialized registrar pass.

## 1. Root `Cargo.toml` — members

**Remove all 107 lines** currently at lines **274–380** (a single contiguous block — every line matching
`"✏️s/🔌️plugins/📕️norm/…/⚡️implementations/🦀️rust"`; the exact list with its current line numbers is in
this folder's `🧾️registrar-member-lines.txt`). They are:

- `🔨️modules/🫀️core/⚡️implementations/🦀️rust` (×1)
- `🎛️apps/<E><variant>/⚡️implementations/🦀️rust` (×15)
- `🎛️apps/<E><variant>/🔨️modules/{⚙️engine,🗣️dsl,🔧️op,🎒️pack,📡️protocol,🖱️ui}/⚡️implementations/🦀️rust` (×90)
- `🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust` (×1)

All 107 directories are already deleted from disk.

**Add** (one line, same alphabetical slot):

```
    "✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust",
```

## 2. Root `Cargo.toml` — `[workspace.dependencies]`

**Remove all 17 entries** (lines 479–494 + 507). Verified: after this ticket's cross-cutting fix (§4)
**no crate anywhere in the repo still references any of them**, so all 17 are dead keys.

```
semio-s-app-norm-din16798        semio-s-app-norm-en1993        semio-s-app-norm-en1998
semio-s-app-norm-din18599        semio-s-app-norm-en1994        semio-s-app-norm-en1999
semio-s-app-norm-din4108         semio-s-app-norm-en1995        semio-s-app-norm-iso16757
semio-s-app-norm-en1990          semio-s-app-norm-en1996        semio-s-app-norm-vdi3805
semio-s-app-norm-en1990-engine   semio-s-app-norm-en1997        semio-s-plugin-norm-core
semio-s-app-norm-en1991
semio-s-app-norm-en1992
```

Do **not** add a `[workspace.dependencies]` entry for the new crate — nothing depends on norm by
workspace key; `dsl/🧪️fixture-sweep` uses a plain `path` + `package` pair (§4).

## 3. Package identity — unchanged

`name = "semio-s-plugin-norm"`, `[package.metadata.component] package = "semio:norm"`, and all fifteen
`[[package.metadata.semio.playground]]` blocks (variants din4108/din16798/din18599/en1990–en1999/
iso16757/vdi3805, apps `norm-*-play`, react ports 6091–6105, wgpu ports 6191–6205) are **byte-identical**
to the old bundle crate. `launch.json` regeneration is a no-op. The one new key is
`[package.metadata.semio] role = "plugin"`. Norm's old bundle declared no `aliases` and no `assets`; none
were invented.

## 4. Cross-cutting file already edited by this ticket

`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/`

- `Cargo.toml`: the fifteen `semio-s-app-norm-*` dev-dependency aliases collapsed into one
  `norm = { path = "…/📕️norm/📦️packages/🦀️rust", package = "semio-s-plugin-norm" }`.
- `📦️lib.rs`: the fifteen `use <variant>::Document as <X>Document;` lines repointed to
  `use norm::artifacts::<variant>::Document as <X>Document;`.

Every norm `Document` is genuinely plugin-owned (grep-verified per artifact, not inferred from naming:
14 are a `pub struct Document` defined in norm's own artifact component; 📙️din18599's is
`pub type Document = BalancingInputs;` aliasing a struct defined two lines above it in the same file —
none resolve into a framework kernel crate, so there is no `flow`/`dag`-style name-collision trap here).
**Not yet compile-verified** — needs a healthy root workspace.

## 5. ⚠️ Cross-plugin dependency on 🏗️fem — needs a follow-up repoint

`🗿️artifacts/📘️en1992/⚙️engine` and `🗿️artifacts/📘️en1993/⚙️engine` solve a simply supported beam with the
FEM plugin's shared kernel before running their ULS checks (`fem_core::{BeamEb2, Dof, MemberUdl, Model,
Node, Support, StaticResult, ElementResult, FemError, solve_linear_static}`). This is a **real, pre-existing
cross-plugin dependency**, not something this migration introduced.

It is still pointed at fem's OLD crate (`semio-s-plugin-fem-core`, still a live workspace member):

```toml
fem_core = { path = "../../../🏗️fem/🔨️modules/🫀️core/⚡️implementations/🦀️rust", package = "semio-s-plugin-fem-core" }
```

It was deliberately NOT repointed at fem's new `semio-s-plugin-fem` crate because 🏗️fem's own migration is
still in flight (ticket open, old crates still on disk and still workspace members) and its new crate
could not even resolve at verification time (blocked by an unrelated in-flight `🧰️framework/🔨️modules/🖱️ui`
move). **When 🏗️fem's registrar pass deletes its old crates, norm's dep breaks** — the fix is a 3-line
forward-fix, all inside norm:

1. `📦️packages/🦀️rust/Cargo.toml`: `fem = { path = "../../../🏗️fem/📦️packages/🦀️rust", package = "semio-s-plugin-fem" }`
2. + 3. In both engine component files: `fem_core::` → `fem::core::` (verified: fem's new
   `🫀️core/🦀️component.rs` re-exports every symbol norm uses, including `BeamEb2` via
   `pub use crate::core::elements2d::{Bar2, BeamEb2};`).

## 6. Commands still un-run (need a healthy root workspace)

- `cargo metadata` / `cargo check --workspace`
- `cargo check -p semio-framework-os-kernel-dsl-fixture-sweep --tests` (proves §4 compiles)
- `bun 🧰️framework/…/📇️registry/📜️script.ts check` and `… generate`
- `bun nx run @semio-tech/framework-os-dev:plugin -- norm` (the real pipeline; the manual equivalent —
  `cargo build --target wasm32-wasip2 --profile wasm-release` + `jco transpile` — **did** run green here)
- `bun ./📜️script.ts dev norm`, `bun ./📜️script.ts verify gate`

At handoff time the root workspace was already red for an unrelated reason: `📜️imperative`'s old member
lines dangle (another session's in-flight migration).
