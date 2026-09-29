#!/usr/bin/env python3
"""🪞️ SH2: rebuilds the landing state of both prepared sets (space-home set `--write`, then B1 `--write`) in the compose scratch
root and mirrors every path either set touches into the SH2 overlay (`.🧬semio/🌐hub/s14-sh2-overlay`), deleting what the sets
delete — so the overlay always holds exactly the tree + both sets. Identical files are left untouched, so cargo never
rebuilds a crate whose sources did not change."""
import filecmp, json, os, shutil, subprocess, sys

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
SH2 = os.path.dirname(HERE)
COMPOSE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-compose")
OVERLAY = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-overlay")


def main():
    if subprocess.run(["python3", os.path.join(HERE, "b1-compose.py")], capture_output=True, text=True).returncode != 0:
        print("compose dry run failed")
        return 1
    result = subprocess.run(["python3", os.path.join(HERE, "b1-apply.py"), "--write", "--root", COMPOSE], capture_output=True, text=True)
    print(result.stdout.strip().splitlines()[-1])
    if result.returncode != 0:
        return 1
    paths = sorted({entry["path"] for manifest in (os.path.join(SH2, "payload/manifest.json"), os.path.join(HERE, "payload/manifest.json")) for entry in json.load(open(manifest, encoding="utf-8"))})
    copied = removed = 0
    for relative in paths:
        source, target = os.path.join(COMPOSE, relative), os.path.join(OVERLAY, relative)
        if os.path.exists(source):
            if os.path.exists(target) and filecmp.cmp(source, target, shallow=False):
                continue
            os.makedirs(os.path.dirname(target), exist_ok=True)
            shutil.copyfile(source, target)
            copied += 1
        elif os.path.exists(target):
            os.remove(target)
            removed += 1
    print(f"overlay refreshed: {copied} written, {removed} removed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
