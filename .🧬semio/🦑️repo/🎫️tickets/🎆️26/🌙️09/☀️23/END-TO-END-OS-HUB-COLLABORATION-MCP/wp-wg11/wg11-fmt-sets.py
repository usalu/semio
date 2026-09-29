#!/usr/bin/env python3
"""🧹️ WG11 session 14d: makes the prepared sets rustfmt-clean. For every Rust file the sets touch, runs `rustfmt --check` in WG11's
overlay (all sets applied, repo `rustfmt.toml`) and in the live tree; each `-`/`+` group that only the overlay shows is looked up
in the set scripts' REPLACEMENT text and rewritten to rustfmt's form (a group found in an anchor, or not found exactly once, is
reported and left alone). Dry run by default; `--write` rewrites the set scripts.
Usage: python3 wg11-fmt-sets.py <file list> [--write]"""
import re
import subprocess
import sys
from pathlib import Path

LIVE = "/Users/ueli/Documents/semio"
OVERLAY = LIVE + "/.🧬semio/🌐hub/s14-wg11-overlay"
WORK = Path(LIVE + "/.tmp-ticket/wp-wg11")
SETS = [WORK / line.split()[0] for line in (WORK / "wg11-overlay-dev-patches.txt").read_text().splitlines() if line.strip() and not line.startswith("#")] + sorted(WORK.glob("*/*.rs"))


def groups(root: str, rel: str):
    out = subprocess.run(["rustfmt", "--edition", "2021", "--check", rel], cwd=root, capture_output=True, text=True).stdout
    name = "/".join(rel.split("/")[-2:])
    found, current, keep = [], None, False
    for line in out.splitlines():
        if line.startswith("Diff in "):
            keep = line.rstrip(":").rsplit(":", 1)[0].endswith(name)
            current = None
            continue
        if not keep:
            continue
        if line.startswith("-"):
            if current is None or current[1]:
                current = ([], [])
                found.append(current)
            current[0].append(line[1:])
        elif line.startswith("+"):
            if current is None:
                current = ([], [])
                found.append(current)
            current[1].append(line[1:])
        else:
            current = None
    return [("\n".join(removed), "\n".join(added)) for removed, added in found]


def main():
    write = "--write" in sys.argv
    files = [line.strip().replace(LIVE + "/", "") for line in Path(sys.argv[1]).read_text().splitlines() if line.strip()]
    sources = {path: path.read_text(encoding="utf-8") for path in SETS}
    fixed, skipped = 0, 0
    for rel in files:
        if not Path(OVERLAY, rel).exists():
            continue
        live = set(groups(LIVE, rel)) if Path(LIVE, rel).exists() else set()
        for removed, added in groups(OVERLAY, rel):
            if (removed, added) in live or not removed:
                continue
            if not added:
                print(f"SKIP {rel}: pure removal (a moved line — fix by hand): {removed[:90]!r}")
                skipped += 1
                continue
            hits = [path for path, source in sources.items() if source.count(removed) == 1]
            total = sum(source.count(removed) for source in sources.values())
            if total != 1 or len(hits) != 1:
                print(f"SKIP {rel}: group found {total}x: {removed[:90]!r}")
                skipped += 1
                continue
            if Path(LIVE, rel).exists() and removed in Path(LIVE, rel).read_text(encoding="utf-8"):
                print(f"SKIP {rel}: group is live code (an anchor, not a replacement): {removed[:90]!r}")
                skipped += 1
                continue
            sources[hits[0]] = sources[hits[0]].replace(removed, added)
            print(f"FIX {hits[0].name} <- {rel.split('/')[-2]}: {removed.strip()[:70]!r}")
            fixed += 1
    if write:
        for path, source in sources.items():
            if source != path.read_text(encoding="utf-8"):
                path.write_text(source, encoding="utf-8")
    print(f"{'WRITTEN' if write else 'DRY RUN'}: {fixed} groups fixed, {skipped} skipped")


if __name__ == "__main__":
    main()
