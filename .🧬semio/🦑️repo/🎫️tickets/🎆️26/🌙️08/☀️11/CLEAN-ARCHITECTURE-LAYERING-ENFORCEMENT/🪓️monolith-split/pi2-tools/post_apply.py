#!/usr/bin/env python3
"""🔧️ Integration edits after the companion test bins are applied: the media-turn native target moves to the bin that owns the media hosts, the facade no longer re-exports a host module it does not mount, the TS runtime-include law follows the mounts.
usage: post_apply.py [--apply]"""
import json, os, re, sys
from lib import *
import ed

apply = "--apply" in sys.argv
crates = json.load(open(TOOLS + "/crates.json"))
def bin_of(mod):
    for k, r in crates.items():
        root = PLUG + "/" + r["root"] + "/🦀️.rs"
        if r.get("kind") == "companion" and os.path.exists(root) and re.search(r"\bpub mod " + mod + ";", read(root)):
            return k
    raise SystemExit("no bin hosts " + mod)


K = bin_of("document_media_test_hosts")
row = crates[K]
CNAME = row["name"]
LIB = CNAME.replace("-", "_")
FAC = PLUG + "/" + PKGDIR
edits = {}
lib_rs = read(PLUG + "/🦀️.rs")
a = '#[cfg(feature="media-turn-testing")]\npub use document_media_test_hosts::document_media_unit_1::tests as media_turn_laws;\n'
assert a in lib_rs
edits[PLUG + "/🦀️.rs"] = lib_rs.replace(a, "")
fac = read(FAC + "/Cargo.toml")
blk = '[[test]]\nname = "media-turn"\npath = "../../🎞️media/⚠️turn/🧪️tests/🎟️native/🦀️.rs"\nrequired-features = ["media-turn-testing"]\n\n'
assert blk in fac
edits[FAC + "/Cargo.toml"] = fac.replace(blk, "")
bpkg = REPO + "/" + row["pkg"]
man = read(bpkg + "/Cargo.toml")
tpath = os.path.relpath(PLUG + "/🎞️media/⚠️turn/🧪️tests/🎟️native/🦀️.rs", bpkg)
if '[[test]]\nname = "media-turn"' not in man:
    man = man.rstrip("\n") + f'\n\n[[test]]\nname = "media-turn"\npath = "{tpath}"\nrequired-features = ["media-turn-testing"]\n'
edits[bpkg + "/Cargo.toml"] = man
nat = PLUG + "/🎞️media/⚠️turn/🧪️tests/🎟️native/🦀️.rs"
nt = read(nat).replace("semio_framework_plugin::media_turn_laws::", LIB + "::document_media_test_hosts::document_media_unit_1::tests::")
edits[nat] = nt
sc = FAC + "/📜️script.ts"
st = read(sc)
b = 'await runRepositoryCargoTests([],this.root,this.invocation,["--offline","--test","media-turn","--features","media-turn-testing","--","--nocapture"]);'
assert b in st
edits[sc] = st.replace(b, 'await runRepositoryCargoTests(["' + CNAME + '"],this.root,this.invocation,["--offline","--test","media-turn","--features","media-turn-testing","--","--nocapture"]);')
for p, t in edits.items():
    print(rel(p)[-80:], "changed" if t != read(p) else "same")
if apply:
    for p, t in edits.items():
        if t != read(p):
            ed.put(p, t)
    print("applied")
