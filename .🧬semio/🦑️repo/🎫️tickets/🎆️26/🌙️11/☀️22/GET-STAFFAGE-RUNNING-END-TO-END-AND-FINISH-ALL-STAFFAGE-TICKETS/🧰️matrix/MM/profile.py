"""📈️ Reads a V8 `.cpuprofile` (as `bun --cpu-prof` writes it) and prints the functions by self time and by inclusive time.

Usage: python3 profile.py <file.cpuprofile> [rows]
"""
import json
import sys
from collections import defaultdict


def main() -> None:
    profile = json.load(open(sys.argv[1], encoding="utf8"))
    rows = int(sys.argv[2]) if len(sys.argv) > 2 else 30
    nodes = {node["id"]: node for node in profile["nodes"]}
    parent = {child: node["id"] for node in profile["nodes"] for child in node.get("children", [])}
    name = lambda node: f'{node["callFrame"]["functionName"] or "(anonymous)"} {node["callFrame"]["url"].split("/")[-2:]}:{node["callFrame"]["lineNumber"]}'
    self_time = defaultdict(float)
    inclusive = defaultdict(float)
    deltas = profile.get("timeDeltas") or [1] * len(profile["samples"])
    for sample, delta in zip(profile["samples"], deltas):
        self_time[name(nodes[sample])] += delta
        seen = set()
        at = sample
        while at is not None:
            label = name(nodes[at])
            if label not in seen:
                inclusive[label] += delta
                seen.add(label)
            at = parent.get(at)
    total = sum(deltas)
    for title, table in (("self", self_time), ("inclusive", inclusive)):
        print(f"--- {title} (total {total / 1e6:.1f} s)")
        for label, value in sorted(table.items(), key=lambda item: -item[1])[:rows]:
            print(f"{100 * value / total:6.2f} % {value / 1e6:8.2f} s  {label}")


main()
