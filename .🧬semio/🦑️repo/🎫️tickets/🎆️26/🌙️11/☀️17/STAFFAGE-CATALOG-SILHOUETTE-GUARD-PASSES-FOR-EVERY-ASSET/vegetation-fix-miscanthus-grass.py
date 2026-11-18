"""Miscanthus: the drawn plume plates (profile length 0.403 m) were 0.02 m longer than the volume plumes (0.382 m); the profile of plume-2 and plume-3 in front and plan is scaled to the volume length at 1:50."""
import json, subprocess, os
HERE = os.path.dirname(os.path.abspath(__file__)); G = HERE + "/vegetation-work/"
rows = [{"scale": 50, "view": v, "id": i, "bone": i, "su": round(0.38248 / 0.403, 4), "sv": 1, "iou0": 0, "iou1": 1} for v in ("front", "plan") for i in ("plume-2", "plume-3")]
json.dump(rows, open(G + "fit-misc-fix.json", "w"))
print(subprocess.run(["python3", HERE + "/vegetation-fit-apply.py", "miscanthus-grass", G + "fit-misc-fix.json", "0"], capture_output=True, text=True).stdout.strip())
