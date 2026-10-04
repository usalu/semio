#!/usr/bin/env python3
"""🔎️ Prints what the sample menagerie of the schema-conformance fixture says about moods, states, tricks and chemistry (work package A7)."""

import io
import json
import os
import sys

sys.stdout.reconfigure(encoding="utf-8")
TICKET = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(TICKET, "..", "..", "..", "..", "..", "..", ".."))
PATH = os.path.join(ROOT, "🧰️framework", "🛍️products", "🐾️pets", "🧫️fixtures", "🧬️schema-conformance", "🔣️.json")

document = json.load(io.open(PATH, encoding="utf-8"))
print("keys", list(document.keys()))
menagerie = document.get("menagerie") or {}
print("menagerie keys", list(menagerie.keys()))
for species in menagerie.get("species", []):
    print(species["id"], "mood", species.get("mood"), "size", species.get("size"), "temperament", species.get("temperament"))
    print("  repertoire", list(species.get("repertoire", {}).keys()))
    for state in species.get("states", []):
        print("  state", {key: value for key, value in state.items() if key not in ("name", "tint")})
    for trick in species.get("tricks", []):
        print("  trick", {key: value for key, value in trick.items() if key != "name"})
    print("  purr", species.get("purr"), "gear", species.get("gear"))
print("bonds", menagerie.get("bonds"))
print("chemistry", json.dumps(menagerie.get("chemistry"), ensure_ascii=False, indent=1))
