#!/usr/bin/env python3
"""Copies the files the staged set edits (read only) into an overlay root so apply.py can be exercised against it."""
import os,shutil,sys
here=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,here)
import names,wire
ov=sys.argv[1] if len(sys.argv)>1 else "/private/tmp/perf-slo/ap";shutil.rmtree(ov,ignore_errors=True)
for rel in [r for r,_ in wire.EDITS+wire.WORKFLOW_EDITS]+[wire.HELP_FIXTURE]+wire.HELP_CLOSURE:
    dst=os.path.join(ov,rel);os.makedirs(os.path.dirname(dst),exist_ok=True);shutil.copyfile(os.path.join(names.R,rel),dst)
print(ov)
