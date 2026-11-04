# EN 1990 Outliner Manifest

Edited the hub descriptors: no.

The EN 1990 editor registers `norm-en1990-outliner`. Both committed descriptor twins omit it. A generator owns the pair, so they were left unchanged.

## Editor

`create_en1990_app` in `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` registers window kinds in this order, then sets the default layout from `edit_mode::layout()`:

1. `outliner::definition()` → `norm-en1990-outliner`
2. `properties::definition()` → `norm-en1990-properties`
3. `viewport::definition()` → `norm-en1990-viewport`
4. `diagrams::definition()` → `norm-en1990-diagrams`
5. `checks::definition()` → `norm-en1990-checks`

`🌲️outliner/🦀️.rs` defines the outliner as `window_definition(WINDOW_OUTLINER, LocalizedLabel::native("Outliner", "Gliederung"), BODY_OUTLINER, "list-tree")` with body key `norm.en1990.play.outliner`.

`edit_mode::layout()` is the desktop layout at default preferences. It calls `visual_layout_with("en1990", …)`, whose role list is exactly:

`outliner`, `viewport`, `properties`, `checks`, `diagrams`

Desktop placement (`visual_layout_roles`, default dock `left 0.22`, `centre 0.52`, `right 0.26`, splits `0.5`): left column outliner then properties, centre viewport titled Model, right column checks then diagrams. The unit test `the_default_layout_lists_all_five_window_kinds` requires all five ids in that layout.

## Committed twins

`🌎️hub/🧩️compositions/📕️norm/🧩️component/🔣️.json` app `s.norm.en1990@1/*#editor`:

- `windowKinds`: `norm-en1990-properties`, `norm-en1990-viewport`, `norm-en1990-diagrams`, `norm-en1990-checks`
- `defaultLayout`: row of properties (size 22), viewport (size 50), then a column of checks and diagrams (size 28)
- `norm-en1990-outliner` count: 0
- `norm-en1990-inputs` / `norm-en1990-results` count: 0

`🌎️hub/🧩️compositions/📕️norm/🧩️component/🛂️.descriptor.semio` (504782 bytes) has no `outliner` bytes. Its length-prefixed `norm-en1990-*` ids are only `norm-en1990-inputs` and `norm-en1990-results`. The readable JSON and the pack already disagree. The JSON has the four visual windows; the pack still has the old inputs/results pair.

## Why not hand-edited

`write_descriptor_pair_atomic` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs` writes `🛂️.descriptor.semio` and `🔣️.json` as one pair from `describe()` of the built component. `components describe` is that write (`describe --write`) onto the committed pair. A hand edit of the JSON would be overwritten, would not update the pack, and would not match the canonical layout sizes or the actions the app builder adds on top of `window_definition`.

## Command

```
repo run workspace:components describe norm
```

Plugin id is `norm` (`packageId` `semio:norm`). That rebuilds the component and rewrites both twins from `describe()`, including `norm-en1990-outliner` and the five-role default layout.
