---
name: spatial static schemas
overview: Eliminate every dynamic property key in spatial schemas, fixtures, and TS runtime. Convert all id/name-keyed maps to typed arrays, turn Expr and commit/display payloads into discriminated unions, and replace free `Record<string, unknown>` with closed enums + tagged variants — end-to-end, no compatibility shims.
todos:
  - id: expr
    content: Rewrite expression.json + Expr TS union + evalExpr switch; convert all guards in fixtures
    status: completed
  - id: topology
    content: Convert topology.json + TopologyGraph (arrays + internal Map index) + TopologyDiff arrays + migration script for fixtures
    status: completed
  - id: machine
    content: Convert factory.json machine/guards/display/state.on to arrays; rewrite applyTransition, resolveDisplay, listKeyedCommandTransitions, getActiveSelectionSpec
    status: completed
  - id: tagged-payloads
    content: Tagged commit.operation, tagged display items, collapsed box.transform action; rewrite CommandRuntime.commit and renderer-r3f consumers
    status: completed
  - id: context
    content: Add typed context.fields + structured action paths; rewrite getPath/setPath usages and box helpers
    status: completed
  - id: downstream
    content: Update machine-stately adapter and kernel-brepjs; extend all vitest suites and run nx test until green
    status: completed
isProject: false
---

# Eliminate Dynamic Keys From Spatial Schemas

Greenfield sweep: every place where a JSON object uses a runtime value as a property name becomes a typed array of records; every free-form `params`/`Expr` becomes a closed discriminated union. The TS runtime mirrors the JSON exactly (no `Record<dynamic, T>` on the public surface; internal lookup indexes are rebuilt from arrays on load).

## 1. New canonical shapes

### Topology — [spatial/schema/json/topology.json](spatial/schema/json/topology.json)
Replace every id-keyed map with an array of records:

```json
{
  "schema": "spatial.topology/v1",
  "revision": 1,
  "vertices": [ { "id": "v0", "position": [0,0,0] } ],
  "edges":    [ { "id": "e0", "vertexIds": ["v0","v1"] } ],
  "wires":    [ ... ], "faces": [...], "shells": [...],
  "cells":    [...], "cellComplexes": [...], "clusters": [...]
}
```

Same for `TopologyDiff`:

```ts
type EntityDiff<T, P, Id> = {
  readonly added?: readonly T[];
  readonly modified?: readonly P[];   // patches carry their own id
  readonly removed?: readonly Id[];
};
```

### Expression — [spatial/schema/json/expression.json](spatial/schema/json/expression.json)
Tagged union discriminated by `kind` (no more "the operator is the key"):

```json
{ "kind": "path",     "path": "origin.0" }
{ "kind": "const",    "value": 0 }
{ "kind": "event",    "key": "point" }
{ "kind": "var",      "name": "z1" }
{ "kind": "let",      "bindings": [{ "name":"z0","value":{...} }], "in": {...} }
{ "kind": "exists",   "path": "origin" }
{ "kind": "notEmpty", "path": "corner" }
{ "kind": "all",      "args": [ ... ] }
{ "kind": "any",      "args": [ ... ] }
{ "kind": "not",      "arg":  { ... } }
{ "kind": "abs",      "arg":  { ... } }
{ "kind": "distance", "a": {...}, "b": {...} }
{ "kind": "binop",    "op": "==" | "!=" | ">" | "<" | ">=" | "<=" | "+" | "-" | "*" | "/", "left": {...}, "right": {...} }
{ "kind": "fold",     "op": "min" | "max", "args": [ {...}, {...} ] }
```

`op` becomes a closed JSON Schema `enum`. `evalExpr` in [spatial/js/core/index.ts](spatial/js/core/index.ts) switches on `expr.kind`.

### Factory / Command — [spatial/schema/json/factory.json](spatial/schema/json/factory.json)
Replace every name-keyed object:

```json
{
  "schema": "spatial.command/v1", "id": "...", "version": "...", "label": "...",
  "requires": { "kernel": {
      "operations": ["cell.createBox", ...],
      "editableEntities": ["vertex","edge", ...]
  } },
  "guards": [ { "name": "hasValidBox", "expr": { ... } } ],
  "history": { "excludeEvents": ["pointer.move","confirm"] },
  "machine": {
    "initial": "idle",
    "states": [
      { "name": "idle", "final": false,
        "selection": { "accept": ["vertex"], "multiple": false, "prompt": "..." },
        "on": [
          { "event": "start", "transitions": [
              { "target": "first_corner", "key":"s", "label":"Start",
                "actions": [ ... ] } ] } ] },
      { "name": "first_corner", "on": [ ... ] }
    ]
  },
  "display": { "states": [
      { "state": "first_corner", "items": [ ... ] } ] },
  "commit": { ... }
}
```

`State.on` is `EventHandler[]` with `event` (closed enum) + `transitions` (always an array — drop the `T | T[]` union).

### Actions — closed discriminated union
Replace `additionalProperties: true` with a strict per-op shape:

```json
{ "op": "assign",   "path": "corner", "value": { /* Expr */ } }
{ "op": "clear",    "path": "cursor" }
{ "op": "append",   "path": "...", "value": { /* Expr */ } }
{ "op": "emit",     "event": { "kind": "..." } }
{ "op": "raise",    "event": "..." }
{ "op": "openTransaction"|"commitTransaction"|"rollbackTransaction" }
{ "op": "requestPreview" }
{ "op": "kernel.query",       "query": "...", "params": { /* tagged */ }, "assignTo": "..." }
{ "op": "resolveEditable",    ... }
{ "op": "setDiagnostic",      "severity": "info"|"warning"|"error", "code": "...", "message": "..." }
{ "op": "clearDiagnostic",    "code": "..." }
{ "op": "box.transform",      "transform": "aabbFromDiagonalCorners"|"tripletRubber"|"tripletCommit"|"snapSquareFootprint"|"setCubeHeightFromFootprint"|"rubberCornerFromCenter"|"rubberSquareFromCenter"|"verticalFinalizeFootprint"|"initPeakAboveOrigin"|"peakFromOriginZ"|"verticalRubberCorner"|"cornerFromLengthWidth" }
```

The `box.*` ops collapse into one `box.transform` action carrying a `transform` enum, eliminating the open `op: string` namespace.

### Commit operation — tagged per kind
Replace `operation: { kind: string, params: object }` with a `oneOf` keyed by `kind`:

```json
{ "kind": "cell.createBox",      "cornerA": Expr, "cornerB": Expr, "height": Expr }
{ "kind": "wire.extrudeToCell",  "wireId": Expr, "distance": Expr, "direction": Expr }
{ "kind": "face.offset",         "faceIds": Expr, "distance": Expr }
{ "kind": "measure.distance",    "a": Expr, "b": Expr }
{ "kind": "measure.area",        "faceId": Expr }
{ "kind": "measure.volume",      "cellId": Expr }
```

`commit.when` references a guard `name` from the `guards[]` array (closed reference, not free key).

### Display — tagged per item kind — [spatial/schema/json/display.json](spatial/schema/json/display.json)
Drop `params: object` + `additionalProperties: true`. Each `kind` defines its own fields:

```json
{ "kind": "point",            "id": "...", "role": "...", "position": Expr }
{ "kind": "label",            "id": "...", "role": "...", "text": "...", "position": Expr }
{ "kind": "segment",          "id": "...", "role": "...", "from": Expr, "to": Expr }
{ "kind": "linear-handle",    "id": "...", "role": "...", "axis": [n,n,n], "origin": Expr }
{ "kind": "box-preview",      "id": "...", "role": "...", "cornerA": Expr, "cornerB": Expr, "height": Expr }
{ "kind": "entity-highlight", "id": "...", "role": "...", "entity": { "kind": "vertex"|..., "id": "..." } }
{ "kind": "curve" | "mesh",   ... }
```

### Command context — typed
Add a top-level `context` declaration to `CommandSpec` so action `path`s are not arbitrary strings:

```json
"context": {
  "fields": [
    { "name": "origin",      "kind": "vec3" },
    { "name": "corner",      "kind": "vec3" },
    { "name": "height",      "kind": "number" },
    { "name": "boxMode",     "kind": "enum", "values": ["point","diagonal","threePoint","vertical","center"] },
    { "name": "previewA",    "kind": "vec3" },
    ...
  ]
}
```

Action `path` becomes a typed reference (`{ field: "origin", axis?: "x"|"y"|"z" }`) instead of a dotted string. `Expr.path` and `Expr.event.key` likewise become structured.

## 2. TS runtime rewrite — [spatial/js/core/index.ts](spatial/js/core/index.ts)

- `TopologyGraphJson` fields become arrays; `TopologyGraph` internally builds `Map<Id, Record>` indexes from those arrays on `fromJSON` and serializes them back in `toJSON`.
- `CommandSpec.guards: NamedGuard[]`, `machine.states: StateDef[]`, `StateDef.on: EventHandler[]`, `display.states: StateDisplaySection[]`.
- New `type Expr = ExprPath | ExprConst | ExprEvent | ExprVar | ExprLet | ExprExists | ExprNotEmpty | ExprAll | ExprAny | ExprNot | ExprAbs | ExprDistance | ExprBinop | ExprFold;` — `evalExpr` is a single `switch (expr.kind)`. Delete the `binKeys` loop and the in-key dispatch.
- `ActionSpec` becomes a discriminated union; `applyActionAsync` switches on `op`. `box.*` ops collapse into one `box.transform` case dispatching on the `transform` enum.
- `resolveTemplate` is removed — `Expr` is the only template language; `evalExpr` covers every site (display item field, action `value`, commit operation arg).
- `TopologyDiff` rewrites `added`/`modified` as arrays; `applyTopologyDiff` builds the inverse the same way. `meshFaceTopologyDiff` returns the new shape.
- `parseCommandSpec` is replaced by a structural validator that walks the new arrays and rejects unknown enum values; no string-key iteration except inside the array loops it owns.
- `CommandSnapshot.context` stays a `Record<string, unknown>` *internally* but is built by writing into named context fields declared by the spec — public reads go through `getContextField(name)`. Diagnostic / message records already use named fields.

## 3. Fixtures — all 14 files

Rewrite to the new shapes:
- [spatial/fixture/box.command.json](spatial/fixture/box.command.json), [extrude-wire.command.json](spatial/fixture/extrude-wire.command.json), [offset-surface.command.json](spatial/fixture/offset-surface.command.json), [distance.command.json](spatial/fixture/distance.command.json), [area.command.json](spatial/fixture/area.command.json), [extrude.factory.json](spatial/fixture/extrude.factory.json), [offset-surface.factory.json](spatial/fixture/offset-surface.factory.json), [factory.json](spatial/fixture/factory.json) → tagged Expr / array states / typed display / typed commit / `box.transform`.
- [spatial/fixture/geometry.json](spatial/fixture/geometry.json), [geometry-routes.json](spatial/fixture/geometry-routes.json), [geometry-loom.json](spatial/fixture/geometry-loom.json), [small-building.topology.json](spatial/fixture/small-building.topology.json) (~1700 lines), [tall-building.topology.json](spatial/fixture/tall-building.topology.json), [large-building.topology.json](spatial/fixture/large-building.topology.json) → topology arrays. The large topology fixtures are mechanically converted by a one-off `spatial/js/core/script.ts migrate-fixtures` command (added once, kept in repo).

## 4. Downstream packages

- [spatial/js/machine-stately/index.ts](spatial/js/machine-stately/index.ts): translate `StateDef[]` / `EventHandler[]` to xstate config; keys for xstate's internal map are derived from the array `name`/`event` fields (xstate library still needs them, but that's purely internal).
- [spatial/js/kernel-brepjs/index.ts](spatial/js/kernel-brepjs/index.ts): consume `TopologyGraph` index API instead of iterating `Object.entries`.
- [spatial/js/renderer-r3f](spatial/js/renderer-r3f): switch from `DisplayItem.params.*` lookups to typed per-kind fields.
- Extend the existing vitest files in each package to cover: array-form topology round-trip, tagged Expr evaluation parity with the old in-key form, box command full workflow with `box.transform`, display tagged variants, diff inverse on array form.

## 5. JSON Schema rigor

In every schema file:
- `additionalProperties: false` everywhere; remove every `additionalProperties: <schemaRef>`.
- All arrays-of-records get `"uniqueItems": true` enforced on the `id`/`name` field via `items.required`.
- Every enum is closed: action `op`, expression `kind`, expression `binop.op`, display `kind`, commit `kind`, context-field `kind`, selection `accept` items, requires `editableEntities` items.
- Replace `"type": "object"` placeholders (`commit.operation.params`, `display.item.params`, `action` extras) with the new tagged `oneOf`.

## 6. Execution order (single sweep, no compat)

1. Land new `expression.json` + `Expr` TS union + `evalExpr` switch; update box guards in fixtures.
2. Land array-form `topology.json` + `TopologyGraph` index rebuild + `TopologyDiff` arrays; migrate all topology fixtures with the script.
3. Land array-form `machine.states/on`, `guards`, `display.states`; rewrite `applyTransition`, `listKeyedCommandTransitions`, `resolveDisplay`, `getActiveSelectionSpec`.
4. Land tagged commit `operation` + tagged display items + `box.transform` action; rewrite `CommandRuntime.commit` and renderer.
5. Land typed `context.fields` + structured action `path`; rewrite `getPath/setPath` usages, snapshot serialization, and box helpers.
6. Run `nx test` across `@spatial/js-core`, `@spatial/js-machine-stately`, `@spatial/js-kernel-brepjs`, `@spatial/js-renderer-r3f`; fix everything until green.

## 7. Out of scope

- `spatial.factory/v1` vs `spatial.command/v1` schema-id duality (will keep both ids; only the shape changes).
- `spatial/net/*` (only `AGENTS.md` present).
- Cross-tech surfaces (`elements`, `coda`, `semio`).

After the user confirms, work proceeds inside a ticket under `.repo/🎫/26/05/24/<slug>` with all intermediate logs and the fixture migration script kept there. Three delegated generalists in parallel are appropriate: (A) schemas + Expr + tests, (B) topology arrays + fixture migration + diff, (C) machine/display/commit/context restructure + downstream renderer & xstate adapter.
