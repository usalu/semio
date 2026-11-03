"""Probe: which bars of a frame fixture carry a bow, and the orientation-reversal invariance of the oracle rows. Usage: probe_bows.py <fixture.json> [reverse-bar-id]"""
import importlib.util, json, pathlib, sys
base = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🚦️check-outcome")
spec = importlib.util.spec_from_file_location("main_oracle", base / "🐍️.py")
M = importlib.util.module_from_spec(spec); sys.modules["main_oracle"] = M; spec.loader.exec_module(M)
W = M.SECOND_WAVE
doc = json.load(open(sys.argv[1], encoding="utf8"))
if len(sys.argv) > 2:
    for bar in doc["frameBars"]:
        if bar["id"] == sys.argv[2]:
            bar["startNodeId"], bar["endNodeId"] = bar["endNodeId"], bar["startNodeId"]
            bar["releaseStart"], bar["releaseEnd"] = bar["releaseEnd"], bar["releaseStart"]
p = M.PARAMS["de"]
fr = W.Frame(doc)
nodal = {n["id"]: (n["loadX"], n["loadZ"]) for n in doc["frameNodes"] if n["loadX"] or n["loadZ"]}
_, ends, _, _ = fr.solve(nodal, None, iterate=False)
first_n = {b: n for b, (n, _) in fr.effects(ends).items()}
print("bows", W.bow_loads(doc, M, first_n))
for r in W.second_order_rows(doc, M, M.FAMILIES, p):
    print(r[0], round(r[1], 9))
