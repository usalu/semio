"""🕸️ Crate graph: prints lib and dev edges of the scene crates from their manifests and checks both graphs (and their union) for cycles."""
import re
from lib import *
from model import *
root = REPO + "/" + SCENE_REL
lib, dev = {}, {}
for k, c in CRATES.items():
    sec = None
    lib[c["pkg"]], dev[c["pkg"]] = set(), set()
    for line in rd(root + "/" + c["node"] + "/📦️packages/🦀️rust/Cargo.toml").split("\n"):
        m = re.match(r"\[(.*)\]$", line.strip())
        if m:
            sec = m.group(1); continue
        m = re.match(r"(semio-framework-ui-scene-[a-z0-9-]+)\s*=", line)
        if m and sec == "dependencies": lib[c["pkg"]].add(m.group(1))
        if m and sec == "dev-dependencies": dev[c["pkg"]].add(m.group(1))
fac = "semio-framework-ui-scene"
lib[fac] = {c["pkg"] for c in CRATES.values()}; dev[fac] = set()
def cyc(g):
    state = {}
    def dfs(n, path):
        state[n] = 1
        for m in g.get(n, ()):
            if state.get(m) == 1: return path + [n, m]
            if state.get(m) is None:
                r = dfs(m, path + [n])
                if r: return r
        state[n] = 2
        return None
    for n in g:
        if state.get(n) is None:
            r = dfs(n, [])
            if r: return r
    return None
both = {n: lib[n] | dev[n] for n in lib}
print("lib cycle:", cyc(lib), "| lib+dev cycle:", cyc(both))
for n in sorted(lib): print(n.replace("semio-framework-ui-scene-", "") or "FACADE", "->", sorted(x.replace("semio-framework-ui-scene-", "") for x in lib[n]), "dev", sorted(x.replace("semio-framework-ui-scene-", "") for x in dev[n]))
