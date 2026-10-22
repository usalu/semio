#!/usr/bin/env python3
"""Copy of the cache-gc set for apply4 on the split build script: the dispatch of `cache status|gc` into the ledger kernel now exists in three places (💾️cache kernel guard, 📋️ledger kernel guard, 🧭️selection kernel choice); `verify` joins the list in each (same intent as the old single edit). Seen through view.py the three sites are unique strings of the monolith."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/cache-gc-tools", "cache-gc")
P = T + "/patches.py"
text = open(P, encoding="utf8").read()
a = text.index("BUILD_SCRIPT4 = [")
b = text.index("]\n", text.index("),\n]", a)) + 2 if False else text.index("\n]\n", a) + 3
new = '''BUILD_SCRIPT4 = [
    ('(["status","gc"].includes(resolved.args[1]??"")&&!resolved.args.includes("--tasks")))throw Error("Cache command kernel', '(["status","gc","verify"].includes(resolved.args[1]??"")&&!resolved.args.includes("--tasks")))throw Error("Cache command kernel'),
    ('!["status","gc"].includes(resolved.args[1]??"")||resolved.args.includes("--tasks"))throw Error("Cache ledger command kernel', '!["status","gc","verify"].includes(resolved.args[1]??"")||resolved.args.includes("--tasks"))throw Error("Cache ledger command kernel'),
    ('command==="cache"?(["status","gc"].includes(selected[1]??"")&&!selected.includes("--tasks")?"cache-ledger":"cache")', 'command==="cache"?(["status","gc","verify"].includes(selected[1]??"")&&!selected.includes("--tasks")?"cache-ledger":"cache")'),
]
'''
open(P, "w", encoding="utf8").write(text[:a] + new + text[b:])
print("cache-gc copy:", T)
