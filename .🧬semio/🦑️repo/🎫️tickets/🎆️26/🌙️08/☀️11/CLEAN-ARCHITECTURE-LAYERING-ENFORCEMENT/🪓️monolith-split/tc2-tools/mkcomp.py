"""🥣 tc2: move test files into a companion `-tests` crate (files stay in place, the companion root mounts them through `#[path]` in the same module nest).
usage: mkcomp.py <spec.json> [--apply]
spec: {"parent": "<package>", "name": "<companion package>", "dir": "<repo-relative companion node dir>", "files": ["<repo-relative test file>", ...],
       "testkit": ["<package>", ...]   # packages whose dependency edge gets features = ["testkit"]
       "dropdev": ["<dev-dep key>", ...]  # dev-dependency keys removed from the parent (only used by the moved files)
       "unglob": true|false}
Creates: <dir>/🦀️.rs, <dir>/📦️packages/🦀️rust/Cargo.toml. Edits: the parent files that mount the moved files (mount lines removed), the moved files (`crate::` -> parent crate ident, run-time fixture paths re-relativised).
Registration (members, owners, script.ts, lock) is done by reg.py."""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, HERE + "/ttc")
from ttclib import *
from ttcstub import mask
import modtree
import fixrt
import unglob

MANS = Manifests(Source(None))
MANS.build_index()
SRC = Source(None)
STD_ROOTS = {"std", "core", "alloc", "self", "super", "crate", "Self", "f32", "f64", "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "str", "bool", "char"}


def abs_(rel):
    return rel if rel.startswith("/") else ROOT + "/" + rel


def rel_(p):
    return os.path.relpath(p, ROOT)


def parent_info(pkg):
    man = MANS.manifest(pkg, SRC)
    mdir = ROOT + "/" + man["dir"]
    lib = ROOT + "/" + MANS.lib_rel(man)
    return man, mdir, lib


def used_roots(texts):
    roots = set()
    for t in texts:
        m = mask(t)
        for x in re.finditer(r"(?<![A-Za-z0-9_.])(?<!::)([a-z_][a-z0-9_]*)::", m):
            roots.add(x.group(1))
        for x in re.finditer(r"#\[\s*([a-z_][a-z0-9_]*)::", m):
            roots.add(x.group(1))
    return roots - STD_ROOTS


def extern_aliases(lib_text):
    out = {}
    for m in re.finditer(r"extern\s+crate\s+([A-Za-z0-9_]+)\s+as\s+([A-Za-z0-9_]+)\s*;", lib_text):
        out[m.group(2)] = m.group(1)
    return out


def ws_dir_of(man, mdir):
    sec = man["sections"].get("package", {})
    w = sec.get("workspace", '"."').strip().strip('"')
    return os.path.normpath(os.path.join(mdir, w))


def ws_deps(wsdir):
    p = wsdir + "/Cargo.toml"
    sec = parse_toml(open(p, encoding="utf-8").read())
    out = {}
    for s, kv in sec.items():
        if s == "workspace.dependencies":
            for k, v in kv.items():
                out[k] = v
        elif s.startswith("workspace.dependencies."):
            out[s[len("workspace.dependencies."):].strip('"')] = "{ table }"
    return out


def plan(spec):
    parent = spec["parent"]
    man, mdir, lib = parent_info(parent)
    comp_dir = abs_(spec["dir"])
    pkg_dir = comp_dir + "/📦️packages/🦀️rust"
    root = comp_dir + "/🦀️.rs"
    mods = modtree.build(lib)
    retr = bool(spec.get("retransform"))
    cmods = modtree.build(root) if retr else None
    files = [abs_(f) for f in spec["files"]]
    by_file = {}
    for m in (cmods if retr else mods):
        if not m.inline:
            by_file.setdefault(os.path.normpath(m.file), []).append(m)
    edits = {}
    parent_ident = ident(parent)
    tree = {}
    mounted = []
    moved_set = {os.path.normpath(f) for f in files}
    descendants = set()
    for f in files:
        ms = by_file.get(os.path.normpath(f))
        if spec.get("post") and not retr:
            continue
        assert ms and len(ms) == 1, ("test file is not mounted exactly once by the parent", f, ms)
        m = ms[0]
        if os.path.normpath(m.parent_file) in moved_set:
            descendants.add(os.path.normpath(f))
            continue
        assert retr or m.test_gated, ("moved root is not cfg(test)-gated", f)
        assert retr or "feature" not in m.attrs, ("moved root is a feature-gated testkit module of the parent", f)
        if not retr:
            mounted.append(m)
        node = tree
        for part in m.chain[:-1]:
            node = node.setdefault(part, {})
        node[m.chain[-1]] = f
    texts = {}
    top_names = set(tree.keys())
    for f in files:
        t = open(f, encoding="utf-8").read()
        texts[f] = t
    new_texts = {}
    manual_inc = []
    all_gaps = []
    super_manual = []
    for f, t in texts.items():
        out = t
        if spec.get("post") and not retr:
            new_texts[f] = out
            continue
        mods_for = [x for x in (cmods if retr else mods) if os.path.normpath(x.file) == os.path.normpath(f) and not x.inline]
        assert len(mods_for) == 1, ("module of moved file", f)
        tm = mods_for[0]
        mchain = list(tm.chain[:-1])
        moved_chains = {tuple(x.chain) for x in (cmods if retr else mods) if not x.inline and os.path.normpath(x.file) in moved_set}
        glob_stmt = re.compile(r"[ \t]*use\s+((?:crate|super|self)(?:::(?:super|[A-Za-z0-9_]+))*)::\*\s*;")
        done = set()
        desc = "\n".join(texts[os.path.normpath(x.file)] if os.path.normpath(x.file) in texts else texts.get(x.file, "") for x in (cmods if retr else mods) if not x.inline and len(x.chain) > len(tm.chain) and x.chain[:len(tm.chain)] == tm.chain and os.path.normpath(x.file) in moved_set)
        for gm in sorted(glob_stmt.finditer(mask(out)), key=lambda x: -x.start()):
            head = gm.group(1).split("::")
            if head[0] == "crate":
                chain = head[1:]
            elif head[0] == "super":
                chain = mchain[:len(mchain) - (len(head) - 1)]
            else:
                chain = mchain + head[1:]
            if tuple(chain) in moved_chains:
                continue
            if head[0] == "crate" and chain != mchain[:len(chain)]:
                for g in unglob.glob_gaps(out[gm.start():gm.end()], mods, f, out + "\n" + desc):
                    all_gaps.append((f,) + g)
                continue
            if tuple(chain) in done:
                lines = []
            else:
                lines, gaps = unglob.unglob(tm, mods, parent_ident, lib, chain, desc)
                done.add(tuple(chain))
                for g in gaps:
                    all_gaps.append((f,) + g)
            out = out[:gm.start()] + "\n".join(lines) + out[gm.end():]
        for x in sorted(re.finditer(r"(?<![A-Za-z0-9_])(?<!::)crate::((?:[A-Za-z0-9_]+::)*[A-Za-z0-9_]+|\{)", mask(out)), key=lambda x: -x.start()):
            tail = x.group(1)
            cm = spec.get("crate_map", {})
            hit = next((k for k in sorted(cm, key=len, reverse=True) if (tail.startswith(k) if k.endswith("::") else (tail == k or tail.startswith(k + "::")))), None)
            if hit:
                out = out[:x.start()] + cm[hit] + out[x.start() + len("crate::") + len(hit):]
                continue
            if tail == "{":
                rep = parent_ident + "::"
            else:
                segs = tail.split("::")
                node, i = tree, 0
                while i < len(segs) and isinstance(node, dict) and segs[i] in node:
                    node = node[segs[i]]
                    i += 1
                keep = i >= 1 and not isinstance(node, dict)
                rep = "crate::" if keep else parent_ident + "::"
            out = out[:x.start()] + rep + out[x.start() + len("crate::"):]
        comp_chains = set(moved_chains)
        def walk(node, pre):
            for k, v in node.items():
                comp_chains.add(tuple(pre + [k]))
                if isinstance(v, dict):
                    walk(v, pre + [k])
        walk(tree, [])
        for x in sorted(re.finditer(r"(?<![A-Za-z0-9_])(?<!::)((?:super::)+)", mask(out)), key=lambda x: -x.start()):
            n = x.group(1).count("super::")
            if tuple(mchain[:max(0, len(mchain) - (n - 1))]) in moved_chains:
                continue
            nx = re.match(r"([A-Za-z_][A-Za-z0-9_]*)", mask(out)[x.end():])
            if nx and tuple(mchain[:max(0, len(mchain) - (n - 1))]) + (nx.group(1),) in comp_chains:
                continue
            base = [parent_ident] + mchain[:max(0, len(mchain) - (n - 1))]
            out = out[:x.start()] + "::".join(base) + "::" + out[x.end():]
        for suffix, extra in spec.get("extra_use", {}).items():
            if f.endswith(suffix):
                head_doc = re.match(r"(?:\s*//![^\n]*\n)*", out).group(0)
                out = head_doc + "".join(f"#[allow(unused_imports)] {l}\n" for l in extra) + out[len(head_doc):]
        out, _count, manual = fixrt.rewrite_text(out, f, pkg_dir, old_manifest_dir=mdir)
        if manual:
            manual_inc.append((f, [out[x[1]:x[1] + 120] for x in manual]))
        new_texts[f] = out
    deps_text = {f: new_texts[f] for f in files}
    lib_text = open(lib, encoding="utf-8").read()
    aliases = extern_aliases(lib_text)
    roots = used_roots(deps_text.values())
    pdeps = MANS.deps(man)
    by_ident = {}
    for kind, name, pkg, feats, optional, renamed in pdeps:
        by_ident.setdefault(ident(name), []).append((kind, name, pkg, feats, optional, renamed))
    wsdir = ws_dir_of(man, mdir)
    wdeps = ws_deps(wsdir)
    spec_by_key = {}
    for kind, name, specstr in dep_sections(man["sections"]):
        spec_by_key.setdefault(name, []).append((kind, specstr))
    needed = {}
    extern_lines = []
    unknown = []
    for r in sorted(roots):
        if r in aliases:
            target = aliases[r]
            extern_lines.append(f"extern crate {target} as {r};")
            r_dep = target
        else:
            r_dep = r
        cands = [k for k in spec_by_key if ident(k) == r_dep]
        if not cands:
            if r == parent_ident:
                continue
            wk = [k for k in wdeps if ident(k) == r_dep]
            if wk:
                needed[wk[0]] = [("dev-dependencies", "{ workspace = true }")]
                continue
            unknown.append(r)
            continue
        needed[cands[0]] = spec_by_key[cands[0]]
    return dict(man=man, mdir=mdir, lib=lib, comp_dir=comp_dir, pkg_dir=pkg_dir, root=root, tree=tree, mounted=mounted, files=files,
                new_texts=new_texts, texts=texts, needed=needed, extern_lines=sorted(set(extern_lines)), unknown=unknown, manual_inc=manual_inc, mods=mods, gaps=all_gaps, super_manual=super_manual, wsdir=wsdir, wdeps=wdeps, parent_ident=parent_ident)


def emit_tree(tree, root_dir, depth=0):
    out = []
    pad = "    " * depth
    for name in sorted(tree):
        v = tree[name]
        if isinstance(v, dict):
            out.append(f"{pad}#[path = \".\"]")
            out.append(f"{pad}pub mod {name} {{")
            out += emit_tree(v, root_dir, depth + 1)
            out.append(f"{pad}}}")
        else:
            out.append(f"{pad}#[path = \"{os.path.relpath(v, root_dir)}\"]")
            out.append(f"{pad}pub mod {name};")
    return out


def render_root(p, spec):
    lines = [f"//! {spec['emoji']} Companion test crate of `{spec['parent']}`: {spec['summary']} The test files stay at their taxonomy nodes and are mounted here in the module nest the parent used.", "", "#![cfg(test)]", ""]
    lines += p["extern_lines"]
    if p["extern_lines"]:
        lines.append("")
    lines += emit_tree(p["tree"], p["comp_dir"])
    return "\n".join(lines) + "\n"


def render_manifest(p, spec):
    man, mdir = p["man"], p["mdir"]
    pkg_dir = p["pkg_dir"]
    wrel = os.path.relpath(p["wsdir"], pkg_dir)
    parent = spec["parent"]
    ptab = MANS.features_table(man)
    testkit = set(x for x in spec.get("testkit", [parent]) if x != parent or "testkit" in ptab)
    deps, devs = [], []

    def rel_path_fix(specstr):
        def sub(m):
            tgt = os.path.normpath(os.path.join(mdir, m.group(1)))
            return 'path = "' + os.path.relpath(tgt, pkg_dir) + '"'
        return re.sub(r'path\s*=\s*"([^"]+)"', sub, specstr)

    def with_testkit(key, specstr):
        pk = key
        mm = re.search(r'package\s*=\s*"([^"]+)"', specstr)
        if mm:
            pk = mm.group(1)
        if pk not in testkit:
            return specstr
        if re.search(r"features\s*=\s*\[", specstr):
            return re.sub(r"features\s*=\s*\[([^\]]*)\]", lambda m: 'features = [' + (m.group(1).strip() + ", " if m.group(1).strip() else "") + '"testkit"]', specstr, count=1)
        if specstr.strip().startswith("{"):
            return specstr.rstrip().rstrip("}").rstrip() + ', features = ["testkit"] }'
        return '{ workspace = true, features = ["testkit"] }'

    tk = ', features = ["testkit"]' if parent in testkit else ""
    if parent in p["wdeps"]:
        devs.append(f'{parent} = {{ workspace = true{tk} }}')
    else:
        devs.append(f'{parent} = {{ path = "{os.path.relpath(p["mdir"], pkg_dir)}"{tk} }}')
    for key, rows in sorted(p["needed"].items()):
        if key == parent:
            continue
        kinds = {k for k, _ in rows}
        specstr = sorted(rows, key=lambda r: 0 if r[0] == "dependencies" else 1)[0][1]
        if len(rows) > 1:
            featrows = [s for _, s in rows if "features" in s]
            if featrows:
                specstr = featrows[0]
        line = f"{key} = {with_testkit(key, rel_path_fix(specstr))}"
        (deps if spec.get("firstparty_normal") and key.startswith("semio-s-artifact") else devs).append(line)
    out = ["[package]", f'workspace = "{wrel}"', f'name = "{spec["name"]}"', "version.workspace = true", "edition.workspace = true", "rust-version.workspace = true",
           f'description = "{spec["description"]}"', "", "[package.metadata.semio]", 'role = "library"', "", "[lints]", "workspace = true", "", "[lib]",
           f'path = "{os.path.relpath(p["root"], pkg_dir)}"']
    if deps:
        out += ["", "[dependencies]"] + deps
    if devs:
        devs.sort()
        out += ["", "[dev-dependencies]"] + devs
    return "\n".join(out) + "\n"


def fix_regions(t):
    lines = t.split("\n")
    stack, drop = [], set()
    for i, l in enumerate(lines):
        m = re.match(r"^\s*//#region\s+(.*)$", l)
        e = re.match(r"^\s*//#endregion\s+(.*)$", l)
        if m:
            stack.append((m.group(1).strip(), i))
        elif e:
            name = e.group(1).strip()
            hit = next((k for k in range(len(stack) - 1, -1, -1) if stack[k][0] == name), None)
            if hit is None:
                drop.add(i)
            else:
                _, start = stack[hit]
                del stack[hit:]
                if all(not x.strip() for x in lines[start + 1:i]):
                    drop.update(range(start, i + 1))
    return "\n".join(l for i, l in enumerate(lines) if i not in drop)


def apply(spec, do=False):
    p = plan(spec)
    root_text = render_root(p, spec)
    man_text = render_manifest(p, spec)
    print("companion root:", rel_(p["root"]), len(rel_(p["root"]).encode()), "bytes")
    print("companion manifest:", rel_(p["pkg_dir"]) + "/Cargo.toml", len((rel_(p["pkg_dir"]) + "/Cargo.toml").encode()), "bytes")
    print("gaps:", len(p["gaps"]), "super manual:", p["super_manual"])
    for g in p["gaps"]:
        print("  GAP", g[0].split("subsets/")[-1][-60:], g[1:])
    print("moved files:", len(p["files"]), "unknown roots:", p["unknown"], "manual include sites:", p["manual_inc"])
    if not do:
        print(root_text)
        print(man_text)
        return p
    os.makedirs(p["pkg_dir"], exist_ok=True)
    open(p["root"], "w", encoding="utf-8").write(root_text)
    open(p["pkg_dir"] + "/Cargo.toml", "w", encoding="utf-8").write(man_text)
    file_edits = {}
    for m in p["mounted"]:
        t = open(m.parent_file, encoding="utf-8").read()
        a, b = m.span
        while a > 0 and t[a - 1] in " \t":
            a -= 1
        e = b
        while e < len(t) and t[e] in " \t":
            e += 1
        if e < len(t) and t[e] == "\n":
            e += 1
        file_edits.setdefault(m.parent_file, []).append((a, e, ""))
    skip = set(spec.get("skip_widen", []))
    wgaps = [g[1:] for g in p["gaps"] if g[1] in ("private-child", "private-item", "private-module")]
    wgaps = [g for g in wgaps if g[4] not in skip]
    for f, ed in unglob.widen_edits(p["mods"], p["lib"], wgaps).items():
        if f == "__missing__":
            print("WIDEN MISSING", ed)
            continue
        file_edits.setdefault(f, []).extend(ed)
    widened = []
    for pf, ed in file_edits.items():
        fixrt.save_orig(pf)
        t = open(pf, encoding="utf-8").read()
        for a, e, rep in sorted(ed, key=lambda x: -x[0]):
            t = t[:a] + rep + t[e:]
            if rep:
                widened.append((pf, a))
        t = fix_regions(t)
        open(pf, "w", encoding="utf-8").write(t)
    p["widened"] = len(widened)
    print("widened items:", len(widened))
    for f in p["files"]:
        if p["new_texts"][f] != p["texts"][f]:
            fixrt.save_orig(f)
            open(f, "w", encoding="utf-8").write(p["new_texts"][f])
    return p


def retransform(spec):
    idx = {}
    for l in open(fixrt.ORIG + "/index.tsv", encoding="utf-8"):
        h, sha, pth = l.rstrip("\n").split("\t")
        idx.setdefault(pth, h)
    files = [abs_(f) for f in spec["files"]]
    for f in files:
        h = idx.get(f)
        if h:
            open(f, "w", encoding="utf-8").write(open(fixrt.ORIG + f"/{h}.rs", encoding="utf-8").read())
        else:
            snap = "/Users/ueli/.cache/tc2/tc2-snap/base/" + os.path.relpath(f, ROOT)
            assert os.path.exists(snap), ("no original and no snapshot for", f)
            open(f, "w", encoding="utf-8").write(open(snap, encoding="utf-8").read())
            fixrt.save_orig(f)
    spec = dict(spec, post=True, retransform=True)
    p = plan(spec)
    for f in files:
        open(f, "w", encoding="utf-8").write(p["new_texts"][f])
    open(p["pkg_dir"] + "/Cargo.toml", "w", encoding="utf-8").write(render_manifest(p, spec))
    skip = set(spec.get("skip_widen", []))
    wgaps = [g[1:] for g in p["gaps"] if g[1] in ("private-child", "private-item", "private-module") and g[5] not in skip]
    n = 0
    for pf, ed in unglob.widen_edits(p["mods"], p["lib"], wgaps).items():
        if pf == "__missing__":
            print("WIDEN MISSING", ed)
            continue
        fixrt.save_orig(pf)
        t = open(pf, encoding="utf-8").read()
        for a, e, rep in sorted(ed, key=lambda x: -x[0]):
            t = t[:a] + rep + t[e:]
            n += 1
        open(pf, "w", encoding="utf-8").write(t)
    print("retransformed", len(files), "files; gaps", len(p["gaps"]), "widened", n, "unknown", p["unknown"])
    for g in p["gaps"]:
        print("  GAP", g[0].split("subsets/")[-1][-60:], g[2:])
    return p


def rerender_manifest(spec):
    spec = dict(spec, post=True)
    p = plan(spec)
    path = p["pkg_dir"] + "/Cargo.toml"
    open(path, "w", encoding="utf-8").write(render_manifest(p, spec))
    return path


if __name__ == "__main__":
    spec = json.load(open(sys.argv[1], encoding="utf-8"))
    if "--retransform" in sys.argv:
        retransform(spec)
    elif "--manifest-only" in sys.argv:
        print(rerender_manifest(spec))
    else:
        apply(spec, "--apply" in sys.argv)
