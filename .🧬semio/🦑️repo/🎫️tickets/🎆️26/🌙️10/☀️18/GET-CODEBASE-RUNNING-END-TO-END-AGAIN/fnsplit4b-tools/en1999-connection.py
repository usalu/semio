import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import phspec
F = "/Users/ueli/Documents/semio/" + sys.argv[2]
types = {"params": "na_de::AnnexParams", "alloy": "part_1_1::AlloyProps", "part": "&str", "out": "Vec<CheckResult>", "kind": "String", "combo": "&str", "action_id": "&str",
         "lever": "f64", "n_ed": "f64", "v_ed": "f64", "m_ed": "f64", "conn_dig": "String"}
modes = {"out": "mut", "kind": "ref", "alloy": "ref", "params": "ref", "conn_dig": "ref", "lever": "val", "n_ed": "val", "v_ed": "val", "m_ed": "val", "annex": "val"}
s = phspec.build(F, "check_connection", [
 ("check_bolted_connection", "Checks the bearing, shear, tension and interaction resistance of the bolts of a connection.", 69, 69),
 ("check_welded_connection", "Checks the weld resistance of a connection.", 182, 182, {"allow_control": [198]}),
 ("check_connection_serviceability", "Checks the serviceability of a connection under its frequent actions.", 253, 253),
], types=types, modes=modes, keep="auto", nth=0, out=None, allow_reason="the extracted check keeps the connection values of its original function as explicit parameters")
json.dump(s, open(sys.argv[1], "w", encoding="utf-8"), ensure_ascii=False, indent=1)
