import json, subprocess, sys, os
from pathlib import Path
R = Path("C:/git/semio")
T = next((R / ".🧬semio/🦑️repo/🎫️tickets").glob("*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"))
G = T / "🗑️generated/sm"
res = G / "results.jsonl"
files = [l.strip() for l in (T / "🧰️sm/files.txt").read_text(encoding="utf-8").splitlines() if l.strip()]
start = sys.argv[1]
files = files[[i for i, f in enumerate(files) if start in f][0]:]
def rows(f):
    out = []
    for l in res.read_text(encoding="utf-8").splitlines():
        d = json.loads(l)
        if d["file"].replace("\\", "/").endswith(f.split("/", 1)[1]):
            out.append(d)
    return out
for f in files:
    if "💊️capsule" in f:
        continue
    for attempt in range(12):
        known = {d["scene"] for d in rows(f) if d["status"] in ("pass", "timeout", "skipped")}
        env = dict(os.environ, SM_SCENE_TIMEOUT="420", SM_TIMEOUT="3000")
        with (G / "progress.log").open("a", encoding="utf-8") as p:
            p.write(f"{f} attempt {attempt} skip {sorted(known)}\n")
        subprocess.run(["bash", str(T / "🧰️sm/run.sh"), f, "", ",".join(sorted(known))], env=env, stdout=(G / "progress.log").open("a", encoding="utf-8"), stderr=subprocess.STDOUT)
        last = rows(f)[-1] if rows(f) else {}
        if last.get("status") != "timeout":
            break
with (G / "progress.log").open("a", encoding="utf-8") as p:
    p.write("ALLDONE2\n")
