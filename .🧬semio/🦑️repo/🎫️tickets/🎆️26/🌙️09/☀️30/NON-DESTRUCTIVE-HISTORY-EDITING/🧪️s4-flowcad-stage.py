"""🧺️ S4-FLOWCAD staging (fleet rule 45): edits to crates that may not be saved under the cargo freeze are prepared in a
mirror tree under `🗑️generated/s4-flowcad/stage-<wave>/tree/` and landed atomically at "CARGO OPEN".

Every staged path records the sha256 of the repository file it was based on (absent for a new file), so `apply` refuses
the whole wave when a peer changed any base meanwhile (rule 6: never clobber a peer) and writes nothing in that case.

Usage (cwd = repository root):
  python3 T/🧪️s4-flowcad-stage.py <wave> add <path>...     copy current files into the stage, record their bases
  python3 T/🧪️s4-flowcad-stage.py <wave> new <path>...     declare new files (must not exist at apply time)
  python3 T/🧪️s4-flowcad-stage.py <wave> delete <path>...  stage deletions (file or directory) with their bases
  python3 T/🧪️s4-flowcad-stage.py <wave> status            list staged paths and every drifted base
  python3 T/🧪️s4-flowcad-stage.py <wave> diff [path]       unified diff repository → staged
  python3 T/🧪️s4-flowcad-stage.py <wave> apply             land the wave (all or nothing)
"""

import difflib
import hashlib
import json
import os
import shutil
import sys

TICKET = os.path.dirname(os.path.abspath(__file__))


def digest(path):
    if os.path.isdir(path):
        entries = []
        for root, _, files in sorted(os.walk(path)):
            for name in sorted(files):
                full = os.path.join(root, name)
                entries.append(os.path.relpath(full, path) + ":" + digest(full))
        return "dir:" + hashlib.sha256("\n".join(entries).encode()).hexdigest()
    with open(path, "rb") as handle:
        return hashlib.sha256(handle.read()).hexdigest()


def load(stage):
    path = os.path.join(stage, "manifest.json")
    return json.load(open(path)) if os.path.exists(path) else {}


def save(stage, manifest):
    os.makedirs(stage, exist_ok=True)
    with open(os.path.join(stage, "manifest.json"), "w") as handle:
        json.dump(manifest, handle, indent=2, ensure_ascii=False, sort_keys=True)
        handle.write("\n")


def drifted(manifest):
    out = []
    for path, entry in sorted(manifest.items()):
        base = entry["base"]
        current = digest(path) if os.path.exists(path) else None
        if current != base:
            out.append((path, base, current))
    return out


def main():
    wave, command, *paths = sys.argv[1:]
    stage = os.path.join(TICKET, "🗑️generated", "s4-flowcad", f"stage-{wave}")
    tree = os.path.join(stage, "tree")
    manifest = load(stage)
    if command == "add":
        for path in paths:
            if path not in manifest:
                manifest[path] = {"action": "write", "base": digest(path)}
            os.makedirs(os.path.dirname(os.path.join(tree, path)), exist_ok=True)
            if not os.path.exists(os.path.join(tree, path)):
                shutil.copyfile(path, os.path.join(tree, path))
    elif command == "new":
        for path in paths:
            assert not os.path.exists(path), f"{path} exists; use add"
            manifest[path] = {"action": "write", "base": None}
            os.makedirs(os.path.dirname(os.path.join(tree, path)), exist_ok=True)
    elif command == "delete":
        for path in paths:
            manifest[path] = {"action": "delete", "base": digest(path)}
    elif command == "status":
        for path, entry in sorted(manifest.items()):
            print(f"{entry['action']:6} {'new' if entry['base'] is None else 'base'} {path}")
        for path, base, current in drifted(manifest):
            print(f"DRIFT {path}: base {base} now {current}")
    elif command == "diff":
        for path, entry in sorted(manifest.items()):
            if paths and path not in paths:
                continue
            if entry["action"] == "delete":
                print(f"--- delete {path}")
                continue
            before = open(path).read().splitlines(True) if os.path.isfile(path) else []
            after = open(os.path.join(tree, path)).read().splitlines(True)
            sys.stdout.writelines(difflib.unified_diff(before, after, path, f"staged/{path}"))
    elif command == "apply":
        conflicts = drifted(manifest)
        if conflicts:
            for path, base, current in conflicts:
                print(f"CONFLICT {path}: base {base} now {current}")
            sys.exit(1)
        missing = [path for path, entry in manifest.items() if entry["action"] == "write" and not os.path.isfile(os.path.join(tree, path))]
        if missing:
            print("staged content missing:", *missing, sep="\n  ")
            sys.exit(1)
        for path, entry in sorted(manifest.items()):
            if entry["action"] == "delete":
                shutil.rmtree(path) if os.path.isdir(path) else os.remove(path)
            else:
                os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
                shutil.copyfile(os.path.join(tree, path), path)
        print(f"applied {len(manifest)} path(s) of stage-{wave}")
        return
    else:
        raise SystemExit(__doc__)
    save(stage, manifest)


if __name__ == "__main__":
    main()
