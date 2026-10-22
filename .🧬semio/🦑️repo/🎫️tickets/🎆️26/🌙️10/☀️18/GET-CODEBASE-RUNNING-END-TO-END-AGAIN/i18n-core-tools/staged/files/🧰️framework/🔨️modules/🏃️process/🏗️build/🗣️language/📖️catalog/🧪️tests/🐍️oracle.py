#!/usr/bin/env python3
"""Independent oracle of the language catalog renderer: templates, rendering and catalog admission.

usage: oracle.py < corpus.json > result.json
It reads only the catalog schema and the corpus and shares no code with the TypeScript implementation; it renders with `string.Formatter`.
"""
import json
import os
import re
import string
import sys

SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧬️schema", "🔣️.json")
with open(SCHEMA, encoding="utf-8") as handle:
    CATALOG_SCHEMA = json.load(handle)

NAME = re.compile(r"[A-Za-z_][A-Za-z0-9_]*")
ID = re.compile(CATALOG_SCHEMA["properties"]["id"]["pattern"])
KEY = re.compile(CATALOG_SCHEMA["properties"]["messages"]["propertyNames"]["pattern"])
LANGUAGES = CATALOG_SCHEMA["properties"]["languages"]["const"]


def placeholders(template):
    names = []
    try:
        for _literal, field, spec, conversion in string.Formatter().parse(template):
            if field is None:
                continue
            if NAME.fullmatch(field) is None or spec or conversion:
                return None
            names.append(field)
    except ValueError:
        return None
    return sorted(set(names))


def render(catalog, case):
    language, key, values = case["language"], case["key"], case["values"]
    if not isinstance(language, str) or language == "":
        return {"refusal": "language-required"}
    if language not in catalog["languages"]:
        return {"refusal": "language-unavailable"}
    leaf = catalog["messages"].get(key)
    if leaf is None:
        return {"refusal": "unknown-key"}
    names = placeholders(leaf[language])
    if names is None:
        return {"refusal": "invalid-template"}
    if any(name not in names for name in values):
        return {"refusal": "unknown-placeholder"}
    if any(name not in values for name in names):
        return {"refusal": "missing-placeholder"}
    return {"text": leaf[language].format(**values)}


def valid(document):
    if not isinstance(document, dict) or set(document) - {"version", "id", "languages", "messages"}:
        return False
    version = document.get("version")
    if isinstance(version, bool) or version != 1:
        return False
    if "id" in document and (not isinstance(document["id"], str) or ID.fullmatch(document["id"]) is None):
        return False
    if document.get("languages") != LANGUAGES:
        return False
    messages = document.get("messages")
    if not isinstance(messages, dict) or not messages:
        return False
    for key, leaf in messages.items():
        if KEY.fullmatch(key) is None or not isinstance(leaf, dict) or set(leaf) != {"en", "de"}:
            return False
        if not isinstance(leaf["en"], str) or not isinstance(leaf["de"], str):
            return False
        english, german = placeholders(leaf["en"]), placeholders(leaf["de"])
        if english is None or german is None or english != german:
            return False
    return True


def main():
    corpus = json.load(sys.stdin)
    result = {
        "render": [{"name": case["name"], **render(corpus["catalog"], case)} for case in corpus["render"]],
        "templates": [{"template": case["template"], **({"placeholders": names} if (names := placeholders(case["template"])) is not None else {"refusal": "invalid-template"})} for case in corpus["templates"]],
        "documents": [{"name": case["name"], "valid": valid(case["document"])} for case in corpus["documents"]],
    }
    json.dump(result, sys.stdout, ensure_ascii=False)


if __name__ == "__main__":
    main()
