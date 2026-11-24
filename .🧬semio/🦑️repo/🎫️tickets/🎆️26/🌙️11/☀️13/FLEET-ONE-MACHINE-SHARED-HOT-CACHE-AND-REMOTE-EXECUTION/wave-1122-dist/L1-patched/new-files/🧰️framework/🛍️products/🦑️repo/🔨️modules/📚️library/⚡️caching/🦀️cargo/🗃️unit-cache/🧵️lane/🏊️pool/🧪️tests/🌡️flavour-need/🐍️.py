"""Independent oracle of the flavour need and the pool trim with requested lanes and free-space pressure. Usage: python -I oracle.py <vectors.json>"""
import json, sys


def need(lanes):
    sums = {}
    for lane in lanes:
        if "flavour" in lane:
            sums[lane["flavour"]] = sums.get(lane["flavour"], 0) + lane["bytes"]
    return max(sums.values(), default=0)


def trim(case):
    lanes, bounds, requested = case["lanes"], case["bounds"], set(case["requested"])
    count, total, victims = len(lanes), sum(lane["bytes"] for lane in lanes), []
    allowed = [lane for lane in lanes if not lane["claimed"] and lane["id"] not in requested and (case["pressure"] or not lane.get("recent"))]
    for lane in sorted(allowed, key=lambda lane: (lane["lastUsedMs"], lane["id"])):
        if count <= bounds["maximumLanes"] and total <= bounds["quotaBytes"]:
            break
        victims.append(lane["id"])
        count -= 1
        total -= lane["bytes"]
    return victims


if __name__ == "__main__":
    corpus = json.load(open(sys.argv[1], encoding="utf-8"))
    print(json.dumps({"need": [need(case["lanes"]) for case in corpus["need"]], "trim": [trim(case) for case in corpus["trim"]]}))
