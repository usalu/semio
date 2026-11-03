import sys, importlib.util, os
HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("convert", HERE + "/convert.py")
C = importlib.util.module_from_spec(spec); sys.modules["convert"] = C; spec.loader.exec_module(C)
art = sys.argv[1]
root = C.artifact_root(art)
conv = C.Conversion(art, [], "X", [])
ctx = C.Ctx(root, conv)
for hf in sys.argv[2:]:
    host, field = hf.split(".")
    site = C.Site(ctx, host, field)
    vals = C.collect_values(ctx, site)
    print(hf, "props", len(site.props), "patches", {k: v for k, v in site.patches.items()})
    for (kind, v), n in sorted(vals.items()):
        print("   ", kind, repr(v), n)
