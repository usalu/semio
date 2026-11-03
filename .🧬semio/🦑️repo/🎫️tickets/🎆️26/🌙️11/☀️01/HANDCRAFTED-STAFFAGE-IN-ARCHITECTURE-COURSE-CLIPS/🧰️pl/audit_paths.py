#!/usr/bin/env python3
"""🧭 Audit path lengths and naming of new or modified ticket paths (read-only)."""
import json
import os
import re
import sys

ROOT = r"C:\git\semio"
TICKET = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
GEN = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🗑️generated", "pl")
BIN = os.path.join(GEN, "status.bin")
OUT = os.path.join(GEN, "audit.json")
EXPECTED_TEXT_EXT = {"json", "ts", "py", "svg", "md", "cases.json", "schema.json"}
STRAY_EXT = (".txt", ".log", ".png", ".jsonl", ".out")
STAFFAGE = "🎓️teaching/🏛️architecture/🧍️staffage"


def entries():
    raw = open(BIN, "rb").read().decode("utf-8").split("\0")
    i = 0
    while i < len(raw) and raw[i]:
        xy, path = raw[i][:2], raw[i][3:]
        i += 1
        if xy[0] in "RC":
            i += 1
        yield xy, path


def win(rel):
    return ROOT + "\\" + rel.replace("/", "\\")


def is_emoji_seg(seg):
    return bool(seg) and ord(seg[0]) > 127


def main():
    rows = []
    for xy, rel in entries():
        full = win(rel)
        rows.append({
            "xy": xy,
            "rel": rel,
            "utf16": len(full.encode("utf-16-le")) // 2,
            "chars": len(full),
            "bytes_utf8": len(full.encode("utf-8")),
        })
    rows.sort(key=lambda r: -r["utf16"])
    over256 = [r for r in rows if r["utf16"] > 256]
    over240 = [r for r in rows if 240 < r["utf16"] <= 256]
    newdirs, deviations, stray = set(), [], []
    stems = {}
    for r in rows:
        rel = r["rel"]
        segs = rel.split("/")
        if rel.startswith(STAFFAGE) or "/🧍️staffage/" in rel or rel.startswith("🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring"):
            pass
        for s in segs[:-1]:
            if not is_emoji_seg(s) and not s.startswith("."):
                deviations.append(("ascii-folder", rel, s))
        name = segs[-1]
        if name.lower().endswith(STRAY_EXT):
            stray.append((r["xy"], rel))
        if r["xy"] in ("??", "A ", "AM") and name:
            stem = name.split(".")[0]
            ext = name[len(stem) + 1:]
            stems.setdefault(ext, set()).add(stem)
            if stem and not is_emoji_seg(stem):
                deviations.append(("ascii-stem", rel, name))
            if ext not in EXPECTED_TEXT_EXT and not ext.endswith(("md", "json")):
                deviations.append(("unexpected-ext", rel, name))
            if "🧪️tests" in rel and "/" in rel:
                pass
            if r["xy"] == "??":
                for s in segs[:-1]:
                    if s.startswith("🧪") and s != "🧪️tests":
                        deviations.append(("tests-folder-variant", rel, s))
    ticket_files = []
    for dp, dn, fn in os.walk(os.path.join(ROOT, TICKET)):
        for f in fn:
            if f.lower().endswith(STRAY_EXT):
                ticket_files.append(os.path.relpath(os.path.join(dp, f), ROOT).replace(os.sep, "/"))
    data = {
        "count": len(rows),
        "by_status": {k: sum(1 for r in rows if r["xy"] == k) for k in ("??", " M", "M ", " D", "D ")},
        "max_utf16": rows[0]["utf16"] if rows else 0,
        "max_row": rows[0] if rows else None,
        "over256": over256,
        "over240": over240,
        "deviations": deviations,
        "stray_status": stray,
        "stray_ticket_disk": ticket_files,
        "ext_stems_sample": {k: sorted(v)[:12] for k, v in stems.items()},
        "ext_counts": {k: len(v) for k, v in stems.items()},
    }
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(data, fh, ensure_ascii=False, indent=1)
    print("count", data["count"], data["by_status"])
    print("max_utf16", data["max_utf16"], "over256", len(over256), "over240", len(over240))
    print("deviations", len(deviations), "stray_status", len(stray), "stray_ticket_disk", len(ticket_files))


if __name__ == "__main__":
    sys.exit(main())
