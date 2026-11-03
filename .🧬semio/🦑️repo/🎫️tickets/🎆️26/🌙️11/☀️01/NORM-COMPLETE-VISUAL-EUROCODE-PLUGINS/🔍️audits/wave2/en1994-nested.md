# EN 1994 nested action kinds (wave 2)

Nothing compiled or run (no cargo/rustc). No git writes. Rust was read against the definitions only.

## Diff lanes (schema first)
- `En1994{Beams,Columns,Slabs}ActionsRows` now carry `removed: [{id, index}]`, `inserted: [{index, row}]`, `modified` (base-position patches). Rust apply (id is verified at its base index, ascending, range, removed-and-modified conflict), inverse and absorb were extended in `🔺️diff/{🏗️beam,🏛️column,🧱️slab}-rows`. A local `KeyOf` marker per row type (`En1994{Beams,Columns,Slabs}ActionKey`) names rows by id.
- Four twins (json, ts, graphql, proto) updated; ts imports `CharacteristicAction`/`ColumnAction`. 20 existing diff vectors null-padded with `removed`/`inserted`.
- Script: `🧰️scripts/en1994-nested/lanes.py` (idempotent).

## Kinds (6)
`insert-/remove-{beam,column,slab}-action` in crates `mut-{beam,column,slab}-action-rows`. Insert payload: `<parent>Id`, `index` (insert position only), `action`. Remove payload: `<parent>Id`, `id` (id only). Diff bodies drive the keyed lane; duplicate id, unknown parent or id and out-of-range index are refused.
- Wired: root enum, KINDS, ts and json unions, binary tags 110..115, edit rules (3 `InsertItemRule`, new `ID_REMOVALS` plus `resolve_with_id_removals` in `resolve_edit`), oracle manifest, taxonomy, mutate harness (py, rs, feature), testkit demo cases, 6 `✅apply` vector sets, Cargo.toml/Cargo.lock/script.ts/build-owners. Script: `🧰️scripts/leaf-kinds/en1994_actions.py` (merge mode, stage `🗑️generated/leaf-kinds/stage-en1994-nested`).
- Coordinator request: `📦️packages/🦀️rust/📜️script.ts` had two `runArtifactRustPackageMain` calls; merged into one with the union of both package lists (includes the three new crates).

## Tests added (not run)
- `🔺️diff/🧪️tests/🔬️unit`: apply and inverse by id for beams, columns and slabs, refusal cases, absorb laws (cancel, fold into inserted row, remove-then-reinsert).
- `🧬️mutations/🧪️tests/🔬️unit`: kind round trip through inverse for all three parents, refusals, edit resolution to the id kinds. The testkit demo cases also feed the existing every-kind and text op round-trip tests.

## Checks
- `repo ./📜️script.ts verify build-owners sync`: passed. `check-cargo-authority.py`: 28 crates, 0 errors.
- SCENE handle bindings: there is no runtime test in the repo for this. Static check: all 27 `beams|columns|slabs[id=..]` write paths of the scene resolve to a `SetFieldRule` template (incl. the 5 column section leaves); the 4 unmatched templates are entity references, not writes.

## Open
- Parent rows and `remove-placement` stay index-addressed in existing kinds. The `mutate` rs `KINDS` still lacks the placement kinds. `insert-placement` ts imports `En1994Placement` although the snapshot exports `Placement`.
- Cargo.lock edited by the generator, `cargo` must synchronise.
