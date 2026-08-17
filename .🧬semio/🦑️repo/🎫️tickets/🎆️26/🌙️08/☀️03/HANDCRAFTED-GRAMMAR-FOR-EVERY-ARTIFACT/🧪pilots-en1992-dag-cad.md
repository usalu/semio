# Pilots: en1992, dag, cad

Handcrafted domain-driven specs for P4 pilots (grammar + protocol per facet, wiring via `include_str!` and `register_language`).

## en1992 (family-sheet)

| Facet | Grammar / protocol id | Keywords / layout |
|-------|----------------------|-------------------|
| dsl | `en1992.document` | `semio norm.en1992.dsl v1`; typed fields: `annex=en|de`, Eurocode quantities via `qty` / `QUANTITY`, `fire-rating=r30|r60|r90|r120`, `tightness-class=tc0|tc1|tc2`, `use-fem`, `anchor-cracked`; `reference-sheet`, `clause-sheet` from family-sheet |
| op | `en1992.op` | `set-document` + TEXT (whole document replace) |
| diff | `en1992.diff` | `document` + same typed assign list as dsl |
| pack | `en1992.pack` | magic `0x894E19920E0A1A0A`; schema `norm.en1992.v1`; segments `clause`, `quantity`; footer 64 |
| spr | `en1992.spr` | `set-document` tag 1 + document bytes |

**Operation enum (Rust):** `SetDocumentOperation<Document>` → single keyword `set-document`.

## dag (family-graph)

| Facet | Grammar / protocol id | Keywords / layout |
|-------|----------------------|-------------------|
| dsl | `dag.document` | `semio dag.dag.dsl v1`; `schema=`; typed node kinds (`slider`, `select`, `computation`, `screen`, …); `inputs`/`outputs` port tables; `edges` table with `wire-endpoint` + `edge-arrow`; `chain` |
| op | `dag.op` | `nodes-add`, `nodes-remove`, `nodes-move`, `nodes-patch`, `edges-add`, `edges-remove`, `edges-move`, `edges-patch`, `set-nodes`, `set-edges`, `set-document` |
| diff | `dag.diff` | `document`, `set-nodes`, `set-edges`, `nodes`/`edges` collection blocks (`added` / `removed` / `modified`) |
| pack | `dag.pack` | magic `0x894441470E0A1A0A`; schema `dag.fixture.v1`; segments `node-graph`, `edge-graph`; records `node-spec` tag 10, `edge-spec` tag 11 |
| spr | `dag.spr` | tags 1–11: `nodes-add`, `nodes-remove`, `nodes-move`, `nodes-patch`, `edges-add`, `edges-remove`, `edges-move`, `edges-patch`, `set-nodes`, `set-edges`, `set-document` |

**Operation enum (Rust):** `DagOperation` collection ops + `SetNodes`, `SetEdges`, `SetDocument`.

## cad (family-scene)

| Facet | Grammar / protocol id | Keywords / layout |
|-------|----------------------|-------------------|
| dsl | `cad.document` | `semio cad.cad.dsl v1`; `schema`, `id`, `active-model-definition-id`, `references-by-model-definition-id`, pane geometries (brep tables), typed object tables, `nodes` table; `layer` from family-scene |
| op | `cad.op` | `add-object`, `remove-object`, `patch-object`, `translate-objects`, `rotate-objects`, `scale-objects`, `set-pane-objects`, `add-node`, `remove-node`, `rename-node`, `patch-reference`, `set-references`, `set-active-model-definition`, `set-scene`; panes `shape|building|energy|structure-classic` |
| diff | `cad.diff` | `scene`, `active-model-definition-id`, per-pane `objects` collections, `references-by-model-definition-id`, `nodes` |
| pack | `cad.pack` | magic `0x894341443E0A1A0A`; schema `cad.scene.v2`; segments per pane + `brep-topology`, `reference-media`; records `cad-object` 20, `cad-node` 21 |
| spr | `cad.spr` | tags 1–14 matching each `CadOperation` variant |

**Operation enum (Rust):** `CadOperation` — AddObject, RemoveObject, PatchObject, TranslateObjects, RotateObjects, ScaleObjects, SetPaneObjects, AddNode, RemoveNode, RenameNode, PatchReference, SetReferences, SetActiveModelDefinition, SetScene.

## Wiring

- **en1992:** `include_str!` on all five facets; `artifacts::en1992::engine::register_pilot_languages()` from norm glue after en1992 codec registration.
- **dag:** already wired; updated spec files only.
- **cad:** `include_str!` added on dsl/op/diff/pack/spr; `register_pilot_languages()` in cad engine `register()`.

## Distinctness (normalized id/schema/start stripped)

- Three different pack magics: `EN1992`, `DAG`, `CAD3` ASCII prefixes.
- Three spr record vocabularies (1 vs 11 vs 14 tagged variants).
- Grammars use different families (sheet vs graph vs scene) and disjoint production sets.
