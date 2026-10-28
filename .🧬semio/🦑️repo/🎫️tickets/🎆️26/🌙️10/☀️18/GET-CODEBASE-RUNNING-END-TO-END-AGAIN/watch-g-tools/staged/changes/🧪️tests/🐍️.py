#!/usr/bin/env python3
"""Independent model of the change journal (shares no code with 🟦️.ts): strict record decoding, the contiguity-checked read of an epoch after a sequence number, the affects rule, row merging, cursor text and exclusion.

  🐍️.py vectors VECTORS.json     -> JSON {decode, affects, merge, cursor, excluded, read} computed from the vectors
  🐍️.py diff    SNAPSHOT.json    -> JSON delta of two tree snapshots {before, after}
"""
import json
import os
import re
import sys
import unicodedata

KINDS = {"a", "m", "r", "n"}
SEGMENT = re.compile(r"^([0-9a-f]{16})\.(\d{16})\.log$")


def valid_path(path):
    return isinstance(path, str) and 0 < len(path) <= 4096 and not re.search(r"^/|(^|/)\.\.?(/|$)|\x00|\\", path)


def is_int(value):
    return isinstance(value, int) and not isinstance(value, bool)


def decode(line):
    try:
        value = json.loads(line)
    except ValueError:
        return None
    if not isinstance(value, dict) or set(value) - {"s", "t", "r", "l"}:
        return None
    s, t = value.get("s"), value.get("t")
    if not is_int(s) or s < 1 or not is_int(t) or t < 0 or ("r" in value) == ("l" in value):
        return None
    if "l" in value:
        l = value["l"]
        if not isinstance(l, list) or len(l) != 2 or not isinstance(l[0], str) or not l[0] or len(l[0]) > 64 or not isinstance(l[1], str) or len(l[1]) > 4096:
            return None
        return {"seq": s, "lost": l}
    r = value["r"]
    if not isinstance(r, list):
        return None
    for row in r:
        if not isinstance(row, list) or len(row) != 3 or row[0] not in KINDS or row[1] not in (0, 1) or isinstance(row[1], bool) or not valid_path(row[2]):
            return None
    return {"seq": s, "rows": r}


def fold(path):
    return unicodedata.normalize("NFC", path).lower()


def affects(row, root, insensitive):
    if root == "":
        return True
    path, base = (fold(row[2]), fold(root)) if insensitive else (row[2], root)
    if path == base or path.startswith(base + "/"):
        return True
    return base.startswith(path + "/") and not (row[1] == 1 and row[0] == "m")


def merge(rows):
    merged = {}
    for kind, directory, path in rows:
        if path in merged:
            old = merged[path]
            merged[path] = [old[0] if old[0] != "m" else kind, old[1] | directory, path]
        else:
            merged[path] = [kind, directory, path]
    return sorted(merged.values(), key=lambda row: row[2])


def changed_watched(watched, rows, insensitive):
    return sorted({path for path in watched if any(affects(row, path, insensitive) for row in rows)})


def excluded(state, root, insensitive):
    if root == "":
        return False
    path = fold(root) if insensitive else root
    for prefix in state["exclude"]["rooted"]:
        base = fold(prefix) if insensitive else prefix
        if path == base or path.startswith(base + "/"):
            return True
    names = {fold(n) if insensitive else n for n in state["exclude"]["names"]}
    return any(part in names for part in path.split("/"))


def read_after(directory, epoch, after):
    segments = sorted((int(m.group(2)), name) for name in os.listdir(directory) for m in [SEGMENT.match(name)] if m and m.group(1) == epoch)
    if not segments:
        return {"ok": True, "seqs": [], "head": 0} if after == 0 else {"ok": False, "reason": "gap"}
    records, seen, previous_last = [], segments[-1][0] - 1, None
    for first, name in reversed(segments):
        with open(os.path.join(directory, name), "rb") as handle:
            text = handle.read().decode("utf8")
        lines = text.split("\n")[:-1]
        mine = []
        for line in lines:
            m = re.match(r'^\{"s":(\d+),', line)
            if not m:
                return {"ok": False, "reason": "gap"}
            seen = max(seen, int(m.group(1)))
            if int(m.group(1)) > after:
                record = decode(line)
                if record is None:
                    return {"ok": False, "reason": "gap"}
                mine.append(record)
        records = mine + records
        if first <= after + 1:
            break
        if first == segments[0][0]:
            return {"ok": False, "reason": "floor"}
    for index, record in enumerate(records):
        if record["seq"] != after + 1 + index:
            return {"ok": False, "reason": "gap"}
    head = records[-1]["seq"] if records else seen
    if head < after:
        return {"ok": False, "reason": "gap"}
    return {"ok": True, "seqs": [r["seq"] for r in records], "head": head}


def cursor_text(text):
    m = re.match(r"^([0-9a-f]{16}):(\d{1,15})$", text) if isinstance(text, str) else None
    return {"epoch": m.group(1), "seq": int(m.group(2))} if m else None


def diff(before, after):
    return {
        "added": sorted(p for p in after if p not in before),
        "removed": sorted(p for p in before if p not in after),
        "changed": sorted(p for p in after if p in before and after[p] != before[p]),
    }


def main():
    command = sys.argv[1]
    data = json.load(open(sys.argv[2], encoding="utf8"))
    if command == "diff":
        print(json.dumps(diff(data["before"], data["after"])))
        return
    out = {
        "decode": [decode(line) is not None for line in data["decode"]],
        "affects": [affects(c["row"], c["root"], c["insensitive"]) for c in data["affects"]],
        "merge": [merge(rows) for rows in data["merge"]],
        "cursor": [cursor_text(text) for text in data["cursor"]],
        "watched": [changed_watched(c["watched"], c["rows"], c["insensitive"]) for c in data.get("watched", [])],
        "excluded": [excluded(c["state"], c["root"], c["insensitive"]) for c in data["excluded"]],
        "read": [read_after(c["directory"], c["epoch"], c["after"]) for c in data["read"]],
    }
    print(json.dumps(out))


if __name__ == "__main__":
    main()
