"""🏗 Builds the candidate document of `humans/standing-adult` (round 3): root fixes (`r3-standing-adult-roots.py`) plus the handcrafted volumes (`r3-standing-adult-model.py`) and the redrawn 2D shapes (`r3-standing-adult-2d.ts`) in one file `🗑️generated/r3-standing-adult/🔣️.json` for `r3-validate.ts --file` and `r3-2d.ts --file`.

Usage: python3 r3-standing-adult-build.py
"""
import json
import os
import subprocess
import sys

T = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(T, "🗑️generated/r3-standing-adult")
BUN = sorted(__import__("glob").glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]


def run(*command):
    done = subprocess.run(command, cwd=T, capture_output=True, text=True)
    if done.returncode != 0:
        sys.exit(done.stdout + done.stderr)
    return done.stdout


sys.path.insert(0, T)
if os.path.exists(os.path.join(OUT, "shapes.json")):
    os.remove(os.path.join(OUT, "shapes.json"))
run("python3", "r3-standing-adult-roots.py")
open(os.path.join(OUT, "frames.json"), "w", encoding="utf-8").write(run(BUN, "r3-standing-adult-frames.ts"))
print(run("python3", "r3-standing-adult-model.py").strip())
import r3_standing_adult_strokes

json.dump(r3_standing_adult_strokes.strokes(), open(os.path.join(OUT, "strokes.json"), "w", encoding="utf-8"))
print(run(BUN, "r3-standing-adult-2d.ts").strip())
run("python3", "r3-standing-adult-roots.py")
document = json.load(open(os.path.join(OUT, "asset.json"), encoding="utf-8"))
document["asset"]["volumes"] = json.load(open(os.path.join(T, "volumes/humans/standing-adult.json"), encoding="utf-8"))["volumes"]
document["asset"].pop("volume", None)
json.dump(document, open(os.path.join(OUT, "🔣️.json"), "w", encoding="utf-8"), ensure_ascii=False)
print("candidate", os.path.join(OUT, "🔣️.json"))
