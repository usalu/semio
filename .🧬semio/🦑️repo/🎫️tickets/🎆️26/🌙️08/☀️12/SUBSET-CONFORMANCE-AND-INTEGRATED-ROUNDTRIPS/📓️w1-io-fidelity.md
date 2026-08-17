# W1 — Io Fidelity Types

## Scope

Additive worker for ticket `SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS`.

**Edited file:** `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`

## Changes

### `//#region 🔖️IoFidelity` (after `SubsetValidator`, before `Wire`)

- `IoFidelityClass` — `Exact | Canonical | Semantic | Lossy` with `as_str`, `parse`, and ordered `rank` (3→0).
- `IoFidelityDeclaration` — manifest-facing `{ class, drops }` with `validate()` enforcing:
  - non-lossy classes must have empty `drops`
  - lossy class requires non-empty `drops`

### Subset validator registry helper

- `list_registered_subset_validator_dialects() -> Vec<Dialect>` — returns keys from `SUBSET_VALIDATOR_REGISTRY` (poisoned lock → empty vec).

### Unit tests (`io_fidelity_tests` under the new region)

- `io_fidelity_class_parse_and_rank` — parse round-trip, rank ordering, `as_str`
- `io_fidelity_declaration_validate` — valid/invalid drops per class

## Package & compile

| Item | Value |
|------|-------|
| Crate | `semio-framework` |
| Path | `🧰️framework/📦️packages/🦀️rust` |
| Io module | `pub mod io` via `#[path = "../../🔨️modules/🚪️io/🦀️component.rs"]` in `📦️glue.rs` |

### Verification command

```bash
CARGO_TARGET_DIR=".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SUBSET-CONFORMANCE-AND-INTEGRATED-ROUNDTRIPS/🎯️target" \
  cargo test -p semio-framework io_fidelity
```

### Result

- **Compile:** success (warnings only, pre-existing in `semio-framework-os-kernel`)
- **Tests:** 2 passed (`io_fidelity_class_parse_and_rank`, `io_fidelity_declaration_validate`)
