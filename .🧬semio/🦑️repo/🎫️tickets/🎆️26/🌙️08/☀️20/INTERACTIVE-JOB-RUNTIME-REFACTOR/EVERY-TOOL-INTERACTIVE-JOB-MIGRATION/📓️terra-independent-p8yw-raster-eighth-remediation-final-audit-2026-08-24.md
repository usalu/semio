# P8yw Raster Eighth Remediation Independent Final Audit

## Verdict

**RED — permanent-proof ordering blocker.** The live hostile fixture correctly proves the two rejected backing identities. The permanent Raster predicate does not prove the required complete ordering: it omits the returned-binding ordering and both assertion-before-retirement orderings. Its successful self-test consequently cannot close that proof gap. No production source was edited, and P2a1 was not started.

## Live Fixture Evidence

The parameter path is materially repaired in `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs:4817-4827`: it names a nonempty `String`, captures its backing at line 4818, moves that exact owner through `DslValue::String(plus_one_param_value)` into the rejected capacity insertion at line 4819, binds the returned `DslValue::String` at lines 4821-4824, and compares that binding's pointer at line 4825. Only then does line 4827 move the rejected key/value pair into `RasterOwnedRetirement`.

The asset path has the analogous live sequence at `.../🦀️component.rs:4854-4863`: it names the actual `RasterAssetChild`, captures `child_id` backing at line 4858, moves the child into the saturated insert at line 4859, and compares the handed-back child backing at line 4861. Its retained retirement transfer is later, at line 4863. Thus neither key-only identity, reconstruction, nor pre-retirement deep destruction can satisfy the fixture source itself.

The seventh public closure remains structurally intact: one O(1) snapshot-shell condition, two public codec preflights, four empty-map guards, two empty-layer guards, and exactly eight mounted serializer guards/callers. The sixth empty-only serde closure remains: no `RasterOwnedMap: Serialize`, no length-based map serializer, no per-entry serializer, and exactly three field guards. Earlier retained ownership, page/control, saturation retry, depth, cancellation, acknowledgement, and fail-closed clone/DSL checks remain represented in the predicate.

## Blocking Permanent-Predicate Defect

`toolJobRasterEnvelopeCallerRetainedExact` captures only three position values for each payload at `📜️script.ts:1797-1805`. Its actual ordering requirements are only:

- parameter capture `<` insertion and parameter assertion `>` insertion (`:1892-1896`);
- asset capture `>` the preceding parameter assertion, asset insertion `>` asset capture, and asset assertion `>` asset insertion (`:1897-1900`).

The parameter returned binding is only an unordered `includes("let rejected_param_value = match &rejected_param.value")` at line 1895. There is no `indexOf` for it and no condition `insert < returned binding < assertion`. The asset has no separately identified returned-child binding at all, only the `rejected_asset` insertion binding and direct field assertion.

More importantly, the predicate has no position for either `RasterOwnedRetirement::new(RasterRetirementOwner::ValueEntry { key: rejected_param.key, ... })` or `...AssetEntry { key: rejected_asset.key, ... }`. It therefore cannot require either pointer assertion to occur before the exact owner moves to retained retirement. This leaves the stated capture → moved insert → returned binding → backing assertion → retained-retirement proof unencoded. A source mutation that preserves the required tokens while relocating a binding or assertion outside the required fixture ordering can still pass this predicate.

The four eighth self-tests at `📜️script.ts:4285-4308` faithfully kill deletion and key-only substitution of both assertions, and the verifier self-test passed. They do not mutate either returned binding's location or an assertion's location relative to its retained handback, so they cannot demonstrate the missing ordering requirements.

Required repair: make the predicate inspect the hostile-fixture span and require, for each pair, capture < exact moved insertion < returned binding < backing assertion < corresponding `RasterOwnedRetirement::new` transfer. Add faithful self-test mutations that move/remove the returned binding and move the backing assertion after retirement (or otherwise break each ordering), and require each mutation to fail.

## Scoped Gates

| Gate | Result |
|---|---|
| Scoped `rustfmt --check --edition 2021` on changed Raster Rust files | PASS |
| Scoped `git diff --check` on changed Raster files and `📜️script.ts` | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS — `self-tests=328 clean` |
| Live predicate | Expected global RED — 884 unrelated live command registrations and other global categories; it emitted no Raster predicate failure, which is not acceptance because the Raster predicate has the ordering gap above |
| Cargo / Nx / Wasm / browser / runtime / network / broad build | Not run by instruction |
