# 🐚️ Field-touch evidence for the Shell crate DAG: for every ShellState method (Appendix B homing) the components whose fields it reads or writes (table 7.3), so a component crate depends on the lower components its methods take as parameters.
import json, os, re, sys, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
R = "/Users/ueli/Documents/semio"
PLAN = os.path.dirname(os.path.abspath(__file__)) + "/../PLAN-RENDERER-WGPU-CRATES.md"
SH = R + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu"
text = open(PLAN, encoding="utf-8").read()
s73 = text[text.index("### 7.3 Components"):text.index("### 7.4 Method homing")]
fields = {}
order = []
for line in s73.split("\n"):
    cells = [c.strip() for c in line.strip().strip("|").split("|")]
    if len(cells) == 4 and cells[0].startswith("`") and cells[1].startswith("`shell-"):
        comp, crate = cells[0].strip("`"), cells[1].strip("`")
        for f in re.findall(r"`([a-z0-9_]+)`", cells[3]): fields[f] = crate
        if crate not in order: order.append(crate)
print(len(fields), "fields", order, file=sys.stderr)
sb = text[text.index("## Appendix B."):text.index("## Appendix C.")]
home = collections.defaultdict(set)
for m in re.finditer(r"\*\*(shell-[a-z-]+)\*\* \((\d+) methods, (\d+) lines\): (.*)", sb):
    crate = m.group(1)
    for n in re.findall(r"`([a-z0-9_]+)`", m.group(4)): home[n].add(crate)
print(len(home), "homed methods", file=sys.stderr)
def strip(t):
    t = re.sub(r'/\*.*?\*/', ' ', t, flags=re.S)
    t = re.sub(r'(?m)//.*$', '', t)
    return re.sub(r'"(?:[^"\\\n]|\\.)*"', '""', t)
touch = collections.defaultdict(collections.Counter)
nmeth = 0
for dp, dn, fn in os.walk(SH):
    dn[:] = [d for d in dn if d not in ("🧪️tests", "node_modules")]
    for f in fn:
        if f != "🦀️.rs": continue
        raw = open(os.path.join(dp, f), encoding="utf-8").read()
        t = strip(raw)
        for im in re.finditer(r"(?m)^impl(?:<[^>]*>)?\s+ShellState\s*\{", t):
            depth = 0; i = im.end() - 1; start = i
            while i < len(t):
                if t[i] == "{": depth += 1
                elif t[i] == "}":
                    depth -= 1
                    if depth == 0: break
                i += 1
            body = t[start + 1:i]
            for mm in re.finditer(r"(?m)^    (?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:const\s+)?(?:unsafe\s+)?fn\s+([a-z0-9_]+)", body):
                name = mm.group(1)
                j = body.index("{", mm.end()) if "{" in body[mm.end():mm.end() + 600] else mm.end()
                d2 = 0; k = j
                while k < len(body):
                    if body[k] == "{": d2 += 1
                    elif body[k] == "}":
                        d2 -= 1
                        if d2 == 0: break
                    k += 1
                mb = body[j:k]
                nmeth += 1
                comps = set(fields[x] for x in re.findall(r"\bself\.([a-z0-9_]+)", mb) if x in fields)
                for crate in home.get(name, {"?"}):
                    for c in comps: touch[crate][c] += 1
print(nmeth, "methods scanned", file=sys.stderr)
edges = {}
for crate, cnt in touch.items():
    edges[crate] = sorted(c for c in cnt if c != crate)
json.dump(dict(order=order, edges=edges, touch={k: dict(v) for k, v in touch.items()}), open(os.path.dirname(os.path.abspath(__file__)) + "/../🗑️generated/rwp0-shelledges.json", "w"), indent=1, ensure_ascii=False)
rank = {c: i for i, c in enumerate(order)}
for crate in order:
    ups = [c for c in edges.get(crate, []) if rank.get(c, -1) > rank[crate]]
    print(crate, "->", edges.get(crate, []), "UPWARD" if ups else "", ups)
print([c for c in edges if c not in order])
