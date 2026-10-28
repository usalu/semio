import sys, json, resource
path = sys.argv[1]
keys = sys.argv[2].split(',') if len(sys.argv) > 2 else None
for n, line in enumerate(open(path, encoding='utf-8', errors='replace'), 1):
    if resource.getrusage(resource.RUSAGE_SELF).ru_maxrss > 500 * 1024 * 1024:
        sys.exit('abort: rss > 500 MB')
    try:
        r = json.loads(line)
    except Exception:
        continue
    if keys:
        print(n, {k: r.get(k) for k in keys})
    else:
        print(n, sorted(r.keys())[:40], r.get('label'))
