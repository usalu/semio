#!/usr/bin/env python3
# 📋️ Final re-homing plan: member-level moves (source file, item, destination crate/slug) from the solver, written to ri2-tools/moves.json (computed once from the pristine tree).
import json, os, io, contextlib, runpy, sys, re
HERE = os.path.dirname(os.path.abspath(__file__))
sys.argv = [sys.argv[0]]
buf = io.StringIO()
with contextlib.redirect_stdout(buf):
    S = runpy.run_path(HERE + "/solve3.py")
its, units, home = S["its"], S["units"], S["home"]
out = []
for u in units:
    uid = u["id"]
    h = home[uid]
    orig = u["crate"]
    if h == orig:
        continue
    if h is None:
        continue
    for m in u["members"]:
        it = its[m]
        text = open(it["file"], encoding="utf8").read().split("\n")
        head = text[it["head"] - 1] if "head" in it else ""
        out.append(dict(src=it["file"], src_crate=it["crate"], src_slug=it["slug"], kind=it["kind"], name=it["name"], head=text[it["start"] - 1:it["end"]][[i for i, l in enumerate(text[it["start"] - 1:it["end"]]) if not l.lstrip().startswith(("///", "#[", "//"))][0]].strip(), dest=h, slug=it["slug"], unit=uid))
json.dump(out, open(os.environ.get("RI2_MOVES_OUT", HERE + "/moves.json"), "w"), ensure_ascii=False, indent=1)
import collections
print(len(out), "member moves in", len({o["unit"] for o in out}), "units")
print(collections.Counter(o["dest"] for o in out))
