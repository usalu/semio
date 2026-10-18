"""📏 C6PLUG lib-wiring ratchet updater: removes the pins of roots that comply with the budget and lowers the pins of roots that shrank; re-read guard, byte-identical JSON round trip, atomic rename.
usage: ratchet.py check|apply <tag>..."""
import json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
R = "/Users/ueli/Documents/semio"
FIX = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"
ft = dict(l.rstrip("\n").split("\t") for l in open(HERE + "/files.tsv", encoding="utf-8"))
mode = sys.argv[1]
text = open(FIX, encoding="utf-8").read()
data = json.loads(text)
assert json.dumps(data, indent=2, ensure_ascii=False) + "\n" == text or json.dumps(data, indent=2, ensure_ascii=False) == text, "round trip differs"
budget = data["budget"]
changed = 0
for tag in sys.argv[2:]:
    rel = ft[tag]
    n = len(open(os.path.join(R, rel), encoding="utf-8").read().split("\n"))
    pin = data["entries"].get(rel)
    if pin is None:
        print(f"[{tag}] no pin; count {n}" + (f" EXCEEDS budget {budget}" if n > budget else ""))
        continue
    if n <= budget:
        del data["entries"][rel]
        print(f"[{tag}] pin {pin} removed (count {n} <= {budget})")
        changed += 1
    elif n < pin:
        data["entries"][rel] = n
        print(f"[{tag}] pin {pin} lowered to {n}")
        changed += 1
    elif n > pin:
        print(f"[{tag}] GREW {pin} -> {n}")
    else:
        print(f"[{tag}] pin unchanged {pin}")
if mode == "apply" and changed:
    cur = open(FIX, encoding="utf-8").read()
    assert cur == text, "fixture changed meanwhile; rerun"
    out = json.dumps(data, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")
    tmp = FIX + ".c6plug-tmp"
    open(tmp, "w", encoding="utf-8").write(out)
    os.replace(tmp, FIX)
    print("fixture written", changed, "changes")
