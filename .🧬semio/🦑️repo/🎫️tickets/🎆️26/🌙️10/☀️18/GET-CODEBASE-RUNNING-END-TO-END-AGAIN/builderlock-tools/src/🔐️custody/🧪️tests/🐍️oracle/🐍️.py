"""🐍 Independent model of the slice custody admission: reads {"cases": [{mine, others}]} on stdin and prints {"decisions": [{admit, blockers}]}.

A claim conflicts with another when either is structural or their owner sets intersect; `mine` is admitted when no other claim
that is held or carries a smaller ticket conflicts with it; the blockers are those claims ordered by ticket."""
import json
import sys


def conflicts(a, b):
    return a["structural"] or b["structural"] or bool(set(a["owners"]) & set(b["owners"]))


def decide(mine, others):
    blockers = [o for o in others if o["token"] != mine["token"] and (o["state"] == "held" or o["ticket"] < mine["ticket"]) and conflicts(mine, o)]
    blockers.sort(key=lambda o: o["ticket"])
    return {"admit": not blockers, "blockers": [o["token"] for o in blockers]}


def main():
    cases = json.load(sys.stdin)["cases"]
    json.dump({"decisions": [decide(c["mine"], c["others"]) for c in cases]}, sys.stdout)


main()
