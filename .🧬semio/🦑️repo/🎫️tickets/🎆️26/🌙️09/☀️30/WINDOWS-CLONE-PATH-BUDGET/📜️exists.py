#!/usr/bin/env python3
"""🪟 Check whether indexed shortened fixture files exist on disk."""

import os
import subprocess

PREFIXES = [
    "👁️set-layer-visibility/✅️set-layer/",
    "turns-multiple-resisting-systems",
]


def main() -> None:
    raw = subprocess.check_output(["git", "ls-files", "-z"])
    paths = [item.decode() for item in raw.split(b"\0") if item]
    for prefix in PREFIXES:
        hits = [path for path in paths if prefix in path]
        print(f"\n== {prefix} indexed {len(hits)}")
        missing = [path for path in hits if not os.path.exists(path)]
        print("missing", len(missing), "present", len(hits) - len(missing))
        for path in (missing or hits)[:5]:
            print(" ", path)
            parent = os.path.dirname(path)
            print("   parent_exists", os.path.isdir(parent))
            grand = os.path.dirname(parent)
            if os.path.isdir(grand):
                print("   siblings", os.listdir(grand)[:12])


if __name__ == "__main__":
    main()
