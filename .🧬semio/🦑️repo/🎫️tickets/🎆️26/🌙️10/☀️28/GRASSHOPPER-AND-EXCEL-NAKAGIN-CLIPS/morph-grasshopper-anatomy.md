# Morph Storyboard: grasshopper_anatomy (`Beat1_Anatomy`)

File: `🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py`. Plan: `morph-plan.md`. Status: Phase A (storyboard) done. Phase B (implementation) waits for `api.md` section 7 (`Status: ready`); on disk there is no `🔀️morph` module and no section 7 yet.

## Ground rules for this clip

- Clauses stay at 24 (hook ... legend_data); keys, order and the engine/model numbers stay. Narration texts keep their facts; only the sentences listed under "narration" below get a short morph-aware tail where a caption has room (two lines max, en and de together).
- Numbers come from the model, not from the strings: the model changed (Tower A 76 capsules in 41 branches and 13 storeys, Tower B 64 capsules in 33 branches and 11 storeys, 140 slots in 74 branches). The clauses `series`, `count`, `mistake`, `legend_*` currently say "nine", "eleven", "27 branches"; those texts and the numbers drawn in `present_anatomy` must be re-derived from `_nk.slots()` / `_nk.tree()` / `_gh.aggregate_tree()` at implementation time (the scene already derives `tower_branches`; the captions are the open item and belong to the codex session that edits the transcripts).
- Tokens keep the capsule type colour (`_nk.type_color`) or the dimension colour (width yellow `#C28F00`, length teal `#0B8F86`, height green `#3E9B45`); heavy groups morph from `proxy()` or one `silhouette()` per capsule.
- A thin tether (dashed leader, `tether`) keeps the concrete origin visible while the abstract form is used.

## Morph beats (14)

Direction: C>A = concrete to abstract, A>C = abstract to concrete. Durations 1.0 to 2.0 s inside the clause time.

| # | clause | concrete object | abstract object | dir | carries over | dur | narration (while it happens) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | hook > script | the one lit capsule on the tower in the viewport (`silhouette()` of capsule the second item of the chosen branch) | the file chip `Nakagin_Capsule.gh` on the canvas sheet | C>A | outline colour, the "1 of 140" count: the capsule shrinks into the chip, the chip keeps a tiny capsule glyph | 1.4 | "We describe it as a script: a Grasshopper definition, saved as a .gh file." |
| 2 | slider | the width dimension line on the capsule (yellow, label 2.5 m) | the Width slider knob and its value token `2.5` | A>C | width yellow, value 2.5, domain 1.0 to 5.0: the knob moves from 1.0 to 2.5 and the token flies out of the knob onto the dimension label; the dimension line grows from its start length to 2.5 m with the knob | 1.4 | "A Number Slider is the first input: a name, a domain from 1.0 to 5.0, one decimal and a value, 2.5." |
| 3 | box | the three dimension lines of the capsule (width yellow, length teal, height green) | the three wires into X, Y, Z of Center Box | C>A | colour, order W/L/H: each dimension line detaches, bends into a bezier and settles as the wire between slider grip and input grip (`morph` with path_arc, LaggedStart 0.15) | 1.6 | "A component turns inputs into an output. Center Box takes width, length and height and builds a box." |
| 3b | box | the Box output (the output grip token on the right of the component) | the box drawn in the viewport | A>C | box dimensions 2.5 x 4.0 x 2.5 from the engine: a small cube token leaves the output grip along a tether and grows into the live viewport box | 1.0 | "...and builds a box; the viewport shows it." |
| 4 | flow | the number pulse dots on the three wires | the box edges: width, length, height | A>C | the three pulse dots keep their colour, arrive at X, Y, Z as number tokens and the box edge of the same colour grows (scale from the corner) | 2.4 | "Wires carry data from an output grip to an input grip and normally flow from left to right." |
| 5 | sliders | the box edges length (teal) and height (green) | the slider knobs of Length and Height | A>C (live, tethered) | colour, values 5.0 and 3.0 m: dashed tethers join knob and edge; when the knobs move the edges stretch together, then the pair returns | 2.0 + 1.0 | "Each slider is one dimension: wire and dimension line share its colour. Length 5.0 and height 3.0 metres make the box grow." |
| 6 | preview | the box in the viewport (volume 25 m3) | the Panel row `25 m³` under the Volume component | C>A | the box collapses into its row token and leaves an empty viewport; the eye is struck through at the same time | 1.2 | "The preview eye hides only the drawing, not the data: the box disappears, the volume still reads 25 cubic metres." |
| 7 | panel | the width dimension label `2.5` on the capsule | the Panel rows `{0}` / `0` / `2.5` | C>A | width yellow, value 2.5: label splits into the three rows (path header, index, value) with `split()` | 1.2 | "A Panel shows what a wire carries: path {0}, index 0, value 2.5." |
| 8 | toggle | the True knob of the Boolean Toggle | the round window (Circle, 1.3 m) on the front face of the capsule | A>C | True/green fill, 1.3 m: the knob lifts off, flies to the front face and expands to the window circle with the amber diameter chip | 1.4 | "True gives the capsule its round window, 1.3 metres across, in the front face." |
| 9 | valuelist | the selected Value List row (`Type A`, `B`, `D`, `C`, each one in its type colour) | the capsule silhouette of that type in the viewport (shape, window diameter, window offset) | A>C | type colour from `_nk.type_color`, window Ø per type (1.3, 1.3, 1.6, 1.3 with the splay of D): the row token morphs through each type in turn; the silhouette is a `Transform` between the four silhouettes | 2.4 | "A Value List offers fixed choices: the capsule types A to D. Pick one and the capsule takes that type's window." |
| 10 | plugin | the window circle of the capsule | the Ladybug `Window` Brep parameter hexagon and its sun-colour bar, then the tint back on the window | C>A then A>C | window ring: circle > hexagon on the canvas > the bar value > amber tint disc on the window (qualitative: no sun-hour numbers) | 1.6 | "A plugin, a .gha file, adds a tab to the ribbon. Ladybug Tools studies the sun on the capsule window: more sun, warmer tint." |
| 11 | item > series | the first-storey capsule on Tower A (one capsule, lit) > the nine (generally `count`) level capsules of Tower B | the Panel row `0  0.0 m` > the Series rows `0 ... 22.0 m` in `{0}` | C>A then many>one list | type colour of each capsule, height value per row: item = one capsule becomes one row token (`morph`); series = the `count` lit capsules fly into the rows in storey order (`scatter`, lag 0.05), the list panel grows row by row with them | 1.2 + 2.0 | "Now the data. One value is an item: the height 0 m of the first storey, one capsule on Tower A." / "A Series ... makes a list: ... indices 0 to 8, one capsule on each of ... storeys." |
| 12 | count | two extra Series rows (Count 9 > 11 on the slider) | two additional capsules growing onto Tower B (storeys 9 and 10) | A>C | row text, type colour: each new row token leaves its panel row and lands as a capsule on the tower, the list grows by two while the tower grows by two (`split`) | 1.6 | "Change Count from 9 to 11 and the list grows by two: Tower B has eleven capsule storeys." |
| 13 | tree | the two lit capsules of facade 1 on storey 3 (item 0 and 1, Tower A), then storey band, facade arrow, tower | the two leaf tokens in the branch `{0;0;0;3}`; the path chip built segment by segment `0` > `0` > `3` > `1` | C>A, chained | type colour of each capsule on its leaf, path numbers: each segment chip is born from the object it names (building 0 from the tower silhouette, tower 0 from the tower, storey 3 from the storey band, facade 1 from the facade arrow); the chips join into `{0;0;0;3}` above the leaves | 1.8 | "Many lists form a data tree. Each facade of a storey is one branch with its own path, here {0;0;0;3}, holding two capsule items." |
| 14 | mistake | the index token `[1]` | one `[1]` token per branch (35 of the 41 branches of Tower A carry an index 1; taken from the model) flying onto the capsule of item 1 of that branch on the tower | A>C (one>many) | the index chip duplicates (`split`, lag 0.02) and each copy lands on its capsule, lighting it; the lit count gathers to the red tag "[1] fits 41 branches" | 1.6 | "A common mistake is to read a tree as one long list. The index 1 alone fits all branches of Tower A." |
| 15 | address | the many lit capsules | one path chip + one index chip on the one capsule `{0;0;0;3} [1]` | many>one, A>C | the [1] copies collapse (`gather`, lag 0.02) into the single token of branch `{0;0;0;3}`; path chip and index chip dock onto the one lit capsule; the Panel row 5 lights in the same green | 1.4 | "Only path plus index names exactly one capsule: path {0;0;0;3}, index 1. The Panel shows both." |
| 16 | legend_tool | each numbered marker on the mini canvas (script chip, Center Box, floating param, port, slider, Panel, Ladybug tile) | the legend row with the same number | C>A | marker number and position order: the marker discs fly into their table row and become its number disc (`scatter`, lag 0.15) | 2.4 | "The canvas terms, row by row ... " |
| 17 | legend_data | the Panel rows/brackets of the tree panel (item, list, tree, branch, path, index) | the legend rows of the data terms | C>A | bracket marker numbers, row text: the six brackets fly to their term rows | 2.0 | "The data terms ..." |

(Seventeen rows because beats 3b, 11, 15 are the second halves of a chain; the morph-beat count the plan asks for, 8 to 14, is met by the 14 clause-level beats 1 to 15 without 3b/legend merging: beats 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 are the film's morphs; 16 and 17 restate the legend sweep and may be implemented as light versions.)

## Direction count

- Concrete > abstract: 1 (capsule > .gh chip), 3 (dimension lines > wires), 6 (box > volume row), 7 (label > panel rows), 10a (window > Ladybug parameter), 11 (capsule > item > list rows), 13 (capsules > leaves > path chip), 16, 17: nine.
- Abstract > concrete: 2 (knob > dimension), 3b (Box output > viewport box), 4 (pulses > box edges), 5 (knobs > edges), 8 (toggle > window), 9 (Value List row > capsule type), 10b (bar > tint), 12 (rows > capsules), 14 (index > 41 lit capsules), 15 (token > one capsule): ten.

## The chain (one capsule through the whole data half)

Capsule the second item of the chosen branch lit on the tower (`item` shows the first storey capsule) > its row in the Series list (`item`/`series`) > leaf in branch `{0;0;0;3}` (`tree`) > path chip segments, each born from a building part (`tree`) > index chip `[1]` copied to all branches (`mistake`) > collapses back to this one capsule with the path and index chips (`address`) > its numbered marker in the legend (`legend_data`). The same silhouette colour (type of this slot) and the same path/index text stay on screen across all of them.

## Replaced fade-swaps (what the implementation removes)

- `script`: `FadeOut(whole)` + `FadeIn(preview)` > beat 1 (silhouette morph + chip).
- `box`: `FadeOut(notes)` / `FadeIn` of component and wires > beats 3/3b.
- `preview`: eye strike alone > beat 6 collapses the box into the Panel row.
- `panel`/`params`: fade of panel > beat 7 (label splits) and a morph of the panel row to the floating Number parameter and to the port X ring.
- `toggle`/`valuelist`/`plugin`: fade swaps > beats 8, 9, 10.
- `item`/`series`/`count`/`tree`/`mistake`/`address`: fade-ins of list panel and tree panel > beats 11 to 15.
- `legend_tool`/`legend_data`: staggered fade-in of rows > beats 16/17.

## Performance and risks

- Tower groups come from `tower_axon` (thousands of faces): use `proxy()` for the whole tower and `silhouette()` per capsule from `drawing.capsules[key]` only for the lit ones (at most 41 in `mistake`).
- Live drawings are rebuilt by `self.live`; a morph must run on a static snapshot of the source and hand over to the live object at the end (keep the live object hidden until then).
- The data-half counts (9 and 11 storeys, 27 branches) disagree with the model on disk (13 and 11 storeys; 41 and 33 branches); verify when Phase B starts and tell the codex session.

## Update (audit round)

Values now follow the model at run time: the chosen branch is the first Tower A branch with two items (path {0;0;0;3}), `[1]` exists in 35 of the 41 Tower A branches, Series rows are Tower B (nine levels) and then Tower A (eleven levels). Preview target is the Volume Panel row; the tower capital is not drawn in the data clauses so no wedge covers the tower top.
