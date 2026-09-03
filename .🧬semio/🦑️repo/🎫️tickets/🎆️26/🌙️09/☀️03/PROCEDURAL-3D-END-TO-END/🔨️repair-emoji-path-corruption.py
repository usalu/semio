#!/usr/bin/env python3
"""🩹 Repairs the 2026-09-03 emoji path corruption (doubled, glued or substituted emoji segments) in
path string literals — `#[path = "…"]` / `path = "…"` in Rust and Cargo files, `from "…"` /
`import("…")` / quoted paths in TypeScript — using two oracles that never guess: the same file at git
HEAD (literals matched by their ASCII skeleton) and, for literals new since HEAD, collapse/drop of emoji
tokens. A literal is rewritten only when the repaired path exists on disk relative to the file.

Usage: 🔨️repair-emoji-path-corruption.py [--apply] FILE…   (dry-run without --apply)
Related: semio-2f's root Cargo.toml repair (same existence rule), ticket 📓️status.md.
"""
import os
import re
import subprocess
import sys

EMOJI_TOKEN = re.compile(r"[^\x00-\x7F]️?")
DOUBLED = re.compile(r"([^\x00-\x7F]️?)\1+")
QUOTED = re.compile(r'(["\'])([^"\'\n]*[^\x00-\x7F][^"\'\n]*)\1')
REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()


def skeleton(literal: str) -> str:
    return EMOJI_TOKEN.sub("", literal)


def head_literals(path: str) -> dict[str, str]:
    relative = os.path.relpath(os.path.abspath(path), REPO)
    shown = subprocess.run(["git", "show", f"HEAD:{relative}"], capture_output=True, text=True, cwd=REPO)
    if shown.returncode != 0:
        return {}
    oracle: dict[str, str] = {}
    for match in QUOTED.finditer(shown.stdout):
        oracle.setdefault(skeleton(match.group(2)), match.group(2))
    return oracle


def candidates(segment: str):
    yield DOUBLED.sub(r"\1", segment)
    tokens = EMOJI_TOKEN.findall(segment)
    rest = EMOJI_TOKEN.sub("", segment)
    for keep in range(len(tokens) - 1, 0, -1):
        for start in range(0, len(tokens) - keep + 1):
            yield "".join(tokens[start : start + keep]) + rest


def repair_by_collapse(base_dir: str, literal: str):
    fixed: list[str] = []
    for part in literal.split("/"):
        if not EMOJI_TOKEN.search(part) or part in ("..", "."):
            fixed.append(part)
            continue
        prefix = os.path.join(base_dir, *fixed) if fixed else base_dir
        chosen = next((cand for cand in candidates(part) if os.path.exists(os.path.join(prefix, cand))), None)
        if chosen is None:
            return None
        fixed.append(chosen)
    return "/".join(fixed)


def is_path_like(literal: str) -> bool:
    return "/" in literal or literal.startswith(".")


def repair_file(path: str, apply: bool) -> tuple[int, int]:
    base_dir = os.path.dirname(os.path.abspath(path))
    oracle = head_literals(path)
    text = open(path, encoding="utf-8").read()
    changed = unresolved = 0

    def resolve(literal: str):
        for root in (base_dir, REPO):
            if os.path.exists(os.path.join(root, literal)):
                return literal
        from_head = oracle.get(skeleton(literal))
        for root in (base_dir, REPO):
            if from_head and os.path.exists(os.path.join(root, from_head)):
                return from_head
        return repair_by_collapse(base_dir, literal) or repair_by_collapse(REPO, literal)

    def sub(match: re.Match) -> str:
        nonlocal changed, unresolved
        original = match.group(2)
        if not is_path_like(original):
            return match.group(0)
        repaired = resolve(original)
        if repaired is None:
            unresolved += 1
            print(f"  UNRESOLVED {original}")
            return match.group(0)
        if repaired == original:
            return match.group(0)
        changed += 1
        print(f"  {original}\n    -> {repaired}")
        return f"{match.group(1)}{repaired}{match.group(1)}"

    new_text = QUOTED.sub(sub, text)
    if apply and new_text != text:
        open(path, "w", encoding="utf-8").write(new_text)
    return changed, unresolved


def main(argv: list[str]) -> int:
    apply = "--apply" in argv
    total_changed = total_unresolved = 0
    for file in (a for a in argv if a != "--apply"):
        print(f"== {file} ({'apply' if apply else 'dry-run'})")
        changed, unresolved = repair_file(file, apply)
        total_changed += changed
        total_unresolved += unresolved
    print(f"changed={total_changed} unresolved={total_unresolved}")
    return 1 if total_unresolved else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
