import importlib.util, itertools, json, sys
import numpy as np
CLIP = "C:/git/semio/🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip"
path = sys.argv[1] if len(sys.argv) > 1 and sys.argv[1] else CLIP + "/🧮️model/🐍️.py"
spec = importlib.util.spec_from_file_location("m", path); m = importlib.util.module_from_spec(spec); spec.loader.exec_module(m)
QUANTITIES = ("heating", "cooling", "lighting", "electricity")
def curve(wwr, d, step):
    thetas = np.arange(90, 450 + step / 2, step)
    values = np.array([[m._evaluate(float(t), wwr, d, 0.16)[q] for q in QUANTITIES] for t in thetas])
    return thetas, values
report, worst = {}, {q: (0.0, None) for q in QUANTITIES}
for wwr, d in itertools.product((0.3, 0.6, 0.8), (0.0, 0.3, 0.6, 1.0, 1.2)):
    t1, v1 = curve(wwr, d, 0.25)
    t2, v2 = curve(wwr, d, 0.0625)
    steps1 = np.abs(np.diff(v1, axis=0)).max(axis=0)
    steps2 = np.abs(np.diff(v2, axis=0)).max(axis=0)
    bound = steps2 * 4
    arg = np.abs(np.diff(v1, axis=0)).argmax(axis=0)
    for i, q in enumerate(QUANTITIES):
        report[f"wwr{wwr} d{d} {q}"] = {"max_step_0.25": steps1[i], "max_step_0.0625": steps2[i], "bound_4x": bound[i], "ratio": steps1[i] / steps2[i] if steps2[i] else None, "theta": float(t1[arg[i]])}
        if steps1[i] > worst[q][0]:
            worst[q] = (steps1[i], f"wwr {wwr} d {d} at {t1[arg[i]]}°")
print(json.dumps({"worst": worst}, default=float, indent=1))
ratios = [r["ratio"] for r in report.values() if r["ratio"]]
print("ratio step(0.25)/step(0.0625): min %.2f max %.2f (a jump gives 1, a Lipschitz curve gives about 4)" % (min(ratios), max(ratios)))
print("any step above 4.2 x finer step:", [k for k, r in report.items() if r["ratio"] and r["ratio"] > 4.2][:10])
json.dump(report, open(sys.argv[2] if len(sys.argv) > 2 else "🗑️generated/model-r3/continuity-after.json", "w", encoding="utf-8"), default=float, indent=1)
