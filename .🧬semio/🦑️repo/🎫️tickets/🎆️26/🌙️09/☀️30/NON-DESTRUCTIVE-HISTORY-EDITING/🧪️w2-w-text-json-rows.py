#!/usr/bin/env python3
"""🧾️ W2-W-text: rewrites the JSON RFC 8259 feature rows (`🧱️base`, `🛜️i-json`) from the hand-mapped shorthand into
the leaf wire payloads (`payload_value()`): `path` entries become `JsonPathSegment` (`{"kind":"key"|"index","value"}`),
every literal JSON value becomes the tagged `JsonValue` (`{"kind":"number","lexeme"}` …), `set-snapshot` carries a
whole `JsonSnapshot`, `set-top-level` a `JsonIJsonRoot`. `no-mutation` baseline scenarios are dropped. Idempotent
only on the shorthand input; run once.

Usage: python3 🧪️w2-w-text-json-rows.py
"""
import json
import re

REPO = "/Users/ueli/Documents/semio"
SUBSETS = f"{REPO}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets"
FEATURES = [f"{SUBSETS}/🧱️base/🧪️tests/🔀️mutate-json-rfc8259/🥒️.feature", f"{SUBSETS}/🛜️i-json/🧪️tests/🔀️mutate-json-rfc8259-i-json/🥒️.feature"]


def lexeme(number):
    return json.dumps(number)


def wire_value(value):
    if value is None:
        return {"kind": "null"}
    if isinstance(value, bool):
        return {"kind": "bool", "value": value}
    if isinstance(value, (int, float)):
        return {"kind": "number", "lexeme": lexeme(value)}
    if isinstance(value, str):
        return {"kind": "string", "value": value}
    if isinstance(value, list):
        return {"kind": "array", "items": [wire_value(item) for item in value]}
    return {"kind": "object", "members": [{"key": key, "value": wire_value(item)} for key, item in value.items()]}


def wire_path(path):
    return [{"kind": "index", "value": segment} if isinstance(segment, int) else {"kind": "key", "value": segment} for segment in path]


def wire_params(kind, params):
    out = {}
    if kind == "set-snapshot":
        return {"snapshot": {"schema": "stdio.json", "value": wire_value(params["value"])}}
    if kind == "set-top-level":
        root = wire_value(params["object"] if "object" in params else params["array"])
        return {"root": root}
    for key, value in params.items():
        if key == "path":
            out[key] = wire_path(value)
        elif key == "value" and kind != "set-string":
            out[key] = wire_value(value)
        else:
            out[key] = value
    return out


def rewrite(path):
    text = open(path, encoding="utf-8").read()
    row = re.compile(r"^(\s*\|\s*)([a-z-]+)(\s*\|\s*)(\{.*\})(\s*\|)\s*$", re.M)

    def replace(match):
        kind, params = match.group(2), json.loads(match.group(4))
        rendered = json.dumps(wire_params(kind, params), ensure_ascii=False)
        assert "|" not in rendered, rendered
        return f"{match.group(1)}{kind}{match.group(3)}{rendered} |"

    text = row.sub(replace, text)
    for tag in ("@id-no-mutation-baseline-mutate", "@id-no-mutation-baseline-inverse"):
        if f"  {tag}\n" in text:
            start = text.index(f"  {tag}\n")
            text = text[:start] + text[text.index("\n\n", start) + 2 :]
    assert "no-mutation" not in text, path
    open(path, "w", encoding="utf-8").write(text)
    print("rewrote", path)


for feature in FEATURES:
    rewrite(feature)
