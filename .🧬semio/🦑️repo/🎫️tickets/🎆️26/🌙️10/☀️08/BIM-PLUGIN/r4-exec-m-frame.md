# r4 execution report: m-frame (Wave M slice 5, binary tags 500 to 505)

`T` = ticket folder `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. Result

Six leaves under `S/🧬️schema/🧬️mutations/`, all green: `create-column` (🏛️, tag 500), `delete-column` (🪦️, 501), `set-column` (🎛️, 502), `create-beam` (➖️, 503), `delete-beam` (✂️, 504), `set-beam` (📐️, 505). 24 fixture cases, 188 tests (6 per applied case incl. the sum law, 5 per rejected case, plus 2 authored-top tests). Leaf emojis are unique among all 100 siblings of `🧬️mutations/` (checked at the end).

| Leaf | Applied cases | Rejected cases |
|---|---|---|
| create-column | adds-to-the-storey-top, spans-into-the-storey-above | duplicate, storey-missing, type-missing, top-storey-missing, top-storey-other-building, top-below-base, zero-height |
| delete-column | removes | missing |
| set-column | retypes-moves-and-renames, constrains-the-top, keeps-equal-fields-out-of-the-diff | missing, type-missing, top-storey-missing, top-storey-other-building, top-below-base, base-above-top, nothing-to-change (no-op) |
| create-beam | adds-below-the-storey-top | duplicate, storey-missing, type-missing, zero-length |
| delete-beam | removes | missing |
| set-beam | retypes-and-stretches, lowers-the-beam, keeps-equal-fields-out-of-the-diff | missing, type-missing, zero-length, nothing-to-change (no-op) |

## 2. Design decisions

- `set-column` / `set-beam` are sparse: every payload field except `id` is `Option` (`#[value(default, skip_serializing_if = "Option::is_none")]`, schema `required` is only `mutation` + `id`, descriptions "leave empty to keep"). The diff patches only fields that really change (equal values are dropped, blessed fixtures `keeps-equal-fields-out-of-the-diff` prove it); all fields equal gives `mutation.no-op`.
- The inverse of a set is an absolute set restoring exactly the fields the forward really changed; none when nothing changes.
- "Top below base" is decided from authored parameters only, using the same datum rule as the inference: `Unconnected{height}` needs height > 0, `StoreyTop{offset}` needs `storey.height + offset - base_offset > 0`, `Storey{storey, offset}` needs target elevation + offset - own elevation - base_offset > 0 (elevation gap via `inferences::storey_levels::compute_storey_levels`, only when the target is another storey). Refusal `mutation.invariant`, path `["column","top"]` (create) / `["top"]` or `["base_offset"]` (set).
- Nothing derived is stored: `Column` has no height, `Beam` no elevation. Per create leaf a hand test (`🧪️tests/📌️authored-top-is-stored`, `📌️authored-top-offset-is-stored`) asserts the authored constraint is stored verbatim, the wire record has exactly the authored keys, and a taller storey (via `set-storey-height`) leaves the stored record untouched.
- Deletes have no dependants (nothing hosts on columns/beams), properties/classifications are orphan-tolerant.
- Finite checks (position, rotation, offsets) refuse with `mutation.invariant`.
- `ui`: `x-semio-ui` on every property (reference for ids and types, `record` for Point2/TopConstraint, `dial` rad/deg for rotation, `number` with unit m), en + de labels (Stütze, Träger, Stützentyp, Trägertyp, ...).

## 3. Files

Created: the six leaf directories (descriptor, payload schema, mutation, `🔺️diff`, `↩️inverse`, tests) and 24 fixture directories under `S/🧫️fixtures/🧬️mutations/`; spec `T/r3-m-frame-leaves.ts` (idempotent: `bun r3-m-frame-leaves.ts [register]` rewrites generated files, patches the optional fields, writes the authored-top tests, registers missing mounts surgically).
Updated: `S/🧬️schema/🧬️mutations/🦀️.rs` (6 variants, 6 `KINDS`), `A/🦀️.rs` (6 mount blocks before `//#endregion 🔖️Leaves`), generated facets (aggregate json/ts/graphql/proto, wire protocol, op grammar) via `bun T/r3-f1-gen-mutation-facets.ts`, `S/🧪️tests/🏙️mutate-model-1-any/*` via `bun T/r3-f1-gen-feature.ts`.
Shared generator edit: `T/r3-f1-gen-mutation-facets.ts` now emits optional properties as optional (TS `name?:`, GraphQL without `!`, proto `optional`) from the schema `required` list; before, sparse payloads were declared required in all facets (also for the other agents' sparse leaves).

## 4. Commands and results

All through `T/🚦️gate.sh m-frame -- cargo ...`.
- `cargo test -p semio-s-artifact-bim-model --lib -- mutations::create_column:: mutations::delete_column:: mutations::set_column:: mutations::create_beam:: mutations::delete_beam:: mutations::set_beam::` with `BIM_BLESS=1` once (wrote `➡️after` and `🔺️diff` of the 11 applied cases; reviewed by hand: sparse, no derived field), then without: `188 passed; 0 failed`.
- `cargo test -p semio-s-artifact-bim-model --lib` (full): `3099 passed; 18 failed`. None of the 18 is mine: 13 `editor::*`, 1 `viewer`, 3 `io::*ifc*`, 1 `inferences::spaces::an_island_wall_is_a_hole_of_the_room` (u-editor, x-ifc, i-spaces owners, in progress).
- `cargo check -p semio-s-artifact-bim-model --target wasm32-wasip2`: exit 0.
- `bun T/r3-f1-gen-mutation-facets.ts`: 88 leaves, ok. `bun T/r3-f1-check-names.ts`: no problem in my files (31 problems, all in peers' trees: non-emoji fixture folders, duplicate emojis in inference folders). `bun T/r3-f1-gen-feature.ts`: 88 kinds. `bun T/r3-f1-gen-oracle.ts`: fails ("the manifest has no mutationCatalogs/mutationManifests block to replace"), the oracle manifest was restructured by someone else; not touched.

## 5. Build obstacles (for the coordinator)

For about 3 hours the crate could not compile: x-ifc's new dependency `semio-s-artifact-stdio-ifc` pulled `stdio-contract` / `stdio-binary` (callers of the new three-argument `MutationDiff::apply`) and `os-infinite` (stale gitignored generated icon catalog lacking `CloudDownload`); then peers' dangling `#[path]` mounts (curtain-layout tests, viewer windows, editor chrome tests) and compile errors in peers' leaf tests. I did not touch any of it; it resolved by itself. A private workspace experiment (copy of the manifest without the ifc dependency) proved my leaves compile with no diagnostics while the full crate was blocked; it was deleted.

## 6. Open items

1. `delete-storey` (golden leaf) still refuses with `mutation.target-referenced` while a column or beam stands on the storey; now that both have create leaves it should cascade over them (inverse: `CreateColumn` / `CreateBeam` rows before the storey). One consolidation pass after all waves should extend it with every kind that has a create leaf; I did not edit the shared golden leaf.
2. Column/beam type deletes (`delete-column-type`, `delete-beam-type`, m-profiles) must refuse while columns/beams reference the type; the fixtures here reference `ct-400/ct-500`, `bt-30x50/bt-40x60`.
3. The `rise` helper is duplicated in `create-column` and `set-column` diff (kept adjacent; a leaf may not import another leaf).
4. `T/r3-f1-gen-oracle.ts` must be re-run once the oracle manifest block exists again.
