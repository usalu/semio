"""Hoist leaf modules (declared `#[path = ".."] pub mod NAME;` in a parent file of crate OLD) into a new crate NEW that sits below OLD.
The new crate keeps the module chain (standards::...::<parent mod>::NAME) so importers only change the crate identifier.
spec: {ws, old_name, old_pkg, parent_file, chain:[mods...], modules:[names], new:{name,dir,emoji,doc,desc,extern:[lines]}, subtree, rewrite_prefixes:[ "crate::a::b::parent" ], script}"""
import sys, json, re, os, subprocess
sys.path.insert(0, os.path.dirname(__file__))
from rgp import *

def run(s):
    ws = s["ws"]; wsdir = os.path.dirname(ws)
    parent = rd(s["parent_file"])
    pdir = os.path.dirname(s["parent_file"])
    new = s["new"]
    ndir = new["dir"]; npkg = ndir + "/📦️packages/🦀️rust"
    mounts = {}
    for name in s["modules"]:
        m = re.search(r'(?m)^((?:\s*#\[cfg[^\n]*\]\n)?)#\[path = "([^"]+)"\]\n(pub(?:\([^)]*\))? )?mod ' + re.escape(name) + r';\n', parent)
        assert m, name
        mounts[name] = (m, os.path.normpath(os.path.join(pdir, m.group(2))))
    body = ""
    for name in s["modules"]:
        rel = os.path.relpath(mounts[name][1], ndir)
        body += '#[path = "%s"]\npub mod %s;\n' % (rel, name)
    chain = s["chain"]
    text = '//! %s %s\n\n' % (new["emoji"], new["doc"])
    for l in new.get("extern", []):
        text += l + "\n"
    if new.get("extern"):
        text += "\n"
    ind = ""
    for seg in chain:
        text += ind + '#[path = "."]\n' + ind + 'pub mod %s {\n' % seg
        ind += "    "
    for line in body.split("\n"):
        if line:
            text += ind + line + "\n"
    for seg in reversed(chain):
        ind = ind[:-4]
        text += ind + "}\n"
    wr(ndir + "/🦀️.rs", text)
    old_manifest = rd(s["old_pkg"] + "/Cargo.toml")
    extra = new.get("extra_up", 1)
    lib_old = re.search(r'(?m)^\[lib\]\s*\npath = "([^"]*)"', old_manifest).group(1)
    mt = rebase_manifest(old_manifest, len(npkg.split("/")) - len(s["old_pkg"].split("/")))
    mt = manifest_for(mt, new["name"], new["desc"], "../../🦀️.rs")
    wr(npkg + "/Cargo.toml", mt)
    rel = lambda p: p[len(wsdir) + 1:]
    ws_add_member(ws, rel(npkg))
    ws_add_dep(ws, new["name"], rel(npkg), after=s["old_name"])
    if s.get("script"):
        script_add(s["script"], s["old_name"], new["name"])
    # remove mounts from parent, rewrite imports
    pt = parent
    for name in s["modules"]:
        m = mounts[name][0]
        pt = pt.replace(m.group(0), "", 1)
    ident = cargo_ident(new["name"])
    chain_path = "::".join(chain)
    for pre in s["rewrite_prefixes"]:
        for name in s["modules"]:
            pt = re.sub(re.escape(pre) + r'::' + re.escape(name) + r'\b', ident + "::" + "::".join(s["chain_full"]) + "::" + name, pt)
    wr(s["parent_file"], pt)
    # dependency of old on new
    add_dep_line(s["old_pkg"] + "/Cargo.toml", "dependencies", new["name"], "%s = { workspace = true }" % new["name"])
    for pkg in (npkg, s["old_pkg"]):
        files = closure(pkg + "/Cargo.toml")
        txt = closure_text(files)
        mt = rd(pkg + "/Cargo.toml")
        pr = prune_manifest(mt, txt)
        if pr != mt:
            wr(pkg + "/Cargo.toml", pr)
    print(run_owners(s["subtree"], True))

if __name__ == "__main__":
    run(json.load(open(sys.argv[1])))
