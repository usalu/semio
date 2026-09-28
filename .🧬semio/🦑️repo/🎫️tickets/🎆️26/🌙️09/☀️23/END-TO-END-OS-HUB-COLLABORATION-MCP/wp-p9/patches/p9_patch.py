"""🧩️ Shared engine of P9 prepared patch sets (session 14): every hunk is anchored and must match exactly the stated number of
times; `--dry-run` proves every anchor and prints the file list, `--write` applies all or nothing. New files must not
exist yet (or equal their target when re-run)."""
import os
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(os.environ.get("P9_ROOT", "/Users/ueli/Documents/semio"))
edits: dict = {}
originals: dict = {}
problems: list = []
counts: dict = {}


def text(path: Path) -> str:
    if path not in edits:
        originals[path] = path.read_text(encoding="utf-8")
        edits[path] = originals[path]
    return edits[path]


def replace(part: str, path: Path, old: str, new: str, count: int = 1) -> None:
    source = text(path)
    found = source.count(old)
    if found != count:
        problems.append(f"{part}: {path.relative_to(ROOT)}: expected {count}x {old[:100]!r}, found {found}")
        return
    edits[path] = source.replace(old, new)
    counts[part] = counts.get(part, 0) + count


def regex(part: str, path: Path, pattern: str, new, count: int, flags: int = 0) -> None:
    source = text(path)
    updated, found = re.subn(pattern, new, source, flags=flags)
    if found != count:
        problems.append(f"{part}: {path.relative_to(ROOT)}: expected {count}x /{pattern[:90]}/, found {found}")
        return
    edits[path] = updated
    counts[part] = counts.get(part, 0) + count


def create(part: str, path: Path, content: str) -> None:
    if path.exists() and path.read_text(encoding="utf-8") != content:
        problems.append(f"{part}: {path.relative_to(ROOT)} already exists with other content")
        return
    originals.setdefault(path, None)
    edits[path] = content
    counts[part] = counts.get(part, 0) + 1


removals: list = []


def delete_tree(part: str, path: Path, expected_files: int) -> None:
    """Removes a whole directory (tracked and ignored files alike); a re-run after `--write` finds it gone and is a no-op."""
    if not path.exists():
        return
    found = sum(1 for item in path.rglob("*") if item.is_file() and not {"pkg", "node_modules"} & set(item.relative_to(path).parts))
    if found != expected_files:
        problems.append(f"{part}: {path.relative_to(ROOT)}: expected {expected_files} source file(s) outside pkg/ and node_modules/, found {found}")
        return
    removals.append(path)
    counts[part] = counts.get(part, 0) + 1


def finish(doc: str) -> None:
    write = "--write" in sys.argv
    if not write and "--dry-run" not in sys.argv:
        sys.exit(doc)
    if problems:
        print("\n".join(problems))
        sys.exit(f"{len(problems)} problem(s); nothing written")
    for path in removals:
        print(f"   {'removed' if write else 'would remove'} {path.relative_to(ROOT)}/ ({sum(1 for item in path.rglob('*') if item.is_file())} file(s))")
        if write:
            shutil.rmtree(path)
    changed = [path for path, content in edits.items() if originals.get(path) != content]
    for part, n in sorted(counts.items()):
        print(f"part {part}: {n} hunk(s)")
    print(f"{len(changed)} file(s) {'written' if write else 'would change'}:")
    for path in changed:
        print("  ", path.relative_to(ROOT))
        if write:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(edits[path], encoding="utf-8")
