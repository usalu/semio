import os, re, sys, json
root = "/Users/ueli/Documents/semio"
skip = {"node_modules", ".git", "target", ".🧬semio", ".tmp-ticket", ".tmp-ticket-0918", "dist"}
out = []
for dirpath, dirnames, filenames in os.walk(root):
    dirnames[:] = [d for d in dirnames if d not in skip and not d.startswith(".tmp")]
    parts = dirpath.split("/")
    if len(parts) >= 2 and parts[-2] == "🧪️tests" and "🧫️fixtures" in dirnames and "🥒️.feature" in filenames:
        case = dirpath
        owner = os.path.dirname(os.path.dirname(case))
        name = os.path.basename(case)
        adapters = [f for f in filenames if f != "🥒️.feature"]
        refs = {}
        for f in filenames:
            txt = open(os.path.join(case, f), encoding="utf-8", errors="replace").read()
            refs[f] = {"local": len(re.findall(r"local://", txt)), "fixturesDir": len(re.findall(r"🧫️fixtures", txt)), "shared": len(re.findall(r"shared://", txt))}
        target = os.path.join(owner, "🧫️fixtures", name)
        out.append({"case": os.path.relpath(case, root), "owner": os.path.relpath(owner, root), "targetExists": os.path.exists(target), "refs": refs,
                    "files": sorted(os.path.relpath(os.path.join(dp, fn), os.path.join(case, "🧫️fixtures")) for dp, _, fns in os.walk(os.path.join(case, "🧫️fixtures")) for fn in fns)})
json.dump(out, open(sys.argv[1], "w"), ensure_ascii=False, indent=1)
print(len(out))
for o in out:
    flags = [f"{k}:{v['local']}/{v['fixturesDir']}/{v['shared']}" for k, v in o["refs"].items()]
    print(len(o["files"]), "EXISTS" if o["targetExists"] else "", o["case"], " ".join(flags))
