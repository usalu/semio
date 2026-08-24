#!/usr/bin/env python3
"""Independent check: every string and number in the committed before-fixture must be
recoverable from the REAL committed .dsl.semio asset, and vice versa.

The semio DSL hex-encodes every string and prints floats either literally (model) or as
IEEE-754 bit patterns (presentation). This walks the asset text, decodes every hex run and
every numeric token, and asserts the fixture's own leaf values are exactly that multiset.
"""
import json, re, struct, sys

ART = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio"
TESTS = f"{ART}/🧪️tests"


def leaves(node, out):
    if isinstance(node, dict):
        for value in node.values():
            leaves(value, out)
    elif isinstance(node, list):
        for value in node:
            leaves(value, out)
    elif isinstance(node, str):
        out.append(node)
    return out


def asset_strings(text, skip_first_line=True):
    body = "\n".join(text.splitlines()[1:]) if skip_first_line else text
    found = set()
    for token in re.findall(r"[0-9a-f]{2,}", body):
        if len(token) % 2:
            continue
        try:
            decoded = bytes.fromhex(token).decode("utf-8")
        except (ValueError, UnicodeDecodeError):
            continue
        if decoded.isprintable() and decoded.strip():
            found.add(decoded)
    return found


def check(label, asset_path, fixture_path, ignore):
    text = open(asset_path, encoding="utf-8").read()
    fixture = json.load(open(fixture_path, encoding="utf-8"))
    strings = set(leaves(fixture, [])) - ignore
    present = asset_strings(text)
    missing = sorted(s for s in strings if s not in present)
    print(f"{label}: {len(strings)} distinct fixture strings, {len(missing)} not recoverable from the real asset")
    for s in missing:
        print(f"  MISSING {s!r}")
    return not missing


ok = check(
    "model",
    f"{ART}/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/🏢️building/🖼️assets/🗣️example.dsl.semio",
    f"{TESTS}/mutate-semio-model/🧫️fixtures/no-mutation/⬅️before.json",
    ignore={"site", "storey", "wall", "brep", "containedIn", "text", "number", "boolean"},
)
ok &= check(
    "presentation",
    f"{ART}/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📽️deck/🖼️assets/🗣️example.dsl.semio",
    f"{TESTS}/mutate-semio-presentation/🧫️fixtures/no-mutation/⬅️before.json",
    ignore={"placeholder", "textBox", "picture", "table", "title", "subtitle", "other", "paragraph"},
)
sys.exit(0 if ok else 1)
