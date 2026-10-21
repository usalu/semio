#!/usr/bin/env python3
"""Checks the oracle against the hand-written expectations of the fixture (no TypeScript involved): the oracle and the fixture must agree before the TypeScript implementation is judged."""
import importlib.util, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); S = os.path.join(HERE, "staged", "🧩️units", "🧪️tests")
spec = importlib.util.spec_from_file_location("oracle", os.path.join(S, "🐍️oracle", "🐍️.py")); o = importlib.util.module_from_spec(spec); spec.loader.exec_module(o)
f = json.load(open(os.path.join(S, "🔣️.json"), encoding="utf8")); kinds = f["policy"]["interfaceKinds"]; bad = 0
for c in f["graphs"]:
    a = o.answer_graph(c, kinds); e = c["expect"]
    ok = a["valid"] == e["valid"] and (a.get("order") == e.get("order") and a.get("layer") == e.get("layer") if e["valid"] else a["kinds"] == sorted(e["kinds"]))
    if not ok: bad += 1; print("graph", c["name"], a, e)
for c in f["scenarios"]:
    got = o.run_scenarios(c, kinds)
    for i, (g, s) in enumerate(zip(got, c["steps"])):
        if g != s["expect"]: bad += 1; print("scenario", c["name"], "step", i, g, s["expect"])
for c in f["python"]["analyze"]:
    r = o.analyze_ast(c["source"], c.get("init", False)); e = c["expect"]
    srt = lambda rows: sorted(json.dumps(x) for x in rows)
    if srt(r["imports"]) != srt(e["imports"]) or r["names"] != e["names"] or r["dynamic"] != e["dynamic"]: bad += 1; print("analyze", c["name"], r, e)
for c in f["python"]["pairs"]:
    m = c.get("unannotatedReturns", "ignore"); a = o.analyze_ast(c["before"], c.get("init", False), m)["fingerprint"]; b = o.analyze_ast(c["after"], c.get("init", False), m)["fingerprint"]
    if (a == b) != c["same"]: bad += 1; print("pair", c["name"], a == b, c["same"])
for c in f["names"]:
    if o.unit_target_name(c["id"]) != c["expect"]: bad += 1; print("name", c["id"])
print("oracle vs fixture:", "OK" if not bad else "%d mismatches" % bad); sys.exit(1 if bad else 0)
