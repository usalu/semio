# r4 execution report: m-profiles-openings-types (Wave M, slice 2, wire tags 200 to 211)

`T` = this ticket folder, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. Recipe followed: `r3-golden-leaf.md`.

## 1. Result

Twelve leaves are in, green and registered: create / delete / set of column types, beam types, window types and door types.
`cargo test --lib -p semio-s-artifact-bim-model` ran 3117 tests: 3099 passed, 18 failed, none of them in my area (all 18 are the editor, viewer, IFC export/import and `spaces` inference of other agents, in progress). The filtered run of my four families (`-- column_type beam_type window_type door_type`) is 368 passed, 0 failed. `cargo check --target wasm32-wasip2` exit 0. `bun r3-f1-gen-mutation-facets.ts` (88 leaves at that moment) and `bun r3-f1-gen-feature.ts` (88 kinds) ran clean; `bun r3-f1-check-names.ts` reports no problem in `🧬️mutations/` (its remaining findings are other agents' inference fixtures, the plugin `Cargo.toml`/`package.json`, and the pre-existing `📖️` grammar duplicate in `🚪️io/📝️text/🧬️mutations`).

## 2. What was built

| Kind | Dir emoji | Tag | Cases (applied / rejected) |
|---|---|---|---|
| create-column-type | 🗼 | 200 | adds-a-round-column, adds-an-l-shaped-outline / duplicate, material-missing, non-positive-profile, clockwise-outline |
| delete-column-type | 🏺 | 201 | removes / in-use, missing |
| set-column-type | 🎚 | 202 | renames, swaps-the-profile, changes-every-field / missing, material-missing, invalid-profile, unchanged (no-op) |
| create-beam-type | 🌉 | 203 | adds-an-i-shape, adds-a-bulged-outline / duplicate, material-missing, oversized-flanges, self-crossing-outline |
| delete-beam-type | 🪢 | 204 | removes / in-use, missing |
| set-beam-type | 🎞 | 205 | renames, swaps-the-profile, changes-every-field / missing, material-missing, invalid-profile, unchanged |
| create-window-type | 🖼 | 206 | adds, adds-a-floor-level-window (sill 0) / duplicate, material-missing, non-positive-width, negative-sill, no-panes, non-positive-frame |
| delete-window-type | 🧊 | 207 | removes / in-use (an opening), missing |
| set-window-type | 🔅 | 208 | widens, lowers-the-sill-and-adds-panes, renames-and-reframes / missing, material-missing, non-positive-width, negative-sill, no-panes, non-positive-frame-depth, unchanged |
| create-door-type | 🛗 | 209 | adds-a-double-door / duplicate, material-missing, non-positive-height, non-positive-frame |
| delete-door-type | 🔒 | 210 | removes / in-use (an opening), missing |
| set-door-type | 🔑 | 211 | widens, swaps-leaves-and-swing / missing, material-missing, non-positive-height, non-positive-frame-width, unchanged |

Semantics (all in the committed `🔺️diff` / `↩️inverse` files of each leaf):
- create: refuses `mutation.duplicate-id` [id], `mutation.target-missing` [`<record>`, `material`] for an absent material, then `mutation.invariant`: column/beam types validate the profile ([`<record>`, `profile`]), window/door types validate each dimension ([`<record>`, `<field>`]). Window: width, height, frame_width, frame_depth positive, sill >= 0, panes >= 1. Door: width, height, frame_width, frame_depth positive. Inverse: the concrete delete.
- delete: `mutation.target-missing`, then `mutation.target-referenced` [id] while columns / beams / openings (`OpeningKind::Window` / `Door`) reference the type. Inverse: the concrete create with the captured record.
- set: id plus optional fields (`#[value(default, skip_serializing_if = "Option::is_none")]`, not in the schema `required`). Diff is `Entry::Patched` of exactly the provided fields (profile is one field). Refusals in order: missing id, missing material, invariants of the provided values (path `[field]`), then `mutation.no-op` when the patch changes nothing (including a payload with no field). Inverse: the same set kind carrying the base values of exactly the provided fields.
- Every leaf: en + de label, `x-semio-ui` for every payload property (reference widgets with `ref.kind` for ids and material, `record` for profile and records, `select` for door leaves/swing, `number`, `integer`, `text`), `outcomeClasses` applied/no-op/rejected, one fixture quintet per case, six tests per applied case (including the sum law) and five per rejected case.

Shared helper (new, mine): `S/🧬️schema/📸️snapshot/✅️validity/🦀️.rs`, mounted from `S/🧬️schema/📸️snapshot/🦀️.rs` (`pub mod validity; pub use validity::*;`). It exports `is_positive_length`, `is_non_negative_length`, `signed_loop_area` (exact for bulged arcs), `outline_problem` (>= 3 finite vertices, no repeated vertex because a loop closes itself, no crossing chords, CCW with an area) and `profile_problem` (rectangle, circle, I shape with web < width and 2 x flange < depth, custom outline). Other slices (slab, roof, space, railing) can reuse `outline_problem` / `signed_loop_area` through `crate::`. Logic verified standalone with plain `rustc` (L shape area 0.07, D shape with a semicircle equals 0.12 + pi * 0.02 to the last digit, clockwise, bow tie, oversized I flanges, zero rectangle and circle all classified as designed) before the cargo build was possible.

## 3. Files

Created: 12 leaf directories under `S/🧬️schema/🧬️mutations/` (🗼 🏺 🎚 🌉 🪢 🎞 🖼 🧊 🔅 🛗 🔒 🔑, each with descriptor, payload schema, `🦠️mutation`, `🔺️diff`, `↩️inverse`, `🧪️tests/*`), the matching fixture trees under `S/🧫️fixtures/🧬️mutations/` (68 cases), and `S/🧬️schema/📸️snapshot/✅️validity/🦀️.rs`.
Surgically edited shared files: `S/🧬️schema/🧬️mutations/🦀️.rs` (12 enum variants after `SetWallTop`, 12 `KINDS` rows after `"set-wall-top"`), `A/🦀️.rs` (12 mount blocks before the end of the Leaves region), `S/🧬️schema/📸️snapshot/🦀️.rs` (validity mount). Regenerated by the idempotent generators: the mutation facets (`🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto`, wire `📡️.protocol.semio`, op grammar) and `S/🧪️tests/🏙️mutate-model-1-any/{🥒️.feature,🦀️.rs}`.
Kept inputs in `T`: `r3-m-profiles-openings-types-leaves.ts` (spec, cases, registration, idempotent), `r3-m-profiles-openings-types-rust.ts` (renders the hand-written `🔺️diff` / `↩️inverse` from one entity description each, plus the validity module; the rendered files are the source of truth now).

## 4. Commands and results (all through `🚦️gate.sh`)

| Command | Result |
|---|---|
| `bun T/r3-m-profiles-openings-types-leaves.ts` | 12 leaves emitted, registered once (idempotent on rerun) |
| `cargo check -p semio-s-artifact-bim-model --lib` | exit 0 (after the stdio / editor breaks of other streams cleared) |
| `BIM_BLESS=1 cargo test --lib -- column_type beam_type window_type door_type` | blessed `➡️after` and `🔺️diff` of the 22 applied cases; 16 `committed_json_is_canonical` tests failed during the bless run only because they race the placeholder files (expected) |
| `cargo test --lib -- column_type beam_type window_type door_type` | `368 passed; 0 failed` |
| `cargo test --lib` (whole crate) | `3099 passed; 18 failed` (editor, viewer, ifc export/import, spaces inference of other agents; the aggregate tests `kinds_match_the_enum_and_the_catalog`, `binary_tags_are_unique_and_registered_in_the_protocol`, `every_committed_mutation_round_trips_text_and_binary` all pass with my kinds) |
| `cargo check --lib --target wasm32-wasip2` | exit 0 |
| `bun T/r3-f1-gen-mutation-facets.ts`, `bun T/r3-f1-gen-feature.ts`, `bun T/r3-f1-check-names.ts` | clean for my leaves (see section 1) |

The blessed diffs were reviewed by hand (sparse `Created` / `Deleted` / `Patched` of exactly the provided fields, floats and nested profile enums canonical).

## 5. Decisions and traps

1. Optional fields in set payloads: `emitLeaf` has no notion of optional properties, so the spec file post-processes the emitted payload struct (`Option<T>` with the `value` attribute) and the schema (`required` without the optional names) after `emitLeaf`. Another slice that needs optional payload fields can copy `patchOptional` from `r3-m-profiles-openings-types-leaves.ts`.
2. A set payload with no field, or with values equal to the base, is `mutation.no-op` [id]; a payload that mixes changed and equal fields keeps all provided fields in the patch ("exactly the provided fields") so the inverse restores exactly them.
3. Emoji collisions: three of my first picks (🪦 delete-column, 🎛 set-column, 🛖 create-roof-type, 🪞 flip-wall, 🪓 split-wall) were taken by peers within minutes; I re-picked 🏺 / 🎞 / 🛗 / 🧊 / 🪢 before any registration of the old ones stuck (the old 🪓 dir and fixtures were removed by a one-off script). Final listing of `🧬️mutations/` has no duplicate leading code point.
4. Window `frame_width` and `frame_depth` must be positive; no "frame fits inside the opening" rule was added (not in the brief).
5. Custom outline: arcs are not checked for self-intersection, only chords (documented in the helper's docstring). A repeated first/last vertex is refused because the loop closes itself.

## 6. Open issues / hand-off

1. Blocker for everyone during this run (not mine, resolved by other streams in the meantime): `cargo check` of the BIM crate failed for about 90 minutes because the IFC export of another stream made the crate depend on `semio-s-artifact-stdio-ifc`, whose chain (`stdio-contract`, `stdio-binary`, `stdio-txt`, `stdio-step`, `stdio-ifc`) still called `MutationDiff::apply` with two arguments, and `semio-framework-os-infinite` had an icon (`CloudDownload`) missing from the generated catalog.
2. The 18 failing crate tests above belong to the editor, viewer, IFC and spaces streams.
3. Not run by me (coordinator, last): `bun T/r3-f1-gen-oracle.ts` (mutation catalog rows for the new kinds) and the platform check of `🏙️mutate-model-1-any` (the adapter is regenerated for 88 kinds but compiled by the platform, not by me); the prose sentence of `🥒️.feature` still lists the foundation families.
4. The text op grammar and binary protocol already carry the 12 kinds (the aggregate round trip test passes on every committed fixture).

## 7. Scratch

`T/🗑️generated/m-profiles-openings-types/` (cargo logs, the one-off emoji fix script, mounts text) was deleted at the end. The standalone validity probe lived in the session scratchpad only.
