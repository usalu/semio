"""🧰 Shared helpers of the replication carve generator: scratch roots, staging tree, item cutting, module-world crawl and name resolver."""
import os, re, sys, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import items, tokenize
import usetree

SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad/rep4"
BASE = SP + "/base/"
STAGE = SP + "/stage/"
REPO = "/Users/ueli/Documents/semio/"
FW = "🧰️framework/🔨️modules/"
R = FW + "📡️replication/"
FACADE_PKG = R + "📦️packages/🦀️rust"
FACADE_ROOT = FACADE_PKG + "/🦀️.rs"
PACKAGE = "📦️packages/🦀️rust"

EXT_EXPORTS = {
    "semio_framework_replication_crypto": ("RecordHasher", "Signer", "SignatureVerifier"),
}

DEF_KINDS = ("fn", "struct", "enum", "const", "static", "type", "trait", "macro_rules", "union")

def read_base(rel):
    return open(BASE + rel, encoding="utf8").read()

def read_stage_or_base(rel):
    p = STAGE + rel
    return open(p if os.path.exists(p) else BASE + rel, encoding="utf8").read()

def exists_stage_or_base(rel):
    return (os.path.exists(STAGE + rel) and not os.path.exists(STAGE + rel + ".deleted")) or (os.path.exists(BASE + rel) and not os.path.exists(STAGE + rel + ".deleted"))

def write(rel, text):
    p = STAGE + rel
    if os.path.exists(p + ".deleted"): os.remove(p + ".deleted")
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w", encoding="utf8").write(text)

def delete(rel):
    p = STAGE + rel + ".deleted"
    os.makedirs(os.path.dirname(p), exist_ok=True)
    open(p, "w").write("")
    if os.path.exists(STAGE + rel): os.remove(STAGE + rel)

def ids(text):
    return set(t[1] for t in tokenize(text) if t[0] == "id")

def bare_ids(text):
    toks = [t for t in tokenize(text) if t[0] not in ("ws", "comment")]
    out = set()
    for k, (kd, tx, a, b) in enumerate(toks):
        if kd != "id": continue
        if k >= 1 and toks[k - 1][1] == ".": continue
        if k + 1 < len(toks) and toks[k + 1][1] == ":" and not (k + 2 < len(toks) and toks[k + 2][1] == ":") and not (k >= 1 and toks[k - 1][1] == ":"):
            continue
        if k >= 2 and toks[k - 1][1] == ":" and toks[k - 2][1] == ":":
            before = toks[k - 3] if k >= 3 else ("punct", "", 0, 0)
            if before[0] == "id" or before[1] in (">", ")", "]"): continue
        out.add(tx)
    return out

def use_tree_of(it):
    t = it["text"]
    for kd, tx, a, b in tokenize(t):
        if kd == "id" and tx == "use":
            body = t[b:].rstrip()
            if body.endswith(";"): body = body[:-1]
            return re.sub(r"//[^\n]*", "", body)
    raise ValueError("no use keyword")

PATH_RX = re.compile(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]|\binclude!\(\s*"([^"]*\.rs)"\s*\)')

def mount_targets(rel, text=None):
    """Relative repo paths of every file a source file mounts through #[path] or include!."""
    text = text if text is not None else read_stage_or_base(rel)
    out = []
    for m in PATH_RX.finditer(text):
        raw = m.group(1) if m.group(1) is not None else m.group(2)
        if not raw or not raw.endswith(".rs"): continue
        p = os.path.normpath(os.path.join(os.path.dirname(rel), raw))
        out.append(p)
    return out

def crawl(root_rel):
    """Files reachable from a lib root, in discovery order (stage over base)."""
    seen = []; stack = [root_rel]
    while stack:
        rel = stack.pop()
        if rel in seen: continue
        if not exists_stage_or_base(rel): continue
        seen.append(rel)
        for p in mount_targets(rel): stack.append(p)
    return seen

def region_lines(text):
    """[(name, start_line_index, end_line_index)] of top-level (depth 0) regions by marker lines."""
    out = []; stack = []
    L = text.split("\n")
    for i, l in enumerate(L):
        s = l.strip()
        if s.startswith("//#region "):
            stack.append((s[len("//#region "):].strip(), i))
        elif s.startswith("//#endregion "):
            name = s[len("//#endregion "):].strip()
            assert stack and stack[-1][0] == name, ("unbalanced region", name, stack[-1:] if stack else None, i)
            n, a = stack.pop()
            out.append((n, a, i, len(stack)))
    assert not stack, stack
    return out

def region_slice(text, name, nested=False):
    """Text from the `//#region name` line through the `//#endregion name` line inclusive."""
    L = text.split("\n")
    hits = [r for r in region_lines(text) if r[0] == name]
    assert len(hits) == 1, (name, hits)
    n, a, b, d = hits[0]
    return "\n".join(L[a:b + 1])

def cut_region(text, name):
    """Removes a region (marker lines included); returns (rest, region_text)."""
    L = text.split("\n")
    hits = [r for r in region_lines(text) if r[0] == name]
    assert len(hits) == 1, (name, hits)
    n, a, b, d = hits[0]
    reg = "\n".join(L[a:b + 1])
    rest = "\n".join(L[:a] + L[b + 1:])
    return re.sub(r"\n{3,}", "\n\n", rest), reg

def cut_items(text, names, kinds=DEF_KINDS):
    """Removes top-level items by name; returns (remaining_text, [removed texts in file order])."""
    its = items(text)
    byname = {}
    for it in its:
        if it["kind"] in kinds and it["name"] in names:
            assert it["name"] not in byname, ("duplicate", it["name"])
            byname[it["name"]] = it
    missing = set(names) - set(byname)
    assert not missing, ("missing items", missing)
    spans = []
    for it in byname.values():
        a, b = it["start"], it["end"]
        while text[a:b].lstrip().startswith("//#"):
            a = text.index("\n", text.index("//#", a)) + 1
        t = text[a:b]
        assert "//#" not in t, ("region marker inside cut item", it["name"])
        spans.append((a, b, it["name"]))
    spans.sort()
    out = []; last = 0; removed = []
    for a, b, n in spans:
        out.append(text[last:a]); last = b; removed.append(text[a:b])
    out.append(text[last:])
    return re.sub(r"\n{3,}", "\n\n", "".join(out)), removed

def top_defs(text):
    out = {}
    for it in items(text):
        if it["kind"] in DEF_KINDS and it["name"]:
            out[it["name"]] = it
    return out

class Node:
    def __init__(self, key, text, parent=None, name=None, base_dir=None, file=None):
        self.key = key; self.text = text; self.parent = parent; self.name = name
        self.base_dir = base_dir; self.file = file
        self.defs = {}; self.uses = []; self.mods = {}; self.mod_vis = {}

def inline_body(item_text):
    """Body text of an inline `mod name { .. }` item, or None for `mod name;`."""
    toks = [x for x in tokenize(item_text) if x[0] not in ("ws", "comment")]
    k = 0
    while k < len(toks) and not (toks[k][0] == "id" and toks[k][1] == "mod"): k += 1
    k += 2
    if k < len(toks) and toks[k][1] == "{":
        a = toks[k][3]; b = toks[-1][2]
        return item_text[a:b]
    return None

def build_node(key, text, parent, name, base_dir, file, nodes, loader):
    n = Node(key, text, parent, name, base_dir, file)
    nodes[key] = n
    for it in items(text):
        k = it["kind"]
        if k in DEF_KINDS and it["name"]:
            n.defs[it["name"]] = it
        elif k == "use":
            try:
                leaves = usetree.parse(use_tree_of(it))
            except Exception:
                leaves = []
            n.uses.append((bool(it["vis"]), leaves, it))
        elif k == "mod":
            attrs = " ".join(it["attrs"])
            pm = re.search(r'#\[\s*path\s*=\s*"([^"]*)"\s*\]', attrs)
            body = inline_body(it["text"])
            if body is not None:
                child = build_node(key + "::" + it["name"], body, n, it["name"], base_dir, file, nodes, loader)
                n.mods[it["name"]] = child; n.mod_vis[it["name"]] = it["vis"]
            elif pm is not None:
                rel = os.path.normpath(os.path.join(base_dir, pm.group(1)))
                ctext = loader(rel)
                if ctext is None:
                    continue
                child = build_node(rel, ctext, n, it["name"], os.path.dirname(rel), rel, nodes, loader)
                n.mods[it["name"]] = child; n.mod_vis[it["name"]] = it["vis"]
    return n

class World:
    """Module tree of a crate (or of the pre-split replication crate) with a small name resolver."""
    def __init__(self, root_rel, loader=None, staged=True):
        if loader is None:
            loader = (lambda rel: read_stage_or_base(rel) if exists_stage_or_base(rel) else None) if staged else (lambda rel: read_base(rel) if os.path.exists(BASE + rel) else None)
        self.loader = loader
        self.nodes = {}
        self.root = build_node(root_rel, self.loader(root_rel), None, "crate", os.path.dirname(root_rel), root_rel, self.nodes, self.loader)

    def lookup(self, node, name, seen=None, mode=0):
        """mode 0: every item (own module or descendant), 1: crate-visible (`pub*`), 2: exactly `pub`."""
        seen = seen if seen is not None else set()
        k = (node.key, name, mode)
        if k in seen: return None
        seen.add(k)
        def ok(vis):
            return mode == 0 or (mode == 1 and vis.startswith("pub")) or (mode == 2 and vis == "pub")
        if name in node.mods and ok(node.mod_vis.get(name, "")): return ("mod", node.mods[name])
        if name in node.defs and ok(node.defs[name]["vis"]): return ("def", node, name)
        for pub, leaves, it in node.uses:
            if not ok(it["vis"]): continue
            for path, alias, glob in leaves:
                if glob: continue
                if usetree.local_name((path, alias, glob)) == name:
                    r = self.resolve_path(node, path, set())
                    if r is not None: return r
        for pub, leaves, it in node.uses:
            if not ok(it["vis"]): continue
            for path, alias, glob in leaves:
                if not glob: continue
                r = self.resolve_path(node, path, set())
                if r is None: continue
                if r[0] == "ext" and name in EXT_EXPORTS.get(r[1][0], ()):
                    return ("ext", tuple(r[1]) + (name,))
                if r[0] == "mod":
                    gm = mode if mode == 2 else (0 if self.is_within(node, r[1]) else 1)
                    rr = self.lookup(r[1], name, seen, gm)
                    if rr is not None: return rr
        return None

    def is_within(self, node, ancestor):
        n = node
        while n is not None:
            if n is ancestor: return True
            n = n.parent
        return False

    def resolve_path(self, node, segs, seen=None):
        key = (node.key, tuple(segs))
        stack = self.__dict__.setdefault("_stack", set())
        if key in stack: return None
        stack.add(key)
        try:
            return self._resolve_path(node, segs, seen)
        finally:
            stack.discard(key)

    def _resolve_path(self, node, segs, seen=None):
        return self.walk(node, segs, seen)[0]

    def walk(self, node, segs, seen=None, mode=0):
        """Resolves a path; returns (result, consumed segment count). Result stops at the first def or external item."""
        seen = seen if seen is not None else set()
        i = 0
        s0 = segs[0]
        if s0 == "crate": cur = ("mod", self.root); i = 1
        elif s0 == "self": cur = ("mod", node); i = 1
        elif s0 == "super":
            cur = ("mod", node)
            while i < len(segs) and segs[i] == "super":
                p = cur[1].parent
                if p is None: return None, 0
                cur = ("mod", p); i += 1
        else:
            r = self.lookup(node, s0, seen)
            if r is None: return ("ext", tuple(segs)), len(segs)
            cur = r; i = 1
        while i < len(segs):
            if cur[0] != "mod":
                return cur, i
            r = self.lookup(cur[1], segs[i], seen, mode)
            if r is None: return None, i
            cur = r; i += 1
        return cur, i

def file_of(node):
    return node.file
