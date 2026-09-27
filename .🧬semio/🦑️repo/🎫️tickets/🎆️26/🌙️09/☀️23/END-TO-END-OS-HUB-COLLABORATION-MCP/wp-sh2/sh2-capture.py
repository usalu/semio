#!/usr/bin/env python3
"""📸️ SH2 payload capture: for every path in `sh2-files.txt`, the tree content it was prepared against (`s14-sh2-base`) becomes
`payload/<n>.old` and the prepared content (`s14-sh2-stage`) becomes `payload/<n>.new`; `payload/manifest.json` lists them.
Unchanged files are dropped. Paths only under the gitignored hub dir are written besides the payload.
"""
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
BASE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-base")
STAGE = os.path.join(REPO, ".🧬semio/🌐hub/s14-sh2-stage")
PAYLOAD = os.path.join(HERE, "payload")


def main():
    with open(os.path.join(HERE, "sh2-files.txt"), encoding="utf-8") as handle:
        paths = [line.rstrip("\n") for line in handle if line.strip()]
    for name in os.listdir(PAYLOAD):
        os.remove(os.path.join(PAYLOAD, name))
    manifest = []
    for index, relative in enumerate(paths):
        base_path, stage_path = os.path.join(BASE, relative), os.path.join(STAGE, relative)
        old = open(base_path, encoding="utf-8").read() if os.path.exists(base_path) else None
        new = open(stage_path, encoding="utf-8").read() if os.path.exists(stage_path) else None
        if old == new:
            continue
        hunk = f"{index:03d}"
        if old is not None:
            with open(os.path.join(PAYLOAD, f"{hunk}.old"), "w", encoding="utf-8") as handle:
                handle.write(old)
        if new is not None:
            with open(os.path.join(PAYLOAD, f"{hunk}.new"), "w", encoding="utf-8") as handle:
                handle.write(new)
        manifest.append({"id": hunk, "path": relative, "mode": "create" if old is None else ("delete" if new is None else "whole")})
    with open(os.path.join(PAYLOAD, "manifest.json"), "w", encoding="utf-8") as handle:
        json.dump(manifest, handle, ensure_ascii=False, indent=1)
    print(f"captured {len(manifest)} changed file(s) of {len(paths)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
