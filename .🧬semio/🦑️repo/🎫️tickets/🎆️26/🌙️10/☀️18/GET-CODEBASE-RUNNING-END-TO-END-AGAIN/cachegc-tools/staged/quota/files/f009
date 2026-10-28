"""Independent oracle of the least-recently-used retirement of optional graph cache groups: groups without optional bytes are not candidates; candidates go oldest newest-write first (ties by name) while the cache holds more than the target; the caller's own groups, groups written within the protected idle time and groups whose writer is busy stay. Reads {"cases"} from the file in argv[1] and prints, per case, the sorted retired groups and the remaining bytes."""
import json
import sys


def retire(case):
    groups = sorted((g for g in case["groups"] if g["bytes"] > 0), key=lambda g: (g["newest"], g["group"]))
    remaining, retired = sum(g["bytes"] for g in groups), []
    for g in groups:
        if remaining <= case["targetBytes"]:
            break
        if g["group"] in case["exclude"] or case["now"] - g["newest"] < case["protectedMs"] or g["group"] in case["busy"]:
            continue
        retired.append(g["group"])
        remaining -= g["bytes"]
    return {"retired": sorted(retired), "remainingBytes": max(0, remaining)}


print(json.dumps([retire(case) for case in json.load(open(sys.argv[1], encoding="utf8"))["cases"]]))
