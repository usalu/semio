"""Hosta: the drawn profile of petiole-l2 (plan 1:50) widened by 15 % and of blade-l4 (elevation 1:100) narrowed by 2 %, so that the enclosed gaps between the plates close and open like those of the volume (a 4-connected channel of one cell)."""
import json, subprocess, os
HERE = os.path.dirname(os.path.abspath(__file__)); G = HERE + "/vegetation-work/"
rows = [{"scale": 50, "view": "plan", "id": "petiole-l2", "bone": "petiole-l2", "su": 1, "sv": 1.15, "iou0": 0, "iou1": 1},
        {"scale": 100, "view": "elevation", "id": "blade-l4", "bone": "blade-l4", "su": 1, "sv": 0.98, "iou0": 0, "iou1": 1}]
json.dump(rows, open(G + "fit-hosta-fix.json", "w"))
print(subprocess.run(["python3", HERE + "/vegetation-fit-apply.py", "hosta-clump", G + "fit-hosta-fix.json", "0"], capture_output=True, text=True).stdout.strip())
