import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import os, re, shutil
from lib import *

FAMILY_DIR = "🗂️design-cases"
FAMILY = "design-cases"


def toml_for(art, catalogue_deps, desc):
    src = rd(T.mut + "/🗂️design-cases/📦️packages/🦀️rust/Cargo.toml")
    src = src.replace("en1995", art.short)
    src = re.sub(r'name = "[^"]+"', 'name = "%smut-%s"' % (art.crate, FAMILY), src, 1)
    src = re.sub(r'description = "[^"]+"', 'description = "%s"' % desc, src, 1)
    src = re.sub(r"semio-s-artifact-norm-%s-kernel = \{ workspace = true \}\n" % art.short, "".join("%s = { workspace = true }\n" % c for c in catalogue_deps), src)
    return src


def lib_rs(art, leaves, title):
    src = rd(T.mut + "/🗂️design-cases/🦀️.rs")
    head = src[: src.index("//#region 🪢️Mounts")]
    head = re.sub(r"^//! .*\n", "//! " + title + "\n", head, 1)
    body = "//#region 🪢️Mounts\n" + "".join('#[path = "../%s/🦀️.rs"]\npub mod %s;\n' % (d, m) for d, m in sorted(leaves, key=lambda x: x[1])) + "//#endregion 🪢️Mounts\n"
    return head + body


def create(art, leaves, catalogue_deps, title, desc):
    d = art.mut + "/" + FAMILY_DIR
    wr(d + "/🦀️.rs", lib_rs(art, leaves, title))
    wr(d + "/📦️packages/🦀️rust/Cargo.toml", toml_for(art, catalogue_deps, desc))
    return d


def lock_block(name, deps):
    return '[[package]]\nname = "%s"\nversion = "0.1.0"\ndependencies = [\n%s\n]' % (name, "\n".join(' "%s",' % x for x in sorted(deps)))


def register(art, cname, cdir, lock_deps, old=(), anchor=None):
    """Registers crate `cname` (at `cdir`): workspace member and dependency, script.ts, build-owners, Cargo.lock and every manifest that depended on a crate of `old` (or on `anchor`); the crates of `old` are removed everywhere."""
    names_old = list(old)
    rel = os.path.relpath(cdir + "/📦️packages/🦀️rust", art.root)
    f = art.root + "/Cargo.toml"
    s = rd(f)
    for o in names_old:
        s = "".join(l for l in s.splitlines(True) if not (l.startswith('    "') and ("/" + o["dir"] + "/📦️packages") in l))
        s = re.sub(r'\[workspace\.dependencies\.%s\]\npath = "[^"\n]*"\n\n' % re.escape(o["name"]), "", s)
    if '"%s"' % rel not in s:
        s = s.replace("members = [\n", 'members = [\n    "%s",\n' % rel, 1)
    if "[workspace.dependencies.%s]" % cname not in s:
        s = s.replace("[workspace.lints.rust]", '[workspace.dependencies.%s]\npath = "%s"\n\n[workspace.lints.rust]' % (cname, rel), 1)
    wr(f, s)
    f = art.root + "/📦️packages/🦀️rust/📜️script.ts"
    s = rd(f)
    for o in names_old:
        s = s.replace('"%s",' % o["name"], "")
    if '"%s"' % cname not in s:
        s = s.replace('"%smutations",' % art.crate, '"%s","%smutations",' % (cname, art.crate), 1) if ('"%smutations",' % art.crate) in s else s
    wr(f, s)
    s = rd(OWNERS)
    for o in names_old:
        s = re.sub(r'    \{\n      "name": "%s",\n.*?\n    \},\n' % re.escape(o["name"]), "", s, flags=re.S)
    if '"name": "%s"' % cname not in s:
        root = os.path.relpath(cdir + "/📦️packages/🦀️rust", SEMIO)
        ent = '    {\n      "name": "%s",\n      "root": "%s",\n      "manifest": "%s/Cargo.toml",\n      "packages": []\n    },\n' % (cname, root, root)
        m = re.search(r'    \{\n      "name": "%smutations",\n.*?\n    \},\n' % re.escape(art.crate), s, re.S)
        s = s.replace(m.group(0), ent + m.group(0), 1)
    wr(OWNERS, s)
    gone = [o["name"] for o in names_old]
    dep_pat = [o["name"] for o in names_old] + ([anchor] if anchor else [])
    consumers = []
    for dp, dn, fn in os.walk(art.root):
        dn[:] = [x for x in dn if x not in ("target", "node_modules")]
        if "Cargo.toml" in fn:
            p = os.path.join(dp, "Cargo.toml")
            if dp == art.root:
                continue
            t = rd(p)
            if any(("%s = { workspace = true }\n" % n) in t for n in dep_pat) and ('name = "%s"' % cname) not in t:
                consumers.append(p)
    for p in consumers:
        t = rd(p)
        first = min(t.index("%s = { workspace = true }\n" % n) for n in dep_pat if ("%s = { workspace = true }\n" % n) in t)
        for n in gone:
            t = t.replace("%s = { workspace = true }\n" % n, "")
        if "%s = { workspace = true }" % cname not in t:
            line = "%s = { workspace = true }\n" % cname
            anchor_line = ("%s = { workspace = true }\n" % anchor) if anchor and ("%s = { workspace = true }\n" % anchor) in t else None
            if anchor_line:
                t = t.replace(anchor_line, anchor_line + line, 1)
            else:
                t = t[:first] + line + t[first:]
        wr(p, t)
    f = art.root + "/Cargo.lock"
    s = rd(f)
    parts = re.split(r"(?m)^(?=\[\[package\]\]$)", s)
    head, blocks = parts[0], [x.rstrip("\n") for x in parts[1:]]
    nm = lambda b: re.search(r'name = "([^"]+)"', b).group(1)
    blocks = [b for b in blocks if nm(b) not in gone]
    if not any(nm(b) == cname for b in blocks):
        at = next((i for i, b in enumerate(blocks) if nm(b) > cname), len(blocks))
        blocks.insert(at, lock_block(cname, lock_deps))
    out = []
    for b in blocks:
        m = re.search(r"dependencies = \[\n(.*?)\n\]", b, re.S)
        if m and nm(b) != cname:
            deps = [d.strip().strip('",') for d in m.group(1).split("\n")]
            if any(d in gone or d == anchor for d in deps) :
                deps = [d for d in deps if d not in gone]
                if cname not in deps:
                    deps.append(cname)
                deps = sorted(set(deps), key=lambda x: x)
                b = b.replace(m.group(1), "\n".join(' "%s",' % d for d in deps), 1)
        out.append(b)
    wr(f, head + "\n\n".join(out) + "\n")
    return consumers


def drop_dirs(*dirs):
    for d in dirs:
        shutil.rmtree(d)
