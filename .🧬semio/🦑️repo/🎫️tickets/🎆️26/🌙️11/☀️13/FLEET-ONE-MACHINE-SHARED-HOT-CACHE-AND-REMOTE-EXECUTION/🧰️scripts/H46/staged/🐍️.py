"""🪂️ Independent Python oracle (standard library only) of backfill in host admission. A large request (`big`) waits beside running small requests while a stream of small requests arrives. Contract modelled, not the implementation: the large request is the holder of the line once it has waited `bypassMilliseconds` or six requests were admitted behind it; a holder that can be served once everything running has ended (it `drains`) lets a smaller request run ahead only while the holder has waited less than the backfill bound T and only while the request fits beside what the holder needs (the room once everything has ended, less the holder, less every request admitted since the holder arrived); T is twice the median measured wait of the running requests, at least `bypassMilliseconds`, at most `agingMilliseconds` (the shipped default of fewer than three measurements: `bypassMilliseconds`). `python3 🐍️.py` prints, for the fixture's wait and for a measured wait of zero (the old behaviour), the second the large request is admitted, how many requests ran ahead of it while it held the line and the most room ever used."""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = json.load(open(os.path.join(HERE, "🧫️fixtures", "🔣️.json"), encoding="utf-8"))
BASE = json.load(open(os.environ.get("BASE_FIXTURE") or os.path.join(HERE, "..", "🧫️fixtures", "🔣️.json"), encoding="utf-8"))["policy"]


def run(measured_wait):
    total = FIX["totalMegabytes"]
    m = BASE["memory"]
    floor = min(total, max(m["floorMinimumMegabytes"], min(m["floorMaximumMegabytes"], total * m["floorBasisPoints"] // 10000)))
    mem = FIX["memory"]
    reserve = min(total, max(mem["interactiveReserveMinimumMegabytes"], min(mem["interactiveReserveMaximumMegabytes"], total * mem["interactiveReserveBasisPoints"] // 10000)))
    share = (total - floor) - reserve
    wait = BASE["wait"]
    big, small, foreign = FIX["bigMegabytes"], FIX["smallMegabytes"], FIX["foreignMegabytes"]
    room = min(total - foreign - floor, share)
    bound = min(wait["agingMilliseconds"], max(wait["bypassMilliseconds"], 2 * measured_wait * 1000))
    live = [(end, -1) for end in FIX["initialEndSeconds"]]
    queue, ahead, most, held_since = [], 0, 0, None
    for t in range(0, FIX["horizonSeconds"], FIX["stepSeconds"]):
        live = [row for row in live if row[0] > t]
        if t % FIX["arrivalEverySeconds"] == 0:
            queue.append(t)
        used = small * len(live)
        if big <= room - used:
            return {"admittedAtSecond": t, "ranAhead": ahead, "mostUsed": most}
        since = len([row for row in live if row[1] >= 0])
        patient = t * 1000 >= wait["bypassMilliseconds"] or since >= wait["maximumBypasses"]
        blocked = patient
        bypasses = since
        extra = room - big - small * since
        soft = wait["bypassMilliseconds"] <= t * 1000 < bound
        while queue:
            if blocked and not (soft and small <= extra):
                break
            if small > room - used:
                break
            queue.pop(0)
            live.append((t + FIX["runSeconds"], t))
            used += small
            extra -= small
            if blocked:
                if t * 1000 >= wait["bypassMilliseconds"]:
                    ahead += 1
            else:
                bypasses += 1
                if bypasses >= wait["maximumBypasses"]:
                    blocked = True
        most = max(most, used)
    return {"admittedAtSecond": None, "ranAhead": ahead, "mostUsed": most}


if __name__ == "__main__":
    print(json.dumps({"backfill": run(FIX["measuredWaitSeconds"]), "strict": run(0)}))
