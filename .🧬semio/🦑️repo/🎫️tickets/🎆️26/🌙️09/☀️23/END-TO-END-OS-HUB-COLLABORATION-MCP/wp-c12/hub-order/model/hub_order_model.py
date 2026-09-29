"""🧮️ C12: Python model of the Rust hub-order seeded property (`seeded_concurrent_typing_and_reconnects_converge_on_the_hubs_order`):
same splitmix64 roll stream, same harness (author / sequence / deliver / cut / quiesce), the store's intended placement rule. Prints the
action trace of one seed and whether every replica converges on the hub's log. usage: python3 hub_order_model.py [seed] [--trace]"""
import sys

M = (1 << 64) - 1


def rng(seed):
    state = seed
    while True:
        state = (state + 0x9E3779B97F4A7C15) & M
        z = state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M
        yield z ^ (z >> 31)


class Replica:
    def __init__(self, name):
        self.name, self.applied, self.ledger, self.out, self.inbound, self.cut = name, [], [], [], [], False

    def ops(self):
        return [op for edit in self.applied for op in edit["ops"]]

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

    def fold(self, message, trace):
        kind, ops = message
        if kind == "committed":
            self.ledger = [op for op in self.ledger if op not in ops]
            return 0
        splits = 0
        for op in ops:
            point = self.rebase_point()
            if point is None:
                k = len(self.applied)
            elif point[1] == 0:
                k = point[0]
            else:
                edit = self.applied[point[0]]
                suffix = {"ops": edit["ops"][point[1]:], "key": edit["key"], "own": True}
                edit["ops"] = edit["ops"][: point[1]]
                self.applied.insert(point[0] + 1, suffix)
                k = point[0] + 1
                splits += 1
            self.applied.insert(k, {"ops": [op], "key": None, "own": False})
        return splits


def run(seed, trace=False, replicas=3, actions=48, maximum=8):
    roll = rng(seed)
    nxt = lambda: next(roll)
    rs = [Replica(i) for i in range(replicas)]
    log, authored, counter, splits = [], [0] * replicas, [0], [0]

    def author(i, typing):
        counter[0] += 1
        op = f"{'abc'[i]}{counter[0]}"
        r = rs[i]
        tail = r.applied[-1] if r.applied else None
        if typing and tail is not None and tail["key"] == "typing":
            tail["ops"].append(op)
        else:
            r.applied.append({"ops": [op], "key": "typing" if typing else None, "own": True})
        r.ledger.append(op)
        r.out.append(op)
        return op

    def sequence(i):
        if rs[i].cut:
            return
        for op in rs[i].out:
            log.append(op)
            for j, other in enumerate(rs):
                other.inbound.append(("committed", [op]) if j == i else ("sequenced", [op]))
        rs[i].out = []

    def deliver(i, count):
        if rs[i].cut:
            return
        take, rs[i].inbound = rs[i].inbound[:count], rs[i].inbound[count:]
        for message in take:
            splits[0] += rs[i].fold(message, trace)

    bounds = [(35, "type"), (45, "apply"), (65, "sequence"), (90, "deliver"), (100, "cut")]
    for step in range(actions):
        value = nxt() % 100
        i = nxt() % replicas
        action = next(name for total, name in bounds if value < total)
        note = ""
        if action in ("type", "apply") and authored[i] < maximum:
            if nxt() % 4 == 0:
                nxt()
            else:
                nxt()
            note = author(i, action == "type")
            authored[i] += 1
        elif action == "sequence":
            sequence(i)
        elif action == "deliver":
            held = len(rs[i].inbound)
            count = 0 if held == 0 else 1 + nxt() % held
            note = f"count={count} cut={rs[i].cut}"
            deliver(i, count)
        elif action == "cut":
            rs[i].cut = not rs[i].cut
            note = f"-> {rs[i].cut}"
        elif action in ("type", "apply"):
            note = "(max)"
        if trace:
            print(f"{step:2} {action:8} r{i} {note:18} log={log} " + " | ".join(f"r{r.name}:{[e['ops'] for e in r.applied]} L{r.ledger}" for r in rs))
    for r in rs:
        r.cut = False
    while True:
        for i in range(replicas):
            sequence(i)
        if all(not r.inbound for r in rs):
            break
        for i in range(replicas):
            deliver(i, len(rs[i].inbound))
    ok = all(r.ops() == log and not r.ledger for r in rs)
    if trace:
        print("log", log)
        for r in rs:
            print(f"r{r.name}", r.ops(), r.ledger)
    return ok, splits[0]


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if args:
        print(run(int(args[0]), "--trace" in sys.argv))
    else:
        results = [run(seed) for seed in range(1, 129)]
        print("converged", sum(ok for ok, _ in results), "/ 128; splits", sum(s for _, s in results), "; failing", [seed for seed, (ok, _) in zip(range(1, 129), results) if not ok][:10])
