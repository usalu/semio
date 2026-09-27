"""🧫️ F2 — writes `🧑‍💻dev/🧫️fixtures/💤️idle-budget.json`: a synthetic Chromium trace with known frame/draw/task counts and two memory
soaks, with every expected value computed here (independent least squares), the oracle of the TS reducers.
usage: python3 f2-idle-fixture.py <out.json>"""
import json, sys
events = []
def meta(pid, tid, name): events.append({"ph": "M", "name": "thread_name", "pid": pid, "tid": tid, "args": {"name": name}})
meta(10, 1, "CrRendererMain"); meta(10, 2, "Compositor"); meta(10, 3, "DedicatedWorker thread"); meta(20, 1, "VizCompositorThread"); meta(10, 4, "CompositorTileWorker1")
meta(30, 1, "CrRendererMain")
events.append({"ph": "X", "name": "RunTask", "pid": 30, "tid": 1, "ts": 0, "dur": 1000})
seconds = 10
for i in range(7): events.append({"ph": "X", "name": "BeginMainThreadFrame", "pid": 10, "tid": 1, "ts": i * 1000, "dur": 10})
for i in range(3): events.append({"ph": "X", "name": "FireAnimationFrame", "pid": 10, "tid": 1, "ts": i * 1000, "dur": 10})
for i in range(4): events.append({"ph": "X", "name": "DrawFrame", "pid": 10, "tid": 2, "ts": i * 1000, "dur": 10})
for i in range(2): events.append({"ph": "X", "name": "DrawFrame", "pid": 20, "tid": 1, "ts": i * 1000, "dur": 10})
for i in range(5): events.append({"ph": "X", "name": "DrawFrame", "pid": 10, "tid": 4, "ts": i * 1000, "dur": 10})
for ts, dur in [(0, 100000), (50000, 100000), (1000000, 200000), (2000000, 0)]:
    events.append({"ph": "X", "name": "ThreadControllerImpl::RunTask", "pid": 10, "tid": 1, "ts": ts, "dur": dur})
events.append({"ph": "X", "name": "RunTask", "pid": 10, "tid": 3, "ts": 0, "dur": 250000})
expected = {"seconds": 10, "framesPerSec": 0.7, "rafPerSec": 0.3, "drawsPerSec": 0.6, "mainBusyPct": 3.5, "workersBusyPct": 2.5}
def slope(xs, ys):
    n = len(xs); mx = sum(xs) / n; my = sum(ys) / n
    num = sum((x - mx) * (y - my) for x, y in zip(xs, ys)); den = sum((x - mx) ** 2 for x in xs)
    return 0 if den == 0 else num / den
def idle(f=0.0): return {"seconds": 5, "framesPerSec": f, "rafPerSec": 0, "drawsPerSec": 0, "mainBusyPct": 0.1, "workersBusyPct": 0.1}
flat = [{"minute": m, "jsHeapMB": round(100 + (8 if m < 6 else 0) * (6 - m) / 6 + (0.3 if m % 2 else 0), 2), "workersHeapMB": 40.0, "nodes": 5000 + (m % 3), "listeners": 900, "idle": idle()} for m in range(0, 31)]
leak = [{"minute": m, "jsHeapMB": round(100 + 0.9 * m, 2), "workersHeapMB": 40.0 + 0.1 * m, "nodes": 5000 + 20 * m, "listeners": 900, "idle": idle(2.0 if m == 17 else 0.0)} for m in range(0, 31)]
def verdict(samples):
    last = samples[-1]["minute"]; warmfrom = -(-last // 5)
    warm = [s for s in samples if s["minute"] >= warmfrom]
    xs = [s["minute"] for s in warm]
    hs = round(slope(xs, [s["jsHeapMB"] for s in warm]), 3); ws = round(slope(xs, [s["workersHeapMB"] for s in warm]), 3)
    g = lambda a, b: 0 if a <= 0 else round((b - a) / a, 4)
    dg = g(warm[0]["nodes"], warm[-1]["nodes"]); lg = g(warm[0]["listeners"], warm[-1]["listeners"])
    mf = max([0] + [max(s["idle"]["framesPerSec"], s["idle"]["rafPerSec"], s["idle"]["drawsPerSec"]) for s in samples])
    v = [k for k, bad in [("heap", hs > 0.5), ("workers", ws > 0.5), ("dom", dg > 0.01), ("listeners", lg > 0.01), ("frames", mf >= 1)] if bad]
    return {"heapSlopeMBPerMin": hs, "workersSlopeMBPerMin": ws, "domGrowthRatio": dg, "listenerGrowthRatio": lg, "maxFramesPerSec": mf, "violationKinds": v}
fx = {"schema": "semio.os-dev.idle-budget-fixture/v1", "trace": {"seconds": seconds, "events": events, "expected": expected},
      "soaks": [{"name": "warm-up then flat", "samples": flat, "expected": verdict(flat)}, {"name": "leaking heap, DOM and a frame burst", "samples": leak, "expected": verdict(leak)}]}
open(sys.argv[1], "w").write(json.dumps(fx, indent=1, ensure_ascii=False) + "\n")
