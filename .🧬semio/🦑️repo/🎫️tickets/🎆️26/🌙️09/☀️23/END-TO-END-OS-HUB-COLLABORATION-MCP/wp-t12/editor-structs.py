"""📐️ Prints, per editor vocabulary, the snapshot struct, the aggregate enum attributes and each leaf payload struct,
so the specification vectors can be authored by hand against the real field shapes."""
import json, os, re
root = "/Users/ueli/Documents/semio/"
survey = json.load(open(root + ".tmp-ticket/wp-t12/generated/editor-survey.json", encoding="utf-8"))
def find_struct(owner, name):
    for dp, ds, fs in os.walk(root + owner):
        ds[:] = [d for d in ds if d not in ("🧪️tests", "target")]
        for f in fs:
            if not f.endswith(".rs"): continue
            t = open(os.path.join(dp, f), encoding="utf-8").read()
            m = re.search(r"((?:#\[[^\n]*\]\n)*)pub (?:struct|enum) " + name + r"\b[^{;]*(\{.*?\n\}|;)", t, re.S)
            if m: return os.path.join(dp, f)[len(root):], m.group(0)
    return None, None
for i, v in enumerate(survey):
    print(f"==== [{i}] {v['artifact']} {v['surface']}  S={v['snapshot']} M={v['aggregate']}")
    base = v["subsetDir"]
    for name in [v["snapshot"], v["aggregate"]]:
        path, src = find_struct(v["owner"], name)
        if src is None: path, src = find_struct(base, name)
        print(f"--- {name} @ {path}\n{src}")
    for l in v["leaves"]:
        print(f"--- leaf {l['kind']} ({l['variant']}) outcomes={l['outcomes']}\n{l['struct']}")
