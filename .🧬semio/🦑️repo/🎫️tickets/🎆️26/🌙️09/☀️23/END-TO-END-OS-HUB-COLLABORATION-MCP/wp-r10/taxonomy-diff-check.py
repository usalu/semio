#!/usr/bin/env python3
"""⚖️ R10 T0: static proof that a candidate taxonomy only ADDS member names / member kinds to the live file, and that no
name sits in two member kinds sharing an owner kind (a new ambiguity anywhere in the repo).
usage: python3 taxonomy-diff-check.py <live.json> <candidate.json>"""
import json
import sys

live, cand = (json.load(open(path, encoding="utf-8")) for path in sys.argv[1:3])
problems = []
for key in set(live) | set(cand):
    if key != "semanticDirectoryMemberKinds" and live.get(key) != cand.get(key):
        problems.append(f"top-level {key} changed")
lk, ck = live["semanticDirectoryMemberKinds"], cand["semanticDirectoryMemberKinds"]
added = 0
for kind, spec in lk.items():
    other = ck.get(kind)
    if other is None:
        problems.append(f"member kind {kind} removed")
        continue
    if {k: v for k, v in spec.items() if k != "memberNames"} != {k: v for k, v in other.items() if k != "memberNames"}:
        problems.append(f"member kind {kind} fields changed")
    if other["memberNames"][: len(spec["memberNames"])] != spec["memberNames"]:
        problems.append(f"member kind {kind} existing names reordered/removed")
    added += len(other["memberNames"]) - len(spec["memberNames"])
created = [kind for kind in ck if kind not in lk]
added += sum(len(ck[kind]["memberNames"]) for kind in created)
owners = {}
for kind, spec in ck.items():
    for name in spec.get("memberNames", []):
        for owner in spec["ownerKindIds"]:
            owners.setdefault((name, owner), []).append(kind)
conflicts = {f"{name} @ {owner}": kinds for (name, owner), kinds in owners.items() if len(kinds) > 1}
live_conflicts = set()
for kind, spec in lk.items():
    for name in spec.get("memberNames", []):
        for owner in spec["ownerKindIds"]:
            live_conflicts.add((name, owner, kind))
new_conflicts = {key: kinds for key, kinds in conflicts.items() if any((key.split(" @ ")[0], key.split(" @ ")[1], kind) not in live_conflicts for kind in kinds)}
print(json.dumps({"namesAdded": added, "memberKindsCreated": created, "problems": problems, "sameOwnerConflictsNew": new_conflicts, "sameOwnerConflictsPreexisting": len(conflicts) - len(new_conflicts)}, ensure_ascii=False, indent=1))
sys.exit(1 if problems or new_conflicts else 0)
