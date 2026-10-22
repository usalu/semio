#!/usr/bin/env python3
"""Merged copy of the backend-contract set on top of the chain (the per-backend code is sliced from the monolith view of the split script: the reset block now ends at the `graph|show` dispatch because the cache dispatch left the script):
 - the cache ledger's `owner` enum already ends with "remote-cache" (rcache-server): `semio` is inserted into that list;
 - rcache-server's RemoteCacheStatus schema has no `tool` property any more: the schema edit becomes a no-op there;
 - the adapters are cut again from the build script as the chain leaves it (gen-backends.py) and the staged files re-rendered (build.py): cli-latency, tool-iface and lock-sync edit the script slices the adapters are cut from."""
import os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/backend-contract-tools", "backend-contract")
_w2 = open(T + "/wire2.py", encoding="utf8").read()
if _w2.count('"test-evidence", "dashboard"] }') == 2:
    open(T + "/wire2.py", "w", encoding="utf8").write(_w2.replace('"test-evidence", "dashboard"] }', '"test-evidence", "dashboard", "graph", "remote-cache"] }'))
patch(T + "/wire.py", "def remote_schema(t):\n", "def remote_schema(t):\n    if '\"tool\"' not in t:return t\n")
patch(T + "/scriptslices.py", "cache=find(lines,'    if(command===\"cache\"){',reset)", "cache=find(lines,'    if([\"graph\",\"show\"].includes(command)){',reset)")

patch(T + "/scriptslices.py", "setup_b=find(lines,'      if(tools!==\"nx\"){',tools);setup_n=find(lines,'      if(tools!==\"bazel\"){',setup_b)", "setup_b=find(lines,'      if(tools!==\"nx\"&&tools!==\"toolchains\"){',tools);setup_n=find(lines,'      if(tools!==\"bazel\"&&tools!==\"toolchains\"){',setup_b)")
patch(T + "/wire.py", "|all]`);\n      const tools=resolved.args[2]??selected.tool;\n      if(tools!==\"all\")requireBuildBackendIdV1(tools);\n      for(const id of tools===\"all\"?buildBackendIdsV1():[tools])await", "|toolchains|all] or repo tools verify [tool...]`);\n      const tools=resolved.args[2]??selected.tool;\n      if(![\"toolchains\",\"all\"].includes(tools))requireBuildBackendIdV1(tools);\n{toolchains}\n      for(const id of tools===\"toolchains\"?[]:tools===\"all\"?buildBackendIdsV1():[tools])await")
patch(T + "/wire.py", "    BKI='./'+BK+'/'+TS\n", "    BKI='./'+BK+'/'+TS\n    toolchains=lines[S.find(lines,'      if(tools===\"toolchains\"||tools===\"all\"){')]\n")

patch(T + "/wire2.py", "GC_OLD='if(owner!==\"test-evidence\")return Promise.resolve(undefined);'", "GC_OLD='return Promise.resolve(undefined);'")
patch(T + "/wire2.py", "GC_NEW='if(owner!==\"test-evidence\")return import(", "GC_NEW='return import(")
patch(T + "/wire.py", ".replace('tools setup [nx|bazel|all]','tools setup [{backends}|all]')", ".replace('tools setup [nx|bazel|all]','tools setup [{backends}|all]').replace('tools setup [nx|bazel|toolchains|all]','tools setup [{backends}|toolchains|all]')")
patch(T + "/src/backends_test.ts", "toContain(`tools setup [${alternation}|all]`)", "toContain(`tools setup [${alternation}|toolchains|all]`)")
_gb = open(T + "/gen-backends.py", encoding="utf8").read()
_anchor = 'open(os.path.join(here,"src","nx_backend.ts"),"w",encoding="utf8").write(nx)'
assert _gb.count(_anchor) == 1
_fix = (
    "def with_profile(code):\n"
    "    names=[n for n in ('profileMarkV1','profilePhaseV1') if n in code]\n"
    "    if not names:return code\n"
    "    line='import {'+','.join(names)+'} from \"@@UP@@\\u23F1\\ufe0fprofile/\\U0001F7E8\\ufe0f.mjs\";\\n'\n"
    "    first='import {join} from \"node:path\";\\n'\n"
    "    return code.replace(first,first+line,1) if first in code else line+code\n"
    "nx=with_profile(nx);bz=with_profile(bz)\n"
)
open(T + "/gen-backends.py", "w", encoding="utf8").write(_gb.replace(_anchor, _fix + _anchor, 1))
_w3 = open(T + "/wire3.py", encoding="utf8").read()
_keep = [l for l in _w3.split("\n") if not (l.startswith(" (p(B,T[\"REMOTECACHE\"]") and l.rstrip().endswith(("blob_store),", "blob_server),", "blob_tests),")))]
assert len(_keep) == len(_w3.split("\n")) - 3, "remote-cache blob rows"
open(T + "/wire3.py", "w", encoding="utf8").write("\n".join(_keep))
for script in ("gen-backends.py", "build.py"):
    run = subprocess.run([sys.executable, T + "/" + script], capture_output=True, text=True, cwd=T)
    if run.returncode:
        raise SystemExit("maker: backend-contract %s failed: %s" % (script, (run.stdout + run.stderr)[-800:]))
print("backend-contract merged copy:", T)
