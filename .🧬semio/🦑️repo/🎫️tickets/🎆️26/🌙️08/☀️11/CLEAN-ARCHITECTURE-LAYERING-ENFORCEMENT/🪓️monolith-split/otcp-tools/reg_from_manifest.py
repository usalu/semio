"""Register a companion from its manifest. usage: reg_from_manifest.py <companion pkg dir (repo-rel)> <order_index> [--p-lock NAME --remove DEP[,DEP] --add DEP[,DEP]] [--taxonomy NAME[,NAME]]"""
import json, os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(__file__))
import edit as E

pkgdir = sys.argv[1]
idx = int(sys.argv[2])
args = sys.argv[3:]
def opt(name):
    return args[args.index(name) + 1] if name in args else None
m = E.read(pkgdir + "/Cargo.toml")
name = re.search(r'^name = "([^"]+)"', m, re.M).group(1)
cid = re.search(r'^id = "([^"]+)"', m, re.M).group(1)
sec = re.search(r"^\[dependencies\]\n(.*?)(?=^\[|\Z)", m, re.M | re.S).group(1)
deps = []
for line in sec.splitlines():
    if "=" not in line or line.lstrip().startswith("#"):
        continue
    key = line.split("=", 1)[0].strip()
    pk = re.search(r'package\s*=\s*"([^"]+)"', line)
    deps.append(pk.group(1) if pk else key)
spec = {"crate": name, "project": f"@semio-tech/{cid}-rs", "id": cid, "order_index": idx, "dir": pkgdir, "deps": sorted(set(deps))}
if opt("--p-lock"):
    spec["p_lock"] = {"name": opt("--p-lock"), "remove": [x for x in (opt("--remove") or "").split(",") if x], "add": [x for x in (opt("--add") or "").split(",") if x]}
spec["framework"] = "--framework" in args
spec["both_locks"] = "--both-locks" in args
spec["taxonomy"] = [x for x in (opt("--taxonomy") or "").split(",") if x]
path = E.F + f"/🗑️generated/otcp-reg-{cid}.json"
json.dump(spec, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(subprocess.run([sys.executable, os.path.dirname(__file__) + "/reg_companion.py", path], capture_output=True, text=True, cwd=E.ROOT))
