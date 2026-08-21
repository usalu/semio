# 🧊️ Handcrafted mutation fixtures — `🗄️stdio` / `🧊️gltf`

Tree: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Contract: D1/D6, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`.

## 📊️ Result

| metric | value |
| --- | --- |
| mutation leaves | 120 |
| leaves with a committed case | **120** |
| applied cases (carry `🔺️diff/🔣️component.json`) | 116 |
| rejected cases (carry `🔺️diff/🚫️component.absent`) | 4 |
| committed files written | 720 |
| `include_str!` targets verified present | 596 |
| test modules mounted in the mutations root | 7 (see ⚠️ below) |

`bun ./📜️script.ts fixtures lint --by-tree` (from `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust`):

```
🧬️ 115 artifact mutation trees · 1558 mutations · 1558 covered · 0 uncovered
⚠️ 12322 derived-encoding gap(s) pending `fixtures generate`
✅️ fixture contract satisfied
```

Scoped re-run of the same rules over this tree alone (`🛠️lint-scope-gltf.ts` in this ticket folder —
the CLI truncates its error list at 40 rows repo-wide, so the scoped count is the load-bearing one):

```
🧬️ stdio/gltf · 120 mutations · 120 covered · 0 uncovered
✅️ 0 error(s) · ⚠️ 952 derived-encoding warning(s)
```

Structural verification (no `cargo` — the workspace is mid-refactor, see ⚠️ below):

- `rustfmt --edition 2021 --emit stdout` parses all **120** test files **and** the mutations root.
- Every `#[path]` in the mutations root resolves to an existing file (**0** unresolved).
- Every `include_str!` target exists (**596** checked, **0** missing).
- Every committed `.json` parses.
- **No test is claimed to pass** — none has been executed.

## 🧬️ How the cases were derived

Each case was transcribed by hand from that leaf's own `🦠️mutation/🦀️component.rs` (payload struct,
`validate()` guards, `apply()`), its `🔺️diff/🦀️component.rs` (the oracle for the committed diff) and
its `↩️inverse/🦀️component.rs`. `stdio.gltf` has **seven** distinct leaf contracts, and the fixtures
follow whichever one the leaf actually implements:

| contract | leaves | diff shape |
| --- | --- | --- |
| top-level collection | 50 | leaf-owned `{id, version, touchedPaths, payload, operation}` with a `kind`-tagged `insert`/`delete`/`move`/`reorder` |
| artifact `GltfDiff` | 45 | the sparse `GltfDiff`, narrowed to the single slot the leaf owns |
| leaf relation witness | 17 | `{operation, after, touchedPaths}` (wildcard paths) or `{…coords, before, after, touchedPaths}` (concrete paths) |
| descriptor-backed scene | 2 | `create-scene` / `delete-scene`: full pre-state witness + `phase` |
| descriptor-backed material | 2 | `change-material-*`: `expected*` pre-state witness |
| no diff module at all | 4 | `🔺️diff/🚫️component.absent` |

Per-leaf specifics that the assertions pin (a sample, not the whole list): `repair(<family>, …)`
cascades (`create-node` renumbering scene roots, `delete-node` DROPPING them, `delete-texture` taking
the renumber branch rather than the clear-to-`None` branch); the two-lane buffer leaves
(`create/delete/move/reorder-buffer(s)`) declaring both `document/buffers/*` and `buffers/*`;
`reorder-*` touched paths following the ORDER, not the index; `reorder-meshs` emitting the spec
spelling `document/meshes` while keeping the leaf spelling in its descriptor id; `move-scene` sliding
`document/scene` while `reorder-scenes` remaps it through `order.position(..)`; the mesh family's
whole-primitive replacement being the only granularity `GltfMeshDiff` offers; `reparent-node` being
the one leaf that writes two collections; `require-extension` demanding prior USE and
`withdraw-used-extension` demanding prior UN-REQUIRE.

Floats are dyadic throughout (`0.5`, `0.25`, `2.0`) and `GltfJson` numbers carry an explicit `.0`,
because `GltfJson::Number` widens to `f64` and an integer literal would not survive the canonical
round trip.

### 🎯️ The four rejected cases

`move-node-child`, `move-scene-root-node`, `reorder-node-children` and `reorder-scene-root-nodes` ship
**no `🔺️diff` and no `↩️inverse` module at all** — `🦠️mutation/🦀️component.rs` is their entire Rust
implementation. There is therefore no diff type to serialize, and an invented empty patch is
explicitly forbidden by the contract. Each one instead pins a real guard branch, and each pins a
DIFFERENT rejection code so no two of the four would pass each other's fixture:

| leaf | payload | code |
| --- | --- | --- |
| `move-node-child` | destination equals the child's current slot | `gltf.mutation.no-observable-change` |
| `move-scene-root-node` | node exists but is not a root of that scene | `gltf.mutation.relation-absent` |
| `reorder-node-children` | order repeats a child at the right length | `gltf.mutation.invalid-permutation` |
| `reorder-scene-root-nodes` | scene index past the end (checked BEFORE the permutation) | `gltf.mutation.index-out-of-range` |

### 🧨️ `Option<Option<_>>` round-trip limitation — `unbind-default-scene`

`GltfDiff::scene` is `Option<Option<usize>>` and `unbind-default-scene` writes `Some(None)`, which
serde encodes as a bare `null` — indistinguishable from the field being absent, so decoding the
committed diff yields `None`, not `Some(None)`. The fixed-point assertion every other leaf uses would
be false here. That case therefore pins the limitation explicitly (the committed file still carries
the `null` the typed diff produces, the re-encode is asserted **unequal**, and the TYPED delta is
separately proved to carry `before` → `after`). Every other fixture chooses a non-null value for its
`Option<Option<_>>` slots so the ordinary fixed point holds exactly.

## ⚠️ Reachability: only 7 of 120 leaves are in the crate's module tree

`📦️glue.rs` mounts exactly seven gltf mutation leaves as production modules: `create-scene`,
`change-material-alpha-mode`, `change-material-double-sided`, `bind-node-child`, `unbind-node-child`,
`bind-scene-root-node`, `unbind-scene-root-node`. The other **113** leaves exist as source on disk but
are not compiled at all, and several of them cannot compile as written:

- they import `mutations::top_level_collections_private`, but glue mounts that file under the name
  `top_level_private` (and the separate `🔒️top-level-private/` directory is unmounted);
- `bind-primitive-indices` names `GltfComponentType::F32`, a variant that does not exist (`Float`);
- `bind-node-camera` / `change-scene-name` / `change-node-morph-weights` inverses reference an
  undefined local `diff` inside `apply`;
- `create-camera`'s payload embeds `GltfCameraProjection`, which derives no serde impls at all.

Consequences, and what was done about them:

1. **All 120 test files are written against each leaf's own real entry points** (`mutation::apply`,
   `diff::derive`/`apply`/`apply_diff`, `inverse::derive`/`apply`/`apply_inverse`/`reconstruct`), so
   each one compiles the moment its leaf is wired.
2. **Only the seven reachable leaves are mounted** in the `#[cfg(test)] #[path = "."] mod
   fixture_tests` block appended to this tree's own `🧬️mutations/🦀️component.rs` (appended strictly
   additively; `📦️glue.rs` untouched). Mounting the other 113 would break `cargo test` for the whole
   `semio-s-plugin-stdio` crate. The block carries a comment telling whoever wires a leaf to add the
   matching `mod tests_…;` line.
3. `create-camera`'s payload JSON is committed in serde's default externally-tagged shape
   (`{"Perspective": {…}}`) — the only defensible guess while `GltfCameraProjection` has no derive.
   Its docstring says so.

Nothing above was authored or "fixed" by this lane; it is reported as found. `cargo` was not run
(forbidden by the brief, and the workspace is mid-de-async-sweep: `🧬️schema/🔺️diff/🦀️component.rs`
still declares `ItemDiff` methods `async` while calling them without `.await`, and
`🔨️modules/🧭️mutation-dispatch` has `.await` on non-futures).

## 📁️ Authoring tooling (kept in this ticket folder)

| file | role |
| --- | --- |
| `🛠️emit-gltf-fixture.ts` | renders one case's six files from a hand-authored spec — an authoring aid, NOT a test harness: every value it writes came from a spec entry transcribed by hand from that leaf's oracle |
| `🛠️gltf-specs-collections.ts` | `create-*` + `create-scene` |
| `🛠️gltf-specs-deletes.ts` | `delete-*` + `delete-scene` |
| `🛠️gltf-specs-moves.ts` | `move-*` (top level) |
| `🛠️gltf-specs-reorders.ts` | `reorder-*` (top level) |
| `🛠️gltf-specs-meshes-a.ts` | mesh/primitive `bind-*` and `change-*` |
| `🛠️gltf-specs-meshes-b.ts` | mesh/primitive `create/delete/move/reorder/unbind-*` |
| `🛠️gltf-specs-nodes.ts` | node/scene relation leaves |
| `🛠️gltf-specs-document.ts` | default scene, transform/reparent, asset, document extras, extension lists, materials, the four refusals |
| `🛠️run-gltf-fixtures.ts` | re-emits every table |
| `🛠️lint-scope-gltf.ts` | `fixtures lint`'s own rules, scoped to this tree |
