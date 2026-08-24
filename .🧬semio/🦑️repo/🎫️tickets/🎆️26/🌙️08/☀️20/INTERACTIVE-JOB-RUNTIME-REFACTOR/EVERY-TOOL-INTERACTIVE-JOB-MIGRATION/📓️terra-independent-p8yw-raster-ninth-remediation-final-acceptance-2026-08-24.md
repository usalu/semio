# P8yw Raster Ninth Remediation Independent Final Acceptance

## Verdict

**GREEN — the ninth verifier-only ordering repair closes the eighth permanent-proof blocker.** The permanent predicate is confined to the exact hostile populated-snapshot-output fixture and requires the full ordered ownership chain for both rejected payloads. The requested static verifier self-test passed, including the prior removal/key-only checks and all four ordering mutations. The live Raster predicate passed; the enclosing tool-job command remains globally RED only for unrelated migration failures. P2a1 was not started.

## Reviewed Inputs

- Repository `AGENTS.md`.
- Eighth independent RED audit: `📓️terra-independent-p8yw-raster-eighth-remediation-final-audit-2026-08-24.md`.
- Ninth remediation handoff: section `Ninth independent remediation` in `📓️p8yw-raster-retained-envelope-ingress-2026-08-23.md`.

## Exact Fixture-Scoped Proof

`toolJobRasterEnvelopeCallerRetainedExact` in `📜️script.ts:1797-1818` first takes the source span from `fn raster_populated_snapshot_output_max_plus_one_nested_cancel_fault_panic_and_close_are_exact()` to the next `fn raster_maximum_combined_layer_and_value_depth_retires_to_terminal()`. All ten position tokens below are resolved only from that span; tokens elsewhere cannot satisfy the predicate.

| Rejected owner | Capture | Moved insertion | Returned binding/access | Exact backing assertion | Exact retirement transfer |
| --- | --- | --- | --- | --- | --- |
| Parameter value | `let plus_one_param_value_pointer = plus_one_param_value.as_ptr();` | `params.insert(plus_one_param_key, dsl::DslValue::String(plus_one_param_value))` | `let rejected_param_value = match &rejected_param.value` | `assert_eq!(rejected_param_value.as_ptr(), plus_one_param_value_pointer, ...)` | `RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, value: Some(rejected_param.value) })` |
| Asset child | `let plus_one_asset_child_pointer = plus_one_asset_child.child_id.as_ptr();` | `assets.insert(plus_one_asset_key, plus_one_asset_child)` | `rejected_asset.value.child_id.as_ptr()` in the actual identity assertion | `assert_eq!(rejected_asset.value.child_id.as_ptr(), plus_one_asset_child_pointer, ...)` | `RasterOwnedRetirement::new(RasterRetirementOwner::AssetEntry { key: rejected_asset.key, child: Some(rejected_asset.value) })` |

The predicate's strict comparisons at `📜️script.ts:1901-1910` require, independently for each row: capture `<` moved insertion `<` returned binding/access `<` pointer assertion `<` matching retained-owner transfer. The live fixture supplies those exact sites at `.../🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:4818-4827` (parameter) and `:4858-4863` (asset).

## Mutation Evidence

The existing parameter and asset identity-removal and key-only substitution mutations remain at `📜️script.ts:4300-4311` and `:4323-4334`. The new order-specific mutations at `:4312-4322` and `:4335-4344` are all invoked against the same predicate:

- parameter returned binding before moved insertion;
- parameter retirement transfer before pointer assertion;
- asset returned-child access/assertion before moved insertion; and
- asset retirement transfer before pointer assertion.

The self-test completed clean, so every one of those mutations was killed.

## Preserved Raster Invariants

Current live source still has the populated-output fail closure: one O(1) snapshot shell condition, the two public codec preflights, four empty-map guards, two empty-layer guards, and exactly eight mounted serializer guards/callers. It retains the three `serialize_empty_owned_map` field guards and no public `RasterOwnedMap` serializer, whole-map serializer, or entry serializer. `RasterOwnedMap::remove_entry` remains the ownership-preserving removal API; the ownership source still contains the explicit retained `ValueEntry` and `AssetEntry` transfers, including the two fixture transfers above.

The shared worktree has pre-existing concurrent Raster source modifications from earlier work, so Git diff provenance cannot independently separate the ninth edit from that dirty baseline. This acceptance confirms the live source invariants and the verifier's current behaviour; it did not modify production or fixture Rust source.

## Scoped Gates

| Gate | Result |
| --- | --- |
| `git diff --check -- 📜️script.ts ✏️s/🔌️plugins/🖨️raster` | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS — `self-tests=328 clean` |
| Live `bun ./📜️script.ts verify interactivity tool-jobs` Raster predicate | PASS — command exits RED for listed unrelated global categories and 884 remaining registrations; its failure list contains no Raster envelope-predicate failure |
| Cargo / Nx / native / Wasm / browser / runtime / network / broad build | Not run by instruction |
