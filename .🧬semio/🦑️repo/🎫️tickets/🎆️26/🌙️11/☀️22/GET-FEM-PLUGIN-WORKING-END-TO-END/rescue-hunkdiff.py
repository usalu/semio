import sys, difflib
sys.path.insert(0, ".")
import importlib.util
spec = importlib.util.spec_from_file_location("rh", "rescue-hunks.py"); rh = importlib.util.module_from_spec(spec); spec.loader.exec_module(rh)
data = rh.load()
for i in sys.argv[1].split(","):
    d, p, hs = data[i]
    k = 0
    for h in hs:
        if rh.kind(h) != "other":
            continue
        print(f"=== {i} hunk {k} ({p.split('/')[-2].encode('ascii','replace').decode()})"); k += 1
        for l in difflib.unified_diff(h["new"], h["old"], "new", "old", lineterm="", n=0):
            if l.startswith(("---", "+++", "@@")):
                continue
            print(l[:200])
