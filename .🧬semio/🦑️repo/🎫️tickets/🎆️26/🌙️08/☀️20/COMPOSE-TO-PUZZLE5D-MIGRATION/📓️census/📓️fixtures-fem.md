# 🏗️ FEM plugin — handcrafted mutation fixtures (50 cases)

Slice: both mutation trees of `✏️s/🔌️plugins/🏗️fem`, plus the plugin's own
`📦️packages/🦀️rust/📦️glue.rs` wiring. Nothing outside the plugin was touched.

## Result

| tree | leaves | covered | uncovered |
| --- | --- | --- | --- |
| `🗿️artifacts/◻2d/…/🧬️schema/🧬️mutations` | 25 | 25 | **0/25** |
| `🗿️artifacts/🧊️3d/…/🧬️schema/🧬️mutations` | 25 | 25 | **0/25** |

`bun ./📜️script.ts fixtures lint --by-tree` (run from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`)
lists **no `🏗️fem` row at all** — neither in the `--by-tree` uncovered list nor among the error
lines. Only the expected derived-encoding warnings remain (`.op.semio` / `.spr.semio` /
`.patch.semio` / `.dsl.semio` / `.pack.semio` are produced by `fixtures generate`, contract D1/D11,
and were deliberately not hand-forged).

## Layout per case

Every leaf carries exactly one case:

```
<leaf>/🧪️tests/<case>/
  📸️snapshot/⬅️before/🔣️component.json
  📸️snapshot/➡️after/🔣️component.json
  🦠️mutation/🔣️component.json
  🔺️diff/🔣️component.json      ← transcribed from that leaf's own 🔺️diff/🦀️component.rs
  🎯️outcome/🔣️component.json   ← {"status":"applied"} on all 50
  🦀️component.rs               ← 7 laws + 2–3 extra assertions worded for that mutation
```

No case is `rejected`, so no `🔺️diff/🚫️component.absent` file exists in this slice.

## The 50 cases

### `◻2d` — `Fem2dSnapshot` / `Fem2dMutation` / `Fem2dDiff`

| leaf | case | diff shape |
| --- | --- | --- |
| `🌱⚪️create-node` | `appends-node-n3` | `nodes.added` |
| `🗑⚪️delete-node` | `removes-node-n3-without-cascading-to-its-support` | `nodes.removed` |
| `🌱🧩️create-element` | `appends-bar-e2-between-n2-and-n3` | `elements.added` |
| `🗑🧩️delete-element` | `removes-bar-e2-and-keeps-its-end-nodes` | `elements.removed` |
| `🔁🧩️replace-element` | `converts-beam-e1-into-a-bar-in-place` | `elements.patched` |
| `🌱🧱️create-material` | `appends-concrete-c30` | `materials.added` |
| `🗑🧱️delete-material` | `removes-the-unreferenced-timber-material` | `materials.removed` |
| `🔁🧱️replace-material` | `restates-steel-as-s355-in-its-original-slot` | `materials.patched` |
| `🌱create-section` | `appends-the-ipe300-profile` | `sections.added` |
| `🗑📐️delete-section` | `removes-the-spare-hollow-section` | `sections.removed` |
| `🔁📐️replace-section` | `stiffens-ipe200-with-a-reinforced-profile` | `sections.patched` |
| `🌱🛡️create-support` | `adds-a-vertical-roller-at-node-n2` | `supports.added` |
| `🗑delete-support` | `releases-the-roller-at-node-n2` | `supports.removed` |
| `🔁replace-support` | `upgrades-the-roller-at-n2-to-a-full-fixity` | `supports.patched` |
| `🌱🗺️create-region` | `appends-a-solid-rectangular-slab` | `regions.added` |
| `🗑🗺️delete-region` | `removes-the-slab-and-keeps-its-material` | `regions.removed` |
| `🔁🗺️replace-region` | `punches-a-stair-opening-through-the-slab` | `regions.patched` |
| `🌱📋️create-load-case` | `appends-a-live-case-carrying-one-nodal-load` | `loadCases.added` |
| `🗑📋️delete-load-case` | `removes-the-live-case-together-with-its-loads` | `loadCases.removed` |
| `➕add-load` | `appends-a-member-udl-to-the-dead-case` | `loadCases.patched` |
| `➖remove-load` | `strips-the-trailing-member-udl-from-the-dead-case` | `loadCases.patched` |
| `⚖change-load-case-self-weight` | `switches-self-weight-on-for-the-dead-case` | `loadCases.patched` |
| `🌱🔗️create-combination` | `appends-an-uls-combination-over-both-cases` | `combinations.added` |
| `🗑🔗️delete-combination` | `removes-the-uls-combination-and-keeps-both-cases` | `combinations.removed` |
| `🎛update-analysis-settings` | `doubles-the-modal-count-and-halves-the-deformation-scale` | scalar `analysis` |

### `🧊️3d` — `Fem3dSnapshot` / `Fem3dMutation` / `Fem3dDiff`

| leaf | case | diff shape |
| --- | --- | --- |
| `🌱⚪️create-node` | `appends-the-column-head-node-n3` | `nodes.added` |
| `🗑⚪️delete-node` | `removes-the-column-head-node-under-a-live-frame` | `nodes.removed` |
| `🌱🧩️create-element` | `appends-a-diagonal-bracing-bar` | `elements.added` |
| `🗑🧩️delete-element` | `removes-the-bracing-bar-and-leaves-the-frame` | `elements.removed` |
| `🔁🧩️replace-element` | `rolls-the-column-about-its-own-axis` | `elements.patched` |
| `🌱🧱️create-material` | `appends-an-aluminium-alloy` | `materials.added` |
| `🗑🧱️delete-material` | `removes-the-unreferenced-aluminium-alloy` | `materials.removed` |
| `🔁🧱️replace-material` | `softens-the-steel-shear-modulus-in-place` | `materials.patched` |
| `🌱create-section` | `appends-a-square-hollow-profile` | `sections.added` |
| `🗑📐️delete-section` | `removes-the-spare-square-hollow-profile` | `sections.removed` |
| `🔁📐️replace-section` | `raises-the-torsion-constant-of-hea200` | `sections.patched` |
| `🌱🛡️create-support` | `clamps-the-column-base-in-all-six-dofs` | `supports.added` |
| `🗑delete-support` | `releases-the-pinned-node-n2` | `supports.removed` |
| `🔁🛡️replace-support` | `frees-the-three-rotations-at-the-column-base` | `supports.patched` |
| `🌱🧊️create-solid` | `appends-an-extruded-roof-slab` | `solids.added` |
| `🗑🧊️delete-solid` | `removes-the-roof-slab-and-keeps-its-material` | `solids.removed` |
| `🔁replace-solid` | `thickens-the-slab-and-adds-a-mesh-layer` | `solids.patched` |
| `🌱📋️create-load-case` | `appends-a-wind-case-pushing-on-the-column-head` | `loadCases.added` |
| `🗑📋️delete-load-case` | `removes-the-wind-case-together-with-its-load` | `loadCases.removed` |
| `➕add-load` | `lays-an-area-pressure-over-the-roof-slab` | `loadCases.patched` |
| `➖remove-load` | `drops-the-trailing-member-udl-from-the-dead-case` | `loadCases.patched` |
| `⚖change-load-case-self-weight` | `switches-self-weight-off-for-the-dead-case` | `loadCases.patched` |
| `🌱🔗️create-combination` | `appends-a-serviceability-combination-keyed-by-case-id` | `combinations.added` |
| `🗑🔗️delete-combination` | `removes-the-serviceability-combination-and-keeps-both-cases` | `combinations.removed` |
| `🎛update-analysis-settings` | `doubles-the-buckling-mode-count` | scalar `analysis` |

## How the diffs were derived

Each `🔺️diff/🔣️component.json` was transcribed from that leaf's own `🔺️diff/🦀️component.rs`, never
from the leaf name:

- `Fem2dDiff` carries **17** fields, `Fem3dDiff` **16**, both `#[serde(rename_all = "camelCase",
  default)]` with **no** `skip_serializing_if` — so every field is emitted, `null` for the untouched
  ones. All 50 committed diffs carry the full field list in struct-declaration order and set exactly
  **one** field.
- fem's patch entry is `{ "id", "item" }` (a whole-entity replacement), **not** puzzle5d's
  `{ "id", "patch": { "replacement" } }`.
- Each collection delta is `{ added, removed, patched, reordered }` — also `default`, also
  no-skip, so `reordered: null` is always present.
- Loads have no collection of their own: `add-load` / `remove-load` /
  `change-load-case-self-weight` all re-emit the **whole owning load case** as one
  `loadCases.patched` entry. The fixtures assert exactly that, so a future nested-load delta would
  break them loudly.
- `update-analysis-settings` is the only mutation in either tree whose payload is a scalar facet
  (`analysis`) rather than a collection delta.

## Verification performed (no `cargo`, per the ticket's standing constraint)

The workspace is broken by a peer's in-flight de-async sweep, so **no test was run and none is
claimed to pass**. Structural verification instead:

1. **`fixtures lint --by-tree`** — both fem trees absent from the uncovered list and from every
   error line (`0/25` each); only derived-encoding warnings remain.
2. **`include_str!` resolution** — all **250** `include_str!` targets across the 50 test files exist
   on disk (5 per file).
3. **`#[path]` resolution** — all **341** `#[path]` targets in `📦️glue.rs` resolve, including the
   50 newly added `#[cfg(test)] mod tests_*;` lines.
4. **`rustfmt --edition 2021 --emit stdout`** parses all 50 test files and `📦️glue.rs`.
5. **Diff replay simulation** — a Python port of `🔺️diff/📝️text/🦀️component.rs`'s `apply_delta`
   (remove → add-append → patch-in-place, with the same duplicate/missing guards) reproduces
   `➡️after` from `⬅️before` for all 50 committed diffs, and confirms exactly one diff field is
   non-null per case and that the JSON key order matches the Rust struct field order.
6. **Inverse replay simulation** — a Python port of each leaf's `↩️inverse/🦀️component.rs` rule
   lands back on `⬅️before` for all 50. This is what forced every deletion case to target the
   **last** row of its collection (and `remove-load` the **last** load of its case): the inverse
   `create-*` / `add-load` appends at the tail, so a mid-list deletion would not round-trip.
7. **Float canonicality** — an exact port of `ryu-1.0.23`'s `pretty::format64` branch selection
   (decimal notation iff `0 <= k && kk <= 16`, or `0 < kk <= 16`, or `-5 < kk <= 0`) confirms all
   **1211** committed float literals (49 distinct) are byte-for-byte what `serde_json` will
   re-emit — no literal is outside the decimal window, so nothing flips to `e`-notation and the
   `committed_json_is_canonical` assertion holds. Values were chosen to stay inside
   `[0.00001, 1e16)`: section inertias like `0.00008356`, moduli like `210000000000.0`, and dyadic
   deltas like `0.25` / `0.375` / `1.5`.

## Style notes

- Written in the **de-async** target style (no `.await`), matching the committed puzzle5d reference
  — `apply_puzzle5d_mutation` is itself still `async fn` today and the reference test calls it bare.
- Each test carries the seven required laws plus 2–3 extra assertions naming that mutation's own
  entities and invariants (e.g. `delete-node` asserting the *absence* of a support cascade,
  `replace-material` asserting the patched row keeps its slot, `create-support` in 3D asserting the
  DOF list round-trips as `FemDof::ALL`). No shared harness, no macro, no loop.

## Files

- 50 × `<leaf>/🧪️tests/<case>/` (6 files each = 300 new files) under both fem mutation roots.
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/📦️glue.rs` — 150 inserted lines (50 × 3), each a
  `#[cfg(test)] #[path = …] mod tests_<case>;` placed immediately after that leaf's
  `pub mod inverse;` at the same indentation.
