#!/usr/bin/env python3
"""Anchored, idempotent, all-or-nothing apply of the conc-deps set (see C/result-conc-deps.md).

usage: apply.py check [--group 1|2|3|all]                       read only; rc 0 applied, rc 1 pending, rc 2 refused
       apply.py apply [--group 1|2|3|all] [--write|--dry-run]   dry run unless --write; refuses on drift, and while a dependency ensure|sync|lock runs (--ignore-running overrides)
       apply.py mirror DIR [--apply] [--group 1|2|3|all]        scratch tree: real copies along the patched paths, symlinks elsewhere (never hard links), optionally with the set applied

Group 1 is the side-by-side generation protocol; group 2 (generation-scoped maintenance and image-fullness admission) is built on top of it:
`--group 2` applies only group 2 and refuses unless group 1 is already applied, `--group all` applies both in order, the default is group 1.
       apply.py render NAME OUT             writes the patched text of one target key (storage, bootstrap, ...) for tsc or diffing

Phase 1 reads every target and computes every new text in memory: each operation's old text (`staged/ops/<id>.old.txt`, cut by `mkops.py`) must occur exactly once,
or its marker must already be present. Any refusal prints REFUSED and writes nothing. Phase 2 writes each file through a temporary file and os.replace, with rollback.
"""
import hashlib
import json
import os
import shutil
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
OPS = os.path.join(STAGED, "ops")
DEFAULT_ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 8))
LIBRARY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DEPENDENCIES = LIBRARY + "/⚡️caching/🚀️bootstrap/📦️dependencies"
CONCURRENT = DEPENDENCIES + "/🚦️concurrent"
NEW = {
    "core.ts": CONCURRENT + "/🟦️.ts",
    "glue.ts": CONCURRENT + "/💾️storage/🟦️.ts",
    "schema.json": CONCURRENT + "/🧬️schema/🔣️.json",
    "concurrent.test.ts": CONCURRENT + "/🧪️tests/🟦️.ts",
    "concurrent.fixture.json": CONCURRENT + "/🧪️tests/🔣️.json",
    "concurrent.oracle.py": CONCURRENT + "/🧪️tests/🐍️oracle.py",
    "concurrent.reader.ts": CONCURRENT + "/🧪️tests/📖️reader.ts",
    "concurrent.switch.py": CONCURRENT + "/🧪️tests/🐍️switch.py",
}
FULL = [LIBRARY, "🧰️framework/🔨️modules/🏃️process", "🧰️framework/🔨️modules/📁️filesystem", "🧰️framework/🔨️modules/🧬️schema"]


class Refused(Exception):
    pass


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


GROUP_FILES = ["core.ts", "glue.ts", "schema.json", "concurrent.test.ts", "concurrent.fixture.json", "concurrent.oracle.py"]


def versions(key):
    """Every staged version of one new file as (group, text), oldest first; group 3 is cut from the live tree and lists the live texts it may replace."""
    rows = [(1, read(os.path.join(STAGED, key)))]
    for group in (2, 3):
        path = os.path.join(STAGED, "g%d" % group, key)
        if key in GROUP_FILES and os.path.exists(path):
            rows.append((group, read(path)))
    return rows


def known_bases(group):
    path = os.path.join(STAGED, "g%d" % group, "bases.json")
    return json.load(open(path, encoding="utf-8")) if os.path.exists(path) else {}


def text_sha(value):
    return hashlib.sha256(value.encode()).hexdigest()


PREREQUISITES = {2: 'acquireConcurrentLeaseV1("build"', 3: "async function admitJavascriptDependencyGenerationV1("}


def operations(upto=3, only=None):
    with open(os.path.join(STAGED, "ops.json"), encoding="utf-8") as handle:
        index = [row for row in json.load(handle) if row["group"] <= upto and (only is None or row["group"] == only)]
    rows = []
    for row in index:
        old = read(os.path.join(OPS, row["id"] + ".old.txt"))
        new = read(os.path.join(OPS, row["id"] + ".new.txt"))
        if hashlib.sha256(old.encode()).hexdigest() != row["oldSha256"]:
            raise Refused("staged old text of %s does not match its recorded sha256" % row["id"])
        rows.append({**row, "old": old, "new": new})
    return rows


def patch(text, row):
    if row["new"] and row["marker"] in text:
        return text, "present"
    found = text.count(row["old"])
    if found != 1:
        if not row["new"] and found == 0 and row["marker"] in text:
            return text, "present"
        raise Refused("anchor of '%s' must occur once in %s, found %d" % (row["id"], row["path"], found))
    return text.replace(row["old"], row["new"]), "replace"


def busy():
    out = subprocess.run(["ps", "-axo", "pid,command"], capture_output=True, text=True).stdout.splitlines()
    marks = ("dependencies sync", "dependencies lock", "dependencies ensure")
    return [line for line in out if any(mark in line for mark in marks) and "apply.py" not in line and "grep" not in line]


def plan(root, group="1"):
    upto = 3 if group == "all" else int(group)
    texts, rows = {}, []
    paths = {}
    only = int(group) if group in ("2", "3") else None
    for row in operations(upto, only):
        paths.setdefault(row["path"], []).append(row)
    for relative, items in paths.items():
        text = read(os.path.join(root, relative))
        if only and relative.endswith("💾️storage/🟦️.ts") and PREREQUISITES[only] not in text:
            raise Refused("group %d must be applied first (marker %s absent)" % (only - 1, PREREQUISITES[only]))
        for row in items:
            text, how = patch(text, row)
            rows.append((os.path.basename(os.path.dirname(relative)) + "/" + os.path.basename(relative), row["id"], how))
        texts[relative] = text
    for key, relative in NEW.items():
        rows_of = [row for row in versions(key) if row[0] <= upto]
        wanted = rows_of[-1][1]
        target = os.path.join(root, relative)
        current = read(target) if os.path.exists(target) else None
        allowed = {text_sha(text) for _, text in rows_of}
        for group_number in range(2, upto + 1):
            allowed |= set(known_bases(group_number).get(key, []))
        if current is not None and text_sha(current) not in allowed:
            raise Refused("new file %s exists with foreign content (neither staged nor a recorded base)" % relative)
        rows.append((relative.split("/")[-2] + "/" + key, "new file", "present" if current == wanted else "write"))
    return texts, rows


def run(root, write, guard=True, group="1"):
    try:
        texts, rows = plan(root, group)
    except Refused as error:
        print("REFUSED, nothing written: %s" % error)
        return 2
    for row in rows:
        print("%-34s %-40s %s" % row)
    pending = [row for row in rows if row[2] != "present"]
    print("%d operations, %d pending%s" % (len(rows), len(pending), "" if write else " (dry run)"))
    if write and pending:
        running = busy() if guard else []
        if running:
            print("REFUSED, a dependency ensure, sync or lock runs: %s" % running[0][:160])
            return 2
        originals = {}
        written = []
        try:
            for relative, text in texts.items():
                path = os.path.join(root, relative)
                if read(path) != text:
                    originals[path] = read(path)
                    with open(path + ".conc-deps.tmp", "w", encoding="utf-8") as handle:
                        handle.write(text)
                    os.replace(path + ".conc-deps.tmp", path)
                    written.append(path)
            for key, relative in NEW.items():
                target = os.path.join(root, relative)
                os.makedirs(os.path.dirname(target), exist_ok=True)
                originals[target] = read(target) if os.path.exists(target) else None
                upto = 3 if group == "all" else int(group)
                with open(target + ".conc-deps.tmp", "w", encoding="utf-8") as handle:
                    handle.write([row for row in versions(key) if row[0] <= upto][-1][1])
                os.replace(target + ".conc-deps.tmp", target)
                written.append(target)
        except Exception as error:
            for path in written:
                if originals.get(path) is None:
                    os.path.exists(path) and os.remove(path)
                else:
                    with open(path, "w", encoding="utf-8") as handle:
                        handle.write(originals[path])
            print("ROLLED BACK after %s" % error)
            return 2
    return 1 if pending else 0


def mirror(destination, root, with_set, group="all"):
    targets = {row["path"] for row in operations()} | set(NEW.values())
    chain = set()
    for relative in targets:
        parts = relative.split("/")
        for depth in range(1, len(parts)):
            chain.add("/".join(parts[:depth]))
    shutil.rmtree(destination, ignore_errors=True)
    os.makedirs(destination)

    def build(relative_dir):
        source = os.path.join(root, relative_dir) if relative_dir else root
        target = os.path.join(destination, relative_dir) if relative_dir else destination
        os.makedirs(target, exist_ok=True)
        for name in sorted(os.listdir(source)):
            child = (relative_dir + "/" + name) if relative_dir else name
            if child in chain:
                build(child)
            elif child in targets:
                shutil.copyfile(os.path.join(root, child), os.path.join(destination, child))
            elif not os.path.exists(os.path.join(destination, child)):
                os.symlink(os.path.join(root, child), os.path.join(destination, child))

    build("")
    skip = shutil.ignore_patterns("node_modules", "dist", "target", ".cache", "*.dmg")
    for relative in FULL:
        place = os.path.join(destination, relative)
        probe = destination
        for part in os.path.relpath(os.path.dirname(place), destination).split(os.sep):
            probe = os.path.join(probe, part)
            if os.path.islink(probe):
                os.unlink(probe)
                os.makedirs(probe)
            elif not os.path.exists(probe):
                os.makedirs(probe)
        if os.path.islink(place):
            os.unlink(place)
        elif os.path.isdir(place):
            shutil.rmtree(place)
        shutil.copytree(os.path.join(root, relative), place, symlinks=True, ignore=skip)
    for name in os.listdir(root):
        if name == "node_modules" and not os.path.lexists(os.path.join(destination, name)):
            os.symlink(os.path.join(root, name), os.path.join(destination, name))
    return run(destination, True, False, group) if with_set else 0


def main():
    args = sys.argv[1:]
    root = DEFAULT_ROOT
    if "--root" in args:
        root = os.path.abspath(args[args.index("--root") + 1])
    if not args or args[0] not in ("check", "apply", "mirror", "render"):
        raise SystemExit(__doc__)
    group = args[args.index("--group") + 1] if "--group" in args else "1"
    if group not in ("1", "2", "3", "all"):
        raise SystemExit(__doc__)
    if args[0] == "check":
        sys.exit(run(root, False, True, group))
    if args[0] == "apply":
        sys.exit(run(root, "--write" in args and "--dry-run" not in args, "--ignore-running" not in args, group))
    if args[0] == "render":
        texts, _ = plan(root, group if "--group" in args else "all")
        keys = {row["target"]: row["path"] for row in json.load(open(os.path.join(STAGED, "ops.json"), encoding="utf-8"))}
        with open(args[2], "w", encoding="utf-8") as handle:
            handle.write(texts[keys[args[1]]])
        sys.exit(0)
    code = mirror(os.path.abspath(args[1]), root, "--apply" in args, group if "--group" in args else "all")
    sys.exit(code if code in (0, 1) else 2)


main()
