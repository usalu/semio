#!/usr/bin/env python3
"""🔎️ 5b B2 census: for every wrapper that still takes `Option<serde_json::Value>` action args, classify each caller's
argument (None / `Some(json!(literal))` / variable / other) so the B2 codemod rewrites the literal ones and lists the rest.
usage: b2-census.py [--root <tree>]"""
import re
import subprocess
import sys
from collections import Counter
from pathlib import Path

LIVE = Path("/Users/ueli/Documents/semio")
ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else LIVE
WRAPPERS = {"generation3d_action": 1, "generation3d_view_action": 1, "gis2d_window_action": 1, "lowpoly_window_action": 1, "layout_context_menu_item": 4, "note_action": 1, "puzzle2d_action": 1, "item": 4, "scene_action": 2, "canvas_addressed_action": 3, "map_action": 2, "board_action": 2, "action": 1}


def split_args(text, start):
    depth, index, parts, begin = 1, start, [], start
    while index < len(text) and depth:
        char = text[index]
        if char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
            if depth == 0:
                parts.append(text[begin:index])
                break
        elif char == "," and depth == 1:
            parts.append(text[begin:index])
            begin = index + 1
        elif char == '"':
            index += 1
            while index < len(text) and text[index] != '"':
                index += 2 if text[index] == "\\" else 1
        index += 1
    return parts


def classify(arg):
    arg = arg.strip()
    if arg == "None":
        return "none"
    if re.fullmatch(r"Some\(\s*(?:serde_json::|pack::)?json!\(.*\)\s*\)", arg, re.S):
        return "literal"
    return "other"


if __name__ == "__main__":
    only = sys.argv[sys.argv.index("--only") + 1].split(",") if "--only" in sys.argv else [name for name in WRAPPERS if name not in ("item", "action")]
    for name in only:
        index = WRAPPERS[name]
        files = subprocess.run(["git", "grep", "-l", "-w", name, "--", "✏️s/*.rs", "🧰️framework/*.rs"], cwd=LIVE, capture_output=True, text=True).stdout.split()
        counts, others = Counter(), []
        for rel in files:
            text = (ROOT / rel).read_text(encoding="utf-8")
            for match in re.finditer(rf"(?<![A-Za-z0-9_]){name}\(", text):
                head = text[max(0, match.start() - 3):match.start()]
                if head.endswith("fn "):
                    continue
                parts = split_args(text, match.end())
                if len(parts) <= index:
                    counts["short"] += 1
                    continue
                kind = classify(parts[index])
                counts[kind] += 1
                if kind == "other":
                    others.append(f"{rel}:{text.count(chr(10), 0, match.start()) + 1}: {parts[index].strip()[:100]}")
        print(f"{name}: {dict(counts)}")
        for row in others:
            print("   ", row)
