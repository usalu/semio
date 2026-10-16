"""Negative control of the union oracle: append a probe to the sandbox companion root and require the expected diagnostic. usage: negcontrol.py <cid> '<probe rust>' '<expected substring>'"""
import sys, os, json
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import union
cid, probe, expect = sys.argv[1], sys.argv[2], sys.argv[3]
c = union.cfg_of(cid)
B = union.build_current(cid)
f = f"{B}/{c['comps'][0]['lib']}"
union.sw(f, open(f, encoding="utf-8").read() + "\n" + probe + "\n")
e = union.diags(union.run_rustc(B, "__otcp_union.rs", "crate_b", ["--test"], c.get("features", ["testkit"])), B)
hits = [d["message"] for d in e if expect in d["message"]]
print("NEGATIVE CONTROL", "PASS" if hits else "FAIL", hits[:2])
