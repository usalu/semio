"""🔗️ Independent model of the owners that only get their edges again, without inference: owners of real project roots (no tag) that are neither inferred again nor stale by root are refreshed when the environment moved, or when they are named stale (their stored edges are of another environment: always refreshed, never carried, also while the environment stays); split by whether they are projects or withheld; when the consumers of the edit are known, the projects that are no consumer keep their edges (carried). A world with a `heal` key is a stored owner table after a compute: the stale untagged owners get their edges again in the first round, tagged ones and whatever the first round could not repair are inferred, nothing is left after two rounds or on a cold build. Reads one world on stdin, prints the sorted lists."""
import json
import sys

world = json.load(sys.stdin)


def heal(world):
    heal = world["heal"]
    if heal["cold"] or heal["healed"] >= 2:
        return {"edges": [], "infer": []}
    stale = sorted(name for name, row in heal["owners"].items() if row["edgesEnv"] != heal["environment"])
    edges = [name for name in stale if not heal["owners"][name].get("tag")]
    tagged = [name for name in stale if heal["owners"][name].get("tag")]
    if heal["healed"] == 0:
        return {"edges": edges, "infer": tagged}
    return {"edges": [], "infer": sorted(edges + tagged)}


def refresh(world):
    forced = set(world.get("stale", []))
    moved = not world["cold"] and world["environment"] != world["previousEnvironment"]
    projects, withheld, carried = [], [], []
    consumers = world.get("consumers")
    if world["cold"] or not (moved or forced):
        return {"projects": [], "withheld": [], "carried": []}
    for name, stub in world["stubs"].items():
        if stub.get("tag") or name in world["inferred"] or stub["root"] in world["roots"]:
            continue
        if not (moved or name in forced):
            continue
        if stub["kind"] != "project":
            withheld.append(name)
        elif moved and consumers is not None and name not in consumers and name not in forced:
            carried.append(name)
        else:
            projects.append(name)
    return {"projects": sorted(projects), "withheld": sorted(withheld), "carried": sorted(carried)}


print(json.dumps(heal(world) if "heal" in world else refresh(world)))
