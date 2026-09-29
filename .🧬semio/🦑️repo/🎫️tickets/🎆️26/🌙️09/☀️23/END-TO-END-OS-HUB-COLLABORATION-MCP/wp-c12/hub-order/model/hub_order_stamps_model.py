"""🧮️ C12: Python model of the EFFECTIVE-STAMP rule on top of the hub-order placement (`hub_order_model.py`): every replica stamps
each decided log entry `eff = next_after(frontier, sent)` and re-stamps its undecided entries past every placed one, so the history
fold's (HLC, id) order equals the placement (= hub) order after EVERY action — the property the Rust `reproject()` relies on.
usage: python3 hub_order_stamps_model.py [seed] [--trace]"""
import random
import sys

from hub_order_model import rng

TIE = 3


def key(stamp):
    return (stamp[0], stamp[1], stamp[2])


def next_after(frontier, stamp):
    if frontier is None or key(stamp) > key(frontier):
        return stamp
    return (frontier[0], frontier[1] + 1, stamp[2])


class Replica:
    def __init__(self, index, actor):
        self.index, self.actor = index, actor
        self.applied, self.ledger, self.out, self.inbound, self.cut = [], [], [], [], False
        self.stamp, self.sent, self.clock, self.frontier = {}, {}, (0, 0, actor), None

    def tick(self, now):
        physical, logical, _ = self.clock
        self.clock = (now, 0, self.actor) if now > physical else (physical, logical + 1, self.actor)
        return self.clock

    def merge(self, stamp):
        if key(stamp) > key(self.clock):
            self.clock = (stamp[0], stamp[1], self.actor)

    def ops(self):
        return [op for edit in self.applied for op in edit["ops"]]

    def restamp_undecided(self):
        previous = self.frontier
        for op in self.ledger:
            stamp = next_after(previous, self.stamp[op])
            if stamp == self.stamp[op]:
                break
            self.stamp[op] = previous = stamp
            self.merge(stamp)

    def rebase_point(self):
        point = None
        for index in range(len(self.applied) - 1, -1, -1):
            hits = [i for i, op in enumerate(self.applied[index]["ops"]) if op in self.ledger]
            if not hits:
                break
            point = (index, hits[0])
            if hits[0] > 0:
                break
        return point

    def fold(self, message):
        kind, op, sent = message
        if kind == "committed":
            self.frontier = next_after(self.frontier, self.sent[op])
            self.stamp[op] = self.frontier
            self.ledger.remove(op)
            self.merge(self.frontier)
            self.restamp_undecided()
            return
        self.frontier = next_after(self.frontier, sent)
        self.stamp[op] = self.frontier
        self.merge(self.frontier)
        point = self.rebase_point()
        if point is None:
            k = len(self.applied)
        elif point[1] == 0:
            k = point[0]
        else:
            edit = self.applied[point[0]]
            self.applied.insert(point[0] + 1, {"ops": edit["ops"][point[1]:], "key": edit["key"]})
            edit["ops"] = edit["ops"][: point[1]]
            k = point[0] + 1
        self.applied.insert(k, {"ops": [op], "key": None})
        self.restamp_undecided()

    def fold_order_holds(self):
        order = sorted(self.applied, key=lambda edit: (key(self.stamp[edit["ops"][0]]), edit["ops"][0]))
        return [edit["ops"][0] for edit in order] == [edit["ops"][0] for edit in self.applied]


def run(seed, trace=False, replicas=3, actions=48, maximum=8):
    roll = rng(seed)
    nxt = lambda: next(roll)
    actors = random.Random(seed).sample(range(1, 1 << 30), replicas)
    rs = [Replica(i, actors[i]) for i in range(replicas)]
    log, authored, counter, broken = [], [0] * replicas, [0], []

    def author(i, typing, now):
        counter[0] += 1
        op = f"{'abc'[i]}{counter[0]}"
        r = rs[i]
        r.stamp[op] = r.sent[op] = r.tick(now)
        tail = r.applied[-1] if r.applied else None
        if typing and tail is not None and tail["key"] == "typing":
            tail["ops"].append(op)
        else:
            r.applied.append({"ops": [op], "key": "typing" if typing else None})
        r.ledger.append(op)
        r.out.append(op)
        return op

    def sequence(i):
        if rs[i].cut:
            return
        for op in rs[i].out:
            log.append(op)
            for j, other in enumerate(rs):
                other.inbound.append(("committed" if j == i else "sequenced", op, rs[i].sent[op]))
        rs[i].out = []

    def deliver(i, count):
        if rs[i].cut:
            return
        take, rs[i].inbound = rs[i].inbound[:count], rs[i].inbound[count:]
        for message in take:
            rs[i].fold(message)

    bounds = [(35, "type"), (45, "apply"), (65, "sequence"), (90, "deliver"), (100, "cut")]
    for step in range(actions):
        value, i = nxt() % 100, nxt() % replicas
        action = next(name for total, name in bounds if value < total)
        if action in ("type", "apply") and authored[i] < maximum:
            nxt(), nxt()
            author(i, action == "type", step // TIE)
            authored[i] += 1
        elif action == "sequence":
            sequence(i)
        elif action == "deliver":
            held = len(rs[i].inbound)
            deliver(i, 0 if held == 0 else 1 + nxt() % held)
        elif action == "cut":
            rs[i].cut = not rs[i].cut
        broken += [(step, r.index) for r in rs if not r.fold_order_holds()]
    for r in rs:
        r.cut = False
    while True:
        for i in range(replicas):
            sequence(i)
        if all(not r.inbound for r in rs):
            break
        for i in range(replicas):
            deliver(i, len(rs[i].inbound))
        broken += [("quiesce", r.index) for r in rs if not r.fold_order_holds()]
    converged = all(r.ops() == log and not r.ledger for r in rs)
    stamps_agree = all(rs[0].stamp[op] == r.stamp[op] for r in rs for op in log)
    if trace:
        print("log", log, "broken", broken[:5])
    return converged, stamps_agree, broken


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    seeds = [int(args[0])] if args else range(1, 129)
    results = [(seed, run(seed, "--trace" in sys.argv)) for seed in seeds]
    print("converged", sum(r[0] for _, r in results), "stamps agree", sum(r[1] for _, r in results), "fold order held", sum(not r[2] for _, r in results), "/", len(results), "first broken", next(((s, r[2][:3]) for s, r in results if r[2]), None))
