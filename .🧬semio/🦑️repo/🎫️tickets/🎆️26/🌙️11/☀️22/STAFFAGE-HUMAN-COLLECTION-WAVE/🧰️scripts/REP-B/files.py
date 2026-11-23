"""🗂️ Writes `record/files.md`: every file this repair created, updated or left behind, as repo-relative paths (the scripts of this folder, the repair ops per person, the catalog files and the row files named in `record/rows.txt`).
python3 files.py"""
import glob
import os
import re

R = "/Users/ueli/Documents/semio"
K = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE"
U = f"{R}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS"
HERE = f"{K}/🧰️scripts/REP-B"
rel = lambda p: os.path.relpath(p, R)
people = re.findall(r'"([a-z-]+)": "HW-\d+"', open(f"{HERE}/people.ts", encoding="utf8").read())
rows = open(f"{HERE}/record/rows.txt", encoding="utf8").read() if os.path.exists(f"{HERE}/record/rows.txt") else ""
changed_rows = re.search(r"rows changed: (.*?);", rows)
changed_rows = [] if not changed_rows or changed_rows.group(1) == "none" else changed_rows.group(1).split()
created = sorted(glob.glob(f"{HERE}/*.*")) + sorted(glob.glob(f"{HERE}/record/*")) + sorted(glob.glob(f"{K}/🧰️scripts/HW-1*/repair/*.json")) + [f"{K}/🧰️scripts/HW-12/easel-stay.py", f"{U}/humans-b-repair.md"]
updated = [f"{R}/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/{slug}/🔣️.json" for slug in people] + [f"{K}/🧰️scripts/HW-17/hw17lib.py"] + [f"{K}/📥️manifest-rows/{slug}.json" for slug in changed_rows]
removed = open(f"{HERE}/record/removed.txt", encoding="utf8").read().strip() if os.path.exists(f"{HERE}/record/removed.txt") else "nothing"
with open(f"{HERE}/record/files.md", "w", encoding="utf8") as f:
    f.write("Created:\n\n" + "\n".join(f"- `{rel(p)}`" for p in created) + "\n\nUpdated:\n\n" + "\n".join(f"- `{rel(p)}`" for p in updated) + "\n\nRemoved or left behind:\n\n" + removed + "\n")
print(len(created), "created,", len(updated), "updated")
