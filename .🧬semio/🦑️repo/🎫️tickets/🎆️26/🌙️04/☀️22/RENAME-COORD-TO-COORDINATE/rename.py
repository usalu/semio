"""🔠️Rename `Coord`/`coord` to `Coordinate`/`coordinate` across compose source files.

Uses negative lookahead `(?!i)` to skip already-renamed `Coordinate*` tokens.
Skips historical folders (.repo/⚡️, .repo/🎫️ other than this ticket, .cursor, node_modules, target, __pycache__, dist, build).
"""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

REPO_ROOT = Path(r"c:/git/compose").resolve()
TICKET_DIR = REPO_ROOT / ".repo" / "🎫️" / "26" / "04" / "22" / "RENAME-COORD-TO-COORDINATE"

# File extensions to process
EXTENSIONS = {
    ".py", ".ts", ".tsx", ".js", ".jsx", ".rs", ".cs", ".go", ".rb",
    ".json", ".graphql", ".liveblocks", ".xmi", ".md", ".mdx", ".mmd",
    ".ttl", ".svg", ".yaml", ".yml", ".html", ".css",
}

# Skip any path containing one of these segments
SKIP_SEGMENTS = {
    ".git", "node_modules", "target", "__pycache__", "dist", "build",
    ".next", ".venv", "venv", ".cache", "test-results", "reports",
    "storybook-static", "htmlcov", ".vscode-test", "temp",
    "⚡️",
}

# Only rename within these top-level roots (plus the ticket itself)
INCLUDE_ROOTS = {"compose"}

# Skip other tickets & cursor plans but keep THIS ticket
def is_skipped(path: Path) -> bool:
    parts = set(path.parts)
    if parts & SKIP_SEGMENTS:
        return True
    # Skip .cursor/plans
    if ".cursor" in path.parts:
        return True
    # Skip other tickets (🎫️) except the current one
    try:
        rel = path.relative_to(REPO_ROOT)
    except ValueError:
        return True
    rel_parts = rel.parts
    # Skip the entire .repo tree (tickets contain historical literal `Coord` references)
    if len(rel_parts) >= 1 and rel_parts[0] == ".repo":
        return True
    # Restrict to INCLUDE_ROOTS (except root-level single files)
    if len(rel_parts) > 1 and rel_parts[0] not in INCLUDE_ROOTS:
        return True
    return False


PAT_UPPER = re.compile(r"Coord(?!i)")  # Coord, CoordFoo, Coords; not Coordinate*
PAT_LOWER = re.compile(r"coord(?!i)")  # coord, coords, coordFoo; not coordinate*

def process_file(path: Path) -> tuple[int, int]:
    try:
        text = path.read_text(encoding="utf-8")
    except (UnicodeDecodeError, PermissionError):
        return 0, 0
    new_text, n_upper = PAT_UPPER.subn("Coordinate", text)
    new_text, n_lower = PAT_LOWER.subn("coordinate", new_text)
    if n_upper or n_lower:
        path.write_text(new_text, encoding="utf-8", newline="")
    return n_upper, n_lower


def main(dry_run: bool = False) -> None:
    total_files = 0
    total_upper = 0
    total_lower = 0
    changed_files: list[tuple[Path, int, int]] = []
    for dirpath, dirnames, filenames in os.walk(REPO_ROOT):
        dp = Path(dirpath)
        # prune
        dirnames[:] = [
            d for d in dirnames
            if d not in SKIP_SEGMENTS and not (dp / d).is_symlink()
        ]
        for name in filenames:
            p = dp / name
            if p.suffix.lower() not in EXTENSIONS:
                continue
            if is_skipped(p):
                continue
            if dry_run:
                try:
                    text = p.read_text(encoding="utf-8")
                except (UnicodeDecodeError, PermissionError):
                    continue
                nu = len(PAT_UPPER.findall(text))
                nl = len(PAT_LOWER.findall(text))
                if nu or nl:
                    changed_files.append((p, nu, nl))
                    total_upper += nu
                    total_lower += nl
                    total_files += 1
            else:
                nu, nl = process_file(p)
                if nu or nl:
                    changed_files.append((p, nu, nl))
                    total_upper += nu
                    total_lower += nl
                    total_files += 1
    print(f"{'Would modify' if dry_run else 'Modified'} {total_files} files.")
    print(f"Upper replacements: {total_upper}")
    print(f"Lower replacements: {total_lower}")
    for p, nu, nl in sorted(changed_files, key=lambda x: -(x[1] + x[2]))[:100]:
        rel = p.relative_to(REPO_ROOT)
        safe = str(rel).encode("ascii", errors="replace").decode("ascii")
        print(f"  {safe}: Coord={nu} coord={nl}")


if __name__ == "__main__":
    main(dry_run="--dry-run" in sys.argv)
