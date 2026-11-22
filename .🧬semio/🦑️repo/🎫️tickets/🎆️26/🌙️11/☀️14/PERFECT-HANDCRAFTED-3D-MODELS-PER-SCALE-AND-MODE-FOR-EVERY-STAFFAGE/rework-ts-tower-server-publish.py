"""📮 Publishes the tower-server candidate document: splices the named root members (e.g. rig poses representations) into the catalog file, leaving every other byte, and writes its `volumes` to the source."""
import json, sys, os, re, importlib.util
T = os.path.dirname(os.path.abspath(__file__))
A = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/tower-server/🔣️.json"
SRC = T + "/volumes/appliances/tower-server.json"
CAND = T + "/🗑️generated/r3-tower-server/🔣️.json"
members = sys.argv[1:]
spec = importlib.util.spec_from_file_location("vi", T + "/volumes-insert.py"); vi = importlib.util.module_from_spec(spec)
sys.argv = ["x"]
try: spec.loader.exec_module(vi)
except SystemExit: pass
cand = json.load(open(CAND))["asset"]
text = open(A, encoding="utf-8").read()
indent = "    "
out = text
for key in members:
    start, end = vi.member_range(out, indent, key)
    body = json.dumps(cand[key], ensure_ascii=False, indent=2).replace("\n", "\n" + indent)
    out = out[:start] + indent + '"' + key + '": ' + body + out[end:]
check = json.loads(out)
for key in members: assert check["asset"][key] == cand[key]
assert {k: v for k, v in check["asset"].items() if k not in members} == {k: v for k, v in json.loads(text)["asset"].items() if k not in members}
if open(A, encoding="utf-8").read() != text: raise SystemExit("asset changed; retry")
open(A, "w", encoding="utf-8").write(out)
json.dump({"volumes": cand["volumes"]}, open(SRC, "w", encoding="utf-8"), ensure_ascii=False, indent=2)
print("spliced", members, "and wrote source")
