#!/usr/bin/env python3
"""🩹 Restores files and directories the 2026-09-03 emoji codemod renamed on disk: every tracked path
git reports as deleted is matched — by ASCII skeleton (all emoji stripped) — to an untracked path, and the
top-most differing directory (or the file itself) is moved back with `mv`, so working-tree edits inside
the moved trees survive. Ambiguous or unmatched deletions are reported, never guessed.

Usage: 🔨️restore-renamed-paths.py [--apply] [--doubles-only] [--under PREFIX]   (dry-run without --apply)
Related: 🔨️repair-emoji-path-corruption.py (run AFTER this one), ticket 📓️status.md.
"""
import os
import re
import shutil
import subprocess
import sys
from collections import defaultdict

EMOJI_TOKEN = re.compile(r"[^\x00-\x7F]️?")
DOUBLED = re.compile(r"([^\x00-\x7F]️?)\1+")
REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()


def skeleton(path: str) -> str:
    return EMOJI_TOKEN.sub("", path)


def git_status() -> tuple[list[str], list[str]]:
    out = subprocess.run(["git", "status", "--porcelain=v1", "-z", "--untracked-files=all"], capture_output=True, text=True, cwd=REPO, check=True).stdout
    deleted, untracked = [], []
    for entry in out.split("\0"):
        if len(entry) < 4:
            continue
        code, path = entry[:2], entry[3:]
        if "D" in code:
            deleted.append(path)
        elif code == "??":
            untracked.append(path)
    return deleted, untracked


def first_divergence(deleted: str, untracked: str) -> "tuple[str, str] | None":
    a, b = deleted.split("/"), untracked.split("/")
    if len(a) != len(b):
        return None
    for index, (x, y) in enumerate(zip(a, b)):
        if x != y:
            return "/".join(b[: index + 1]), "/".join(a[: index + 1])
    return None


def main(argv: list[str]) -> int:
    apply = "--apply" in argv
    under = argv[argv.index("--under") + 1] if "--under" in argv else ""
    doubles_only = "--doubles-only" in argv
    deleted, untracked = git_status()
    by_skeleton: dict[str, list[str]] = defaultdict(list)
    for path in untracked:
        by_skeleton[skeleton(path)].append(path)
    moves: dict[str, str] = {}
    unmatched, ambiguous = [], []
    for path in deleted:
        if under and not path.startswith(under):
            continue
        if path.startswith(".🧬semio/"):
            continue
        candidates = [c for c in by_skeleton.get(skeleton(path), []) if c != path]
        if len(candidates) > 1:
            divergences = [(c, first_divergence(path, c)) for c in candidates]
            preferred = [c for c, d in divergences if d and (EMOJI_TOKEN.findall(d[1].split("/")[-1]) or [None])[0] == (EMOJI_TOKEN.findall(d[0].split("/")[-1]) or [""])[0]]
            candidates = preferred if len(preferred) == 1 else candidates
        if not candidates:
            unmatched.append(path)
            continue
        if len(candidates) > 1:
            ambiguous.append((path, candidates))
            continue
        divergence = first_divergence(path, candidates[0])
        if divergence is None:
            unmatched.append(path)
            continue
        source, target = divergence
        if doubles_only and DOUBLED.sub(r"\1", source.split("/")[-1]) != target.split("/")[-1]:
            continue
        if source in moves and moves[source] != target:
            ambiguous.append((path, [moves[source], target]))
            continue
        moves[source] = target
    print(f"deleted={len(deleted)} untracked={len(untracked)} planned_moves={len(moves)} unmatched={len(unmatched)} ambiguous={len(ambiguous)}")
    for source, target in sorted(moves.items(), key=lambda item: item[0].count("/")):
        print(f"  mv {source}\n  -> {target}")
    for path in unmatched[:40]:
        print(f"  UNMATCHED {path}")
    for path, candidates in ambiguous[:20]:
        print(f"  AMBIGUOUS {path} <- {candidates}")
    if not apply:
        return 0
    done = 0
    for source, target in sorted(moves.items(), key=lambda item: item[0].count("/")):
        src, dst = os.path.join(REPO, source), os.path.join(REPO, target)
        if not os.path.exists(src):
            continue
        if os.path.exists(dst):
            print(f"  SKIP target exists: {target}")
            continue
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.move(src, dst)
        done += 1
    print(f"moved={done}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
