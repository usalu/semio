"""🧮 Row diff of non-stub oracle rows between two pos result.json files (resolution/privacy codes by default; --all-codes for type codes too).
usage: rowdiff.py <before-result.json> <after-result.json> [--codes E0425,..] [--all-codes] [--max N]"""
import json, re, sys, collections
a = sys.argv[1:]
RES = "E0425 E0432 E0433 E0412 E0603 E0616 E0624 E0107 E0422 E0423 E0405 E0407 E0437 E0438".split()
codes = None if "--all-codes" in a else set(a[a.index("--codes") + 1].split(",")) if "--codes" in a else set(RES)
mx = int(a[a.index("--max") + 1]) if "--max" in a else 30
def rows(p):
    r = json.load(open(p, encoding="utf-8"))
    out = {}
    for cfg, e in r["summary"].items():
        c = collections.Counter()
        for d in e["new_list"]:
            if d["stub"] or d.get("inherited"):
                continue
            if codes is not None and d["code"] not in codes:
                continue
            f = re.sub(r"^__m_[A-Za-z0-9_]+/", "", d["file"])
            f = "/".join(f.split("/")[-4:])
            c[(d["code"], f, re.sub(r"__m_[A-Za-z0-9_]+", "M", d["msg"])[:130])] += 1
        out[cfg] = c
    return out
b, af = rows(a[0]), rows(a[1])
for cfg in b:
    gone = b[cfg] - af.get(cfg, collections.Counter())
    new = af.get(cfg, collections.Counter()) - b[cfg]
    print(f"== {cfg}: before {sum(b[cfg].values())} after {sum(af.get(cfg, {}).values())} gone {sum(gone.values())} new {sum(new.values())}")
    for t, c in (("GONE", gone), ("NEW", new)):
        for k, n in list(c.items())[:mx]:
            print(f"  {t} x{n} {k[0]} {k[1]}: {k[2]}")
