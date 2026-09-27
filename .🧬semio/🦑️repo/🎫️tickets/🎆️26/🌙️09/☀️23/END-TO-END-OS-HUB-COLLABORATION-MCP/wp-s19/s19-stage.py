#!/usr/bin/env python3
"""🧷️ S19 prepared-patch stage: whole-file payload pairs captured from the scratch overlay, applied to the repo tree in
window 3 with a three-way merge over peers' later edits.

  s19-stage.py track <set> <relpath>…   record the overlay's CURRENT content as the base (`.old`) before editing it there
  s19-stage.py capture [<set>]           record the overlay's edited content (`.new`; a vanished file becomes a delete)
  s19-stage.py apply [<set>] [--write] [--root <dir>]
                                         dry run by default; a target equal to `.old` is replaced, equal to `.new` counts as
                                         applied, anything else is merged three-way (`git merge-file -p`, read-only) and
                                         applied only when conflict-free; tree writes keep a backup under
                                         `.🧬semio/🌐hub/s14-s19-backup/<stamp>/`.
  s19-stage.py status [<set>]            lists every entry with its overlay/tree state.
"""
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time

REPO = "/Users/ueli/Documents/semio"
OVERLAY = os.path.join(REPO, ".🧬semio/🌐hub/s14-s19-overlay")
HERE = os.path.dirname(os.path.abspath(__file__))
PAYLOAD = os.path.join(HERE, "payload")
MANIFEST = os.path.join(PAYLOAD, "manifest.json")
BACKUP = os.path.join(REPO, ".🧬semio/🌐hub/s14-s19-backup")


def read(path):
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def load():
    return json.loads(read(MANIFEST) or "[]")


def save(manifest):
    write(MANIFEST, json.dumps(manifest, ensure_ascii=False, indent=1) + "\n")


def merge(current, old, new):
    with tempfile.TemporaryDirectory() as scratch:
        names = []
        for label, text in (("current", current), ("old", old), ("new", new)):
            path = os.path.join(scratch, label)
            write(path, text)
            names.append(path)
        result = subprocess.run(["git", "merge-file", "-p", *names], capture_output=True, text=True)
    return result.stdout, result.returncode == 0


def track(set_name, rels):
    manifest = load()
    known = {entry["path"] for entry in manifest}
    for rel in rels:
        rel = os.path.relpath(os.path.join(OVERLAY, rel), OVERLAY) if not os.path.isabs(rel) else os.path.relpath(rel, OVERLAY)
        if rel in known:
            continue
        entry_id = f"{len(manifest):04d}"
        base = read(os.path.join(OVERLAY, rel))
        if base is not None:
            write(os.path.join(PAYLOAD, f"{entry_id}.old"), base)
        manifest.append({"id": entry_id, "set": set_name, "path": rel, "mode": "modify" if base is not None else "create"})
        known.add(rel)
        print(f"tracked {entry_id} {set_name} {rel} ({'modify' if base is not None else 'create'})")
    save(manifest)


def capture(set_name):
    manifest = load()
    for entry in manifest:
        if set_name and entry["set"] != set_name:
            continue
        current = read(os.path.join(OVERLAY, entry["path"]))
        new_path = os.path.join(PAYLOAD, f"{entry['id']}.new")
        if current is None:
            entry["mode"] = "delete" if entry["mode"] != "create" else "create"
            if os.path.exists(new_path):
                os.unlink(new_path)
        else:
            write(new_path, current)
            if entry["mode"] == "delete":
                entry["mode"] = "modify"
    save(manifest)
    print(f"captured {sum(1 for e in manifest if not set_name or e['set'] == set_name)} entries")


def plan(root, entry):
    target = os.path.join(root, entry["path"])
    old = read(os.path.join(PAYLOAD, f"{entry['id']}.old"))
    new = read(os.path.join(PAYLOAD, f"{entry['id']}.new"))
    current = read(target)
    if entry["mode"] == "create":
        if new is None:
            return "skip: nothing captured", target, None
        if current is None:
            return "apply", target, new
        return ("applied", target, None) if current == new else ("conflict: file exists with other content", target, None)
    if entry["mode"] == "delete":
        if current is None:
            return "applied", target, None
        return ("apply", target, None) if current == old else ("conflict: file differs from the prepared base", target, None)
    if new is None:
        return "skip: nothing captured", target, None
    if current is None:
        return "conflict: file missing", target, None
    if current == new:
        return "applied", target, None
    if current == old:
        return "apply", target, new
    merged, clean = merge(current, old, new)
    return ("apply (3-way merge over a peer's change)", target, merged) if clean else ("conflict: 3-way merge has conflicts", target, None)


def apply(set_name, do_write, root):
    manifest = [entry for entry in load() if not set_name or entry["set"] == set_name]
    results = [(entry, *plan(root, entry)) for entry in manifest]
    for entry, state, _, _ in results:
        print(f"{state:>44}  {entry['id']} {entry['set']} {entry['path']}")
    conflicts = [r for r in results if r[1].startswith("conflict")]
    pending = [r for r in results if r[1].startswith("apply")]
    print(f"{len(results)} entries: {len(pending)} to apply, {len(results) - len(pending) - len(conflicts)} applied/skipped, {len(conflicts)} conflicts")
    if conflicts:
        return 1
    if not do_write:
        print("dry run: nothing written")
        return 0
    stamp = time.strftime("%Y%m%d-%H%M%S")
    for entry, state, target, text in pending:
        if os.path.exists(target) and root == REPO:
            backup = os.path.join(BACKUP, stamp, entry["path"])
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            shutil.copy2(target, backup)
        if entry["mode"] == "delete":
            os.unlink(target)
        else:
            write(target, text)
        print(f"wrote {entry['path']}")
    return 0


def status(set_name):
    for entry in load():
        if set_name and entry["set"] != set_name:
            continue
        overlay = read(os.path.join(OVERLAY, entry["path"]))
        tree = read(os.path.join(REPO, entry["path"]))
        old = read(os.path.join(PAYLOAD, f"{entry['id']}.old"))
        print(f"{entry['id']} {entry['set']:<18} {entry['mode']:<7} overlay={'edited' if overlay != old else 'base'} tree={'base' if tree == old else ('absent' if tree is None else 'moved')} {entry['path']}")


def main():
    command, rest = sys.argv[1], sys.argv[2:]
    if command == "track":
        track(rest[0], rest[1:])
    elif command == "capture":
        capture(rest[0] if rest else "")
    elif command == "apply":
        root = REPO
        if "--root" in rest:
            root = os.path.abspath(rest[rest.index("--root") + 1])
        positional = [arg for index, arg in enumerate(rest) if not arg.startswith("--") and (index == 0 or rest[index - 1] != "--root")]
        sys.exit(apply(positional[0] if positional else "", "--write" in rest, root))
    elif command == "status":
        status(rest[0] if rest else "")
    else:
        sys.exit(f"unknown command {command}")


if __name__ == "__main__":
    main()
