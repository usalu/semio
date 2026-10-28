"""🧮️ Independent Python model of the host capacity admission (fix-g: per-grant ramp = time to peak, a grant counts what it holds when that is more, ramp slots for shrunk reservations): the pure decision (port of `evaluateHostCapacityV1`, equal to all stored TypeScript cases incl. the ramp cases, see `check_model.py`); the measured-demand estimator lives in the repo oracle `🧪️tests/📐️estimates/🐍️oracle/🐍️.py`."""
import math

def floor_mb(policy, total):
    m = policy["memory"]
    share = total * m["floorBasisPoints"] // 10000
    return min(total, max(m["floorMinimumMegabytes"], min(m["floorMaximumMegabytes"], share)))

def order(waiters, ledger):
    def held(w):
        return 0 if w.get("session") is None else sum(g["reservedMegabytes"] for g in ledger if g.get("session") == w["session"])
    def weight(w):
        return w.get("weight", 1)
    import functools
    def cmp(a, b):
        ra = 0 if a["request"]["priority"] == "interactive" else 1
        rb = 0 if b["request"]["priority"] == "interactive" else 1
        for left, right in ((ra, rb), (held(a) * weight(b), held(b) * weight(a)), (a["arrivedAtMs"], b["arrivedAtMs"]), (a["request"]["id"], b["request"]["id"])):
            if left != right:
                return -1 if left < right else 1
        return 0
    return sorted(waiters, key=functools.cmp_to_key(cmp))

def soft_reason(policy, host):
    if host["pressure"] == "warn":
        return "pressure-warn"
    if host["openFiles"] is not None and host["openFilesMaximum"] is not None and host["openFiles"] * 10000 > host["openFilesMaximum"] * policy["files"]["ceilingBasisPoints"]:
        return "files"
    if host["attachedImages"] is not None and host["attachedImages"] >= policy["attaches"]["maximum"]:
        return "attaches"
    return None

def evaluate(policy, host, ledger, waiters, now):
    floor = floor_mb(policy, host["totalMegabytes"])
    cap = max(0, host["totalMegabytes"] - floor)
    def ramping(g):
        return now - g.get("resumedAtMs", g["admittedAtMs"]) < g.get("rampMilliseconds", policy["memory"]["rampMilliseconds"])
    active = [g for g in ledger if g.get("lentSinceMs") is None]
    unrealized = sum(max(0, g["reservedMegabytes"] - (g["observedMegabytes"] or 0)) if ramping(g) else 0 for g in active)
    committed = sum(max(g["reservedMegabytes"], g["observedMegabytes"] or 0) if ramping(g) else (g["observedMegabytes"] if g["observedMegabytes"] is not None else g["reservedMegabytes"]) for g in active)
    observed = host["availableMegabytes"] - floor - unrealized
    budget = cap * policy["memory"]["budgetBasisPoints"] // 10000 - committed
    binding = "budget" if budget < observed else "available"
    room = min(observed, budget)
    headroom = room
    ledger_cpu = sum(g["cpu"] for g in active)
    ceiling = policy["load"]["perCoreCeiling"] * host["cores"]
    load_room = math.inf if host["load"] is None and ledger_cpu == 0 else ceiling - max(host["load"] or 0, ledger_cpu)
    reported = None if load_room == math.inf else load_room
    swap_ceiling = host["totalMegabytes"] * policy["swap"]["ceilingBasisPoints"] // 10000
    rate = host["swapOutMegabytesPerSecond"]
    swapping = rate is None or rate > 0
    soft = soft_reason(policy, host)
    if host["pressure"] == "critical":
        hard = "pressure-critical"
    elif host["swapUsedMegabytes"] is not None and host["swapUsedMegabytes"] > swap_ceiling and swapping:
        hard = "swap"
    elif rate is not None and rate > policy["swap"]["outMegabytesPerSecond"]:
        hard = "swap-activity"
    else:
        hard = None
    drained_room = min(host["availableMegabytes"] + sum(g["observedMegabytes"] or 0 for g in ledger), host["totalMegabytes"]) - floor
    drained_budget = cap * policy["memory"]["budgetBasisPoints"] // 10000
    aged = []
    for w in waiters:
        if w["request"]["priority"] == "batch" and now - w["arrivedAtMs"] >= policy["wait"]["agingMilliseconds"]:
            aged.append({**w, "request": {**w["request"], "priority": "interactive"}})
        else:
            aged.append(w)
    decisions = []
    heads = []
    blocked = False
    admitted = 0
    own = ledger_cpu
    rampers = len([g for g in active if g.get("rampSlot") is True and ramping(g)])
    w_ = policy["wait"]
    for waiter in order(aged, active):
        r = waiter["request"]
        def need(jobs):
            return min(r["fixedMegabytes"] + jobs * r["perJobMegabytes"], cap)
        waited = now - waiter["arrivedAtMs"]
        def refuse(reason):
            decisions.append({"id": r["id"], "verdict": "wait", "jobs": 0, "reservedMegabytes": 0, "reason": reason})
        def admit(jobs, reason):
            nonlocal room, load_room, own, admitted, blocked, rampers
            reserved = need(jobs)
            room -= reserved
            if load_room != math.inf:
                load_room -= jobs * r["cpuPerJob"]
            own += jobs * r["cpuPerJob"]
            admitted += 1
            if r.get("rampSlot") is True:
                rampers += 1
            for head in heads:
                head["bypasses"] += 1
                if head["bypasses"] >= w_["maximumBypasses"]:
                    blocked = True
            decisions.append({"id": r["id"], "verdict": "admit", "jobs": jobs, "reservedMegabytes": reserved, "reason": reason})
        if hard:
            refuse(hard); continue
        if blocked:
            refuse("queue-order"); continue
        if r.get("rampSlot") is True and rampers >= policy["memory"]["maximumRamping"]:
            refuse("ramp"); continue
        memory_fits = need(r["minimumJobs"]) <= room
        jobs = 0
        if soft is None:
            for candidate in range(r["preferredJobs"], r["minimumJobs"] - 1, -1):
                if need(candidate) <= room and candidate * r["cpuPerJob"] <= load_room:
                    jobs = candidate; break
        if jobs == 0 and soft is None and memory_fits and own + r["minimumJobs"] * r["cpuPerJob"] <= ceiling:
            jobs = r["minimumJobs"]
        if jobs > 0:
            admit(jobs, "reduced" if jobs < r["preferredJobs"] else "fits"); continue
        reason = "memory" if not memory_fits else (soft or "load")
        if reason != "memory" and len(active) == 0 and admitted == 0 and waited >= policy["load"]["overrideAfterMilliseconds"]:
            admit(r["minimumJobs"], "override")
        else:
            refuse(reason)
            if reason in ("memory", "load"):
                drains = soft is None and need(r["minimumJobs"]) <= min(drained_room, drained_budget) and r["minimumJobs"] * r["cpuPerJob"] <= ceiling
                bypasses = len([g for g in ledger if g["admittedAtMs"] >= waiter["arrivedAtMs"]])
                if drains and (waited >= w_["bypassMilliseconds"] or bypasses >= w_["maximumBypasses"]):
                    blocked = True
                elif drains:
                    heads.append({"bypasses": bypasses})
    return {"floorMegabytes": floor, "unrealizedMegabytes": unrealized, "headroomMegabytes": headroom, "binding": binding, "loadRoom": reported, "decisions": decisions}
