#!/usr/bin/env python3
"""Independent Python model of the birth handshake work clock; writes fixture.json: python3 -I oracle.py
Model: budget B ms of work, every wall ms is worth 1/clamp(ratio,1,8) work ms until `waiting` or `settle`; expiry = the wall time where the integral reaches B.
A tick attributes its whole interval to the stretch sampled at its end, so a stepped row is late by at most one tick times the rate change (tolerance 2000).
Rows: ratio steps [[fromWall, ratio]...], events [[at, kind]...], deadline (ms after start or null)."""
import json, os
B, LOW, HIGH = 10000.0, 1.0, 8.0
def clamp(r): return min(HIGH, max(LOW, r))
def expiry(steps):
    t, credit = 0.0, 0.0
    steps = sorted(steps)
    for i, (frm, ratio) in enumerate(steps):
        to = steps[i + 1][0] if i + 1 < len(steps) else float("inf"); rate = 1 / clamp(ratio)
        if credit + (to - max(t, frm)) * rate >= B - 1e-9: return max(t, frm) + (B - credit) / rate
        credit += (to - max(t, frm)) * rate; t = to
    return None
def row(name, steps, events=(), deadline=None, until=600000, tolerance=2):
    end = expiry(steps); waiting = [e[0] for e in events if e[1] == "waiting"]; settle = [e[0] for e in events if e[1] == "settle"]
    cut = min(waiting + settle + [float("inf")])
    birth = end if end is not None and end < cut else None
    final = None
    if birth is not None: final = ["build.bazel.birth-handshake-deadline-exhausted", birth]
    if deadline is not None and (final is None or deadline < final[1]) and not (settle and settle[0] <= deadline): final = ["build.bazel.birth-deadline-exhausted", float(deadline)]
    if final and final[0] == "build.bazel.birth-deadline-exhausted" and (birth is not None and birth <= deadline): final = ["build.bazel.birth-handshake-deadline-exhausted", birth]
    checks = []
    for at, kind in events:
        if kind != "check": continue
        phase_over = at >= cut
        checks.append([at, (not phase_over) and end is not None and at >= end])
    return {"name": name, "ratios": steps, "events": [list(e) for e in events], "deadline": deadline, "until": until, "tolerance": tolerance, "fails": final, "checks": checks}
rows = [
    row("idle host: the plain budget", [[0, 0.4]], [[5000, "check"], [9000, "check"], [12000, "check"]]),
    row("load 18.5 on 10 cores, no waiting: the budget counts at 1/1.85", [[0, 1.85]], [[9000, "check"], [15000, "check"], [17000, "check"], [20000, "check"]]),
    row("load 18.5 on 10 cores, waiting at 600 ms: nothing fails, the late joined passes", [[0, 1.85]], [[600, "waiting"], [15000, "check"], [120000, "check"]]),
    row("the g48p-edit log: waiting for the grant 15 s, then joined", [[0, 1.85]], [[300, "waiting"], [15200, "check"], [16000, "settle"], [200000, "check"]]),
    row("hung parent on a saturated host: the ceiling is eight times the budget", [[0, 12.0]], [[70000, "check"], [90000, "check"]]),
    row("host load rises after the budget was mostly used", [[0, 1.5], [12000, 4.0]], [[11000, "check"], [18000, "check"], [23000, "check"]], tolerance=2000),
    row("the owner's deadline bounds a waiting birth", [[0, 1.85]], [[400, "waiting"]], deadline=5000),
    row("settled birth is never cancelled", [[0, 1.85]], [[2000, "settle"], [500000, "check"]], deadline=None),
    row("settled birth ignores the owner deadline timer", [[0, 1.0]], [[2000, "settle"]], deadline=5000),
]
json.dump({"budget": B, "epoch": 1700000000000, "rows": rows}, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "fixture.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
for r in rows: print(r["name"][:60], r["fails"], r["checks"])
