"""🧮 sh7: line-multiset losslessness of each package-1 group (root + children on disk) against the HEAD blob; `pub(super) ` is normalised, added lines are classified."""
import collections, re, subprocess, sys, os
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/c4p1-tools")
from plans import PLANS, ORDER
R = "/Users/ueli/Documents/semio/"
norm = lambda t: collections.Counter(l.strip().replace("pub(super) ", "") for l in t.split("\n") if l.strip())
bad = 0
for key in ORDER:
    spec = PLANS[key]
    head = subprocess.run(["git", "-c", "core.quotepath=false", "show", "HEAD:" + spec["rel"]], cwd=R, capture_output=True).stdout.decode()
    base = os.path.dirname(spec["rel"])
    files = [spec["rel"]] + [f"{base}/{c['folder']}/🦀️.rs" for c in spec["children"]]
    cur = collections.Counter()
    for f in files:
        cur += norm(open(R + f, encoding="utf-8").read())
    lost = norm(head) - cur
    added = cur - norm(head)
    kinds = collections.Counter()
    for l, n in added.items():
        k = "region" if re.match(r"//\s?#(end)?region", l) else "wiring" if re.match(r'^(#\[path = ".*"\]|mod \w+;|pub use \w+::\*;|use super::\*;|include!\(".*"\);)$', l) else "doc" if l.startswith(("//!", "///")) else "other"
        kinds[k] += n
    print(f"{key}: lost={sum(lost.values())} added={sum(added.values())} {dict(kinds)}")
    bad += sum(lost.values())
print("TOTAL LOST", bad)
sys.exit(1 if bad else 0)
