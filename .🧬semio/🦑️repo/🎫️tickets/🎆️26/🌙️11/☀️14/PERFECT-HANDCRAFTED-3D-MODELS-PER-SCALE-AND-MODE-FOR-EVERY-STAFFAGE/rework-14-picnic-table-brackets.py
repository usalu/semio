"""🧺️ Author 14, picnic table: at 1:500 both benches hung in the air, 27.5 mm short of the A-frames at either end (board and bracket blocks ended at z = 0.750 m, the inner face of the frames is at 0.7775 m), in body, thermal and structure alike. The 75 mm bracket block under each board end now runs from 0.7025 m to the frame face at 0.7775 m, where the steel gusset of the finer scales is bolted and pivots, so each bench hangs between the frames through its brackets in every pose.

Run from the ticket folder: python3 rework-14-picnic-table-brackets.py
"""
import json

PATH = "volumes/furniture/picnic-table.json"
document = json.load(open(PATH))
volume = document["volumes"]["500"]
for solid in volume["body"]:
    if solid["id"] in ("bracket-r", "bracket-l"):
        solid["from"], solid["to"] = 0.7025, 0.7775
    if solid["id"] in ("bracket-r-b", "bracket-l-b"):
        solid["from"], solid["to"] = -0.7775, -0.7025
EDITS = (
    ("en", "each bracket as a 75 mm block under the board end;", "each bracket as a 75 mm block under the board end that reaches the inner face of the A-frame, so the bench hangs between the frames;"),
    ("de", "jede Konsole als 75-mm-Block unter dem Brettende;", "jede Konsole als 75-mm-Block unter dem Brettende, der bis an die Innenfläche des A-Bocks reicht, sodass die Bank zwischen den Böcken hängt;"),
)
for language, old, new in EDITS:
    text = volume["decisions"][0][language]
    if new not in text:
        assert old in text, old
        volume["decisions"][0][language] = text.replace(old, new)
json.dump(document, open(PATH, "w"), ensure_ascii=False, indent=1)
