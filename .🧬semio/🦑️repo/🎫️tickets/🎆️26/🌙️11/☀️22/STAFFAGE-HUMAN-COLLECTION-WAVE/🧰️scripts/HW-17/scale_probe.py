"""🧪️ Probe: scales standing-adult into the person-queuing folder unchanged otherwise and lists numeric keys the scaler left alone."""
import collections, copy, re, sys
from hw17lib import *
doc = load("standing-adult"); a = doc["asset"]
before = dict(numeric_paths(a))
a["id"] = "person-queuing"
scale_asset(a, float(sys.argv[1]), float(sys.argv[2]))
after = dict(numeric_paths(a))
same = collections.Counter()
for p, v in before.items():
    if after.get(p) == v and v != 0:
        same[re.sub(r"\[\d+\]", "[]", re.sub(r"\.(bones|regions|emitters)\.[a-z0-9-]+", r".\1.ID", p)).split(".")[-1]] += 1
print(dict(same))
write("person-queuing", doc)
