# Manifest Dependency Validation Privacy Acceptance

## Result

- Status: source-complete / not-green.
- Source: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`.
- Baseline SHA-256: `92fbd693b8fb1f983c9c934381e531b516ac61e7e050662bb6a20a2dae893673`.
- Final SHA-256: `388f8dfb960608361ea0534c3b295af5ec1d570b2b919251b9561f379dd69e0d`.

## Change

- Made `validate_dependency_graph` private by changing its sole declaration from `pub fn` to `fn`.
- Preserved the external ordinary `crate::ArtifactDialect::export().unwrap();` insertion unchanged.
- Cached scoped diff was empty before and after the edit.

## Validation

- `git diff --check -- <source>` completed without output.
- The declaration is exactly `fn validate_dependency_graph`.
- No external Rust code calls `validate_dependency_graph`; `resolve_load_order` remains its owner-local caller.
- The scoped ordinary diff contains exactly the one-line visibility change and the pre-existing external dialect export insertion.
- `bun nx run @semio-tech/framework:test --skip-nx-cache` is queued and intentionally not started: active shared Cargo/rustc processes would contend on the Cargo build directory.
