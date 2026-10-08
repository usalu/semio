#!/usr/bin/env python3
"""M-2 round 2b helper: Nx targets (outside tickets) whose command contains a server or watcher word but that are not `continuous`.
Usage (repository root): python r2-m2-continuous-scan2.py"""
import json
import os
import subprocess
import sys

sys.stdout.reconfigure(encoding="utf-8")
listing = lambda *args: subprocess.run(["git", "-c", "core.quotepath=false", "ls-files", *args, "--", "*project.json"], capture_output=True, encoding="utf8").stdout.split("\n")
files = [f for f in listing() + listing("-o", "--exclude-standard") if f and os.path.exists(f) and "🎫️tickets" not in f]
words = (" serve", " dev", " watch", " start", "listen", "vite", "storybook", "--watch", " http", " preview")
skip = ("test", "build", "verify", "check", "typecheck", "lint", "generate", "describe", "package", "setup", "prepare")
for file in files:
    try:
        manifest = json.load(open(file, encoding="utf8"))
    except ValueError:
        continue
    for target, spec in (manifest.get("targets") or {}).items():
        if spec.get("continuous") or target.startswith(skip):
            continue
        options = spec.get("options") or {}
        command = str(options.get("command") or options.get("commands") or "")
        if any(word in command for word in words):
            print(manifest.get("name"), target, command[:110])
