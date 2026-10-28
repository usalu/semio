#!/usr/bin/env python3
"""🧮️ dashplan-tools: independent Python oracle of the Bazel slot order (stdlib only). A slot holds the package of its LAST selection only, so the order is: the slot whose last selection it is (most recent first), slots never used (slot number order), then every other used slot (least recently used first); ties keep the slot number order.
Usage: python3 -I rank_oracle.py check <fixture.json> | emit <fixture.json> (prints the fixture with the new rows and every expected order recomputed)."""
import json, sys


def order(slots, states, key):
    def rank(slot):
        state = states.get(slot)
        index = int(slot[1:])
        if state is None:
            return (1, 0, index)
        if state["key"] == key:
            return (0, -state["usedAt"], index)
        return (2, state["usedAt"], index)
    return sorted(slots, key=rank)


def state(slot, key, keys, used):
    return {"version": 1, "slot": slot, "key": key, "keys": keys, "usedAt": used}


A, B, C = "0000000000000001", "0000000000000002", "0000000000000003"
NEW = [
    {"name": "a slot that once served the key but holds another selection now is an ordinary used slot", "slots": ["s0", "s1", "s2", "s3"], "key": A,
     "states": {"s0": state("s0", B, [B, A], 100), "s2": state("s2", C, [C], 10)}},
    {"name": "two selections that once shared a slot do not pull each other back onto it", "slots": ["s0", "s1", "s2", "s3"], "key": B,
     "states": {"s0": state("s0", A, [A, B], 100), "s1": state("s1", C, [C, B], 200)}},
]
RENAMED = {"a slot that still keeps the package beats a never used slot": "a slot that only remembers the key is no better than any other used slot"}

if __name__ == "__main__":
    mode, path = sys.argv[1], sys.argv[2]
    fixture = json.load(open(path, encoding="utf8"))
    for row in fixture["ranks"]:
        row["name"] = RENAMED.get(row["name"], row["name"])
    names = {row["name"] for row in fixture["ranks"]}
    for row in NEW:
        if row["name"] not in names:
            fixture["ranks"].append(dict(row, expected=[]))
    for row in fixture["ranks"]:
        row["expected"] = order(row["slots"], row["states"], row["key"])
    if mode == "emit":
        sys.stdout.write(json.dumps(fixture, indent=1, ensure_ascii=False) + "\n")
    else:
        print("rows", len(fixture["ranks"]))
