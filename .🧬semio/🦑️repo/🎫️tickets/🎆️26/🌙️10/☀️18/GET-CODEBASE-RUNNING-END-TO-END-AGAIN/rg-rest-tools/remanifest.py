"""Regenerate the new crate manifest of a carve spec from the first-touch backup of the old manifest. usage: remanifest.py <spec.json>"""
import sys, os, re, json
RT = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
sys.argv_saved = sys.argv; spec_path = sys.argv[1]; sys.argv = ["carve.py", "/dev/null"]
src = open(RT + "/rg-rest-tools/carve.py", encoding="utf8").read().replace("\nmain()\n", "\n")
g = {"__name__": "carvelib"}
exec(compile(src, "carve.py", "exec"), g)
rgp = g["rgp"]
spec = json.load(open(spec_path))
old_pkg = spec["old_pkg"]; npkg = spec["new"]["dir"] + "/📦️packages/🦀️rust"
led = json.load(open(rgp.BK + "/ledger.json"))
h = led.get(old_pkg + "/Cargo.toml")
old = open(rgp.BK + "/" + h + ".orig", encoding="utf8").read() if h else g["rd"](old_pkg + "/Cargo.toml")
mt = g["rebase_manifest2"](old, old_pkg, npkg)
mt = g["manifest_for"](mt, spec["new"]["name"], spec["new"]["desc"], "../../🦀️.rs")
txt = g["closure_text"](rgp.closure(npkg + "/Cargo.toml"))
g["wr"](npkg + "/Cargo.toml", g["prune_new"](mt, txt))
print("ok")
