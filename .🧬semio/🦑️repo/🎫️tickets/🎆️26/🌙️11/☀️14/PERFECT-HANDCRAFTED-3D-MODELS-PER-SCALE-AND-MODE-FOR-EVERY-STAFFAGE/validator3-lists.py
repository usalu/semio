"""Writes the live lists of validator.md from the last catalog run: Trend (with the change against the previous run per rule), Ready, Thermal only, Geometry.
usage: python3 validator3-lists.py [wallSeconds] [--table-only]   (reads 🗑️generated/schema/catalog/results.json, appends 🗑️generated/validator3/trend.jsonl)"""
import json, os, re, sys, time

T = os.path.dirname(os.path.abspath(__file__))
G = os.path.join(T, "🗑️generated", "validator3")
os.makedirs(G, exist_ok=True)
results = json.load(open(os.path.join(T, "🗑️generated", "schema", "catalog", "results.json"), encoding="utf8"))
spliced = [r for r in results if r["spliced"]]
failing = [r for r in spliced if r["rows"] or r.get("error")]
clean = [r for r in spliced if not r["rows"] and not r.get("error")]
rules = {}
for r in failing:
    for row in r["rows"]:
        rules.setdefault(row["rule"], set()).add(r["id"])
entry = {"time": time.strftime("%H:%M"), "spliced": len(spliced), "clean": len(clean), "failing": len(failing), "rules": {k: len(v) for k, v in rules.items()}, "rows": sum(len(r["rows"]) for r in spliced)}
if len(sys.argv) > 1 and sys.argv[1].isdigit():
    entry["wallS"] = int(sys.argv[1])
trend = os.path.join(G, "trend.jsonl")
if "--table-only" not in sys.argv:
    open(trend, "a", encoding="utf8").write(json.dumps(entry) + "\n")
history = [json.loads(line) for line in open(trend, encoding="utf8")] if os.path.exists(trend) else [entry]
names = sorted({k for h in history for k in h["rules"]})
short = lambda n: n.replace("volumes-", "")
lines = ["## Trend (validator3)", "", "Failing assets per rule after each run (before -> after in the last two lines).", "", "| time | wall s | spliced | clean | failing | " + " | ".join(short(n) for n in names) + " |", "|" + "---|" * (5 + len(names)) + ""]
for h in history[-14:]:
    lines.append(f"| {h['time']} | {h.get('wallS', '')} | {h['spliced']} | {h['clean']} | {h['failing']} | " + " | ".join(str(h["rules"].get(n, 0)) for n in names) + " |")
if len(history) >= 2:
    a, b = history[-2], history[-1]
    delta = lambda x, y: f"{x} -> {y}" + ("" if x == y else f" ({y - x:+d})")
    lines += ["", f"Last run {b['time']} against {a['time']}: clean {delta(a['clean'], b['clean'])}, failing {delta(a['failing'], b['failing'])}; " + ", ".join(f"{short(n)} {delta(a['rules'].get(n, 0), b['rules'].get(n, 0))}" for n in names if a["rules"].get(n, 0) != b["rules"].get(n, 0)) + "."]
lines += ["", "## Ready", "", f"Clean at every authored scale in the run of {entry['time']}: {len(clean)}.", "", ", ".join(sorted(r["id"] for r in clean)) or "none", ""]

def thermal(r):
    out = []
    for row in r["rows"]:
        m = row["message"]
        w = row.get("worst") or {}
        name = w.get("name", "")
        region = name.split(":", 1)[1] if name.startswith("areaDeviation:") else ("all regions" if name == "areaDeviationTotal" else (re.search(r'region "([^"]+)"', m) or [None, name or "box"])[1])
        nums = re.search(r"is ([\d.]+) m\S+ against (?:the sum of )?areaM2 ([\d.]+)", m)
        out.append((row.get("scale"), region, float(nums[1]) if nums else None, float(nums[2]) if nums else None, w.get("value"), w.get("limit")))
    return out

only = [r for r in failing if r["rows"] and all(row["rule"] == "volumes-thermal-coverage" for row in r["rows"])]
geo = [r for r in failing if not (r["rows"] and all(row["rule"] == "volumes-thermal-coverage" for row in r["rows"]))]
worst = lambda r: max([row["worst"]["ratio"] for row in r["rows"] if row["rule"] != "volumes-thermal-coverage" and row.get("worst") and row["worst"]["ratio"] is not None and row["worst"]["ratio"] != float("inf")] + [0])
lines += ["## Thermal only", "", f"Assets whose only findings are `volumes-thermal-coverage` (the area reviewers' worklist), {len(only)}: region at scale, measured outer area against areaM2, deviation against limit.", ""]
for r in sorted(only, key=lambda r: r["id"]):
    parts = []
    for scale, region, got, want, value, limit in thermal(r):
        num = f"{got:.3g} vs {want:.3g} m2, " if got is not None else ""
        dev = f"{value * 100:.0f} % > {limit * 100:.0f} %" if value is not None and limit is not None and limit < 1e9 else "box"
        parts.append(f"{region} 1:{scale} ({num}{dev})")
    lines.append(f"- **{r['category']}/{r['id']}**: " + "; ".join(parts[:8]) + (f"; +{len(parts) - 8} more" if len(parts) > 8 else ""))
lines += ["", "## Geometry", "", f"Assets that fail any other rule (the authors' worklist), {len(geo)}, worst first (worst ratio of limit over value among the non-thermal findings; rules with the number of findings).", ""]
for r in sorted(geo, key=lambda r: -worst(r)):
    counts = {}
    for row in r["rows"]:
        counts[short(row["rule"])] = counts.get(short(row["rule"]), 0) + 1
    err = f" ERROR {r['error'][:80]}" if r.get("error") else ""
    lines.append(f"- **{r['category']}/{r['id']}** worst x{worst(r):.2f}: " + ", ".join(f"{k} {v}" for k, v in sorted(counts.items())) + err)
lines.append("")
p = os.path.join(T, "validator.md")
s = open(p, encoding="utf8").read()
a, b = "<!-- lists:begin -->", "<!-- lists:end -->"
block = a + "\n" + "\n".join(lines) + "\n" + b
if a in s:
    s = s[: s.index(a)] + block + s[s.index(b) + len(b):]
else:
    s = s.rstrip("\n") + "\n\n" + block + "\n"
open(p + ".tmp3", "w", encoding="utf8").write(s)
os.replace(p + ".tmp3", p)
print(entry)
