# r11 execution report: `r11-w05-modify` (WP-05 modify toolset)

`T` = ticket folder, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `E` = `S/✏️editor`, `M` = `S/🧬️schema/🧬️mutations`.

STATUS: WRITTEN, GENERATORS AND PYTHON ORACLES RUN, RUST BUILD AND TESTS NOT RUN. Every gate build of `semio-s-artifact-bim-model` since the restart stops in the framework
(`semio-framework-os-kernel`, then `-plugin`, `-artifact-infinite-dag`, ... on the owner's half-finished retirement refactor of commit 677, owned by `r11-store`; last log
`T/🗑️generated/r11-w05-modify/check13.txt`). Nothing below has been compiled; section 6 lists exactly what must be run once `r11-exec-store.md` exists.

## 1. Audit findings that turned out wrong or deeper than the audit said

- Previews were NOT missing: `Tool::preview` -> `ToolSession` -> `BimWindowTransient.preview` -> plan overlay / 3D `engagement_preview_json` already carried the marks of every modify gesture. What was missing:
  the window-transient schema facets (`preview` was absent from the json/ts/proto/graphql), hover previews before the first click, and tests per modify gesture.
- The nine leaves had never been blessed: the `after` and `diff` fixtures of all 31 applied cases were the placeholder `{}` (`r11-w05-modify-oracle-check.py` reported 32 problems, all "whole collection" operations).
  They need `BIM_BLESS=1` (section 6); the leaf code and the kernel oracle (`🧪️tests/🧙️modify-bim-1/🐍️.py check`: "ok: 30 cases agree with numpy 2.5.0 and shapely") are green on their own.
- Hotkey collisions: mirror and ceiling both `i`; offset and the global wall flip both `f`; ramp and tag both `shift+t`. `bindings` was only tested over the utility rows.
- `delete-elements`, `delete-site/-building/-storey` declared `x-semio-inverse-rows bounded 65536`; the store admits a gesture at most 65 536 staged rows and a leaf costs `bound + 1`, so none of them could ever be committed
  by the retained route. The Delete key also fanned out into one per-kind `delete-*` (4096 declared rows each), so ~16 selected walls were already refused as `mutation.too-large`.
- Name-check: `offset-wall` shared its emoji with `delete-beam-type`, `set-wall-end-join` with the `🔗️.graphql` facet, `split-slab` had two `🕳️` cases.

## 2. Changes

Delete key (r9 §4.4):
- `M/🌊️cascade/🦀️.rs`: `INVERSE_ROWS = 8191`, `Removal::rows(base)` and `Removal::ids()`; the shared `outcome` refuses a removal that restores more rows with `mutation.inverse-refused`, `inverse` yields none for it. The bound is a promise the diff keeps.
- Schemas `bounded 65536 -> 8191` for `delete-elements`, `delete-site`, `delete-building`, `delete-storey` (`M/*/🧬️schema/🔣️.json`, and the generator inputs `r3-m-multi-data-leaves.ts`, `r4-m-multi-data-cascades.ts`); `M/🧪️tests/🔬️unit/🦀️.rs` expects 8191.
- `mirror-elements`: `bounded 65536 -> fixed 1 + perTarget ids 2` (its inverse is one `place-elements` plus two `set-wall-end-join` per wall), `r10-w05-leaves.ts` updated.
- `M/🧙️modify/👯️copies/🦀️.rs`: `MAX_CREATED = INVERSE_ROWS`, counted in records plus data rows, so that undoing any copy, mirror copy or array is one admissible `delete-elements`.
- `E/🎮️commands/🗑️delete-selection/🦀️.rs` rewritten: element ids leave through `delete-elements` in parts that restore at most `INVERSE_ROWS` rows (halving a part until it fits); library entries and kinds outside the cascade keep their `delete-*`;
  `store::ArtifactStoreOneItemFootprint::for_gesture` decides what fits one gesture; the parts that do not fit stay selected (streamed over successive presses); a container larger than one removal is deleted contents first and stays selected.
  Tests: 20 000 columns -> bounded parts, every inverse <= 8191 rows and the gesture admissible; 70 000 columns -> first press leaves a remainder, second press finishes; a 20 000-column storey; one removal above the bound is refused with no inverse.

Previews and hover:
- `E/🧵️gestures/🚚️transform/🦀️.rs`: `outline` (selection-styled twin of `ghost`, shared `traced`).
- `E/🧵️gestures/👯️duplicate/🦀️.rs`: copy, mirror, array and radial array outline what they will act on (the selection, else the element under the pointer) before the first click.
- `E/🧵️gestures/✂️reshape/🦀️.rs`: offset and trim/extend highlight the wall under the pointer before the first press, align highlights the element it would select, split highlights a slab it would hold; selection outlines use `outline`.
- `E/🫧️transient/🧬️schema/{🔣️.json,🟦️.ts,🛰️.proto,🔗️.graphql}` and `r4-u-editor-gen-facets.py`: the `preview` field.
- Tests: `E/🧵️gestures/🧪️tests/🔬️unit/🦀️.rs` `every_modify_gesture_previews_live_...` (10 scripted gestures: something shows at every step, the document is untouched until the closing click, the text round-trips through the transient, the plan overlay and the 3D `preview_items` paint it);
  `E/🧵️gestures/🧪️tests/🧷️app/🦀️.rs` `an_offset_previews_in_the_window_transient_...` (mounted app: transient-only write, document bytes unchanged, "1.50 m" painted, one undoable row).

Hotkeys:
- mirror `i -> shift+k`; wall flip `f -> shift+f` (`COMMAND_KEYBINDINGS`); tag `shift+t -> shift+g` (ramp keeps `shift+t`); describe texts (en, de) updated.
- `E/🦀️.rs`: `COMMAND_KEYBINDINGS` and `all_keybindings()` (commands, utilities, gesture keys) feed the manifest.
- `E/🧪️tests/⌨️completeness/🦀️.rs`: no key bound twice across all three sources; all nine modify utilities have key + arm command + modify group + plan and 3D window + en/de label + manifest entry, and the flip does not take offset's key.
  `E/🧵️gestures/🧪️tests/🔬️unit/🦀️.rs` hotkey table updated.

Leaves, features, oracles:
- Sum law read, not grepped: `assert_mutation_inverse_sum_law` (`📡️spr/🧪️tests/⚖️protocol-laws`) checks that the inverse is non-empty when the state changed, that the sequential replay restores the base, that the summed inverse diffs restore it,
  and that their canonical sum equals `forward.inverse(base)`. Applied cases / cases carrying the test: copy 3/3, mirror 5/5, array 3/3, align 3/3, offset 3/3, trim-extend 4/4, split-slab 4/4, split-beam 2/2, set-wall-end-join 4/4, delete-elements 3/3; every leaf has >= 2 rejected cases.
- New `M/🧪️tests/🔬️unit/🦀️.rs` test: the inverse of every committed applied case of the nine kinds and of the delete leaves has between 1 and the declared number of rows.
- `S/🧪️tests/🧙️modify-bim-1/🥒️.feature` (new, tags `@capability-bim-1-modify @oracle-bim-1-shapely-geometry`): maps, loops, offsets, trims, splits, arrays, joins against the shapely/numpy oracle `🐍️.py` and `🧫️fixtures/🧙️modify/🔣️.json`; plus a gesture scenario (live preview, one write on the closing click) and a hotkey scenario.
- Renames to satisfy `r3-f1-check-names.ts`: `🪢️offset-wall -> 🧶️`, `🔗️set-wall-end-join -> 🧷️`, case `🕳️sends-a-hole-with-its-piece -> 🧱️` (leaf and fixture dirs, test `include_str!` paths, artifact-root mounts, oracle manifest, `r10-w05-leaves.ts`).
  Longest path of my area: 207 characters.

Examples:
- House: the attic north and west walls are the mirror images of the south and east walls: ids `w-a-north-1-0`, `w-a-west-1-0` (what `mirror-elements` mints), `r4-x-examples-gen.ts` writes `🖼️assets/🏡️house/🧬️derivations.json`
  (two `mirrorElements` + `renameElement` recipes); `examples::checks::replay_derived` / `assert_replay_derived` replay them as the toolset's own mutations instead of `create-wall`; house test `..._mirror_images_...`.
- Office: test `a_column_row_is_the_array_of_its_first_column_and_a_beam_the_copy_of_its_neighbour` (array of `c-0-A1` equals `c-0-B1..F1`, copy of `bm-0-x-A1` by (0, 6) equals `bm-0-x-A2`). The office ids are not changed: its tags, properties and schedules refer to them.

## 3. Commands and results

| Command | Result |
|---|---|
| `.venv/Scripts/python.exe 🧪️tests/🧙️modify-bim-1/🐍️.py check 🧫️fixtures/🧙️modify/🔣️.json` | ok: 30 cases agree with numpy and shapely |
| `bun r3-f1-gen-mutation-facets.ts` / `r3-f1-gen-oracle.ts` / `r3-f1-gen-feature.ts` | 136 leaves / 136 kinds, 955 scenarios, already current / feature + adapter written for 136 kinds |
| `bun r3-f1-check-names.ts` | my three findings gone; 5 remain in other areas (sqlite schema, `__pycache__`, ceilings-meshes, delete-beam-type vs ... ) |
| `r11-w05-modify-oracle-check.py` (jsonpatch + deepdiff + jsonschema over the modify and delete kinds) | 95 quintets, 32 problems = the unblessed `{}` after/diff files |
| `cargo check ... --lib` (gate) | blocked in the framework, see status |

## 4. Open items

- Run section 6.
- `r3-f1-check-names.ts` still reports four findings outside this package (sqlite `🗄️`, `🔲️` ceilings fixtures, `__pycache__`).
- Only `delete-wall`-like leaves still declare 4096 rows while the shared cascade guard allows 8191; harmless (a wall's cascade is its openings).
- A selection of more than ~64k rows is deleted over several presses by design (the store admits 65 536 staged rows per gesture); a resumable job would remove that, not needed for any model in the repository.

## 6. To run once the framework builds (in this order)

1. `"$T/🚦️gate.sh" r11-w05-modify -- cargo check --manifest-path ✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.toml -p semio-s-artifact-bim-model --lib --tests` and the same with `--target wasm32-wasip2 --lib`.
2. `bun r4-x-examples-gen.ts` then `BIM_BLESS=1 cargo test ... bless_the_` (house and office DSL texts follow the new house ids).
3. `BIM_BLESS=1 cargo test ... -p semio-s-artifact-bim-model --lib mutations::` filtered to the nine kinds to write the 31 `after`/`diff` fixtures; review them by hand; run `r11-w05-modify-oracle-check.py` and the full `mutate-model-1-any/🐍️.py check`.
4. `cargo test ... --lib` (exact counts), then re-run `bun r3-f1-gen-*.ts` and `r3-f1-check-names.ts`.
