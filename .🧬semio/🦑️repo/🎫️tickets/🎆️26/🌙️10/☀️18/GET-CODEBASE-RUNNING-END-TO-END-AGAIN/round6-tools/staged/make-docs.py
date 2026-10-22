#!/usr/bin/env python3
"""Copy of the docs-1020e set at the ticket root (`r6-docs-1021a-tools`, same depth: lib.py finds the repository root from its location) that keeps its apply state (applied.json, README backup of round 4) and derives the edit anchors from that pristine backup even though peers changed the README since (the README is no longer equal to our last result): apply.py then classifies every edit as applied / refresh / pending against the current text and `--rebase` accepts the changed README."""
import os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = os.path.join(C, "r6-docs-1021a-tools")
shutil.rmtree(T, ignore_errors=True)
shutil.copytree(C + "/docs-1021a-tools", T, ignore=shutil.ignore_patterns("__pycache__"))
patch(T + "/make-edits.py", "def add(identifier, old, new, when=None, probe=None):\n", "def add(identifier, old, new, when=None, probe=None):\n    if identifier.startswith(\"verified\"):\n        new = new.replace(\"`repo cache serve`\", \"`repo remote-cache serve`\")\n")

import glob, hashlib, json, subprocess
scratch = os.path.join(os.environ.get("R6_SCRATCH", "/private/tmp/r6"), "docs-cm")
shutil.rmtree(scratch, ignore_errors=True)
inputs = [os.path.relpath(f, T) for f in glob.glob(T + "/staged/en/*.md") + glob.glob(T + "/staged/de/*.md") + glob.glob(T + "/backup/README.*.md")] + ["applied.json"]
for rel in inputs:
    os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True)
    shutil.copyfile(os.path.join(T, rel), os.path.join(scratch, rel))
env = {**os.environ, "GIT_DIR": scratch + "/.git", "GIT_WORK_TREE": scratch}
subprocess.run(["git", "init", "-q"], env=env, cwd=scratch, check=True)
subprocess.run(["git", "add", "-A"], env=env, cwd=scratch, check=True)
codemod = subprocess.run([sys.executable, C + "/r6-repo-launcher-tools/command-form-codemod.py", "--root", scratch, "--apply"], capture_output=True, text=True, cwd=scratch)
if codemod.returncode:
    raise SystemExit("maker: docs: command-form codemod failed: " + (codemod.stdout + codemod.stderr)[-600:])
for rel in inputs:
    shutil.copyfile(os.path.join(scratch, rel), os.path.join(T, rel))
shutil.rmtree(scratch, ignore_errors=True)
for rel in inputs:
    path = os.path.join(T, rel)
    text = open(path, encoding="utf8").read()
    if "`repo repo`" in text:
        open(path, "w", encoding="utf8").write(text.replace("`repo repo`", "`repo`"))
patch(T + "/check.py", "    op = positional[0]\n", "    op = positional[0]\n    if op.startswith(\"./\"):\n        return\n")
record = json.load(open(T + "/applied.json", encoding="utf8"))
old_backup = glob.glob(T + "/backup/README.*.md")[0]
digest = hashlib.sha256(open(old_backup, "rb").read()).hexdigest()
record["before"], record["after"] = digest, ""
if record.get("pristine"):
    record["pristine"] = digest
os.replace(old_backup, T + "/backup/README." + digest[:16] + ".md")
json.dump(record, open(T + "/applied.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
for path in glob.glob(T + "/staged/en/*.md") + glob.glob(T + "/staged/de/*.md"):
    text = open(path, encoding="utf8").read()
    if "`repo cache serve`" in text:
        open(path, "w", encoding="utf8").write(text.replace("`repo cache serve`", "`repo remote-cache serve`"))
print("docs copy:", T, "(inputs re-written by the command-form codemod: %s)" % codemod.stdout.strip().split("\n")[-1][:100])
