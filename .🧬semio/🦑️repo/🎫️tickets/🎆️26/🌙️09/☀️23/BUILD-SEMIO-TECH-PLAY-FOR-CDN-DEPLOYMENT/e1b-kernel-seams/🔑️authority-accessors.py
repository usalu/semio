#!/usr/bin/env python3
"""🔑️ Rewrites every field read of the sealed one-item authority, seal and prepared candidate that rustc reports as
private (E0616, `cargo --message-format short` log) into the owning module's accessor call (wave E1b).
Usage: authority-accessors.py CARGO_SHORT_LOG
"""
import collections, os, re, sys

AUTHORITY = {"operation": "operation()", "generation": "generation()", "base_revision": "base_revision()", "base_applied_edit_count": "base_applied_edit_count()",
             "next_sequence_number": "next_sequence_number()", "next_clock": "next_clock()", "actor": "actor()", "line": "line_id()", "group_id": "group_id()", "stamped_edit_id": "stamped_edit_id()"}
PREPARED = {"edit": "edit()", "post_snapshot": "post_snapshot()", "next_clock": "next_clock()", "edit_digest": "edit_digest()", "local_actor": "local_actor()",
            "applied_edit_id": "applied_edit_id()", "tail_edit_id": "tail_edit_id()", "foreign_step_presence": "foreign_step_presence()"}
edits = collections.defaultdict(set)
for l in open(sys.argv[1], encoding="utf-8"):
    m = re.match(r"^(.*🦀️\.rs):(\d+):(\d+): error\[E0616\]: field `(\w+)` of struct `(?:[\w:]+::)?(\w+)` is private", l)
    if m: edits[os.path.normpath(m.group(1))].add((int(m.group(2)), int(m.group(3)), m.group(4), m.group(5)))
total, skipped = 0, []
for f, es in edits.items():
    lines = open(f, encoding="utf-8").read().split("\n")
    for line, col, field, struct in sorted(es, key=lambda e: (e[0], -e[1])):
        text = lines[line - 1]
        i = len(text.encode("utf-8")[:col - 1].decode("utf-8", "ignore"))
        if not text[i:].startswith(field): skipped.append((f, line, field)); continue
        if struct == "ArtifactStoreOneItemSeal" and field == "authority" and text[:i].endswith(".seal."): text = text[:i - 5] + "authority()" + text[i + len(field):]
        elif struct == "ArtifactStoreOneItemLiveAuthority": text = text[:i] + AUTHORITY[field] + text[i + len(field):]
        elif struct == "ArtifactStoreOneItemPrepared": text = text[:i] + PREPARED[field] + text[i + len(field):]
        else: skipped.append((f, line, field)); continue
        lines[line - 1] = text; total += 1
    open(f, "w", encoding="utf-8").write("\n".join(lines))
print(f"[accessors] {total} reads rewritten")
for s in skipped: print("[accessors] skipped", s[0].split("🏪️store/")[-1], s[1:])
