# B1 — per-subset catalog scoping (test-only-mutation repair)

## Root cause, precisely

`mutationInventoryBreaches` (🟦️.ts:4629) computes, for **each manifest it iterates**:

```
claimed = registry.mutationCatalogs
  .filter(catalog => catalog.capability !== "" && manifest.mutations.some(m => m.capability === catalog.capability))
  .flatMap(catalog => catalog.kinds)
```

`registry.mutationCatalogs` is the **whole repo's flat, merged list** — not scoped to the manifest's
own owner directory. The filter keys on **`capability`**, not on catalog `id`. Every note/mathematical/
sequence/draw subset catalog I inspected already had its `kinds` list correctly scoped to its own
manifest (I verified this per-file before touching anything) — the earlier a8 wave got the *kinds*
right. But every sibling subset of one artifact shared **one capability string**
(`note-1-mutate`, `mathematical-1-mutate`, `sequence-1-mutate`, `draw-1-mutate`). Because the filter
matches by capability across the *entire* registry, checking e.g. `note-1-ink`'s manifest pulled in
the kinds of `note-1-asset`, `note-1-block`, `note-1-canvas`, … too — every one of those foreign kinds
then failed `manifest.mutations.has(id)` and became a `test-only-mutation` breach. This is why the
breach count (231 for note) is so much larger than the kind-count mismatch you'd expect from reading
any single file: it's an N×(N-1) cross-product effect of N sibling catalogs sharing one capability.

`step`'s `cc1..cc6` already used a **distinct capability per subset** (`step-ap214-cc6-mutate`, …) —
that's why only 1 of step's breaches was this class at all, and it had a different cause (below).

**The fix**: give every subset's catalog + manifest mutations a **capability string unique to that
subset** (`<catalog-id>-mutate`). This is a pure **top-level `mutation.capability` / `catalog.capability`**
change — I left `oracleRequirements[].capability` (the field that actually gates third-party-oracle
qualification, `oracleRequirementBreaches` at 🟦️.ts:4698) untouched everywhere, since that field is
independently validated and still points at each artifact's real shared oracle registration
(`note-1-mutate`, `mathematical-1-mutate`, …). Verified in code that no rule requires
`mutation.capability === oracleRequirements[].capability`; they are validated independently
(`mutationManifestProblems`, 🟦️.ts:2852 vs 2868). No manifest kinds were deleted anywhere — only the
`capability` field values changed, plus one bad catalog *kind* removed (step, see below) and draw's
leftover wildcard catalog emptied.

## Catalog table — what B3 should point features at

| Artifact | Standard | Subset | Catalog id (`@mutations-`) | Capability (`@capability-`) | Kinds |
|---|---|---|---|---|---|
| note | 1 | asset | `note-1-asset` | `note-1-asset-mutate` | create-asset, delete-asset, replace-asset-payload |
| note | 1 | block | `note-1-block` | `note-1-block-mutate` | change-block-font-size, change-block-locked, change-block-visible, create-block, delete-block, delete-blocks, drag-blocks, duplicate-block, duplicate-blocks, move-block, move-block-to-container, rename-block, resize-block |
| note | 1 | canvas | `note-1-canvas` | `note-1-canvas-mutate` | change-grid-opacity, change-grid-spacing, change-grid-subdivisions, change-grid-visible, change-snap-enabled, change-snap-grid-spacing |
| note | 1 | document | `note-1-document` | `note-1-document-mutate` | rename-note |
| note | 1 | ink | `note-1-ink` | `note-1-ink-mutate` | change-block-ink-width, change-eraser-radius, change-pencil-width, edit-block-ink-stroke |
| note | 1 | math | `note-1-math` | `note-1-math-mutate` | edit-block-math |
| note | 1 | table | `note-1-table` | `note-1-table-mutate` | insert-table-column, insert-table-row, remove-table-column, remove-table-row |
| note | 1 | text | `note-1-text` | `note-1-text-mutate` | edit-block-text |
| draw | 1 | metadata | `draw-1-metadata` | `draw-1-metadata-mutate` | rename-layer, set-layer-locked, set-layer-visible |
| draw | 1 | structure | `draw-1-structure` | `draw-1-structure-mutate` | create-layer, delete-layer, duplicate-layer, reorder-layer |
| draw | 1 | style | `draw-1-style` | `draw-1-style-mutate` | replace-layer-fill, replace-layer-stroke, set-layer-blend-mode, set-layer-opacity |
| draw | 1 | transform | `draw-1-transform` | `draw-1-transform-mutate` | set-layer-boolean-operation, update-layer-trace-params, update-layer-transform |
| mathematical | 1 | equation | `mathematical-1-equation` | `mathematical-1-equation-mutate` | change-coefficient |
| mathematical | 1 | geometry | `mathematical-1-geometry` | `mathematical-1-geometry-mutate` | insert-point, move-point, remove-point, replace-points |
| mathematical | 1 | graph | `mathematical-1-graph` | `mathematical-1-graph-mutate` | change-graph-directed, change-node-label, connect-nodes, create-node, delete-node, delete-nodes, disconnect-nodes, move-node, replace-graph, update-graph-algorithm |
| sequence | 1 | dependency | `sequence-1-dependency` | `sequence-1-dependency-mutate` | connect-steps, disconnect-steps |
| sequence | 1 | step | `sequence-1-step` | `sequence-1-step-mutate` | change-step-collapsed, create-step, delete-step, duplicate-step, edit-step-params, move-step |
| step | ap214 | any | `step-ap214-any` | `step-ap214-any-mutate` | (unchanged — already unique; `no-mutation` kind dropped, see below) |
| step | ap214 | cc1..cc6 | `step-ap214-cc<N>` | `step-ap214-cc<N>-mutate` | (unchanged — already unique; `no-mutation` kind dropped from cc1-cc6 too) |

**⚠️ Critical for B3**: every retagged feature must carry **both** the new `@mutations-<catalog-id>`
tag **and** a matching `@capability-<capability>` tag from the table above. `@capability-` is what
`buildCasePlan` uses (🟦️.ts:1061) to bind a case to its owning manifest; the old shared tags
(`@capability-note-1-mutate`, `@capability-draw-1-mutate`, `@capability-mathematical-1-mutate`,
`@capability-sequence-1-mutate`) no longer match **any** manifest mutation in these five artifacts —
only `oracleRequirements[].capability` (unrelated to feature binding) still uses those old strings.
I already see the fallout of this in the live registry: the two `mutate-sequence-1-dependency` /
`mutate-sequence-1-step` `🥒️.feature` files B3 has already written carry `@mutations-sequence-1-dependency`
/ `@mutations-sequence-1-step` (correct, new) but still `@capability-sequence-1-mutate` (stale) — that
is producing 2 `mutation-catalog-capability-mismatch` breaches right now. Retagging those two features'
`@capability-` line to `sequence-1-dependency-mutate` / `sequence-1-step-mutate` closes them.

## draw: the `draw-1-any` reuse trade is now void

`✳️any/🧪️oracle/🔣️.json` still declared a `draw-1-any` catalog with all 14 kinds from all four real
subsets and zero manifests (dead leftover from before the split), and all four real subsets
(metadata/structure/style/transform) also each declared their own `mutationCatalogs` entry reusing
that **same** `draw-1-any` id (with their own already-correct kinds — just the id was shared). I:
- Emptied `✳️any`'s `mutationCatalogs` to `[]` (it owns no manifest, matching note/mathematical/sequence's
  `✳️any` pattern; its shared oracle registrations `quick-xml-draw-1-mutate` /
  `serde-json-draw-carrier-reader` and the `draw-mutation-semantics` no-oracle decision are untouched).
- Gave each real subset its own `draw-1-<subset>` id + `draw-1-<subset>-mutate` capability.

This produces one new `unknown-mutation-catalog` breach: the old artifact-level
`✏️s/🔌️plugins/🖍️draw/…/🧪️tests/mutate-draw-1/🥒️.feature` still tags `@mutations-draw-1-any`, which no
longer resolves. This is the exact trade the shard brief called out as "now void" — B3 splitting that
feature into per-subset cases against the table above closes it.

## step: separate, smaller cause — a test-only sentinel leaking into the catalog vocabulary

step's `cc1..cc6` (and `✳️any`) already used per-subset capabilities, so they were never subject to
the cross-subset union bug above. Their catalogs each declared a `no-mutation` **kind** that no
manifest owns. I traced it: `no-mutation` is a deliberate **control/baseline row** used inside every
`mutate-step-ap214*` `🥒️.feature`'s `Examples` table and inside every conformance class's
`🧪️oracle/🦀️.rs` (`KINDS` const, dispatch `"no-mutation" => {}`) to produce an unmutated baseline
projection for comparison — **not** a real mutation. Production dispatch confirms this deliberately:
every `StepCc<N>Mutation` / `StepMutation` enum's own doc-comment says *"`NoMutation` is GONE —
`#[derive(dsl::Mutations)]` requires every variant to wrap exactly one leaf payload and a unit variant
wraps none"* — there never was, and now can never be, a `NoMutation` production dispatch variant. Per
the rule's own remedy ("Add the mutation to its owning manifest, **or drop the catalog kind**"),
adding it to any manifest would immediately create a `manifest-only-mutation` breach (no dispatch
behind it), so the correct fix is dropping it from every catalog's `kinds` list. Done for `✳️any` and
`cc1`..`cc6`. cc1-cc5 had **zero** `mutationManifests` at the start of my run (a separate, pre-existing
`capability-without-manifest` gap, 6 breaches, unrelated to this ticket's earlier waves) — another
concurrent session added their manifests mid-session (I re-read and adapted, per house rules); once
they existed, cc1-cc5 showed the identical `no-mutation` mismatch cc6 already had, so I applied the
same one-line fix to all of them. I did not touch that concurrent session's new manifest content,
and I did not touch cc1-cc5's pre-existing `capability-without-manifest` breaches — out of my scope
(the brief scoped step to its one `test-only-mutation` breach).

## Counts

### `test-only-mutation` (the gate for this shard)

| Artifact | Before | After |
|---|---:|---:|
| note | 231 | 0 |
| draw | 42 | 0 |
| mathematical | 30 | 0 |
| sequence | 8 | 0 |
| step | 1 | 0 |
| **Total** | **312** | **0** |

### Guard classes (must not rise) — confirmed unchanged, all zero before and after in my 5 territories

`manifest-only-mutation`, `duplicate-mutation-owner`, `mutation-outcome-mismatch`,
`mutation-variant-mismatch` — 0 before, 0 after, in all five artifacts.

### `mutation-catalog-unclaimed` — handed to B3

Repo-wide total: **31 → 33**. Of the 33, **15 are mine** (rest belong to other shards' territories —
fem2d/fem3d/gltf, 18, not mine):

- `note-1-asset`, `note-1-block`, `note-1-canvas`, `note-1-document`, `note-1-ink`, `note-1-math`,
  `note-1-table`, `note-1-text` (8 — pre-existing before my edit, capability rename doesn't affect
  this axis, it's `id`/`@mutations-` driven)
- `mathematical-1-equation`, `mathematical-1-geometry`, `mathematical-1-graph` (3 — pre-existing)
- `draw-1-metadata`, `draw-1-structure`, `draw-1-style`, `draw-1-transform` (4 — **new**, direct
  result of giving draw real per-subset ids instead of the shared `draw-1-any`)

`sequence-1-dependency` / `sequence-1-step` are **not** in the unclaimed list — B3 already retagged
sequence's features live during this session. `step-ap214-*` were never unclaimed.

### New breach B3 should also expect: `mutation-catalog-capability-mismatch` (2, sequence only so far)

Caused by the interaction above — B3's already-written sequence features carry the right
`@mutations-` tag but the stale `@capability-sequence-1-mutate` tag. Retagging closes both. Expect the
same pattern once B3 retags note/draw/mathematical features unless it uses the capability column of
the table above from the start.

### New breach B3 should also expect: `unknown-mutation-catalog` (1, draw)

`mutate-draw-1/🥒️.feature` still references the now-removed `draw-1-any` id (see draw section above).

## Files touched

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/{✳️asset,✳️block,✳️canvas,✳️document,✳️ink,✳️math,✳️table,✳️text}/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/{✳️any,✳️metadata,✳️structure,✳️style,✳️transform}/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/{✳️equation,✳️geometry,✳️graph}/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/{✳️dependency,✳️step}/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/{✳️any,✳️cc1,✳️cc2,✳️cc3,✳️cc4,✳️cc5,✳️cc6}/🧪️oracle/🔣️.json`

No `🧪️tests` directory or `🥒️.feature` file was touched (B3's territory, respected throughout).
