#!/usr/bin/env python3
"""🪟 Diagnose include_str paths that no longer resolve after fixture renames."""

import os
import re
import subprocess

ROOTS = ["✏️s"]


def missing_includes() -> list[tuple[str, str, str]]:
    missing: list[tuple[str, str, str]] = []
    for dirpath, _, names in os.walk("✏️s"):
        for name in names:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            try:
                text = open(path, encoding="utf-8").read()
            except (OSError, UnicodeDecodeError):
                continue
            for rel in re.findall(r'include_str!\(\s*"([^"]+)"', text):
                target = os.path.normpath(os.path.join(dirpath, rel))
                if not os.path.exists(target):
                    missing.append((path, rel, target))
    return missing


def head_includes(path: str) -> list[str]:
    show = subprocess.run(["git", "show", f"HEAD:{path}"], capture_output=True)
    if show.returncode != 0:
        return ["ABSENT_IN_HEAD"]
    text = show.stdout.decode("utf-8", "replace")
    return re.findall(r'include_str!\(\s*"([^"]+)"', text)


def main() -> None:
    missing = missing_includes()
    files = sorted({item[0] for item in missing})
    print(f"missing {len(missing)} files {len(files)}")
    for path in files:
        rels = [rel for file, rel, _ in missing if file == path]
        heads = head_includes(path)
        print("---")
        print(path)
        print("missing_count", len(rels), "head_includes", len(heads))
        print("NOW ", rels[0])
        same = next((item for item in heads if item.endswith(rels[0].split("/")[-1]) and rels[0].split("/")[-3] in item), "")
        print("HEAD", same or (heads[0] if heads else ""))


if __name__ == "__main__":
    main()
