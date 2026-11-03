# r2 bt: bilingual catalog text fixes

All edits in `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`, Edit tool only, en and de.

## person-cooking and standing-adult (`👤️humans/*/🔣️.json`)
The two assets carry text derived from standing-phone-user with the phone/ponytail/headphone words cut out, so more than the audited fields were broken. Reference for the intact originals: `standing-phone-user`.
- `decisions.omitted` (50): "camera lens of." / "dess." became "the camera lens." / "die Kameralinse."
- `decisions.merged` (200): "hair tie into." / "Haargummi in." became "... into the hair cap." / "... in die Haarkappe."
- `decisions.thinElements`, subject derived from the element ids and shapes:
  - person-cooking 50: "the 10 mm is a real ... slab" became the 18 mm `spoon-handle` (path +-0.009, `penMm` 0.1), a filled shape with a 0.1 mm outline.
  - person-cooking 100: `spoon-handle` (18 mm wide, 260 mm long, `penMm` 0.1) and `pot-handle` (no outline).
  - person-cooking 200: the single `spoon` path (350 mm long) replaces the separate handle and bowl shapes; the existing spoon-handle sentence is kept.
  - standing-adult 50: the `pocket` (detail) and hood fold as 0.13 mm detail lines (no phone or slab exists in the asset).
  - standing-adult 100: the `thumb-l/-r` filled shapes (about 110 mm) and the `pocket` 0.1 mm line.
  - standing-adult 200: each `hand` is one filled shape without thumb or fingers; the pocket is the only 0.10 mm line.
  - The invented "148 mm stroke" numbers (leftovers of the phone) are gone, because no element has that length.
- Same-origin damage fixed in the same assets (en and de): `retained` 100 ("Hair mass with and ear ... band at the neck ... stroke between the hands" now "Hair mass with ear, face profile with nose and chin, mitten hand with thumb."), `overlays` 50/100/200 ("sitzenplatte trägtwert", ";schale und Bügel tragenwert", "der Strich trägtwert" replaced by complete clauses; the cooking overlays name the spoon).

## Castor labels
- `🛋️furniture/office-swivel-chair` and `👤️humans/seated-office-worker`: `Hinterer Rollen A/B` became `Hintere Rolle A/B`; `Vorderer Rollen` became `Vordere Rolle` (same grammar error).
- `office-swivel-chair` contacts: `Kontakt Hinterer Rolle A/B` became `Kontakt Hintere Rolle A/B`.

## english-oak and sibling trees
- The audited oak string ("stroke (0.2 mm, true length 148 mm) ...") no longer exists in english-oak; the only subject-less line left was the lowercase `thinElements` en of rep 50, now "Twigs, leaf marks ...".
- Same lowercase start fixed in date-palm (en, de), hornbeam-hedge, lombardy-poplar, miscanthus-grass, multi-stem-birch, scots-pine, weeping-willow (en, de).

## Verification
- All 12 touched JSON files parse (python json).
- Catalog check `bun 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id> --samples 3` (accepts `--asset`): rc=0 and checkmark for all 12 assets (person-cooking, standing-adult, seated-office-worker, office-swivel-chair, english-oak, date-palm, hornbeam-hedge, lombardy-poplar, miscanthus-grass, multi-stem-birch, scots-pine, weeping-willow). One transient loader crash on the vegetation batch (parallel load while others edited) passed on rerun.

## Open
- Low items from the audit not handled (outside the brief): `Jogger:in`, `Aus` vs `Ausgeschaltet`.
- Script `🧰️bt/frag.py` is a heuristic fragment scan; it shows more benign hits (lowercase after semicolon, "None; ...").
