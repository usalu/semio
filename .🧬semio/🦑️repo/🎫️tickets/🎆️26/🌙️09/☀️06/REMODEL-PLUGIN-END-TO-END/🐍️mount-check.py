#!/usr/bin/env python3
"""🧭️ Resolves every `#[path = "..."]` attribute found in the remodel plugin's two crate entry
files and in every `🦀️.rs` file under the remodeling artifact tree, relative to the file that
declares it, and reports which targets do not exist on disk. Read-only: makes no edits.
"""
import re
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")

ENTRY_FILES = [
    REPO / "✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/🦀️.rs",
    REPO / "✏️s/🔌️plugins/📸️remodel/🦀️.rs",
]

ARTIFACT_ROOT = REPO / "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling"

PATH_RE = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')


def find_rs_files():
    files = list(ENTRY_FILES)
    files.extend(sorted(ARTIFACT_ROOT.rglob("🦀️.rs")))
    # de-dup while preserving order
    seen = set()
    unique = []
    for f in files:
        if f not in seen:
            seen.add(f)
            unique.append(f)
    return unique


def main():
    total = 0
    dangling = []
    per_file_counts = {}
    for rs in find_rs_files():
        if not rs.exists():
            print(f"MISSING ENTRY FILE: {rs}")
            continue
        text = rs.read_text(encoding="utf-8", errors="replace")
        lines = text.splitlines()
        count_here = 0
        for lineno, line in enumerate(lines, start=1):
            m = PATH_RE.search(line)
            if not m:
                continue
            raw = m.group(1)
            count_here += 1
            total += 1
            if raw == ".":
                # self-mount: re-roots the module at the current directory; always "resolves"
                continue
            target = (rs.parent / raw).resolve()
            if not target.exists():
                dangling.append((str(rs), lineno, raw, str(target)))
        if count_here:
            per_file_counts[str(rs)] = count_here

    print(f"# scanned .rs files: {len(find_rs_files())}")
    print(f"# total #[path] attributes: {total}")
    print(f"# dangling (non-'.' targets that do not exist): {len(dangling)}")
    print()
    if dangling:
        print("=== DANGLING #[path] TARGETS ===")
        for f, lineno, raw, target in dangling:
            print(f"{f}:{lineno}: path={raw!r}")
            print(f"    resolved -> {target}")
    else:
        print("No dangling non-'.' #[path] targets found.")

    print()
    print("=== per-file #[path] attribute counts (non-zero) ===")
    for f, c in per_file_counts.items():
        print(f"{c:4d}  {f}")


if __name__ == "__main__":
    sys.exit(main())
