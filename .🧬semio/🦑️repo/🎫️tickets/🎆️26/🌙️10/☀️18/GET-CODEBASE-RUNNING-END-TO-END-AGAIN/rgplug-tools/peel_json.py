"""Peel one include_str! JSON document out of a crate into a new tiny document crate."""
import sys, json, os, re
sys.path.insert(0, os.path.dirname(__file__))
from rgp import *

def run(s):
    ws = s["ws"]; wsdir = os.path.dirname(ws)
    ddir = s["dir"]
    pkg = ddir + "/📦️packages/🦀️rust"
    const = s["const"]
    wr(ddir + "/🦀️.rs", "//! %s %s\n\npub const %s: &str = include_str!(\"%s\");\n" % (s["emoji"], s["doc"], const, s["json_rel"]))
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
''' % (s["workspace_rel"], s["name"], s["desc"])
    wr(pkg + "/Cargo.toml", mt)
    rel = lambda p: p[len(wsdir) + 1:]
    ws_add_member(ws, rel(pkg))
    ws_add_dep(ws, s["name"], rel(pkg), after=s.get("after"))
    f = s["user_rs"]
    t = rd(f)
    assert s["include_old"] in t
    wr(f, t.replace(s["include_old"], "%s::%s" % (cargo_ident(s["name"]), const)))
    add_dep_line(s["user_manifest"], "dependencies", s["name"], "%s = { workspace = true }" % s["name"])
    print(run_owners(s["scope"], True))

if __name__ == "__main__":
    run(json.load(open(sys.argv[1])))
