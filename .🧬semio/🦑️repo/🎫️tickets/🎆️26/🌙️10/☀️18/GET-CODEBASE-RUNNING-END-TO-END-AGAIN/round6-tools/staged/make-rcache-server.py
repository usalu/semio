#!/usr/bin/env python3
"""Merged copy of the rcache-server set on top of cache-gc staged2: the `cache` operation's flag list gained `--deep`, so the three catalogue edits that cut `cache serve|remote` carry it; cache-gc staged3 added the `graph` owner and the `graph-inference` delegate (language parameter), so the owner enum / type and the delegate anchor follow."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/rcache-server-tools", "rcache-server")
patch(T + "/edits.py", '"--json", "--dry-run", "--area", "--tasks"]', '"--json", "--dry-run", "--area", "--deep", "--tasks"]', count=3)
E = T + "/edits.py"
patch(E, '"test-evidence", "dashboard"]', '"test-evidence", "dashboard", "graph"]', count=1)
patch(E, '"test-evidence", "dashboard", "remote-cache"]', '"test-evidence", "dashboard", "graph", "remote-cache"]', count=1)
patch(E, '"test-evidence"|"dashboard";', '"test-evidence"|"dashboard"|"graph";', count=1)
patch(E, '"test-evidence"|"dashboard"|"remote-cache";', '"test-evidence"|"dashboard"|"graph"|"remote-cache";', count=1)
patch(E, "  delegate:(owner,_id,dryRun)=>{\n", "", count=2)
patch(E, "invocation.control.signal", "phaseControl.signal", count=1)
patch_if(E, "region(SCRIPT, '''    if(command===\"cache\"&&[\"serve\",\"remote\"]", "region(SCRIPT, '''if(command===\"cache\"&&[\"serve\",\"remote\"]")
_t = open(E, encoding="utf8").read()
_t = _t.replace("'''    const {withRepositoryCacheStoreV1}=await import('''", "'''const{withRepositoryCacheStoreV1}=await import('''")
open(E, "w", encoding="utf8").write(_t)
_t = open(E, encoding="utf8").read()
_t = _t.replace('"synopsis": "cache status|gc|clear|remote status|serve"', '"synopsis": "cache status|gc|verify|clear|remote status|serve"').replace('"synopsis": "cache status|gc|clear",', '"synopsis": "cache status|gc|verify|clear",')
open(E, "w", encoding="utf8").write(_t)
import subprocess
subprocess.run([sys.executable, os.path.join(HERE, "fixups", "rcache-cap-gc.py"), E], check=True)
print("rcache-server merged copy:", T)
