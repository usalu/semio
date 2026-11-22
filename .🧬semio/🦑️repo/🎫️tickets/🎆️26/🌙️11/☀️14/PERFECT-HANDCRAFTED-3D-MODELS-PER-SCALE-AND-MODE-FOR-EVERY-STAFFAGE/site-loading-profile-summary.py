"""🔬 Summarises a V8/JSC `.cpuprofile`: self time and inclusive time per function, in milliseconds.
Usage: python3 site-loading-profile-summary.py <file.cpuprofile> [rows]"""
import json, sys, collections, urllib.parse
profile = json.load(open(sys.argv[1]))
rows = int(sys.argv[2]) if len(sys.argv) > 2 else 25
nodes = {node["id"]: node for node in profile["nodes"]}
parent = {child: node["id"] for node in profile["nodes"] for child in node.get("children", [])}
deltas = profile.get("timeDeltas") or []
self_us = collections.Counter()
for sample, delta in zip(profile["samples"], deltas):
    self_us[sample] += delta
def name(node):
    frame = node["callFrame"]
    url = urllib.parse.unquote(frame.get("url", "")).split("staffage/")[-1].split("semio/")[-1]
    return f'{frame.get("functionName") or "(anonymous)"} {url}:{frame.get("lineNumber", 0) + 1}'
own = collections.Counter()
inclusive = collections.Counter()
for node_id, micros in self_us.items():
    own[name(nodes[node_id])] += micros
    seen = set()
    cursor = node_id
    while cursor is not None:
        label = name(nodes[cursor])
        if label not in seen:
            inclusive[label] += micros
            seen.add(label)
        cursor = parent.get(cursor)
total = sum(self_us.values())
print(f"total {total / 1000:.0f} ms")
print("-- self")
for label, micros in own.most_common(rows):
    print(f"{micros / 1000:8.0f} ms {100 * micros / total:5.1f}%  {label}")
print("-- inclusive")
for label, micros in inclusive.most_common(rows + 15):
    print(f"{micros / 1000:8.0f} ms {100 * micros / total:5.1f}%  {label}")
