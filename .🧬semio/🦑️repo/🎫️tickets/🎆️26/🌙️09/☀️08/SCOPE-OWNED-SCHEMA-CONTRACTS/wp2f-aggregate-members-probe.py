#!/usr/bin/env python3
"""🧬️ Replicates root 📜️script.ts `policyStructuralMutationChildren` + `policyMutationAggregateMembers`
resolution on disk, so row 138's question can be answered per mutation root without paying for the
repo-wide source admission the `clean taxonomy verify --kind mutation` CLI performs (which races with
concurrent peer edits on this live tree). Reports, per root: which children are leaves, which are domain
directories, and whether every aggregate `oneOf` branch resolves to a real direct leaf payload schema."""
import json
import os
import posixpath
import sys

ROOT = os.getcwd()
TAX = json.load(open(os.path.join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), encoding="utf8"))
DESCRIPTOR = "🔣️.json"
AGGREGATE = "🔣️.json"


def owner_identity(root, owner_path):
    domains = TAX["mutationDomainOwners"].get(root)
    if domains is not None:
        parts = owner_path.split("/")
        if len(parts) != 2:
            return None
        domain, operation = parts
        return domains.get(domain, {}).get(operation)
    if "/" in owner_path:
        return None
    import re
    if not re.search(TAX["mutationDirectoryPattern"], owner_path):
        return None
    m = re.search(r"[a-z][a-z0-9]*(?:-[a-z0-9]+)+$", owner_path)
    return m.group(0) if m else None


def classify(root):
    domains = TAX["mutationDomainOwners"].get(root)
    facets = set(TAX["mutationBehaviorFacetDirs"]) | set(TAX["mutationOrganizationalFacetDirs"])
    leaves, domain_dirs, other = [], [], []
    for name in sorted(os.listdir(root)):
        path = os.path.join(root, name)
        if not os.path.isdir(path):
            continue
        if domains is not None and name in domains:
            domain_dirs.append(name)
            for verb in sorted(os.listdir(path)):
                if not os.path.isdir(os.path.join(path, verb)):
                    continue
                candidate = f"{name}/{verb}"
                (leaves if owner_identity(root, candidate) else other).append(candidate)
            continue
        if owner_identity(root, name):
            leaves.append(name)
        elif name in facets:
            pass
        else:
            other.append(name)
    return leaves, domain_dirs, other


def probe(root):
    leaves, domain_dirs, other = classify(root)
    by_path, by_id, required = {}, {}, []
    problems = []
    for leaf in leaves:
        descriptor_rel = f"{root}/{leaf}/{DESCRIPTOR}"
        if not os.path.isfile(os.path.join(ROOT, descriptor_rel)):
            problems.append(f"leaf {leaf} carries no {DESCRIPTOR} descriptor")
            continue
        descriptor = json.load(open(os.path.join(ROOT, descriptor_rel), encoding="utf8"))
        if "json-schema" not in descriptor.get("requiredLanguageSurfaces", []):
            continue
        required.append(leaf)
        payload_rel = f"{root}/{leaf}/{descriptor['payloadSchema'].split('#')[0]}"
        by_path[payload_rel] = leaf
        if os.path.isfile(os.path.join(ROOT, payload_rel)):
            document = json.load(open(os.path.join(ROOT, payload_rel), encoding="utf8"))
            if isinstance(document.get("$id"), str) and document["$id"]:
                by_id[document["$id"]] = leaf
        else:
            problems.append(f"leaf {leaf} payload {payload_rel} is absent")
    aggregate_rel = f"{root}/{AGGREGATE}"
    aggregate = json.load(open(os.path.join(ROOT, aggregate_rel), encoding="utf8"))
    branches = aggregate.get("oneOf")
    referenced = set()
    if not isinstance(branches, list):
        problems.append("aggregate declares no oneOf union")
        branches = []
    for index, member in enumerate(branches):
        ref = member.get("$ref") if isinstance(member, dict) else None
        if not isinstance(ref, str) or not ref:
            problems.append(f"oneOf[{index}] is not a $ref")
            continue
        target = ref.split("#")[0]
        leaf = by_id.get(target) or by_path.get(target if "://" in target else posixpath.normpath(f"{root}/{target}"))
        if leaf is None:
            problems.append(f"oneOf[{index}] references {ref!r}, not a direct leaf payload schema")
        else:
            referenced.add(leaf)
    for leaf in required:
        if leaf not in referenced:
            problems.append(f"aggregate omits the payload schema of direct leaf {leaf!r}")
    print(f"[probe] root {root}")
    print(f"[probe]   leaves={len(leaves)} (two-segment={sum(1 for leaf in leaves if '/' in leaf)}) domain-dirs={len(domain_dirs)} json-schema-required={len(required)}")
    print(f"[probe]   aggregate branches={len(branches)} resolved={len(referenced)} unresolved={len(branches) - len(referenced)}")
    print(f"[probe]   domain directories reported as leaves: {[d for d in domain_dirs if d in leaves]}")
    print(f"[probe]   unclassified children: {other}")
    for problem in problems:
        print(f"[probe]   PROBLEM {problem}")
    print(f"[probe]   problems={len(problems)}")
    return len(problems)


if __name__ == "__main__":
    roots = sys.argv[1:] or sorted(TAX["mutationDomainOwners"])
    sys.exit(1 if sum(probe(root) for root in roots) else 0)
