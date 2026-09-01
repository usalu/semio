# Fix renderer + stately: final 9 errors → 0

## Files touched (exclusive ownership, as assigned)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️标准/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🟦️component.tsx`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️标准/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎰️stately/🟦️component.ts`

## Fixes

### stately(319) — WireRecord cast
`normalizeModelDiffIds` cast `w` through a structural `{ edgeIds: string[] }` to mutate a readonly `WireRecord.edgeIds: readonly EdgeRef[]` in place. Replaced the in-place mutation with an immutable rebuild: the function now returns a fresh object (`{ ...clone, wires: { ...clone.wires, added: clone.wires.added?.map(w => ({ ...w, edgeIds: w.edgeIds.map(() => "__edge__" as EdgeRef) })) } }`). No cast through `unknown`, no touching `readonly`; the placeholder id is branded at its true origin (a fresh synthetic test literal), consistent with the file's existing idiom for `stamp()`.

### renderer(7325) — attachment id
`attachment: { kind: "vertex", id: "v0" }` — the literal `"v0"` needed to enter the system as `VertexRef` at its declaration, matching the sibling `model.vertices["v0"]` declared one line below. Fixed to `id: "v0" as VertexRef`.

### renderer(7330) — faceIds
`const faceId = Object.keys(model.faces)[0]!` produced a plain `string`. Threaded the branded type through instead of re-branding: `const faceId = Object.values(model.faces)[0]!.id` — `FaceRecord.id` is already `FaceRef`.

### renderer(7569) — face id in ModelDiff literal
`const faceId = "f0"` is a genuinely-new synthetic literal (not derived from anything else in scope), so branded at that boundary: `"f0" as kernelGeometry.FaceRef` (no local `FaceRef` alias existed in this file's test scope, so used the already-imported `kernelGeometry` namespace).

### renderer(7596) — MeshTransfer literal
Hand-built object literal used plain arrays and was missing required `edgeGroups`/`edgeInfos` fields. Replaced with `{ ...emptyMeshTransfer(), position: new Float32Array([...]), normal: new Float32Array([...]), index: new Uint32Array([...]) }`, reusing the existing helper as suggested.

## Verification
Ran the full repo typecheck (`npx tsc -p "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json" --noEmit`).

Remaining output (repo-wide, 2 errors total):
```
🧰️framework/🔨️modules/🧊️3d/🟦️.ts(338,31): error TS2339: Property 'tessellate' does not exist on type 'typeof import(".../flow_core/pkg/flow_core")'.
🧰️framework/🔨️modules/🧊️3d/🟦️.ts(338,43): error TS2339: Property 'dispose' does not exist on type 'typeof import(".../flow_core/pkg/flow_core")'.
```
Both are in `🧰️framework/🔨️modules/🧊️3d/🟦️.ts`, explicitly out of scope (owned by the sibling flow_core wasm bindings agent). Filtering for `renderer|stately` in the tsc output returns zero matches.

**Result: 0 errors in my two owned files. Repo-wide: 2 errors, both pre-existing and outside my slice.**
