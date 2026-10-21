#!/usr/bin/env python3
"""Copy of the tool-iface set at the ticket root (`r6-tool-iface-tools`, same depth as the original: it derives the repository root from its location) with the catalogue anchors re-based on rcache-server's `remote-cache` operation."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/tool-iface-tools", "r6-tool-iface-tools", beside=True)
E = T + "/build/edits.py"
patch(E, '"operations": ["graph", "show", "incremental", "concurrency", "capacity", "leases", "cache"],', '"operations": ["graph", "show", "incremental", "concurrency", "capacity", "leases", "cache", "remote-cache"],')
patch(E, '"operations": ["graph", "show", "incremental", "concurrency", "capacity", "leases", "cache", "tools"],', '"operations": ["graph", "show", "incremental", "concurrency", "capacity", "leases", "cache", "remote-cache", "tools"],')
import re
text = open(E, encoding="utf8").read()
pattern = r"SCRIPT_OLD = \'\'\'  const\{withBuildCapacityV1\}=await import\((\"[^\"]+\")\);\n  await withBuildCapacityV1\(args\.slice\(prefix\),environment,granted=>runAdmittedBuildCliV1\(\'\'\'"
if len(re.findall(pattern, text)) != 1:
    raise SystemExit("maker: tool-iface SCRIPT_OLD not found once")
text = re.sub(pattern, lambda m: "SCRIPT_OLD = \'\'\'  const{withBuildCapacityV1}=await profilePhaseV1(\"door.load\",()=>import(" + m.group(1) + "));\'\'\'", text)
open(E, "w", encoding="utf8").write(text)
print("tool-iface copy:", T)
