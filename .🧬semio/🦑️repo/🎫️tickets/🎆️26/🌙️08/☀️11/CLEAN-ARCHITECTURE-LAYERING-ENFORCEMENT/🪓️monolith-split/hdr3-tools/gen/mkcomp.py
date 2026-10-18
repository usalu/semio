import json, os, re, sys
sys.path.insert(0, "/Users/ueli/.cache/hdr3/tools")
from h3reg import *

def closure(root):
    seen, order, stack = set(), [], [root]
    while stack:
        f = stack.pop()
        if f in seen or not os.path.isfile(f): continue
        seen.add(f); order.append(f)
        t = rd(f)
        d = os.path.dirname(f)
        for m in re.finditer(r'include!\(\s*"([^"]+)"\s*\)', t):
            stack.append(os.path.normpath(os.path.join(d, m.group(1))))
        for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\s*\]', t):
            stack.append(os.path.normpath(os.path.join(d, m.group(1))))
    return order

def dep_lines(man):
    mt = rd(man)
    out = {}
    for sec in ("dependencies", "dev-dependencies"):
        m = re.search(r"(?ms)^\[%s\]\n(.*?)(?=^\[|\Z)" % re.escape(sec), mt)
        if not m: continue
        for l in m.group(1).split("\n"):
            mm = re.match(r"^([\w\-]+)(\.workspace)?\s*=", l)
            if mm and l.strip() and not l.startswith("#"):
                out[mm.group(1)] = (sec, l)
    return out

def lib_ident_of(man):
    mt = rd(man)
    m = re.search(r'\[lib\][^\[]*?name = "([^"]+)"', mt)
    if m: return m.group(1)
    return re.search(r'^name = "([^"]+)"', mt, re.M).group(1).replace("-", "_")

def dep_manifest(l, frm, wsd, key):
    pm = re.search(r'path = "([^"]+)"', l)
    if pm:
        return os.path.join(os.path.normpath(os.path.join(frm, pm.group(1))), "Cargo.toml")
    if "workspace" in l:
        wt = rd(wsd + "/Cargo.toml")
        m = re.search(r'\[workspace\.dependencies\.%s\]\n\s*path = "([^"]+)"' % re.escape(key), wt) or re.search(r'(?m)^%s\s*=\s*\{[^}\n]*path = "([^"]+)"' % re.escape(key), wt)
        if m: return os.path.join(os.path.normpath(os.path.join(wsd, m.group(1))), "Cargo.toml")
    return None

def rebase(l, frm, to):
    pm = re.search(r'path = "([^"]+)"', l)
    if not pm: return l
    return l.replace(pm.group(0), 'path = "%s"' % os.path.relpath(os.path.normpath(os.path.join(frm, pm.group(1))), to))

def used_in(nm, text):
    return re.search(r"\b%s\b" % re.escape(nm), text) is not None

def make_companion(owner_name, suffix, root_file, desc, kind="framework", ident=None, route_after=None, extra_dev=(), own_features=("testkit",), dry=False, role=None, tk_in_owner=True, register=True):
    """root_file: repo-absolute file that becomes the lib root (already rewritten). Creates <dir(root_file)>/📦️packages/🦀️rust/Cargo.toml."""
    oman = census_manifest(owner_name)
    opkgdir = os.path.dirname(oman)
    wsd = ws_dir(oman)
    newname = "%s-%s" % (owner_name, suffix)
    newpkgdir = os.path.join(os.path.dirname(root_file), PKG)
    files = closure(root_file)
    text = "".join(rd(f) for f in files)
    deps = dep_lines(oman)
    keep = []
    for key, (sec, l) in deps.items():
        names = {key.replace("-", "_")}
        pk = re.search(r'package = "([^"]+)"', l)
        if pk: names.add(pk.group(1).replace("-", "_"))
        dm = dep_manifest(l, opkgdir, wsd, key)
        if dm and os.path.exists(dm) and not re.search(r'(?<![\w-])package\s*=', l) and "=" in l and key.replace("-", "_") != lib_ident_of(dm):
            pass
        if dm and os.path.exists(dm): names.add(lib_ident_of(dm))
        if any(used_in(n, text) for n in names):
            keep.append(rebase(l, opkgdir, newpkgdir))
    for l in extra_dev: keep.append(l)
    wsrel = os.path.relpath(wsd, newpkgdir)
    libpath = "../../" + os.path.basename(root_file)
    owner_line = '%s = { path = "%s"%s }' % (owner_name, os.path.relpath(opkgdir, newpkgdir), (", features = [%s]" % ", ".join('"%s"' % f for f in own_features)) if own_features else "")
    keep = [l for l in keep if not l.startswith(owner_name + " ") and not l.startswith(owner_name + ".")]
    keep.sort(key=lambda s: s.split(" ")[0].split(".")[0])
    has_project = os.path.exists(opkgdir + "/📋️project.json")
    orole = (re.search(r'role = "([^"]+)"', rd(oman)) or [None, "library"])[1]
    if kind == "framework" and not has_project:
        kind = "plain"
    if kind == "framework":
        ident = ident or newname.replace("semio-framework-", "")
        head = ('[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "%s"\nid = "%s"\n\n[package.metadata.semio.rust-source-boundary]\nmaxSourceBytes = 262144\nmaxSourceFiles = 32\nmaxActiveLines = 4000\n\n[lints]\nworkspace = true\n\n[lib]\npath = "%s"\n\n'
                % (wsrel, newname, desc, role or "framework", ident, libpath))
    else:
        head = ('[package]\nworkspace = "%s"\nname = "%s"\nversion.workspace = true\nedition.workspace = true\nrust-version.workspace = true\ndescription = "%s"\n\n[package.metadata.semio]\nrole = "%s"\n\n[lints]\nworkspace = true\n\n[lib]\npath = "%s"\n\n'
                % (wsrel, newname, desc, role or orole, libpath))
    manifest = head + "[dependencies]\n\n[dev-dependencies]\n" + owner_line + "\n" + "".join(l + "\n" for l in keep)
    print(newname, "files", len(files), "deps", len(keep), "dir", os.path.relpath(newpkgdir, R))
    if dry:
        print(manifest); return
    if register: assert not os.path.exists(newpkgdir + "/Cargo.toml"), "exists"
    os.makedirs(newpkgdir, exist_ok=True)
    open(newpkgdir + "/Cargo.toml", "w", encoding="utf8").write(manifest)
    if not register: return newname, newpkgdir
    add_member(wsd, newpkgdir, opkgdir)
    if kind == "framework":
        project_json(newpkgdir, ident)
        add_owner(newname, newpkgdir, project="@semio-tech/%s-rs" % ident)
        routes(newname, ident, next_route_index(), route_after or owner_name)
    else:
        add_owner(newname, newpkgdir)
    if tk_in_owner:
        mt = rd(oman)
        if not re.search(r"(?m)^testkit\s*=", mt):
            if "[features]" in mt:
                mt = mt.replace("[features]\n", "[features]\ntestkit = []\n", 1)
            else:
                mt = mt.replace("[dependencies]\n", "[features]\ntestkit = []\n\n[dependencies]\n", 1)
            wr(oman, mt)
    return newname, newpkgdir

import usetree as U

def split_doc(body):
    m = re.match(r"((?:\s*//[!/][^\n]*\n)*)", body)
    return m.group(1), body[m.end():]

def companion_root_text(owner_root_text, test_text, owner_ident, testkit=True, extra_header="", keep_doc=True, extra_text="", local_mods=()):
    """New lib root for a moved test file: inner attrs, pruned extern crates, pruned owner-root uses, owner glob and testkit glob."""
    body = test_text
    body = re.sub(r"(?m)^use super::\*;\n", "", body, count=1)
    doc, rest = split_doc(body)
    stmts = U.top_statements(owner_root_text)
    allt = rest + extra_text
    # own-bindings of the test file's explicit uses
    own = set()
    for _, st in U.top_statements(rest):
        fl = U.flatten(st)
        if fl:
            for lf in fl[1]:
                own.add(U.binding(lf))
    uses = U.prune(stmts, allt, own=own, skip_first=set(local_mods) | {'self', 'super'})
    uses = [u.replace("#[cfg(test)] ", "") for u in uses]
    uses = list(dict.fromkeys(uses))
    exts = U.externs(stmts, allt + "".join(uses))
    exts = [e.replace("#[cfg(test)]\n", "") for e in exts]
    head = doc + "#![cfg(test)]\n#![allow(unused_imports, unused_qualifications)]\n\n" + "".join(e + "\n" for e in exts)
    head += "use %s::*;\n" % owner_ident
    if testkit: head += "use %s::testkit::*;\n" % owner_ident
    head += "".join(u + "\n" for u in uses) + extra_header + "\n"
    return head + rest


FIX_RX = re.compile(r'(\b(?:runtime_fixture_\w+\(|fixture_(?:text|path|bytes)!\()\s*)"([^"]+)"')

def rebase_fixtures(files, old_dir, new_dir):
    n = 0
    for f in files:
        s = rd(f)
        def sub(m):
            nonlocal n
            lit = m.group(2)
            new = os.path.relpath(os.path.normpath(os.path.join(old_dir, lit)), new_dir)
            n += 1
            return '%s"%s"' % (m.group(1), new)
        s2 = FIX_RX.sub(sub, s)
        if s2 != s: wr(f, s2)
        for mm in re.finditer(r'CARGO_MANIFEST_DIR|manifest_dir!\(\)|fixture_[a-z]+!\(\s*[^"\s]', s2):
            print("WARN non-literal manifest-relative use in", os.path.relpath(f, R), ":", s2[max(0, mm.start() - 40):mm.end() + 60].replace("\n", " "))
    rebase_laws(files, old_dir, new_dir)
    print("rebased fixture literals:", n)


LAW_RX = re.compile(r'(history_edit_acceptance_law!\((?:[^;]|\n)*?,\s*)"([^"]+)"(\s*\))')

def rebase_laws(files, old_dir, new_dir):
    for f in files:
        s = rd(f)
        def sub(m):
            new = os.path.relpath(os.path.normpath(os.path.join(old_dir, m.group(2))), new_dir)
            print("rebased law path", m.group(2), "->", new)
            return '%s"%s"%s' % (m.group(1), new, m.group(3))
        s2 = LAW_RX.sub(sub, s)
        if s2 != s: wr(f, s2)
