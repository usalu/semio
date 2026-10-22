#!/usr/bin/env python3
# usage: mkoverlay.py [--clean]   builds /private/tmp/rexec-inhouse/ov: the staged files at their live paths, copies of the live files they import (relative-import closure), and dereferenced copies of ajv and protobufjs.
# Verification only; the live tree is never written.
import importlib.util, os, re, shutil, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from lib import *

BASELINE = "--baseline" in sys.argv
OV = "/private/tmp/rexec-inhouse/" + ("base" if BASELINE else "ov")
IMPORT = re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+)"(\.{1,2}/[^"]+)"''')


def copy(relative):
    source = os.path.join(ROOT, relative)
    target = os.path.join(OV, relative)
    if os.path.isfile(source) and not os.path.exists(target):
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copy2(source, target)


PREREQUISITES = ["rcache-server-tools"]


def apply_prerequisites():
    """applies the staged sets this one builds on (their own apply logic, on the overlay only), once per overlay."""
    marker = os.path.join(OV, ".prerequisites")
    if os.path.exists(marker):
        return
    for name in PREREQUISITES:
        directory = os.path.join(os.path.dirname(TOOLS), name)
        spec = importlib.util.spec_from_file_location("pre_" + name.replace("-", "_") + "_lib", os.path.join(directory, "lib.py"))
        module = importlib.util.module_from_spec(spec)
        sys.path.insert(0, directory)
        spec.loader.exec_module(module)
        targets, edits = module.load("targets"), module.load("edits").EDITS
        touched = sorted({e["file"] for e in edits} | set(getattr(targets, "REPLACE", {}).values()))
        for relative in touched:
            copy(relative)
        writes, problems, notes = module.plan(OV)
        if problems:
            print("\n".join(problems))
            sys.exit(1)
        for relative, content in writes:
            path = os.path.join(OV, relative)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w", encoding="utf8", newline="") as handle:
                handle.write(content)
        sys.path.remove(directory)
        print(f"prerequisite {name}: {len(writes)} file(s) applied to the overlay")
    open(marker, "w").write("done")


PHYSICAL = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/🧬️schema"]


def physical_copies():
    """real Bazel runs read directories and sibling files the import closure does not name (revision capture), so the library and the two modules it builds on are cloned whole (APFS clones, copy-on-write)."""
    marker = os.path.join(OV, ".physical")
    if os.path.exists(marker):
        return
    for relative in PHYSICAL:
        target = os.path.join(OV, relative)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        subprocess.run(["cp", "-Rc" if sys.platform == "darwin" else "-R", os.path.join(ROOT, relative), target], check=True)
    open(marker, "w").write("done")


def main():
    if "--clean" in sys.argv and os.path.isdir(OV):
        shutil.rmtree(OV)
    os.makedirs(OV, exist_ok=True)
    if "--physical" in sys.argv or os.path.exists(os.path.join(OV, ".physical")):
        physical_copies()
    apply_prerequisites()
    targets = load("targets")
    if BASELINE:
        print("baseline overlay ready (live copies + prerequisites, nothing of this set)")
        return
    pending = []
    for name, relative in {**targets.NEW, **targets.REPLACE}.items():
        if not os.path.isfile(os.path.join(STAGED, name)):
            continue
        path = os.path.join(OV, relative)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        shutil.copyfile(os.path.join(STAGED, name), path)
        if relative.endswith((".ts", ".mjs", ".js")):
            pending.append(relative)
    touched = sorted({e["file"] for e in load("edits").EDITS})
    for relative in touched:
        copy(relative)
    edited = [e["file"] for e in load("edits").EDITS]
    mine_edits = [e for e in load("edits").EDITS]
    by_file = {}
    for edit in mine_edits:
        by_file.setdefault(edit["file"], []).append(edit)
    for relative, group in by_file.items():
        path = os.path.join(OV, relative)
        if not os.path.isfile(path):
            print("missing edit target in overlay:", relative)
            sys.exit(1)
        updated, _ = apply_edits(read(path), group)
        with open(path, "w", encoding="utf8", newline="") as handle:
            handle.write(updated)
    for relative in getattr(targets, "REMOVE", []):
        path = os.path.join(OV, relative)
        if os.path.exists(path):
            os.remove(path)
    seen = set()
    while pending:
        relative = pending.pop()
        if relative in seen:
            continue
        seen.add(relative)
        path = os.path.join(OV, relative)
        if not os.path.isfile(path):
            continue
        for match in IMPORT.finditer(read(path)):
            target = os.path.normpath(os.path.join(os.path.dirname(relative), match.group(1)))
            copy(target)
            if os.path.isfile(os.path.join(OV, target)) and target.endswith((".ts", ".mjs", ".js")):
                pending.append(target)
    for extra in ["package.json", "bunfig.toml", ".bazelrc"]:
        copy(extra)
    modules = os.path.join(OV, "node_modules")
    if not os.path.isdir(modules):
        os.makedirs(modules)
        store = os.path.realpath(os.path.join(ROOT, "node_modules")) + "/.bun"
        for package in sorted(os.listdir(store)):
            if re.match(r"^(ajv@8|protobufjs@8\.8\.1)", package):
                subprocess.run(["cp", "-RL", store + "/" + package + "/node_modules/.", modules], check=True)
    print(f"overlay ready: {len(targets.NEW)} staged, {len(seen)} files in the closure")


main()
