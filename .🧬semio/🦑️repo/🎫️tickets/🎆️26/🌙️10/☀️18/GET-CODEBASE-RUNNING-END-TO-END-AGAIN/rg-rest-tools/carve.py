"""Carve inline-module subtrees of one crate root file into a new crate that keeps the module path (rg-rest).
usage: carve.py <spec.json> [--apply]   (default dry run: analysis only)
spec: ws, old_name, old_pkg (dir with Cargo.toml), root_file (file holding the inline modules; default = lib root),
      moves [ "a::b", ...] (inline/decl module paths inside root_file), crate_prefix ("" when root_file is the crate root or a flattened mirror),
      dups [{in:"a::b", match:"substr"}] (non-mod chunks copied into the new shell, kept in old),
      new {name, dir, emoji, doc, desc}, subtree (repo-rel dir to search importers; default whole repo), script, aliases [extra idents for old crate]"""
import sys, os, re, json, subprocess, copy
RT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sys.path.insert(0, RT + "/rgplug-tools"); sys.path.insert(0, RT + "/rg-rest-tools")
import rgp
from rgp import *
rgp.BK = RT + "/🗑️generated/rg-rest/backup"
from modtree import parse_chunk, attr_path, walk_src, crate_root, Node

class N:
    pass

def ws_add_member2(ws, member_rel):
    t = rd(ws)
    m = re.search(r'(?ms)^members\s*=\s*\[\n(.*?)\n\]', t)
    rows = m.group(1).split("\n")
    ind = re.match(r'[ \t]*', rows[0]).group(0) if rows else "  "
    entry = '%s"%s",' % (ind, member_rel)
    if entry in rows:
        return False
    keys = [r.strip().strip(",").strip('"') for r in rows]
    pos = len(rows)
    for i, k in enumerate(keys):
        if k > member_rel:
            pos = i
            break
    rows.insert(pos, entry)
    wr(ws, t[:m.start(1)] + "\n".join(rows) + t[m.end(1):])
    return True

def script_add2(script, old, new):
    t = rd(script)
    if '"%s"' % new in t:
        return False
    if re.search(r'(?m)^[ \t]+"%s",\s*$' % re.escape(old), t):
        return script_add(script, old, new)
    if '"%s"' % old not in t:
        return False
    m = re.search(r'"%s"(,?)' % re.escape(old), t)
    ins = '"%s", "%s"' % (old, new) if m.group(1) else '"%s", "%s"' % (old, new)
    t = t[:m.start()] + ('"%s", "%s"%s' % (old, new, m.group(1))) + t[m.end():]
    wr(script, t)
    return True

def rebase_manifest2(text, old_pkg, new_pkg):
    oa = os.path.join(ROOT, old_pkg); na = os.path.join(ROOT, new_pkg)
    def f(m):
        x = m.group(2)
        ab = os.path.normpath(os.path.join(oa, x))
        return m.group(1) + os.path.relpath(ab, na) + '"'
    text = re.sub(r'(?m)^(workspace = ")(\.\.[^"]*)"', f, text)
    text = re.sub(r'(path = ")(\.\.[^"]*)"', f, text)
    return text

IMPLICIT = [
    (r'value_derive|ToValue|FromValue|RetireOwned|CanonicalJsonTree', ["semio-framework-value"]),
    (r'RetireOwned', ["semio-framework-value-retirement"]),
    (r'DslRecord|DslEnum|dsl_record_derive', ["semio-framework-dsl-record", "semio-framework-value"]),
    (r'MutationLeaf|Mutations\b', ["semio-framework-os-kernel"]),
    (r'dsl::Mutations', ["semio-framework-os-command", "semio-framework-schema-state", "semio-framework-ui-locale"]),
]

def prune_deps(manifest_text, text, keep=()):
    out = []; section = None
    for line in manifest_text.split("\n"):
        m = re.match(r'\[([^\]]+)\]\s*$', line)
        if m:
            section = m.group(1)
            out.append(line); continue
        if section in ("dependencies", "dev-dependencies", "build-dependencies"):
            km = re.match(r'([A-Za-z0-9_\-]+)(?:\.workspace)?\s*=', line)
            if km:
                key = km.group(1)
                if key not in keep and not re.search(r'(?<![A-Za-z0-9_])' + re.escape(key.replace("-", "_")) + r'(?![A-Za-z0-9_])', text):
                    continue
        out.append(line)
    return "\n".join(out)

def prune_targets(manifest_text, text):
    out = []; section = None; buf = []
    def flush():
        if section is None:
            return
    parts = re.split(r'(?m)^(?=\[)', manifest_text)
    kept = []
    for part in parts:
        mh = re.match(r'\[(target\.[^\]]+\.(?:dev-|build-)?dependencies)\]', part)
        if mh:
            lines = part.split("\n")
            body = [l for l in lines[1:] if l.strip()]
            nb = []
            for l in lines[1:]:
                km = re.match(r'([A-Za-z0-9_\-]+)\s*(?:=|\.)', l)
                if km and not re.search(r'(?<![A-Za-z0-9_])' + re.escape(km.group(1).replace("-", "_")) + r'(?![A-Za-z0-9_])', text):
                    continue
                nb.append(l)
            if not any(l.strip() for l in nb):
                continue
            part = lines[0] + "\n" + "\n".join(nb)
        kept.append(part)
    return "".join(kept)

def prune_new(manifest_text, text):
    keep = set()
    for pat, ks in IMPLICIT:
        if re.search(pat, text):
            keep.update(ks)
    out = prune_deps(manifest_text, text, keep=tuple(keep))
    out = prune_targets(out, text)
    if not re.search(r'#\[(?:cfg\(test\)|test|tokio::test)\]', text):
        out = re.sub(r'(?ms)^\[dev-dependencies\]\n.*?(?=^\[|\Z)', '', out)
        out = out.rstrip("\n") + "\n"
    return out

def parse_nodes(src):
    out = []
    for ch in top_items(src):
        n = N(); n.text = ch; n.children = None; n.head = n.tail = None; n.name = None; n.kind = None
        attrs, kind, name, body = parse_chunk(ch)
        n.attrs = attrs; n.kind = kind; n.name = name
        if kind == "mod" and body is not None:
            m = re.search(r'(?m)^[ \t]*(?:pub(?:\([^)]*\))?\s+)?mod\s+' + re.escape(name) + r'\s*\{', ch)
            o = m.end() - 1
            c = block_bounds(ch, o) - 1
            n.head = ch[:o + 1]; n.tail = ch[c:]
            n.children = parse_nodes(ch[o + 1:c])
        out.append(n)
    return out

def render(n):
    if n.children is None:
        return n.text
    inner = "".join(render(c) for c in n.children)
    if getattr(n, "modified", False):
        ms = list(re.finditer(r'(?m)^([ \t]*)(?:pub(?:\([^)]*\))?\s+)?mod\b', n.head))
        ind = ms[-1].group(1) if ms else ""
        return n.head.rstrip() + "\n" + inner.strip("\n").rstrip() + "\n" + ind + n.tail.lstrip(" \t")
    return n.head + inner + n.tail

def find(nodes, path):
    cur = nodes
    node = None
    for seg in path:
        node = next((c for c in cur if c.kind == "mod" and c.name == seg), None)
        if node is None:
            raise KeyError("::".join(path))
        cur = node.children or []
    return node

def has_selected(n, sel, prefix):
    p = prefix + (n.name,)
    if p in sel:
        return True
    return any(c.kind == "mod" and has_selected(c, sel, p) for c in (n.children or []))

def shell(nodes, sel, prefix, dups):
    out = []
    key = "::".join(prefix)
    for c in nodes:
        if c.kind == "mod":
            p = prefix + (c.name,)
            if p in sel:
                out.append(c)
            elif has_selected(c, sel, prefix):
                s = N(); s.__dict__.update(c.__dict__)
                s.modified = True
                s.head = re.sub(r'(?m)^[ \t]*//![^\n]*\n(?:[ \t]*\n)*', '', s.head)
                s.children = shell(c.children or [], sel, p, dups)
                out.append(s)
        else:
            for d in dups:
                if d["in"] == key and d["match"] in c.text:
                    out.append(c); break
    return out

def remove_selected(nodes, sel, prefix):
    out = []
    for c in nodes:
        if c.kind == "mod":
            p = prefix + (c.name,)
            if p in sel:
                continue
            if c.children is not None and has_selected(c, sel, prefix):
                c.children = remove_selected(c.children, sel, p)
                c.modified = True
                if all(not x.text.strip() for x in c.children):
                    continue
        out.append(c)
    return out

def child_dir(n, pdir):
    p = attr_path(n.attrs)
    if p == ".":
        return pdir
    if p:
        return os.path.normpath(os.path.join(pdir, p))
    return os.path.join(pdir, n.name)

def rebase(nodes, old_dir, new_dir):
    for n in nodes:
        if n.kind != "mod":
            continue
        p = attr_path(n.attrs)
        if n.children is None:
            if p is None:
                raise ValueError("decl without #[path]: " + n.name)
            res = os.path.normpath(os.path.join(old_dir, p))
            np_ = os.path.relpath(res, new_dir)
            n.text = re.sub(r'(#\[path\s*=\s*")([^"]*)(")', lambda m: m.group(1) + np_ + m.group(3), n.text, count=1)
        else:
            if p == ".":
                nd = new_dir; od = old_dir
            elif p:
                od = os.path.normpath(os.path.join(old_dir, p))
                np_ = os.path.relpath(od, new_dir)
                n.head = re.sub(r'(#\[path\s*=\s*")([^"]*)(")', lambda m: m.group(1) + np_ + m.group(3), n.head, count=1)
                nd = os.path.normpath(os.path.join(new_dir, np_))
            else:
                od = os.path.join(old_dir, n.name); nd = os.path.join(new_dir, n.name)
            rebase(n.children, od, nd)

def collect_files(n, old_dir, root_file):
    """all files mounted under node n (recursively) -> list of abs paths"""
    nodes = []
    src = render(n)
    walk_src(src, root_file, old_dir, (), nodes, ())
    return [x.file for x in nodes if x.file]

def node_dir_of(path, tree, root_dir):
    cur = tree; d = root_dir
    for seg in path[:-1]:
        nd = next(c for c in cur if c.kind == "mod" and c.name == seg)
        d = child_dir(nd, d)
        cur = nd.children
    return d

def main():
    spec = json.load(open(sys.argv[1]))
    apply_ = "--apply" in sys.argv
    old_pkg = spec["old_pkg"]; old_manifest_rel = old_pkg + "/Cargo.toml"
    root_abs = crate_root(old_manifest_rel)
    root_file_abs = (ROOT + "/" + spec["root_file"]) if spec.get("root_file") else root_abs
    root_dir = os.path.dirname(root_file_abs)
    src = open(root_file_abs, encoding="utf8").read()
    tree = parse_nodes(src)
    moves = [tuple(m.split("::")) for m in spec["moves"]]
    sel = set(moves)
    dups = spec.get("dups", [])
    prefix = tuple(spec.get("crate_prefix", "").split("::")) if spec.get("crate_prefix") else ()
    new = spec["new"]
    ndir = new["dir"]; npkg = ndir + "/📦️packages/🦀️rust"
    new_root_dir = ROOT + "/" + ndir
    new_ident = cargo_ident(new["name"]); old_ident = cargo_ident(spec["old_name"])
    moved_nodes = [find(tree, m) for m in moves]
    for m, n in zip(moves, moved_nodes):
        pass
    moved_files = []
    for m, n in zip(moves, moved_nodes):
        od = node_dir_of(m, tree, root_dir)
        moved_files += collect_files(n, od, root_file_abs)
        moved_files.append(None)
    extras = spec.get("extra_decls", [])
    extra_nodes_text = ""
    for ex in extras:
        fa = os.path.join(ROOT, ex["file"])
        ex["_abs"] = fa
        moved_files.append(fa)
        sub = []
        walk_src(open(fa, encoding="utf8").read(), fa, os.path.dirname(fa), (), sub, ())
        moved_files += [x.file for x in sub if x.file]
    moved_files = [f for f in moved_files if f]
    mtext = "\n".join(render(n) for n in moved_nodes) + "\n" + "\n".join(open(f, encoding="utf8").read() for f in moved_files if os.path.exists(f))
    mpaths = ["::".join(prefix + m) for m in moves] + ["::".join(prefix + tuple(ex["in"].split("::")) + (ex["name"],)) for ex in extras]
    back = {}
    for f in moved_files:
        t = open(f, encoding="utf8").read() if os.path.exists(f) else ""
        for mm in re.finditer(r'\bcrate::([A-Za-z0-9_]+(?:::[A-Za-z0-9_]+)*)', t):
            p = mm.group(1)
            if not any(p == mp or p.startswith(mp + "::") for mp in mpaths):
                back.setdefault(os.path.relpath(f, ROOT)[-70:], set()).add("crate::" + "::".join(p.split("::")[:6]))
        for mm in re.finditer(r'\bcrate::\{([^}]*)\}', t):
            back.setdefault(os.path.relpath(f, ROOT)[-70:], set()).add("crate::{" + mm.group(1).strip()[:60] + "}")
        if re.search(r'\bsuper::super::super::', t):
            back.setdefault(os.path.relpath(f, ROOT)[-70:], set()).add("super^3")
    print("moved files:", len(moved_files))
    for f, s in sorted(back.items()):
        print("BACKREF", f, sorted(s)[:8])
    lib_text = open(root_abs, encoding="utf8").read()
    externs = [l for l in lib_text.split("\n") if l.startswith("extern crate ")]
    inner = [l for l in lib_text.split("\n") if l.startswith("#![")]
    used_ext = []
    for l in externs:
        m = re.match(r'extern crate ([A-Za-z0-9_]+)(?: as ([A-Za-z0-9_]+))?;', l)
        alias = m.group(2) or m.group(1)
        if re.search(r'(?<![A-Za-z0-9_])' + re.escape(alias) + r'::', mtext) or re.search(r'::' + re.escape(alias) + r'(?![A-Za-z0-9_])', mtext):
            used_ext.append(l)
    shells = shell(tree, sel, (), dups)
    shells = copy.deepcopy(shells)
    rebase(shells, root_dir, new_root_dir)

    def ensure_chain(nodes, path):
        cur = nodes
        for depth, seg in enumerate(path):
            nd = next((c for c in cur if c.kind == "mod" and c.name == seg and c.children is not None), None)
            if nd is None:
                ind = "    " * depth
                nd = N(); nd.kind = "mod"; nd.name = seg; nd.attrs = ['path = "."']; nd.text = ""
                nd.head = '%s#[path = "."]\n%spub mod %s {\n' % (ind, ind, seg); nd.tail = ind + "}\n"
                nd.children = []; nd.modified = True
                cur.append(nd)
            cur = nd.children
        return cur
    for ex in extras:
        chain = tuple(ex["in"].split("::"))
        kids = ensure_chain(shells, chain)
        ind = "    " * len(chain)
        d = N(); d.kind = "mod"; d.name = ex["name"]; d.children = None; d.attrs = []
        d.text = '%s#[path = "%s"]\n%spub mod %s;\n' % (ind, os.path.relpath(ex["_abs"], new_root_dir), ind, ex["name"])
        kids.append(d)
    for il in spec.get("inject_lines", []):
        chain = tuple(il["in"].split("::")) if il["in"] else ()
        kids = ensure_chain(shells, chain) if chain else shells
        d = N(); d.kind = "other"; d.name = None; d.children = None; d.attrs = []
        d.text = "    " * len(chain) + il["line"] + "\n"
        kids.insert(0, d)
    new_body = "".join(render(n) for n in shells)
    new_src = "//! %s %s\n\n" % (new["emoji"], new["doc"])
    if inner:
        new_src += "\n".join(inner) + "\n\n"
    if used_ext:
        new_src += "\n".join(used_ext) + "\n\n"
    new_src += new_body.lstrip("\n")
    if not new_src.endswith("\n"):
        new_src += "\n"
    # importer rewrite plan
    idents = [old_ident] + spec.get("aliases", [])
    r = subprocess.run(["git", "grep", "--untracked", "-l", "-e", old_ident, "--", spec.get("subtree", ".")], cwd=ROOT, capture_output=True, text=True)
    files = [x for x in r.stdout.split("\n") if x.endswith(".rs")]
    plan = {}
    flags = []
    def conv(text, in_old):
        cnt = 0
        for mp in mpaths:
            segs = mp.split("::")
            pat_ext = r'(?<![A-Za-z0-9_])(' + "|".join(re.escape(i) for i in idents) + r')::' + "::".join(re.escape(s) for s in segs) + r'(?![A-Za-z0-9_])'
            text, c = re.subn(pat_ext, lambda m: new_ident + "::" + "::".join(segs), text)
            cnt += c
            if in_old:
                pat_cr = r'(?<![A-Za-z0-9_])crate::' + "::".join(re.escape(s) for s in segs) + r'(?![A-Za-z0-9_])'
                text, c = re.subn(pat_cr, lambda m: new_ident + "::" + "::".join(segs), text)
                cnt += c
            parent = segs[:-1]
            if parent:
                for i in idents + (["crate"] if in_old else []):
                    if re.search(r'(?<![A-Za-z0-9_])' + re.escape(i) + r'::' + "::".join(re.escape(s) for s in parent) + r'::\{[^}]*\b' + re.escape(segs[-1]) + r'\b', text):
                        flags.append(("nested-use-list", i, mp))
        return text, cnt
    old_files = set()
    cl = rgp.closure(old_manifest_rel)
    for x in cl:
        if x["p"].endswith(".rs"):
            old_files.add(os.path.relpath(x["p"], ROOT) if x["p"].startswith("/") else x["p"])
    moved_rel = set(os.path.relpath(f, ROOT) for f in moved_files)
    texts = {}
    remaining = remove_selected(copy.deepcopy(tree), sel, ())
    remaining_text = "".join(render(n) for n in remaining)
    root_rel = os.path.relpath(root_file_abs, ROOT)
    for f in files + [f for f in old_files if f not in files]:
        if f in moved_rel:
            continue
        try:
            t = remaining_text if f == root_rel else rd(f)
        except Exception:
            continue
        in_old = f in old_files
        nt, c = conv(t, in_old)
        if nt != t or f == root_rel:
            plan[f] = (nt, c)
    print("importer rewrites:", {k[-60:]: v[1] for k, v in plan.items()})
    for fl in flags:
        print("FLAG", fl)
    print("externs used:", used_ext)
    print("new root head:\n" + new_src[:1500])
    if not apply_:
        return
    # ---- apply
    wr(ndir + "/🦀️.rs", new_src)
    for ex in extras:
        pf = ex["from"]; pt = rd(pf)
        assert ex["decl"] in pt, ex
        wr(pf, pt.replace(ex["decl"], "", 1))
    old_manifest = rd(old_manifest_rel)
    depth_old = len(old_pkg.split("/")); depth_new = len(npkg.split("/"))
    mt = rebase_manifest2(old_manifest, old_pkg, npkg)
    mt = manifest_for(mt, new["name"], new["desc"], "../../🦀️.rs")
    mt = re.sub(r'(?ms)^\[\[(?:test|bench|example|bin)\]\]\n.*?(?=^\[|\Z)', '', mt)
    mt = re.sub(r'(?ms)^\[features\]\n.*?(?=^\[|\Z)', '', mt).rstrip("\n") + "\n"
    wr(npkg + "/Cargo.toml", mt)
    wsdir = os.path.dirname(spec["ws"])
    rel = lambda p: p[len(wsdir) + 1:] if wsdir else p
    ws_add_member2(spec["ws"], rel(npkg))
    ws_add_dep(spec["ws"], new["name"], rel(npkg), after=spec["old_name"])
    if spec.get("script"):
        script_add2(spec["script"], spec["old_name"], new["name"])
    # old root
    for f, (nt, c) in plan.items():
        wr(f, nt)
    for f in moved_files:
        if not f.endswith(".rs") or not os.path.exists(f):
            continue
        t0 = open(f, encoding="utf8").read(); t1 = t0
        for rw in spec.get("rewrites", []):
            t1 = re.sub(rw["re"], rw["to"], t1)
        if t1 != t0:
            wr(os.path.relpath(f, ROOT), t1)
    # manifests
    files_ = rgp.closure(npkg + "/Cargo.toml")
    txt = closure_text(files_)
    cur = rd(npkg + "/Cargo.toml")
    pr = prune_new(cur, txt)
    if pr != cur:
        wr(npkg + "/Cargo.toml", pr)
    if re.search(r'(?<![A-Za-z0-9_])' + old_ident + r'(?![A-Za-z0-9_])', txt):
        add_dep_line(npkg + "/Cargo.toml", "dependencies", spec["old_name"], '%s = { path = "%s" }' % (spec["old_name"], os.path.relpath(os.path.join(ROOT, old_pkg), os.path.join(ROOT, npkg))))
    old_txt0 = closure_text(rgp.closure(old_pkg + "/Cargo.toml"))
    cur_old = rd(old_pkg + "/Cargo.toml")
    out_lines = []; sec = None
    for line in cur_old.split("\n"):
        mh = re.match(r'\[([^\]]+)\]\s*$', line)
        if mh:
            sec = mh.group(1)
        mk = re.match(r'([A-Za-z0-9_\-]+)\s*(?:=|\.)', line)
        if sec in ("dependencies", "dev-dependencies") and mk:
            idn = mk.group(1).replace("-", "_")
            if re.search(r'(?<![A-Za-z0-9_])' + re.escape(idn) + r'(?![A-Za-z0-9_])', mtext) and not re.search(r'(?<![A-Za-z0-9_])' + re.escape(idn) + r'(?![A-Za-z0-9_])', old_txt0):
                continue
        out_lines.append(line)
    if "\n".join(out_lines) != cur_old:
        wr(old_pkg + "/Cargo.toml", "\n".join(out_lines))
    # old depends on new when old still names new ident
    old_txt = closure_text(rgp.closure(old_pkg + "/Cargo.toml"))
    if re.search(r'\b' + new_ident + r'\b', old_txt):
        add_dep_line(old_pkg + "/Cargo.toml", "dependencies", new["name"], "%s = { workspace = true }" % new["name"])
    # importer manifests
    mr = subprocess.run(["git", "grep", "--untracked", "-l", "-e", spec["old_name"], "--", "*Cargo.toml"], cwd=ROOT, capture_output=True, text=True)
    skip = {spec["ws"], old_manifest_rel, npkg + "/Cargo.toml"}
    for mf in [x for x in mr.stdout.split("\n") if x.endswith("Cargo.toml") and x not in skip]:
        txt = closure_text(rgp.closure(mf))
        mtx = rd(mf)
        for sec in ("dependencies", "dev-dependencies"):
            b = sec_bounds(mtx, sec)
            if not b:
                continue
            body = mtx[b[0]:b[1]]
            mline = re.search(r'(?m)^' + re.escape(spec["old_name"]) + r'\s*(?:=|\.workspace)[^\n]*', body)
            if mline and re.search(r'\b' + new_ident + r'\b', txt):
                line = mline.group(0)
                if "workspace" in line:
                    nl = "%s = { workspace = true }" % new["name"]
                else:
                    pm = re.search(r'path\s*=\s*"([^"]+)"', line)
                    if pm:
                        np2 = os.path.relpath(os.path.normpath(os.path.join(ROOT, os.path.dirname(mf), pm.group(1))).replace(os.path.join(ROOT, old_pkg), os.path.join(ROOT, npkg)), os.path.join(ROOT, os.path.dirname(mf)))
                        nl = '%s = { path = "%s" }' % (new["name"], np2)
                    else:
                        nl = "%s = { workspace = true }" % new["name"]
                add_dep_line(mf, sec, new["name"], nl)
                mtx = rd(mf)
        if not re.search(r'\b' + old_ident + r'\b', txt):
            for sec in ("dependencies", "dev-dependencies"):
                remove_dep_line(mf, sec, spec["old_name"])
    print(run_owners(spec.get("owners_scope", spec.get("subtree", ".")), True))

main()
