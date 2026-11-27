"""📋️ The findings of the `🧊️model-catalog` shards of lane MZ (read-only: `🗑️generated/closing/model-catalog-NN.log`): per failed asset its budget and bounds findings in short and the count of every other kind of finding (ids over 64 characters and the like belong to another lane). reds.py [--mine]"""
import glob, os, re, sys

CLOSING = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../🗑️generated/closing")
totals = {"shards": 0, "failed": 0, "budget or bounds": 0}
for path in sorted(glob.glob(os.path.join(CLOSING, "model-catalog-[0-9][0-9].log"))):
    text = open(path, encoding="utf-8", errors="replace").read()
    totals["shards"] += 1
    blocks = re.split(r"\n\s*FAIL\s+\|@teaching/staffage\| \S+ > [^>]+ > ", text)[1:]
    for block in blocks:
        asset = block.split(" > ")[0].strip()
        body = block.split("⎯⎯⎯⎯")[0]
        found = re.findall(r'^\+\s+"((?:budget|bounds|closed|geometry|model|glb|variants|fbx|json|set|determinism) [^"]*)"', body, re.M)
        mine = [f for f in found if f.startswith(("budget", "bounds"))]
        other = {}
        for f in found:
            if not f.startswith(("budget", "bounds")):
                key = re.sub(r"\S*[0-9/:]\S*", "#", f)[:50]
                other[key] = other.get(key, 0) + 1
        totals["failed"] += 1
        totals["budget or bounds"] += bool(mine)
        if "--mine" in sys.argv and not mine:
            continue
        glb = [float(m) for m in re.findall(r"GLB of ([0-9.]+) MB", " ".join(mine))]
        rest = [re.sub(r"^budget \S+ ", "", f) for f in mine if "GLB of" not in f]
        first = "" if found else re.sub(r"\s+", " ", body)[len(asset):len(asset) + 190]
        print(f"{os.path.basename(path)[14:16]} {asset:36} {'GLB %.1f to %.1f MB (%d poses); ' % (min(glb), max(glb), len(glb)) if glb else ''}{'; '.join(rest)}{' | other: ' + ', '.join(f'{n}x {k}' for k, n in other.items()) if other else ''}{first}")
print(totals)
