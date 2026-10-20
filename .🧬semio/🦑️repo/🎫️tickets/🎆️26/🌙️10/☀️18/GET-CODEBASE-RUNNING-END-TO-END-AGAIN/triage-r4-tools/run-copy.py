#!/usr/bin/env python3
"""python3 run-copy.py <live test path rel root> <out> [old=>new ...]: writes a harness-runnable copy of one test outside the build library (absolute imports), optionally with literal replacements."""
import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio"
rel, out = sys.argv[1], sys.argv[2]
origin = ROOT + "/" + rel
s = open(origin, encoding="utf8").read()
for pair in sys.argv[3:]:
    a, b = pair.split("=>", 1)
    assert s.count(a) >= 1, a
    s = s.replace(a, b)
d = os.path.dirname(origin)
def spec(m): return m.group(1) + os.path.normpath(os.path.join(d, m.group(2))) + m.group(3)
s = re.sub(r'(from\s*")(\.\.?/[^"]*)(")', spec, s)
s = re.sub(r'(?m)^(import\s*")(\.\.?/[^"]*)(")', spec, s)
s = re.sub(r'(import\(")(\.\.?/[^"]*)("\))', spec, s)
s = re.sub(r'(?<!")import\.meta\.dir(?!")', lambda m: json.dumps(d, ensure_ascii=False), s)
s = re.sub(r'(?<!")import\.meta\.url(?!")', lambda m: json.dumps("file://" + origin, ensure_ascii=False), s)
os.makedirs(os.path.dirname(out), exist_ok=True)
open(out, "w", encoding="utf8").write(s)
