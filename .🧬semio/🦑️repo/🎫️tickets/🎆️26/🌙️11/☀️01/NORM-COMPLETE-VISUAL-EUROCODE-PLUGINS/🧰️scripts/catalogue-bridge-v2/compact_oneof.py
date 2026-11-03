import os, sys, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import jsonspan as J
import compact_defs as K
A = Art(sys.argv[1])
K.compact(A.snap + "/🔣️.json", "$defs")
p = A.mut + "/🔣️.json"
t = rd(p)
node = J.locate(t, ["oneOf"])
for item in reversed(node["items"][-7:]):
    s, e = item["span"]
    t = t[:s] + json.dumps(json.loads(t[s:e]), ensure_ascii=False) + t[e:]
json.loads(t)
wr(p, t)
