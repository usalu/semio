# Final Sweep: 🦗️grasshopper Beat1_Anatomy (Class Beat1_Anatomy)

Verdict: **FIX**

Scope: all 26 PNGs in each language, read in clause order from the NARRATION list of
`C:/git/semio/🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py` (24 clauses; `warning_fixed` and `error_fixed` are the after-states of `warning` and `error`).

Frames: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/🗑️generated/frames/🦗️grasshopper/Beat1_Anatomy/{en,de}/`

Staleness: scene file mtime 2026-10-29 04:16; frames range 04:07 to 04:24. None is more than 30 minutes older than the scene, so nothing is flagged as stale.

Numbers: checked against the authoritative model `C:/git/semio/🎓️teaching/🏛️architecture/🏢️nakagin/README.md` and `🔣️.json`. All values agree: capsule 2.5 x 4.0 x 2.5 m, volume 25 m³, window 1.3 m, Type C window 1.6 m, storey pitch 2.75 m, 140 capsules, Tower A 9 storeys x 3 facades (27 branches), Tower B 11 storeys, 2 capsules per facade, series heights 0 to 22 m (9 storeys) and 0 to 27.5 m (11 storeys). No wrong number found.

## Blockers

None.

## Majors (FIX)

### DE, `de/warning.png`: English Grasshopper balloon
- Defect: balloon reads "Data conversion failed from Text to Number" (English in the German video).
- Fix: German balloon text, e.g. "Datenkonvertierung von Text zu Zahl fehlgeschlagen".

### DE, `de/error.png`: English Grasshopper balloon
- Defect: balloon reads "Input parameter Z failed to collect data". The same frame already says "erstes Problem", so the German video is inconsistent.
- Fix: German balloon text, e.g. "Eingabeparameter Z konnte keine Daten sammeln".

### DE, `de/sliders.png` and `de/live.png`: English "Slider" in German captions
- Defect: "Jeder Slider ist ein Maß" (sliders) and "Zieht man einen Slider" (live). The German legend (`de/legend_tool.png`, row 6) defines Slider as "Schieberegler".
- Fix: "Jeder Schieberegler ist ein Maß" and "Zieht man einen Schieberegler". Keep "Number Slider" in `de/slider.png`, since that is the real UI name.

### DE, `de/plugin.png`: English "Tab" in German caption
- Defect: "fügt dem Ribbon einen Tab hinzu".
- Fix: "fügt dem Ribbon einen Reiter hinzu". "Ribbon" can stay as a UI name.

### DE, `de/warning.png`, `de/warning_fixed.png`, `de/error_fixed.png`: decimal point in the Z value
- Defect: the Z value shows "2.5" (dot). Captions and other German frames use "2,5" (comma), e.g. `de/panel.png` shows "2,5".
- Fix: render "2,5" in these three frames.

### EN and DE, `series.png` and `count.png`: list values without unit (judgement call)
- Defect: the Panel lists heights (0, 2.75, 5.5, ...) with no unit. The captions call them heights in metres. Viewport labels carry "m" ("8 → 22.0 m"), but the list itself does not.
- Fix: add "m" to the Panel header or to each value, e.g. "{0} · m". Drop this item if the team's convention puts units only in the viewport.

## Checked and OK
- Every caption matches its picture, in both languages, including the EN/DE legend frames.
- No empty or static stage. `hook` and `preview` are intentionally sparse.
- No English prose in the German video apart from the items above, Grasshopper UI names ("Center Box", "Number", "Series", "Volume", "Canvas", "Brep", "True", "LB Direct Sun Hours") and the legend's deliberate English column.
- The strict layout check (`layout.md`, EN and DE) flags leader lines through "0 → 0.0 m" and the "1" / "2.75" labels in `item`, `series` and `count`. Visually these are the intended annotation leaders, and no text is clipped or overlapped. Not reported.

## Not reported (minor, per brief)
- The Volume panel in `en/preview.png` and `de/preview.png` runs to the right edge of the canvas. Text stays readable.
- The Value List label reads "Ø 1.60 m" in `valuelist`, against "Ø 1.6 m" on the perspective card.
- The "S Start 0 ..." legend sits tight against the "12" max label in `de/item.png`.
- The "3" marker in `legend_data` sits on the panel edge (EN and DE).
