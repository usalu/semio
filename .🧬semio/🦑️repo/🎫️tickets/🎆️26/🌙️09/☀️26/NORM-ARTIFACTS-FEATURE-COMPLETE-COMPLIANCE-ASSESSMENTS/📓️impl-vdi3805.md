# Impl — VDI 3805 (`🏭️vdi3805`)

Wave C/D + Round 2 verifier fixes for the manufacturer product-data exchange guideline.

## Final subject schema (sketch)

```text
Vdi3805Snapshot
├── manufacturerFile / catalog.file
├── catalog.products[id=…]
│   ├── id (= articleNumber)
│   ├── identity / title / sheet / records[]
│   ├── configuration.attributes : SheetAttributes
│   │   ├── valveHeating | radiator | pumpHeating | heatGenerator
│   │   └── generic { entries[] { key, value, unit? } }  # mirrors native 210
│   ├── accessories[] { accessoryId, required, quantity }
│   └── components[] { componentId, quantity }
├── geometry / curves / editionProfile / correctionAsOf / index / limits
```

SI: `kvsM3S` in m³/s, lengths in m, power in W, temperatures in °C.

## Round 2 blockers (all fixed)

1. **Sheet routing** — `check_sheet_product` switches on `product.sheet` (2→valve, 3→radiator, 5→pump, 6→heat-gen, else operative). Sync Fail when 210 implies typed attrs but configuration is Generic.
2. **Operative Blatt rules** — mandatory keys + numeric ranges + curve x-monotonicity per published sheet; `part_N` identity Pass removed (returns real Pass/Fail from sheet checks); coverage is one summary N/A (not 55 label-flooding rows).
3. **Typed catalogues + Part 1** — radiator L/H/D; pump DN/head/power; heat-gen flow/return temps; `correctionAsOf` year/month; index product ids; accessories/components link ids.
4. **Writable remedies** — sync/edition/mandatory Fail remedies target typed/generic attribute leaves (sync regenerates 210). CRUD mutation names renamed to approved semantic verbs: `add-`/`remove-`/`change-` (no create/update/delete/replace).
5. **Field meta** — `[]` wildcards for correctionAsOf, index, accessories, components, geometry connections, pump/radiator/heat-gen, generic entries; leaf walk test asserts en+de.
6. **Facets** — GraphQL `Product.id`; `GenericAttributes { entries }`; JSON/TS/proto updated.
7. **Oracle** — Blatt 2/3/5/6 + representative generic mandatory; `every_emitted_path_resolves` on conforming+nonconforming.
8. **Localization** — distinct en+de (no `copy(x,x)` for structure/mandatory why text).

## Check catalogue

| Part | Clause | Check id | Remedy |
|------|--------|----------|--------|
| 1 | 4.1–4.5 | structure / correctionAsOf / index / accessories / components | scalar leaves |
| 2 | 4.2 / 5.1 | dn / kvs / pn / authority / curve | attribute + point y/x |
| 3 | 4.1–4.3 | phi / n / lengthM / heightM / depthM | attribute leaves |
| 5 | 4.1–4.3 / 5.1 | q / eta / DN / head / power / Q-H | attribute + point y |
| 6 | 4.1–4.4 | qn / fuel / flowTempMaxC / returnTempMinC | attribute leaves |
| Operative | 4.1–5.1 | mandatory / range / curve | attribute entry value |
| Sync | 4.0 | attributes ↔ 210 | typed attribute leaf |
| Edition | edition | profile mandatory set | attribute / productGroup |

## DE/EN

VDI 3805 DE-origin only; `annex = De`.

## Examples

| Id | Role |
|----|------|
| conforming / demo | Blatt-2 DN50 valve with 210 + curve-kvs |
| nonconforming | Multi-fail (structure, DN, kvs, historical, …) |

## Mutations (semantic verbs)

`change-manufacturer-file`, `change-limits`, `add-product`, `remove-product`, `change-product-configuration`, `add-geometry`, `remove-geometry`, `change-geometry-parameters`, `add-curve`, `remove-curve`, `change-curve-points`, plus existing `change-*` / `remove-*` / `rename-*` / `resize-*` / `add-geometry-connection`.

## Tests run

`bun nx run @semio-tech/norm-vdi3805-rs:test --skip-nx-cache -- --no-fail-fast` → **Summary [0.696s] 255 tests run: 255 passed, 0 skipped**.

## Remaining gaps

None.
