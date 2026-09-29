"""🔍 FH5 (session 15): fast source search for the class review (the repo-wide grep stalls on build dirs): indexes
`.rs`/`.ts` under 🧰️framework and ✏️s/🔌️plugins once (pruning build/vendor dirs), then greps the index.
Usage: python3 fh5-grep.py <regex> [context_lines] [max_hits]"""
from __future__ import annotations

import os
import re
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
INDEX = REPO / ".tmp-ticket/wp-fh5/🗑️generated/files.txt"
PRUNE = {"target", "node_modules", "dist", "build", ".git", "lanes", ".vite", "pkg", "out", ".turbo", "coverage"}


def index() -> list[str]:
    if INDEX.exists():
        return INDEX.read_text().splitlines()
    files = []
    for top in ("🧰️framework", "✏️s/🔌️plugins"):
        for root, dirs, names in os.walk(REPO / top):
            dirs[:] = [name for name in dirs if name not in PRUNE and not name.startswith(".")]
            files.extend(str(Path(root, name).relative_to(REPO)) for name in names if name.endswith((".rs", ".ts", ".tsx")))
    INDEX.write_text("\n".join(files))
    return files


def main() -> None:
    pattern = re.compile(sys.argv[1])
    context = int(sys.argv[2]) if len(sys.argv) > 2 else 0
    limit = int(sys.argv[3]) if len(sys.argv) > 3 else 40
    hits = 0
    for path in index():
        try:
            lines = (REPO / path).read_text(errors="replace").splitlines()
        except OSError:
            continue
        for number, line in enumerate(lines):
            if pattern.search(line):
                hits += 1
                print(f"{path}:{number + 1}")
                for index_ in range(max(0, number - context), min(len(lines), number + context + 1)):
                    print(f"  {index_ + 1}: {lines[index_].strip()[:240]}")
                if hits >= limit:
                    return


if __name__ == "__main__":
    main()
