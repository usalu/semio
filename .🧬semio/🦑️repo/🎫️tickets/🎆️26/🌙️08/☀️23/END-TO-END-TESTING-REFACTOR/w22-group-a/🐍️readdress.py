#!/usr/bin/env python3
"""📝️ Rewrites one recorded no-oracle decision's rationale and its feature's opening paragraph, so a
refusal that was inherited from a falsified premise is replaced by one that argues from what actually
blocks the case today. Authoring aid; every text it writes is passed in."""
import collections, json, sys

registry, feature, decision_id, rationale_path, paragraph_path = sys.argv[1:6]

d = json.load(open(registry, encoding="utf-8"), object_pairs_hook=collections.OrderedDict)
found = [entry for entry in d.get("noOracleDecisions", []) if entry["id"] == decision_id]
if not found:
    raise SystemExit("registry %s holds no decision %r" % (registry, decision_id))
found[0]["rationale"] = open(rationale_path, encoding="utf-8").read().strip()
open(registry, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")

lines = open(feature, encoding="utf-8").read().split("\n")
head = next(i for i, l in enumerate(lines) if l.startswith("Feature:"))
at = head + 1
while at < len(lines) and lines[at].strip() == "":
    at += 1
end = at
while end < len(lines) and lines[end].strip() != "":
    end += 1
lines[at:end] = open(paragraph_path, encoding="utf-8").read().rstrip("\n").split("\n")
open(feature, "w", encoding="utf-8").write("\n".join(lines))
print("re-addressed %s :: %s" % (decision_id, feature.split("/")[-2]))
