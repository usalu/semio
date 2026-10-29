# Final Verification: grasshopper_capsule (`Beat2_Capsule`, topic 💊️capsule)

Scope: fresh keyframes in `en` and `de` (23 each, 1280x720), captured with `capture.py` into `🗑️generated/frames/🦗️grasshopper/Beat2_Capsule/{en,de}/`. Earlier audit: `audit-grasshopper-capsule.md`. Model: `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json` (authoritative; values in frames match: 140 capsules, 2.50 x 4.00 x 2.50 m, window 1.30 m, type C window 1.60 m, D offset -0.45 m and 5 deg, bolt points, volume 24.54 m³, domains 1.50 to 3.50 / 2.50 to 5.50 / 1.50 to 3.50 m).

Capture note: the first `en` run ended with a cp1252 `UnicodeEncodeError` on the final print, after frames and `layout.md` were written. Both languages were re-run with `PYTHONIOENCODING=utf-8 PYTHONUTF8=1`; both runs completed ("captured", 23 beats). Logs: `🗑️generated/cap-en-run.log`, `🗑️generated/cap-de-run.log`.

Layout: `layout.md` reports 52 strict findings in 17 of 23 frames, in both languages (the audit had 47). I did not triage the five extra hits one by one; they are wire-behind-component and chip-on-geometry types that the scene notes already accept.

## Verdict: NOT READY

Most audit items are fixed. Three majors remain (wrap, frontplane, lift wiring), and the German has two wrong words that are new blockers under the brief.

## Audit blockers and majors: status

| audit frame | status | evidence |
| --- | --- | --- |
| lift: "back face stands on base plane" (en, de) | FIXED | en "now the box stands with its bottom face on the base plane"; de "Unterseite auf der Grundebene" |
| lift: wires cross behind A/B nodes and Pt | NOT FIXED (major) | `en/lift.png`, `de/lift.png`: Length and Height wires still enter A/B from crossing paths; the Width wire passes near A/B |
| live: frame identical to lift | FIXED | `en/live.png` Length 5.20, panel {0.00, 2.60, 1.25}, dimension 5.20 m; de "Boxmitte {0,00; 2,60; 1,25}" |
| sliders: empty stage | FIXED | `en/sliders.png` shows the capsule, dimensions and the live parameter card |
| sliders and all slider frames: Window Ø colour bar clipped by sheet | FIXED | bar now sits inside the canvas sheet (`en/sliders.png`, `de/sliders.png`) |
| domains: truck-carries wording (transport limit as fact) | FIXED | en "here the limits are teaching assumptions, for example a transport envelope"; de "hier sind die Grenzen Lehrannahmen" |
| domains: card title lacks "teaching assumption" | FIXED | "Slider domain = design limit (teaching assumption)" (en and de) |
| limit: ~25 px gap to envelope | PARTLY (minor) | caption now "reaches"; a small gap (~10 px) remains at the right edge in `en/limit.png` and `de/limit.png` |
| rotate: Panel overlaps Rotate | FIXED | Panel now starts at the Rotate bottom edge; no overlap |
| rotate: "-90" without unit | FIXED | "-90 °" on the slider and "-90°" in the viewport; the storyboard arc is still missing (minor) |
| plane / box / lift: coordinate panels without object or unit | FIXED | "Origin [m]", "Box centre [m]", "Boxmitte [m]", "Ursprung [m]" |
| bolts: no unit or label; index panel | FIXED | "Bolt points [m]" / "Schraubenpunkte [m]" with indices 0 to 3 |
| rounded: volume panel without object name | FIXED | "Volume 24.54 m³" / "Volumen 24,54 m³" |
| cut: "0.30 m" without object name | FIXED | "Cylinder length 0.30 m" / "Zylinderlänge 0,30 m" |
| frontplane: wire tangle; Offset wire untraceable | NOT FIXED (major) | `en/frontplane.png`, `de/frontplane.png`: the same tangle around A/B and Pt, the "2" constants still in the wire paths |
| wrap: capsules hide one another | NOT FIXED (major) | `en/wrap.png`, `de/wrap.png`: back green C and red D capsules are still mostly behind the core; "every plane gets a capsule" cannot be counted |
| types: row A not highlighted | FIXED | row A highlighted in `en/types.png` and `de/types.png` |
| recap: no icons; colours reuse parameter colours | FIXED | icons per row; "Parameter" no longer orange |
| planes: missing units | FIXED | "Base plane A [m]", "Target plane B [m]" headers |

Other audit minors, still open (not blockers): `rounded` radius chip "r 0.25 m" sits on the outline and the quarter-circle dimension is missing; `cut` window-area panel missing; `wrongplane` "Circle on base plane" chip covers the capsule top, and the "2" box sits under the Offset stub; `frontplane` axes still short; `four` caption and frame disagree (see below); `types` table top and right edge on the sheet border; `orient` Capsule wire disappears behind the Target B panel; `planes` viewport shows the full tower while the storyboard says "core fragment"; `outside` "!" markers sit next to the index tags; `rotate` storyboard arc missing; `rounded` and `cut` panel details as above.

## New blockers and majors

1. **Wrong German, de `cut`**: "Das runde Fenster entsteht in der Vorderseite." must be "an der Vorderseite" (`de/cut.png`). Wrong preposition, major.
2. **Wrong German, de `four`**: "Speisen Sie Typ A bis D in dieselbe Slider" must be "in denselben Slider" (Slider is masculine; `de/four.png`). Major.
3. **Caption and drawing disagree, `four` (en and de)**: the caption says window diameter and window offset change, but the only capsule in the viewport is type D with Ø 1.30 m and offset -0.45 m. The diameter change is only visible in the table (type C 1.60 m). Fix the caption to name only the offset, or show type C. Major by the brief's rule.
4. **Minor German, de `domains`**: "hier sind die Grenzen Lehrannahmen" is grammatical but awkward; suggest "hier sind die Grenzen Annahmen des Lehrmodells, zum Beispiel eine Transporthülle."

## Must still change before READY

1. `wrap`: move the back planes' capsules in front or change the azimuth so all eight capsules are countable (en, de).
2. `frontplane`: reroute the Offset and A/B wires so each input is traceable (en, de). The same tangle in `lift` (en, de): keep the Width wire away from A/B.
3. de `cut`: "an der Vorderseite".
4. de `four`: "in denselben Slider".
5. `four` (en, de): caption must match the frame (offset only, or show type C).
6. Optional polish (minor list above) before final render.

## Files

- Report: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/verify-grasshopper-capsule.md`
- Audit checked: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/audit-grasshopper-capsule.md`
- Fresh frames: `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS/🗑️generated/frames/🦗️grasshopper/Beat2_Capsule/en/` and `.../de/`
- Layout reports: `.../🗑️generated/frames/🦗️grasshopper/Beat2_Capsule/en/layout.md` and `.../de/layout.md`
