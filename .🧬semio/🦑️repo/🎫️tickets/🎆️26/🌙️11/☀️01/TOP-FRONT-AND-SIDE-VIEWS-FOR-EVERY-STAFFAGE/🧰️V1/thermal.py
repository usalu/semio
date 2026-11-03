import json, sys
from lib import *

for a in sys.argv[1:]:
    d = load(a)["asset"]
    print("==", a)
    print(json.dumps(d["thermal"], ensure_ascii=False)[:1800])
    print("anchors", [(x["id"], x["bone"]) for x in d["anchors"]][:12])
