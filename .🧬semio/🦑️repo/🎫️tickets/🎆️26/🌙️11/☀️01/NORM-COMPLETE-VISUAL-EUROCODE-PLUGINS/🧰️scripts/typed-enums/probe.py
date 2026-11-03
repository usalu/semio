import sys, importlib.util, os, json, difflib
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("convert", HERE + "/convert.py")
C = importlib.util.module_from_spec(spec); sys.modules["convert"] = C; spec.loader.exec_module(C)
art, spec_path, enum = sys.argv[1:4]
needles = sys.argv[4:]
d = json.load(open(HERE + "/" + spec_path, encoding="utf-8"))
root = C.artifact_root(art)
for c in d["conversions"]:
    if c["enum"] != enum:
        continue
    variants = [C.Variant(v[0], v[1], v[2], v[3]) for v in c["variants"]]
    conv = C.Conversion(art, c["hosts"], c["enum"], variants, c.get("doc", ""), c.get("default"), catalogue=c.get("catalogue"))
    ctx = C.Ctx(root, conv)
    plan = C.Plan(ctx, conv)
    for v in conv.variants:
        conv.legacy.setdefault(plan.tokens[v.name], v.name)
    for p in ctx.files:
        if not any(n in p for n in needles):
            continue
        old = C.read_text(p)
        new = C.apply_rules(ctx, plan, p)
        if new is not None and new != old:
            print("###", ctx.rel(p))
            for l in list(difflib.unified_diff(old.splitlines(), new.splitlines(), lineterm="", n=0))[:12]:
                print(l[:200])
