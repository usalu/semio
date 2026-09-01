# 🧭️ `listModelObjectsForModelDefinition` — fixture count fix

## Ground truth established

Read the fixture directly (`…/🎮️play/🔣️hexagonal-cut-concrete-forest-left.model.json`) with a
standalone JSON parse, independent of the sibling's earlier claim:

- `models[].id === "aec.building"` → `model.objects` has **exactly 11** entries:
  `object-…-bim-1` … `object-…-bim-11`, sequential, no gaps, typologies
  `building.building.slab` (1), `building.building.beam` (9), `building.building.column` (2).
- `models[].id === "aec.building.structure.classic"` also has 11 objects (`…-classic-structure-1..11`,
  independent BIM set — corroborates the pattern but isn't itself proof for `aec.building`).

## Why 12 was structurally impossible, not just empirically wrong

`Model.fromJSON` (🟦️component.ts:1244) builds `model.objects` as a 1:1 `recordsById` map over the
fixture's raw `objects` array — no synthesis, no duplication, no transformation application (confirmed
by reading `ModelSpace.fromJSON`, 🟦️component.ts:1455, which does a mechanical `Model.fromJSON` per row
and nothing else — no `transfer`/`applyTransformation` call at load time).

`listModelObjectsForModelDefinition` (🟦️component.ts:3263) is a **pure filter**:
`Object.values(model.objects).filter(...)`. A filter cannot emit more rows than its input has. Since
`space.models["aec.building"].objects` has 11 keys, the maximum possible return length is 11 —
independent of which/how many typologies belong to the `aec.building` model definition (there are 11
typology assets defined for it: Stair, Ceiling, Column, Slab, Door, Wall, Roof, Window, Foundation,
Railing, Beam — but the fixture only instantiates 3 of them, Slab/Beam/Column, across its 11 objects).

So `toHaveLength(12)` at 🟦️component.ts:3827 could never pass against this fixture, regardless of the
typology-matching logic. This rules out "the function under-filters" as an explanation and confirms the
assertion itself was wrong.

**Fix**: changed the expectation from `12` to `11` (🟦️component.ts:3827). No other line touched.

## The "returns 0" failure — not actually the same function

The brief described a second failure in the sibling's file, same function, returning `0`. A fresh full
suite run (`bun nx run @semio-tech/cad-js:test-long`) shows this framing is stale/incorrect for the
current tree state:

- There is only **one** test anywhere in the CAD suite named
  `listModelObjectsForModelDefinition lists BIM class objects for aec.building`, and it lives in
  **my own file** (🟦️component.ts:3819, `describe("core transformations")`) — not in the sibling's
  `📄️artifact/🟦️component.ts`.
- The sibling's file's `AssertionError: expected 0 to be greater than 0` failure is a **different**
  test — `"every typology ships construct kit or legacy create interactions"` (📄️artifact/🟦️component.ts:1354),
  asserting `typology.actions.length` / `typology.interactions.length`, not object counts, and not
  calling `listModelObjectsForModelDefinition` at all. This is the already-tracked, separate item in
  `📓️status.md` ("11 `aec.building.*` typologies ship no actions/interactions ... needs a modeling
  decision") — out of scope for this ticket slice and not a defect in my file.

No hand-off needed: nothing in the sibling's file calls into my function incorrectly: I grepped every
`listModelObjectsForModelDefinition` call site in `📄️artifact/🟦️component.ts`,
`🎬️actions/🟦️component.ts` and `📺️renderer/🟦️component.tsx` — all pass a live `Model` plus a
`modelDefinitionId` string consistent with the function's contract; none of them are asserting `> 0`
against this function's result in a way that currently fails.

## ModelSpace/transformation neighbourhood sanity check (AGENTS.md)

AGENTS.md's `ModelSpace` semantics require: "When a primitive is edited inside a model space, then all
primitives with the same hash are also edited... If the user tries to change something that is
affecting primitives that can't be linked back, give the users a warning that the models are no longer
linked."

Found a **real gap, not touched**: `ModelSpace` (🟦️component.ts:1401) has `link`, `unlink`, `get`,
`transfer`, `vertexHashesByModel`, `geometryHashesByModel` — the hash-lookup primitives needed to
detect cross-model-linked primitives — but no edit-propagation path and no warning emission consume
them. `vertexHashesByModel`/`geometryHashesByModel` are exercised only by this file's own round-trip
serialization tests (🟦️component.ts:3723-3734); no call site anywhere in the CAD plugin uses them to
propagate an edit by hash across linked models or to warn when a primitive can't be traced back. This
is a missing feature, not a regression (no test currently covers or expects it), so per the ticket's
"only fix what is clearly broken" instruction I did not implement it — flagging for a future ticket
since it's core to the ModelSpace contract.

## Verification run

```
bun nx run @semio-tech/cad-js:test-long
Test Files  1 failed | 8 passed (9)
     Tests  18 failed | 303 passed (321)
```

Before: 302 passing / 19 failing (2 files failing). After: 303 passing / 18 failing (1 file failing) —
my file (`🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts`) is now fully green. The
remaining 18 failures are entirely inside the sibling's `📄️artifact/🟦️component.ts` and untouched by
this change.

`tsc -p …/📦️typescript/tsconfig.json --noEmit` → same 3 pre-existing, expected repo-wide errors (2×
`flow_core` missing `tessellate`/`dispose` wasm exports, 1× implicit-`any` in the library index), zero
new errors, zero errors in CAD.

## Files touched

- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts` — one-line assertion fix
  (line 3827: `toHaveLength(12)` → `toHaveLength(11)`).
