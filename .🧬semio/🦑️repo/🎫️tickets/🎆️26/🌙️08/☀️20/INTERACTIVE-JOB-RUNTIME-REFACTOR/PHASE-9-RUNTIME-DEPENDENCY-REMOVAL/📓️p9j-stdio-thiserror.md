# P9j Stdio `thiserror` Removal

## Scope

The final six stdio production leaves using generated `thiserror` implementations were converted to owned standard-library contracts:

- B-Rep engine `BrepError`;
- table probability `ProbabilityError`;
- table statistics `StatisticsError`;
- table storage `TabularError`;
- causal inference `CausalError`;
- value fuzzy inference `FuzzyError`.

The pre-change surface contained six derives and 39 variants. Five `#[from]` conversions and five transparent/source-bearing wrappers existed across `StatisticsError` and `CausalError`.

## Contract Preservation

Every old format string is reproduced by an explicit `std::fmt::Display` match. Simple variants implement `std::error::Error` with `source() == None`. The statistics and causal transparent variants return the wrapped error as `source()` and delegate display without adding a prefix. Explicit `From` implementations preserve both statistics conversions and all three causal conversions.

Each owning module now tests every simple display string and `source()` contract. Statistics and causal tests additionally exercise every generated conversion replacement and transparent display/source behavior.

No external type was added to a public signature.

## Dependency Ratchet

After the source rewrite, the production census

```text
rg -n 'thiserror::|use thiserror|#\[(error|from|source|transparent)' ✏️s/🔌️plugins/🗄️stdio --glob '*.rs'
```

returns zero hits. A B-Rep snapshot module-level document says “no `thiserror`” but contains no dependency use. Only after that zero production census was established was `thiserror = { workspace = true }` removed from stdio's Rust manifest.

Direct stdio normal dependencies therefore fall by one and the direct `thiserror` edge is **1 → 0**. Workspace transitive users remain outside this packet.

## Verification

- PDF-inclusive native lib gate: `CARGO_TARGET_DIR=<ticket>/🧪️target-stdio-thiserror cargo check -p semio-s-plugin-stdio --lib --message-format=json` — **exit 0, zero diagnostics** (`📝️p9j-stdio-check-5.json`, `📝️p9j-stdio-check-5.stderr.txt`). Four preceding compiler-guided iterations monotonically exposed and removed the final PDF-only stale synchronous builder wrappers; every iteration had zero non-PDF/P9 diagnostic.
- Wasm lib gate: the same package check with `--target wasm32-unknown-unknown` — **exit 0, zero diagnostics** (`📝️p9j-stdio-wasm-1.json`, `📝️p9j-stdio-wasm-1.stderr.txt`).
- Focused native contract-test command: `cargo test -p semio-s-plugin-stdio --lib error_contract` — test compilation is blocked before execution by **902 unrelated pre-existing stdio test-world diagnostics** and 528 warnings (`📝️p9j-error-contract-debug-1.txt`). Representative blockers are corrupted identifiers/format strings from the earlier await codemod, stale `UiNode` tests, non-async functions with `.await`, and stale async builder tests. No diagnostic names or points to one of the six new contract tests. A release test is not claimed because it reaches the same lib-test compilation wall.
- Direct dependency census: `cargo tree -p semio-s-plugin-stdio -e normal --depth 1` contains no `thiserror` edge (`📝️p9j-stdio-cargo-tree.txt`).
- Repository dependency ratchet: `bun ./📜️script.ts verify dependencies` — **pass**, baseline 238 and current 234 third-party packages; `thiserror` is among four packages removed since baseline (`📝️p9j-dependency-ratchet.txt`).
- Production source/manifest census for `thiserror` derives, imports, and attributes — zero. The sole textual source hit is a B-Rep module document explicitly stating that its errors do not use `thiserror`.
- All six owned sources were formatted with `rustfmt --edition 2021`; scoped `git diff --check` is clean.

## Files

- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🎲️probability-internals/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📊️statistics-internals/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📋️tabular-internals/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️value/🧬️schema/🌫️fuzzy-internals/🦀️component.rs`
- this report.

The Phase-9 ticket remains open as required.
