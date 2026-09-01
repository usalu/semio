# Lowpoly schema-truth fix — meshJson, payload schemas, diff structural/null bugs

Scope owned: every non-Rust `component.{json,proto,graphql,ts}` under `$A/🧬️schema/**`
(`$A` = `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any`) plus the three
per-mutation `🔣️payload.schema.json` files named in the briefing. Rust untouched (read-only truth).
Built on, did not undo, the prior agent's 17-mutation dispatch rewrite (verified intact, still 17/17).

## Authoritative field lists (from Rust, cross-checked against fixtures)

- `LowpolyObject` (`✏️s/…/🦀️component.rs:101-137`): `id, name, transform, smoothShading, mesh,
  paintLayers`. `mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>` → wire `null | {childId,
  target: ArtifactRef}`; `ArtifactRef = {artifactId, dialect}`; `ArtifactDialect = {artifactKind,
  standard, subset}` (`🚪️io/🧬️schema/🦀️component.rs:53,154`).
- `LowpolyObjectPatch` (same file, `:187-200`): `name, smoothShading, transform, mesh` only — no
  `paintLayers`. `mesh: Option<Option<ArtifactChild<…>>>`.
- `LowpolyDiff` (`🔺️diff/🦀️component.rs`): 38 top-level fields, **every one `Option<T>`**, no
  `skip_serializing_if` anywhere ⇒ every key is always present on the wire, `null` when untouched
  (confirmed on every diff fixture, e.g. `🔖️rename-paint-layer/…`).
- `LowpolyObjectPatchEntry`: `id, patch, paintLayers` (paintLayers lives HERE, not on `ObjectPatch`).
- `LowpolyObjectsDelta`: `added, removed, patched` (plain, always arrays) + `reordered` (`Option`,
  nullable). `LowpolyPaintLayersDelta`: `added, removed, patched, strokes` (all plain arrays).
  `LowpolyPaintLayerPatch`/`LowpolyIndexedPaintLayerPatch.patch`: `name, visible, opacity, blendMode`
  — all `Option<T>`, always-present+nullable (fixture-confirmed).

## Bug 1 — stale `meshJson` (10+ occurrences found and fixed)

`meshJson: string` / `mesh_json` never existed as `String` in Rust — it's a leftover from before
`s.stdio.semio` child-handle mesh references. Fixed in all 4 languages at **artifact + snapshot**
levels: `🧬️schema/{🟦️component.ts,🔣️component.json,🛰️component.proto,🔗️component.graphql}` and the
same 4 under `📸️snapshot/`. Each gained `LowpolyMeshHandle`/`ArtifactRef`/`ArtifactDialect`
type/message/def, matching the exact naming/shape the prior agent already used in
`🧬️mutations/🟦️component.ts` (verified, not guessed). Also fixed the same stale field inside
`🔺️diff/{🟦️component.ts,🛰️component.proto,🔗️component.graphql}`'s embedded `LowpolyObject`/
`LowpolyObjectPatch` copies (beyond the literal artifact/snapshot scope, but the same bug, same
owned files, left inconsistent otherwise) — `🔺️diff/🔣️component.json` didn't have this field at all
(see Bug 4). Confirmed zero remaining `meshJson`/`mesh_json` in any owned non-Rust file (grep swept
clean; remaining `mesh_json` hits are legitimate Rust locals/fn-names for mesh geometry JSON content,
a different concept, in read-only files).

## Bug 2 — three broken payload schemas → 17/17 ajv

- `🧬️mutations/🌱️create-object/🔣️payload.schema.json`: `object` was missing `mesh` entirely (added,
  `null | LowpolyMeshHandle`) **and** `paintLayers[].pixels` was `array<integer>` (fixed to base64
  `string`).
- `🧬️mutations/➕️insert-paint-layer/🔣️payload.schema.json`: `layer.pixels` was `array<integer>` →
  base64 `string`.
- `🧬️mutations/🎨️edit-paint-layer/🔣️payload.schema.json`: `runs[].bytes` was `array<integer>` →
  base64 `string`.

ajv 8.20 (repo root `node_modules`), throwaway script kept at
`🗑️generated/verify-schema-truth.mjs` (dereferences `🧬️mutations/🔣️component.json`'s 17 `oneOf`
branches against sibling `payload.schema.json` files, manually — ajv's URI resolver mangles the
emoji path segments in these `$ref`s, double-percent-encoding them):

```
[mutations] oneOf branches found: 17
[mutations] distinct payload $refs: 17
[mutations] 17/17 passed
[mutations] hostile double-tag rejected: true
```

## Bug 3 — diff TS structural bug (`paintLayers` misnested)

`🔺️diff/🟦️component.ts`'s `LowpolyObjectPatch` incorrectly carried `paintLayers?:
LowpolyPaintLayersDelta` and stale `meshJson?: string`. Fixed: `LowpolyObjectPatch` is now `{name?,
smoothShading?, transform?, mesh?: LowpolyMeshHandle | null}` (matches Rust exactly); `paintLayers?:
LowpolyPaintLayersDelta` moved to `LowpolyObjectPatchEntry` where Rust actually has it. Confirmed
against fixture `🔖️rename-paint-layer/…/🔺️diff/🔣️component.json`: `patch` has no `paintLayers` key;
`paintLayers` is a sibling of `patch` on the entry.

## Bug 4 — diff JSON Schema shallow + missing `null`

Full rewrite of `🔺️diff/🔣️component.json` (built programmatically, not hand-typed, to avoid syntax
error risk — script at `🗑️generated` was not kept, only the output). Findings:
- **Dangling `$ref`**: the original file `$ref`'d `#/$defs/LowpolySelection` but never defined it —
  a genuine bug beyond what was briefed, now fixed with `LowpolySelection`/`LowpolySelectionTargets`
  `$defs` added.
- `added`/`patched` in `LowpolyObjectsDelta` were bare `{"type":"object"}` → now `LowpolyObject[]` /
  `LowpolyObjectPatchEntry[]`, with 12 new `$defs` (`LowpolyObjectPatchEntry`, `LowpolyObjectPatch`,
  `LowpolyPaintLayersDelta`, `LowpolyIndexedPaintLayer(Patch)`, `LowpolyPaintStrokeAt`, `PixelRun`,
  `LowpolyPaintLayerPatch`, `LowpolyObject`, `LowpolyTransform`, `LowpolyPaintLayer`,
  `LowpolyMeshHandle`/`ArtifactRef`/`ArtifactDialect`), matching the diff TS sibling's depth exactly
  (not deeper — `LowpolyArtifact` here stays the TS file's existing shallow `{schema, objects}` stub,
  since expanding it wasn't briefed and would create a *new* json-vs-ts mismatch).
- **`null` added** to every top-level `LowpolyDiff` property (all 38 are `Option<T>`, all
  always-present+nullable per fixture) except the 5 that already had it (`activeObjectId`,
  `hoveredObjectId`, `hoveredTargetObjectId`, `hoveredTargetMode`, `hoveredTargetId` — genuinely
  double-`Option`). `required` now lists all 38 top-level keys (they're never omitted, only
  null-valued) — the original file's `"required": []` was itself wrong. Also fixed the two
  `LowpolyDiff` proto/graphql `LowpolyObjectPatch`'s stale `mesh_json`/`meshJson` (trivial rename,
  same evidence) so the field name stays consistent across languages; did **not** attempt full
  proto/graphql diff-schema depth parity (unbriefed, higher-risk, flagged below).

## Verification

- **ajv, snapshot**: `lowpoly-snapshot.json` fixture → PASS.
- **ajv, diff**: all 17 `🔺️diff/🔣️component.json` fixtures under `🧬️mutations/**` → **17/17 passed**.
- **ajv, mutations**: 17/17 passed + hostile double-tag fixture correctly rejected (shown above).
- **JSON syntax**: all 6 edited `.json` files parse clean (`python3 -m json` load).
- **Proto structural** (no protoc in repo, confirmed again): brace balance OK, 0 duplicate messages,
  0 unresolved field types, for artifact/snapshot/diff proto.
- **GraphQL structural** (no graphql-js in repo, confirmed again): brace balance OK, 0 duplicate
  types, 0 unresolved field types (2 regex false-positives from `@state(class: …)` directive args and
  one doc-comment string, manually confirmed non-references) for artifact/snapshot/diff graphql.
- **TypeScript, scoped `tsc`**: consumer at `🗑️generated/verify-schema-truth-ts.ts` imports real
  `LowpolyObject` from artifact/snapshot/diff `component.ts` plus `LowpolyObjectPatch`,
  `LowpolyObjectPatchEntry`, `LowpolyPaintLayersDelta`; exercises `mesh`/`null`, and two
  `@ts-expect-error` assertions (stale `meshJson` rejected; `paintLayers` rejected on
  `LowpolyObjectPatch`). `bunx tsc --noEmit --strict …` → exit 0. Proved real: copied the file,
  typo'd `paintLayers`→`paintLayersTYPO`, re-ran → `TS2561 … Did you mean to write 'paintLayers'?`,
  then deleted the copy (original confirmed byte-identical via `diff`).
- **Cross-language field-set diff** (`LowpolyObject`, `LowpolySnapshot`, `LowpolyDiff` top-level)
  across rust/ts/json/proto/graphql at every applicable level: **all MATCH** the Rust ground truth
  (script output captured; every representation reported `MATCH`, rust field count 38 for
  `LowpolyDiff`).

## Fixture-vs-Rust disagreement

None found. Every fixture inspected (17 mutation, 17 diff, 1 snapshot) matched the Rust struct
shapes exactly once the schemas were corrected — the bugs were entirely in the non-Rust schema
files, not in the fixtures or the Rust model.

## Known remaining gaps (found, not fixed — outside this pass's briefed scope)

- `🔺️diff/🛰️component.proto` and `🔺️diff/🔗️component.graphql`'s own `LowpolyObject`/
  `LowpolyObjectPatchEntry` messages/types remain shallow (`LowpolyObject` is just `{id, name}` in
  proto; `LowpolyObjectPatchEntry` has no `paintLayers` field in either) — same shallowness the prior
  agent flagged as suspected-but-unverified for JSON; now confirmed real for proto/graphql too, but
  bringing them to full parity is a materially larger, unbriefed change (would need ~10 more
  messages/types each) and wasn't attempted here.
- Artifact-level (non-diff) `Option<T>` scalar fields (e.g. `LowpolyArtifact.activeObjectId`) use the
  TS `field?: T` convention without `| null`, even though Rust serializes them as always-present+
  nullable too (same pattern as Bug 4, confirmed via the same serde reasoning) — this is a pre-existing,
  repo-wide convention applied consistently outside the diff facet and was not in scope to redesign.

## Files touched (15)

`🧬️schema/{🟦️component.ts,🔣️component.json,🛰️component.proto,🔗️component.graphql}`,
`🧬️schema/📸️snapshot/{same 4}`, `🧬️schema/🔺️diff/{same 4}`,
`🧬️schema/🧬️mutations/{🌱️create-object,➕️insert-paint-layer,🎨️edit-paint-layer}/🔣️payload.schema.json`.
Plus two kept verification inputs: `🗑️generated/verify-schema-truth.mjs`,
`🗑️generated/verify-schema-truth-ts.ts`. No Rust file, no `✏️editor/`, `🚪️io/`, `👁️viewer/`, or
`📦️packages/` file touched.
