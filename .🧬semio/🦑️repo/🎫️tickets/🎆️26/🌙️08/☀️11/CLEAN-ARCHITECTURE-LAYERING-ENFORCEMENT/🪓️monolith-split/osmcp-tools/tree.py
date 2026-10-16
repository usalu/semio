import os, re, sys, json
from rs_lex import strip_code

REPO = os.environ.get("OSMCP_REPO", "/Users/ueli/Documents/semio")
MCP = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp"
LIBROOT = MCP + "/📦️packages/🦀️rust/🦀️.rs"

MOD_RE = re.compile(r'(?<![\w])(pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_]\w*)\s*(;|\{)')
ATTR_BACK = re.compile(r'#\[[^\]]*\]\s*$')

def attrs_before(src, pos):
    """Collect consecutive `#[...]` attributes immediately preceding src[pos]."""
    attrs = []
    end = pos
    while True:
        head = src[:end].rstrip()
        m = ATTR_BACK.search(head[-600:]) if head.endswith("]") else None
        if not m: break
        txt = head[-600:][m.start():]
        attrs.insert(0, txt.strip())
        end = len(head) - len(head[-600:]) + m.start()
    return " ".join(attrs)

PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]')

class File:
    def __init__(self, path, modpath, parent, attrs):
        self.path = path; self.modpath = modpath; self.parent = parent; self.attrs = attrs
        self.src = open(path, encoding="utf-8").read()
        self.code = strip_code(self.src)
        self.children = []

def walk(path, modpath, parent=None, attrs="", seen=None, files=None):
    if files is None: files = {}
    f = File(path, modpath, parent, attrs)
    files[path] = f
    d = os.path.dirname(path)
    # scan the raw source for mod declarations using code (comments blanked) but attributes from src
    code = f.code
    for m in MOD_RE.finditer(code):
        name = m.group(2); term = m.group(3)
        if term != ";": continue
        attrtxt = attrs_before(f.src, m.start())
        pm = PATH_ATTR.search(attrtxt)
        if pm:
            child = os.path.normpath(os.path.join(d, pm.group(1)))
        else:
            cand = [os.path.join(d, name + ".rs"), os.path.join(d, name, "mod.rs")]
            child = next((c for c in cand if os.path.exists(c)), None)
            if child is None:
                continue
        cfg = " ".join(re.findall(r'#\[\s*cfg[^\]]*\]', attrtxt))
        vis = (m.group(1) or "").strip()
        sub = walk(child, modpath + [name], f, cfg, seen, files)
        f.children.append((name, child, cfg, vis))
    return files

def facet_of(path):
    rel = os.path.relpath(path, MCP)
    first = rel.split(os.sep)[0]
    if first.endswith(".rs"): return "ROOT"
    return first

if __name__ == "__main__":
    files = walk(LIBROOT, ["crate"])
    for p, f in sorted(files.items(), key=lambda kv: kv[0]):
        print("::".join(f.modpath), "|", os.path.relpath(p, MCP), "|", f.attrs)
    print(len(files))
