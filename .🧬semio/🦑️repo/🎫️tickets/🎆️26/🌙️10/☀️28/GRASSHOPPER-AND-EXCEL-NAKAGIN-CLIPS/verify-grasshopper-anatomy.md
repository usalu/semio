# Verification: grasshopper_anatomy (`Beat1_Anatomy`)

Scope: fresh keyframes from `capture.py` for `🦗️grasshopper` / `🦴️anatomy` / `Beat1_Anatomy`, en and de, 24 beats each (`🗑️generated/frames/🦗️grasshopper/Beat1_Anatomy/{en,de}/`). Audit read: `audit-grasshopper-anatomy.md`. Model read: `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json` and `🧪️cases/🔣️.json`.

Capture notes: the first en run wrote all frames but exited 1 on a cp1252 print error (`[DEBUG] captured` line with an emoji path). The re-run with `PYTHONIOENCODING=utf-8` exited 0 for en and for de. Frames and `layout.md` are from the clean runs. Strict layout counts: en 35 findings in 13 of 24 frames, de 36 in 13 of 24 frames. These were not dispositioned one by one in this pass.

## Verdict

NOT READY. The fix round closed almost every audit item in both languages. Two new majors are visible on screen in both languages, and one minor carried over. Fix the two majors and the clip is READY.

## 1. Audit items: status

Audit verdict says two blockers; its table lists one (valuelist de). Both are dispositioned below.

| audit item | lang | status | evidence |
| --- | --- | --- | --- |
| valuelist header overprint (blocker) | de | FIXED | `valuelist.png`: header reads "Perspective" clean; type note "Typ C · Atelier Ø 1,6 m" moved to bottom right |
| valuelist overlap (major) | en | FIXED | `valuelist.png`: "Type C · Studio Ø 1.6 m" bottom right, header clear |
| params floating Width reads 2.3 (major) | en, de | FIXED | `params.png`/`params` de: Panel reads 2.5 / 2,5, wired from Width slider |
| parts body "Center Box" (major) | en, de | FIXED in `parts`, REGRESSED in `box` | `parts.png` body "Box", leader "Name: Center Box". See new major N1 |
| box Y and Z unwired (major) | en, de | FIXED | `box.png`: Length to Y, Height to Z |
| warning caption vs null output (major) | en, de | FIXED | `warning.png`: caption "One box is built, the second slot stays null"; output shows Box, null |
| warning Width/Length English (minor) | de | FIXED | `warning` de: "Breite", "Länge" |
| mistake count not checkable (major) | en, de | FIXED | `mistake.png`: "3 of 27 branches shown" on screen, tag "27 capsules" |
| legend_tool caption, no German word (major) | en | FIXED | "script, Skript; component, Komponente; input, Eingang; output, Ausgang; slider, Schieberegler" |
| legend_tool caption, English only (major) | de | FIXED | "Skript, Komponente, Parameter, Eingang, Ausgang, Schieberegler, Panel und Plugin" |
| legend_data caption, English only (major) | de | FIXED | "Element, Liste, Datenbaum, Ast, Pfad und Index" |
| colour linkage (major) | en, de | FIXED | sliders, box, hook, live, flow: Width gold, Length teal, Height green on wire and dimension |
| legend rows without pings (minor) | en, de | FIXED | numbered badges 1 to 8 (legend_tool) and 1 to 6 (legend_data) pinned to canvas or viewport objects |
| flow: only X pulses (minor) | en, de | FIXED | `flow`: X, Y, Z all pulse |
| preview: no off-state, volume unit (minor) | en, de | FIXED | `preview`: eye crossed out; "25 m³" |
| hook dimension labels on outline (minor) | en, de | FIXED | labels offset |
| toggle chip over window (minor) | en, de | FIXED | chip above capsule, leader to window |
| plugin whole face tinted (minor) | en, de | FIXED | only window area tinted |
| item/series chip at sheet edge (minor) | en, de | FIXED | slider chip inside canvas |
| error "first problem" green (minor) | de | FIXED | red marker and label |
| teaching note on every keyframe (minor) | en, de | FIXED | note only on `hook` |
| address `{0;0;3;1}[1]` notation (minor) | en, de | FIXED | "{0;0;3;1}, index 1: one capsule" |
| script de "Zeichenfläche" (minor) | de | FIXED | "Ihre Arbeitsfläche ist der Canvas." |
| valuelist de idiom (minor) | de | FIXED | "bietet feste Auswahlmöglichkeiten" |
| flow en "always" (minor) | en | FIXED | "normally flow from left to right" |
| legend_data en German column not spoken (minor) | en | FIXED | "item, Element; ..." |
| count "Count" English (minor) | de | NOT FIXED | `count` de: slider label "Count" |
| error engine text in English (minor, by design) | en, de | UNCHANGED | "Input parameter Z failed to collect data" in de frame |
| valuelist caption "window changes with type" (minor) | en, de | FIXED | de: "übernimmt die Kapsel dessen Fenster" |
| strict layout findings dismissed (minor) | en, de | NOT DISPOSITIONED | 35 / 36 findings, see section 3 |

## 2. Model cross-check (all on-screen numbers)

Matches `🏢️nakagin/🔣️.json`:
- Capsule outer 2.5 x 4.0 x 2.5 m (hook, sliders, live). Volume 25 m³ (preview).
- Window Ø 1.3 m (toggle, types A, B, D). Type C window Ø 1.6 m (valuelist).
- Tower A capsule storeys 9 on facades 0, 1, 2 (27 branches, 54 capsules). Tower B capsule storeys 11, 4 facades (43 branches, 86 capsules). The count frame shows 11 storeys; the tree frames show 27 branches for Tower A.
- Series step 2.75 m (repo IFC storey pitch, teaching value). Series list 0 to 22 at count 9, 0 to 27.5 at count 11.

Not an error in this clip: the model's "140 slots" total (54 + 86) and the audit's older 63-vs-54 note are not contradicted by any frame. The clip only shows Tower A branch counts.

## 3. New blockers and majors (fresh frames, en and de)

- **N1 (major, en + de): component label contradicts itself.** `box` shows the Center Box body as "Center Box" (de "Center Box"). `parts` teaches "the canvas shows the nickname", and every other frame (`hook`, `sliders`, `live`, `flow`, `warning`, `error`) shows "Box". Fix: body reads "Box" in `box` too (en and de), keeping "Center Box" only in the name leader.
- **N2 (major, en + de): unexplained bare number.** `valuelist` shows the Value List selecting "Type C" with a Panel reading `2` (de `2`). The caption says the capsule takes that type's window, and the window label shows Ø 1.60 m. Nothing explains that 2 is the index of Type C. Fix: panel shows the selected item (for example "Type C"), or the caption and a small note say "index 2 = Type C".

Minor, fresh:
- **m1 (en, de):** list numbers in `series`, `item`, `count` have no unit, while captions say metres. The slider names carry the object; add "m" to the list or drop "metres" from the caption.
- **m2 (en):** `warning` caption "Give Z a number and a text" reads as two inputs; Z receives the text "2.5 tall". Reword to "Give Z a text that holds a number".
- **m3 (de):** `params` tag "Eingang X · Number" and the Width param label keep "Number"; German Grasshopper says "Zahl". Also `error` keeps "Input parameter Z failed to collect data" in English (already accepted).
- **m4 (de):** `slider` range caption "Bereich 1,0 bis 5,0 · 1 Dezimalstelle" starts flush at the canvas's left edge (en has margin). Indent it.
- **m5 (en, de):** `live` caption says the width goes to 3.0 and back; no 3.0 keyframe exists. Acceptable for a still, but say "see the next frame" or add a state.

No new text overlaps or clipping found by eye in either language beyond the open count (de) and slider caption margin (de).

## 4. Must change before READY

1. N1: body label "Box" in `box`, en and de.
2. N2: show the selected item or index meaning in `valuelist`, en and de.
3. Localize `count` slider label in de ("Anzahl").
4. Optional but recommended: m1 to m4; disposition the 35 / 36 strict findings and record the verdict per finding in `scene-grasshopper-anatomy.md`.
