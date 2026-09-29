#!/usr/bin/env python3
"""🧮️ RS1 set `rs1-raster-tier` (ticket 26/09/23 session 15): the first-party OpenType reader `semio-framework-fonts`, the
target-neutral CPU tier `semio_framework_raster::cpu` (VectorScene grows Paint/FillRule/StrokeStyle; the GPU tier maps them),
stdio's `s.stdio.semio/v1/drawing` → png leaf moved onto the tier (text + bilinear images), and shooting `photos:out` rendered
through it on every tier (no resvg, no `semio-framework-os` dependency).

The set is data: `rs1-raster-tier.set.json` beside this script (created files verbatim; edited files as exact-match hunks with
enough context to be unique, so peers' edits elsewhere in the same files survive). Idempotent: a created file already
identical, or a hunk whose replacement is already present, is reported `applied` and skipped. Cargo.lock is NOT in the set:
the first cargo run adds the `semio-framework-fonts` package entry offline (no version changes).

usage:
  rs1-raster-tier.py [--dry-run] [--root <tree>]   report create/apply/applied/conflict per file (default; writes nothing)
  rs1-raster-tier.py --write [--root <tree>]       apply; backups + created-file list under .🧬semio/🌐hub/s15-rs1-backup/<stamp>/
  rs1-raster-tier.py --revert [--root <tree>]      restore the newest backup of that root and delete the files it created
  rs1-raster-tier.py capture                       (RS1 only) rebuild the set JSON from the RS1 overlay against set/base/
"""
import difflib
import json
import os
import shutil
import sys
import time

REPO = "/Users/ueli/Documents/semio"
HERE = os.path.dirname(os.path.abspath(__file__))
SET = os.path.join(HERE, "rs1-raster-tier.set.json")
BASE = os.path.join(HERE, "set", "base")
OVERLAY = os.path.join(REPO, ".🧬semio", "🌐hub", "s15-rs1-overlay")
BACKUPS = os.path.join(REPO, ".🧬semio", "🌐hub", "s15-rs1-backup")

CREATED = [
    "🧰️framework/🔨️modules/🔤️fonts/🦀️.rs",
    "🧰️framework/🔨️modules/🔤️fonts/🧪️tests/🔬️unit/🦀️.rs",
    "🧰️framework/🔨️modules/🔤️fonts/📦️packages/🦀️rust/Cargo.toml",
    "🧰️framework/🔨️modules/🔤️fonts/📦️packages/🦀️rust/🦀️.rs",
    "🧰️framework/🔨️modules/🔤️fonts/📦️packages/🦀️rust/📋️project.json",
    "🧰️framework/🔨️modules/🔤️fonts/📦️packages/🦀️rust/📜️script.ts",
    "🧰️framework/🔨️modules/🔤️fonts/🔮️oracles/🔣️.json",
    "🧰️framework/🔨️modules/🖌️raster/🧮️cpu/🦀️.rs",
    "🧰️framework/🔨️modules/🖌️raster/🧮️cpu/🧪️tests/🔬️unit/🦀️.rs",
    "🧰️framework/🔨️modules/🖌️raster/🧮️cpu/🧫️fixtures/🔣️.json",
    "🧰️framework/🔨️modules/🖌️raster/🧮️cpu/🧬️schema/🔣️.json",
    "🧰️framework/🔨️modules/🖌️raster/🧮️cpu/🔮️oracles/🔣️.json",
]

EDITED = [
    "Cargo.toml",
    "🧰️framework/🔨️modules/🖌️raster/🦀️.rs",
    "🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/Cargo.toml",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🧪️tests/🔬️unit/🦀️.rs",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml",
    "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
    "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
    "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs",
    "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/📦️packages/🦀️rust/Cargo.toml",
]


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def hunks(before, after):
    old_lines, new_lines = before.splitlines(keepends=True), after.splitlines(keepends=True)
    for context in range(3, 40):
        matcher = difflib.SequenceMatcher(a=old_lines, b=new_lines, autojunk=False)
        found = []
        for group in matcher.get_grouped_opcodes(context):
            (i1, j1), (i2, j2) = (group[0][1], group[0][3]), (group[-1][2], group[-1][4])
            found.append({"old": "".join(old_lines[i1:i2]), "new": "".join(new_lines[j1:j2])})
        if all(before.count(h["old"]) == 1 and after.count(h["new"]) == 1 for h in found):
            return found
    raise SystemExit("hunks stay ambiguous at 40 lines of context")


def capture():
    payload = {"set": "rs1-raster-tier", "captured": time.strftime("%Y-%m-%d %H:%M:%S"), "created": {}, "edited": {}}
    for rel in CREATED:
        payload["created"][rel] = read(os.path.join(OVERLAY, rel))
    for rel in EDITED:
        payload["edited"][rel] = hunks(read(os.path.join(BASE, rel)), read(os.path.join(OVERLAY, rel)))
    with open(SET, "w", encoding="utf-8") as handle:
        json.dump(payload, handle, ensure_ascii=False, indent=1)
    print(f"captured {len(CREATED)} created + {sum(len(v) for v in payload['edited'].values())} hunks in {len(EDITED)} files -> {SET}")


def plan(root, payload):
    rows, conflicts = [], 0
    for rel, content in payload["created"].items():
        target = os.path.join(root, rel)
        if not os.path.exists(target):
            rows.append(("create", rel, None))
        elif read(target) == content:
            rows.append(("applied", rel, None))
        else:
            rows.append(("CONFLICT", rel, "exists with other content"))
            conflicts += 1
    for rel, file_hunks in payload["edited"].items():
        target = os.path.join(root, rel)
        text = read(target) if os.path.exists(target) else None
        for index, hunk in enumerate(file_hunks):
            if text is None:
                rows.append(("CONFLICT", rel, f"hunk {index}: file missing"))
                conflicts += 1
            elif text.count(hunk["new"]) == 1:
                rows.append(("applied", rel, f"hunk {index}"))
            elif text.count(hunk["old"]) == 1:
                rows.append(("apply", rel, f"hunk {index}"))
            else:
                rows.append(("CONFLICT", rel, f"hunk {index}: anchor found {text.count(hunk['old'])}x, replacement {text.count(hunk['new'])}x"))
                conflicts += 1
    return rows, conflicts


def write(root, payload):
    rows, conflicts = plan(root, payload)
    if conflicts:
        report(rows)
        raise SystemExit(f"{conflicts} conflict(s): nothing written")
    stamp = time.strftime("%Y%m%d-%H%M%S")
    backup = os.path.join(BACKUPS, stamp)
    os.makedirs(backup, exist_ok=True)
    created = []
    for rel, content in payload["created"].items():
        target = os.path.join(root, rel)
        if os.path.exists(target):
            continue
        os.makedirs(os.path.dirname(target), exist_ok=True)
        with open(target, "w", encoding="utf-8") as handle:
            handle.write(content)
        created.append(rel)
    for rel, file_hunks in payload["edited"].items():
        target = os.path.join(root, rel)
        text = read(target)
        updated = text
        for hunk in file_hunks:
            if updated.count(hunk["new"]) != 1 and updated.count(hunk["old"]) == 1:
                updated = updated.replace(hunk["old"], hunk["new"], 1)
        if updated != text:
            os.makedirs(os.path.dirname(os.path.join(backup, rel)), exist_ok=True)
            shutil.copy2(target, os.path.join(backup, rel))
            with open(target, "w", encoding="utf-8") as handle:
                handle.write(updated)
    with open(os.path.join(backup, ".rs1-manifest.json"), "w", encoding="utf-8") as handle:
        json.dump({"root": root, "created": created}, handle, ensure_ascii=False, indent=1)
    report(rows)
    print(f"written; backup {backup}")


def revert(root):
    stamps = sorted(entry for entry in os.listdir(BACKUPS) if os.path.exists(os.path.join(BACKUPS, entry, ".rs1-manifest.json"))) if os.path.isdir(BACKUPS) else []
    for stamp in reversed(stamps):
        backup = os.path.join(BACKUPS, stamp)
        manifest = json.load(open(os.path.join(backup, ".rs1-manifest.json"), encoding="utf-8"))
        if os.path.realpath(manifest["root"]) != os.path.realpath(root):
            continue
        for rel in manifest["created"]:
            target = os.path.join(root, rel)
            if os.path.exists(target):
                os.remove(target)
                parent = os.path.dirname(target)
                while parent != root and os.path.isdir(parent) and not os.listdir(parent):
                    os.rmdir(parent)
                    parent = os.path.dirname(parent)
        for folder, _, files in os.walk(backup):
            for name in files:
                if name == ".rs1-manifest.json":
                    continue
                source = os.path.join(folder, name)
                shutil.copy2(source, os.path.join(root, os.path.relpath(source, backup)))
        os.rename(os.path.join(backup, ".rs1-manifest.json"), os.path.join(backup, ".rs1-manifest.reverted.json"))
        print(f"reverted {backup}")
        return
    raise SystemExit(f"no backup of {root} to revert")


def report(rows):
    for state, rel, detail in rows:
        print(f"{state:9} {rel}{'  ' + detail if detail else ''}")
    counts = {}
    for state, _, _ in rows:
        counts[state] = counts.get(state, 0) + 1
    print("summary", counts)


def main(argv):
    if argv[:1] == ["capture"]:
        return capture()
    root = argv[argv.index("--root") + 1] if "--root" in argv else REPO
    if "--revert" in argv:
        return revert(root)
    payload = json.load(open(SET, encoding="utf-8"))
    if "--write" in argv:
        return write(root, payload)
    rows, conflicts = plan(root, payload)
    report(rows)
    if conflicts:
        raise SystemExit(f"{conflicts} conflict(s)")


if __name__ == "__main__":
    main(sys.argv[1:])
