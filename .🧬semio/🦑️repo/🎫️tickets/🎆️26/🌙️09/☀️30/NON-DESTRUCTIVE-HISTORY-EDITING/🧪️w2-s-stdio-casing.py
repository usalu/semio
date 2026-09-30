#!/usr/bin/env python3
"""🐫️ W2-S stdio: renames the snake_case wire keys of the stdio enums that gained `rename_all_fields = "camelCase"` (and of
the cad leaf fields whose structs were already camelCase) in every fixture, feature table and test adapter that still spells
them snake_case. JSON keys are matched as `"key":`, adapter string literals as `"key"`; Rust identifiers stay untouched.
Feature tables whose rows changed are re-aligned. Idempotent; each file is re-read right before it is written.

    python3 🧪️w2-s-stdio-casing.py [--dry-run]
"""
import glob
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
ART = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
SEMIO = ART + "/🧿️semio/🏅️standards/🔖️v1/🪆️subsets"
AVI = {"bit_count": "bitCount", "size_image": "sizeImage", "x_pels_per_meter": "xPelsPerMeter", "y_pels_per_meter": "yPelsPerMeter", "colors_used": "colorsUsed", "colors_important": "colorsImportant", "format_tag": "formatTag", "samples_per_sec": "samplesPerSec", "avg_bytes_per_sec": "avgBytesPerSec", "block_align": "blockAlign", "bits_per_sample": "bitsPerSample"}
CAD = {"block_name": "blockName", "base_point": "basePoint", "color_index": "colorIndex", "line_type": "lineType", "start_angle": "startAngle", "end_angle": "endAngle", "major_axis_end": "majorAxisEnd", "start_param": "startParam", "end_param": "endParam", "insertion_point": "insertionPoint", "def_point": "defPoint", "text_position": "textPosition"}
MODEL = {"brep_id": "brepId", "mesh_id": "meshId"}
DRAWING = {"x_rotation": "xRotation", "large_arc": "largeArc"}
DOCUMENT = {"block_index": "blockIndex"}
PLAN = [
    (AVI, [ART + "/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧫️fixtures/**/*.json"]),
    (CAD, [SEMIO + "/📐️cad/🧫️fixtures/📐️mutate-semio-cad/**/*.json", SEMIO + "/📐️cad/🧪️tests/📐️mutate-semio-cad/🐍️.py", SEMIO + "/📐️cad/🧪️tests/📐️mutate-semio-cad/🥒️.feature"]),
    (MODEL, [SEMIO + "/🏛️model/🧫️fixtures/**/*.json", SEMIO + "/✉️base/🧫️fixtures/🏛️apply-model-applied/*.json", SEMIO + "/🏛️model/🧪️tests/🏛️mutate-semio-model/🐍️.py", SEMIO + "/🏛️model/🧪️tests/🏛️mutate-semio-model/🥒️.feature", SEMIO + "/🏛️model/🧪️tests/🏛️mutate-semio-model/🦀️.rs"]),
    (DRAWING, [SEMIO + "/🖊️drawing/🧫️fixtures/**/*.json", SEMIO + "/🖊️drawing/🧪️tests/🖊️mutate-semio-drawing/🐍️.py"]),
    (DOCUMENT, [SEMIO + "/📑️document/🧫️fixtures/**/*.json", SEMIO + "/📑️document/🧪️tests/📃️mutate-semio-document/🐍️.py", SEMIO + "/📑️document/🧪️tests/📃️mutate-semio-document/🥒️.feature"]),
]


def rename(text, mapping, path):
    """🔁️ Every `"old"` key (JSON: only when a `:` follows) spelled with its camelCase twin."""
    json_only = path.endswith(".json") or path.endswith(".feature")
    for old, new in mapping.items():
        text = re.sub(r'"%s"(?=\s*:)' % re.escape(old), '"%s"' % new, text) if json_only else text.replace('"%s"' % old, '"%s"' % new)
    return text


def realign(text, touched):
    """📐️ Pads every Gherkin table block holding a `touched` line to one column width per column."""
    lines, out, block = text.split("\n"), [], []

    def flush():
        if not block:
            return
        if not any(index in touched for index, _ in block):
            out.extend(line for _, line in block)
            block.clear()
            return
        rows = [[cell.strip() for cell in line.strip()[1:-1].split("|")] for _, line in block]
        widths = [max(len(row[index]) for row in rows if index < len(row)) for index in range(max(len(row) for row in rows))]
        first = block[0][1]
        indent = first[: len(first) - len(first.lstrip())]
        out.extend(indent + "| " + " | ".join(cell.ljust(widths[index]) for index, cell in enumerate(row)) + " |" for row in rows)
        block.clear()

    for index, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("|") and stripped.endswith("|"):
            block.append((index, line))
            continue
        flush()
        out.append(line)
    flush()
    return "\n".join(out)


def main():
    dry = "--dry-run" in sys.argv
    for mapping, patterns in PLAN:
        for pattern in patterns:
            for path in sorted(glob.glob(os.path.join(REPO, pattern), recursive=True)):
                text = open(path, encoding="utf-8").read()
                updated = rename(text, mapping, path)
                if updated == text:
                    continue
                if path.endswith(".feature"):
                    touched = {index for index, (a, b) in enumerate(zip(text.split("\n"), updated.split("\n"))) if a != b}
                    updated = realign(updated, touched) if touched else updated
                print(("[dry] " if dry else "") + os.path.relpath(path, REPO))
                if not dry:
                    if open(path, encoding="utf-8").read() != text:
                        print("  changed while scanning; rerun")
                        continue
                    open(path, "w", encoding="utf-8").write(updated)


if __name__ == "__main__":
    main()
