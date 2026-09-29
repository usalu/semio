"""🔎 FH7 review helper: prints each CEFG work-list entry of one owner with its census raise sites (enclosing fn + context
lines from the overlay the census indexes). Usage: python3 fh7-show.py <owner-substring> [before=5] [max-raises=3] [code-substring]"""
import json, re, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults")
CENSUS = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🤖️generated/🎯️acceptance/🧯️fault-census.json"
WORK = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-sets/class/family-CEFG.json")
owner = sys.argv[1]
before = int(sys.argv[2]) if len(sys.argv) > 2 else 5
most = int(sys.argv[3]) if len(sys.argv) > 3 else 3
filt = sys.argv[4] if len(sys.argv) > 4 else ""
census = json.loads(CENSUS.read_text())
raises = {}
for r in census["raises"]:
    raises.setdefault((r["path"].split("/")[2], r["code"]), []).append(r)
cache = {}
def lines(p):
    if p not in cache:
        cache[p] = (ROOT / p).read_text(errors="replace").splitlines()
    return cache[p]
def short(p):
    parts = p.split("/")
    return "/".join(parts[2:4] + ["…"] + parts[-4:])
for i, e in enumerate(json.loads(WORK.read_text())):
    if owner not in e["owner"] or filt not in e["code"]:
        continue
    print(f"=== [{i}] {e['code']} | {e['class']} | {e['rule']}\n    en: {e['en']}")
    rs = raises.get((e["owner"], e["code"]), [])
    for r in rs[:most]:
        L = lines(r["path"]); n = r["line"] - 1
        fn = next((L[k].strip()[:140] for k in range(n, max(-1, n - 400), -1) if re.search(r"\bfn\s+\w+", L[k])), "?")
        print(f"  @ {short(r['path'])}:{r['line']}  in {fn}")
        for k in range(max(0, n - before), min(len(L), n + 2)):
            s = L[k].strip()
            if s:
                print(f"    {'>' if k == n else ' '} {s[:170]}")
    if len(rs) > most:
        print(f"  (+{len(rs) - most} more raises)")
