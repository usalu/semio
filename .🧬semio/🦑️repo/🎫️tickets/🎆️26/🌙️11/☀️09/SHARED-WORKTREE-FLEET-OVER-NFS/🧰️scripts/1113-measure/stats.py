import json, statistics, glob, os, csv
S = os.path.dirname(os.path.abspath(__file__))
def pct(vals, p):
    v = sorted(vals); k = (len(v) - 1) * p / 100.0
    lo = int(k); hi = min(lo + 1, len(v) - 1)
    return round(v[lo] + (v[hi] - v[lo]) * (k - lo), 1)
def summ(vals):
    return dict(n=len(vals), p50=pct(vals, 50), p95=pct(vals, 95), max=max(vals), min=min(vals))
rows = list(csv.DictReader(open(os.path.join(S, "samples.csv"))))
for col in ["daemon_cpu", "daemon_rss_kib", "supervisor_cpu", "supervisor_rss_kib"]:
    vals = [float(r[col.replace("daemon_cpu","daemon_cpu")]) for r in rows] if False else [float(r[col]) for r in rows]
    print(col, "min", min(vals), "median", statistics.median(vals), "max", max(vals), "n", len(vals))
for f in sorted(glob.glob(os.path.join(S, "probe-*.jsonl"))):
    recs = [json.loads(l) for l in open(f)]
    ok = [r for r in recs if not r["timeout"]]
    to = [r for r in recs if r["timeout"]]
    print(os.path.basename(f), "total", len(recs), "published", len(ok), "timeouts", len(to))
    if ok:
        print("  contains_ms", summ([r["contains_ms"] for r in ok]))
    hc = [r["hint_consumed_ms"] for r in ok if r["hint_consumed_ms"] is not None]
    if hc:
        print("  hint_consumed_ms", summ(hc))
