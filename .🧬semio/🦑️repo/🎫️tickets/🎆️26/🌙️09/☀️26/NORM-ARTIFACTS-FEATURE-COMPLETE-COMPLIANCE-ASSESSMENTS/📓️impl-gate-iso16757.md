# Impl Gate — ISO 16757 Compliance Remedies

**Family:** `iso16757` (`@semio-tech/norm-iso16757-rs`)  
**Date:** 2026-09-28

## Runner

```
NX_DAEMON=false bun nx run @semio-tech/norm-iso16757-rs:test --skip-nx-cache
```

```
Summary [   2.657s] 313 tests run: 313 passed, 0 skipped
```

## Blocking Failures Cleared

Gate previously reported these `apply_remedy_edit` / `evaluate` Fail sticks. Each now flips off Fail (single remedy or sequential applicables):

| Check | Fix |
|-------|-----|
| `iso16757.1.6.4.searchTags.index-cv50-dup` | `AtLeast`→`Remedy::one_of` with product-token options (writes a real tag, not `"1"`) |
| `iso16757.1.4.2.selection.empty` | OneOf on `selection.constraints[id=…].value.value` from real property values; computed encodes property metric; broken DSL/fixture keep properties + unmatched constraint `999` |
| `iso16757.1.10.bim.index-cv50` / `…-dup` | Err-path remedies OneOf out-of-domain `parameterValues.*.value` (and geometryId); `params.clone()` for resolve |
| `iso16757.2.5.3.5.clearance.geom-valve-50` | Pass when envelope `have ≥ needed` (`axis_ok`); sequential max[] remedies reach u=1 as Pass |
| `iso16757.5.6.10.partNumber` | Three AtLeast remedies (`maxSteps`/`maxRecursion`/`timeoutMs`); sequential raises all zero caps |
| `iso16757.5.8.scriptLimits` | Same three-limit AtLeast set for sequential clear |

## Collateral

- Multilingual / composition-cycle fails: applicable `OneOf` remedies (locale / `componentProductId`)
- Broken DSL: property values restored, constraint `999`, FR name placeholder, ports restored
- No gate weaken, no check deletion, no fingerprint / dummy utilization shortcuts

## Files Touched

- `⚖️checks/📈️part1.rs`, `📐️part2.rs`, `🔄part5.rs`
- `🦀️.rs` (`broken_fixture`)
- `🖼️assets/🚫️broken/🗣️.dsl.semio`
- Broken example unit assert (lists missing remedy ids on failure)
