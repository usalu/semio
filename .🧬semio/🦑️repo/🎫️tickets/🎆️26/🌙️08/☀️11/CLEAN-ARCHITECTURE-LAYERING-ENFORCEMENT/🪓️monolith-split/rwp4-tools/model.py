# 🧭️ WP4 model: Shell source files, ShellState members, Appendix B assignment, component field table.
import os, re, sys, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *
R = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
PLAN = HERE + "/../PLAN-RENDERER-WGPU-CRATES.md"
ENG = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SH_LIVE = R + "/" + ENG + "/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu"
SH = os.environ.get("RWP4_SRC") or SH_LIVE
T = R + "/" + ENG + "/🎯️targets/🧊️wgpu"
GEN = HERE + "/../🗑️generated"
plan = open(PLAN, encoding="utf-8").read()

def appendix_b():
    sb = plan[plan.index("## Appendix B."):plan.index("## Appendix C.")]
    out = collections.OrderedDict()
    for m in re.finditer(r"\*\*(shell-[a-z-]+)\*\* \((\d+) methods, (\d+) lines\): (.*)", sb):
        out[m.group(1)] = re.findall(r"`([a-z0-9_]+)`", m.group(4))
    return out

def components():
    s73 = plan[plan.index("### 7.3 Components"):plan.index("### 7.4 Method homing")]
    comps = []
    for line in s73.split("\n"):
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) == 4 and cells[0].startswith("`") and cells[1].startswith("`shell-"):
            comps.append(dict(struct=cells[0].strip("`"), crate=cells[1].strip("`"), fields=re.findall(r"`([a-z0-9_]+)`", cells[3])))
    return comps

def shell_files():
    out = []
    for d in sorted(os.listdir(SH)):
        p = SH + "/" + d + "/🦀️.rs" if d != "🦀️.rs" else SH + "/🦀️.rs"
        if os.path.exists(p) and os.path.isfile(p): out.append((d, p))
    for d in ("time-travel",):
        pass
    for dd in ("⏪️time-travel/🗣️labels", "⏪️time-travel/🚦️refusals"):
        p = SH + "/" + dd + "/🦀️.rs"
        if os.path.exists(p): out.append((dd, p))
    return out

if __name__ == "__main__":
    ab = appendix_b()
    comps = components()
    print(len(comps), [c['struct'] for c in comps], sum(len(c['fields']) for c in comps))
    names = collections.Counter()
    for c, ns in ab.items():
        for n in ns: names[n] += 1
    print(sum(len(v) for v in ab.values()), len(names), [n for n, k in names.items() if k > 1])
    memb = collections.defaultdict(list)
    for d, p in shell_files():
        src = Src(open(p, encoding='utf-8').read())
        for it in top_items(src):
            if it['kind'] == 'impl' and re.match(r'impl\s+ShellState\b', it['head']):
                for mem in top_items(src, it['first'] + 1, it['last'], 4):
                    nm = item_name(mem['head'], mem['kind'])
                    memb[nm].append(d)
    dup = {n: v for n, v in memb.items() if len(v) > 1}
    print(len(memb), dup)
    missing = [n for n in memb if n not in names]
    extra = [n for n in names if n not in memb]
    print('not in appendix:', missing, 'in appendix not in code:', extra)
