import re, sys
sys.path.insert(0, ".")
from resolver import *

SERDE_STR = re.compile(r'\b(with|serialize_with|deserialize_with|default|skip_serializing_if|from|into|try_from|crate)\s*=\s*"([A-Za-z_][\w:]*)"')

def src_of(it):
    return it.file.src[it.start:it.end]

def resolve_ix(w, f, segs):
    """Resolve path segs in scope of file f. Returns (atom, ix) for the first segment index that resolves to an atom; ('ext', ix) / ('mod', ix) / None."""
    if segs and segs[0] == "semio_framework_os_mcp" and f.modpath[0] == "integration": segs = ["crate"] + list(segs[1:])
    first = segs[0]
    if first == "crate": cur = [Target("mod", w.root)]; i = 1
    elif first == "self": cur = [Target("mod", f)]; i = 1
    elif first == "super":
        m = f; i = 0
        while i < len(segs) and segs[i] == "super":
            m = m.parent if m.parent is not None else m; i += 1
        cur = [Target("mod", m)]
    else:
        cur = w.lookup(f, first, frm=f)
        i = 1
        if not cur: return ("ext", 0)
        atoms = [t for t in cur if t.kind == "atom"]
        if atoms and not any(t.kind == "mod" for t in cur): return (atoms, 0)
        if atoms: return (atoms, 0)
    while i < len(segs):
        seg = segs[i]
        nxt = []
        for t in cur:
            if t.kind == "mod": nxt += w.lookup(t.obj, seg, frm=f, public_only=True)
            elif t.kind == "atom": nxt.append(t)
        if not nxt: return ("mod", i - 1) if cur and cur[0].kind == "mod" else None
        atoms = [t for t in nxt if t.kind == "atom"]
        if atoms: return (atoms, i)
        cur = nxt; i += 1
    return ("mod", len(segs) - 1)

def scan_item(w, it, prod_only=False):
    """Return list of refs: dict(start,end,segs,atoms(list),ix, follow, kind) for matches in item `it` (offsets relative to it.start)."""
    f = it.file
    text = it.text
    if prod_only:
        from prod import blank_cfg_test
        text = blank_cfg_test(text)
    out = []
    for m in PATHRE.finditer(text):
        p = m.group("p")
        if p.startswith("::"): continue
        if m.start() >= 1 and text[m.start()-1] == "." and not (m.start() >= 2 and text[m.start()-2] == "."): continue
        segs = [s.strip() for s in re.split(r"\s*::\s*", re.sub(r"::\s*<[^<>]*(?:<[^<>]*>[^<>]*)*>\s*(?=::)", "", p)) if s.strip()]
        if not segs: continue
        if segs[0] in KEYWORDS and segs[0] not in ("crate", "self", "super"): continue
        k = m.end()
        while k < len(text) and text[k] in " \t\n": k += 1
        follow = text[k:k+1]
        before = text[max(0, m.start()-4):m.start()].rstrip()
        if before.endswith("fn") and len(segs) == 1: continue
        res = resolve_ix(w, f, segs)
        if res is None: continue
        kind, ix = res
        if isinstance(kind, list):
            atoms = [t.obj for t in kind]
            if len(segs) == 1:
                fns = all(a.kind == "fn" for a in atoms)
                if fns and follow not in ("(", ":", "<", ")", ",", ";", "}", "]", ""): continue
                if follow == ":" and text[k:k+2] != "::": continue
            out.append(dict(start=m.start("p"), end=m.end("p"), segs=segs, atoms=atoms, ix=ix, follow=follow))
        elif kind == "ext":
            out.append(dict(start=m.start("p"), end=m.end("p"), segs=segs, atoms=[], ix=0, follow=follow, ext=segs[0]))
    # serde string paths
    for m in SERDE_STR.finditer(src_of(it)):
        segs = m.group(2).split("::")
        res = resolve_ix(w, f, segs)
        if res and isinstance(res[0], list):
            out.append(dict(start=-1, end=-1, segs=segs, atoms=[t.obj for t in res[0]], ix=res[1], follow="", serde=True))
    return out

if __name__ == "__main__":
    w = World()
    a = [x for x in w.atoms if x.name == "register_inference_tools"][0]
    for r in scan_item(w, a.item)[:20]: print(r["segs"], [x.name for x in r["atoms"]], r["ix"])
