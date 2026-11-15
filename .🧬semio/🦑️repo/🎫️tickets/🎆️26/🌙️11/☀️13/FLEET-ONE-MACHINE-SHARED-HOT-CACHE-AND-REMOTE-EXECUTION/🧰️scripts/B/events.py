import sys, json
for line in open(sys.argv[1]):
    r = json.loads(line)
    print(r["kind"], r["crate"], (r["key"] or "")[:12], (r.get("reason") or "")[:int(sys.argv[2]) if len(sys.argv) > 2 else 100])
