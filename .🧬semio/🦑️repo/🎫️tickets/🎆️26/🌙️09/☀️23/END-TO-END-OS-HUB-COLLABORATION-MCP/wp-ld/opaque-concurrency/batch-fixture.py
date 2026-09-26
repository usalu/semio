#!/usr/bin/env python3
"""🧮️ LD item 2: rewrites the document-backbone batch fixture + schema for the envelope's `observed`/`target`
fields (one-off, ticket-local). Every case whose bytes reach past `dependencies` gains `00 00` (no observation,
whole-artifact target) at that position; its limits gain the two target-segment limits; new cases pin the new
fields' canonical, limit and malformed outcomes.

usage: batch-fixture.py [--dry-run]
"""
import json
import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🔗️causal")
FIXTURE = ROOT / "🧫️fixtures/🧮️document-backbone-batch-v1/🔣️.json"
SCHEMA = ROOT / "🧬️schema/🧮️document-backbone-batch-v1/🔣️.json"


def varint(value):
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        out.append(byte | 0x80 if value else byte)
        if not value:
            return bytes(out)


def text(value):
    raw = value.encode()
    return varint(len(raw)) + raw


def dependencies_end(raw):
    position = 0

    def read():
        nonlocal position
        value, shift = 0, 0
        while True:
            byte = raw[position]
            position += 1
            value |= (byte & 0x7F) << shift
            shift += 7
            if not byte & 0x80:
                return value

    count = read()
    if count != 1:
        raise ValueError("fixture rewrite expects one-envelope vectors")
    for _ in range(3):
        length = read()
        position += length
    for _ in range(read()):
        length = read()
        position += length
    return position


def limits(case_limits):
    ordered = {}
    for key, value in case_limits.items():
        ordered[key] = value
        if key == "maximumTotalDependencies":
            ordered["maximumTargetSegmentsPerEnvelope"] = 8192
            ordered["maximumTotalTargetSegments"] = 8192
    return ordered


def envelope(expected, observed=None, target=()):
    out = {}
    for key, value in expected.items():
        out[key] = value
        if key == "dependencies":
            out["observed"] = observed
            out["target"] = list(target)
    return out


def compact(value):
    return json.dumps(value, ensure_ascii=False, separators=(", ", ": ")).replace("{", "{ ").replace("}", " }").replace("{  }", "{}")


def render(fixture):
    cases = []
    for case in fixture["cases"]:
        cases.append("    {\n" + ",\n".join(f'      "{key}": {compact(value)}' for key, value in case.items()) + "\n    }")
    head = ",\n".join(f'  "{key}": {compact(value)}' for key, value in fixture.items() if key != "cases")
    return "{\n" + head + ',\n  "cases": [\n' + ",\n".join(cases) + "\n  ]\n}\n"


def main():
    dry = "--dry-run" in sys.argv
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    base = None
    for case in fixture["cases"]:
        raw = bytes.fromhex(case["rawHex"])
        case["limits"] = limits(case["limits"])
        try:
            end = dependencies_end(raw)
        except (IndexError, ValueError):
            end = None
        if end is not None and end < len(raw):
            case["rawHex"] = (raw[:end] + b"\x00\x00" + raw[end:]).hex()
        if case["expect"]["outcome"] == "accepted":
            case["expect"]["envelopes"] = [envelope(item) for item in case["expect"]["envelopes"]]
        if case["id"] == "one-canonical":
            base = case
    head = bytes.fromhex("01016d0164016100")
    tail = bytes.fromhex("017301aa016902bbcc030405")
    observed_raw = head + b"\x01" + text("o") + varint(2) + text("tiles") + text("t-hero") + tail
    default_limits = dict(base["limits"])
    added = [
        {"id": "observed-and-target-canonical", "rawHex": observed_raw.hex(), "limits": default_limits, "expect": {"outcome": "accepted", "reason": "canonical", "envelopes": [envelope(base["expect"]["envelopes"][0], "o", ["tiles", "t-hero"])]}},
        {"id": "observed-flag-invalid", "rawHex": (head + b"\x02" + text("o") + b"\x00" + tail).hex(), "limits": default_limits, "expect": {"outcome": "malformed", "reason": "observed-flag"}},
        {"id": "target-segments-over-limit", "rawHex": observed_raw.hex(), "limits": {**default_limits, "maximumTargetSegmentsPerEnvelope": 1}, "expect": {"outcome": "limit", "reason": "target-segments"}},
        {"id": "total-target-segments-over-limit", "rawHex": observed_raw.hex(), "limits": {**default_limits, "maximumTotalTargetSegments": 1}, "expect": {"outcome": "limit", "reason": "target-segments"}},
        {"id": "observed-identifier-over-limit", "rawHex": (head + b"\x01" + text("oo") + b"\x00" + tail).hex(), "limits": {**default_limits, "maximumIdentifierBytes": 1}, "expect": {"outcome": "limit", "reason": "identifier-bytes"}},
    ]
    if any(case["id"] == added[0]["id"] for case in fixture["cases"]):
        raise SystemExit("fixture already rewritten")
    fixture["cases"].extend(added)
    schema_text = SCHEMA.read_text(encoding="utf-8")
    schema_edits = [
        ('        "maximumTotalDependencies",\n', '        "maximumTotalDependencies",\n        "maximumTargetSegmentsPerEnvelope",\n        "maximumTotalTargetSegments",\n'),
        ('        "maximumTotalDependencies": { "type": "integer", "minimum": 0, "maximum": 8192 },\n', '        "maximumTotalDependencies": { "type": "integer", "minimum": 0, "maximum": 8192 },\n        "maximumTargetSegmentsPerEnvelope": { "type": "integer", "minimum": 0, "maximum": 8192 },\n        "maximumTotalTargetSegments": { "type": "integer", "minimum": 0, "maximum": 8192 },\n'),
        ('"required": ["mutationId", "documentId", "actor", "dependencies", "diff", "inverse", "timestamp"],', '"required": ["mutationId", "documentId", "actor", "dependencies", "observed", "target", "diff", "inverse", "timestamp"],'),
        ('        "dependencies": { "type": "array", "items": { "type": "string" } },\n', '        "dependencies": { "type": "array", "items": { "type": "string" } },\n        "observed": { "type": ["string", "null"], "description": "Advisory, never an ordering constraint: the newest operation of another author the author\'s replica had applied." },\n        "target": { "type": "array", "items": { "type": "string" }, "description": "The structured address the operation writes, outermost segment first; empty is the whole artifact." },\n'),
    ]
    for before, after in schema_edits:
        if schema_text.count(before) != 1:
            raise SystemExit(f"schema anchor found {schema_text.count(before)} times: {before[:60]}")
        schema_text = schema_text.replace(before, after, 1)
    if not dry:
        FIXTURE.write_text(render(fixture), encoding="utf-8")
        SCHEMA.write_text(schema_text, encoding="utf-8")
    for case in fixture["cases"]:
        print(case["id"], case["expect"]["outcome"], case["expect"]["reason"], case["rawHex"][:60])


if __name__ == "__main__":
    main()
