"""📋 Classifies the gaps of gap-<code>.json into diagnostics (integrity, reference, unknown, empty, na), catalogue case ids and engineering families; prints a markdown table and the engineering lists."""
import json, os, re, sys
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🗑️generated", "oracles")
DIAG = re.compile(r"(integrity|\.ref[.{]|\.ref$|unknown|\.empty|subject\.empty|\.na$|\.na\.|\.na\{|-ref|\.duplicate|\.input\.|\.link\.|\.layer\.model|\.match|\.scope|\.form\.|\.regime|selection|governing\.situation|override|\.members$|\.actions\.\{\}$|dangling|orphan|\.category\.|\.pairs\.na|-alloy|-position|-section|\.pin$|\.grade\.|\.kind\.|\.variable\.|\.material|\.placement|\.vertex|\.weldrun|\.outline|\.props|\.torsion\.\{\}$)")
CAT = re.compile(r"(^|\.)([a-z]{1,4}-\d{2}|[a-z]{1,2}-\d{2,3}|geo-f\d\d|pr-\d\d|dt-\d\d)(\.|$)")
rows, eng = [], {}
for code in ["en%d" % n for n in range(1990, 2000)]:
    g = json.load(open(os.path.join(OUT, f"gap-{code}.json")))
    miss = g["missingNoStatic"]
    d = [m for m in miss if DIAG.search(m)]
    c = [m for m in miss if m not in d and (CAT.search(m) or re.fullmatch(r"[a-z]-\d\d", m))]
    e = [m for m in miss if m not in d and m not in c]
    eng[code] = e
    rows.append((code, g["rust"], g["oracleIds"], len(g["coarse"]), len(g["staticOnly"]), len(miss), len(d), len(c), len(e)))
print("| code | Rust families | oracle ids | coarse | twin without fixture | missing | diagnostics | catalogue case ids | engineering |")
print("|---|---|---|---|---|---|---|---|---|")
for r in rows:
    print("| %s | %d | %d | %d | %d | %d | %d | %d | %d |" % r)
if "--lists" in sys.argv:
    for code, e in eng.items():
        print("\n### %s engineering gaps (%d)\n" % (code, len(e)))
        print(", ".join("`%s`" % x for x in e))
