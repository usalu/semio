#!/usr/bin/env python3
"""Builds a path-merge overlay of the repository under OV: directories on the way to a staged file are real, every other entry is a symlink to the live tree, so relative imports reach live siblings and the staged files shadow their live originals. Usage: overlay.py [--ov DIR]"""
import os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import layout
OV = "/private/tmp/bzt/ov"

def staged_files(live=False):
    out = {}
    for flat in (list(layout.EDIT) if live else list(layout.NEW) + list(layout.EDIT)):
        src = layout.dest(flat) if live else os.path.join(layout.HERE, "staged", flat)
        if os.path.exists(src):
            out[layout.dest(flat)] = src
    return out

REAL = [layout.BZ + "/🧪️tests/" + name for name in ("📥️coverage", "📥️actions", "📤️arguments", "📥️sources", "📥️workspace", "📥️inventory", "📥️delegation")] + [layout.B + "/🧭️selection/🧪️tests", layout.F + "/🧪️testing/🎛️execution/🧪️tests", layout.F + "/🎛️owned-execution/🧪️tests", layout.F + "/🎛️owned-execution/🧪️tests/📤️progress", layout.F + "/🎛️owned-execution/🧪️tests/📤️stdout", layout.F + "/🎛️owned-execution/🧪️tests/📤️stream", layout.F + "/🎛️owned-execution/🧪️tests/📬️output", layout.L + "/🏃️process/🎛️owned-execution/🧪️tests", layout.L + "/🏃️process/🎛️owned-execution/🧪️tests/📥️original-control"]

def adapter_tests():
    root = os.path.join(layout.ROOT, layout.BZ, "🧪️tests")
    return [layout.BZ + "/🧪️tests/" + name for name in sorted(os.listdir(root)) if os.path.isdir(os.path.join(root, name))] if os.path.isdir(root) else []

def build(ov=OV, extra=(), live=False):
    files = staged_files(live)
    extra = list(extra) + adapter_tests()
    shutil.rmtree(ov, ignore_errors=True)
    trie = {}
    for dest, src in files.items():
        node = trie
        parts = os.path.relpath(dest, layout.ROOT).split(os.sep)
        for part in parts[:-1]:
            node = node.setdefault(part, {})
        node[parts[-1]] = src
    for rel in list(REAL) + list(extra):
        node = trie
        for part in rel.split("/"):
            node = node.setdefault(part, {})
    def walk(live, target, node):
        os.makedirs(target, exist_ok=True)
        entries = set(os.listdir(live)) if os.path.isdir(live) else set()
        for name in entries - set(node):
            source = os.path.join(live, name)
            if os.path.isfile(source) and not os.path.islink(source) and live != layout.ROOT:
                shutil.copyfile(source, os.path.join(target, name))
            else:
                os.symlink(source, os.path.join(target, name))
        for name, child in node.items():
            if isinstance(child, dict):
                walk(os.path.join(live, name), os.path.join(target, name), child)
            else:
                shutil.copyfile(child, os.path.join(target, name))
    walk(layout.ROOT, ov, trie)
    return len(files)

if __name__ == "__main__":
    ov = OV
    if "--ov" in sys.argv:
        ov = sys.argv[sys.argv.index("--ov") + 1]
    print(build(ov, live="--live" in sys.argv), "files in", ov)
