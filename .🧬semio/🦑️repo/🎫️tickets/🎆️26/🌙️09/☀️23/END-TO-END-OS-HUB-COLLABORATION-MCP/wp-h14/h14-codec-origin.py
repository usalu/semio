#!/usr/bin/env python3
"""🧊️ H14 window-3 patch set "codec origin": one post-assembly codec origin per compiled guest in the owned runtime
(`OwnedRuntime::codec_call`, `CompiledHandle.owned` → `OwnedCompiledGuest`), its laws, and the hub's residency charging the
origin (`GuestResidentFootprintV1 for CompiledHandle` → `codec_origin_bytes()`).

  capture: h14-codec-origin.py capture   — records payload/<id>.old (the tree file now) and payload/<id>.new (the prepared
                                           file: the overlay's copy, or the hub hunk applied to the tree file)
  apply:   h14-codec-origin.py [--write] [--root <dir>] [--only a,b]
           dry run by default; a target equal to `.old` is replaced, one equal to `.new` counts as applied, one a peer changed
           since is merged three-way (`git merge-file -p`, read-only) and applied only when the merge is clean. Every target
           is re-read right before it is written; tree writes keep a backup under `.🧬semio/🌐hub/s14-h14-backup/<stamp>/`.
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time

REPO = "/Users/ueli/Documents/semio"
OVERLAY = os.path.join(REPO, ".🧬semio/🌐hub/s14-h14-overlay")
HERE = os.path.dirname(os.path.abspath(__file__))
PAYLOAD = os.path.join(HERE, "payload")
BACKUP = os.path.join(REPO, ".🧬semio/🌐hub/s14-h14-backup")
HOST = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host"
CATALOG = "🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs"
HUB_OLD = '''impl GuestResidentFootprintV1 for semio_framework_plugin_host::CompiledHandle {
    /// 🧩️ A compiled handle holds its parsed component, which its registration charges, and nothing beyond it.
    fn footprint_bytes(&self) -> u64 {
        0
    }
}'''
HUB_NEW = '''impl GuestResidentFootprintV1 for semio_framework_plugin_host::CompiledHandle {
    /// 🧊️ Beyond its parsed component (which its registration charges) a compiled guest holds the owned interpreter's codec
    /// origin once a codec call assembled it: the guest's linear memory right after its plugin bundle assembled.
    fn footprint_bytes(&self) -> u64 {
        self.codec_origin_bytes()
    }
}'''
ENTRIES = [
    {"id": "plugin-host", "path": f"{HOST}/🦀️.rs", "source": "overlay"},
    {"id": "owned-instance-open-laws", "path": f"{HOST}/🧪️tests/🔬️owned-instance-open/🦀️.rs", "source": "overlay"},
    {"id": "hub-residency-footprint", "path": CATALOG, "source": "hunk"},
]


def read(path):
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write(path, text):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def capture():
    os.makedirs(PAYLOAD, exist_ok=True)
    for entry in ENTRIES:
        old = read(os.path.join(REPO, entry["path"]))
        if entry["source"] == "overlay":
            new = read(os.path.join(OVERLAY, entry["path"]))
        else:
            if old.count(HUB_OLD) != 1:
                sys.exit(f"{entry['id']}: the hub hunk anchor is not in the tree exactly once")
            new = old.replace(HUB_OLD, HUB_NEW)
        write(os.path.join(PAYLOAD, f"{entry['id']}.old"), old)
        write(os.path.join(PAYLOAD, f"{entry['id']}.new"), new)
    write(os.path.join(PAYLOAD, "manifest.json"), json.dumps({"captured": time.strftime("%Y-%m-%d %H:%M:%S"), "entries": ENTRIES}, ensure_ascii=False, indent=2))
    print(f"captured {len(ENTRIES)} entries")


def merge(current, old, new):
    with tempfile.TemporaryDirectory() as scratch:
        names = []
        for label, text in (("current", current), ("old", old), ("new", new)):
            path = os.path.join(scratch, label)
            write(path, text)
            names.append(path)
        result = subprocess.run(["git", "merge-file", "-p", names[0], names[1], names[2]], capture_output=True, text=True)
    return result.stdout, result.returncode == 0


def apply(root, write_mode, only):
    stamp = time.strftime("%Y%m%dT%H%M%S")
    failed = False
    for entry in [entry for entry in ENTRIES if not only or entry["id"] in only]:
        target = os.path.join(root, entry["path"])
        old, new = read(os.path.join(PAYLOAD, f"{entry['id']}.old")), read(os.path.join(PAYLOAD, f"{entry['id']}.new"))
        current = read(target)
        if current == new:
            print(f"{entry['id']}: applied already")
            continue
        if current == old:
            planned, how = new, "replace"
        else:
            planned, clean = merge(current, old, new)
            if not clean:
                print(f"{entry['id']}: CONFLICT — a peer changed {entry['path']} where this set does; re-derive")
                failed = True
                continue
            how = "merge"
        print(f"{entry['id']}: {how} ({len(current)} → {len(planned)} bytes){'' if write_mode else ' [dry run]'}")
        if write_mode:
            if read(target) != current:
                print(f"{entry['id']}: changed while planning — rerun")
                failed = True
                continue
            if root == REPO:
                backup = os.path.join(BACKUP, stamp, entry["path"])
                os.makedirs(os.path.dirname(backup), exist_ok=True)
                shutil.copy2(target, backup)
            write(target, planned)
    return 1 if failed else 0


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("command", nargs="?", default="apply", choices=["apply", "capture"])
    parser.add_argument("--write", action="store_true")
    parser.add_argument("--root", default=REPO)
    parser.add_argument("--only", default="")
    args = parser.parse_args()
    if args.command == "capture":
        capture()
        return 0
    return apply(args.root, args.write, set(filter(None, args.only.split(","))))


if __name__ == "__main__":
    sys.exit(main())
