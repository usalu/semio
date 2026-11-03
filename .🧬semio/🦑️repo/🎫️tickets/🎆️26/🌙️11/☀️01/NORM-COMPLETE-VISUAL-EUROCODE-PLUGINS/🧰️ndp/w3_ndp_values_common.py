import re, os, sys, json, collections
sys.path.insert(0, os.path.dirname(__file__))
import w3_ndp_values_scan as s
CALL = re.compile(r'(?:\bndp_common|\bndp_cell_common|(?<![\w.])ndp|\bread)\(\s*(&format!\(\s*)?"(ndp\.[^"]*)"')
def classify(i):
    ids = s.resolve(i) if "{" not in i else [k for k in s.E if k.startswith(i.split("{")[0])]
    if not ids: return "unregistered", []
    kinds = collections.Counter()
    for k in ids:
        a, b = s.E[k]["recommended"], s.E[k]["naDe"]
        kinds["both-none" if a is None and b is None else "en-only" if b is None else "de-only" if a is None else "equal" if a == b else "differ"] += 1
    return dict(kinds), ids
if __name__ == "__main__":
    code = sys.argv[1]
    for dp, dn, fn in os.walk(os.path.join(s.ART, s.CODES[code])):
        dn[:] = [d for d in dn if not d.startswith("🧪") and d not in ("tests", "target", "__pycache__", "🗑️generated")]
        for f in fn:
            if not f.endswith(".rs"): continue
            p = os.path.join(dp, f)
            for ln, line in enumerate(open(p, encoding="utf-8"), 1):
                for m in CALL.finditer(line):
                    k, ids = classify(m.group(2))
                    print(os.path.relpath(p, s.ROOT).split("/")[-2], ln, m.group(2), k)
