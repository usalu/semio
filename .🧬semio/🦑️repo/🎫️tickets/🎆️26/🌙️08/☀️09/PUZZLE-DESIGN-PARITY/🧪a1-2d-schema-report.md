# A1-2d Schema Report — Puzzle Design Parity

Ticket: `26/08/09/PUZZLE-DESIGN-PARITY`  
Agent: A1-2d  
Ownership: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/**`

## Summary

Schema surgery for puzzle **2d** is complete under the artifact tree: connection params, node anchor, typed kind catalogs, unified compatibility, all 15 schema leaves, DSL/pack/spr constructors, example DSL assets, and inline `#[test]` coverage.

Crate-level `cargo test -p semio-s-plugin-puzzle --lib -- artifacts::puzzle2d` **does not yet execute tests** because parallel A3-5d work still breaks `🎛️apps/🖐️5d` (renamed catalog types / missing `anchor` in app literals). **Zero compile errors reference `puzzle2d` / `◻2d` artifact sources.**

## Struct field lists (normative)

### `Puzzle2dEdge`
- `id: String`
- `source: String` (refs handle)
- `target: String` (refs handle)
- `edge_kind: Option<String>`
- `gap: f64` (default 0.0)
- `shift: f64` (default 0.0)
- `rise: f64` (default 0.0)
- `rotation: f64` (default 0.0, degrees)
- `turn: f64` (default 0.0, degrees)
- `tilt: f64` (default 0.0, degrees)
- `x: f64` (default 0.0, diagram offset)
- `y: f64` (default 0.0, diagram offset)
- `source_tip: Option<String>`
- `target_tip: Option<String>`
- `visible: Option<bool>`
- `locked: Option<bool>`

### `Puzzle2dNodeAnchor`
- `Fixed` (default)
- `Derived`

### `Puzzle2dNode` (added)
- `anchor: Puzzle2dNodeAnchor` (default `Fixed`)

### `Puzzle2dKindCompatibility` (unified)
- `source: String`
- `target: String`
- `bidirectional: bool` (default false)
- `important: bool` (default false)
- `specificity: Puzzle2dCompatSpecificity`

### `Puzzle2dKindCatalogs` (replaces `Option<dsl::DslValue>`)
- `nodes: Vec<Puzzle2dCatalogNodeKind>`
- `handles: Vec<Puzzle2dCatalogHandleKind>`
- `edges: Vec<Puzzle2dCatalogEdgeKind>`
- `wires: Vec<Puzzle2dCatalogWireKind>`

### `Puzzle2dCatalogNodeKind` (type-like)
- `id, name, label, description, icon, image, unit`
- `abstract_: bool` (serde/json `"abstract"`)
- `base_kinds: Vec<String>`
- `representations: Vec<Puzzle2dRepresentation>`
- `handles: Vec<Puzzle2dHandleTemplate>`
- `attributes: Vec<Puzzle2dAttribute>`
- `authors: Vec<Puzzle2dAuthor>`

### `Puzzle2dRepresentation`
- `id, name, url, mime, tags, lod: Option<String>, description`

### `Puzzle2dHandleTemplate` (2d: `angle` instead of point/direction)
- `id, name, label, description, icon`
- `handle_kind: Option<String>`
- `angle: f64` (dsl angle rad)
- `t: Option<f64>, mandatory: Option<bool>, radius: Option<f64>`

### `Puzzle2dCatalogHandleKind` (port-like)
- `id, code, label, order, compatible_with, description, icon, color, default_wire_kind`

### `Puzzle2dCatalogEdgeKind`
- `id, name, label, description, icon, color`

### `Puzzle2dCatalogWireKind`
- `id, name, label, description, icon, color, default_edge_kind`

### Supporting
- `Puzzle2dAttribute { id, key, value, definition }`
- `Puzzle2dAuthor { id, name, email, role, rank }`

## Files changed

### Artifact root / types
- `🗿️artifacts/◻2d/🦀️component.rs` — core types, Defaults, inline tests

### 15 schema leaves (nested types expanded)
- `🧬️schema/{🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `📸️snapshot/🧬️schema/{🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- `🔺️diff/🧬️schema/{🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}`
- Rust facet leaves for artifact/snapshot/diff unchanged (they reference shared types from root)

### DSL / pack / spr
- `🗣️dsl/🦀️component.rs` — constructions + `puzzle2d_dsl_parses_edge_with_all_connection_params`
- `📡️spr/🦀️component.rs` — Node construction via `Default`
- `📸️snapshot/🎒️pack/🦀️component.rs` — Node construction via `Default`

### Examples
- `📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio`
- `📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🗣️tower.dsl.semio`
  - kind-compatibility columns → `source target bidirectional important specificity`
  - nodes gained `anchor:ENUM` (`fixed`)
  - edges gained `gap shift rise rotation turn tilt x y` (zeros)

### Out-of-ownership exception (compile unblock for 2d)
- `🎛️apps/◻2d/🦀️component.rs` — one test literal updated to `Puzzle2dNode { id, ..Default::default() }` so the new required `anchor` field does not break the crate from the 2d app side. **A3 still owns remaining `apps/🖐️5d` breakages.**

### Not requiring edits
- `🧬️mutations/**`, `🔧️op/**` — pass through `Puzzle2dEdge`/`Puzzle2dNode`/`Puzzle2dMeta` by value; no field-specific constructors
- Engine `set_board_kind_catalogs_from_json` remains a separate runtime JSON API (unchanged)

## Inline tests added (`◻2d/🦀️component.rs`)

1. `puzzle2d_edge_connection_params_default_to_zero`
2. `puzzle2d_node_anchor_defaults_to_fixed`
3. `puzzle2d_edge_serde_roundtrips_connection_params`
4. `puzzle2d_kind_compatibility_includes_important`
5. `puzzle2d_kind_catalogs_serde_roundtrip`

Plus DSL: `puzzle2d_dsl_parses_edge_with_all_connection_params` in `🗣️dsl/🦀️component.rs`.

## Test / compile status

Command:
```bash
cargo test -p semio-s-plugin-puzzle --lib -- artifacts::puzzle2d -- --nocapture
```

Result (latest): **could not compile** — exit 101.

2d-related errors: **0**.

Remaining blockers (peer agents, not A1-2d):
- `error[E0422]: cannot find struct, variant or union type `Puzzle5dCatalogGripTemplate` in module `crate::artifacts::puzzle5d``
- `error[E0422]: cannot find struct, variant or union type `Puzzle5dCatalogGripTemplate3d` in module `crate::artifacts::puzzle5d``
- `error[E0560]: struct `artifacts::puzzle5d::component::Puzzle5dCatalogPartKind` has no field named `mesh_url``
- `error[E0063]: missing field `anchor` in initializer of `artifacts::puzzle5d::component::Puzzle5dPart``
- `error[E0560]: struct `artifacts::puzzle5d::component::Puzzle5dCatalogGripKind` has no field named `name``
- `error[E0308]: mismatched types`
- `error: could not compile `semio-s-plugin-puzzle` (lib test) due to 6 previous errors; 84 warnings emitted`

Log: `🧪a1-2d-cargo-test-final.log` (exit 101).

Once A3 restores `apps/🖐️5d` against the new catalog type names and `Part.anchor`, re-run the scoped command; expected green tests include the five root tests plus DSL example round-trips and the 8-param edge parse test.

## Notes for B1 / peers

- New public types are `pub` from `🗿️artifacts/◻2d/🦀️component.rs` and already `pub use component::*` via glue — B1 should re-export any TS facades needed from the expanded leaf types.
- Serde JSON still accepts missing connection/anchor fields via `#[serde(default)]`; DSL tables require columns present (examples updated).
