# 🧪️ Handcrafted mutation fixtures — seven mid-size plugins (86 cases)

Slice: `➗️mathematical` (15) · `🔱️trinity` (15, two artifacts) · `🌍️gis` (14, two artifacts) ·
`🖨️raster` (12) · `📋️forms` (10) · `💡️reasoning` (10) · `🌊️flow` (10) — **86 leaves, 86 cases,
one case per leaf**, plus each plugin's own `📦️packages/🦀️rust/📦️glue.rs`.

`🚪️io/🧬️mutations` exists but is EMPTY in `➗️mathematical`, `📋️forms` and `💡️reasoning` — every
leaf lives under `🧬️schema/🧬️mutations`. Nine mutation trees in total.

Every case was derived from a direct read of its own leaf's `🔺️diff/🦀️component.rs` (the oracle),
`🦠️mutation/🦀️component.rs` and `↩️inverse/🦀️component.rs`. No shared harness, no macro, no
generic sweep, no loop over mutations: 587 hand-written test functions, 5–9 per case, each worded
for the verb it pins.

## ✅️ Verification (no `cargo` — workspace broken by the peer de-async sweep, per `📓️fixture-recipe.md`)

| check | result |
|---|---|
| `fixtures lint --by-tree` | `🧬️ 115 artifact mutation trees · 1558 mutations · 623 covered · 935 uncovered` — all nine of this slice's trees are gone from the uncovered list and **no error finding names any of the seven plugins** |
| lint logic replayed scoped to the slice | `scoped trees: 9 · errors: 0` (the 957 repo-wide errors and 4872 derived-encoding warnings are peers' trees / the expected `fixtures generate` gap) |
| `include_str!` targets | 404/404 resolve |
| glue `#[path]` targets | every `#[path]` in all seven `📦️glue.rs` resolves; 86 new `#[cfg(test)] mod tests_…` entries, one per case, no duplicates, no orphans |
| `rustfmt --edition 2021 --emit stdout` | 86 test files + 7 `📦️glue.rs` — 93/93 parse, 0 failures |
| JSON | 404/404 fixture JSON files parse |
| diff-JSON key sets | every applied `🔺️diff/🔣️component.json`'s top-level key list matches its Rust diff struct's serialized field list **exactly, in order** (all nine diff types carry container `#[serde(rename_all = "camelCase", default)]` with no per-field `skip_serializing_if`, so every slot is emitted, `null` when untouched) |
| mutation-JSON payload shapes | all 86 checked against their payload structs' real serde attrs (tag key, variant name casing, field casing, `skip_serializing_if`) — 0 mismatches |
| rejected-case contract | 26/26 carry a 0-byte `🔺️diff/🚫️component.absent`, no diff JSON, and an `➡️after` byte-identical to `⬅️before` |

No `cargo` was run and **no test is claimed to pass**. No git-modifying command, no ticket
open/close/reopen, no repo-MCP `search`.

## 🧭️ Outcome mix and why

| bucket | count | when it applies |
|---|---|---|
| applied, real state change, full `🔺️diff/🔣️component.json` | 34 | the artifact's changed state is hand-authorable |
| applied no-op (`warn mutation.no-op`, empty diff = the diff type's `Default`) | 26 | the leaf has a real no-op guard that returns **before** any content handle is minted |
| rejected (`🔺️diff/🚫️component.absent`) | 26 | the leaf has no no-op guard and every applying path re-mints a hashed handle |

The split is forced by architecture, not by convenience. `🌊️flow`, `💡️reasoning/🔌️wires`,
`🔱️trinity/🔌️jack`, `📋️forms` and (for its graph/geometry verbs) `➗️mathematical` persist their real
content in a composed CHILD whose `child_id` is a `std::collections::hash_map::DefaultHasher` digest
of the child content (`flow-content-…`, `wires-content-…`, `jack-content-…`, `forms-scene-…`,
`mathematical-scene-…`). Hand-forging such an `➡️after` would mean forging a value `std`
deliberately leaves unspecified, so those trees pin the branches that reach no hash at all — exactly
the precedent `🕸️dag` set (`…/✂️disconnect-nodes/🧪️tests/rejects-disconnecting-a-missing-edge/` and
`…/🌱create-node/🧪️tests/rejects-a-duplicate-node-id/`, which seeds the working-scene cache from the
committed payload). Where a leaf had a genuine no-op guard, the applied no-op case was preferred over
a rejection: it keeps the mandatory `🔺️diff/🔣️component.json` and all seven core assertions.

`🔱️trinity/♻️rewrite` (fully inline snapshot), `🖨️raster` (inline layer tree) and `🌍️gis` (see below)
are the trees that carry real state-changing applied cases.

## 📋️ Case inventory

### `➗️mathematical` — `🗿️artifacts/➗️mathematical` (15)
| leaf | case | outcome |
|---|---|---|
| `🔄️change-coefficient` | `raises-the-leading-coefficient-to-three-halves` | applied (inline `equation` only) |
| `➕️insert-point` | `seeds-the-empty-cloud-with-its-first-point` | applied |
| `🔀️change-graph-directed` | `keeps-an-already-directed-graph-directed` | applied no-op |
| `🧮️update-graph-algorithm` | `restates-the-unset-algorithm-and-its-absent-seed` | applied no-op |
| `🔁️replace-graph` | `replays-the-identical-empty-graph` | applied no-op |
| `🌀️replace-points` | `replays-the-identical-empty-point-cloud` | applied no-op |
| `🟢️create-node` | `rejects-a-duplicate-node-id` | rejected `mutation.duplicate-id` (Fatal) |
| `➖️remove-point` | `rejects-removing-a-point-from-an-empty-cloud` | rejected `mutation.target-missing` |
| `🎯️move-point` | `rejects-moving-a-point-that-is-not-in-the-cloud` | rejected `mutation.target-missing` |
| `❌️delete-node` | `rejects-deleting-a-node-that-is-not-in-the-graph` | rejected `mutation.target-missing` |
| `🗑️delete-nodes` | `rejects-a-bulk-delete-where-every-id-is-absent` | rejected `mutation.target-missing` |
| `🔗️connect-nodes` | `rejects-an-edge-between-two-absent-endpoints` | rejected `mutation.target-missing` |
| `✂️disconnect-nodes` | `rejects-severing-an-edge-that-is-not-in-the-graph` | rejected `mutation.target-missing` |
| `🕹️move-node` | `rejects-moving-a-node-that-is-not-in-the-graph` | rejected `mutation.target-missing` |
| `🏷️change-node-label` | `rejects-relabelling-a-node-that-is-not-in-the-graph` | rejected `mutation.target-missing` |

`➕️insert-point` is the one mathematical verb with **neither** a rejection branch **nor** a no-op
guard — an out-of-range index is CLAMPED (Warning `mutation.clamped`), never refused. Its committed
`childId`s are therefore documented placeholders that the test substitutes at run time through the
plugin's own `mathematical_children_from_state`, fed the hand-authored `(graph, geometry)` pair each
side is claimed to hold. Nothing hashed is frozen into a committed file. Same treatment for
`🟢️create-node`'s seeded scene, which is built entirely from the committed payload's own node.

### `🔱️trinity` — `🗿️artifacts/♻️rewrite` (7) + `🗿️artifacts/🔌️jack` (8)
| leaf | case | outcome |
|---|---|---|
| `♻️rewrite/🎯️edit-rhs` | `rewrites-the-rhs-to-set-a-second-property` | applied |
| `♻️rewrite/🔍️edit-lhs` | `narrows-the-lhs-pattern-to-a-shaft-neighbour` | applied |
| `♻️rewrite/🖼️edit-before-fixture` | `swaps-in-a-two-node-before-graph` | applied |
| `♻️rewrite/🔧️change-parameter-binding` | `retitles-the-caption-binding` | applied |
| `♻️rewrite/🧹️remove-parameter-binding` | `drops-the-repeat-binding` | applied |
| `♻️rewrite/📐️change-rule-layout-point` | `nudges-the-capsule-var-off-the-shaft` | applied |
| `♻️rewrite/🗑️remove-rule-layout-point` | `clears-the-shaft-layout-point` | applied |
| `🔌️jack/✏️rename-node` | `keeps-the-name-a-node-already-carries` | applied no-op |
| `🔌️jack/📍️move-node` | `keeps-a-node-at-the-point-it-already-occupies` | applied no-op |
| `🔌️jack/🔧️change-data-property` | `keeps-a-node-property-at-the-value-it-already-holds` | applied no-op |
| `🔌️jack/🧹️remove-data-property` | `keeps-an-edge-without-the-property-it-never-had` | applied no-op |
| `🔌️jack/🌱️create-node` | `rejects-a-node-id-the-scene-already-holds` | rejected `mutation.duplicate-id` (Fatal) |
| `🔌️jack/🗑️delete-node` | `rejects-deleting-a-node-the-scene-never-had` | rejected `mutation.target-missing` |
| `🔌️jack/🔗️create-edge` | `rejects-an-edge-whose-endpoints-are-absent` | rejected `mutation.invariant` (Fatal) |
| `🔌️jack/✂️delete-edge` | `rejects-cutting-an-edge-the-scene-never-had` | rejected `mutation.target-missing` |

All eight jack diff builders end in `diff_replace_content` on their changing path
(`jack_content_child_handle` → `jack-content-{DefaultHasher:016x}`). Every seeded jack case uses a
distinct `childId` so the thread-local scratch cache cannot cross-contaminate between cases.

### `🌍️gis` — `🗿️artifacts/🗺️gismap` (12) + `🗿️artifacts/🏔️gisterrain` (2)
All 14 are **applied with real state changes** and a full `🔺️diff/🔣️component.json`.

| leaf | case |
|---|---|
| `🗺️gismap/🆕create-position` | `adds-a-lighthouse-position-after-the-harbor` |
| `🗺️gismap/🗑delete-position` | `removes-the-lighthouse-position` |
| `🗺️gismap/🔁replace-position-data` | `rewrites-the-harbor-position-payload` |
| `🗺️gismap/🔀reorder-positions` | `moves-the-harbor-position-to-the-end` |
| `🗺️gismap/🛣️create-route` | `adds-a-tram-route-after-the-ferry` |
| `🗺️gismap/✂️delete-route` | `removes-the-tram-route` |
| `🗺️gismap/♻️replace-route-data` | `rewrites-the-ferry-route-payload` |
| `🗺️gismap/🧭reorder-routes` | `moves-the-bus-route-to-the-front` |
| `🗺️gismap/🌐create-region` | `adds-the-old-town-region-after-the-harbor-district` |
| `🗺️gismap/🧹delete-region` | `removes-the-old-town-region` |
| `🗺️gismap/🔄replace-region-data` | `rewrites-the-harbor-district-region-payload` |
| `🗺️gismap/🔃reorder-regions` | `moves-the-park-region-between-the-two-districts` |
| `🏔️gisterrain/🎚change-exaggeration` | `raises-the-exaggeration-from-one-to-two-and-a-half` |
| `🏔️gisterrain/📥change-imported-features` | `imports-a-single-harbor-position-descriptor` |

gis differs from the other composed plugins in a decisive way: neither `GisMapDiff` nor
`GisTerrainDiff` carries a child field, and **`MutationDiff::apply` itself calls the derivation
funnel** (`gis_map_snapshot_with_derived_children` / `gis_terrain_snapshot_with_derived_mesh`), so
the diff path and the mutation path re-mint identically. The committed snapshots carry readable
placeholders (`gismap-drawing-derived`, `gismap-value-derived`, `gisterrain-mesh-derived`) in the
derived child slots ONLY, and `before()`/`expected_after()` funnel the decoded JSON through the
artifact's own derivation before any assertion. No hash is frozen into any file; every artifact-lane
value (`positions`/`routes`/`regions`, `exaggeration`, `importedFeaturesJson`) is hand-authored and
asserted verbatim. Each case's module docstring states this.

### `🖨️raster` — `🗿️artifacts/🖨️raster` (12)
| leaf | case | outcome |
|---|---|---|
| `👁️change-layer-visible` | `hides-the-overlay-layer` | applied |
| `↔️move-layer` | `slides-the-stamp-layer-off-the-origin` | applied |
| `🌱create-layer` | `creates-an-ink-layer-inside-the-artwork-group` | applied |
| `🎚️change-layer-adjustment-kind` | `switches-the-tone-layer-from-levels-to-curves` | applied |
| `✏️rename-layer` | `renames-the-sketch-layer-to-final-linework` | applied |
| `📐resize-layer` | `resizes-the-canvas-layer-to-256-by-128` | applied |
| `🎨change-layer-blend-mode` | `switches-the-glow-layer-to-screen` | applied |
| `🔀reorder-layers` | `lifts-the-caption-layer-out-of-the-frame-group` | applied |
| `🌫️change-layer-opacity` | `fades-the-highlight-layer-to-a-quarter` | applied |
| `🗑️delete-layer` | `deletes-the-frame-group-and-its-nested-children` | applied |
| `🖇️add-layer-asset` | `declines-to-reattach-an-asset-already-on-the-document` | applied no-op |
| `🗂️remove-layer-asset` | `rejects-removing-an-asset-the-document-never-attached` | rejected `mutation.target-missing` |

The ten layer verbs are fully hand-authorable. The two asset verbs are not:
`RasterDiff.assets` is `BTreeMap<String, Option<RasterImageAsset>>` — raw bytes, not the handle —
and `MutationDiff::apply` runs `mint_and_stash_asset`, producing
`raster-asset-{DefaultHasher:016x}`. `add` mints on the forward path and `remove`'s inverse
(`add-layer-asset`) mints on the undo path, so both take the no-op / rejection route. Both fixture
handles use `child_id: "raster-asset-unresolved-fixture"`, a shape the minting path can never
produce, making the scratch-cache miss deterministic regardless of test order.

### `📋️forms` — `🗿️artifacts/📋️forms` (10)
| leaf | case | outcome |
|---|---|---|
| `🏷️change-form-title` | `titles-an-untitled-survey` | applied (inline `title` only) |
| `✏️rename-step` | `no-ops-when-the-step-already-carries-that-title` | applied no-op |
| `📝change-step-description` | `no-ops-when-clearing-an-already-absent-description` | applied no-op |
| `🔀reorder-step` | `no-ops-when-the-step-already-sits-at-that-index` | applied no-op |
| `🔁replace-block` | `no-ops-when-the-replacement-block-is-identical` | applied no-op |
| `📦move-block-to-step` | `no-ops-when-the-block-stays-at-its-index-in-its-own-step` | applied no-op |
| `🌱create-step` | `rejects-a-duplicate-step-id` | rejected `mutation.duplicate-id` (Fatal) |
| `➕create-block` | `rejects-a-block-for-a-step-that-does-not-exist` | rejected `mutation.invariant` (Fatal) |
| `➖delete-block` | `rejects-deleting-a-block-missing-from-an-existing-step` | rejected `mutation.target-missing` (2-segment target) |
| `🗑️delete-step` | `rejects-deleting-a-step-the-scene-does-not-hold` | rejected `mutation.target-missing` |

`🏷️change-form-title`'s diff builder never touches `structure`/`results`, so it is the one honestly
applicable real applied case; every other verb routes through `forms_diff_from_delta`, which re-mints
both hashed child handles.

### `💡️reasoning` — `🗿️artifacts/🔌️wires` (10)
| leaf | case | outcome |
|---|---|---|
| `📐resize-node` | `reports-a-no-op-when-the-radius-already-matches` | applied no-op |
| `🏷️change-node-kind` | `reports-a-no-op-when-the-kind-already-reads-topic` | applied no-op |
| `🚩set-node-root` | `reports-a-no-op-when-an-unflagged-node-is-set-to-not-root` | applied no-op |
| `🧭move-node` | `reports-a-no-op-when-a-y-less-node-is-moved-to-y-zero` | applied no-op |
| `✏️edit-node-text` | `reports-a-no-op-when-the-label-is-retyped-verbatim` | applied no-op |
| `🔷change-node-shape` | `reports-a-no-op-when-the-shape-already-reads-circle` | applied no-op |
| `🌱create-node` | `rejects-a-node-id-the-board-already-holds` | rejected `mutation.duplicate-id` (Fatal) |
| `🗑️delete-node` | `rejects-deleting-a-node-the-board-never-held` | rejected `mutation.target-missing` |
| `🔗connect-nodes` | `rejects-an-edge-whose-source-node-is-absent` | rejected `mutation.target-missing` (on the endpoint node id) |
| `✂️disconnect-nodes` | `rejects-cutting-an-edge-the-board-never-carried` | rejected `mutation.target-missing` |

wires shares dag's verb vocabulary, so each case deliberately pins something dag's cannot: resize's
payload-masked inverse, move's `node_position` origin default, `set-node-root`'s `unwrap_or(false)`
read of an absent key, `edit-node-text` as the vocabulary's sole `edit` verb, the `kind`/`shape` pair
separated only by descriptor, and — for `connect-nodes` — the one place where the diagnostic target
(`node-alpha`) and `SemanticMutation::target()` (`edge-alpha-beta`) disagree. Seeding comes from the
committed snapshot's own persisted `wiresFixture.board` mirror, nothing invented.

### `🌊️flow` — `🗿️artifacts/🌊️flow` (10)
| leaf | case | outcome |
|---|---|---|
| `🔗️connect-widgets` | `refuses-a-parallel-synapse-as-a-no-op` | applied no-op |
| `🔀️🪟️reorder-widgets` | `clamps-an-out-of-range-index-onto-the-last-slot` | applied no-op |
| `🔀️reorder-synapses` | `keeps-the-leading-synapse-at-index-zero` | applied no-op |
| `🔁️replace-widget` | `replaces-a-note-with-an-identical-note` | applied no-op |
| `🔄️update-synapse-endpoints` | `re-declares-the-same-endpoints` | applied no-op |
| `📍️move-widgets` | `re-applies-the-current-layout-to-both-widgets` | applied no-op |
| `➕️create-widget` | `rejects-a-duplicate-widget-id` | rejected `mutation.duplicate-id` (Fatal) |
| `🗑️delete-widget` | `rejects-deleting-a-missing-widget` | rejected `mutation.target-missing` |
| `✂️disconnect-widgets` | `rejects-disconnecting-a-missing-synapse` | rejected `mutation.target-missing` |
| `👯️duplicate-widget` | `rejects-duplicating-onto-a-taken-id` | rejected `mutation.invariant` (Fatal, empty target) |

`🌊️flow`'s `🧩️extensions/` sub-crates carry no `🧬️mutations` leaf, so nothing there is in scope.

## 🔧️ One shared-file edit (inside the slice)

`✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🦀️component.rs` gained one function:

```rust
pub async fn cache_forms_steps(child_id: &str, steps: Vec<FormStep>)
```

the id-keyed twin of `forms_children_from_steps`, which only ever seeds the scratch cache under the
hash it itself mints — meaning no persisted handle could ever be resolved, and `🌱create-step`
(no no-op guard, no rejection reachable on an empty scene) had no authorable case at all.
`forms_children_from_steps` now delegates to it, so behaviour is unchanged. This is exactly dag's
`cache_dag_content` and playbook's `cache_playbook_steps`, both already committed. Eight of the ten
forms cases depend on it.

## ⚠️ serde traps found (none of the puzzle5d conventions transfer wholesale)

- `MathematicalMutation`, `GisMapMutation` and `GisTerrainMutation` carry **no `#[serde(tag = …)]`
  and no `rename_all`** — they are externally tagged with PascalCase variant names
  (`{"InsertPoint":{…}}`, `{"CreatePosition":{…}}`), and their payload fields stay **snake_case**.
- Payload structs frequently lack their own `rename_all` even when the enum has one: jack's
  `RenameNode.new_name`, forms' `RenameStep.new_title`/`step_id`/`to_index`, and flow's
  `DuplicateWidget.source_id`/`new_id` all serialize snake_case under a camelCased tag.
- jack's `EntityRef` is *adjacently* tagged (`tag = "entity", content = "id"`), so it nests as
  `{"entity":"node","id":"capsule-a"}` inside an internally-tagged mutation — a doubled `entity`
  key that looks wrong and is correct.
- `FormsSnapshot::title` has `skip_serializing_if = "Option::is_none"`, so an untitled snapshot must
  omit the key entirely — `"title": null` breaks the canonical-JSON assertion.
- `protocol::Severity`'s variant is `Warning` and serializes as `"warning"`, while the recipe's
  `🎯️outcome` vocabulary spells the level `"warn"`. Every no-op case asserts both sides explicitly
  rather than assuming they match.
- `MutationOutcome::warn`/`info` are the 2-arg chainable builders and attach **no** target, unlike
  the static `error`/`fatal` shortcuts — so no-op diagnostics carry an empty `target`.
- `➗️mathematical`, `💡️reasoning/🔌️wires` and `📋️forms` expose **no** `apply_…_mutation` /
  `inverse_…_mutation` free function (deleted in favour of `#[derive(dsl::Mutations)]`); their tests
  go through `protocol::Mutation` / `protocol::MutationDiff` / `store::apply_mutation` directly,
  matching each tree's own committed tests.
- This slice's leaves carry no `#[dsl(keyword = …)]`. The lint tolerates it (it uses `keyword` only
  for reporting), but `fixtures generate` may not — worth checking before D1's derived encodings.

## ❓️ Open notes for whoever runs `fixtures generate`

1. The placeholder `child_id`s in `➗️mathematical`'s two applied cases and in all 14 gis snapshots
   will be encoded verbatim into the derived `.dsl.semio`/`.pack.semio`. That is unavoidable without
   resolving the digest, but those encodings must not then be treated as ground truth for the handle.
2. Pre-existing and untouched (spotted in passing, outside this slice's edit boundary):
   `✏️s/🔌️plugins/🌊️flow/…/🧬️schema/🔺️diff/📝️text/🦀️component.rs`'s own test writes
   `let diff: FlowDiff = operation.diff(&base);` while `Mutation::diff` returns
   `MutationOutcome<FlowDiff>`. No `cargo` was run, so this is a read-only observation.
