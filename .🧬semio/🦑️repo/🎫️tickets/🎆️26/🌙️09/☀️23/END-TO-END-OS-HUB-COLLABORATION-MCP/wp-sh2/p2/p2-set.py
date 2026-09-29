#!/usr/bin/env python3
"""🧷️ SH2 P2 prepared set (Space activity: the kernel folds checkpoint publications into per-document activity; the Space index's
directory projection and presence move to its Transient lane, folded incrementally from bounded delta batches; the host feeds them).

Subcommands:
  add <path…>       list repo paths in `p2-files.txt`, seed base (tree content) and stage (copy) under `.🧬semio/🌐hub/s14-sh2-p2-*`
  rebase            re-base every listed path the tree moved: the stage merges three-way onto the tree, then the base
                    becomes the tree (a conflict leaves both untouched and is reported)
  capture           write `payload/<n>.old|.new` + `payload/manifest.json` for every changed path
  apply [--write] [--root <dir>]
                    dry run by default; a file equal to `.old` is replaced, equal to `.new` counts as applied, a peer-changed file
                    merges three-way (`git merge-file -p`) and applies only when conflict-free; tree writes back up under
                    `.🧬semio/🌐hub/s14-sh2-p2-backup/<stamp>/`
  revert [--root <dir>]
                    restores the newest backup of a `--write` on that root
  clear-created --root <scratch>
                    removes the files the set creates from a scratch root (an overlay re-synced from the tree keeps them)
"""
import json, os, shutil, subprocess, sys, tempfile, time

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
HUB = os.path.join(REPO, ".🧬semio/🌐hub")
BASE, STAGE, BACKUP = (os.path.join(HUB, f"s14-sh2-p2-{name}") for name in ("base", "stage", "backup"))
LIST, PAYLOAD = os.path.join(HERE, "p2-files.txt"), os.path.join(HERE, "payload")


def read(path):
    return open(path, encoding="utf-8").read() if os.path.exists(path) else None


def listed():
    return [line.rstrip("\n") for line in open(LIST, encoding="utf-8") if line.strip()] if os.path.exists(LIST) else []


def seed(path, stage_too):
    source = os.path.join(REPO, path)
    for root, copy in ((BASE, True), (STAGE, stage_too)):
        if not copy:
            continue
        target = os.path.join(root, path)
        os.makedirs(os.path.dirname(target), exist_ok=True)
        if os.path.exists(source):
            shutil.copy2(source, target)
        elif os.path.exists(target) and root == BASE:
            os.remove(target)


def merge(current, old, new):
    with tempfile.TemporaryDirectory() as scratch:
        names = []
        for label, text in (("current", current), ("old", old), ("new", new)):
            path = os.path.join(scratch, label)
            open(path, "w", encoding="utf-8").write(text)
            names.append(path)
        result = subprocess.run(["git", "merge-file", "-p", *names], capture_output=True, text=True)
    return result.stdout, result.returncode == 0


def plan(root, entry):
    target = os.path.join(root, entry["path"])
    old, new, current = read(os.path.join(PAYLOAD, f"{entry['id']}.old")), read(os.path.join(PAYLOAD, f"{entry['id']}.new")), read(target)
    if entry["mode"] == "create":
        return ("apply", target, new) if current is None else (("applied", target, None) if current == new else ("conflict: file exists with other content", target, None))
    if entry["mode"] == "delete":
        return ("applied", target, None) if current is None else (("apply", target, None) if current == old else ("conflict: file differs from the prepared base", target, None))
    if current is None:
        return "conflict: file missing", target, None
    if current == new:
        return "applied", target, None
    if current == old:
        return "apply", target, new
    merged, clean = merge(current, old, new)
    return ("apply (3-way merge over a peer's change)", target, merged) if clean else ("conflict: 3-way merge has conflicts", target, None)


def capture():
    for name in os.listdir(PAYLOAD):
        os.remove(os.path.join(PAYLOAD, name))
    manifest, paths = [], listed()
    for index, relative in enumerate(paths):
        old, new = read(os.path.join(BASE, relative)), read(os.path.join(STAGE, relative))
        if old == new:
            continue
        hunk = f"{index:03d}"
        for suffix, text in (("old", old), ("new", new)):
            if text is not None:
                open(os.path.join(PAYLOAD, f"{hunk}.{suffix}"), "w", encoding="utf-8").write(text)
        manifest.append({"id": hunk, "path": relative, "mode": "create" if old is None else ("delete" if new is None else "whole")})
    json.dump(manifest, open(os.path.join(PAYLOAD, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"captured {len(manifest)} changed file(s) of {len(paths)}")


def apply(arguments):
    write, root = "--write" in arguments, os.path.abspath(arguments[arguments.index("--root") + 1]) if "--root" in arguments else REPO
    manifest = json.load(open(os.path.join(PAYLOAD, "manifest.json"), encoding="utf-8"))
    results = [(entry, *plan(root, entry)) for entry in manifest]
    for entry, state, _, _ in results:
        print(f"{state:>44}  {entry['id']}  {entry['path']}")
    conflicts = [r for r in results if r[1].startswith("conflict")]
    print(f"files={len(results)} apply={sum(r[1].startswith('apply') for r in results)} applied={sum(r[1] == 'applied' for r in results)} conflicts={len(conflicts)} root={root}")
    if not write or conflicts:
        return 1 if conflicts else 0
    stamp = os.path.join(BACKUP, time.strftime("%Y%m%d-%H%M%S"))
    for entry, state, target, content in results:
        if not state.startswith("apply"):
            continue
        if os.path.exists(target):
            backup = os.path.join(stamp, os.path.relpath(target, root))
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            shutil.copy2(target, backup)
        if entry["mode"] == "delete":
            os.remove(target)
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        open(target, "w", encoding="utf-8").write(content)
    json.dump({"root": root, "created": [e["path"] for e, s, _, _ in results if s.startswith("apply") and e["mode"] == "create"]}, open(os.path.join(stamp, "p2-write.json") if os.path.isdir(stamp) else os.path.join(BACKUP, "p2-write-empty.json"), "w", encoding="utf-8"))
    print(f"written; backup {stamp}")
    return 0


def revert(arguments):
    root = os.path.abspath(arguments[arguments.index("--root") + 1]) if "--root" in arguments else REPO
    stamps = sorted(name for name in os.listdir(BACKUP) if os.path.exists(os.path.join(BACKUP, name, "p2-write.json")) and json.load(open(os.path.join(BACKUP, name, "p2-write.json")))["root"] == root)
    if not stamps:
        print("no backup for this root")
        return 1
    stamp = os.path.join(BACKUP, stamps[-1])
    record = json.load(open(os.path.join(stamp, "p2-write.json"), encoding="utf-8"))
    for relative in record["created"]:
        target = os.path.join(root, relative)
        if os.path.exists(target):
            os.remove(target)
    for folder, _, files in os.walk(stamp):
        for name in files:
            if name == "p2-write.json":
                continue
            source = os.path.join(folder, name)
            target = os.path.join(root, os.path.relpath(source, stamp))
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copy2(source, target)
    print(f"reverted from {stamp}")
    return 0


def main():
    command, rest = sys.argv[1], sys.argv[2:]
    if command == "add":
        current = listed()
        for path in rest:
            if path not in current:
                current.append(path)
                seed(path, not os.path.exists(os.path.join(STAGE, path)))
        open(LIST, "w", encoding="utf-8").write("\n".join(current) + "\n")
        print(f"{len(current)} path(s) listed")
    elif command == "rebase":
        moved, conflicts = 0, []
        for path in listed():
            base, stage, tree = read(os.path.join(BASE, path)), read(os.path.join(STAGE, path)), read(os.path.join(REPO, path))
            if tree == base:
                continue
            if stage == base:
                merged, clean = tree, True
            elif None in (base, stage, tree):
                merged, clean = None, False
            else:
                merged, clean = merge(tree, base, stage)
            if not clean:
                conflicts.append(path)
                continue
            target = os.path.join(STAGE, path)
            if merged is None:
                if os.path.exists(target):
                    os.remove(target)
            else:
                os.makedirs(os.path.dirname(target), exist_ok=True)
                open(target, "w", encoding="utf-8").write(merged)
            seed(path, False)
            moved += 1
        print(f"{moved} path(s) re-based onto the tree (stage merged three-way); conflicts: {conflicts or 'none'}")
    elif command == "capture":
        capture()
    elif command == "apply":
        return apply(rest)
    elif command == "revert":
        return revert(rest)
    elif command == "clear-created":
        root = os.path.abspath(rest[rest.index("--root") + 1])
        assert root != REPO, "clear-created only ever touches a scratch root"
        cleared = 0
        for entry in json.load(open(os.path.join(PAYLOAD, "manifest.json"), encoding="utf-8")):
            target = os.path.join(root, entry["path"])
            if entry["mode"] == "create" and os.path.exists(target):
                os.remove(target)
                cleared += 1
        print(f"cleared {cleared} created file(s) from {root}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
