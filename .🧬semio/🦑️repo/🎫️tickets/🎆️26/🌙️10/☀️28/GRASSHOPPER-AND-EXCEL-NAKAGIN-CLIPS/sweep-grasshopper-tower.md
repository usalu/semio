# 🗼️ Final Sweep: Grasshopper Tower (Beat3_Tower)

Scope: series `🦗️grasshopper`, scene `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py`, class `Beat3_Tower`. All 22 clauses read in both languages (44 PNGs), in narration order from `NARRATION`.

Staleness: frames captured 03:37 to 04:10 on 2026-10-29. The scene file was last saved 04:02. No PNG is older than the scene by more than 30 minutes (oldest is 25 minutes before). 40 of 44 PNGs predate the 04:02 scene save, so they may not reflect the latest scene edit.

Authority check: the `nakagin-model.md` report gives Tower A = 9 storeys x 3 facades = 27 branches = 54 capsules, Tower B = 10 x 4 + top storey with 3 facades = 43 branches = 86 capsules, total 140. Types {A:36, B:35, C:34, D:35}. The scene's 54 / 88 / Cull Index -2 / 86 / 140, the two cut planes (indices 6 and 7, top storey west), and the swap result (C = 34 + 35 = 69) all agree with it.

## Verdict: FIX

### Defects

| # | Frame | Lang | Severity | Defect | Exact fix |
|---|-------|------|----------|--------|-----------|
| 1 | `nowire` | EN and DE | Major (wrong number) | The formula reads `5 × 4 × 2 = 40`, but the slider shows `Capsule storeys 10` and Tree Statistics still shows `0 10`. The perspective is drawn shorter than 10 storeys. The frame is internally inconsistent: slider, tree and formula disagree. | Capture the frame after the slider has settled, so that slider, Tree Statistics and formula all show the same value. If the intent is "5 storeys, not one wire changes", the slider and tree must read 5 and the formula 40. If the frame should keep 10, use `10 × 4 × 2 = 80` and keep the step at the same value. |
| 2 | `orient` | DE only | Major (English in German video) | The Brep parameter keeps the English nickname "Capsule per slot". | Rename in the DE build to "Kapsel pro Slot" (or localise the nickname). EN keeps "Capsule per slot". |
| 3 | `swap` | EN and DE | Minor (value without object) | Two Panel outputs show bare `0` and `69` with no label. The numbers are correct (B after swap = 0, C after swap = 69) but a student cannot tell what they count. | Add a label to each output, e.g. "Typ B: 0" and "Typ C: 69" (EN: "Type B: 0" and "Type C: 69"), or wrap them in a labelled panel. |

### Cosmetic (not blockers, optional)

- `capital`, `ontop`/`capital` in both languages: the overhang slider's max label ("1.0" / "1,0") is overlapped by the Addition node wires. The "Capital" group also starts past the left canvas edge.
- `mistake` and `fix`, both languages: "Σ = 88" / "Σ = 86" sits on the group border at the canvas bottom edge. Readable, but tight.
- `podium`, both languages: the Podium group runs into the perspective panel border on the right.

### Checked and OK

- All 22 clauses in EN and DE: captions match the picture and the narration. Numbers in captions agree with the frames (140, 35.75 m / 35,75 m, 13 storeys, 2.75 m, 33 m / 33 m base, 31.4 × 20.7 × 5.5 m, 54 / 86 / 88 / 140, 1.30 m → 1.60 m).
- German grammar and terminology in every DE caption and label is correct (Kern, Ebenen, Kapseln, Kopf, Sockel, Geschoss, Fassade, Kapselgeschosse, Sockelgeschosse, decimal commas).
- The `recap` glossary shows German = English in the DE video. This is intentional and bridges the English Grasshopper UI names, so it is not flagged.
- Grasshopper UI names in English in the DE video (Center Box, Rotate, Orient, Series, Tree Statistics, Cull Index, List Item, Graft Tree, Radians, Unit Z, Brep, Plane) are allowed.
- No stage is empty or static. Every frame has a visible 3D model and a live graph.

## Files

- Report (this file): `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/sweep-grasshopper-tower.md`
- Scene (read only): `C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py`
- Frames (read only): `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/🗑️generated/frames/🦗️grasshopper/Beat3_Tower/{en,de}/`
