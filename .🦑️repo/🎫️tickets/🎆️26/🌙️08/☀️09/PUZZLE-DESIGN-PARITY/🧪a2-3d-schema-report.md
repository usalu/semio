# A2-3d Schema Report — Puzzle Design Parity

Ticket: `26/08/09/PUZZLE-DESIGN-PARITY`  
Agent: A2-3d  
Scope: `🗿️artifacts/🌊️3d/**` except `⚙️engine/📐️geometry/`

## Summary

Schema surgery for puzzle 3d is complete for the owned tree: attraction `x`/`y`, object `anchor`, type-like object kinds, port-like vortex kinds, full vortex templates, `Puzzle3dRepresentation`, and unified kind compatibility. All 15 schema leaves were handcrafted. Engine Attraction/Object constructors (non-geometry) were updated. Example DSL assets were rebuilt. Inline serde/DSL tests were added.

Whole-crate `cargo test` cannot finish yet: **6 compile errors remain in `🎛️apps/🖐️5d/`** (owned by Wave 3 / A3), not in the 3d artifact tree. No 3d-artifact error appears in the compile log.

## Normative field landing

| Change | Location |
|--------|----------|
| `Puzzle3dAttraction.{x,y}` defaults `0.0` (degrees already on rotation/turn/tilt) | `🦀️component.rs` + AttractionProps |
| `Puzzle3dObject.anchor: Puzzle3dObjectAnchor { Fixed, Derived }` default Fixed | `🦀️component.rs` + FixtureObject |
| `Puzzle3dCatalogObjectKind` type-like (description/icon/image/unit/abstract/base_kinds/representations/attributes/authors; vortices kept) | `🦀️component.rs` |
| `Puzzle3dCatalogVortexKind` port-like (code/label/order/compatible_with/description/icon/color/default_cable_kind) | `🦀️component.rs` |
| `Puzzle3dRepresentation` | `🦀️component.rs` |
| Vortex templates: id/name/label/description/icon/vortex_kind/point/direction/t/mandatory/radius | `Puzzle3dCatalogVortexTemplate` (+ engine `ObjectKindVortexTemplate`) |
| `Puzzle3dKindCompatibility` + `Puzzle3dCompatSpecificity` (important + typed specificity) | `🦀️component.rs` |

`mesh_url` was removed from catalog object kinds (replaced by `representations[].url`). Instance `Puzzle3dObject.mesh_url` remains. Engine `resolve_object_kind_mesh_url` now reads the first non-empty representation URL.

## Fifteen schema leaves

Updated nested defs for Attraction/Object/Meta/catalogs/compatibility in:

- `🧬️schema/` — rs (via domain types), ts, graphql, json, proto
- `📸️snapshot/🧬️schema/` — same five
- `🔺️diff/🧬️schema/` — same five (stub Attraction/Object expanded)

## Mutations / DSL / Op / SPR / Pack

- Mutations: unchanged apply surface; they carry full `Puzzle3dAttraction` / `Puzzle3dObject` values (new fields flow automatically).
- DSL: derive-driven; examples rebuilt; DSL tests extended for `x`/`y` + `anchor` + typed specificity.
- Op: derive/codecs inherit new fields.
- SPR: document mutation wire guard switched to round-trip after intentional wire move; attraction sample includes `x=7,y=8`; scene sample catalogs use `representations`.
- Pack: constructors updated with `anchor`.

## Engine (non-geometry)

- `AttractionProps`: `x`, `y`
- `FixtureObject`: `anchor`
- `ObjectKind`: `representations` (replaces `mesh_url`)
- `ObjectKindVortexTemplate`: `point` + full connector fields
- `VortexKindCatalog`: port-like optional fields
- Brush placement / resolve / tests updated

**Not touched:** `⚙️engine/📐️geometry/` (Wave 2).

## Examples

- `📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio`
- `📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio`

Catalog object rows → representations + `point=`; vortex kinds → port columns; instance objects → `fixed` anchor; attractions header → `x`/`y`.

## Tests added

`design_parity_schema_tests` in `🗿️artifacts/🌊️3d/🦀️component.rs`:

- attraction 8-parameter defaults + serde
- object anchor default Fixed + `derived` serde
- type-like object kind + port-like vortex kind
- compatibility important + specificity
- (second module coverage) representations / point / abstract JSON key

## Cargo

Command:

```bash
cargo test --lib design_parity_schema_tests
```

Result: **blocked by parallel Wave-1 5d app compile errors** (see `🧪a2-3d-cargo-compile.log`):

1. `Puzzle5dCatalogGripTemplate` / `…3d` renamed in artifact but apps still reference old names
2. `Puzzle5dCatalogPartKind` no longer has `mesh_url`
3. `Puzzle5dPart` missing `anchor` in an app test literal
4. `Puzzle5dCatalogGripKind` no longer has `name`; `label` is `Option<String>`

**No errors under `🗿️artifacts/🌊️3d/`.** Once A3/Wave-3 apps compile, re-run the scoped test command above plus:

```bash
cargo test --lib puzzle3d
```

## Out of ownership (not edited)

- `⚙️engine/📐️geometry/`
- `🎛️apps/`, `◻2d/`, `🖐️5d/`, glue.rs, TS index, launch.json, storybook, compose/
