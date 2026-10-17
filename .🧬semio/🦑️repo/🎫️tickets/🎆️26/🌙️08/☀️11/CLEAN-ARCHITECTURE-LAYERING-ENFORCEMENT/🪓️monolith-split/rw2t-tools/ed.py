"""✏️ rw2t anchored editor: replacements with asserted anchor counts, a re-read right before the write and a first pre-edit copy in the scratchpad."""
import os, shutil, sys
REPO = "/Users/ueli/Documents/semio"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rw2t"
PRE = SCRATCH + "/pre"
TOUCHED = os.path.dirname(os.path.abspath(__file__)) + "/touched.txt"

def backup(path):
    rel = os.path.relpath(path, REPO)
    dst = PRE + "/" + rel
    if os.path.exists(path) and not os.path.exists(dst):
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(path, dst)

def note(path):
    rel = os.path.relpath(path, REPO)
    have = set(open(TOUCHED, encoding="utf8").read().split("\n")) if os.path.exists(TOUCHED) else set()
    if rel not in have:
        with open(TOUCHED, "a", encoding="utf8") as h:
            h.write(rel + "\n")

def read(path):
    with open(path, encoding="utf8") as h:
        return h.read()

def write_new(path, text):
    assert not os.path.exists(path), "exists: " + path
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf8") as h:
        h.write(text)
    note(path)
    print("created", path[-80:])

def replace(path, pairs, dry=False):
    text = read(path)
    orig = text
    for old, new, n in pairs:
        c = text.count(old)
        assert c == n, f"{path[-70:]}: anchor {old[:70]!r} count {c} != {n}"
        text = text.replace(old, new)
    if dry:
        print("dry ok", path[-70:])
        return
    backup(path)
    assert read(path) == orig, "changed while editing: " + path
    with open(path, "w", encoding="utf8") as h:
        h.write(text)
    note(path)
    print("edited", path[-80:])

def overwrite(path, text):
    backup(path)
    with open(path, "w", encoding="utf8") as h:
        h.write(text)
    note(path)
    print("overwrote", path[-80:])
