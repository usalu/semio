# Impl — VDI 3805 (`vdi3805` / 🏭️vdi3805)

Wave C fixer (fresh) against Round-4 `📓️verify-vdi3805.md` FAIL (1). Suite was 282/282 + contract 51/51; German titles and `catalog.file.*` paths left untouched.

## Round 4 residual (FAIL 1 → closed)

Dangling `accessoryId` / `componentId` (including after perturbation to `__dangling__`) now emit `Remedy::one_of` with existing catalogue **product ids** — same pattern as index `productId` and `geometryRef` — not `Remedy::exactly` with a 0→1 placeholder.

| Site | Change |
|------|--------|
| Evaluate accessories / components | `Remedy::one_of(..., product_ids, …)` with en≠de action copy |
| `validate_structure` accessory/component integrity | Compare against `product.id` set (was `article_number`) so structure Fail and evaluate Fail agree on the same identifier space |
| `check_part1` structure branch | `accessoryId` / `componentId` paths take `one_of` product ids instead of falling through to `exactly` |

Test: `dangling_accessory_and_component_ids_fail_with_one_of_existing_product_ids` asserts Fail + non-empty `one_of` options containing an existing product id for both accessory and component.

Not reintroduced: `manufacturerFile.*` evaluate paths, bare-`id` perturbation exemption, fingerprints, `param_metric`, flange default.

## Final subject schema (sketch)

```text
Vdi3805Snapshot
├── catalog.file          # single stored header (diff may carry sparse manufacturerFile → apply onto catalog.file)
├── catalog.products[id=…]
│   ├── id (= articleNumber)
│   ├── identity / title / sheet / records[]
│   ├── configuration.attributes : SheetAttributes
│   │   ├── valveHeating | radiator | pumpHeating | heatGenerator
│   │   └── generic { entries[] { key, value, unit? } }
│   ├── accessories[] { accessoryId → product.id, required, quantity }
│   └── components[] { componentId → product.id, quantity }
├── geometry / curves / editionProfile / correctionAsOf / index / limits
```

SI: `kvsM3S` in m³/s, lengths in m, power in W, temperatures in °C.

## Prior Wave D Round 3 residuals (still closed)

1. Perturbation: bare `id` not exempt; refs perturb to `__dangling__`.
2. Distinct German SubjectRef titles (`Nennweite`, `Durchflusskoeffizient kvs`, …).
3. Structure diagnostics/remedies emit `catalog.file.*` only.

## Gaming removal (14:37 / 14:42)

Guard: `check_sources_contain_no_fingerprint_gaming_patterns`. No `param_metric` / `pos_metric` / fingerprint folds in evaluate source.

## DE/EN

VDI 3805 DE-origin only; `annex = De`. English and German remedy/explanation strings stay different words.

## Runner

`bun nx run @semio-tech/norm-vdi3805-rs:test --skip-nx-cache -- --no-fail-fast`
→ **Summary [2.942s] 283 tests run: 283 passed, 0 skipped**

`bun nx run @semio-tech/norm-artifact-contract-rs:test --skip-nx-cache`
→ **Summary [0.107s] 51 tests run: 51 passed, 0 skipped**

## Remaining gaps

None.

## Requests to coordinator

None.
