#!/usr/bin/env python3
"""🧵️ Wave 2 follow-up: monolith paths inside string literals (derive attributes) and comments follow the cut too."""
import sys, json
sys.path.insert(0, __file__.rsplit("/", 1)[0])
from semio_split import *
from ownership import rust_name
own = json.load(open(__file__.rsplit("/", 1)[0] + "/ownership.json"))
RX = re.compile(r"(?<![\w$])crate::(base_foundation|standards::v1::subsets::(\w+)(?:::io::(import|export)\b)?)")
n = 0
for p, key in own.items():
    s = read(p)
    def rep(m):
        if m.group(1) == "base_foundation":
            t = "base"
        elif m.group(2) == "base":
            t = "agg"
        elif m.group(2) in NAMES:
            t = m.group(2) + ("-" + m.group(3) if m.group(3) else "")
        else:
            return m.group(0)
        return "crate" if t == key else rust_name(t)
    s2 = RX.sub(rep, s)
    if s2 != s:
        write(p, s2); n += 1
print(n)
