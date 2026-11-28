#!/usr/bin/env python3
"""📈️ H102 simulation: whole-read gate (today) vs per-owner record publication, driven by the numbers of the real read log.
Usage: python3 simulate.py <graph.log> [runs] [hours]
The log has no timestamps and no per-owner reads: it calibrates (a) the stale-set sizes that environment moves left behind (the 'N projects carry edges of an older environment' lines), (b) owner count, wave sizes, (c) the number of hand-overs. Arrival rates and throughputs are ASSUMPTIONS, listed in PARAM and swept."""
import random, re, statistics, sys, json

def calibrate(log):
    stale, owners, waves, handovers, refresh = [], 5093, [], 0, 0
    for line in open(log, errors="replace"):
        m = re.match(r"graph: (\d+) projects carry edges of an older environment", line)
        if m: stale.append(int(m.group(1)))
        m = re.match(r"graph: refreshing (\d+) projects", line)
        if m: owners = int(m.group(1)); refresh += 1
        m = re.match(r"graph: inferring (\d+) projects in this process", line)
        if m: waves.append(int(m.group(1)))
        if re.match(r"graph: (this read runs code|pid \d+ runs code)", line): handovers += 1
    return {"stale_series": stale, "owners": owners, "waves": waves, "handovers": handovers, "refresh_lines": refresh}

PARAM = dict(env_every=20.0, code_every=100.0, edge_rate=100.0, infer_rate=6.0, tagged_share=0.3, verify_min=25.0, dirty0=1200, code_relevant=0.1, code_fraction=0.3, publish_min=2.0, round_overhead=18.0, retained=0.2)

def poisson_events(rng, every, horizon):
    t, out = 0.0, []
    while True:
        t += rng.expovariate(1 / every)
        if t > horizon: return out
        out.append(t)

def affected(rng, n, sizes):
    k = min(n, rng.choice(sizes)); return set(rng.sample(range(n), k))

def cost(p, stale_n):
    tagged = p["tagged_share"] * stale_n
    return p["round_overhead"] + (stale_n - tagged) / p["edge_rate"] + tagged / p["infer_rate"]

def make_events(rng, cal, p, horizon):
    n, sizes = cal["owners"], cal["stale_series"]
    env = [(t, affected(rng, n, sizes)) for t in poisson_events(rng, p["env_every"], horizon)]
    landings = [(t, set(rng.sample(range(n), int(p["code_fraction"] * n))) if rng.random() < p["code_relevant"] else set()) for t in poisson_events(rng, p["code_every"], horizon)]
    return env, landings

def stale_in(env, landings, a, b):
    out = set()
    for et, s in env:
        if a <= et <= b: out |= s
    for lt, s in landings:
        if a <= lt <= b: out |= s
    return out

def run_current(rng, cal, p, horizon):
    n = cal["owners"]; env, landings = make_events(rng, cal, p, horizon)
    dirty = set(rng.sample(range(n), p["dirty0"])); t, invalid, pub_times = 0.0, set(dirty), []
    next_landing = lambda after: next((lt for lt, _ in landings if lt > after), None)
    while t < horizon:
        work = p["verify_min"] + p["round_overhead"] + len(invalid) / p["infer_rate"]; start, end = t, t + work
        c = next_landing(start)
        if c is not None and c < end:
            frac = (c - start) / work; invalid = set(rng.sample(sorted(invalid), int(len(invalid) * (1 - p["retained"] * frac)))) if invalid else invalid
            t = c + 5.0; continue
        stale = stale_in(env, landings, start, end); t = end; aborted = False
        while stale:
            d = cost(p, len(stale)); c = next_landing(t)
            if c is not None and c < t + d: t = c + 5.0; aborted = True; break
            new = stale_in(env, landings, t, t + d); t += d; stale = new
        if aborted: continue
        t += p["publish_min"]
        if t > horizon: break
        pub_times.append(t); invalid = set()
    vals = []
    for g in [i * 5.0 for i in range(int(horizon / 5))]:
        P = max([pt for pt in pub_times if pt <= g], default=None)
        bad = set(rng.sample(range(n), p["dirty0"])) if False else None
        if P is None: vals.append(1 - len(dirty | stale_in(env, landings, 0, g)) / n); continue
        vals.append(1 - len(stale_in(env, landings, P, g)) / n)
    return {"pubs": len(pub_times), "first_pub_h": (pub_times[0] / 60 if pub_times else None), "mean_cov": statistics.mean(vals), "frac_ge99": sum(v >= .99 for v in vals) / len(vals)}

def run_incremental(rng, cal, p, horizon):
    n = cal["owners"]; env, landings = make_events(rng, cal, p, horizon)
    events = [(t, "env", s) for t, s in env] + [(t, "code", s) for t, s in landings if s]; events.sort(key=lambda e: e[0])
    invalid_infer, invalid_edge = set(rng.sample(range(n), p["dirty0"])), set()
    t, ei, vals, recover, since = 0.0, 0, [], [], {}
    ce = ci = 0.0
    while t < horizon:
        while ei < len(events) and events[ei][0] <= t:
            _, kind, s = events[ei]; ei += 1
            for o in s:
                if o in invalid_infer: continue
                if kind == "code" or rng.random() < p["tagged_share"]: invalid_infer.add(o); invalid_edge.discard(o)
                elif o not in invalid_edge: invalid_edge.add(o)
                since.setdefault(o, t)
        ce += p["edge_rate"]; ci += p["infer_rate"]
        while ce >= 1 and invalid_edge: o = invalid_edge.pop(); ce -= 1; recover.append(t - since.pop(o, t))
        if not invalid_edge: ce = 0
        while ci >= 1 and invalid_infer: o = invalid_infer.pop(); ci -= 1; recover.append(t - since.pop(o, t))
        if not invalid_infer: ci = 0
        if int(t) % 5 == 0: vals.append(1 - (len(invalid_edge) + len(invalid_infer)) / n)
        t += 1.0
    recover.sort()
    return {"mean_cov": statistics.mean(vals), "frac_ge99": sum(v >= .99 for v in vals) / len(vals), "recover_p50_min": recover[len(recover) // 2] if recover else 0, "recover_p95_min": recover[int(len(recover) * .95)] if recover else 0}

def summarize(rows, keys):
    out = {}
    for k in keys:
        vals = [r[k] for r in rows if r.get(k) is not None]
        out[k] = round(statistics.mean(vals), 3) if vals else None
    return out

if __name__ == "__main__":
    log = sys.argv[1]; runs = int(sys.argv[2]) if len(sys.argv) > 2 else 200; hours = float(sys.argv[3]) if len(sys.argv) > 3 else 24
    cal = calibrate(log); print("calibration", json.dumps(cal))
    for label, over in [("base", {}), ("env 2x faster", {"env_every": 10.0}), ("env 2x slower", {"env_every": 40.0}), ("code 2x faster", {"code_every": 50.0}), ("code relevant 30%", {"code_relevant": 0.3}), ("code relevant 100%", {"code_relevant": 1.0}), ("infer 3x faster", {"infer_rate": 18.0}), ("edges 10x slower", {"edge_rate": 10.0}), ("no code relevance", {"code_relevant": 0.0})]:
        p = dict(PARAM); p.update(over)
        cur = [run_current(random.Random(1000 + i), cal, p, hours * 60) for i in range(runs)]
        inc = [run_incremental(random.Random(1000 + i), cal, p, hours * 60) for i in range(runs)]
        print(f"[{label}] today: publications/day {summarize(cur,['pubs'])['pubs']}, runs with no publication {sum(r['pubs']==0 for r in cur)}/{runs}, mean owners usable {summarize(cur,['mean_cov'])['mean_cov']}, time >=99% usable {summarize(cur,['frac_ge99'])['frac_ge99']}")
        print(f"[{label}] per-owner: mean owners usable {summarize(inc,['mean_cov'])['mean_cov']}, time >=99% usable {summarize(inc,['frac_ge99'])['frac_ge99']}, recovery p50 {summarize(inc,['recover_p50_min'])['recover_p50_min']} min, p95 {summarize(inc,['recover_p95_min'])['recover_p95_min']} min")
