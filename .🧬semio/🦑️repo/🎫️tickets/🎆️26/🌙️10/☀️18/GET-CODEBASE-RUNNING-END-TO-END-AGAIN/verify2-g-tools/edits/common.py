"""Shared helpers of the verify2-g edit scripts: files are copied from the live tree into `work/` once (never overwritten) and edited there, so ticket tooling is the single source of truth and no scratch mirror is needed."""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = os.path.dirname(HERE)
ROOT = TOOLS
while not os.path.isdir(os.path.join(ROOT, "🧰️framework")) and ROOT != "/":
    ROOT = os.path.dirname(ROOT)
WORK = os.path.join(TOOLS, "work")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B = L + "/🏗️build"
N = "🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes"
J = B + "/🏡️resident/📜️journal"

def take(rel):
    target = os.path.join(WORK, rel)
    if not os.path.exists(target):
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(os.path.join(ROOT, rel), target)
    return target

def sub(rel, old, new, count=1, marker=None):
    path = take(rel)
    text = open(path, encoding="utf8").read()
    if marker is not None and marker in text:
        return
    if new in text:
        return
    assert text.count(old) == count, (rel, old[:60], text.count(old))
    open(path, "w", encoding="utf8").write(text.replace(old, new))

def put(rel, text):
    path = os.path.join(WORK, rel)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf8").write(text)
