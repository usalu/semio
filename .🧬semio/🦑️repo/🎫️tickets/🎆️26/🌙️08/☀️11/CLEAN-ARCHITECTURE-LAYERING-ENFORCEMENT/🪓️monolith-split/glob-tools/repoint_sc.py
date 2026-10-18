import sys,os,re,json
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import plan
from plan import *
HERE=os.path.dirname(os.path.abspath(__file__))
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
apply_mode = len(sys.argv) > 1 and sys.argv[1] == "apply"
rootp = os.path.join(find(roots["sc"]), "🦀️.rs")
t = tree(rootp)
name2mod = {}
for m, (p, x) in t.items():
    if len(m) != 1: continue
    for it in top_items(x):
        if it["vis"] == "pub" and it["name"] != "*" and it["kind"] != "use":
            name2mod.setdefault(it["name"], set()).add(m[0])
amb = {n: m for n, m in name2mod.items() if len(m) > 1}
assert not amb, amb
name2mod = {n: next(iter(m)) for n, m in name2mod.items()}
PFX = "layout_layout_engine::scene::"
rx_single = re.compile(r"(layout_layout_engine::scene::)(\w+)")
rx_list = re.compile(r"(layout_layout_engine::scene::)\{([^}]*)\}")
files = [l.strip() for l in open(S + "/rsfiles2.txt") if l.strip()]
changed = {}
for f in files:
    p = os.path.join(R, f)
    try: tx = open(p, encoding="utf-8").read()
    except Exception: continue
    if PFX not in tx: continue
    def rep_list(m):
        if "::" in m.group(2): return m.group(0)
        items = [x.strip() for x in m.group(2).split(",") if x.strip()]
        by = {}
        for it in items:
            base = it.split(" as ")[0].strip()
            mod = name2mod[base]
            by.setdefault(mod, []).append(it)
        parts = []
        for mod in sorted(by):
            parts.append(f"{mod}::{{{', '.join(by[mod])}}}" if len(by[mod]) > 1 else f"{mod}::{by[mod][0]}")
        return m.group(1) + "{" + ", ".join(parts) + "}"
    new = rx_list.sub(rep_list, tx)
    def rep_single(m):
        n = m.group(2)
        if n in name2mod.values() or n not in name2mod:
            return m.group(0)
        return m.group(1) + name2mod[n] + "::" + n
    new = rx_single.sub(rep_single, new)
    if new != tx:
        changed[f] = (tx, new)
unmapped = set()
for f, (a, b) in changed.items():
    for m in rx_single.finditer(b):
        if m.group(2) not in name2mod.values(): unmapped.add((f[-60:], m.group(2)))
print(len(changed), "files;", "unmapped:", unmapped)
for f, (a, b) in changed.items():
    print(f[-80:], sum(1 for x, y in zip(a.split("\n"), b.split("\n")) if x != y), "lines")
if apply_mode:
    man = json.load(open(HERE + "/manifest.json", encoding="utf-8"))
    import hashlib
    for f, (a, b) in changed.items():
        bk = "orig/" + hashlib.sha1(f.encode()).hexdigest()[:12] + ".rs"
        if not os.path.exists(os.path.join(HERE, bk)): open(os.path.join(HERE, bk), "w", encoding="utf-8").write(a)
        assert open(os.path.join(R, f), encoding="utf-8").read() == a
        open(os.path.join(R, f), "w", encoding="utf-8").write(b)
        man.setdefault(f, bk)
    json.dump(man, open(HERE + "/manifest.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("applied")
