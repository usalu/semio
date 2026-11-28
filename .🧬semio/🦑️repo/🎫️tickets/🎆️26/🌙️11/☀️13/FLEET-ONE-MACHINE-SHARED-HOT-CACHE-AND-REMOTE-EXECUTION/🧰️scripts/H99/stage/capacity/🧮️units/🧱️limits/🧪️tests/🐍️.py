#!/usr/bin/env python3
"""Independent model of the unit limit states: reads {policy, states} on stdin, prints the list of states."""
import json, sys

def state(policy, unit):
    time, mem = unit.get("milliseconds"), unit.get("megabytes")
    if time is None and mem is None: return "unmeasured"
    time, mem = time or 0, mem or 0
    d = policy["defaults"]
    row = next((c for c in policy["carving"] if c["crate"] == unit["crate"] and c["target"] == unit["target"]), None)
    max_t, max_m = (row["budgetMilliseconds"], row["budgetMegabytes"]) if row else (d["maximumMilliseconds"], d["maximumMegabytes"])
    if time > max_t or mem > max_m: return "over"
    if row: return "carving"
    w = d["warnBasisPoints"] / 10000
    return "near" if time >= max_t * w or mem >= max_m * w else "ok"

data = json.load(sys.stdin)
print(json.dumps([state(data["policy"], u) for u in data["states"]]))
