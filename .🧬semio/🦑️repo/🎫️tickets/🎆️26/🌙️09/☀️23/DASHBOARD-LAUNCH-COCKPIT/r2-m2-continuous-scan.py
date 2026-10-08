#!/usr/bin/env python3
"""M-2 round 2b helper: lists Nx targets that look like servers or watchers (name or declared `ready`) and lack `"continuous": true`.
Usage (repository root): python r2-m2-continuous-scan.py > candidates.txt"""
import json
import os
import re
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8")
listing = lambda *args: subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", *args, "--", "*project.json"], capture_output=True, encoding="utf8").stdout.split("\n")
files = [f for f in listing() + listing("-o", "--exclude-standard") if f and os.path.exists(f)]
name = re.compile(r"^(dev|serve|start|watch|preview|storybook|activate)(-|$)|(-)(dev|serve|watch|storybook)(-|$)")
for file in files:
    try:
        manifest = json.load(open(file, encoding="utf8"))
    except ValueError:
        continue
    for target, spec in (manifest.get("targets") or {}).items():
        if spec.get("continuous"):
            continue
        ready = (((spec.get("metadata") or {}).get("semio") or {}).get("dashboard") or {}).get("ready")
        if ready or name.search(target):
            options = spec.get("options") or {}
            command = options.get("command") or options.get("commands") or spec.get("executor")
            print(json.dumps({"project": manifest.get("name"), "target": target, "ready": bool(ready), "command": str(command)[:110], "file": file}, ensure_ascii=False))
