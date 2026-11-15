import json, os, sys, time
T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "validator")
os.makedirs(G, exist_ok=True)
results = json.load(open(os.path.join(T, "🗑️generated", "schema", "catalog", "results.json"), encoding="utf8"))
spliced = [r for r in results if r["spliced"]]
failing = [r for r in spliced if r["rows"] or r.get("error")]
clean = [r for r in spliced if not r["rows"] and not r.get("error")]
rules = {}
for r in failing:
    for row in r["rows"]:
        e = rules.setdefault(row["rule"], [set(), 0])
        e[0].add(r["id"])
        e[1] += 1
entry = {"time": time.strftime("%H:%M"), "assets": len(results), "spliced": len(spliced), "clean": len(clean), "failing": len(failing), "ms": sum(r["ms"] for r in spliced), "rules": {k: len(v[0]) for k, v in rules.items()}, "ready": sorted(r["id"] for r in clean)}
if len(sys.argv) > 1:
    entry["wallS"] = int(sys.argv[1])
open(os.path.join(G, "trend.jsonl"), "a", encoding="utf8").write(json.dumps(entry, ensure_ascii=False) + "\n")
rows = [json.loads(l) for l in open(os.path.join(G, "trend.jsonl"), encoding="utf8")]
names = sorted({k for r in rows for k in r["rules"]})
head = "| time | wall s | spliced | clean | failing | " + " | ".join(n.replace("volumes-", "") for n in names) + " |"
lines = ["## Trend", "", "Failing assets per rule after each run of `schema-catalog-check.ts --workers 3` (nice 15), machine under load.", "", head, "|" + "---|" * (5 + len(names) + 0) + "---|"[:0]]
for r in rows:
    lines.append(f"| {r['time']} | {r.get('wallS', '')} | {r['spliced']} | {r['clean']} | {r['failing']} | " + " | ".join(str(r["rules"].get(n, 0)) for n in names) + " |")
lines += ["", "## Ready", "", f"Spliced assets with no finding at any authored scale in the last run ({entry['time']}): {len(clean)}.", "", ", ".join(entry["ready"]) or "none", ""]
p = os.path.join(T, "validator.md")
s = open(p, encoding="utf8").read() if os.path.exists(p) else "# Validator maintainer report\n\n<!-- trend:begin -->\n<!-- trend:end -->\n"
a, b = s.index("<!-- trend:begin -->") + len("<!-- trend:begin -->"), s.index("<!-- trend:end -->")
open(p + ".tmp", "w", encoding="utf8").write(s[:a] + "\n" + "\n".join(lines) + "\n" + s[b:])
os.replace(p + ".tmp", p)
