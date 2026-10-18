"""📏 C5C lib-wiring ratchet maintenance: for the given plugin crate lib roots (repo-relative) recomputes `split("\n").length`; a root at or under the budget is dropped from the pins, a pinned root above the budget is lowered to its count (never raised). Atomic write after a re-read. usage: ratchet.py <root-rel>... [--apply]"""
import json, os, sys
R = "/Users/ueli/Documents/semio/"
P = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json"
raw = open(P, encoding="utf-8").read()
d = json.loads(raw)
assert json.dumps(d, indent=2, ensure_ascii=False) + "\n" == raw or json.dumps(d, indent=2, ensure_ascii=False) == raw, "round-trip differs"
budget = d["budget"]
changed = []
for rel in [a for a in sys.argv[1:] if not a.startswith("--")]:
    n = len(open(R + rel, encoding="utf-8").read().split("\n"))
    pin = d["entries"].get(rel)
    if n <= budget:
        if pin is not None:
            del d["entries"][rel]; changed.append((rel[-50:], pin, "dropped (%d <= %d)" % (n, budget)))
        else:
            changed.append((rel[-50:], None, "complies (%d), no pin" % n))
    else:
        if pin is None:
            changed.append((rel[-50:], None, "UNPINNED OVER BUDGET %d" % n))
        elif n < pin:
            d["entries"][rel] = n; changed.append((rel[-50:], pin, "lowered to %d" % n))
        elif n > pin:
            changed.append((rel[-50:], pin, "GREW %d" % n))
        else:
            changed.append((rel[-50:], pin, "unchanged"))
for c in changed:
    print(c)
if "--apply" in sys.argv:
    now = open(P, encoding="utf-8").read()
    assert now == raw, "ratchet changed meanwhile; re-run"
    out = json.dumps(d, indent=2, ensure_ascii=False) + ("\n" if raw.endswith("\n") else "")
    tmp = P + ".c5c-tmp"
    open(tmp, "w", encoding="utf-8").write(out)
    os.replace(tmp, P)
    print("applied")
