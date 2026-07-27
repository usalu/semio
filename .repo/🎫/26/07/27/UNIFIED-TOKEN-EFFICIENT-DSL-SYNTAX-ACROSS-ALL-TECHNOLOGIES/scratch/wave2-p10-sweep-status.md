# Wave 2 — P10 (sweep/reconciliation) status

## Crates covered (attribute sweep + fixture regen + tests green)

- `mathematical/plugin/rs` — deleted redundant `#[dsl(key)]` overrides; converted `MathEdge`
  (source/target strings) to a DSL-only mirror (`MathEdgeDsl`) using `dsl::Wire`, mirroring
  `flow/core`'s `SynapseDsl` pattern (manual `impl vcs::DocumentDsl`/`OpText` instead of direct
  derive, since the JSON-facing `MathEdge` can't itself hold a `dsl::Wire` field alongside
  `Serialize`/`Deserialize`). Added `#[dsl(table)]` to nodes/edges/points. No fixture file. 13/13 tests green.
- `sourcing/curate/rs` — deleted redundant `#[dsl(key)]` overrides (GeometryRecipe/SortDirection/
  SetDocument all matched auto-kebab). Added `#[dsl(table)]` to `curated: Vec<CuratedItem>` (kept
  `stock: Vec<ObjectKind>` as AoS — `ObjectKind.geometry` is `#[dsl(statements)]`, non-self-delimiting,
  can't be a table column). Hand-regenerated both fixtures (`example/demo-stock.curate`,
  `example/empty-curation.curate`) to kebab keys + SoA table syntax via the canonicalize-test
  procedure. `sourcing_curate` 16/16 and downstream `sourcing-plugin` 16/16 (consumes both fixtures
  via `include_str!`) green.
- `protocol/module/procedural/rs` — deleted one redundant `#[dsl(key = "set-payload")]` (already
  matched auto-kebab). No fixture. 11/14 pass; 3 pre-existing failures unrelated to this ticket
  (icon catalog: `"building"` isn't a registered icon id — see Findings below).
- `vcs/plugin/rs` — deleted 6 redundant `#[dsl(key)]` overrides on `VcsDemoOperation` (all already
  matched auto-kebab). No fixture. 15/15 green.
- `protocol/rs` — deleted all `#[dsl(key)]` overrides on `ProtocolExpr` (6, all matched auto-kebab)
  and `ProtocolOperation` (9, all stale camelCase). No table conversion — `ProtocolBlock`/
  `ProtocolStep` are too heterogeneous (nested statements/lists) for SoA, and `options`/`fields` are
  `Option<Vec<T>>` where `#[dsl(table)]` is a silent no-op (derive's `classify_field` peels `Option`
  before checking `attrs.table`, so it always falls back to `OptionScalar`/List — verified against
  `dsl/derive/rs/lib.rs`). No fixture. `protocol` 21/21, `protocol-plugin` 9/9, `forms` 5/5 green.
  `forms-plugin` 21/31 green; 10 failures pre-existing/unrelated (9 icon-catalog, 1
  `try_wizard_gates_navigation_and_reports_inline_errors` using a hardcoded JSON example unrelated
  to DSL text).
- `cad/rs` (`cad_document`) — deleted all `#[dsl(key)]` overrides (`CadPaneId` kebab-matched,
  `CadOperation`'s 15 were stale camelCase). Added `#[dsl(table)]` to `objects`/`building_objects`/
  `energy_objects`/`structure_classic_objects: Vec<CadObject>` and `nodes: Vec<CadNode>` — all
  self-delimiting (fixed arrays + nested `Vec<CadPrimitiveSlot>` are fine as table cells; no
  duplicate-record-type columns in `CadObject`). No fixture file. `cad_document` 15/15,
  `cad-plugin` 77/78 (1 pre-existing icon-catalog failure) green.
- `remodel/rs` (`remodel_document`) — deleted 69 redundant `#[dsl(key)]` overrides (every
  `DslScalar` enum already kebab-matched; `MeshDataTwin`'s 7 were stale camelCase). Updated the
  hand-written `RemodelMesh::__dsl_spec()`'s literal `"textureAssetId"` key to `"texture-asset-id"`
  for consistency (that struct has a manual, non-derived `DslField` impl). Added `#[dsl(table)]` to
  8 `Vec<Record>` fields (frames/cameras/rig/observations/camera-poses-preview/poses/tracks/streams/
  gcps) — **had to revert `frames`/`observations`** after hitting a genuine engine limitation: a
  `#[dsl(table)]` column whose element type ITSELF has a `#[dsl(table)]` `Vec` field produces
  malformed nested `[header]{rows}` output inside one row that the parser can't recover
  (`node count exceeds max_nodes limit`); kept `streams`/`gcps` as the outer tables and left
  `frames`/`observations` as plain `Vec` (self-delimiting List, fine nested). 32/32 green.
  Downstream `remodel-plugin` 22/23 (1 pre-existing icon-catalog failure), `remodel_engine` 6/7
  (1 pre-existing failure, `tests::long::video_in_yields_watertight_mesh_out`, a slow (~13min)
  SfM/camera-registration numerical test unrelated to serialization — "need >= 3 registered cameras
  to fit a Sim3 gauge alignment, got 2" — nothing to do with DSL).
- `architect/program/rs` (`architect_program`, 16.7k lines, the "sweep" catch-all's biggest crate) —
  had **zero** pre-existing `#[dsl(key)]` overrides anywhere (field/variant names already matched
  auto-kebab). Added `#[dsl(table)]` to all 66 `Vec<Record>` register fields on `Program` (the
  "65 feature-area registers" doc comment underclaims by one). **Discovered and had to revert 13 of
  the 66** after finding a second genuine engine limitation distinct from remodel's: a
  `#[dsl(table)]` row type with **2+ fields of the exact same nested Record type** (e.g.
  `ProgramElement`'s `area`/`volume`/`height`/`occupancy: QuantitySpec`, or any register with 2+
  `TextField` fields) corrupts silently — a Record-shaped table cell's value-parser greedily
  consumes ANY following `key=value` token matching one of its OWN not-yet-set keys, so an unset
  field on column N eats a value that print emitted for column N+1 (proven with a synthetic
  single-row reproduction: `height`'s unset `target` field stole `occupancy`'s `target=4`, both
  columns silently wrong, no parse error). Reverted `#[dsl(table)]` on: `elements`
  (`ProgramElement`×4 `QuantitySpec`), `relationships` (`Relationship`×4 `TextField`), `services`
  (`ServiceRequirement`×2 `QuantitySpec`), `growth` (`GrowthPlan`×2 `QuantitySpec`), `conflicts`
  (`Conflict`×2 `TextField`), `requirements` (`Requirement`×2 `TextField`), `scenarios`
  (`Scenario`×2 `TextField`), `decisions` (`Decision`×3 `TextField`), `changes` (`ChangeRecord`×2
  `TextField`), `issues` (`Issue`×5 `TextField`), `knowledge` (`KnowledgeRecord`×2 `TextField`),
  `assumptions` (`Assumption`×3 `TextField`), `constraints` (`ConstraintRecord`×2 `TextField`) — 53
  tables kept. No fixture file (`extension = "architect"`, no `.architect` on disk). 180/180 green;
  downstream `architect-plugin` 11/11 green.

## Findings for other agents / follow-up (not fixed here — out of P10 scope or another package's file)

1. **`framework/surface/node-graph/rs`** (prompt explicitly asked to check its port-key convention):
   `fixture_from_node_graph_json` (line ~155) still builds `"{nodeId}:{portId}"` strings
   (`format!("{}:{}", edge.source_node_id, edge.source_port_id)`) for `DagFixtureEdge.source`/
   `target`. **Did not migrate to `nodeId@portId`** — the consumer,
   `infinite/board/port/directed/dag/rs::split_dag_endpoint` (P4's crate), still `rsplit_once(':')`
   as of this writing (confirmed via its currently-staged, in-progress P4 diff), and the SAME
   `"node:port"` convention is also built by `s/plugin/rs` (P4's own file, lines ~1289-1290,
   1052, 1059) which is concurrently mid-edit. Changing only `node-graph/rs` would desync from both
   and silently corrupt port routing for any port other than `"out"` (dag's fallback). This 3-way
   convention (node-graph producer, s/plugin producer, dag consumer/parser) needs one atomic change
   — recommend P4 do it across all three once its own attribute sweep lands, or a fast-follow ticket.
2. Pre-existing, unrelated-to-DSL test failures observed repeatedly across P10 packages (icon
   catalog): any test that renders a UI node referencing `IconName::from("building")` (or similar)
   panics with `invalid catalog icon name` at `ui/asset/icon/generated/icon_name.rs:1380` — the
   icon isn't in the generated catalog. Seen in `protocol-module-procedural`, `forms-plugin`,
   `cad-plugin`, `remodel-plugin`. Concurrent icon-catalog/UI-surface work (see
   `UNIFIED-6-LEVEL-UI-SURFACE-SYSTEM`/`CENTER-INTRODUCTION-STEP-COUNT-CHIP` tickets in-flight in
   this same tree) is the likely cause; not touched here.
3. Two now-confirmed general engine gotchas for anyone doing more `#[dsl(table)]` conversions in
   later Wave-2 packages or Wave 3 verification:
   - Never `#[dsl(table)]` a `Vec<T>` field whose `T` itself has a `#[dsl(table)]` field (nested
     table-in-row breaks the printer's one-row-per-line invariant).
   - Never `#[dsl(table)]` a `Vec<T>` field whose `T` has 2+ fields of the same nested `DslRecord`
     type with any optional sub-fields (ambiguous greedy key consumption across columns; silent
     data corruption, not a parse error — hard to catch without a targeted round-trip test per
     register/permutation).
