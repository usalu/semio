"""🐍️ Independent oracle of the lock-order class of the Nx put gate and put slots: reads the authored lease order policy and classifies every given resource name by the longest declared prefix (an exact name first), without sharing a line with the implementation; prints {resource: [class id, rank]}. usage: 🐍️order.py <policy.json> <resource>..."""
import json
import sys


def main():
    classes = json.load(open(sys.argv[1], encoding="utf-8"))["classes"]
    exact = {m["value"]: c for c in classes for m in c["match"] if m["kind"] == "exact"}
    prefixes = sorted(((m["value"], c) for c in classes for m in c["match"] if m["kind"] == "prefix"), key=lambda item: -len(item[0]))
    out = {}
    for resource in sys.argv[2:]:
        row = exact.get(resource) or next((c for value, c in prefixes if resource.startswith(value)), None)
        out[resource] = [row["id"], row["rank"]] if row else ["unclassified", 0]
    print(json.dumps(out, ensure_ascii=False))


main()
