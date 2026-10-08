# r4 execution report: m-railings-spaces (Wave M, slice 8, binary tags 800..805)

`T` = ticket folder `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`. `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. Recipe followed: `T/r3-golden-leaf.md`.

## 1. Result

Six leaves, 31 fixture cases (11 applied, 20 rejected including 2 no-op), all green: `cargo test -p semio-s-artifact-bim-model --lib` ran 3120 tests, 3104 passed, 16 failed. All 16 failures are `editor::bim::*` / `viewer::bim::*` tests of the u-editor / u-viewer work in progress (tool catalog authority, set-field codes, rename mutation), none touches these leaves; all 211 railing/space tests pass. `cargo check --target wasm32-wasip2` exit 0.

| Kind | Tag | Leaf emoji | Cases (applied / rejected) |
|---|---|---|---|
| create-railing | 800 | 🛤️ | adds / duplicate, storey-missing, material-missing, path-too-short, non-positive-height, non-positive-post-spacing |
| delete-railing | 801 | 🪚️ | removes / missing |
| set-railing | 802 | 🔧️ | reshapes, renames-only / nothing-to-change (no-op), missing, path-too-short, non-positive-height, non-positive-post-spacing, material-missing |
| create-space | 803 | 🛋️ | adds-a-bounded-space, adds-an-explicit-space, reuses-a-number-on-another-storey / duplicate, storey-missing, number-taken, outline-degenerate |
| delete-space | 804 | 🧽️ | removes / missing |
| set-space | 805 | 🪑️ | renames-and-retypes, redraws-the-boundary, renumbers / nothing-to-change (no-op), missing, number-taken, outline-degenerate |

## 2. Semantics

- create-railing refuses, in order: duplicate id (`mutation.duplicate-id [id]`), missing storey / material (`mutation.target-missing [railing, storey|material]`), path with fewer than two points, non-finite coordinates or all points equal, non-positive or non-finite height, non-positive post spacing, non-finite base offset (`mutation.invariant [railing, path|height|post_spacing|base_offset]`).
- set-railing is sparse (`path, height, post_spacing, material, base_offset, name` as `Option`, absent = untouched, skipped when `None` on the wire). Same validations with paths `[path]`, `[height]` ... ; `[material]` target-missing; providing nothing or only equal values is `mutation.no-op [id]`. Diff is a `RailingPatch` of exactly the provided fields. Inverse: `SetRailing` with the base values of exactly the provided fields.
- create-space refuses duplicate id, missing storey (`[space, storey]`), number already used within the same storey (`mutation.invariant [space, number]`), undrawable boundary (`[space, boundary]`: Bounded needs a finite seed, Explicit needs at least three finite vertices and a non-zero area or a bulge). A bounded space stores only its seed; outline, area and volume stay inferred.
- set-space is sparse (`number, name, boundary, usage`); a new number must stay unique within the space's storey (the space itself excluded), boundary validated as above, no-op when nothing changes. Inverse restores exactly the provided fields.
- delete-railing / delete-space: nothing references them, so no cascade and no block; inverse is the concrete `CreateRailing` / `CreateSpace` carrying the removed record. The create inverses are the concrete deletes.
- Every payload schema has `x-semio-ui` (widget, role, en+de label, group, order) per property; sparse properties are not in `required`. Labels en+de. All inverses emit one row (default), no `x-semio-inverse-rows`.

## 3. Files

Created (all under `S`): six leaf dirs `🧬️schema/🧬️mutations/{🛤️create-railing,🪚️delete-railing,🔧️set-railing,🛋️create-space,🧽️delete-space,🪑️set-space}` (descriptor, payload schema, `🦠️mutation`, `🔺️diff`, `↩️inverse`, one `🧪️tests/<case>` per case) and the matching fixture quintets under `🧫️fixtures/🧬️mutations/`. In `T`: `r3-m-railings-spaces-leaves.ts` (spec, kept; idempotent: re-emits boilerplate, fixtures, the diff/inverse sources, mounts once, and a TypeScript twin that pre-blesses the applied `after`/`diff` only while they are placeholders).

Updated (surgical): `S/🧬️schema/🧬️mutations/🦀️.rs` (six enum variants, six `KINDS` rows); `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🦀️.rs` (six mount blocks inside `//#region 🔖️Leaves`); generated facets by `bun T/r3-f1-gen-mutation-facets.ts` (aggregate json/ts/graphql/proto, wire protocol with tags 800..805, op grammar).

The generator `emitLeaf` cannot express optional properties, so the spec post-processes each generated payload file (`use crate::{..}` line, `#[value(default, skip_serializing_if = "Option::is_none")]` on optional fields) and the schema (`required`). Other sparse-set agents can copy `fixup()`.

## 4. Verification evidence

- Blessing: `BIM_BLESS=1 cargo test ... --lib railing` and `... -- create_space delete_space set_space`; the Rust-blessed `set-space/renames-and-retypes` diff is byte-identical to the independent TypeScript twin output, and the TS-prepared `after`/`diff` of the other applied cases passed unchanged before the Rust bless.
- Per case: `declared_outcome_holds`, `applies_to_the_committed_after_snapshot`, `produces_the_committed_diff`, `inverse_restores_the_before_snapshot`, `committed_json_is_canonical`, plus `inverse_diffs_sum_to_the_negative_diff` on every applied case.
- Aggregate tests (binary tags unique and present in the protocol, every committed mutation round-trips text and binary, KINDS vs variants) pass with the new leaves.
- `bun T/r3-f1-gen-mutation-facets.ts`: 88 leaves. `bun T/r3-f1-check-names.ts`: no problem inside `🧬️mutations/` (no sibling emoji collision for the six leaf dirs); it reports 31 problems elsewhere (ASCII-named Cargo/README files, peers' inference/test/fixture dirs, `📖️mutations.grammar.semio` vs `📖️.grammar.semio` written by the facet generator).

## 5. Blocker log and open items

- About three hours of waiting: the bim crate gained a dependency on `semio-s-artifact-stdio-ifc` (x-ifc, 06:49), which pulls in `stdio-contract`, `stdio-binary`, `stdio-step` and `os-infinite`, none of which compiled against the sealed `ApplyCapability` (diff-only refactor) or the stale generated icon catalog. I changed none of that; it was repaired by others, then peers' editor files (`outliner`, `schedule`, `kit`, set-slab / set-roof-shape mid-generation) blocked briefly. First green check at 10:02.
- `delete-storey` (foundation leaf) still refuses `railings` and `spaces` as unrestorable dependants. Now that `create-railing` / `create-space` exist, it can cascade them (inverse: add `CreateRailing` / `CreateSpace` rows before the storey row, raise nothing else). Left for the coordinator since the leaf is not mine.
- `delete-material` (m-materials-layers) must refuse while a railing references the material (`railings.*.material`); `create-railing` / `set-railing` already require the material to exist.
- `bun T/r3-f1-gen-oracle.ts` and `T/r3-f1-gen-feature.ts` were not run (brief: coordinator runs them last).
- Not enforced on purpose: orientation (CCW) of explicit space outlines, self-intersection, and whether a bounded seed lies inside walls (inference territory).

## 6. Scratch

`T/🗑️generated/m-railings-spaces/` (cargo logs) deleted. Kept: spec script and this report.
