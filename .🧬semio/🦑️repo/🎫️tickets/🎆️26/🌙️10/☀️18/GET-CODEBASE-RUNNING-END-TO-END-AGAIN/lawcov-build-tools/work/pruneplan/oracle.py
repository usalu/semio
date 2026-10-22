"""🧹️ Independent Python oracle of the cache prune plan and of the binary size formatter (standard library only): age deletions first, then oldest-first budget eviction that never touches a held unit nor a unit inside the guard age, and an exact round-half-up decimal rendering of byte counts. Requests arrive as JSON on stdin."""
import json, sys
from fractions import Fraction


def plan(areas, now, shared_guard):
    result = []
    for area in areas:
        units = area["units"]
        total = sum(u["bytes"] for u in units)
        cutoff = now - area["unusedAgeMs"]
        by_age = [u for u in units if not u["lockHeld"] and u["recencyMs"] <= cutoff]
        gone = {u["path"] for u in by_age}
        rest = [u for u in units if u["path"] not in gone]
        remaining = sum(u["bytes"] for u in rest)
        by_budget, guarded = [], 0
        if area["budgetBytes"] is not None and remaining > area["budgetBytes"]:
            guard_cutoff = now - area.get("guardAgeMs", shared_guard)
            for u in sorted(rest, key=lambda u: (u["recencyMs"], u["path"])):
                if remaining <= area["budgetBytes"]:
                    break
                if u["lockHeld"] or u["recencyMs"] > guard_cutoff:
                    guarded += u["bytes"]
                    continue
                by_budget.append(u)
                remaining -= u["bytes"]
        deleted = sum(u["bytes"] for u in by_age) + sum(u["bytes"] for u in by_budget)
        result.append({"name": area["name"], "totalBytes": total, "unitCount": len(units), "age": [u["path"] for u in by_age], "budget": [u["path"] for u in by_budget], "retainedBytes": total - deleted, "guarded": guarded})
    return result


def fixed(value):
    scaled = value * 100
    integer = scaled.numerator // scaled.denominator
    if scaled - integer >= Fraction(1, 2):
        integer += 1
    return "%d.%02d" % (integer // 100, integer % 100)


def fmt(count):
    if count < 1024:
        return "%d B" % count
    names = ["KiB", "MiB", "GiB", "TiB"]
    value, index = Fraction(count), -1
    while True:
        value /= 1024
        index += 1
        if not (value >= 1024 and index < len(names) - 1):
            break
    return fixed(value) + " " + names[index]


request = json.load(sys.stdin)
if request["op"] == "plan":
    print(json.dumps([plan(row["areas"], request["nowMs"], request["guardAgeMs"]) for row in request["rows"]]))
else:
    print(json.dumps([fmt(count) for count in request["bytes"]]))
