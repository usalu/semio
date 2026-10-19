"""Peel self-contained top-level chunks (e.g. `pub mod editor { ..stub mounts.. }`) of a crate's mirror/root file into a new dependency-free crate; re-point importers."""
import sys, json, re, os, subprocess
sys.path.insert(0, os.path.dirname(__file__))
from rgp import *

def run(s):
    ws = s["ws"]; wsdir = os.path.dirname(ws)
    mf = s["mirror_file"]; mdir = os.path.dirname(mf)
    src = rd(mf)
    items = top_items(src)
    keep, moved = [], []
    for it in items:
        if mod_name(it) in s["chunks"]:
            moved.append(it)
        else:
            keep.append(it)
    assert len(moved) == len(s["chunks"]), (len(moved), s["chunks"])
    new = s["new"]; ndir = new["dir"]; npkg = ndir + "/📦️packages/🦀️rust"
    def rebase(m):
        p = m.group(2)
        if p == ".":
            return m.group(0)
        return m.group(1) + os.path.relpath(os.path.normpath(os.path.join(mdir, p)), ndir) + '"'
    body = "".join(moved)
    body = re.sub(r'(#\[path = ")([^"]*)"', rebase, body)
    wr(ndir + "/🦀️.rs", "//! %s %s\n\n%s" % (new["emoji"], new["doc"], body.lstrip("\n")))
    mt = '''[package]
workspace = "%s"
name = "%s"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
description = "%s"

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "../../🦀️.rs"
''' % ("/".join([".."] * (len(npkg.split("/")) - len(wsdir.split("/")))), new["name"], new["desc"])
    wr(npkg + "/Cargo.toml", mt)
    rel = lambda p: p[len(wsdir) + 1:]
    ws_add_member(ws, rel(npkg))
    ws_add_dep(ws, new["name"], rel(npkg), after=s["old_name"])
    if s.get("script"):
        script_add(s["script"], s["old_name"], new["name"])
    wr(mf, "".join(keep))
    old_ident = cargo_ident(s["old_name"]); new_ident = cargo_ident(new["name"])
    out = subprocess.run(["git", "grep", "-l", "-e", old_ident, "--", s["subtree"]], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    changed = []
    pat = re.compile(r'(?<![A-Za-z0-9_])' + old_ident + r'::(' + "|".join(s["chunks"]) + r')::')
    for f in [x for x in out if x.endswith(".rs")]:
        t = rd(f)
        nt = pat.sub(lambda m: new_ident + "::" + m.group(1) + "::", t)
        if nt != t:
            wr(f, nt); changed.append(f)
    print("repointed", len(changed), [c[-60:] for c in changed][:5])
    mfs = subprocess.run(["git", "grep", "-l", "-e", s["old_name"], "--", s["subtree"]], cwd=ROOT, capture_output=True, text=True).stdout.split("\n")
    skip = {ws, s["old_pkg"] + "/Cargo.toml", npkg + "/Cargo.toml"}
    for mfp in [x for x in mfs if x.endswith("Cargo.toml") and x not in skip]:
        txt = closure_text(closure(mfp))
        mtt = rd(mfp)
        if re.search(r'\b' + new_ident + r'\b', txt):
            for sec in ("dependencies", "dev-dependencies"):
                b = sec_bounds(mtt, sec)
                if b and dep_key_re(s["old_name"]).search(mtt[b[0]:b[1]]):
                    add_dep_line(mfp, sec, new["name"], "%s = { workspace = true }" % new["name"])
                    mtt = rd(mfp)
        if not re.search(r'\b' + old_ident + r'\b', txt):
            for sec in ("dependencies", "dev-dependencies"):
                remove_dep_line(mfp, sec, s["old_name"])
    print(run_owners(s["subtree"], True))

if __name__ == "__main__":
    run(json.load(open(sys.argv[1])))
