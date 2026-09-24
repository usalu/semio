"""🧾️ Aligns every owner manifest row's `outcomes` with the reviewed leaf table (the dispatch truth after the switch)
and measures, per row, which declared classes have no committed vector. `--write` rewrites manifests; plugin filters
as positional args. Region rule mirrors the bridges: an `any` subset whose leaves live in sibling subsets measures the
🪆️subsets root."""
import json, os, re, sys, collections
root = "/Users/ueli/Documents/semio/"
write = "--write" in sys.argv
only = [a for a in sys.argv[1:] if not a.startswith("--")]
table = json.load(open(root + ".tmp-ticket/wp-t10/leaf-outcomes.json"))
by_kind = collections.defaultdict(list)
for rel, classes in table.items():
    kind = json.load(open(root + rel + "/🔣️.json"))["semanticKind"]
    by_kind[kind].append((rel, classes))
ORDER = ["applied", "no-op", "empty", "disjoint", "rejected"]
stats, gaps, rewrites, unresolved = collections.Counter(), collections.Counter(), [], []
for base in ("✏️s", "🧰️framework"):
    for dp, ds, fs in os.walk(root + base):
        ds[:] = [d for d in ds if d not in ("node_modules", "target", "dist", "🗑️generated", ".git")]
        if not dp.endswith("🔮️oracles") or "🔣️.json" not in fs: continue
        rel_o = dp[len(root):]
        if only and not any(o in rel_o for o in only): continue
        path = dp + "/🔣️.json"
        text = open(path, encoding="utf-8").read()
        try: d = json.loads(text)
        except ValueError: continue
        if not isinstance(d, dict) or not d.get("mutationManifests"): continue
        owner = os.path.dirname(rel_o)
        region = owner if os.path.basename(os.path.dirname(owner)) != "🪆️subsets" else os.path.dirname(owner)
        dirty = False
        for m in d["mutationManifests"]:
            for row in m.get("mutations", []):
                cands = [c for r, c in by_kind.get(row["id"], []) if r.startswith(owner + "/")] or [c for r, c in by_kind.get(row["id"], []) if r.startswith(region + "/")]
                if not cands: unresolved.append(f"{rel_o}#{m['subset']}:{row['id']}"); continue
                want = [c for c in ORDER if c in set().union(*map(set, cands))]
                have = sorted(row.get("outcomes", []))
                plugin = "/".join(rel_o.split("/")[:3])
                if sorted(want) != have:
                    stats[(plugin, "changed")] += 1
                    rewrites.append((rel_o, m["subset"], row["id"], have, want))
                    row["outcomes"] = want
                    dirty = True
                else: stats[(plugin, "same")] += 1
        if dirty and write:
            indent = 1 if text.startswith('{\n "') else 2
            open(path, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=indent) + ("\n" if text.endswith("\n") else ""))
tot = collections.Counter()
for (p, k), v in stats.items(): tot[k] += v
print(dict(tot), "unresolved rows:", len(unresolved))
for p in sorted({p for p, _ in stats}): print(f"  {stats[(p, 'changed')]:4} changed {stats[(p, 'same')]:4} same  {p}")
json.dump({"rewrites": rewrites, "unresolved": unresolved}, open(root + ".tmp-ticket/wp-t12/generated/manifest-align.json", "w"), ensure_ascii=False, indent=1)
