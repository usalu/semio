#!/usr/bin/env python3
"""Merged copy of the backend-contract set: the live child door decides the nx bootstrap from the catalogue (`command!=="watch"&&!buildOperationHasNeutralBootstrapV1(command)`), so the wire anchor accepts that form and the nx backend's `bootstrapArguments` carries the same rule (`conformance` requires dependencies like `watch`, so it is listed beside it). The staged files are re-rendered by build.py."""
import os, re, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/backend-contract-tools", "backend-contract")
patch(T + "/wire.py", r'selected\.tool==="nx"&&!\[[^\]]*\]\.includes\(command\?\?""\)\?nxOperationArgumentsV1', r'selected\.tool==="nx"&&(?:!\[[^\]]*\]\.includes\(command\?\?""\)|command!=="watch"&&!buildOperationHasNeutralBootstrapV1\(command\?\?""\))\?nxOperationArgumentsV1')
NX = T + "/src/nx_backend.ts"
text = open(NX, encoding="utf8").read()
ops = re.search(r'@@UP@@[^/"]*operations/[^"]+', text).group(0)
line = re.search(r'import \{nxOperationArgumentsV1\} from "[^"]+";\n', text).group(0)
text = text.replace(line, line + 'import {buildOperationHasNeutralBootstrapV1} from "' + ops + '";\n', 1)
old = '["dependencies","cargo","watch","preflight","incremental","concurrency","conformance"].includes(command??"")?undefined:nxOperationArgumentsV1(args)'
assert text.count(old) == 1
text = text.replace(old, '["watch","conformance"].includes(command??"")||buildOperationHasNeutralBootstrapV1(command??"")?undefined:nxOperationArgumentsV1(args)')
open(NX, "w", encoding="utf8").write(text)
run = subprocess.run([sys.executable, T + "/build.py"], capture_output=True, text=True, cwd=T)
if run.returncode:
    raise SystemExit("maker: backend-contract build.py failed: " + (run.stdout + run.stderr)[-600:])
print("backend-contract merged copy:", T)
