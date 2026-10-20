#!/usr/bin/env python3
"""Copies the files the staged set edits (read only) into an overlay root so apply.py can be exercised against it; with `lease` also the lease, routing and validator modules for a behavioural run of the trace emitter."""
import os,shutil,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,wire
ov=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/conc-harness/ap";shutil.rmtree(ov,ignore_errors=True)
for rel,_ in wire.EDITS+wire.TRACE_EDITS:
    dst=os.path.join(ov,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(os.path.join(names.R,rel),dst)
if "lease" in sys.argv[2:]:
    F=f"{names.FRAMEWORK}/{names.MODULES}"
    for sub in (f"{F}/{names.PROCESS}/{names.LEASES}",f"{F}/{names.PROCESS}/{names.ROUTING}",f"{F}/{names.SCHEMA}/{names.VALIDATOR}"):
        shutil.copytree(os.path.join(names.R,sub),os.path.join(ov,sub),dirs_exist_ok=True,ignore=shutil.ignore_patterns("node_modules"))
print(ov)
