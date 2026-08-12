# W1-HARNESS — Integrated subset roundtrip harness

Completed: 2026-08-12. Scope: additive append to `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` `test_support` region only.

## Delivered

| Item | Location |
|------|----------|
| `IoFidelityClass` (`Exact` \| `Canonical` \| `Semantic` \| `Lossy`) | `test_support` |
| `ExampleAsset<'a>` | `test_support` |
| `SubsetRoundtripSpec` trait | `test_support` |
| `assert_import_export_fidelity_bytes` | `test_support` |
| `assert_inference_determinism` | `test_support` |
| `assert_subset_roundtrip<S>` staged driver | `test_support` |
| Unit smoke tests | `#[cfg(test)] mod tests` in same file |

## Stage mapping (v1)

| Stage | Law | Implementation |
|-------|-----|----------------|
| S0 | Non-empty bytes + provenance | Direct asserts on `ExampleAsset` |
| S1 | Dialect pin | `S::dialect()` must have non-empty `artifact_kind` / `standard` / `subset` |
| S2 | Native import | `S::parse_native` — `Err("SKIP:…")` skips remaining stages |
| S3 | DSL/pack twin laws | `assert_dsl_round_trip`, `assert_pack_round_trip`, `assert_dsl_pack_equivalence` |
| S4 | Diff/codec laws | **Deferred** — no trait hook yet |
| S5 | Mutation roundtrips | Per `sample_mutations`: `assert_operation_round_trip`, `assert_op_line_round_trip`, `assert_op_text_binary_equivalence` |
| S6 | Inference determinism | `assert_inference_determinism` on two `infer` calls |
| S7 | Store apply/undo/redo | `assert_store_roundtrip` with first mutation when non-empty |
| S8 | Export/reimport fidelity | `export_native` → `assert_import_export_fidelity_bytes`; `reimport_native` with class-specific snapshot law; lossy drop-path equality deferred |
| S9 | Validator | `validate_payload` Ok; derived + negative → `validate_negative` returns non-empty codes |
| S10 | Dialect migration | **Deferred** — no trait hook yet |

Skip convention: any trait method returning `Err` whose message (or sole validation code) starts with `SKIP:` skips that stage without failing the harness.

`dialect()` returns `crate::os_io::ArtifactDialect` (store's persisted dialect type — no separate `Dialect` reexport).

## Verification

Command:

```bash
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS/🎯️target-w1-harness" \
  cargo test -p semio-framework-os-kernel assert_subset
```

Result: **pass** — `assert_subset_harness_fidelity_and_inference_helpers` ok (1 passed, 822 filtered).

Additional filter:

```bash
cargo test -p semio-framework-os-kernel assert_import_export_fidelity
```

Result: **pass** — `assert_import_export_fidelity_bytes_exact_rejects_divergence` ok (should-panic law).

Full `semio-framework-os-kernel` compile succeeded under ticket target dir; no new errors introduced by this diff.

## Integration notes

- `subset!` macro conformance tests (W1-MACRO) should call `test_support::assert_subset_roundtrip` via a thin per-subset `SubsetRoundtripSpec` adapter once reference subsets land (W3).
- S4 diff/absorption and S10 dialect-migration hooks can extend the trait in a follow-up without breaking callers (default skip via new optional methods or `SKIP:` returns).
- Lossy fidelity with non-empty `drops()` currently requires successful reimport only; path-level drop-set equality is explicitly out of scope for v1.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (~130 lines appended in `test_support` + 3 unit tests)
