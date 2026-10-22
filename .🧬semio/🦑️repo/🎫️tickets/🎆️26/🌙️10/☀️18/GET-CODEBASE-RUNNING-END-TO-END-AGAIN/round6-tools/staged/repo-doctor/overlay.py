#!/usr/bin/env python3
"""Builds a path-spine overlay of the checkout under /private/tmp/repo-doctor: every directory on the way to a staged or patched file is real, everything else is a symlink to the live tree (nothing under the checkout is written).

usage: overlay.py [OVERLAY_DIR] [--no-patches] [--bare]   (--bare: only the files apply.py edits, as real copies, for an apply rehearsal)"""
import os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import paths  # noqa: E402
import patches  # noqa: E402

J = os.path.join
E_PROJECT = paths.E(0x1F4CB) + "project.json"
NODE = {
    "ajv": "ajv@8.20.0", "fast-deep-equal": "fast-deep-equal@3.1.3", "fast-uri": "fast-uri@3.1.8",
    "json-schema-traverse": "json-schema-traverse@1.0.0", "require-from-string": "require-from-string@2.0.2",
}


def bare():
    files = {}
    for key in patches.EDITORS:
        with open(J(paths.ROOT, paths.TARGET[key]), "rb") as handle:
            files[paths.TARGET[key]] = handle.read()
    with open(J(paths.ROOT, paths.TARGET["help-snapshots"]), "rb") as handle:
        files[paths.TARGET["help-snapshots"]] = handle.read()
    return files


def contents(include_patches):
    files = {target: patches.staged(name).encode("utf-8") for name, target in paths.NEW.items() if os.path.exists(J(patches.STAGED, name))}
    if include_patches:
        for key, editor in patches.EDITORS.items():
            with open(J(paths.ROOT, paths.TARGET[key]), encoding="utf-8") as handle:
                files[paths.TARGET[key]] = editor(handle.read()).encode("utf-8")
        tests = J(paths.B, paths.OPERATIONS, paths.TESTS)
        for folder, _, names in os.walk(J(paths.ROOT, tests)):
            for name in names:
                absolute = J(folder, name)
                with open(absolute, "rb") as handle:
                    files[os.path.relpath(absolute, paths.ROOT)] = handle.read()
    for name in ("package.json", E_PROJECT):
        with open(J(paths.ROOT, name), "rb") as handle:
            files[name] = handle.read()
    return files


def build(overlay, include_patches=True, extra=None, only_targets=False):
    files = bare() if only_targets else contents(include_patches)
    files.update(extra or {})
    tree = {}
    for relative, data in files.items():
        node = tree
        parts = relative.split(os.sep)
        for part in parts[:-1]:
            node = node.setdefault(part, {})
        node[parts[-1]] = data
    shutil.rmtree(overlay, ignore_errors=True)

    def walk(live, destination, node):
        os.makedirs(destination, exist_ok=True)
        try:
            names = os.listdir(live)
        except OSError:
            names = []
        for name in names:
            if name in node or name == ".git" or (live == paths.ROOT and name == "node_modules"):
                continue
            os.symlink(J(live, name), J(destination, name))
        for name, child in node.items():
            if isinstance(child, dict):
                walk(J(live, name), J(destination, name), child)
            else:
                with open(J(destination, name), "wb") as handle:
                    handle.write(child)

    walk(paths.ROOT, overlay, tree)
    if only_targets:
        return overlay
    modules = J(overlay, "node_modules")
    live_modules = J(paths.ROOT, "node_modules")
    os.makedirs(modules, exist_ok=True)
    for name in os.listdir(live_modules):
        source = J(live_modules, name)
        if name in NODE:
            shutil.copytree(J(paths.ROOT, "node_modules", ".bun", NODE[name], "node_modules", name), J(modules, name), symlinks=False)
        elif name == "@types":
            os.makedirs(J(modules, name), exist_ok=True)
            for child in os.listdir(source):
                if child in ("node", "bun"):
                    shutil.copytree(os.path.realpath(J(source, child)), J(modules, name, child), symlinks=False)
                else:
                    os.symlink(J(source, child), J(modules, name, child))
        else:
            os.symlink(source, J(modules, name))
    for folder, name in (("bun-types@1.4.3", "bun-types"), ("undici-types@7.24.6", "undici-types")):
        source = J(paths.ROOT, "node_modules", ".bun", folder, "node_modules", name)
        target = J(modules, name)
        if os.path.isdir(source) and not os.path.lexists(target):
            shutil.copytree(source, target, symlinks=False)
    return overlay


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    out = build(args[0] if args else "/private/tmp/repo-doctor/ov", "--no-patches" not in sys.argv, only_targets="--bare" in sys.argv)
    print("overlay", out)
