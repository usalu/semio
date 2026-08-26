#!/usr/bin/env python3
"""🔁️ Turns one recorded no-oracle case into an oracle-backed one: swaps the registry entry and
retags the feature. Authoring aid; the rationale and the feature description are written by hand and
passed in as files."""
import json, collections, sys, os

registry, feature, oracle_id, capability, rationale_path, desc_path, title = sys.argv[1:8]

d = json.load(open(registry, encoding="utf-8"), object_pairs_hook=collections.OrderedDict)
old = [x["id"] for x in d.get("noOracleDecisions", [])]
d["oracles"] = [collections.OrderedDict([
    ("id", oracle_id), ("ecosystem", "python"), ("package", ""), ("version", ""),
    ("capabilities", [capability]), ("comparisonProfiles", ["ordered-json-v1"]),
    ("license", "AGPL-3.0-only"), ("testOnly", True),
    ("rationale", open(rationale_path, encoding="utf-8").read().strip()),
])]
d["noOracleDecisions"] = []
open(registry, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")

lines = open(feature, encoding="utf-8").read().split("\n")
tagline = next(i for i, l in enumerate(lines) if l.startswith("@no-oracle-"))
lines[tagline] = "@oracle-" + oracle_id
head = next(i for i, l in enumerate(lines) if l.startswith("Feature:"))
lines[head] = "Feature: " + title
end = next(i for i, l in enumerate(lines) if l.startswith("  @id-"))
lines[head + 1:end] = open(desc_path, encoding="utf-8").read().rstrip("\n").split("\n") + [""]
out = "\n".join(lines)
out = out.replace("  @mode-conformance\n  Scenario", "  @mode-differential\n  Scenario").replace("  @mode-property\n  Scenario", "  @mode-differential\n  Scenario")
open(feature, "w", encoding="utf-8").write(out)
print("converted %s: replaced no-oracle %r with oracle %r" % (os.path.basename(os.path.dirname(feature)), old, oracle_id))
