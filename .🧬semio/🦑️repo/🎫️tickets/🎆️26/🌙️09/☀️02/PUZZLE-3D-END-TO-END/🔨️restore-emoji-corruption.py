#!/usr/bin/env python3
"""🔨️ Restores the repository after the non-idempotent emoji rename pass.

A `bun -e` applier for ticket 26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY re-applies its
rename plan on every invocation, so each run re-prefixes names that still match. Two visible shapes:
same-token repeats (`📦️packages` -> `📦️📦️packages`) and a different emoji glued on
(`🔺️mesh-engine` -> `🔺️⚙️mesh-engine`). The rename plan is itself compounded, so HEAD is the ONLY
known-good tree — not the plan, and not the on-disk state.

Order matters and is not negotiable: directories first, then text. Repairing text while corrupted
directories exist bakes the corrupted names in, because an "does the target exist on disk" guard
happily accepts them.

Oracle is git, never inference: `git ls-files` contains ZERO doubled-emoji names, so any on-disk name
carrying one is corruption, and the tracked path with the same ASCII skeleton is its true name.

Dry-run by default. `--apply` performs it. Never runs a modifying git command; only `os.rename`.
"""
import os
import re
import subprocess
import sys

TOKEN = re.compile(r"[^\x00-\x7F]️?")
REPEAT = re.compile(r"((?:[^\x00-\x7F]️?))\1+")
SKIP = ("./target/", "./.git/", "./node_modules/", "./.🧬semio/")


def skeleton(name):
    """@emoji 🦴️ ASCII skeleton — the part the rename pass never altered."""
    return TOKEN.sub("", name)


def tracked_paths():
    out = subprocess.run(["git", "ls-files"], capture_output=True, text=True).stdout.splitlines()
    dirs = set()
    for f in out:
        parts = f.split("/")
        for i in range(1, len(parts)):
            dirs.add("/".join(parts[:i]))
    return set(out), dirs


def corrupted_dirs(tracked_dirs):
    """📁️ On-disk directories whose name is absent from git but whose skeleton matches a tracked one."""
    by_skeleton = {}
    for d in tracked_dirs:
        by_skeleton.setdefault((os.path.dirname(d), skeleton(os.path.basename(d))), []).append(d)
    found = []
    for root, dirs, _ in os.walk("."):
        if any(root.startswith(s) for s in SKIP):
            dirs[:] = []
            continue
        for name in dirs:
            path = os.path.normpath(os.path.join(root, name))
            if path in tracked_dirs:
                continue
            hits = by_skeleton.get((os.path.dirname(path), skeleton(name)), [])
            exact = [h for h in hits if h != path]
            if len(exact) == 1:
                found.append((path, exact[0]))
    return found


def main():
    apply = "--apply" in sys.argv
    _, tracked_dirs = tracked_paths()
    moves = corrupted_dirs(tracked_dirs)
    moves.sort(key=lambda m: m[0].count("/"), reverse=True)
    print(f"{'APPLYING' if apply else 'DRY RUN'} — {len(moves)} directory restore(s)")
    for src, dst in moves[:20]:
        print(f"  {src}\n    -> {dst}")
    if len(moves) > 20:
        print(f"  … and {len(moves) - 20} more")
    if not apply:
        print("\nre-run with --apply to perform. Text repair is a SEPARATE second step,")
        print("and must not run until this reports 0 remaining.")
        return
    done = 0
    for src, dst in moves:
        if os.path.isdir(src) and not os.path.exists(dst):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            os.rename(src, dst)
            done += 1
    print(f"restored {done} directories")


if __name__ == "__main__":
    main()
