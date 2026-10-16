# 🧭️ Method model of the Shell: every `impl ShellState` method with its file, span, Appendix B home crate and WP5 relevance.
import os, re, json, collections, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import shellsrc as S

def appendix_b():
    text = open(S.PLAN, encoding="utf-8").read()
    sb = text[text.index("## Appendix B."):text.index("## Appendix C.")]
    home = collections.defaultdict(list); counts = {}
    for m in re.finditer(r"\*\*(shell-[a-z-]+)\*\* \((\d+) methods, (\d+) lines\): (.*)", sb):
        crate = m.group(1); counts[crate] = int(m.group(2))
        for n in re.findall(r"`([a-z0-9_]+)`", m.group(4)): home[n].append(crate)
    return home, counts

def components():
    text = open(S.PLAN, encoding="utf-8").read()
    s73 = text[text.index("### 7.3 Components"):text.index("### 7.4 Method homing")]
    comp = []; fields = {}
    for line in s73.split("\n"):
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) == 4 and cells[0].startswith("`") and cells[1].startswith("`shell-"):
            c = dict(struct=cells[0].strip("`"), crate=cells[1].strip("`"), fields=re.findall(r"`([a-z0-9_]+)`", cells[3]))
            comp.append(c)
            for f in c["fields"]: fields[f] = c["struct"]
    return comp, fields

def scan(root=S.SH):
    out = []
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in ("🧪️tests", "node_modules")]
        for f in fn:
            if f != "🦀️.rs": continue
            p = os.path.join(dp, f); raw = open(p, encoding="utf-8").read()
            for m in S.methods(raw):
                m["file"] = os.path.relpath(p, root); out.append(m)
    return out

if __name__ == "__main__":
    home, counts = appendix_b()
    comp, fields = components()
    ms = scan()
    print(len(ms), "methods;", len(home), "homed names;", len(fields), "fields")
    byname = collections.defaultdict(list)
    for m in ms: byname[m["name"]].append(m)
    amb = {n: v for n, v in byname.items() if len(v) > 1}
    for n, v in sorted(amb.items()):
        print("DUP", n, [(x["file"][:12], home.get(n)) for x in v])
    unh = [m["name"] for m in ms if m["name"] not in home]
    print("unhomed", len(unh), unh[:50])
    extra = [n for n in home if n not in byname]
    print("homed but not found", extra)
    WP5 = ["shell-session", "shell-window-actions", "shell-actions", "shell-pointer", "shell-view-paint", "shell-view-chrome", "shell-view-overlays", "shell-view-panels"]
    tot = collections.Counter(); lines = collections.Counter()
    for m in ms:
        h = home.get(m["name"], ["?"])
        if len(h) == 1 or True:
            for c in h[:1]:
                tot[c] += 1; lines[c] += m["end"] - m["start"] + 1
    for c in sorted(tot): print(c, tot[c], lines[c])
