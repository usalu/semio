#!/usr/bin/env python3
# 🧵 Merge WP0 slice audits (A: ui, B: actor/kernel/value, C: rest framework/modules,
# D: repo/print/server/mit-bestand/root tests) into one ledger + stats for
# 📊️wp0-framework-modules.json and 📓️wp0-framework-modules.md.
import json
import collections

SCRATCH = "/private/tmp/claude-501/-Users-ueli-Documents-semio/cd1780e5-e2df-474c-a459-b3835d585ab6/scratchpad"
SLICES = {
    "A": f"{SCRATCH}/agentA_ui/findings.json",
    "B": f"{SCRATCH}/agentB_actor_kernel_value/findings.json",
    "C": f"{SCRATCH}/agentC_rest_modules/findings.json",
    "D": f"{SCRATCH}/agentD_products_mitbestand_tests/findings.json",
}

CANONICAL_KEYS = [
    "path", "blob", "role", "classification", "owner", "intendedOwner",
    "exports", "consumers", "decision", "evidence", "open", "slice",
]


def merged():
    out = []
    for slice_id, path in SLICES.items():
        data = json.load(open(path, encoding="utf-8"))
        for rec in data:
            rec = dict(rec)
            rec["slice"] = slice_id
            ordered = {k: rec[k] for k in CANONICAL_KEYS if k in rec}
            for k, v in rec.items():
                if k not in ordered:
                    ordered[k] = v
            out.append(ordered)
    return out


if __name__ == "__main__":
    records = merged()
    outpath = "📊️wp0-framework-modules.json"
    with open(outpath, "w", encoding="utf-8") as f:
        json.dump(records, f, ensure_ascii=False, indent=2)
        f.write("\n")

    paths = collections.Counter(r["path"] for r in records)
    dups = {p: c for p, c in paths.items() if c > 1}

    per_slice_class = collections.defaultdict(collections.Counter)
    for r in records:
        per_slice_class[r["slice"]][r["classification"]] += 1

    print("total records:", len(records))
    print("duplicate paths across slices:", len(dups))
    for p, c in dups.items():
        print(" ", c, p)
    print("per-slice classification counts:")
    for s in sorted(per_slice_class):
        print(" ", s, dict(sorted(per_slice_class[s].items())))
