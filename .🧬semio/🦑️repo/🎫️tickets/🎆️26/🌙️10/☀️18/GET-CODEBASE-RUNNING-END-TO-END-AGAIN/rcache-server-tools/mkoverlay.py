#!/usr/bin/env python3
# usage: mkoverlay.py   rebuilds /private/tmp/rcache-server/ov: copies of the live files the staged set touches, the staged set applied on top, the relative-import closure and dereferenced copies of ajv and protobufjs (verification only; the live tree is never written)
import os, re, shutil, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

BASELINE = "--baseline" in sys.argv
OV = "/private/tmp/rcache-server/" + ("base" if BASELINE else "ov")
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
TREES = [L + "/🏗️build/⚙️operations/🧪️tests", L + "/⚡️caching/💾️storage/🧾️ledger/🧪️tests", L + "/🏗️build/🧹️cache-gc/🧪️tests", L + "/🏗️build/🔧️tools/🏗️bazel/💾️storage/🧪️tests"]
IMPORT = re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+)"(\.{1,2}/[^"]+)"''')


def copy(relative):
    source = os.path.join(ROOT, relative)
    target = os.path.join(OV, relative)
    if os.path.isfile(source) and not os.path.exists(target):
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copy2(source, target)


def main():
    if os.path.isdir(OV):
        for base, dirs, names in os.walk(OV):
            pass
        shutil.rmtree(OV)
    os.makedirs(OV)
    targets = load("targets")
    edits = load("edits").EDITS
    touched = sorted({e["file"] for e in edits} | set(targets.REPLACE.values()))
    for relative in touched:
        copy(relative)
    writes, problems, notes = ([], [], []) if BASELINE else plan(OV)
    if problems:
        print("\n".join(problems))
        sys.exit(1)
    for relative, content in writes:
        path = os.path.join(OV, relative)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf8", newline="") as handle:
            handle.write(content)
    pending = [r for r, _ in writes]
    for tree in TREES:
        for base, dirs, names in os.walk(os.path.join(ROOT, tree)):
            for name in names:
                relative = os.path.relpath(os.path.join(base, name), ROOT)
                copy(relative)
                if relative.endswith((".ts", ".mjs", ".js")):
                    pending.append(relative)
    seen = set()
    while pending:
        relative = pending.pop()
        if relative in seen:
            continue
        seen.add(relative)
        copy(relative)
        if not relative.endswith((".ts", ".mjs", ".js")):
            continue
        path = os.path.join(OV, relative)
        if not os.path.isfile(path):
            continue
        for match in IMPORT.finditer(read(path)):
            target = os.path.normpath(os.path.join(os.path.dirname(relative), match.group(1)))
            if os.path.isfile(os.path.join(ROOT, target)) or os.path.isfile(os.path.join(OV, target)):
                pending.append(target)
    if BASELINE:
        for relative in touched:
            pending_extra = relative
    for extra in extras():
        copy(extra)
    modules = os.path.join(OV, "node_modules")
    os.makedirs(modules, exist_ok=True)
    store = os.path.realpath(os.path.join(ROOT, "node_modules")) + "/.bun"
    for package in sorted(os.listdir(store)):
        if re.match(r"^(ajv@8|protobufjs@8\.8\.1)", package):
            subprocess.run(["cp", "-RL", store + "/" + package + "/node_modules/.", modules], check=True)
    if not BASELINE:
        subprocess.run(["bun", os.path.join(TOOLS, "gen-help.ts")], cwd=OV, check=True)
    print(f"overlay ready: {len(writes)} written, {len(seen)} files in the closure")


def extras():
    return ["package.json", "bunfig.toml"]


main()
