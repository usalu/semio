import os, re, json, sys
ROOT = "/Users/ueli/Documents/semio"
ART = os.path.join(ROOT, "✏️s/🔌️plugins/📕️norm/🗿️artifacts")
out = {}
for art in sorted(os.listdir(ART)):
    sub = os.path.join(ART, art, "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations")
    if not os.path.isdir(sub): continue
    src = open(os.path.join(sub, "🦀️component.rs"), encoding="utf-8").read()
    m = re.search(r"pub enum (\w+Mutation) \{(.*?)\n\}", src, re.S)
    enum_name, body = m.group(1), m.group(2)
    variants = re.findall(r"^\s{4}(\w+)\(([\w:]+)\),", body, re.M)
    # map payload ident -> kind from leaf dirs
    kindmap = {}
    leafinfo = {}
    for leaf in sorted(os.listdir(sub)):
        mf = os.path.join(sub, leaf, "🦠️mutation/🦀️component.rs")
        if not os.path.isfile(mf): continue
        t = open(mf, encoding="utf-8").read()
        km = re.search(r'SemanticDescriptor \{ verb: "([^"]*)", entity: "([^"]*)", kind: "([^"]*)"', t)
        im = re.search(r"impl\s+(?:\w+::)*MutationKind<[^>]*>\s+for\s+(\w+)", t) or re.search(r"for\s+(\w+)\s*\{", t)
        if not km: continue
        ident = im.group(1) if im else None
        kindmap[ident] = km.group(3)
        tests = os.path.join(sub, leaf, "🧪️tests")
        cases = sorted(os.listdir(tests)) if os.path.isdir(tests) else []
        good = []
        for c in cases:
            d = os.path.join(tests, c)
            need = ["📸️snapshot/⬅️before/🔣️component.json", "📸️snapshot/➡️after/🔣️component.json", "🦠️mutation/🔣️component.json"]
            if all(os.path.isfile(os.path.join(d, n)) for n in need): good.append(c)
        leafinfo[km.group(3)] = {"dir": leaf, "cases": good, "allcases": cases}
    kinds = []
    problems = []
    for ident, path in variants:
        payload = path.split("::")[-1]
        k = kindmap.get(payload)
        if k is None:
            problems.append(f"no leaf for {payload}")
            k = None
        kinds.append({"variant": ident, "payload": payload, "kind": k, "leaf": leafinfo.get(k)})
    out[art] = {"enum": enum_name, "count": len(variants), "kinds": kinds, "problems": problems,
                "leaves_without_variant": [k for k in leafinfo if k not in [x["kind"] for x in kinds]]}
json.dump(out, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
for a, v in out.items():
    miss = [x["variant"] for x in v["kinds"] if x["kind"] is None]
    nofix = [x["kind"] for x in v["kinds"] if x["leaf"] and not x["leaf"]["cases"]]
    print(a, v["count"], "unmapped:", miss, "no-fixture:", nofix, "orphan-leaves:", v["leaves_without_variant"])
