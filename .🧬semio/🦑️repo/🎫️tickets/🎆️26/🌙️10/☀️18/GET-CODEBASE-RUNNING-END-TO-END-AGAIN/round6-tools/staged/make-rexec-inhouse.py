#!/usr/bin/env python3
"""Copy of the rexec-inhouse set on top of cache-gc staged3 + rcache-server: the cache ledger's owner enum / type ends `"graph", "remote-cache"` there, so `remote-execution` is appended after it; the remote-execution dispatch block of the split script (⚙️runtime kernel) ends before the cache-store import (the remote-cache block is in another kernel file now), so the region end moves there."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/rexec-inhouse-tools", "rexec-inhouse")
patch(T + "/edits.py", """'"dashboard", "remote-cache"]', '"dashboard", "remote-cache", "remote-execution"]'""", """'"dashboard", "graph", "remote-cache"]', '"dashboard", "graph", "remote-cache", "remote-execution"]'""")
patch(T + "/edits.py", """'|"dashboard"|"remote-cache";', '|"dashboard"|"remote-cache"|"remote-execution";'""", """'|"dashboard"|"graph"|"remote-cache";', '|"dashboard"|"graph"|"remote-cache"|"remote-execution";'""")
patch(T + "/edits.py", "region(BUILD_SCRIPT, '''    if(command===\"remote-execution\"){''', '''    if(command===\"remote-cache\"){'''", "region(BUILD_SCRIPT, '''    if(command===\"remote-execution\"){''', '''    const {withRepositoryCacheStoreV1}=await import('''")
import subprocess
subprocess.run([sys.executable, os.path.join(HERE, "fixups", "rexec-cap-gc.py"), T + "/edits.py"], check=True)
print("rexec-inhouse merged copy:", T)
