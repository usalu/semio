# 📓️ exec-stdio-gltf — 🧊️gltf diff-only mutations

Status: WRITTEN BUT UNVERIFIED by cargo. Four `cargo check -p semio-s-artifact-stdio-gltf --target wasm32-wasip2` attempts (gate-wrapped, once with CARGO_BUILD_JOBS=1; logs `T/🗑️generated/stdio-gltf/check1..4.txt`) were SIGKILLed (exit 137, swap ~4.7/6 GB, disk 99%) while checking `semio-framework-value`, a framework crate, before the gltf crate was reached. Design verified by a Python simulation instead (see Verification).
Scratch: `T/🗑️generated/stdio-gltf/` (generators `gen*.py`/`specs_*.py`/`facets.py`, `backup/` = pre-change leaf sources, `sim*.py`).
The crate is now its own cargo workspace (`🧊️gltf/Cargo.toml`, created by someone else): run cargo from `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf`.

## What changed (all 120 regular kinds; classification before → after)

| Aspect | Before | After |
|---|---|---|
| diff (120) | V1-SNAPSHOT-DIFF: leaf `apply` clone-writer + `GltfDiff::between` | `plan(payload, base) -> Result<GltfDiff, Rejection>`: sparse rows built from payload + base reads (`patch`, `with_inserted/without/with_moved/with_replaced/permuted`, `rewire`) |
| inverse (119 + rename) | V2-DIFF-DERIVED/RESTORE: `Restore(Box<GltfDiff>)` / `Restore{before,after}` | `inverse(payload, base) -> Vec<GltfMutation>` of concrete kinds (rows in storage order; replay is reversed, as the law helper does) |
| `Restore` variants | 120 | deleted; leaf enum is `Apply(payload)` only (phase wire kept: `{phase:"apply",value}`) |
| leaf `apply` | 118 clone-writers | gone; central `protocol::apply_diff` only (aggregate `apply_gltf_mutation(&base,&m)`) |
| `MutationOutcome::apply_to` | bridge | gone |
| `GltfDiff::apply` | no capability | takes `ApplyCapability` |

Shared pure helpers (no snapshot writes): `mutation-support/🗂️top-level-collections` now holds `rewire` (read-only typed-reference scan answering sparse referrer rows
for insert/delete/move/reorder), `require_unreferenced`, index remaps, list-value builders, record reference checks (`check_*`); the old `repair`/`family_ops`/
`family_diff` seams are deleted. `create-scene` helper mutators deleted.

### Semantic rulings taken (greenfield, flagged)
- **Collection ops** are expressed as `removed`+`added` rows (move/reorder re-add the moved entries at final index; node children inside moved nodes are remapped in the
  added item) plus renumbered-referrer `modified` rows — `GltfDiff::inverse`/`absorb` already handled this.
- **delete-X** keeps its cascade only where a bind kind can restore it (node mesh/camera/skin, primitive material/indices/attributes/targets, scene roots, node children,
  default scene); deleting an entry still referenced from a site no kind can rebind (skin joints/skeleton/ibm, animation channel node, bufferView→accessor/image,
  texture→material, image/sampler→texture, buffer→view) is now rejected `gltf.reference.in-use`. Exactly invertible; the independent subset fixtures only delete
  unreferenced or restorable entries.
- **create-X** gained an optional full record (`scene,node,mesh,material,texture,image,sampler,skin,animation,primitive,target,buffer,accessor,bufferView,camera`; for accessor/bufferView/camera it must agree with the still-required minimal fields, `gltf.mutation.record-mismatch`). `delete-X`'s inverse is `create-X(position, record)` + re-binds. Record references are validated (`check_*`).
- `change-primitive-topology-mode.mode` is nullable (`null` clears) so its inverse is exact; clearing morph weights is always admitted (inverse of setting them);
  `create-morph-target` also requires empty mesh weights (symmetric with delete); `change-node-transform` rejects a node holding matrix and TRS.
- `x-semio-inverse-rows`: `bounded 1025` for delete-{scene,node,mesh,accessor,material,skin,camera} and move-node-parent; `fixed 2` for unbind-primitive/morph-attribute.

## Files touched
- 120 leaf `🦀️.rs` (generated from `backup/` by `gen.py`), leaf `🧬️schema/🔣️.json` + `🟦️.ts` + `🔗️.graphql`/`🛰️.proto` (Restore removed; record/mode/inverse-rows added) and the 120 `io/📝️text/🧬️mutations/**/🟦️.ts` twins; both diff `🟦️.ts` (`GltfApplyPhase`/`gltfWireApplyPhase` replace `GltfPhase`/`gltfWirePhase`).
- `🧬️schema/🔺️diff/🦀️.rs` (`apply(.., capability)`), diff unit tests (`protocol::apply_diff`).
- `🧬️mutations/🦀️.rs` (`apply_gltf_mutation`, `gltf_bridge_apply`), `io/📝️text/🧬️mutations/🦀️.rs` + `io/💾️binary/🧬️mutations/🦀️.rs` (Restore facade branches removed; bridge replays inverse reversed), change-node-name test case, carriers fixture (`restore`→`inverse` concrete kind), contract tests (10) and `contract_tests::assert_laws`.
- Corpus helper `🧪️fixture-corpus/🦀️.rs`: every applied case (118+set-snapshot) runs `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law` (via `semio_framework_async::block_on`).
- 8 subset adapters (`subsets/*/🧪️tests/*/🦀️.rs`): `X::apply(..)` → `apply_gltf_mutation(before, &X::mutation(..))`.

## Tests
NOT RUN (build killed, see above). Pending commands: `cargo check -p semio-s-artifact-stdio-gltf --target wasm32-wasip2 --message-format=short` then
`cargo test -p semio-s-artifact-stdio-gltf fixture_corpus` (and `mutations::`), from the gltf workspace dir.
Verification done instead: `T/🗑️generated/stdio-gltf/sim.py`/`sim2.py` — a Python port of `GltfCollectionDiff::apply/absorb/inverse`, `rewire`, the
create/delete/move/reorder rows and delete re-bind chains, 20 000 random documents: rows equal the OLD clone-writer semantics, inverse replay restores the base, and
`canon(Σ inverse diffs) == canon(d.inverse(base))` (12 984 cases, nodes/meshes/accessors/materials/scenes/cameras). Syntax of all 120 generated leaves checked with rustfmt.

## Open issues / not done
1. **Committed `🔺️diff/🔣️.json` fixtures not regenerated**: create/delete/move/reorder cases (and delete cases now rejected/changed by the in-use rule) change shape; bless from the first test run (the corpus failure message prints the produced diff) and review against before/after.
2. **`set-snapshot` / `patch-snapshot` leaves still present** (`GltfDiff::between`, whole-snapshot inverse; editor `snapshot_edit_patch`). Removal touches the aggregate roster, definition constraint, text grammars, oracles, editor; left because the shared `snapshot_patch_leaf!` macro is owned by stdio-small. They remain R10/R14 breaches.
3. `GltfDiff` weak entities (texture/image/bufferView/skin/animation) and `mesh.primitives` stay whole-value fields (referrer renumbering emits the whole entity row); finer per-field diffs need new diff structs plus text/binary diff codecs and grammars.
4. Phase wrapper (`{phase:"apply",value}`) kept: the io/text BDD bridge and fixtures build it; collapsing needs a wire migration.
5. BDD/`.feature`/oracle prose still says `apply()`/`restore`; subset adapters not compiled here.
6. R9 may flag `protocol::apply_diff` in the aggregate file (`🧬️mutations/🦀️.rs`) because it lives under a `🧬️mutations` dir.

## Wave 3

Status: WRITTEN BUT UNVERIFIED by cargo. Six gate-wrapped `cargo check -p semio-s-artifact-stdio-gltf --target wasm32-wasip2` attempts (12:09 to 12:50) ended as follows: one failed in `semio-framework-value` (`🗂️ordered/🚦️native` E0308, `RetainedCloneGrant` vs `Grant`, a peer's in-flight edit, not mine); the others were SIGKILLed (exit 137) while compiling framework dependencies (`wit-parser`, `semio-framework-pack`) before reaching the gltf crate. The gltf crate itself has therefore not been type-checked since wave 1. Static gates that did pass: `rustfmt --edition 2021 --check` syntax gate on all 531 `🦀️.rs` files in the gltf workspace, and every fixture `🔣️.json` parses.

1. **set-snapshot / patch-snapshot removed.** Leaf crates `📸️snapshot` (schema, text-io, fixtures), the `SetSnapshot`/`PatchSnapshot` aggregate variants, `diff_set_snapshot`, root module mounts, oracle vectors, BDD rows and all facet entries (json schema, ts, graphql, proto, ebnf, g4) are deleted. Callers: the stdio-gltf editor planner `snapshot_edit_mutations` now emits concrete kinds via `concrete_snapshot_edit(base, expected)` (field-level setters, binds, reorders for asset/document, default scene, scene, node, mesh, primitive scalar fields, material alpha/double-sided). Structural snapshot edits (add/remove entities) fault with `snapshot-edit.unsupported`; whole-document load stays the genesis/load path. The 8 subset adapters call `apply_gltf_mutation(before, &X::mutation(..))`.
2. **Keyed sparse rows.** `mesh.primitives` is now `GltfCollectionDiff<GltfPrimitive, GltfPrimitiveDiff>` (nested in `GltfMeshDiff`), and texture, image, bufferView, skin, animation collections are `GltfCollectionDiff<_, GltfXDiff>` with new per-entity diff structs (`ItemDiff` with between/apply/inverse/absorb_into). `rewire` (referrer fix-ups on delete/move) emits sparse field rows instead of whole entities. Text and binary diff codecs, `🔺️diff` json/ts/graphql/proto facets and the diff grammar were extended. Still whole-value inside a sparse entity row: skin joints, animation channels/samplers, primitive attributes/targets lists (the field is the unit; the entity is not).
3. **Position-exact inverses + middle-row law cases.** `create-X` carries an optional record whose `position` is the original index, so delete inverses restore the original index; `design.md` "Rulings (wave 2)" already holds the ruling text. New committed case `🔬️middle-row` (3-entry document, delete/unbind the MIDDLE row, unreferenced) for 21 kinds: delete-{scene,node,mesh,accessor,buffer-view,buffer,material,texture,image,sampler,skin,animation,camera,primitive,morph-target}, unbind-{scene-root-node,node-child,primitive-attribute,morph-target-attribute}, remove-{required,used}-extension. Each has `🦠️mutation`, `⬅️before`, `➡️after`, `🔺️diff`, `🎯️outcome` and a mounted leaf test running `assert_case`. Expected diffs are simple `{"<field>":{"removed":[1]}}` (primitive/morph/attribute cases nested under `meshes.modified`; buffer adds `bufferBytes.removed`; list-valued kinds carry the new whole list).
4. **Check and bless.** Not done (see status). Existing `🔺️diff` fixtures for delete/move/reorder cases with referrers (`deleteBuffer`, `deleteSampler`, `deleteCamera`, `deleteSkin`, mesh primitive and weak-entity cases) will change shape under keyed sparse rows; on the first green `cargo test -p semio-s-artifact-stdio-gltf fixture_corpus` they must be re-blessed from the failure output after hand-verifying a sample against before/after. The new `🔬️middle-row` diffs were hand-derived, not produced by the code.

Open issues:
- stdio-small's `validate_snapshot_edit_publication` (`📇️registry/🧬️contract/✏️editing/🦀️.rs`) replays inverse rows in listed order, inconsistent with the framework law helper (reverse) for multi-row inverses; gltf multi-row inverses are stored so that reversed replay is correct.
- `📦️packages/🟦️typescript/dist` is stale; oracle/feature prose and `📜️contract/🔣️.json` law text still mention `apply()`/restore in places.
- Framework build instability (peer edits in `🌱️value/🗂️ordered`, memory kills under fleet load) blocks every verification step above.
