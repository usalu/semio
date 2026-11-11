#!/usr/bin/env python3
"""usage: nfs-outage-edit.py TREE PREFIX COUNT SECONDS -- create COUNT files nfs-outage/PREFIX-NN.txt in TREE evenly over SECONDS (and rewrite nfs-outage/shared.txt once per tree); prints the digest list as JSON."""
import hashlib, json, os, sys, time

tree, prefix, count, seconds = sys.argv[1], sys.argv[2], int(sys.argv[3]), float(sys.argv[4])
d = os.path.join(tree, "nfs-outage"); os.makedirs(d, exist_ok=True)
made = {}
for i in range(count):
    name = "%s-%02d.txt" % (prefix, i); data = ("outage %s %d %s\n" % (prefix, i, os.urandom(6).hex())).encode()
    with open(os.path.join(d, name), "wb") as f: f.write(data)
    made[name] = hashlib.sha256(data).hexdigest(); time.sleep(seconds / count)
print(json.dumps({"files": len(made), "t_end": time.time()}))
