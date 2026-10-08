# r4 execution report: m-multi-data (Wave M slice 9, binary tags 900 to 908)

`T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN`. `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`. `M` = `S/🧬️schema/🧬️mutations`.

## 1. Result

Part A (nine leaves) and Part B (cascades of the four foundation delete leaves over every collection that has a create leaf) are done, blessed, reviewed and green. All 14 element collections have a create leaf by now (site, building, storey, grid line, wall, curtain wall, column, beam, slab, roof, stair, railing, space, opening), so Part B is complete, nothing remains.

Verification (details in section 5): 352 of 352 tests of my 13 leaves pass; the aggregate unit tests (kinds, unique tags, text and binary round trip of every committed mutation) pass; the full crate gate `cargo test -p semio-s-artifact-bim-model --lib` shows 2989 passed and 124 failed, none of the failures in my leaves (they are unblessed fixtures of `set-column`, `create-column`, `*-type`, `delete-beam`, the editor, the ifc export and inference tests of peers, still in progress).

## 2. Part A: the nine leaves

| Kind (tag) | Dir emoji | Cases (applied / rejected) | Inverse |
|---|---|---|---|
| `move-elements` (900) | 🚚 | wall plus its opening, every placed kind / unknown id, empty selection, zero vector (no-op), storey has no placement | one `place-elements` with the exact base placements of the changed elements |
| `rotate-elements` (901) | 🎡 | wall and column, every placed kind, 30 degrees / unknown id, empty, zero angle (no-op), storey | one `place-elements` (rotation fields included) |
| `delete-elements` (902) | 💣 | wall with opening and data, nine other kinds, a storey with everything on it / unknown id, empty, pinned storey | `cascade::inverse`: one concrete create per removed record plus one data setter per property and classification; `x-semio-inverse-rows` bounded 65536 |
| `rename-element` (903) | 🪪 | wall, grid line label / unknown id, same name (no-op) | absolute `rename-element` |
| `set-element-property` (904) | 🧾 | first property, existing set, replaced value / unknown element, mismatched value, same value (no-op) | absolute `set-element-property` or `remove-element-property` |
| `remove-element-property` (905) | 🫧 | one of two, last property (entry deleted) / missing property, unknown element | concrete `set-element-property` |
| `set-element-classification` (906) | 🗂 | first classification, reclassify / unknown element, empty code, same (no-op) | absolute `set-element-classification` or `remove-element-classification` |
| `remove-element-classification` (907) | 🗄 | removes / not classified, unknown element | concrete `set-element-classification` |
| `place-elements` (908, own leaf) | 🪧 | absolute placements / unknown id, kind mismatch, same placement (no-op), empty map | `place-elements` with the base placements |

Design decisions:
- **`place-elements` is the exact inverse of move and rotate.** A back translation or back rotation is never used, so floating point leaves no residue. Its payload is `placements: BTreeMap<id, Placement>`; `Placement` is a tagged enum (`Wall{axis}`, `CurtainWall{axis}`, `Column{position,rotation}`, `Beam{start,end}`, `Slab{boundary,holes,slope?}`, `Roof{footprint,shape}`, `Stair{start,direction}`, `Railing{path}`, `Space{boundary}`, `Grid{start,end}`), the full placement field set of each kind. Move, rotate and place all emit exactly that field set per element, so the sum law (forward negative equals summed inverse diffs) holds field for field. The verb of `place-elements` is `set` (the framework `APPROVED_VERBS` has no `place`).
- Openings are accepted in a selection and skipped (they follow their host by inference); a selection that changes nothing is `mutation.no-op`. Sites, buildings and storeys have no placement of their own (`mutation.invariant`, path `[id]`). Bounded spaces move/turn their seed point; explicit spaces their outline.
- Rotation is counter-clockwise about the pivot; rotated points and turned direction fields (column rotation, stair direction, slab slope direction, roof ridge/shed direction) are snapped to 1e-12 so quarter turns stay exact and implementations agree; JSON round trips of the committed fixtures are exact (a first version without the snap failed on 17-digit floats through the test kit's non-roundtrip float parser).
- Property and classification leaves operate on existing elements only (all 14 element collections); removing the last property deletes the whole properties entry (no empty property set). Typed values must match their kind: Text only `text`, Boolean only `flag`, every other kind a finite `number`.

## 3. Part B: cascades

New shared module `M/🌊️cascade/🦀️.rs` (mounted `mutations::cascade`): `closure` (site, buildings, storeys and grid lines, storey contents, openings of removed walls and curtain walls, properties and classifications of every removed element), `Removal::diff`, `Removal::inverse` (creation order reversed: dependants first, sites last; data setters first), `outcome` and `inverse` helpers. The only remaining refusal is `mutation.target-referenced` for a surviving wall, curtain wall, column or stair whose top constraint points at a removed storey (cannot cascade); types are never blocked by element deletion.

The four leaves keep their dirs and kinds and now delegate: `delete-site`, `delete-building`, `delete-storey` (bounded 65536), `delete-wall` (bounded 4096). Retired cases: `has-buildings`, `has-storeys`, `hosts-an-opening` (storey and wall). New cases: `cascades-its-buildings`, `cascades-the-whole-site`, `cascades-its-storeys`, `cascades-the-whole-building`, `cascades-the-opening`, `cascades-everything-on-it`, `cascades-the-opening-and-data`, and `missing` in every leaf; kept: the `removes`, `cascades-the-walls`, `empty-storey`, `constrains-another-wall` cases. The aggregate unit test `delete_storey_declares_a_bounded_footprint...` was updated from 4096 to 65536.

## 4. Files

Created (all under `M/` unless noted):
- Shared modules: `🧵️elements/🦀️.rs` (Placement, placement diff, element identity, rename), `🌊️cascade/🦀️.rs`.
- Nine leaf dirs, each with `🔣️.json`, `🧬️schema/🔣️.json`, `🦠️mutation/🦀️.rs`, `🔺️diff/🦀️.rs`, `↩️inverse/🦀️.rs`, `🧪️tests/<case>/🦀️.rs`: `🚚️move-elements`, `🎡️rotate-elements`, `💣️delete-elements`, `🪪️rename-element`, `🧾️set-element-property`, `🫧️remove-element-property`, `🗂️set-element-classification`, `🗄️remove-element-classification`, `🪧️place-elements` (its payload schema carries a `$defs.Placement`).
- Fixtures `S/🧫️fixtures/🧬️mutations/<leaf>/<case>/…` for the nine leaves and the new cases of the four delete leaves (blessed from the code, reviewed by hand).
- Inputs in `T/`: `r3-m-multi-data-leaves.ts` (spec, `register` mode splices mounts, enum and KINDS), `r4-m-multi-data-cascades.ts` (Part B spec, retires cases, merges case mounts), `r4-m-multi-data-priv-setup.ts` (private build workspace), `r4-m-multi-data-oracle.py` (shapely cross-check).

Updated (surgical): `A/🦀️.rs` (mount blocks of nine leaves, new case mounts of four, retired mounts), `M/🦀️.rs` (nine enum variants, nine KINDS rows, `mod elements;`, `mod cascade;`), `M/🧪️tests/🔬️unit/🦀️.rs` (65536), the four delete leaves (`🦠️mutation`, `🔺️diff`, `↩️inverse`, schema, tests, fixtures), generated facets (`🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto`, wire protocol, grammar), oracle catalog and feature rows. Shared generators changed minimally and additively: `T/r3-f1-gen-leaf.ts` (new optional `uses: string[]` of extra `use` lines; the import filter accepts only plain identifiers so `Vec<..>`/`BTreeMap<..>` field types no longer produce a broken `use crate::{Vec<String>}`) and `T/r3-f1-gen-mutation-facets.ts` (an object schema with `additionalProperties` becomes `Record<string, unknown>` / `JSON` scalar / `map<string, google.protobuf.Value>`). `T/r3-f1-leaves.ts` still describes the old foundation cases of the four delete leaves and must not be re-run (its cases would resurrect the retired fixtures); `r4-m-multi-data-cascades.ts` is now the spec of those four leaves.

## 5. Commands and results

Every cargo call through the gate. The shared workspace did not build for hours (peer changes: `apply` capability argument in the stdio crates, tests of other leaves), so most runs used a private workspace written by `T/r4-m-multi-data-priv-setup.ts` (schema tree only, no io export/import, no editor/viewer, tests of peers left out, mine kept). The shared workspace built again at the end.

| Command | Result |
|---|---|
| private: `cargo test -p semio-s-artifact-bim-model --lib -- <my 13 leaf filters>` (after `BIM_BLESS=1` for the applied cases) | 352 passed, 0 failed |
| private: aggregate unit tests (`mutations::component::tests`) | 4 passed after the 65536 fix (kinds, unique tags, round trip of every committed mutation in text and binary) |
| shared: `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --lib` | 2989 passed, 124 failed; none in my leaves; failures are peers' unblessed fixtures (`set-column`, `set-beam`, `*-window/door/beam-type`, `create-column/beam`, `delete-beam/column`), editor/viewer, ifc export, inference/render tests |
| private: `cargo check --target wasm32-wasip2` | exit 0 |
| shared: `cargo check --target wasm32-wasip2` | exit 101, one error in a peer file (`✏️editor/🧵️inference/🦀️.rs:51` macro `_` token); my files are not involved |
| `bun T/r3-f1-gen-mutation-facets.ts`, `r3-f1-gen-oracle.ts` (88 kinds, 472 scenarios), `r3-f1-gen-feature.ts` (88 kinds) | ok |
| `bun T/r3-f1-check-names.ts` | 31 problems, none in `M/` or its fixtures (editor, inference, ifc, tests dirs of peers); emoji of all siblings in `M/`, `S/🧫️fixtures/🧬️mutations/` and my test/fixture case dirs are unique |
| third-party cross-check `.venv/Scripts/python.exe T/r4-m-multi-data-oracle.py` (shapely 2.1.2 `affinity.translate`/`rotate`) | move 12, rotate 15, place 2 element placements checked, 0 differ, unselected elements unchanged |

## 6. Open items for others

- `u-editor`/`u-tools`: `delete-selection` can now emit one `delete-elements`, a drag one `move-elements`, a rotate gesture `rotate-elements` (editor `🧩️entities` still maps delete to the four single kinds, which cascade now).
- `x-examples`/`i-*`: no snapshot-affecting change; `delete-*` of a storey now removes openings and data instead of refusing.
- The root `A/🦀️.rs` and the aggregate `M/🦀️.rs` switched to CRLF line endings during the session (written by another tool); my scripts are EOL tolerant.
- The platform oracle catalog does not yet register a third-party mutation oracle (`bim-1-mutate`); the shapely script above is the cross-validation until Wave X registers one.
