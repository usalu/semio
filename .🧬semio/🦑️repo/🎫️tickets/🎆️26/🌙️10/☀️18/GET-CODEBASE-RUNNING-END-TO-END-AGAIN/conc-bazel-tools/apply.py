#!/usr/bin/env python3
"""Anchored, all-or-nothing apply of the conc-bazel set in two groups.

  group 1  slots: bounded build slots, per-slot output bases and packages, shared caches, hierarchical restore locks, GC
  group 2  service slots: sessions that launch long-running watch/serve/dev tasks hold a service slot of their own class
           (own queue, own bound, own output bases) instead of pinning a build slot; builds hold a slot only per cycle

  python3 apply.py --root <checkout or overlay> [--group 1|2|all]       dry run (default group: all)
  python3 apply.py --root <root> --group all --write [--backup DIR]     writes; originals of edited files go to DIR
  python3 apply.py --root <root> --undo DIR                             restores the originals from DIR and removes the new files
  python3 apply.py --root <root> --record                               writes bases.json (sha256 of every edited target as staged)

Group 2 builds on the result of group 1 (applied before it, or in the same run). New files are never overwritten with
other content: a group-2 file may replace only the group-1 version of itself. A moved anchor is refused, never guessed.
"""
import argparse, hashlib, json, os, shutil, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from targets import NEW, NEW2
from edits import EDITS, EDITS2


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def plan(root: str, groups):
    state, rows, problems = {}, [], []

    def read(path):
        if path in state:
            return state[path]
        return open(path, "rb").read() if os.path.exists(path) else None

    def stage_files(label, mapping, source_dir, previous_dir):
        for src, target in mapping.items():
            data = open(os.path.join(HERE, "staged", source_dir, src) if source_dir else os.path.join(HERE, "staged", src), "rb").read()
            old = open(os.path.join(HERE, "staged", previous_dir, src) if previous_dir else os.path.join(HERE, "staged", src), "rb").read() if previous_dir is not None else None
            path = os.path.join(root, target)
            current = read(path)
            if current == data:
                rows.append(("same", f"{label} {target}"))
            elif current is None:
                if previous_dir is not None:
                    problems.append(f"missing: {target} (group 2 needs group 1)")
                else:
                    rows.append(("new", f"{label} {target}"))
                    state[path] = data
            elif previous_dir is not None and current == old:
                rows.append(("upgrade", f"{label} {target}"))
                state[path] = data
            else:
                problems.append(f"conflict: {target} exists with other content")

    def stage_edits(label, table):
        for target, edits in table.items():
            path = os.path.join(root, target)
            current = read(path)
            if current is None:
                problems.append(f"missing: {target}")
                continue
            text = current.decode("utf-8")
            for index, edit in enumerate(edits):
                old, new = edit[0], edit[1]
                expected = edit[2] if len(edit) > 2 else 1
                n_old, n_new = text.count(old), text.count(new)
                if old != new and n_new > 0 and (n_old == 0 or old in new):
                    rows.append(("applied", f"{label} {target} #{index}"))
                elif (expected is None and n_old > 0) or n_old == expected:
                    text = text.replace(old, new)
                    rows.append(("pending", f"{label} {target} #{index}"))
                else:
                    problems.append(f"anchor: {label} {target} #{index} found {n_old} times, expected {expected}: {old[:80]!r}")
            state[path] = text.encode("utf-8")

    superseded = all(read(os.path.join(root, target)) == open(os.path.join(HERE, "staged", "g2", src), "rb").read() for src, target in NEW2.items())
    if 1 in groups and superseded:
        rows.append(("superseded", "g1 (group 2 is applied)"))
    elif 1 in groups:
        stage_files("g1", NEW, None, None)
        stage_edits("g1", EDITS)
    if 2 in groups:
        stage_files("g2", NEW2, "g2", "")
        stage_edits("g2", EDITS2)
    writes = {}
    for path, data in state.items():
        disk = open(path, "rb").read() if os.path.exists(path) else None
        if disk != data:
            writes[path] = data
    return rows, writes, problems


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", required=True)
    ap.add_argument("--group", default="all", choices=["1", "2", "all"])
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--backup")
    ap.add_argument("--undo")
    ap.add_argument("--record", action="store_true")
    args = ap.parse_args()
    root = os.path.realpath(args.root)
    if args.undo:
        manifest = json.load(open(os.path.join(args.undo, "manifest.json")))
        for rel in manifest["new"]:
            if os.path.exists(os.path.join(root, rel)):
                os.remove(os.path.join(root, rel))
        for rel in manifest["edited"]:
            shutil.copyfile(os.path.join(args.undo, "files", sha(rel.encode())), os.path.join(root, rel))
        print("restored", len(manifest["edited"]), "edited files, removed", len(manifest["new"]), "new files")
        return 0
    if args.record:
        bases = {t: sha(open(os.path.join(root, t), "rb").read()) for t in EDITS}
        json.dump(bases, open(os.path.join(HERE, "bases.json"), "w"), ensure_ascii=False, indent=1)
        print("recorded", len(bases), "base hashes")
        return 0
    groups = {"1": [1], "2": [2], "all": [1, 2]}[args.group]
    rows, writes, problems = plan(root, groups)
    bases_path = os.path.join(HERE, "bases.json")
    if os.path.exists(bases_path):
        for target, expected in json.load(open(bases_path)).items():
            path = os.path.join(root, target)
            if os.path.exists(path) and sha(open(path, "rb").read()) != expected:
                print("note: changed since staging (anchors decide):", target)
    counts = {}
    for kind, name in rows:
        counts[kind] = counts.get(kind, 0) + 1
        if kind in ("pending", "new", "upgrade") or "-v" in sys.argv:
            print(f"{kind:8} {name}")
    for problem in problems:
        print("REFUSED", problem)
    print("summary", json.dumps(counts), "problems", len(problems))
    if problems:
        return 1
    if not args.write:
        print("dry run ok:", len(writes), "file(s) would be written")
        return 0
    backup = args.backup or os.path.join(HERE, "backup-" + time.strftime("%Y%m%d-%H%M%S"))
    os.makedirs(os.path.join(backup, "files"), exist_ok=True)
    manifest = {"new": [], "edited": []}
    for path in writes:
        rel = os.path.relpath(path, root)
        if os.path.exists(path):
            shutil.copyfile(path, os.path.join(backup, "files", sha(rel.encode())))
            manifest["edited"].append(rel)
        else:
            manifest["new"].append(rel)
    json.dump(manifest, open(os.path.join(backup, "manifest.json"), "w"), ensure_ascii=False, indent=1)
    for path, data in writes.items():
        os.makedirs(os.path.dirname(path), exist_ok=True)
        temporary = path + ".conc-bazel.tmp"
        open(temporary, "wb").write(data)
        os.replace(temporary, path)
    print("written", len(writes), "file(s); backup", backup)
    return 0


sys.exit(main())
