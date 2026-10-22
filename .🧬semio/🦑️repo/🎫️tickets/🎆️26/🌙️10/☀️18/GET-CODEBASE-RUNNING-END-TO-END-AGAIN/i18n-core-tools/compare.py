#!/usr/bin/env python3
import json, sys
a, b = json.load(open(sys.argv[1], encoding="utf8")), json.load(open(sys.argv[2], encoding="utf8"))
for f in sorted(set(a) | set(b)):
    x, y = a.get(f), b.get(f)
    if x != y and (x is None or y is None or (x["pass"], x["fail"]) != (y["pass"], y["fail"])):
        print("CHANGED", f[-90:], x and (x["pass"], x["fail"]), "->", y and (y["pass"], y["fail"]))
print("compared", len(set(a) | set(b)))
