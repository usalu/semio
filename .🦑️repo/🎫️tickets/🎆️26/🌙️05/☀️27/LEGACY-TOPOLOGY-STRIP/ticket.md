# Legacy Topology Strip

**Goal:** Remove topologic framework entities (Cell, CellComplex, Cluster, Surface, Part, Volume, …); align editable model with Model / Object / Geometry / Attribute; use brepjs vocabulary (`solid` not `cell`) in kernel-private geometry; extension views for derived models.

## Done

- `cell` → `solid` across core, kernel-brepjs, fixtures, schema (`model.json`, `interaction.json` geometryEntities).
- Removed `cellComplex` / `cluster` from `Model`, `KernelGeometryJson`, selection kinds, and selection commands.
- Framework selection kinds: `object`, `geometry`, `attribute` + brepjs sub-picks (`vertex`, `edge`, `wire`, `face`, `solid`, `anchor`).
- `kernelGeometry` namespace: `GeometryEntityKind`, single `solidRef`, no duplicate exports.
- Fixed corrupted `produces.typology` (`builtin.c:.git` → proper `builtin.*` ids).
- `command.addPoint` + delegate unknown actions to `kernel.executeCommandDiff`.
- `curve.line` / `curve.polyline` in `BrepjsKernel.executeCommandDiff`.
- Ticket scripts under `.repo/🎫️/26/05/27/LEGACY-TOPOLOGY-STRIP/`.

## Remaining (follow-up)

- Collapse flat `Model.vertices/…` into per-`geometryRef` buckets (true `Geometry` entity).
- Remove kernel-private `shell` from persistence when brepjs graph allows faces→solid only.
- Repair `query` / `renderer-r3f` after `patch-siblings` (`__cellComplexesRemoved` placeholders).
- Run full nx test matrix (query, kernel-brepjs, machine-stately, renderer-r3f).

**Repo MCP:** unavailable; ticket recorded manually.
