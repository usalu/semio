# [DEBUG] Restore a representation from the git index into the worktree.
import json
import subprocess
from pathlib import Path

rel = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/leaning-person/🔣️.json"
path = Path("/home/deck/semio") / rel
index = json.loads(subprocess.check_output(["git", "show", f":{rel}"]))
work = json.loads(path.read_text())
work["asset"]["representations"]["500"] = index["asset"]["representations"]["500"]
work["asset"]["dimensions"] = index["asset"]["dimensions"]
path.write_text(json.dumps(work, indent=2) + "\n")
print("wrote", path.stat().st_size)
