import sys, re, os, json
sys.path.insert(0, "/Users/ueli/.cache/hdr3/tools")
from mkcomp import *
import argparse
ap = argparse.ArgumentParser()
ap.add_argument("owner"); ap.add_argument("suffix")
ap.add_argument("--mount-file", help="owner lib root (default: lib root), repo-relative")
ap.add_argument("--test-root", required=True, help="repo-relative file that becomes the companion lib root")
ap.add_argument("--kind", default="artifact"); ap.add_argument("--role", default=None)
ap.add_argument("--desc", default=None); ap.add_argument("--route-after", default=None)
ap.add_argument("--no-testkit-mod", action="store_true")
ap.add_argument("--write", action="store_true")
ap.add_argument("--redo", action="store_true", help="regenerate root+manifest of an already registered companion from the backup of the test root")
ap.add_argument("--dev", action="append", default=[])
ap.add_argument("--new-root", help="repo-relative NEW file used as companion root when only some include! files move")
ap.add_argument("--files", help="comma list of include! targets (substring match) to move")
a = ap.parse_args()
oman = census_manifest(a.owner)
oroot = lib_root(oman)
mfile = a.mount_file and os.path.join(R, a.mount_file) or oroot
rt = rd(oroot)
mt = rd(mfile)
troot = os.path.join(R, a.test_root)
rel = os.path.relpath(troot, os.path.dirname(mfile))
ident = lib_ident_of(oman)
mods = re.findall(r'(?m)^(?:#\[cfg[^\n]*\]\n)?#\[path = "[^"]+"\]\n(?:pub )?mod (\w+);', rt)
mods = [x for x in mods if not re.search(r'(?m)^#\[cfg\(test\)\]\n#\[path = "[^"]+"\]\n(?:pub )?mod %s;' % x, rt)]
if a.redo:
    bk = BK + "/" + os.path.relpath(troot, R).replace("/", "__")
    tt = rd(bk)
else:
    tt = rd(troot)
partial = bool(a.files)
if partial:
    assert a.new_root
    nroot = os.path.join(R, a.new_root)
    sel_keys = a.files.split(",")
    inc_lines = [l for l in tt.split("\n") if re.match(r'\s*include!\(\s*"[^"]+"\s*\);', l)]
    sel = [l for l in inc_lines if any(k in l for k in sel_keys)]
    assert sel, "no include selected"
    keep_lines = [l for l in tt.split("\n") if l not in sel]
    prelude = "\n".join(l for l in tt.split("\n") if l not in inc_lines)
    def reb(l):
        m_ = re.search(r'include!\(\s*"([^"]+)"\s*\)', l)
        pth = os.path.relpath(os.path.normpath(os.path.join(os.path.dirname(troot), m_.group(1))), os.path.dirname(nroot))
        return 'include!("%s");' % pth
    new_tt = prelude.rstrip("\n") + "\n\n" + "\n".join(reb(l) for l in sel) + "\n"
    owner_tt = "\n".join(keep_lines)
rx = re.compile(r'(?:[ \t]*//#region[^\n]*\n)?[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path ?= ?"%s"\]\n[ \t]*(?:pub )?mod \w+;\n(?:[ \t]*//#endregion[^\n]*\n)?' % re.escape(rel))
m = rx.search(mt)
if not a.redo and not partial: assert m, ("mount not found", rel)
ttx = new_tt if partial else tt
rootdir = os.path.dirname(nroot if partial else troot)
inc = [os.path.normpath(os.path.join(rootdir, x)) for x in re.findall(r'include!\(\s*"([^"]+)"\s*\)', ttx)]
extra = "".join(rd(f) for f in inc)
new_root = companion_root_text(rt, ttx, ident, extra_text=extra, local_mods=mods)
tk = '#[cfg(any(test, feature = "testkit"))]\npub mod testkit {\n    pub use super::{%s};\n}\n' % ", ".join("%s::*" % x for x in mods)
print("testkit globs:", mods)
if not a.write:
    print(new_root[:2500]); sys.exit(0)
if partial and not a.redo:
    wr(troot, owner_tt)
    mt2 = mt
    if mfile == oroot and not a.no_testkit_mod: mt2 = mt.rstrip("\n") + "\n\n" + tk
    wr(mfile, mt2)
    os.makedirs(os.path.dirname(nroot), exist_ok=True)
    wr(nroot, new_root)
    _, npd = make_companion(a.owner, a.suffix, nroot, a.desc or ("Companion white-box laws of %s" % a.owner), kind=a.kind, role=a.role, route_after=a.route_after, extra_dev=a.dev)
    rebase_fixtures([f for f in closure(nroot) if f != nroot], os.path.dirname(oman), npd)
    sys.exit(0)
if a.redo:
    wr(troot, new_root)
    make_companion(a.owner, a.suffix, troot, "x", kind=a.kind, role=a.role, register=False, extra_dev=a.dev)
    sys.exit(0)
nmt = mt[:m.start()] + (tk if mfile == oroot and not a.no_testkit_mod else "") + mt[m.end():]
wr(troot, new_root); wr(mfile, nmt)
desc = a.desc or ("Companion white-box laws of %s" % a.owner)
_, npd = make_companion(a.owner, a.suffix, troot, desc, kind=a.kind, role=a.role, route_after=a.route_after, extra_dev=a.dev)
rebase_fixtures([f for f in closure(troot) if f != troot] + [troot], os.path.dirname(oman), npd)
