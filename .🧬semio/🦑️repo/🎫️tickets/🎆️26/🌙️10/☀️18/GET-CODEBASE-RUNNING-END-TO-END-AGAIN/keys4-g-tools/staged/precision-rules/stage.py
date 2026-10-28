#!/usr/bin/env python3
"""🎯 usage: stage.py <check|apply|revert|status> [--root DIR] : lands (or removes) the evidenced router rules of keys4-g in the live `🎯️precision/🔣️.json`.
The unit is the RULE ID, not the file: apply puts every staged rule that is absent at the head of the live rule list (JSON round trip keeps the byte format, refuses when the live document is not in canonical format), revert removes exactly those ids and nothing a peer added. Rules of other lanes are never touched. base.txt records the rule ids and the sha256 of the live document the evidence was gathered against (informational: drift is fine as long as the format round trips and the ids are absent)."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
DEFAULT_ROOT = "/Users/ueli/Documents/semio"
ROOT = DEFAULT_ROOT
if "--root" in sys.argv: ROOT = sys.argv[sys.argv.index("--root") + 1]
LIVE = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🎯️precision/🔣️.json"
BACKUP = HERE + "/backup/🔣️.json"
def staged():
    names = sorted(n for n in os.listdir(HERE + "/rules") if n.endswith(".json"))
    return [json.load(open(HERE + "/rules/" + n, encoding="utf-8")) for n in names]
def order(rules):
    key = {r["id"]: i for i, r in enumerate(json.load(open(HERE + "/order.json", encoding="utf-8")))} if os.path.exists(HERE + "/order.json") else {}
    return sorted(rules, key=lambda r: key.get(r["id"], 99))
def load():
    text = open(LIVE, encoding="utf-8").read()
    document = json.loads(text)
    if json.dumps(document, ensure_ascii=False, indent=1) + "\n" != text: sys.exit("REFUSED: live rule document is not in canonical format; no file touched")
    return text, document
mode = sys.argv[1] if len(sys.argv) > 1 else "check"
text, document = load()
present = {r["id"] for r in document["rules"]}
rules = order(staged())
fresh = [r for r in rules if r["id"] not in present]
sha = hashlib.sha256(text.encode()).hexdigest()
base = open(HERE + "/base.txt").read().split() if os.path.exists(HERE + "/base.txt") else []
print("live sha256 %s (%s the evidence base)" % (sha[:12], "equals" if base and base[0] == sha else "DIFFERS from"))
for r in rules: print("%-8s %s  targets=%d" % ("PRESENT" if r["id"] in present else "ABSENT", r["id"], len(r["select"]["targets"])))
if mode in ("check", "status"): sys.exit(0)
if mode == "apply":
    if not fresh: print("nothing to do"); sys.exit(0)
    if ROOT == DEFAULT_ROOT:
        os.makedirs(os.path.dirname(BACKUP), exist_ok=True)
        shutil.copy2(LIVE, BACKUP)
    document["rules"] = fresh + document["rules"]
elif mode == "revert":
    ids = {r["id"] for r in rules}
    document["rules"] = [r for r in document["rules"] if r["id"] not in ids]
else: sys.exit("usage: stage.py check|apply|revert|status")
tmp = LIVE + ".keys4-g.tmp"
open(tmp, "w", encoding="utf-8").write(json.dumps(document, ensure_ascii=False, indent=1) + "\n")
os.replace(tmp, LIVE)
print(mode, "done:", [r["id"] for r in (fresh if mode == "apply" else rules)])
