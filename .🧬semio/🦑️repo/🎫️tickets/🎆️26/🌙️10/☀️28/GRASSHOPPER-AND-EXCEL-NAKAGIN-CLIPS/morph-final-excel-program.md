# Final morph review: excel / 📋️program / `Beat3_Program`

Read: scene file, storyboard, done file, plan lessons, frames en (hook, area, sumproduct, swap mid frames; `index.md`). The en/de counts differ (36/35/33/36 vs 35/33/35/37) because the Nakagin model changed between captures; the code reads every count from `self.vals`/`_nk.slots()`, so this is stale frames, not a code defect. The hook en frame still shows a violet-dominant painted tower and a truncated chip row (stale or mid-gather).

## Per clause

| Clause | Rating | Why |
| --- | --- | --- |
| hook | GOOD | Real type-coloured capsule copies of each type gather into the four count chips (`collect`, swarm 2.4 s): building to number, colour carried. The "? m²" chip is still a static label. |
| program | WEAK | Plain `stage` fade-in of plan, hatches, person and heads; the storyboard morph (fitting rectangles and person glyph into the "fittings / areas / users" tokens) is not implemented. Idea "program = fittings, areas, users" is explained by labels only. |
| sheet | GOOD | Sheet row-label pills (`token(kind="cell")`) fly onto their fitting rectangles on the plan (`fly`, A to C): the cell text becomes the thing. |
| area | GOOD | Rectangles fly into G12:G14 as value pills, then the cells recalculate, and the hatches are in place. Frame area-04 shows the pills in flight. Hatches themselves are faded in with the window swap rather than arriving from the value (lesson 3, minor). |
| net | WEAK | Floor hatch and the two dimension labels fade in beside a `recalculate`; the dimension labels do not travel into B9 and the product does not return as the hatch. The core "width times length" idea is a still. |
| free | WEAK | Fitting hatches, floor tint and label fade in; no gather of hatches into the minus or B16, no morph of the B16 value into the free tint. Only the person-occupant frame and reference chips explain. |
| volume | GOOD | `capsule_axon` box morphs part-wise (`morph`, `PartMorph` in index) into the B10 value pill: an interior box becomes a cell. Pop card (shown vs stored) is static, acceptable. |
| types | ACCEPTABLE | Four plans with hatches and per-type tags fade in; no morph from the tabs or from the Type A plan to the others. Structurally clear, identity not shown. |
| table | GOOD | Ten real capsules scatter with `capsule_pairs` to type-coloured row dots at the rows, the remaining 130 gather into "140 rows"; old panel fades first, then the tower appears (index shows Swarm x2). Capsule to row is the owner's example. Table dressing (tint, bands, marquee) is a fade, fine. |
| row | GOOD | Row pill (type text) morphs back into the real lit capsule with reversed `capsule_pairs`: A to C, back-link of table. The row dots persist next to the list. |
| countif | GOOD | All type-A capsules gather into the counter (collect then adopt the live label), the same tint on the tower via `repaint`. Counter and B4 are linked only by the colour; the counter does not fly into B4 (storyboard had that), acceptable. |
| rest | ACCEPTABLE | The other types' capsules gather into the total counter, but cells B5:B7 only recalculate and the 35 to 140 count is a `ValueTracker` sweep (1.6 s) plus a repaint sweep; four colours cover the tower by painting, not by per-type gathers (open item in the done file, lesson 4). |
| countifs | GOOD | Item pill from B11 splits into the lit type-A capsules of tower A (`morph` per seed with reversed pairs, swarm): cell value to building, the lit predicate visible. Tower B is simply not lit, fine. |
| pull | GOOD | Free-floor texts of the four plans fly into C21:C24 and the cells compute after landing; `xl_trace` arrows show provenance. Value text, not the hatch, flies (A to C reversed from the storyboard) but it is a true identity transfer. |
| xlookup | GOOD | C27 pill morphs into the floor tint of plan C, with the count and plan shown (A to C, back-link of pull). The key-frame glide over A21:A24 from the storyboard is absent but the cell frame is highlighted. |
| sumproduct | GOOD | Each count pill becomes one tile per capsule in its type colour (strips of 35, 33, 35, 37 visible in frame sumproduct-05: comet trail of one fly-in); tiles carry "n x free = product" text. Real discrete-to-many morph. |
| chart | GOOD | Tile strips gather into per-type bar bases, then the bars grow by trackers from D21:D24 (continuous quantity, acceptable by lesson 4). Chain count pill to tiles to bar is intact. |
| towers | GOOD | Tower A and tower B capsules gather into F11 and F12 tokens, and the F13 total pill morphs into a green glow over the whole tower (`proxy`): both directions plus back-link. |
| swap | ACCEPTABLE | Tower capsule lifts out (empty slot) and the new type A capsule drops in and recolours: an event shown as an event. The cell text "Type C" to "Type A" does not retext into the capsule, so cell and capsule are joined only by the chips. Frame swap-06 shows only the list and chips at mid, tower recolour is small at 640 px. |
| update | ACCEPTABLE | +1 / -1 delta chips fly from B21 / B23 to their places and bars `UpdateFromAlphaFunc` to new values; the chips start in the cells, not at the swapped capsule, so the capsule to count link is not a morph. Total chip is a fade. |
| check | WEAK | `recalculate` and a TRUE badge plus "140 = 140" chip; no morph of capsules into B17 or of the two sums into TRUE. A pure state change. |
| typo | GOOD | The red space dot at the cell edge morphs into a ring around the grey capsule (`morph`, path arc): the invisible character becomes the faulty capsule. |
| fix | GOOD | The ring gathers into the TRUE badge (C to A) and the capsule recolours orange; typo to fix is one chain. The drop-down list sentence has no visual. |
| recap | ACCEPTABLE | All 140 capsules gather into the "140 capsules" label and the vocabulary rows appear by fade; no row morphs into the result it produced (tile row, hatch, glow), and the chips are text. The closing sentence is carried by the 140 gather only. |

## Overall verdict: SHIP (with minor polish)

The video is morph-driven where it matters most: the owner's own examples (capsule to table row to lit capsule; count to capsules; free-floor text to cell and back to plan; count pill to tiles to bar; mark to ring to badge) are real identity transfers with type colour carried all the way, both directions and several back-links. The remaining plain-fade beats (program, net, free, check, types) are the early "what is a program / free floor" explanation, where the teaching idea is geometric arithmetic rather than counting, so the gap is noticeable but not film-breaking. If time allows, the five highest-impact polish items, in order:

1. `act_type_sheet` `net` (lines 437-446): fly the two dimension labels `d1`, `d2` (as tokens) into B9 and let the B9 pill morph into `floor_hatch` on arrival (`self.fly` then `_morph.morph(pill, plan.floor.copy())`, post queue), as in `area`.
2. `act_type_sheet` `free` (lines 447-459): gather the fitting hatches (`fit_all` parts) into a minus token at B16 (`self.collect`), then the B16 value pill morphs into `floor_fill` plus `free_label`; this is the central subtraction idea.
3. `act_type_sheet` `program` (lines 411-415): fly the three fitting rectangles and `plan.people` into three item tokens under the "fittings / areas / users" heads (`self.fly`, pre queue) before the heads settle.
4. `act_edit` `check` (lines 704-711): gather `eq` operands (capsules for B17 and the B8 sum token) into the badge `ok` with `self.collect`, so TRUE is produced by the two counts; and `act_edit` `update` (lines 700-701): start the delta chips at `capsule_center(key)` instead of the cells (reverse capsule to count link).
5. `act_counts` `rest` (lines 536-544): replace the 35 to 140 `UpdateFromAlphaFunc` sweep by per-type `self.collect` into B5, B6, B7 and B8 pills (0.7 s each, lesson 4); and in `act_recap` (lines 743-763) morph each vocabulary chip from the matching earlier object (counts chip to B4 colour dots, XLOOKUP to plan C tint, SUMPRODUCT to tile row, SUMIF to glow) instead of fading rows.

Frames to refresh: all en/de frames predate the final tower counts; re-run `capture_morph.py excel 📋️program Beat3_Program en` and `de`, and check hook (violet-heavy tower, chip row near the foot) once more at 1280x720.

## Summary (5 lines)

1. Verdict SHIP: capsule/table/cell/tile/bar/glow chains are genuine two-way identity morphs in type colour, with back-links (row, countifs, xlookup, towers glow, typo, fix).
2. 15 clauses GOOD (hook, sheet, area, volume, table, row, countif, countifs, pull, xlookup, sumproduct, chart, towers, typo, fix), 5 ACCEPTABLE (types, rest, swap, update, recap), 4 WEAK (program, net, free, check).
3. The weak ones are fade-ins of the early arithmetic (net, free, program) and the check badge: no hatch or dimension travels into B9 or B16, nothing flows into TRUE.
4. Polish if wanted: net and free morphs in `act_type_sheet`, check/update capsule links in `act_edit`, per-type gathers in `rest`, recap chips morphing into earlier results.
5. Frames are stale versus the changed Nakagin model (en 36/35/33/36, de 35/33/35/37); the code reads counts from the engine, so it is correct; re-capture before final review.
