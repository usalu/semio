#!/usr/bin/env python3
"""🧰️ S19 set runner shared by the session-15 codemods: every set is a list of (relative path, transform) where a
transform maps the LIVE file text to the edited text (asserting its anchors) and is idempotent (an already-edited
file maps to itself). `--dry-run` (default) reports per file, `--write` backs the originals up under
`.🧬semio/🌐hub/s14-s19-backup/<set>/` and writes, `--revert` restores the backups (a file the set created is removed).
usage (from a set script): run(set_name, edits, argv)"""
import os
import shutil
import sys

ROOT = "/Users/ueli/Documents/semio"
BACKUP = f"{ROOT}/.🧬semio/🌐hub/s14-s19-backup"
CREATED = "\u0000created\u0000"


def run(name, edits, argv):
    mode = next((arg for arg in argv if arg in ("--dry-run", "--write", "--revert")), "--dry-run")
    root = next((arg[len("--root="):] for arg in argv if arg.startswith("--root=")), ROOT)
    backup_root = os.path.join(BACKUP, name)
    faults = 0
    changed = 0
    for rel, transform in edits:
        path = os.path.join(root, rel)
        backup = os.path.join(backup_root, rel)
        if mode == "--revert":
            if os.path.exists(backup):
                if open(backup, encoding="utf-8").read() == CREATED:
                    os.path.exists(path) and os.remove(path)
                else:
                    shutil.copy2(backup, path)
                print(f"restored {rel}")
            continue
        before = open(path, encoding="utf-8").read() if os.path.exists(path) else None
        try:
            after = transform(before)
        except AssertionError as error:
            faults += 1
            print(f"CONFLICT {rel}: {error}")
            continue
        if after == before:
            print(f"already {rel}")
            continue
        changed += 1
        print(f"{'edit' if before is not None else 'create'} {rel} ({0 if before is None else len(before)} → {len(after)} bytes)")
        if mode == "--write":
            os.makedirs(os.path.dirname(backup), exist_ok=True)
            if not os.path.exists(backup):
                if before is None:
                    open(backup, "w", encoding="utf-8").write(CREATED)
                else:
                    shutil.copy2(path, backup)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(after)
    if mode != "--revert":
        print(f"{name}: {len(edits)} files, {changed} to change, {faults} conflicts ({mode})")
    sys.exit(1 if faults else 0)


def replace_once(old, new):
    """🔁️ A transform replacing one exact anchor (already-applied when `new` is present and `old` is gone)."""
    def transform(text):
        if new in text:
            return text
        assert text.count(old) == 1, f"anchor x{text.count(old)}: {old[:80]!r}"
        return text.replace(old, new)
    return transform


def chain(*transforms):
    """⛓️ Applies several transforms to one file in order."""
    def transform(text):
        for step in transforms:
            text = step(text)
        return text
    return transform
