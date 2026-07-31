#!/usr/bin/env python3
"""Verify all slide src paths resolve to files in public/."""
import os
import re
import sys
import unicodedata

ROOT = os.path.abspath(
    os.path.join(os.path.dirname(__file__), "../../../../../../mit-bestand/präsentation/33.projektetage")
)
PUBLIC = os.path.join(ROOT, "public")


def collect_refs() -> set[str]:
    refs: set[str] = set()
    for dirpath, _, files in os.walk(ROOT):
        for fn in files:
            if not fn.endswith((".ts", ".tsx", ".md")):
                continue
            text = open(os.path.join(dirpath, fn), encoding="utf-8").read()
            for match in re.finditer(r'src:\s*["\'](/[^"\']+)["\']', text):
                refs.add(match.group(1).lstrip("/"))
    return refs


def main() -> int:
    on_disk = set(os.listdir(PUBLIC))
    missing: list[str] = []
    nfd_only: list[str] = []
    for ref in sorted(collect_refs()):
        if ref in on_disk:
            continue
        if unicodedata.normalize("NFD", ref) in on_disk:
            nfd_only.append(ref)
            continue
        missing.append(ref)
    if nfd_only:
        print("NFD filename mismatch (use NFC on disk):")
        for name in nfd_only:
            print(f"  {name!r}")
    if missing:
        print("Missing public assets:")
        for name in missing:
            print(f"  {name!r}")
        return 1
    if nfd_only:
        return 1
    print("All slide assets present.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
