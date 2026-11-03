#!/usr/bin/env python3
"""🏷️ Law of unique binary tags: every artifact's mutation leaf JSON `binaryTag` must be unique and equal the tag of its record in the mutations protocol file (the only source of the wire tags; widths beyond u8 are varint). Usage: python3 -I 🐍️.py"""
import glob
import json
import pathlib
import re
import sys

artifacts = pathlib.Path(__file__).resolve().parents[3] / "🗿️artifacts"
norm = lambda s: s.replace("-", "")
failures = []
checked = 0
for art in sorted(artifacts.iterdir()):
    protocols = glob.glob(str(art / "**" / "💾️binary" / "🧬️mutations" / "📡️.protocol.semio"), recursive=True)
    if not protocols:
        continue
    tags = {}
    for p in protocols:
        for kind, tag in re.findall(r"^record (\S+) tag=(\d+)$", pathlib.Path(p).read_text(), re.M):
            if norm(kind) in tags:
                failures.append(f"{art.name}: protocol repeats kind {kind}")
            tags[norm(kind)] = int(tag)
    owners = {}
    for p in glob.glob(str(art / "**" / "🧬️mutations" / "**" / "🔣️.json"), recursive=True):
        try:
            leaf = json.loads(pathlib.Path(p).read_text())
        except ValueError:
            continue
        if not isinstance(leaf, dict) or "binaryTag" not in leaf or "semanticKind" not in leaf:
            continue
        checked += 1
        kind, tag = leaf["semanticKind"], leaf["binaryTag"]
        owners.setdefault(tag, []).append(kind)
        if norm(kind) not in tags:
            failures.append(f"{art.name}: {kind} has no protocol record")
        elif tags[norm(kind)] != tag:
            failures.append(f"{art.name}: {kind} json tag {tag} != protocol tag {tags[norm(kind)]}")
    failures += [f"{art.name}: tag {t} used by {ks}" for t, ks in owners.items() if len(ks) > 1]
    values = list(tags.values())
    failures += [f"{art.name}: protocol tag {t} repeated" for t in set(values) if values.count(t) > 1]
assert checked > 0
if failures:
    print("\n".join(failures))
    sys.exit(1)
print(json.dumps({"leavesChecked": checked}))
