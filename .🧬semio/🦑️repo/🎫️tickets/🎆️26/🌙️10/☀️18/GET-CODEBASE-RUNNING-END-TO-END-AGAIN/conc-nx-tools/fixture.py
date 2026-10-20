#!/usr/bin/env python3
"""Renders a stock-Nx fixture workspace: N projects with one cacheable run-commands target each (writes out.txt), a shared cache directory, git repo.
usage: fixture.py DIR [PROJECTS=200] [FILES_PER_PROJECT=20]"""
import json, os, subprocess, sys

root = os.path.abspath(sys.argv[1])
count = int(sys.argv[2]) if len(sys.argv) > 2 else 200
files = int(sys.argv[3]) if len(sys.argv) > 3 else 20
os.makedirs(root, exist_ok=True)
json.dump({"cacheDirectory": ".cache-nx", "useDaemonProcess": False, "namedInputs": {"default": ["{projectRoot}/**/*"]}, "analytics": False}, open(os.path.join(root, "nx.json"), "w"))
json.dump({"name": "fixture", "private": True, "workspaces": []}, open(os.path.join(root, "package.json"), "w"))
open(os.path.join(root, ".gitignore"), "w").write("node_modules\n.nx\n.cache-nx\nout.txt\n")
for index in range(count):
    base = os.path.join(root, "p%04d" % index)
    os.makedirs(base, exist_ok=True)
    for number in range(files):
        open(os.path.join(base, "f%03d.txt" % number), "w").write("project %d file %d\n" % (index, number))
    json.dump({"name": "p%04d" % index, "targets": {"build": {"executor": "nx:run-commands", "cache": True, "inputs": ["default"], "outputs": ["{projectRoot}/out.txt"], "options": {"command": "node -e \"const fs=require('fs');const c=require('crypto');const d=fs.readdirSync('.').filter(f=>f!=='out.txt').sort().map(f=>fs.readFileSync(f)).join('');fs.writeFileSync('out.txt',c.createHash('sha256').update(d).digest('hex')+'\\\\n')\"", "cwd": "{projectRoot}"}}}}, open(os.path.join(base, "project.json"), "w"))
subprocess.run(["git", "init", "-q"], cwd=root)
subprocess.run(["git", "add", "-A"], cwd=root)
subprocess.run(["git", "-c", "user.email=a@b", "-c", "user.name=x", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "commit", "-qm", "fx"], cwd=root)
print("fixture", root, count, "projects")
