#!/usr/bin/env python3
"""🔍️ Dry-run analysis of missing-fixture local:// references under 🧿️semio test cases.

For every mutate-semio-<subset>/🥒️.feature, extract every local:// reference, check whether it
resolves against <case>/🧫️fixtures/<name>, and if not, try known kind-only-basename-migration
transforms to find the file that actually holds the data on disk. Reports what it would do; does
not write anything.
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
TESTS_ROOT = os.path.join(REPO, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests")

FIXTURE_RE = re.compile(r"\b(shared|local|asset)://([^\s\"'`,;)\]]+)")


def exists_rel(base, rel):
    return os.path.isfile(os.path.join(base, rel))


def candidates_for(name):
    """Yield candidate replacement names for an unresolved local:// name, most likely first."""
    # direct kind-only nesting: X.json -> X/🔣️.json
    if name.endswith(".json"):
        stem = name[: -len(".json")]
        yield f"{stem}/🔣️.json"
        # 🦠️<id>.json -> 🧫️<id>/🦠️mutation/🔣️.json  (Tier A combined-vector subsets)
        if stem.startswith("🦠️"):
            mid = stem[len("🦠️") :]
            yield f"🧫️{mid}/🦠️mutation/🔣️.json"
            yield f"{mid}/🦠️mutation/🔣️.json"
            yield "🦠️mutation/🔣️.json"  # root-level stray (observed for no-mutation)
        # <id>/⬅️before.json -> <id>/⬅️before/🔣️.json etc (handled by the generic stem+/🔣️.json rule above)
    # non-json single-file assets: name.ext -> name/🔣️.ext (rare; dxf, csv, dsl.semio, pack.semio, model.json)
    m = re.match(r"^(.*)\.([A-Za-z0-9.]+)$", name)
    if m and not name.endswith(".json"):
        stem, ext = m.group(1), m.group(2)
        yield f"{stem}/🔣️.{ext}"


def main():
    subsets = sorted(d for d in os.listdir(TESTS_ROOT) if os.path.isdir(os.path.join(TESTS_ROOT, d)))
    total_unresolved = 0
    total_fixed = 0
    total_unfixable = 0
    report = []
    for sub in subsets:
        case_dir = os.path.join(TESTS_ROOT, sub)
        feat_path = os.path.join(case_dir, "🥒️.feature")
        if not os.path.isfile(feat_path):
            continue
        text = open(feat_path, encoding="utf8").read()
        refs = sorted(set(f"{m.group(1)}://{m.group(2)}" for m in FIXTURE_RE.finditer(text) if m.group(1) == "local"))
        fixture_base = os.path.join(case_dir, "🧫️fixtures")
        for ref in refs:
            name = ref[len("local://") :]
            if exists_rel(fixture_base, name):
                continue
            total_unresolved += 1
            found = None
            for cand in candidates_for(name):
                if exists_rel(fixture_base, cand):
                    found = cand
                    break
            if found:
                total_fixed += 1
                report.append(("FIX", sub, ref, found))
            else:
                total_unfixable += 1
                report.append(("MANUAL", sub, ref, None))

    for kind, sub, ref, found in report:
        if kind == "FIX":
            print(f"FIX     {sub:14s} {ref}  ->  local://{found}")
        else:
            print(f"MANUAL  {sub:14s} {ref}")

    print()
    print(f"total unresolved local:// refs: {total_unresolved}")
    print(f"auto-fixable:                   {total_fixed}")
    print(f"needs manual look:               {total_unfixable}")


if __name__ == "__main__":
    main()
