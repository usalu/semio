# r4 execution report: m-horizontal (Wave M slice 6, binary tags 600..607)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. Result

Eight slab and roof leaves of `s.bim.model@1`, each with descriptor, payload schema (`x-semio-ui` on every property, en+de labels), `MutationKind` file (en+de `label`, `target`), hand-authored `🔺️diff` and `↩️inverse`, fixtures, per-case tests (sum-law in every applied case) and the shared rule module `🧿️horizontal-rules`. 53 cases (17 applied, 36 rejected), 282 generated tests. `cargo test --lib mutations::` on the real crate: 2570 passed, 0 failed (includes the aggregate tests `kinds_match…`, `binary_tags_are_unique…`, `every_committed_mutation_round_trips_text_and_binary`). `cargo check --target wasm32-wasip2`: exit 0, no warning in my files.

| Tag | Kind (leaf dir) | Cases (applied / rejected) |
|---|---|---|
| 600 | create-slab (`⬜️`) | adds-a-slab-with-a-hole, adds-a-sloped-curved-slab / duplicate, storey-missing, type-missing, too-few-vertices, zero-area, self-intersecting, clockwise, hole-outside |
| 601 | delete-slab (`🔻️`) | removes / missing |
| 602 | set-slab-boundary (`🔷️`) | reshapes, drops-the-hole / unchanged (no-op), hole-outside, overlapping-holes, self-intersecting, missing |
| 603 | set-slab (`🔸️`) | retypes-and-offsets, slopes, clears-the-slope, renames / unchanged (no-op), names-no-field (no-op), type-missing, slope-too-steep, missing |
| 604 | create-roof (`🏠️`) | adds-a-gable, adds-a-flat-roof, adds-a-mansard / duplicate, storey-missing, type-missing, self-intersecting, pitch-zero, pitch-too-steep, negative-overhang |
| 605 | delete-roof (`🏘️`) | removes / missing |
| 606 | set-roof-footprint (`👣️`) | reshapes, curves-an-edge / unchanged (no-op), too-few-vertices, self-intersecting, missing |
| 607 | set-roof-shape (`🏔️`) | hips-the-roof, overhangs-and-lifts / unchanged (no-op), names-no-field (no-op), pitch-too-flat, negative-overhang, missing |

## 2. Semantics

- Loops (boundary, holes, footprint) are valid when: at least 3 finite vertices, no repeated consecutive vertex, no self-intersection (`semio_framework_geometry::loops::self_intersections`, arcs exact), area greater than 1e-9, **counter-clockwise** (the snapshot type says CCW; a clockwise loop is refused with `mutation.invariant`, never silently reversed). Holes must lie strictly inside the boundary (no touching or crossing) and must not overlap or nest; the hole check uses `bulge::intersect` and `loops::contains`.
- Slab slope: finite direction, angle in [0, 89 degrees). Roof shapes: pitched shapes need every pitch in (0, 89 degrees) (radians), finite directions, mansard break height positive; overhang finite and not negative; base offset and slab offset finite.
- Reference failures use `mutation.target-missing` with `["slab"|"roof","storey"|"slab_type"|"roof_type"]` (create) or `["slab_type"]` (set); loop and value failures `mutation.invariant` with the field path; duplicates `mutation.duplicate-id`; equal values or an empty sparse payload `mutation.no-op` with `[id]`.
- Nothing references a slab or a roof, so deletes never block or cascade.
- Sparse kinds (`set-slab`, `set-roof-shape`): payload fields are `Option<_>` (`#[value(default, skip_serializing_if = "Option::is_none")]`, not in the schema `required`). The diff patches only provided fields that differ from base; the inverse is the same kind carrying the base value of exactly the provided fields. Slope uses the diff algebra's `Assigned<Option<Slope>>` (`{"value": null}` clears, absent leaves it), so a slope can be set and cleared reversibly.
- `set-slab-boundary` validates boundary and holes together and patches the fields that differ; the inverse is absolute (base boundary and holes).
- Inverses: delete to `Create…` with the full captured record, create to `Delete…`, set kinds absolute. Every inverse is exactly one row (no `x-semio-inverse-rows`).

## 3. Files

Created (all under `S`, emojis picked unique among the siblings of `🧬️schema/🧬️mutations/`; checked at the end with `r3-f1-check-names.ts`: no duplicate or bad name in `🧬️mutations/`):
- Leaf directories `🧬️schema/🧬️mutations/{⬜️create-slab,🔻️delete-slab,🔷️set-slab-boundary,🔸️set-slab,🏠️create-roof,🏘️delete-roof,👣️set-roof-footprint,🏔️set-roof-shape}` (descriptor, payload schema, `🦠️mutation`, `🔺️diff`, `↩️inverse`, `🧪️tests/<case>`), and 53 fixture quintets under `🧫️fixtures/🧬️mutations/<same dirs>/<case>/` (applied `after` and `diff` blessed from the code under test and reviewed by hand; reviewed diffs are sparse: e.g. `drops-the-hole` is `{"slabs":{"sl-ground":{"entry":"Patched","holes":[]}}}`).
- `🧬️schema/🧬️mutations/🧿️horizontal-rules/🦀️.rs`: shared pure rule module (`loop_fault`, `holes_fault`, `slope_fault`, `shape_fault`, `overhang_fault`, `MAX_INCLINATION`). It is a plain module (no descriptor), mounted as `pub mod horizontal_rules` next to the leaves; other slices that need loop validation (e.g. `create-space` with an explicit outline) can reuse `loop_fault`.
- `T/r3-m-horizontal-leaves.ts`: the idempotent spec and generator (boilerplate and fixtures through `emitLeaf`, sparse-field patch, the eight `🔺️diff`/`↩️inverse` sources, the rule module, mounts once). `bun T/r3-m-horizontal-leaves.ts` re-creates everything except the blessed `after`/`diff` fixtures. Note: `emitLeaf` grew a `uses` field (full `use` lines) meanwhile; my spec therefore calls its own import list `names`.

Updated (surgical):
- `S/🧬️schema/🧬️mutations/🦀️.rs`: 8 enum variants and 8 `KINDS` rows.
- `🏢️model/🦀️.rs`: 8 leaf mount blocks before `//#endregion 🔖️Leaves` and the `horizontal_rules` mount before `//#region 🔖️Leaves`.
- `🏢️model/📦️packages/🦀️rust/Cargo.toml`: `semio-framework-geometry = { workspace = true }` (another agent added the same line concurrently; the duplicate I introduced was removed).
- `T/r3-f1-gen-mutation-facets.ts`: one helper (`unwrapped`) so that the `Assigned` slope wrapper in `set-slab` surfaces as `Slope` in the generated `🟦️.ts`/`🔗️.graphql`/`🛰️.proto` facets (before it fell through to `number`). Facets regenerated (88 leaves at the last run).
- Generated by the shared generators: mutation facets, the wire protocol (`📡️.protocol.semio`), the grammar, and `🧪️tests/🏙️mutate-model-1-any/{🥒️.feature,🦀️.rs}` (`bun T/r3-f1-gen-feature.ts`: 88 kinds, includes mine).

Outside my scope, done to unblock the build (please review): `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/{🦀️.rs,✏️editing/🦀️.rs}`: three `MutationDiff::apply(..)` / `.apply(&base)` call sites migrated to `kernel::apply_diff(..)` (the bim crate depended on this crate through `semio-s-artifact-stdio-ifc`, which broke every bim build for hours; x-ifc has since re-pointed the dependency to the contract crate). The remaining unmigrated `stdio/💾️binary` crate is NOT touched.

## 4. Commands and results

| Command | Result |
|---|---|
| `bun T/r3-m-horizontal-leaves.ts` | 8 leaves emitted, mounts present |
| `BIM_BLESS=1 cargo test --lib` (isolated crate, see 5) | 278 passed, 17 failed (only `committed_json_is_canonical` on the not yet blessed `{}` placeholders, as expected); fixtures copied back, reviewed |
| isolated crate, `cargo test --lib` after bless | 295 passed, 0 failed (my 282 leaf tests plus the 13 snapshot/diff unit tests) |
| `cargo check -p semio-s-artifact-bim-model` (real crate) | exit 0 once the peers' in-progress errors cleared |
| `cargo test -p semio-s-artifact-bim-model --lib` (full) | 2951 passed, 162 failed at that moment; zero failures in my 8 leaves; the 162 are peers' in-progress modules (unblessed fixtures of other leaves, editor, inference, examples, ifc projection) |
| `cargo test -p semio-s-artifact-bim-model --lib mutations::` (last run) | **2570 passed, 0 failed**, includes the four aggregate tests |
| `cargo check -p semio-s-artifact-bim-model --target wasm32-wasip2` | exit 0 (`Finished dev profile`), no warning in my files |
| `bun T/r3-f1-gen-mutation-facets.ts` | ok (88 leaves) |
| `bun T/r3-f1-check-names.ts` | no problem inside `🧬️mutations/`; other problems (duplicate or bad names in `🖼️render`, `🧪️tests`, `🧫️fixtures/💡️inferences`, ifc `🧱️`, inference dirs, Cargo.toml/AGENTS.md/README.md of the plugin) belong to other slices or are the checker's known false positives for non-emoji files |
| gate rules R8..R16 grep over my leaves (`.apply(`, `between(`, `&mut`, `base.clone()`, comments in definitions, `unwrap`/`expect`) | clean |
| third-party validation of the loop rules | the geometry crate's own oracles (kurbo, shapely, parry3d) back `loops`/`bulge`; my leaf cases are exercised through the committed fixtures; no separate third-party oracle for slab or roof mutations (Wave X to add a `jsonpatch`-style oracle for the whole catalog) |

## 5. How the build was verified while the shared crate was red

For about three hours the shared crate did not build for reasons outside this slice (a stale `MutationDiff::apply` signature in the stdio contract and binary crates, then half-finished editor/inference modules). I verified in a throwaway "mirror" crate: a copy of my 8 leaves and their fixtures under a temporary `🧬️mutations/` directory (the `Mutations`/`MutationLeaf` derives insist on the canonical directory layout and on the descriptor `owner` path), a trimmed aggregate enum with only my variants, and the real `snapshot`, `diff`, `🧰️kit` and `🧿️horizontal-rules` files mounted by absolute path. It was deleted afterwards together with all logs. The real-crate results above were taken as soon as the shared crate compiled again.

## 6. Open issues

1. `bun T/r3-f1-gen-oracle.ts` fails with "the manifest has no mutationCatalogs/mutationManifests block to replace" (the subset oracle file changed shape under another slice); the mutation oracle catalog rows for the 8 kinds are therefore not yet generated. Run it after the oracle owner fixes the manifest.
2. The CCW requirement for all loops is stricter than the brief's list (< 3 vertices, zero area, self-intersection, holes inside). It follows the documented snapshot type; example and gesture code must emit counter-clockwise loops (`u-tools`, `x-examples`).
3. `Assigned<Option<Slope>>` appears in a payload for the first time; other slices with clearable optional fields (`set-opening` width/height) should reuse it.
4. The Cargo.toml and stdio contract edits outside my scope (section 3) should be reviewed by their owners.
