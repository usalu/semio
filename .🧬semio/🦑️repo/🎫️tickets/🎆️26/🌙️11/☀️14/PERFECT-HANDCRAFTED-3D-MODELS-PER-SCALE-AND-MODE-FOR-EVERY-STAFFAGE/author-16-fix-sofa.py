import sys
sys.path.insert(0, ".")
from importlib import import_module
L = import_module("author-16-lib")
def fix(a):
    n = 0
    for r in a["thermal"]["regions"]:
        ls = r["layers"]
        if len(ls) == 2 and ls[0]["id"] == "fabric":
            r["layers"] = [ls[1], ls[0]]
            n += 1
    return n
print(L.edit_asset("🛋️furniture", sys.argv[1], fix))
