"""Moves test-tree fixture/schema files to their owner locations and rewrites path literals that resolve to them."""
import os, re, sys, json, shutil, subprocess
root = "/Users/ueli/Documents/semio"
T, F, S, SH = "🧪️tests", "🧫️fixtures", "🧬️schema", "📐️schema"

def plan_for(rel):
    """Maps one flagged repo-relative file to its owner fixture bundle, or None; shapes go to the bundle's 📐️schema."""
    seg = rel.split("/")
    if T in seg:
        i = seg.index(T); owner = seg[:i]; rest = seg[i + 1:]
        if rest[0] in (F, "🧱️fixtures"): return "/".join(owner + [F] + rest[1:])
        if rest[0] == S: return "/".join(owner + [F] + rest[1:-1] + [SH, rest[-1]])
        case = rest[0]; inner = rest[1:]
        if inner and inner[0] == F:
            tail = inner[1:]
            return "/".join(owner + [F] + (tail if len(tail) > 1 else [case] + tail))
        if inner and inner[0] == S:
            tail = inner[1:]
            return "/".join(owner + [F] + (tail[:-1] if len(tail) > 1 else [case]) + [SH, tail[-1]])
        if S in inner:
            j = inner.index(S)
            return "/".join(owner + [F, case] + inner[:j] + [SH] + inner[j + 1:])
        return "/".join(owner + [F, case] + inner)
    if "🧪️fixtures" in seg:
        i = seg.index("🧪️fixtures"); owner = seg[:i]; rest = seg[i + 1:]
        if S in rest:
            j = rest.index(S)
            return "/".join(owner + [F] + rest[:j] + [SH] + rest[j + 1:])
        return "/".join(owner + [F] + rest)
    return None

files = [l.strip() for l in open(sys.argv[1]) if l.strip()]
mapping = {}
for rel in files:
    new = plan_for(rel)
    if new is None: print("NOPLAN", rel); continue
    mapping[rel] = new
for old, new in mapping.items():
    a, b = os.path.join(root, old), os.path.join(root, new)
    if os.path.exists(b):
        if open(a, "rb").read() == open(b, "rb").read(): os.remove(a); print("DEDUP", old); continue
        print("COLLIDE", old, "->", new); continue
    os.makedirs(os.path.dirname(b), exist_ok=True); shutil.move(a, b); print("MOVE", old, "->", new)
for old in mapping:
    d = os.path.dirname(os.path.join(root, old))
    while d.startswith(root + "/") and os.path.isdir(d) and not os.listdir(d): os.rmdir(d); d = os.path.dirname(d)

absmap = {os.path.normpath(os.path.join(root, o)): os.path.normpath(os.path.join(root, n)) for o, n in mapping.items()}
owners = sorted({o.split("/" + T + "/")[0] if "/" + T + "/" in o else o.split("/🧪️fixtures/")[0] for o in mapping})
cands = set()
for marker in sorted({os.path.basename(os.path.dirname(o)) for o in mapping} | {F, S, "🧪️fixtures", "🧱️fixtures"}):
    out = subprocess.run(["git", "grep", "--untracked", "-l", "-F", marker, "--", *owners], cwd=root, capture_output=True, text=True).stdout
    cands.update(l for l in out.splitlines() if l)
extra = subprocess.run(["git", "grep", "--untracked", "-l", "-F", "🔨️modules/🖱️ui/🧪️fixtures", "--", "🧰️framework", "✏️s"], cwd=root, capture_output=True, text=True).stdout
cands.update(l for l in extra.splitlines() if l)
tok = re.compile(r"[^\s\"'`()<>,;{}\[\]]+")
for c in sorted(cands):
    p = os.path.join(root, c)
    if not os.path.isfile(p) or c.endswith(".md"): continue
    try: text = open(p, encoding="utf-8").read()
    except UnicodeDecodeError: continue
    base = os.path.dirname(p); changed = 0
    def sub(m):
        global changed
        t = m.group(0)
        for anchor, style in ((base, "rel"), (root, "repo")):
            full = os.path.normpath(os.path.join(anchor, t))
            if full in absmap:
                new = absmap[full]
                r = os.path.relpath(new, base) if style == "rel" else os.path.relpath(new, root)
                if style == "rel" and t.startswith("./") and not r.startswith("."): r = "./" + r
                return r
        return t
    new = tok.sub(sub, text)
    if new != text: open(p, "w", encoding="utf-8").write(new); print("REWROTE", c)
