#!/usr/bin/env python3
"""🩹 Line-level HEAD-oracle revert of the 2026-09-03 rename-plan codemod damage inside tracked text
files. For every modified tracked file, each changed line is restored to its HEAD twin only when the two
become identical after stripping every emoji token and every `.` — i.e. when the only difference is
emoji added/duplicated/substituted or dots deleted by the codemod. Legitimate edits never satisfy that
and are left untouched. Whole-file reverts happen only when every changed line qualifies.

Usage: 🔨️revert-codemod-lines.py [--apply] [--under PREFIX]   (dry-run without --apply)
Related: 🔨️restore-renamed-paths.py (paths on disk), 🔨️repair-emoji-path-corruption.py, 📓️status.md.
"""
import difflib
import os
import re
import subprocess
import sys

EMOJI_TOKEN = re.compile(r"[^\x00-\x7F]️?")
REPO = subprocess.run(["git", "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True).stdout.strip()
SKIP_PREFIXES = (".🧬semio/", "target", "node_modules/")


def normalize(line: str) -> str:
    return EMOJI_TOKEN.sub("", line).replace(".", "")


QUOTED_PATH = re.compile(r'["\']((?:\.{1,2}/|/)?[^"\'\n]*[^\x00-\x7F][^"\'\n]*)["\']')


def path_literals(line: str) -> list[str]:
    return [m for m in QUOTED_PATH.findall(line) if "/" in m or m.startswith(".")]


def head_resolves_current_does_not(head_line: str, cur_line: str, base_dir: str) -> bool:
    head_paths, cur_paths = path_literals(head_line), path_literals(cur_line)
    if not head_paths or not cur_paths or len(head_paths) != len(cur_paths):
        return False
    exists = lambda literal: os.path.exists(os.path.join(base_dir, literal)) or os.path.exists(os.path.join(REPO, literal))
    if head_paths == cur_paths:
        return False
    return all(h == c or (exists(h) and not exists(c)) for h, c in zip(head_paths, cur_paths))


def modified_files(under: str) -> list[str]:
    out = subprocess.run(["git", "status", "--porcelain=v1", "-z"], capture_output=True, text=True, cwd=REPO, check=True).stdout
    files = []
    for entry in out.split("\0"):
        if len(entry) < 4:
            continue
        code, path = entry[:2], entry[3:]
        if "M" not in code or path.startswith(SKIP_PREFIXES) or (under and not path.startswith(under)):
            continue
        files.append(path)
    return files


def head_text(path: str):
    shown = subprocess.run(["git", "show", f"HEAD:{path}"], capture_output=True, cwd=REPO)
    if shown.returncode != 0 or b"\0" in shown.stdout[:8192]:
        return None
    try:
        return shown.stdout.decode("utf-8")
    except UnicodeDecodeError:
        return None


def revert_file(path: str, apply: bool) -> tuple[int, int]:
    head = head_text(path)
    absolute = os.path.join(REPO, path)
    if head is None or not os.path.isfile(absolute):
        return 0, 0
    try:
        current = open(absolute, encoding="utf-8").read()
    except UnicodeDecodeError:
        return 0, 0
    head_lines, cur_lines = head.splitlines(keepends=True), current.splitlines(keepends=True)
    result: list[str] = []
    reverted = kept = 0
    for op, h0, h1, c0, c1 in difflib.SequenceMatcher(None, head_lines, cur_lines, autojunk=False).get_opcodes():
        if op == "equal":
            result.extend(cur_lines[c0:c1])
        elif op == "replace" and (h1 - h0) == (c1 - c0) and all(normalize(a) == normalize(b) or head_resolves_current_does_not(a, b, os.path.dirname(absolute)) for a, b in zip(head_lines[h0:h1], cur_lines[c0:c1])):
            result.extend(head_lines[h0:h1])
            reverted += c1 - c0
        else:
            result.extend(cur_lines[c0:c1])
            kept += max(c1 - c0, h1 - h0)
    if reverted and apply:
        open(absolute, "w", encoding="utf-8").write("".join(result))
    return reverted, kept


def main(argv: list[str]) -> int:
    apply = "--apply" in argv
    under = argv[argv.index("--under") + 1] if "--under" in argv else ""
    total_files = total_reverted = 0
    for path in modified_files(under):
        reverted, kept = revert_file(path, apply)
        if reverted:
            total_files += 1
            total_reverted += reverted
            print(f"  {path}: reverted {reverted} codemod line(s), kept {kept} other changed line(s)")
    print(f"{'applied' if apply else 'dry-run'}: files={total_files} lines_reverted={total_reverted}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
