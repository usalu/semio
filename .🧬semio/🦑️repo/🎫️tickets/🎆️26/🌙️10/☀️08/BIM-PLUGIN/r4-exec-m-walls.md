# r4 execution report: m-walls (Wave M slice 4, binary tags 400 to 499)

`S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `M` = `S/🧬️schema/🧬️mutations`, `T` = this ticket folder.

## 1. Result

Ten leaves and one shared helper landed, all green: 307 leaf tests (generated six-test case files plus the hand-written parametric files) and 10 helper tests pass, the aggregate tests (`kinds_match_the_enum_and_the_catalog`, `binary_tags_are_unique_and_registered_in_the_protocol`, `every_committed_mutation_round_trips_text_and_binary`) pass with my kinds, and the lib builds for `wasm32-wasip2`. The unfiltered `cargo test --lib` ended at 3014 passed, 103 failed; none of the failures is in my modules (see section 6).

## 2. Leaves

| Kind (verb) | Dir emoji | Tag | Payload | Inverse rows |
|---|---|---|---|---|
| `set-wall-axis` (set) | 〰️ | 400 | `id`, `axis: Axis` | 1 absolute `SetWallAxis` |
| `set-wall-base-offset` (set) | 🔽️ | 401 | `id`, `base_offset` | 1 |
| `set-wall-type-of` (set) | 🥞️ | 402 | `id`, `wall_type` | 1 |
| `set-wall-location` (set) | 🦚️ | 403 | `id`, `location: LocationLine` | 1 |
| `flip-wall` (toggle) | 🪞️ | 404 | `id` | 1 (`FlipWall`, none when the axis is its own reverse) |
| `split-wall` (split) | 🦈️ | 405 | `id`, `t`, `new_id` | bounded 4096: `SetWallAxis(original)`, `DeleteWall(new)`, one `RehostOpening` back per moved opening |
| `create-curtain-wall` (create) | 🏬️ | 406 | `id`, `curtain_wall: CurtainWall` | 1 `DeleteCurtainWall` |
| `delete-curtain-wall` (delete) | 🦖️ | 407 | `id` | 1 `CreateCurtainWall` with the full record |
| `set-curtain-wall` (set) | 🔆️ | 408 | `id` plus optional `axis, base_offset, top, u_spacing, v_spacing, mullion, panel_material, mullion_material, name` | 1, naming only the changed fields at base values |
| `rehost-opening` (move) | 🪝️ | 409 | `id`, `host`, `offset` | 1 |

Helper (not a leaf, no descriptor): `M/🦉️wall-geometry/🦀️.rs` mounted as `mutations::wall_geometry`: axis validity (`flaw`: finite coordinates, chord above 1 nm, finite bulge, sweep strictly between a nanometre and a full turn), `flipped`, `split` (arc point by centre and sweep share, sub-bulge `tan(t * atan(bulge))`, snapped to 1 nm and 1e-12 so every device derives the same diff), top/spacing/profile/material checks for curtain walls, and a `#[cfg(test)] testing` module (`decode`, `close`, `layout`, `refusal`).

Behaviour notes:
- `set-wall-axis`: refusals `mutation.invariant` at `["axis"]` or `["axis","bulge"]` (zero length, non-finite, flat or full-turn arc), `no-op`, `target-missing`. Hosted openings are never touched.
- `split-wall`: `t` strictly inside (0,1) and both parts valid, else `invariant ["t"]`; `duplicate-id ["new_id"]`; the new wall copies storey, type, location, base, top, phase and name. An opening with `offset >= t * length` is re-hosted by a sparse `OpeningPatch {host, offset}` (offset re-based, snapped); the diff carries an info `mutation.cascade` with the count. The inverse is ordered so the store's reversed replay restores openings, then deletes the new wall, then restores the axis; the sum-law test of every applied case passes.
- `rehost-opening` is a new kind (`m-openings-stairs` has `move-opening`/`set-opening` but neither changes the host). The host may be a wall or a curtain wall; offsets must be finite and non-negative; "does the opening still fit" stays a diagnostic.
- `delete-curtain-wall` refuses `target-referenced` while an opening names the curtain wall as host (the snapshot allows it), otherwise cascades nothing.
- `set-curtain-wall` validates only the named fields, patches only fields that differ, and answers `no-op` when nothing differs.
- Verbs: the derive accepts only the approved verb table, so `flip-wall` uses `toggle` and `rehost-opening` uses `move` (kinds keep their names).

## 3. Files

Created: ten leaf dirs under `M` (descriptor, payload schema, `🦠️mutation`, hand-written `🔺️diff` and `↩️inverse`, 3 to 7 generated case test files each, one hand-written `🧪️tests/🔗️follows-by-inference` file each), `M/🦉️wall-geometry` (+ `🧪️tests/🔬️unit`), 53 fixture quintets under `S/🧫️fixtures/🧬️mutations/<leaf>/<case>` (21 applied, 32 rejected or no-op), and in `T`: `r3-m-walls-leaves.ts` (spec + `emitLeaf` driver, `--register` for the surgical registration), `r4-m-walls-expected.ts` (second implementation, below), this report.

Cases per leaf (applied / rejected): set-wall-axis 3/4, set-wall-base-offset 2/2, set-wall-type-of 1/3, set-wall-location 2/2, flip-wall 2/2, split-wall 3/4 (straight, arc, openings beyond the split; start, beyond end, id taken, missing), create-curtain-wall 2/5, delete-curtain-wall 1/2, set-curtain-wall 3/4, rehost-opening 2/4. Non-JSON inputs (NaN/inf bulge, offsets, spacings, degenerate profiles) are covered in code by the follows files and the helper tests.

Shared files touched (surgical): `M/🦀️.rs` (10 variants at the top of `ModelMutation`, 10 `KINDS` rows), the artifact root `🦀️.rs` (mount blocks before `//#endregion 🔖️Leaves`: `wall_geometry` plus ten leaves with their `tests_follows_by_inference`), and the generated facets by `bun T/r3-f1-gen-mutation-facets.ts` (aggregate `🔣️.json`, `🟦️.ts`, `🔗️.graphql`, `🛰️.proto`, wire protocol, op grammar; optional fields of `set-curtain-wall` come out as `?`). `r3-f1-gen-feature.ts` ran (88 kinds, my rows present); the oracle catalog already lists my kinds.

## 4. Parametric tests (the `🧱️wall-layout` inference follows)

One file per leaf in `🧪️tests/🔗️follows-by-inference`: set-wall-axis (length 8 to 10, bulge 0.5 gives arc length 9.2729..., height and thickness unchanged, other wall keeps its extent, openings keep their record), set-wall-base-offset (StoreyTop wall: base up, top stays, height down; free-height wall: lifted whole), set-wall-type-of (thickness 0.3 to 0.15, volume down, extent kept), set-wall-location (offsets `(0.15,0.15)`, `(0,0.3)`, `(0.3,0)`, core `(0.15,0.15)` as in the join-aware layout, sum equals thickness, extent unchanged), flip-wall (extent unchanged, flip twice is identity), split-wall (lengths add up for a line and for a semicircle `4 pi`, openings change host with re-based offsets), the curtain wall kinds and rehost-opening (diff touches exactly `curtain_walls/<id>[/<field>]` / `openings/<id>/{host,offset}`, wall layouts untouched).

## 5. Third-party / second-implementation validation

- Closed forms in the fixtures: the semicircle `(0,6)` to `(8,6)` bulge 1 splits at `(4,2)` with both bulges `tan(pi/8) = 0.414213562373`; helper test `a_semicircle_splits_at_its_closed_form_midpoint` asserts it.
- `semio-framework-geometry::bulge::BulgeSeg` (which carries `kurbo` and `parry3d` oracles in its own tests) cross-checks `split`, `flipped` and `length` over lines and arcs of both signs and sweeps above pi (helper tests `a_split_agrees_with_the_first_party_bulge_kernel`, `flipping_twice_...`, `the_length_of_an_arc_is_the_kernel_length`).
- `bun T/r4-m-walls-expected.ts`: independent TypeScript implementation of all ten diffs; computes diff and after from `before` + `mutation` only and compares with the Rust-blessed fixtures within 1e-9: `all 42 committed documents agree (diff + after of the 21 applied cases) with the TypeScript second implementation`. The fixtures were first filled by it (`--fill`), the Rust tests passed against them unchanged, and `BIM_BLESS=1` then rewrote them canonically from the code; they still agree.

## 6. Commands and results

| Command (all through `🚦️gate.sh m-walls`) | Result |
|---|---|
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-bim-model --lib` | exit 0 |
| `cargo test ... --lib -- set_wall_axis ... rehost_opening wall_geometry` | 317 passed, 0 failed |
| same with `env BIM_BLESS=1` (bless) | 307 passed; then `bun r4-m-walls-expected.ts` green |
| `cargo test ... --lib` (unfiltered, 10:28) | 3014 passed, 103 failed: all failures are other agents' in-progress work (editor `unit_tests`, outliner, viewer, ifc export/import, element-solids, spaces, and the leaves `create/delete/set` of beam, beam-type, column, window-type, door-type); aggregate and my 11 modules: 0 failed |
| `cargo check ... --lib --target wasm32-wasip2` | exit 0 (`Finished`, 10:30) |
| `bun T/r3-f1-gen-mutation-facets.ts` | `mutation facets: 88 leaves` |
| `bun T/r3-f1-check-names.ts` | no duplicate or bad name under `🧬️mutations` for my directories; the remaining reports (inference folders, ifc export, test dirs, grammar file, Cargo files) belong to other agents |

## 7. Decisions, traps and open issues

1. Emoji: the first picks (✂️ 📐️ 🔲️ 🚧️ 🧭️ 🧨️ 📌️ 🪓️) collided within minutes with peers' leaves; the final set uses animals (🦈 🦖 🦚 🦉) and unusual symbols and was re-checked clean at the end.
2. The shared workspace did not build for about three hours because of upstream breakage (stdio crates after the `MutationDiff::apply` signature change, icon catalog, unapproved verb `place`, editor mounts); I only polled through the gate and changed nothing outside my slice.
3. `delete-storey` (foundation) still refuses curtain walls as unrestorable; now that `create-curtain-wall` exists it should cascade them (`CreateCurtainWall` in its inverse) and `x-semio-inverse-rows` already allows 4096. Left for the coordinator, not in my slice.
4. Gaps no kind covers yet: wall `phase`, wall `storey` and curtain wall `storey` have no setter (the catalogue has `rename-element` for names, `set-element-property` for data); `split-wall` does not copy property sets or classifications of the wall to the new wall (orphan-tolerant by design).
5. `set-wall-axis` and `split-wall` never touch hosted openings except as specified; an opening whose offset ends up beyond a shortened wall is a diagnostic (`⚠️diagnostics`), not a refusal.
6. Scratch under `T/🗑️generated/m-walls` was deleted (including a private workspace whose junction was removed with a non-recursive delete first).
