#!/usr/bin/env python3
"""Independent model of the Nx graph pool lock order: builds the wait-for graph of the roles (a blocking acquire while holding H adds an edge from every held resource) with graphlib, prints the cycle verdict and the holds-while-blocking verdict for one design. usage: oracle.py FIXTURE DESIGN"""
import graphlib, json, sys

design = json.load(open(sys.argv[1]))["designs"][sys.argv[2]]
edges, holds_while_blocking = {}, []
for role, steps in design.items():
    held = []
    for step in steps:
        if step[0] == "acquire":
            if step[2] == "wait":
                for resource in held:
                    edges.setdefault(step[1], set()).add(resource)
                if "pool" in held:
                    holds_while_blocking.append([role, step[1]])
            held.append(step[1])
        else:
            held.remove(step[1])
try:
    list(graphlib.TopologicalSorter(edges).static_order())
    cycle = False
except graphlib.CycleError:
    cycle = True
print(json.dumps({"cycle": cycle, "poolHoldersWaiting": holds_while_blocking}))
