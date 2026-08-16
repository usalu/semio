# Neural DAG Flow Host Dissolution Acceptance

## Baseline

- HEAD: `07873f842a5a99ac2f69c1648c21f36ebf260bdb`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/🦀️component.rs` was clean at SHA-256 `190623e4b08666e6a980b29b1cd7c30c90cff7a73985781e314912bd3dc25b3f`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs` was clean at SHA-256 `10a0c637d57a164d1b4758fb24a54ad3603ebbfda851072e43c4975964d5ce86`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` was clean at SHA-256 `2a3850509a6159f08acb0f6c3a439ff67b9800bb66492fc912ec1d6f4826b048`.
- The only production adapter call was `FlowHost::build_tree`; its glue mount was the sole module assembly reference.

## Implementation

- Deleted `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/🦀️component.rs`.
- Moved `tree_from_dag` and its private property-value, property-bag, and cluster-tree helpers into the nested `🌳️TreeBuilding` / `🔗️DagTreeConversion` region of `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs`.
- `FlowHost::build_tree` now invokes its private local conversion. The moved conversion test now exercises `FlowHost::tree_from_dag` from the host test module.
- Removed the `neural_dag` mount from `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📦️glue.rs`.
- The deleted adapter’s dead `wire_rows_from_dag_fixture_json` parser and `NeuralDagError` were not retained. No compatibility alias was introduced.

## Validation

- Repository-wide `rg -n 'neural_dag|wire_rows_from_dag_fixture_json|NeuralDagError' --glob '!target/**' --glob '!node_modules/**' --glob '!**/.nx/**'` returned no matches.
- Both ordinary and cached scoped `git diff --check` validations exited `0`.
- `bun nx run semio-framework-os-flow-core:test-quick --skip-nx-cache` exited `1`. The runner was healthy; the target stopped in unrelated `semio-s-plugin-stdio` compilation with `10` `E0432` unresolved-import errors and `663` warnings. No failure diagnostic named the moved Flow host conversion or deleted adapter.
- `bun ./📜️script.ts verify taxonomy report --scope framework.product.os.flow` was accepted by the command router but continued scanning the active graph without a report after `60` seconds. It was stopped as an unsafe broad census under concurrent work; `enforce` was not run. No zero-component result is treated as a clean result.

## Final State

- Adapter path is absent.
- Flow host SHA-256: `4781c14579ea620ebdca1d8ba1d0a2ab2192305ea772089094e6236cc93a9850`.
- Flow Rust glue SHA-256: `f1a3035ebc461ea5bf2b6157855d2a432c31222dd1ba8593da16238f0d64fa98`.
- Cached source diff is exactly: glue `0` additions / `6` deletions; host `73` additions / `1` deletion; adapter `0` additions / `132` deletions.
- The three leased paths appeared index-staged after the edit despite being clean before it. No Git-mutating command was issued; the externally controlled index state was preserved.
