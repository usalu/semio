"""🔣️ Sweeps every outline this repo ships under `🖼️assets/🔤️fonts` and reports which characters the
wgpu font atlas can NEVER resolve.

Parses each `🔤️outline.ttf`'s own `cmap` (format 4 and 12) with no third-party module, unions the
faces the atlas actually registers (`📝️text/🦀️.rs`'s `ANTA_LATIN` + `KELLY_SLAB_LATIN` +
`SHARE_TECH_MONO_LATIN` + the 12 `NOTO_EMOJI_BUCKETS`), then scans every Rust string literal under the
wgpu UI target and the os renderer's chrome elements for a codepoint that union does not carry.
A hit is a character the shell writes and the atlas paints as `.notdef`.

Run from the repo root: `python3 .🧬semio/…/WGPU-RENDERER-REACT-PARITY/🐍️w8a-font-coverage-sweep.py`
"""

import collections
import os
import re
import struct

FONTS = "🧰️framework/🔨️modules/🖼️assets/🔤️fonts"
REGISTERED = [
    "🚀️anta/🏛️latin",
    "🧱️kelly-slab/🏛️latin",
    "⌨️share-tech-mono/🏛️latin",
] + [
    f"😀️noto-emoji/{bucket}"
    for bucket in ["🌍️regions", "🚩️flags", "🔣️symbols", "🧰️objects", "🎯️activities", "🧳️travel", "🍽️food", "🌿️nature", "🧑️people", "😀️faces", "🔗️joined-forms", "🪉️supplement"]
]
ROOTS = [
    "🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu",
    "🧰️framework/🔨️modules/🖱️ui/🌐️locale-terminology",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements",
]


def cmap(path):
    data = open(path, "rb").read()
    tables = struct.unpack(">H", data[4:6])[0]
    offset = None
    for index in range(tables):
        entry = 12 + index * 16
        if data[entry : entry + 4] == b"cmap":
            offset = struct.unpack(">I", data[entry + 8 : entry + 12])[0]
    if offset is None:
        return set()
    best = None
    for index in range(struct.unpack(">H", data[offset + 2 : offset + 4])[0]):
        record = offset + 4 + index * 8
        subtable = offset + struct.unpack(">I", data[record + 4 : record + 8])[0]
        fmt = struct.unpack(">H", data[subtable : subtable + 2])[0]
        if fmt in (4, 12) and (best is None or fmt == 12):
            best = (fmt, subtable)
    if best is None:
        return set()
    fmt, base = best
    points = set()
    if fmt == 4:
        span = struct.unpack(">H", data[base + 6 : base + 8])[0]
        ends, starts, deltas, ranges = base + 14, base + 16 + span, base + 16 + span * 2, base + 16 + span * 3
        for segment in range(span // 2):
            end = struct.unpack(">H", data[ends + segment * 2 : ends + segment * 2 + 2])[0]
            start = struct.unpack(">H", data[starts + segment * 2 : starts + segment * 2 + 2])[0]
            delta = struct.unpack(">h", data[deltas + segment * 2 : deltas + segment * 2 + 2])[0]
            range_offset = struct.unpack(">H", data[ranges + segment * 2 : ranges + segment * 2 + 2])[0]
            if start == 0xFFFF:
                continue
            for point in range(start, end + 1):
                if range_offset == 0:
                    glyph = (point + delta) & 0xFFFF
                else:
                    cursor = ranges + segment * 2 + range_offset + (point - start) * 2
                    if cursor + 2 > len(data):
                        continue
                    glyph = struct.unpack(">H", data[cursor : cursor + 2])[0]
                    glyph = (glyph + delta) & 0xFFFF if glyph else 0
                if glyph:
                    points.add(point)
    else:
        for index in range(struct.unpack(">I", data[base + 12 : base + 16])[0]):
            group = base + 16 + index * 12
            start, end, _ = struct.unpack(">III", data[group : group + 12])
            points.update(range(start, min(end, start + 0x20000) + 1))
    return points


def main():
    union = set()
    for family in REGISTERED:
        union |= cmap(f"{FONTS}/{family}/📖️regular/🔤️outline.ttf")
    misses, sites = collections.Counter(), collections.defaultdict(list)
    for root in ROOTS:
        for folder, _, files in os.walk(root):
            if "🧪️tests" in folder or "🗑️generated" in folder:
                continue
            for name in files:
                if not name.endswith(".rs"):
                    continue
                path = os.path.join(folder, name)
                for line_number, line in enumerate(open(path, encoding="utf-8"), 1):
                    stripped = line.lstrip()
                    if stripped.startswith(("//", "*", "/*", "#[path")):
                        continue
                    for literal in re.finditer(r'"((?:[^"\\]|\\.)*)"', line):
                        for character in literal.group(1):
                            if ord(character) > 0x7F and ord(character) not in union:
                                misses[character] += 1
                                if len(sites[character]) < 2:
                                    sites[character].append(f"{path}:{line_number}")
    print(f"registered faces: {len(REGISTERED)}  codepoints: {len(union)}")
    for character, count in sorted(misses.items(), key=lambda row: -row[1]):
        print(f"U+{ord(character):04X} {character!r} x{count} {sites[character]}")
    print(f"total uncovered characters: {len(misses)}")


main()
