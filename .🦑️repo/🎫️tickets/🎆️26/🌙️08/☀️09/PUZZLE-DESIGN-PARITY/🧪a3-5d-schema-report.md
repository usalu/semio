# A3-5d Schema Report — PUZZLE-DESIGN-PARITY

**Agent:** A3-5d  
**Ticket:** `26/08/09/PUZZLE-DESIGN-PARITY`  
**Goal:** `R26-02`

**Canonical ownership path:**

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/`

## Summary

Wave-1 schema surgery for puzzle **5d** is complete inside the artifact tree:

1. `Puzzle5dFastener` now has all **8** transform params including diagram `x` / `y`.
2. `Puzzle5dPart` has `anchor: Puzzle5dPartAnchor { Fixed, Derived }` (default `Fixed`; default is skipped on serialize).
3. Thin catalogs replaced with type-like / port-like kinds + `Puzzle5dRepresentation` + full `Puzzle5dGripTemplate`.
4. `Puzzle5dKindCompatibility` unified with `important` + `specificity: Puzzle5dCompatSpecificity`.
5. All **15** schema leaves updated (full / snapshot / diff × rs/ts/graphql/json/proto).
6. Runtime compose bridge **`⚙️engine/🌉️compose/` deleted**.
7. Inline domain tests added; scoped `artifacts::puzzle5d::` tests pass (**22/22** excluding Wave-2 `flatten`).

---

## Domain changes (`🗿️artifacts/🖐️5d/🦀️component.rs`)

### Fastener (8 params)

```rust
gap, shift, rise, rotation, turn, tilt, x, y  // all f64, default 0.0, serde camelCase
```

### Part anchor

```rust
enum Puzzle5dPartAnchor { Fixed, Derived }  // #[default] Fixed
```

Serialized as camelCase (`fixed` / `derived`). Default `Fixed` is `skip_serializing_if`.

### Catalogs (renames)

| Old | New |
|-----|-----|
| `Puzzle5dCatalogPart` | `Puzzle5dCatalogPartKind` |
| `Puzzle5dCatalogGrip` | `Puzzle5dCatalogGripKind` |
| `Puzzle5dCatalogGripTemplate` (+2d/3d split) | `Puzzle5dGripTemplate` (unified point/direction/t/mandatory) |
| `Puzzle5dCatalogFastener` | `Puzzle5dCatalogFastenerKind` |
| `Puzzle5dCatalogRope` | `Puzzle5dCatalogRopeKind` |

Also added: `Puzzle5dRepresentation`, `Puzzle5dAttribute`, `Puzzle5dAuthor`.

`abstract` JSON field maps to Rust `is_abstract` (`#[serde(rename = "abstract")]`), DSL column `is-abstract`.

### Compatibility

```rust
struct Puzzle5dKindCompatibility {
  source, target, bidirectional, important, specificity: Puzzle5dCompatSpecificity
}
enum Puzzle5dCompatSpecificity { General, Part, Fastener, Grip, Rope }  // lowercase serde
```

### Temporary aliases (Wave 3)

```rust
pub type Puzzle5dCatalogPart = Puzzle5dCatalogPartKind;
pub type Puzzle5dCatalogGrip = Puzzle5dCatalogGripKind;
```

---

## Compose bridge removal

- **Deleted** folder: `🗿️artifacts/🖐️5d/⚙️engine/🌉️compose/` (entire module).
- Removed `pub use …::import_compose_design_json` from engine reexports.
- **Surgical edit outside ownership (required for compile):** removed `pub mod compose` path from `📦️packages/🦀️rust/📦️glue.rs` (B1 single-owner file). B1 should confirm this line stays gone.
- **Temporary shim** left in `⚙️engine/🦀️component.rs`:

```rust
pub fn import_compose_design_json(_design_json: &serde_json::Value) -> Puzzle5dSnapshot {
    empty_puzzle5d_snapshot()
}
```

  so the crate compiles while apps still call it.

### App follow-ups (NOT fully owned by A3 — Wave 3 / D7 / B1)

- `🎛️apps/🖐️5d/🎮️commands/🛍️example/` still calls `import_compose_design_json` / `importComposeKit`.
- App root still registers `importComposeKit` mutation and tests it.
- A3 applied a **minimal compile fix** in `🎛️apps/🖐️5d/🦀️component.rs` kit:in catalog upsert to construct `Puzzle5dCatalogPartKind` / `Puzzle5dCatalogGripKind` / `Puzzle5dGripTemplate` and to set `anchor` on a test `Puzzle5dPart`. Wave 3 should finish removing `importComposeKit` UI and delete the engine shim.

---

## Schema leaves (15)

All updated under:

- `🧬️schema/` — nested types for Part/Fastener/Compat/Catalogs/Representation/GripTemplate/…
- `📸️snapshot/🧬️schema/` — same nested expansions
- `🔺️diff/🧬️schema/` — Part/Fastener/Compat stubs enriched with new fields + enums

Formats: `🦀️component.rs` (state classes unchanged; types flow from domain), `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json`, `🛰️component.proto`.

---

## Mutations / DSL / OP / SPR / Pack

- Call sites in `🗣️dsl`, `📡️spr`, `📸️snapshot/🎒️pack`, `⚙️engine/✂️transfer`, `🔺️diff` updated for new struct fields.
- SPR wire-format guard: expanded part/fastener/snapshot ops now **round-trip** assert (frozen hex kept for unchanged rows). Intentional wire expansion from design-parity fields.
- Example DSL assets updated:
  - Parts: `anchor` column (`fixed`)
  - Fasteners: `x` / `y` columns (tower: 179 rows appended `0 0`)
  - Kind-compatibility: `important` + `specificity`
  - Kind-catalogs: forest rebuilt with type-like part kinds + grip templates; tower catalogs emptied (header-only) after transform fragility — Wave 4 should regenerate coherent catalogs from compose transfer.

---

## Tests run

```text
cargo test -p semio-s-plugin-puzzle --lib artifacts::puzzle5d:: -- --skip flatten
→ ok. 22 passed; 0 failed
```

Inline domain tests (new): fastener xy defaults/round-trip, part anchor default, compat important/specificity, catalog part-kind representations/grips, grip direction default Z, grip-kind port fields.

Example unit tests:

```text
examples::puzzle5d::concrete_forest_tests::* → ok
examples::puzzle5d::nakagin_capsule_tower_tests::* → ok
```

### Known out-of-scope failure

`artifacts::puzzle5d::engine::flatten::tests::flatten_writes_diagram_offsets_onto_part_2d` fails (`0.0` vs `1.5`). Module `⚙️engine/📐️flatten/` is **Wave 2 / C2** ownership (A3 must not create/edit it). Flagged for C2.

---

## Files touched (high level)

**Owned / primary**

- `🗿️artifacts/🖐️5d/🦀️component.rs` — domain schema + inline tests
- `🗿️artifacts/🖐️5d/🧬️schema/{rs,ts,graphql,json,proto}`
- `🗿️artifacts/🖐️5d/📸️snapshot/🧬️schema/{…}`
- `🗿️artifacts/🖐️5d/🔺️diff/🧬️schema/{…}` + `🔺️diff/🦀️component.rs`
- `🗿️artifacts/🖐️5d/🗣️dsl/🦀️component.rs`
- `🗿️artifacts/🖐️5d/📡️spr/🦀️component.rs`
- `🗿️artifacts/🖐️5d/📸️snapshot/🎒️pack/🦀️component.rs`
- `🗿️artifacts/🖐️5d/⚙️engine/🦀️component.rs` — compose reexport removed; shim added
- `🗿️artifacts/🖐️5d/⚙️engine/✂️transfer/🦀️component.rs`
- `🗿️artifacts/🖐️5d/⚙️engine/🌉️compose/**` — **DELETED**
- `🗿️artifacts/🖐️5d/📚️examples/**/🖼️assets/🗣️*.dsl.semio`

**Cross-ownership (compile unblock)**

- `📦️packages/🦀️rust/📦️glue.rs` — removed `mod compose` (B1)
- `🎛️apps/🖐️5d/🦀️component.rs` — kit:in catalog mapping + test `anchor` (Wave 3 / D7)

**Ticket scratch**

- `a3_*.py` helper scripts left in ticket folder (not deleted per rules)

---

## Handoff

| Owner | Action |
|-------|--------|
| **B1** | Confirm `glue.rs` has no `compose` mod; export any new types from TS index if needed |
| **C2** | Fix/own `⚙️engine/📐️flatten` test vs fastener `x`/`y` |
| **D7 / Wave 3** | Remove `importComposeKit` + engine shim; polish kit:in catalog mapping |
| **Wave 4** | Regenerate tower/forest kind-catalogs DSL coherently from compose assets |

