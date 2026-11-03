import os, sys, json; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *
import jsonspan as J
def compact(path, defs_key, names=("CaseNumber", "CaseText", "CaseRow")):
    t = rd(path)
    for n in names:
        node = J.locate(t, [defs_key, n])
        s, e = node["span"]
        v = json.loads(t[s:e])
        t = t[:s] + json.dumps(v, ensure_ascii=False) + t[e:]
    json.loads(t)
    wr(path, t)
if __name__ == "__main__":
    A = Art(sys.argv[1])
    compact(A.snap + "/🔣️.json", "$defs")
