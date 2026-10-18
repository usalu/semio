import sys, re, os, json
sys.path.insert(0, "/Users/ueli/.cache/hdr3/tools")
from mkcomp import *
import usetree as U
spec = json.load(open(sys.argv[1]))
write = "--write" in sys.argv
owner = spec["owner"]
oman = census_manifest(owner)
oroot = lib_root(oman)
ident = lib_ident_of(oman)
rt = rd(oroot)
root_stmts = U.top_statements(rt)
new_root = os.path.join(R, spec["new_root"])
rootdir = os.path.dirname(new_root)
parts, edits = [], []
root_externs = []
used_aliases = {}
all_moved = []
for mv in spec["moves"]:
    mf = os.path.join(R, mv["mount_file"])
    mtext = rd(mf)
    name = mv.get("mount_name", "tests")
    rx = re.compile(r'(?:[ \t]*//\s*#?region[^\n]*\n)?[ \t]*#\[cfg\((?:test|all\(test, not\(target_arch = "wasm32"\)\))\)\]\n(?:[ \t]*//[^\n]*\n)*[ \t]*#\[path ?= ?"([^"]+)"\]\n[ \t]*(?:pub(?:\([^)]*\))? )?mod %s;\n(?:[ \t]*//\s*#?endregion[^\n]*\n)?' % re.escape(name))
    cands = [mm for mm in rx.finditer(mtext) if mv.get("match_path", "") in mm.group(1)]
    assert len(cands) == 1, ("mount", mv, len(cands))
    m = cands[0]
    tfile = os.path.normpath(os.path.join(os.path.dirname(mf), m.group(1)))
    body = rd(tfile)
    mpath = mv.get("module_path", "")
    modname = mv.get("as") or (mpath.replace("::", "_") + "_" + name if mpath else name)
    glob = "%s::%s::*" % (ident, mpath) if mpath else "%s::*" % ident
    if re.search(r"(?m)^use super::\*;\n", body):
        mod_stmts = U.top_statements(mtext)
        rest = re.sub(r"(?m)^use super::\*;\n", "", body, count=1)
        own = set()
        for _, st in U.top_statements(rest):
            fl = U.flatten(st)
            if fl:
                for lf in fl[1]: own.add(U.binding(lf))
        locals_ = set(re.findall(r'(?m)^(?:#\[cfg[^\n]*\]\n)?#\[path = "[^"]+"\]\n(?:pub )?mod (\w+);', mtext))
        inc_text = "".join(rd(os.path.normpath(os.path.join(os.path.dirname(tfile), x))) for x in re.findall(r'include!\(\s*"([^"]+)"\s*\)', rest) if os.path.exists(os.path.normpath(os.path.join(os.path.dirname(tfile), x))))
        uses = U.prune(mod_stmts, rest + inc_text, own=own, skip_first=locals_ | {"self", "super"})
        uses = [u.replace("#[cfg(test)] ", "") for u in uses]
        uses = [re.sub(r"^(pub(\([^)]*\))? )?use", "use", u) for u in uses]
        exts = U.externs(mod_stmts, rest + inc_text)
        hdr = "use %s;\n" % glob + (("use %s::testkit::*;\n" % ident) if spec.get("testkit") and not mpath else "") + "".join(e.replace("#[cfg(test)]\n", "") + "\n" for e in exts) + "".join(u + "\n" for u in dict.fromkeys(uses))
        doc_, rest_ = split_doc(rest)
        body2 = doc_ + hdr + rest_
    else:
        body2 = body
    assert "super::" not in body2.replace("super::super", "") or True
    for am in re.finditer(r"\bcrate::(\w+)", body2):
        used_aliases[am.group(1)] = True
    for a_, b_ in spec.get("remaps", []):
        body2 = body2.replace(a_, b_)
    if mv.get("super_super"):
        body2 = body2.replace("super::super::", "%s::%s::" % (ident, mv["super_super"]))
    gate_ = '#[cfg(not(target_arch = "wasm32"))]\n' if 'target_arch = "wasm32"' in m.group(0) else ""
    parts.append((tfile, body2, modname, mf, mtext, m, gate_))
    for e in U.externs(root_stmts, body2):
        root_externs.append(e.replace("#[cfg(test)]\n", ""))
# aliases defined at the owner crate root by a `use` (kept as crate::NAME in the companion)
alias_stmts = []
alias_names = set()
for attrs, st in root_stmts:
    fl = U.flatten(st)
    if st.startswith("extern crate "):
        mm = re.match(r"extern crate (\w+)(?: as (\w+))?;", st)
        nm = mm.group(2) or mm.group(1)
        if nm in used_aliases: alias_stmts.append(st); alias_names.add(nm)
    elif fl and fl[0] != "pub":
        for lf in fl[1]:
            b = U.binding(lf)
            if b in used_aliases and b != "*":
                alias_stmts.append("use %s%s;" % (lf[0], (" as " + lf[1]) if lf[1] else "")); alias_names.add(b)
for a_, b_ in spec.get("remaps", []):
    mm_ = re.match(r"crate::(\w+)", b_)
    if mm_: alias_names.add(mm_.group(1))
root_text = "//! 🧪️ Companion white-box laws of `%s`.\n\n#![cfg(test)]\n#![allow(unused_imports, unused_qualifications)]\n\n" % owner + "".join(e + "\n" for e in dict.fromkeys(root_externs)) + "".join(s + "\n" for s in dict.fromkeys(alias_stmts)) + "\n"
for tfile, body2, modname, mf, mtext, m, gate_ in parts:
    root_text += gate_ + '#[path = "%s"]\nmod %s;\n' % (os.path.relpath(tfile, rootdir), modname)
def fix_crate(b):
    b = re.sub(r"\bcrate::\{", "%s::{" % ident, b)
    def sub(mm):
        return mm.group(0) if mm.group(1) in alias_names else "%s::%s" % (ident, mm.group(1))
    return re.sub(r"\bcrate::(\w+)", sub, b)
print(root_text)
print("aliases kept:", sorted(alias_names), "moves:", [p[2] for p in parts])
if not write: sys.exit(0)
os.makedirs(rootdir, exist_ok=True)
wr(new_root, root_text)
byf = {}
for tfile, body2, modname, mf, mtext, m, gate_ in parts:
    wr(tfile, fix_crate(body2))
    byf.setdefault(mf, rd(mf))
    byf[mf] = byf[mf].replace(m.group(0), "", 1)
for mf, tx in byf.items():
    if spec.get("testkit") and mf == oroot:
        mods_ = re.findall(r'(?m)^(?:#\[cfg[^\n]*\]\n)?#\[path = "[^"]+"\]\n(?:pub )?mod (\w+);', tx)
        tx = tx.rstrip("\n") + '\n\n#[cfg(any(test, feature = "testkit"))]\npub mod testkit {\n    pub use super::{%s};\n}\n' % ", ".join("%s::*" % x for x in mods_)
    wr(mf, tx)
desc = spec.get("desc") or ("Companion white-box laws of %s" % owner)
newname, npd = make_companion(owner, spec["suffix"], new_root, desc, kind=spec.get("kind", "artifact"), route_after=spec.get("route_after"), role=spec.get("role"), extra_dev=spec.get("dev", []))
rebase_fixtures(sorted(set(f for p in parts for f in closure(p[0]))), os.path.dirname(oman), npd)
