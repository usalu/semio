"""🎯️ Census of committed mutation vectors: per leaf kind (within its subset root), which protocol classes the
committed vectors exercise once a warned `mutation.no-op` applied vector is read as `no-op`, and which classes the
reviewed leaf table requires but no vector exercises (the fixture classes to author)."""
import json, os, re, collections, sys
root = "/Users/ueli/Documents/semio/"
table = json.load(open(root + ".tmp-ticket/wp-t10/leaf-outcomes.json"))
files = [l.strip() for l in open(root + ".tmp-ticket/wp-t10/generated/outcome-files.txt") if l.strip()]
seen = collections.defaultdict(set); relabel = []
for f in files:
    parts = f.split("/")
    try: i = parts.index("🧬️mutations")
    except ValueError: continue
    kind = re.sub(r"^[^a-z0-9]+", "", parts[i + 1])
    sub = "/".join(parts[:parts.index("🪆️subsets") + 2]) if "🪆️subsets" in parts else "/".join(parts[:i])
    d = json.load(open(root + f))
    status = d.get("status")
    noop = any(m.get("code") == "mutation.no-op" for m in d.get("messages", []))
    cls = "no-op" if status == "applied" and noop else status
    if cls != status: relabel.append(f)
    seen[(sub, kind)].add(cls)
gaps = collections.Counter(); rows = []
for rel, classes in table.items():
    parts = rel.split("/")
    sub = "/".join(parts[:parts.index("🪆️subsets") + 2]) if "🪆️subsets" in parts else "/".join(parts[:parts.index("🧬️mutations")])
    kind = json.load(open(root + rel + "/🔣️.json"))["semanticKind"]
    have = seen.get((sub, kind), set())
    for c in classes:
        if c not in have:
            gaps[("/".join(parts[:3]), c)] += 1; rows.append((rel, c))
print("vectors relabelled applied→no-op:", len(relabel))
tot = collections.Counter()
for (p, c), v in gaps.items(): tot[c] += v
print("missing (leaf, class) vectors:", dict(tot))
for p in sorted({p for p, _ in gaps}): print(f"  {p}: " + ", ".join(f"{c}={gaps[(p, c)]}" for c in ('applied', 'no-op', 'rejected') if gaps[(p, c)]))
json.dump({"relabel": relabel, "missing": rows}, open(root + ".tmp-ticket/wp-t10/generated/vector-census.json", "w"), ensure_ascii=False, indent=1)
