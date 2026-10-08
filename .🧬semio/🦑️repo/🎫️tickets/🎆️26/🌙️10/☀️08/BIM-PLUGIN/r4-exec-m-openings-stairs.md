# r4 execution report: m-openings-stairs (Wave M, slice 7, binary tags 700..706)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. Result

Seven leaves, 48 fixture cases, all green: `cargo test -p semio-s-artifact-bim-model --lib -- <the seven kinds> kinds_match binary_tags every_committed` gives 258 passed, 0 failed (255 for the seven leaves alone: five tests per case, six for applied cases). Full lib run (last): 3106 passed, 14 failed; every failure is in `editor::bim::*`, `viewer::bim::*` or `io::export::ifc::projection` (other agents' in-progress work), none in `schema::mutations`. `cargo check --target wasm32-wasip2` exit 0. `bun T/r3-f1-gen-mutation-facets.ts` ran (88 leaves). `bun T/r3-f1-check-names.ts`: no problem in `🧬️mutations/` (the remaining reports are others' fixture folders and the pre-existing grammar and inference duplicates).

| Leaf | Tag | Dir emoji | Applied cases | Rejected cases |
|---|---|---|---|---|
| `create-opening` | 700 | 🕳️ | adds-a-window, adds-a-door-to-a-curtain-wall, adds-a-void | duplicate, host-missing, type-missing, negative-sill, non-positive-width, beyond-the-host-end, overlaps-a-neighbour |
| `delete-opening` | 701 | 🪓️ | removes | missing |
| `move-opening` | 702 | 🛷️ | slides-along-the-host, rehosts-to-another-wall | missing, host-missing, beyond-the-host-end, overlaps-a-neighbour, already-there (no-op) |
| `set-opening` | 703 | 🕹️ | resizes, clears-the-width-override, retypes-and-flips | missing, type-missing, negative-sill, non-positive-height, too-wide-for-the-host, overlaps-a-neighbour, already-set (no-op) |
| `create-stair` | 704 | 🧗️ | adds-a-straight-flight, adds-a-u-run-to-the-first-storey | duplicate, storey-missing, non-positive-width, invalid-riser, invalid-tread, broken-flight, top-storey-missing |
| `delete-stair` | 705 | 🧨️ | removes | missing |
| `set-stair` | 706 | 🧮️ | moves-and-widens, turns-into-an-l-run, renames-and-flattens | missing, non-positive-width, invalid-riser, top-storey-missing, already-set (no-op) |

## 2. Semantics

- Host of an opening: a wall, else a curtain wall (`placement::host_length`, axis length through `inferences::wall_layout::axis_length`; nothing stored). Offset = distance along the axis from its start to the opening centre.
- Opening width = own override, else the width of its window type / door type, else the void width. `placement_issue`: centre in `[w/2, length - w/2]` (1e-9 tolerance) and `|d offset| >= (w + w_other) / 2` against every other opening on the host (touching is allowed).
- `create-opening` refusal order: duplicate id, host missing (`["opening","host"]`), type missing (`["opening","kind"]`) or degenerate void, sill < 0 (`["opening","sill"]`), non-positive width/height override, placement (`["opening","offset"]`, `mutation.invariant`).
- `move-opening { id, offset, host? }`: patch of `offset` and `host` (only differing fields); inverse = `move-opening(base offset, base host when the move named one)`. No-op when host and offset are unchanged.
- `set-opening { id, kind?, sill?, width?, height?, flip_hand?, flip_facing?, name? }`: patch of the provided fields that differ; width/height are `Assigned<Option<f64>>` on the wire (`{"value": 1.5}` sets, `{"value": null}` clears the override). A changed kind or width must still fit and not overlap (path `["width"]` or `["kind"]`). Inverse = `set-opening` with the base values of exactly the provided fields. Nothing provided that differs: `mutation.no-op`.
- `create-stair` / `set-stair` share `placement::stair_issue` (start/direction finite, width, max_riser, min_tread positive, flight sane: L-turn split > 0, U-turn gap >= 0, spiral radius > 0 and sweep != 0, top: unconnected height > 0, storey top must exist and be of the stair's building). `set-stair` validates the patched stair as a whole.
- Deletes: `delete-opening`, `delete-stair` have no dependants, one `mutation.target-missing` refusal; inverses recreate the captured record (`create-opening`, `create-stair`). Creates invert to the matching delete.
- All inverses are one concrete row; sum-law test in every applied case; every payload property carries `x-semio-ui`, en + de labels.
- Optional payload fields (`Option<..>` with `#[value(default, skip_serializing_if = "Option::is_none")]`, not in `required`) are patched in by `r3-m-openings-stairs-leaves.ts` after `emitLeaf`, because `emitLeaf` emits neither the attribute nor a correct `use` for `Option<...>`.

## 3. Files

Created:
- 7 leaf dirs under `S/🧬️schema/🧬️mutations/` (`🕳️create-opening`, `🪓️delete-opening`, `🛷️move-opening`, `🕹️set-opening`, `🧗️create-stair`, `🧨️delete-stair`, `🧮️set-stair`), each with `🔣️.json`, `🧬️schema/🔣️.json`, `🦠️mutation`, `🔺️diff`, `↩️inverse`, `🧪️tests/*` (48 case files).
- 48 fixture quintets under `S/🧫️fixtures/🧬️mutations/<same dirs>/`.
- Shared module `S/🧬️schema/🧬️mutations/📍️placement/🦀️.rs` (opening placement reads, stair invariants), mounted by one line pair in the aggregate (`#[path = "📍️placement/🦀️.rs"] pub mod placement;`).
- `T/r3-m-openings-stairs-leaves.ts` (spec, idempotent; hand-written Rust is installed only when the target is missing, the staging copies were deleted with the generated folder, the artifact is the source of truth).

Updated (surgical): `S/🧬️schema/🧬️mutations/🦀️.rs` (7 variants, 7 kebab kinds, `placement` mod), artifact root `A/🦀️.rs` (7 mount blocks before `//#endregion 🔖️Leaves`), generated facets (`🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto`, wire protocol) via the generator, and `T/r3-f1-gen-mutation-facets.ts`: boolean payload fields now map to `boolean` / `Boolean` / `bool` (they were `number` / `Float` / `double`).

## 4. For the coordinator

1. Overlap: m-walls' `🪝️rehost-opening` (`RehostOpening { id, host, offset }`, tag in its own slice) and my `move-opening` both re-host. `rehost-opening` deliberately skips fit/overlap checks ("a diagnostic, not a refusal") while `move-opening` refuses them. Decide which stays; nothing else depends on the choice.
2. `delete-wall`, `delete-storey` cascade openings and `delete-curtain-wall` refuses them (peers' cases). Check `delete-storey`: stairs (and openings of curtain walls) have create leaves now, so it can cascade over them like walls instead of refusing with `mutation.target-referenced`.
3. `delete-wall` and `set-wall-axis` do not validate hosted openings against the new axis (an axis that becomes shorter than an opening's offset leaves it outside the host). That is an inference diagnostic (`⚠️diagnostics`), or a refusal in `set-wall-axis`, for the owners.
4. Facet generators: `Assigned` payload fields are typed `Record<string, unknown>` / `map<string, Value>` / `JSON` in ts/proto/graphql (same for other agents' optional-assigned fields); acceptable approximation.
5. `r3-f1-gen-oracle.ts` and `r3-f1-gen-feature.ts` were NOT run (they read every leaf on disk, including peers' unfinished ones); run them last.

## 5. Commands and results

All through `T/🚦️gate.sh m-openings-stairs --`.
- `cargo check -p semio-s-artifact-bim-model` blocked for about 2.5 h by other work: `bim-model` gained `semio-s-artifact-stdio-ifc` (x-ifc) whose dependency chain (`stdio-contract`, `stdio-binary`, `stdio-step`) did not build against the new `MutationDiff::apply` / `apply_diff` signature, plus a stale generated icon enum in `os-infinite`; then the lib test target was blocked by others' test files. I did not touch any of it. Final: check exit 0.
- `BIM_BLESS=1 cargo test --lib -- <seven kinds>` (first blessing raced the canonical tests, 18 expected failures; one real fixture error found and fixed: `set-opening/overlaps-a-neighbour` needed width 5, not 2.5), second blessing 255 passed; reviewed the blessed `after`/`diff` JSON by hand (sparse: only the touched fields; width clear is `{"value": null}`).
- Without bless: 255 passed, 0 failed (leaves), 258 with the aggregate tests (kinds match, tags unique and registered, every committed mutation round-trips text and binary).
- `cargo check --target wasm32-wasip2`: exit 0 (retried until m-frame's `set-column` was fixed).
- Rules R8 to R16: no `&mut` in diff/inverse, no `apply(`, `between(`, `diff(` in inverses, no base clone, every applied case has the sum-law test.

Scratch folder `T/🗑️generated/m-openings-stairs/` deleted.
