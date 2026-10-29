# Final Sweep: Grasshopper Beat2_Capsule (series 🦗️grasshopper)

Scene: `C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py`, class `Beat2_Capsule` (scene mtime 2026-10-29 03:55:14).
Frames: `🗑️generated/frames/🦗️grasshopper/Beat2_Capsule/{en,de}/` (23 clauses per language, all PNGs read in narration order).
Staleness: all PNGs are between 03:44 and 03:59, within 11 minutes of the scene mtime. None older than the scene by more than 30 minutes.
Numbers checked against the authoritative nakagin model (`🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json`): 140 capsules, 4 types, 2 cores, window Ø 1.30 (C: 1.60), window u 0.00 (D: -0.45), D yaw 5 deg, corner radius 0.25, core B centre x -11.1. All match. Volume 24.54 m3 is consistent with a 2.50 x 4.00 x 2.50 box filleted at r 0.25.

## Verdict: FIX

### Defect A (all DE frames, 23 of 23): English text in the German video
- Viewport label "Perspective" is hard-coded in English. Rhino's German UI shows "Perspektive".
  - Location: `🐍️.py` line 216, `self._say("Perspective", ...)`. Fix: `self._say(self.tr("Perspective", "Perspektive"), ...)`, as the casting clip already does (`🫗️casting/🐍️.py` line 545).
- Affects every DE frame in the top-right viewport.

### Defect B (DE frames: sliders): slider nicknames are English
- Shown in DE as "Width", "Length", "Height", "Radius", "Window Ø", "Offset", while the parameter panel right next to them says "Breite", "Länge", "Höhe", "Eckenradius", "Fensterdurchmesser", "Fensterversatz".
- Frames: hook (Width), sliders, box, lift, live, rounded, wrongplane, frontplane, cut, domains, limit, types, four.
- Location: `🐍️.py` line 40, `NAMES = {"w": "Width", "l": "Length", "h": "Height", "r": "Radius", "d": "Window Ø", "wu": "Offset", "bu": "Bolt x", "bvl": "Bolt low", "bvh": "Bolt high", "an": "Angle"}`. These are language-independent literals used for the DE canvas.
- Fix: derive the nicknames through the language switch, e.g. Width→Breite, Length→Länge, Height→Höhe, Radius→Radius, Window Ø→Fenster Ø, Offset→Versatz, Bolt x→Bolzen x, Bolt low→Bolzen unten, Bolt high→Bolzen oben, Angle→Winkel.
- Also see line 631 and line 750, which already use tr-pairs for the card and legend rows. Use the same pattern for NAMES.

### Defect C (DE frames: planes, orient, rotate, listitem, outside, wrap): parameter nicknames are English
- Shown as "Base A", "Target B", "Capsule", "Core axis", "Angle", "Types", "Planes". The panels next to them say "Grundebene A", "Zielebene B", "Ebene B gedreht", "Verschiebung". Caption and panel vocabulary mismatch.
- Frames: planes, orient, rotate, listitem, outside, wrap (and the recap has none).
- Location: `🐍️.py` lines 573–586, `"label": "Base A"`, `"Target B"`, `"Capsule"`, `"Core axis"`, `"Angle"`, `"Types"`, `"Planes"`, `"Base A"`.
- Fix: wrap each label in `self.tr(...)`, e.g. Base A→Basis A, Target B→Ziel B, Capsule→Kapsel, Core axis→Kernachse, Angle→Winkel, Types→Typen, Planes→Ebenen. Keep "Wrap" and the GH type hints (Plane, Brep).

### Not blocking (checked, no action)
- Proper name "Nakagin Capsule Tower" in the DE header is accepted as a building name.
- German grammar in all 23 DE captions is correct. The comma decimals and the "Sie" form are used consistently.
- All numbers in the frames match the nakagin model and the scene's own panels: 140, 4, 1.50/3.50, 2.50/4.00/2.50, 24,54 m3, box centre {0,00; 2,00; 1,25} and {0,00; 2,60; 1,25} at length 5.20, rotation -270 deg giving {-13,85; -1,38; 5,50}, bolt points, types A to D, index warning, wrap cycle, and the recap legend.
- Clause-to-picture mapping is correct for all 23 clauses in both languages.

## Per-frame status (clause order)
| # | Frame | EN | DE |
|---|-------|----|----|
| 1 | hook | OK | Defect A, B |
| 2 | parametric | OK | Defect A |
| 3 | sliders | OK | Defect A, B |
| 4 | plane | OK | Defect A, B |
| 5 | box | OK | Defect A, B |
| 6 | lift | OK | Defect A, B |
| 7 | live | OK | Defect A, B |
| 8 | rounded | OK | Defect A, B |
| 9 | wrongplane | OK | Defect A, B |
| 10 | frontplane | OK | Defect A, B |
| 11 | cut | OK | Defect A, B |
| 12 | bolts | OK | Defect A, B (Bolt x/low/high) |
| 13 | domains | OK | Defect A, B |
| 14 | limit | OK | Defect A, B |
| 15 | types | OK | Defect A, B |
| 16 | four | OK | Defect A, B |
| 17 | planes | OK | Defect A, C |
| 18 | orient | OK | Defect A, C |
| 19 | rotate | OK | Defect A, C (Angle, Core axis) |
| 20 | listitem | OK | Defect A, C (Types, Planes, Base A) |
| 21 | outside | OK | Defect A, C |
| 22 | wrap | OK | Defect A, C |
| 23 | recap | OK | Defect A |

Minor layout overlaps (tooltips covering part of a node, clustered wires) were seen and are not reported, per the brief.
