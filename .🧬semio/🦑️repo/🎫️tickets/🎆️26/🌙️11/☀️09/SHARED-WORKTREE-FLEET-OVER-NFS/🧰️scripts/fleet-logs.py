#!/usr/bin/env python3
"""usage: fleet-logs.py LOGDIR -- counts the NDJSON events of every daemon/relay log (sum over files, plus the 5 largest files per event), prints the first message of every distinct error."""
import collections, glob, json, os, sys
tot = collections.Counter(); per = collections.defaultdict(collections.Counter); errs = collections.OrderedDict()
for f in glob.glob(sys.argv[1] + "/*.out"):
    n = os.path.basename(f)[:-4]
    for line in open(f, errors="replace"):
        if not line.startswith("{"): continue
        try: e = json.loads(line)
        except ValueError: continue
        k = e.get("event")
        if k in ("progress", "published", "applied", "scan"): 
            if k in ("published", "applied"): tot[k + "-events"] += 1; tot[k + "-files"] += e.get("count", 0)
            continue
        tot[k] += 1; per[k][n] += 1
        if k in ("error", "blocked", "unstable", "rejoin-required", "blob-missing", "clock-refused", "clock-skew"): errs.setdefault((k, e.get("message") or e.get("reason") or ""), (n, e))
print(json.dumps(dict(tot)))
for k, c in per.items(): print(k, c.most_common(4))
for (k, m), (n, e) in list(errs.items())[:25]: print(k, n, json.dumps(e)[:300])
