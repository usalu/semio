# P9s Stdio Async-Trait Removal

## Outcome

Stdio no longer directly or transitively depends on `async-trait`. The only use was attached to `BrepKernel` and its sole `Brep` implementation even though every trait method and implementation method was already synchronous. The import, both `#[async_trait(?Send)]` attributes, and the direct Cargo dependency were removed. The contract documentation now identifies the kernel interface as synchronous.

No compatibility adapter or boxed-future surface was added.

## Contract census

The repository census in `📝️p9s-stdio-async-trait-census.txt` establishes:

- zero `async_trait`, `async-trait`, or `#[async_trait]` source/manifest matches under stdio;
- zero `dyn BrepKernel`, `Box<dyn BrepKernel>`, or `Arc<dyn BrepKernel>` consumers;
- exactly one trait declaration and one concrete implementation;
- remaining `BrepKernel` text outside the declaration/implementation consists only of ownership documentation.

`📝️p9s-stdio-async-trait-dependency-tree.txt` records the direct normal dependency tree with no `async-trait` row. The reverse normal tree reports that package is not present in stdio's dependency graph at all.

## Gates

All checks used the existing isolated ticket target `🧪️target-stdio-thiserror` with `CARGO_INCREMENTAL=0`.

- Rustfmt check of the B-Rep engine source: exit 0 (`📝️p9s-stdio-async-trait-rustfmt.txt`).
- Native: `cargo check -p semio-s-plugin-stdio --lib --message-format=json` — exit 0, zero diagnostics (`📝️p9s-stdio-native.json`, `📝️p9s-stdio-native.stderr.txt`, empty `📝️p9s-stdio-native-errors.tsv`).
- Release: `cargo check -p semio-s-plugin-stdio --lib --release --message-format=json` — exit 0, zero diagnostics (`📝️p9s-stdio-release.json`, `📝️p9s-stdio-release.stderr.txt`, empty `📝️p9s-stdio-release-errors.tsv`).
- Wasm: `cargo check -p semio-s-plugin-stdio --lib --target wasm32-unknown-unknown --message-format=json` — exit 0, zero diagnostics (`📝️p9s-stdio-wasm.json`, `📝️p9s-stdio-wasm.stderr.txt`, empty `📝️p9s-stdio-wasm-errors.tsv`).

## Files

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs`
