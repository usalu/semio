"""📏 Median total and worker-fetch time of successful remote units in an offload log window."""
import sys, json, re, time, statistics
path, minutes = sys.argv[1], float(sys.argv[2])
since = time.time() - minutes * 60
total, fetch, kinds = [], [], {}
for line in open(path, errors="replace").readlines()[-4000:]:
    try: r = json.loads(line)
    except Exception: continue
    if r.get("atMs", 0) / 1000 < since: continue
    k = r.get("reason") or r.get("kind"); kinds[k] = kinds.get(k, 0) + 1
    if r.get("kind") == "offload-run":
        total.append(r.get("milliseconds", 0) / 1000)
        m = re.search(r"worker fetch (\d+) ms", json.dumps(r))
        if m: fetch.append(int(m.group(1)) / 1000)
med = lambda v: round(statistics.median(v), 1) if v else None
print(time.strftime("%H:%M"), f"last {minutes:g} min:", kinds, "| runs", len(total), "median total s", med(total), "median fetch s", med(fetch))
