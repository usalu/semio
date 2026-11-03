"""🔎 Per-file name resolution without a compiler: unresolved CamelCase type names (missing imports) and `semio_*::` paths whose crate is not a Cargo dependency."""
import os, re, sys, tomllib
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rs import *

MOD = re.compile(r'(?:#\[path\s*=\s*"([^"]+)"\]\s*)?(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*(;|\{)')
USE = re.compile(r'(?:^|[;{}\s])(?:pub(?:\([^)]*\))?\s+)?use\s+([^;]+);')
DEFN0 = re.compile(r'\b(?:struct|enum|trait|type|union|fn|const|static|mod|macro_rules!)\s+(\w+)')
MACDEF = re.compile(r'\bpub(?:\([^)]*\))?\s+([A-Z]\w*)\b|\b(?:removal|insertion|relocation|modification|name|ident|kind|event|request|response)\s*:\s*([A-Z]\w*)\b')
ENUMB = re.compile(r'\benum\s+\w+\s*(?:<[^>{]*>)?\s*\{')
class _D:
    def findall(self, src):
        out = set(DEFN0.findall(src))
        out |= {a or b for a, b in MACDEF.findall(src)}
        out |= {b_ for _, b_ in re.findall(r'extern\s+crate\s+(\w+)\s+as\s+(\w+)', src)} | set(re.findall(r'extern\s+crate\s+(\w+)\s*;', src))
        for m in ENUMB.finditer(src):
            e = match_close(src, m.end() - 1)
            if e > 0: out |= set(variants_of(src[m.end():e]))
        return out
DEFN = _D()
PRELUDE = set("Self String Vec Option Result Some None Ok Err Box Default Clone Copy Send Sync Sized Drop Fn FnMut FnOnce Iterator IntoIterator Extend From Into TryFrom TryInto ToString ToOwned AsRef AsMut Eq PartialEq Ord PartialOrd Debug Hash Display Result Rc Arc".split())
CRATES = {}

def load_crates():
    for dp, dn, fn in os.walk(NORM):
        dn[:] = [d for d in dn if d not in SKIP]
        if "Cargo.toml" in fn:
            try: t = tomllib.load(open(os.path.join(dp, "Cargo.toml"), "rb"))
            except Exception: continue
            if "package" not in t: continue
            lp = t.get("lib", {}).get("path") or "src/lib.rs"
            root = os.path.normpath(os.path.join(dp, lp))
            deps = {}
            for sec in ("dependencies", "dev-dependencies", "build-dependencies"):
                for k, v in t.get(sec, {}).items():
                    real = v.get("package", k) if isinstance(v, dict) else k
                    deps[k.replace("-", "_")] = real.replace("-", "_")
            lib = t.get("lib", {}).get("name") or t["package"]["name"]
            CRATES[lib.replace("-", "_")] = dict(root=root, deps=deps, dir=dp, name=t["package"]["name"])

_tree = {}
RAW = {}
def tree(root):
    if root in _tree: return _tree[root]
    files, parent = {}, {}
    def walk(f, par):
        if f in files or not os.path.exists(f): return
        raw = open(f, encoding="utf-8").read()
        src = strip(raw)
        files[f] = src; parent[f] = par; RAW[f] = strip(raw, keep_strings=True)
        d = os.path.dirname(f); stem = os.path.splitext(os.path.basename(f))[0]
        for m in MOD.finditer(RAW[f]):
            if m.group(3) != ";": continue
            if m.group(1): c = [os.path.normpath(os.path.join(d, m.group(1)))]
            else:
                c = [os.path.join(d, m.group(2) + ".rs"), os.path.join(d, m.group(2), "mod.rs")]
                if stem not in ("lib", "mod", "🦀️"): c.insert(0, os.path.join(d, stem, m.group(2) + ".rs"))
            for x in c:
                if os.path.exists(x): walk(x, f); break
    walk(root, None)
    _tree[root] = (files, parent)
    return _tree[root]

def use_names(body):
    names, globs = set(), []
    body = re.sub(r'\s+as\s+', '@@', re.sub(r'\s+', ' ', body))
    body = body.replace(' ', '')
    def expand(prefix, b):
        parts = split_top(b)
        for p in parts:
            if p.startswith("{"): expand(prefix, p[1:-1]); continue
            if "{" in p:
                i = p.index("{"); expand(prefix + p[:i], p[i+1:p.rindex("}")]); continue
            full = prefix + p
            if full.endswith("::*") or full == "*": globs.append(full[:-3] if full.endswith("::*") else prefix.rstrip(":"))
            elif "@@" in full:
                names.add(full.split("@@")[-1])
            else:
                last = full.split("::")[-1]
                if last == "self": last = full.split("::")[-2]
                names.add(last)
    expand("", body)
    return names, globs

_def_cache = {}
def crate_names(lib, depth=0):
    if lib in _def_cache: return _def_cache[lib]
    _def_cache[lib] = set()
    c = CRATES.get(lib)
    if not c: return set()
    files, _ = tree(c["root"])
    out = set()
    for f, src in files.items():
        out |= set(DEFN.findall(src))
        for m in USE.finditer(src):
            n, g = use_names(m.group(1)); out |= n
            for gg in g:
                first = gg.split("::")[0]
                if depth < 5 and first in CRATES: out |= crate_names(first, depth + 1)
    _def_cache[lib] = out
    return out

INLINE = re.compile(r'\bmod\s+(\w+)\s*\{')
_sc = {}
def scopes(f, files):
    """Scope list of a file: [(text_without_children, parent_index, line_offsets_text)]; index 0 is the file top scope."""
    if f in _sc: return _sc[f]
    src = files[f]; out = []
    def build(text, parent):
        idx = len(out); out.append(None)
        pieces, pos = [], 0
        kids = []
        for m in INLINE.finditer(text):
            if m.start() < pos: continue
            e = match_close(text, m.end() - 1)
            if e < 0: continue
            pieces.append(text[pos:m.start()] + "mod " + m.group(1) + " ;")
            pieces.append(" " * 0)
            kids.append((m.end(), e))
            pos = e + 1
        pieces.append(text[pos:])
        own = "".join(pieces)
        out[idx] = [own, parent, text]
        for (a_, b_) in kids: build(text[a_:b_], idx)
    build(src, None)
    _sc[f] = out
    return out

def child_of(f, files, parent):
    return {m: c for c, p in parent.items() if p == f for m in [mod_name(c, f, files)] if m}

def mod_name(child, f, files):
    d = os.path.dirname(f)
    for m in MOD.finditer(RAW[f]):
        if m.group(3) != ";": continue
        if m.group(1):
            if os.path.normpath(os.path.join(d, m.group(1))) == child: return m.group(2)
        elif os.path.basename(child) in (m.group(2) + ".rs", "mod.rs") or os.path.basename(os.path.dirname(child)) == m.group(2): return m.group(2)
    return None

USEP = re.compile(r'(?:^|[;{}\s])(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);')
PUBDEF = re.compile(r'\bpub(?:\([^)]*\))?\s+(?:unsafe\s+|async\s+)?(?:struct|enum|trait|type|union|fn|const|static|mod)\s+(\w+)')
_vis = {}
_ENUMV = {}
def enum_variants(name):
    if not _ENUMV:
        for lib, c in CRATES.items():
            files, _ = tree(c["root"])
            for f, src in files.items():
                for m in ENUMB.finditer(src):
                    e = match_close(src, m.end() - 1)
                    nm = re.search(r'enum\s+(\w+)', m.group(0)).group(1)
                    if e > 0: _ENUMV.setdefault(nm, set()).update(variants_of(src[m.end():e]))
    return _ENUMV.get(name, set())

def aliases_of(files):
    out = {}
    for src in files.values():
        for a_, b_ in re.findall(r'extern\s+crate\s+(\w+)\s+as\s+(\w+)', src): out[b_] = a_
        for a_ in re.findall(r'extern\s+crate\s+(\w+)\s*;', src): out[a_] = a_
    return out

def glob_vis(f, i, path, files, parent, lib, depth):
    segs = path.split("::")
    al = aliases_of(files)
    first = segs[0]
    vis, opaque = set(), False
    own, par, _ = scopes(f, files)[i]
    if first in ("std", "core", "alloc"): return vis, True
    if first == "super":
        base = (f, par) if par is not None else ((parent[f], 0) if parent.get(f) else None)
        pub = False
    elif first == "crate": base, pub = (CRATES[lib]["root"], 0), False
    elif first == "self": base, pub = (f, i), False
    elif first in CRATES:
        return crate_names(first), False
    elif first in al and al[first] in CRATES:
        return crate_names(al[first]), False
    elif first in child_of(f, files, parent): base, pub = (child_of(f, files, parent)[first], 0), True
    else:
        ev = enum_variants(first)
        return ev, not ev
    if base is None: return set(), True
    for seg in segs[1:] if first not in child_of(f, files, parent) or True else []:
        pass
    rest = segs[1:] if first not in child_of(f, files, parent) or first in ("super", "crate", "self") else segs[1:]
    for seg in rest:
        if seg == "super":
            bf, bi = base
            _, bpar, _ = scopes(bf, files)[bi]
            base = (bf, bpar) if bpar is not None else ((parent[bf], 0) if parent.get(bf) else None)
            if base is None: return set(), True
            continue
        kids = child_of(base[0], files, parent)
        if base[1] == 0 and seg in kids: base, pub = (kids[seg], 0), True
        elif seg in al and al[seg] in CRATES: return crate_names(al[seg]), False
        else:
            ev = enum_variants(seg)
            return ev, not ev
    return vis_scope(base[0], base[1], files, parent, lib, depth + 1, pub)

def vis_scope(f, i, files, parent, lib, depth=0, pub_only=False):
    key = (f, i, pub_only)
    if key in _vis: return _vis[key]
    _vis[key] = (set(), True)
    own, par, _ = scopes(f, files)[i]
    vis = set(PUBDEF.findall(own)) | {a or b for a, b in MACDEF.findall(own)} if pub_only else set(DEFN.findall(own))
    if pub_only:
        for m in ENUMB.finditer(own):
            e = match_close(own, m.end() - 1)
            if e > 0 and re.search(r'pub[^{]*$', own[max(0, m.start() - 12):m.start() + 6]): vis |= set(variants_of(own[m.end():e]))
    opaque = False
    for m in USEP.finditer(own):
        if pub_only and not m.group(1): continue
        n, g = use_names(m.group(2)); vis |= n
        for gg in g:
            v, o = glob_vis(f, i, gg, files, parent, lib, depth)
            vis |= v; opaque |= o
    _vis[key] = (vis, opaque)
    return vis, opaque

TOK = re.compile(r'(?<![\w:.$#])([A-Z][a-z0-9]\w*)\b')
PATHC = re.compile(r'(?<![\w:])(semio_\w+)::')

def scan_artifact(code):
    adir = ART + "/" + CODE[code]
    out = []
    for lib, c in sorted(CRATES.items()):
        if not c["dir"].startswith(adir): continue
        files, parent = tree(c["root"])
        alld = {k.replace("-", "_") for k in c["deps"]}
        for f, src in files.items():
            for si, (own, par, full) in enumerate(scopes(f, files)):
                body = re.sub(r'#!?\[[^\]]*\]', ' ', own)
                body = re.sub(r'(?:^|(?<=[;{}\s]))(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]+;', lambda m: ' ' * len(m.group(0)), body)
                vis, opaque = vis_scope(f, si, files, parent, lib)
                ext = set(re.findall(r'extern\s+crate\s+(\w+)\s+as\s+(\w+)', src))
                aliases = {b for _, b in ext}; origs = {a for a, _ in ext}
                gens = set()
                for g in set(re.findall(r'<([^<>]*)>', src)):
                    gens |= set(re.findall(r'\b([A-Z]\w*)\s*(?::|,|=|$)', g))
                gens |= set(re.findall(r'\b(?:impl|struct|enum|trait|fn|type)\s*<([A-Z]\w*)', src))
                seen = set()
                for m in TOK.finditer(body):
                    n = m.group(1)
                    if n in PRELUDE or n in vis or n in gens or n in seen: continue
                    if n.isupper() or len(n) < 3: continue
                    seen.add(n)
                    line = src.find(own[max(0, m.start() - 20):m.start() + len(n) + 20].strip()[:30]) 
                    line = src.count("\n", 0, max(line, 0)) + 1
                    out.append((f, line, f"unresolved name `{n}` in {lib}" + (" (opaque globs)" if opaque else "")))
                if si == 0 or True:
                    for m in PATHC.finditer(body):
                        k = m.group(1)
                        if k == lib or k in alld or k in aliases or k in origs: continue
                        out.append((f, 0, f"crate path `{k}::` not a dependency of {lib} ({c['name']})"))
    return out

_all = None
def crate_names_all():
    global _all
    if _all is None:
        _all = set()
        for lib in CRATES: _all |= crate_names(lib)
    return _all

if __name__ == "__main__":
    load_crates()
    res = scan_artifact(sys.argv[1])
    seen = set()
    for f, l, msg in sorted(set(res)):
        key = (f, msg)
        if key in seen: continue
        seen.add(key)
        print(f"{rel(f)}:{l}: {msg}")
    print(f"# {len(seen)} findings", file=sys.stderr)
