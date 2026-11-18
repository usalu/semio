"""🧗 Greedy trace refit: for each spec item, trace it on top of the current best patch and keep it when the chosen metric improves. python3 humansb-A-greedy.py <asset> <scale> <view> <dilate> <base.json> <out.json> <metric i|h> <spec,spec,...>"""
import json, os, re, subprocess, sys
a, sc, view, dl, base, out, metric, specs = sys.argv[1:9]
N = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET"
B = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
V = view[0]
def score(path):
    r = subprocess.run([B, N + "/../STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/humans-try.ts", a, path, sc], capture_output=True, text=True, cwd="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript").stdout
    m = re.search(V + r":([\d.]+)/([\d.]+)mm", r)
    iou, h = float(m.group(1)), float(m.group(2))
    if metric == "c": return max(0.0, 0.92 - iou) * 1000 + h, iou, h
    return (-iou if metric == "i" else h), iou, h
cur = base
best = score(cur)
print("start", best, flush=True)
for item in specs.split(","):
    cand = out + ".cand.json"
    subprocess.run([N + "/humansb-A-trace.sh", a, sc, view, dl, item, cur, cand], check=True)
    s = score(cand)
    ok = s[0] < best[0]
    print(item, s, "keep" if ok else "drop", flush=True)
    if ok:
        best = s; cur = out + ".cur.json"; os.replace(cand, cur)
if cur != base: os.replace(cur, out)
else: open(out, "w").write(open(base).read())
print("final", best)
