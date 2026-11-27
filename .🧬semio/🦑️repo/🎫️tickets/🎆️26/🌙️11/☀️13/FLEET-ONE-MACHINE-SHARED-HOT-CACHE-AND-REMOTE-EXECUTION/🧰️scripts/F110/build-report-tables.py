import json, re, collections, sys
src, out = sys.argv[1], sys.argv[2]
rows = [json.loads(l) for l in open(src, encoding="utf-8") if l.strip()]
def area(p, n=3): return "/".join(p.split("/")[:n])
by = collections.defaultdict(list)
for r in rows: by[r["code"]].append(r)
L = []
L.append("| Code | Findings |\n|---|---:|")
for code, rs in sorted(by.items(), key=lambda kv: -len(kv[1])): L.append(f"| `{code}` | {len(rs)} |")
L.append(f"| **total** | **{len(rows)}** |\n")
for code, rs in sorted(by.items(), key=lambda kv: -len(kv[1])):
    L.append(f"### `{code}` ({len(rs)})\n")
    L.append("Top 10 areas (first 3 path segments):\n")
    L.append("| Findings | Area |\n|---:|---|")
    for a, n in collections.Counter(area(r["path"]) for r in rs).most_common(10): L.append(f"| {n} | `{a}` |")
    L.append("\nExamples:\n")
    seen = set(); k = 0
    for r in rs:
        if r["message"][:40] in seen: continue
        seen.add(r["message"][:40]); k += 1
        L.append(f"- `{r['path']}`: {r['message'][:300]}")
        if k == 3: break
    L.append("")
open(out, "w", encoding="utf-8").write("\n".join(L) + "\n")
print(len(L), "lines")
