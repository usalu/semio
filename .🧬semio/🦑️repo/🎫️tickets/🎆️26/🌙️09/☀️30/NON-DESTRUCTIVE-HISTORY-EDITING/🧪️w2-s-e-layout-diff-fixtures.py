#!/usr/bin/env python3
"""🔺️ W2-S-E: brings the committed layout `🔺️diff/🔣️.json` fixtures up to the current `LayoutDiff` wire. The diff patch
structs gained members (page `frame_layer`/`frame_order`/`guides`/`layer_*`/`overrides`/`parent_page_id`, frame
`inset_*`/`story_id`/`thread_next`, link and story patch members) when the 17 later leaves landed, so every quintet's
`produces_committed_diff`/`committed_diff_is_canonical` test fails on the missing `null` members. Reads the produced diff
(`left`) of each failing test from a `cargo test` log (serde_json `Debug` form), inserts exactly the members the committed
file lacks (keeping its own key order), and refuses to write unless the result equals the produced diff.

Usage: python3 🧪️w2-s-e-layout-diff-fixtures.py <cargo-test.log> [--apply]
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
SUBSET = f"{REPO}/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any"


def parse_debug(text):
    """serde_json::Value Debug → JSON text."""
    out, index = [], 0
    while index < len(text):
        for token, replacement in (("Object {", "{"), ("Array [", "["), ("Null", "null"), ("Bool(true)", "true"), ("Bool(false)", "false")):
            if text.startswith(token, index):
                out.append(replacement)
                index += len(token)
                break
        else:
            if text.startswith("String(", index):
                end = index + 7
                literal, end = json.JSONDecoder().raw_decode(text, end)
                out.append(json.dumps(literal, ensure_ascii=False))
                index = end + 1
            elif text.startswith("Number(", index):
                end = text.index(")", index)
                out.append(text[index + 7:end])
                index = end + 1
            else:
                out.append(text[index])
                index += 1
    return json.loads("".join(out))


def merge(committed, produced):
    if isinstance(committed, dict) and isinstance(produced, dict):
        merged = {key: merge(value, produced[key]) if key in produced else value for key, value in committed.items()}
        for key in sorted(produced):
            if key not in merged:
                merged[key] = produced[key]
        return merged
    if isinstance(committed, list) and isinstance(produced, list) and len(committed) == len(produced):
        return [merge(left, right) for left, right in zip(committed, produced)]
    return committed


def main():
    log, apply = sys.argv[1], "--apply" in sys.argv
    lines = open(log, encoding="utf-8").read().splitlines()
    done = set()
    for index, line in enumerate(lines):
        match = re.search(r"panicked at (.*?🧬️mutations/([^/]+)/🧪️tests/([^/]+)/🦀️\.rs)", line)
        if match is None or not lines[index + 2].startswith("  left: ") or "diff" not in lines[index + 1]:
            continue
        leaf, case = match.group(2), match.group(3)
        path = f"{SUBSET}/🧫️fixtures/🧬️mutations/{leaf}/{case}/🔺️diff/🔣️.json"
        if path in done:
            continue
        if not os.path.exists(path):
            candidates = [entry for entry in os.listdir(f"{SUBSET}/🧫️fixtures/🧬️mutations/{leaf}") if entry.endswith(case.lstrip("🔤️🎨️🖊️🔗️📐️🌀️🔒️📍️🏷️📝️🚫️↔️↕️🔲️🏛️"))]
            if len(candidates) != 1:
                print(f"[w2-s-e] no fixture for {leaf}/{case}")
                continue
            path = f"{SUBSET}/🧫️fixtures/🧬️mutations/{leaf}/{candidates[0]}/🔺️diff/🔣️.json"
        produced = parse_debug(lines[index + 2][len("  left: "):])
        text = open(path, encoding="utf-8").read()
        committed = json.loads(text)
        merged = merge(committed, produced)
        if json.dumps(merged, sort_keys=True) != json.dumps(produced, sort_keys=True):
            print(f"[w2-s-e] MISMATCH beyond missing members {path}")
            continue
        done.add(path)
        if merged == committed:
            continue
        if apply:
            open(path, "w", encoding="utf-8").write(json.dumps(merged, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else ""))
        print(f"[w2-s-e] {'wrote' if apply else 'would write'} {os.path.relpath(path, SUBSET)}")


if __name__ == "__main__":
    main()
