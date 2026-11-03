"""Parses every non-mutation norm `🥒️.feature` with the platform's own gherkin parser and prints error counts. Usage: .venv/bin/python -I wave2-features-v2-parse-all.py"""
import json, subprocess
from pathlib import Path
T = Path(__file__).resolve().parent.parent
ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
for f in sorted(ROOT.rglob("🥒️.feature")):
    if "mutate-" in str(f):
        continue
    out = subprocess.run(["node", "--experimental-strip-types", str(T / "🗑️generated/features/dump.ts"), str(f)], capture_output=True)
    try:
        d = json.loads(out.stdout)
    except Exception:
        print("CRASH", f.relative_to(ROOT)); continue
    print("%2d err %2d scen | %s | %s" % (len(d["errors"]), len(d["scenarios"]), str(f.relative_to(ROOT))[:140], (d["errors"][:1] or [""])[0][:110]))
