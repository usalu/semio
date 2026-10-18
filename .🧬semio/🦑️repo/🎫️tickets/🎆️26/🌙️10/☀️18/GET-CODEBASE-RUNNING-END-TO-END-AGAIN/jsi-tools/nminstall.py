"""usage: nminstall.py <package>...  copies packages and their dependency closure from the bun install cache into the scratch node_modules (/Users/ueli/.cache/jsi/nm) and maps them in its tsconfig override."""
import json, os, re, shutil, sys
CACHE = "/Users/ueli/.bun/install/cache"
NM = "/Users/ueli/.cache/jsi/nm/node_modules"
OVERRIDE = "/Users/ueli/.cache/jsi/nm/tsconfig.override.json"

def versions(name):
    if name.startswith("@"):
        scope, bare = name.split("/")
        base = os.path.join(CACHE, scope)
        prefix = bare + "@"
    else:
        base = CACHE
        prefix = name + "@"
    if not os.path.isdir(base):
        return []
    out = []
    for entry in os.listdir(base):
        if entry.startswith(prefix) and entry.endswith("@@@1"):
            v = entry[len(prefix):-4]
            m = re.match(r"^(\d+)\.(\d+)\.(\d+)", v)
            if m and "-" not in v:
                out.append((tuple(int(x) for x in m.groups()), v, os.path.join(base, entry)))
    return sorted(out)

installed = set()
def install(name):
    if name in installed or os.path.isdir(os.path.join(NM, name)):
        installed.add(name)
        return
    vs = versions(name)
    if not vs:
        print("MISSING", name)
        return
    src = vs[-1][2]
    dst = os.path.join(NM, name)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    shutil.copytree(src, dst)
    installed.add(name)
    print("installed", name, vs[-1][1])
    pj = json.load(open(os.path.join(dst, "package.json"), encoding="utf8"))
    for dep in list((pj.get("dependencies") or {}).keys()):
        install(dep)

for name in sys.argv[1:]:
    install(name)

d = json.load(open(OVERRIDE))
import subprocess
for name in sys.argv[1:]:
    try:
        main = subprocess.check_output(["node", "-e", f"process.stdout.write(require.resolve({json.dumps(name)},{{paths:[{json.dumps(NM)}]}}))"], text=True)
        d["compilerOptions"]["paths"][name] = [main]
        print("mapped", name, main[-50:])
    except Exception as e:
        print("unmapped", name, e)
json.dump(d, open(OVERRIDE, "w"))
