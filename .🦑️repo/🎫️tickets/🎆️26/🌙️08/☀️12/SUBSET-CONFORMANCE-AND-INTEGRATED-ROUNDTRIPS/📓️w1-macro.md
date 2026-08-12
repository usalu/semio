# W1-MACRO — `subset!` declaration macro

Completed: 2026-08-12. Scope: additive append to `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` only (freeze ledger held-additive).

## Delivered

| Item | Location |
|------|----------|
| `SubsetKind` (`Owning` \| `Derived`) | tail of `🔌️plugin/🦀️component.rs` |
| `subset!` macro — **owning** arm | wraps `derive_artifact_facets!` + idempotent `register_subset()` (composer entries, optional validator, optional extra IO rows, optional `EXAMPLES`) |
| `subset!` macro — **derived** arm | `SUBSET_DIALECT`, `SubsetValidator` OnceLock registration, optional IO rows, optional positive/negative example consts, inline `conformance` tests |
| Crate smoke tests | `#[cfg(test)] mod subset_macro_tests` (derived validator + register idempotency) |

## Macro surface (summary)

**Owning** — one invocation replaces hand-written facet structs + composer registration:

```rust
subset! {
    pub owning dialect "s.plugin.artifact" / "1" / "*" {
        spec MySpec { construction: …, analysis: …, composition: … }
        builder: MyBuilder,
        analyzer: MyAnalyzer,
        composer: MyComposer,
        // optional: io: [extra ComposerEntry, …],
        // optional: validator: MyValidator,
        // optional: examples: [ExampleSource::new(…), …],
    }
}
```

**Derived** — replaces `OnceLock` + `subset_validator_entry_of` + `register_subset_validator` boilerplate:

```rust
subset! {
    pub derived dialect "s.plugin.artifact" / "1" / "valid" {
        validator: MyDerivedValidator,
        // optional: io: […],
        // optional: positive: […],
        // optional: negative: […],
    }
}
```

Each expansion exports `register_subset`, `SUBSET_DIALECT`, `KIND` at the invocation site.

## Verification

Command: `cargo test -p semio-framework-plugin subset_macro`

Result: **blocked** — pre-existing `semio-framework-plugin` compile error unrelated to this diff:

- `E0499` borrow conflict at `🔌️plugin/🦀️component.rs:5790` (`self.children` in space dispatch loop)
- Additional test-only failures (`ExampleDefinition.document_json`, `TutorialBase.document_dsl`) when `--test` is enabled

Macro expansion itself is syntactically valid (repeat/angle-bracket issues fixed during implementation). Re-run `subset_macro` filter after peer fixes the `E0499` site.

## Out of scope (later W1 tasks)

- `store::test_support::assert_subset_roundtrip` harness wiring in macro conformance tests → **W1-HARNESS**
- `IoFidelity` types → **W1-IOFID**
- Taxonomy archetype keys → **W1-TAX**
- Plugin glue generator consuming `subset!` → **W1-GEN**

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (append ~210 lines after `derive_artifact_facets!`)
