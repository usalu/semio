#!/usr/bin/env python3
"""🐍️ T3 helper: print a readonly TypeScript type alias for a fixture JSON sample.

Usage: python3 🐍️t3-fixture-types.py <TypeName> <fixture.json> [...]
Emits one `type <TypeName> = { ... }` per argument pair, structurally derived from the
sample document. Arrays are typed from their first element; heterogeneous unions are not
inferred, so always eyeball the result before pasting it into a test file.
"""
import json
import io
import sys


def merge(values):
    """🧬️ Collapses every element of one array into a single representative sample.

    Arrays in these fixtures are homogeneous vectors whose members differ only in which optional
    keys they carry and whether a nullable field is null in that row, so the element type is the
    union of the keys seen with the widest value seen per key."""
    values = [v for v in values]
    if any(isinstance(v, dict) for v in values):
        merged = {}
        for value in values:
            if not isinstance(value, dict):
                continue
            for key, item in value.items():
                merged[key] = item if key not in merged or merged[key] is None else merged[key]
        return merged
    if any(isinstance(v, list) for v in values):
        return [item for value in values if isinstance(value, list) for item in value]
    return next((v for v in values if v is not None), values[0] if values else None)


def scalar(values):
    """🔢️ The scalar union a set of samples needs: `null` widens whatever else appeared beside it."""
    kinds = []
    for value in values:
        if isinstance(value, bool):
            kind = "boolean"
        elif isinstance(value, (int, float)):
            kind = "number"
        elif value is None:
            kind = "null"
        else:
            kind = "string"
        if kind not in kinds:
            kinds.append(kind)
    return " | ".join(kinds)


def render(value, indent, siblings=None):
    pad = "  " * indent
    inner = "  " * (indent + 1)
    if isinstance(value, dict):
        if not value:
            return "Record<string, never>"
        peers = [s for s in (siblings or []) if isinstance(s, dict)]
        rows = ""
        for k, v in value.items():
            samples = [s[k] for s in peers if k in s] or [v]
            optional = "" if all(k in s for s in peers) or not peers else "?"
            key = k if k.isidentifier() else json.dumps(k)
            rows += f"{inner}readonly {key}{optional}: {render(merge(samples), indent + 1, samples)};\n"
        return "{\n" + rows + pad + "}"
    if isinstance(value, list):
        if not value:
            return "readonly never[]"
        return f"readonly {render(merge(value), indent, value)}[]"
    return scalar(siblings if siblings else [value])


def main(argv):
    if len(argv) < 3 or len(argv) % 2 == 0:
        raise SystemExit(__doc__)
    for index in range(1, len(argv), 2):
        name, path = argv[index], argv[index + 1]
        document = json.load(io.open(path, encoding="utf-8"))
        print(f"type {name} = {render(document, 0)};\n")


if __name__ == "__main__":
    main(sys.argv)
