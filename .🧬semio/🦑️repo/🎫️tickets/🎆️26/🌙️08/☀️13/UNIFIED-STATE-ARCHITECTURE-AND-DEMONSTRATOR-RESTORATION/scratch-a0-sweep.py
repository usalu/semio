#!/usr/bin/env python3
"""One-off wave-A0 sweep: collapse the 6-variant StateClass vocabulary onto the 4 canonical lanes
and lift `inferred` onto its own `derived` axis. Lives inside the ticket folder (never a permanent
script) per CLAUDE.md."""
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", "..", "..", ".."))
SKIP_DIRS = {".git", "node_modules", "target", "dist", ".nx", "__pycache__", ".🦑️repo"}

RUST = [
    ("#[state(persistent)]", "#[state(artifact)]"),
    ("#[state(local_ui)]", "#[state(config)]"),
    ("#[state(shared_ui)]", "#[state(presence)]"),
    ("#[state(preview)]", "#[state(artifact)]"),
    ("#[state(effect)]", "#[state(transient)]"),
    ("#[state(inferred)]", "#[derived]"),
]
JSON = [
    ('"x-semio-state": "persistent"', '"x-semio-state": "artifact"'),
    ('"x-semio-state": "local-ui"', '"x-semio-state": "config"'),
    ('"x-semio-state": "shared-ui"', '"x-semio-state": "presence"'),
    ('"x-semio-state": "preview"', '"x-semio-state": "artifact"'),
    ('"x-semio-state": "effect"', '"x-semio-state": "transient"'),
    ('"x-semio-state": "identity"', '"x-semio-state": "artifact"'),
    ('"x-semio-state": "inferred"', '"x-semio-derived": true'),
]
GRAPHQL = [
    ("@state(class: PERSISTENT)", "@state(class: ARTIFACT)"),
    ("@state(class: LOCAL_UI)", "@state(class: CONFIG)"),
    ("@state(class: SHARED_UI)", "@state(class: PRESENCE)"),
    ("@state(class: PREVIEW)", "@state(class: ARTIFACT)"),
    ("@state(class: EFFECT)", "@state(class: TRANSIENT)"),
    ("@state(class: INFERRED)", "@derived"),
]
PROTO = [
    ("// @state persistent", "// @state artifact"),
    ("// @state local-ui", "// @state config"),
    ("// @state shared-ui", "// @state presence"),
    ("// @state preview", "// @state artifact"),
    ("// @state effect", "// @state transient"),
    ("// @state inferred", "// @derived"),
]
TS = [
    ("@state persistent", "@state artifact"),
    ("@state local-ui", "@state config"),
    ("@state shared-ui", "@state presence"),
    ("@state preview", "@state artifact"),
    ("@state effect", "@state transient"),
    ("@state inferred", "@derived"),
]

# Rust and TypeScript leaves also embed the other four formats verbatim (doc strings, `include_str!`ed
# SDL, inline JSON Schema fixtures), so they get the union rule set.
EMBEDDED = TS + GRAPHQL + JSON
RULES = {
    ".rs": RUST + EMBEDDED,
    ".json": JSON,
    ".graphql": GRAPHQL,
    ".proto": PROTO,
    ".ts": EMBEDDED,
    ".tsx": EMBEDDED,
}

counts = {}
touched = []
for base, dirs, files in os.walk(ROOT):
    dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
    for name in files:
        ext = os.path.splitext(name)[1]
        rules = RULES.get(ext)
        if not rules:
            continue
        path = os.path.join(base, name)
        try:
            text = open(path, encoding="utf-8").read()
        except (UnicodeDecodeError, OSError):
            continue
        original = text
        for old, new in rules:
            n = text.count(old)
            if n:
                counts[old] = counts.get(old, 0) + n
                text = text.replace(old, new)
        if text != original:
            open(path, "w", encoding="utf-8").write(text)
            touched.append(os.path.relpath(path, ROOT))

print(f"files touched: {len(touched)}")
for key in sorted(counts, key=lambda k: -counts[k]):
    print(f"{counts[key]:7d}  {key}")
