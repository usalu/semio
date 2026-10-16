"""Rewrites `HubDirectory::method` path references (docs and UFCS calls) to the owning capability trait; reports every edit."""
import os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from hdclib import *

files = hub_files() + ["🌎️hub/🧪️tests/🤖️agent-ceiling/🟦️.ts"]
rx = re.compile(r"\bHubDirectory::(\w+)")
n = 0
for f in files:
    p = os.path.join(ROOT, f)
    t = read(p)
    if "HubDirectory::" not in t:
        continue
    def sub(m):
        name = m.group(1)
        assert name in TRAIT_OF, (f, name)
        return TRAIT_OF[name] + "::" + name
    t2 = rx.sub(sub, t)
    if t2 != t:
        write(p, t2)
        n += 1
        print("edited", f, len(rx.findall(t)))
print("files", n)
