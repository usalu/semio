#!/usr/bin/env python3
"""➗️ W2-S-E mathematical parity: every `EquationMutation` leaf struct wires camelCase (`#[value(rename_all = "camelCase")]`),
the committed fixtures and the TS twin follow, and `replace-graph`'s `graph.algorithmSeed` (`Option<String>`) admits `null`.
The handcrafted text codec keeps its own kebab keys (`new-directed=`) and the binary codec is positional, so neither moves.

Idempotent; each file is re-read right before it is written. Usage: python3 🧪️w2-s-e-mathematical.py [--apply]
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
SUBSETS = "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets"
APPLY = "--apply" in sys.argv
RENAMED = {"new_directed": "newDirected", "new_algorithm": "newAlgorithm", "new_algorithm_seed": "newAlgorithmSeed", "new_label": "newLabel"}


def read(path):
    with open(os.path.join(REPO, path), encoding="utf-8") as handle:
        return handle.read()


def write(path, before, after):
    if before == after:
        return
    if APPLY:
        if read(path) != before:
            raise SystemExit(f"[w2-s-e] {path} changed while editing; rerun")
        with open(os.path.join(REPO, path), "w", encoding="utf-8") as handle:
            handle.write(after)
    print(f"[w2-s-e] {'wrote' if APPLY else 'would write'} {path}")


def leaf_files():
    for directory, _dirs, files in os.walk(os.path.join(REPO, SUBSETS)):
        relative = os.path.relpath(directory, REPO)
        parts = relative.split("/")
        if "🦀️.rs" in files and len(parts) >= 2 and parts[-2] == "🧬️mutations" and "✏️editor" not in parts and "🚪️io" not in parts and not parts[-1].startswith(("💾️", "📝️", "🧪️")):
            yield f"{relative}/🦀️.rs"


def rust():
    for path in sorted(leaf_files()):
        before = read(path)
        if "dsl::MutationLeaf" not in before:
            continue
        after = re.sub(r'(#\[mutation_leaf\(contract = ::protocol\)\]\n)(?!#\[value\(rename_all = "camelCase"\)\]\n)(pub struct )', r'\1#[value(rename_all = "camelCase")]\n\2', before, count=1)
        if after.count('#[value(rename_all = "camelCase")]') != 1:
            raise SystemExit(f"[w2-s-e] {path}: unexpected leaf attribute block")
        write(path, before, after)


def fixtures():
    for directory, _dirs, files in os.walk(os.path.join(REPO, SUBSETS)):
        if not directory.endswith("🦠️mutation") or "🔣️.json" not in files:
            continue
        path = os.path.relpath(os.path.join(directory, "🔣️.json"), REPO)
        before = read(path)
        wire = json.loads(before)
        if not isinstance(wire, dict) or len(wire) != 1:
            continue
        ((variant, payload),) = wire.items()
        if not isinstance(payload, dict):
            continue
        renamed = {variant: {RENAMED.get(key, key): value for key, value in payload.items()}}
        if renamed != wire:
            write(path, before, json.dumps(renamed, indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else ""))


def twin():
    path = f"{SUBSETS}/✳️any/🧬️schema/🧬️mutations/🟦️.ts"
    before = read(path)
    header_end = before.index("*/") + 2
    header = ("/** ➗️ EquationMutation — closed semantic mutation vocabulary for the equation document,\n"
              " *  mirrors `🧬️mutations/🦀️.rs`'s `EquationMutation` enum and its 15 per-verb leaf structs. The enum\n"
              " *  carries no `#[value(tag)]`, so it wires EXTERNALLY TAGGED: `{ \"<PascalCaseVariantName>\": { ...leaf\n"
              " *  fields } }`. Every leaf struct carries `#[value(rename_all = \"camelCase\")]`, so its fields wire\n"
              " *  camelCase (`{\"ChangeNodeLabel\":{\"id\":\"n-alpha\",\"newLabel\":\"Alpha\"}}`), like the referenced\n"
              " *  `EquationGraph`/`EquationPoint` records (`ReplaceGraph.graph.algorithmSeed`). */")
    after = header + before[header_end:]
    for snake, camel in RENAMED.items():
        after = re.sub(rf"\b{snake}:", f"{camel}:", after)
    write(path, before, after)


def schema():
    path = f"{SUBSETS}/🕸️graph/🧬️schema/🧬️mutations/🔁️replace-graph/🧬️schema/🔣️.json"
    before = read(path)
    document = json.loads(before)
    seed = document["properties"]["graph"]["properties"]["algorithmSeed"]
    if seed.get("type") == "string":
        seed["type"] = ["string", "null"]
    rendered = json.dumps(document, indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else "")
    if json.loads(before) != document and json.dumps(json.loads(before), indent=2, ensure_ascii=False) + ("\n" if before.endswith("\n") else "") != before:
        raise SystemExit(f"[w2-s-e] {path} is hand-formatted; edit it by hand")
    write(path, before, rendered)


if __name__ == "__main__":
    rust()
    fixtures()
    twin()
    schema()
