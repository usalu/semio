"""📐️ Independent Python oracle of the measured-demand estimates: decayed weighted quantile, shrinkage along the class lineage, recording and request scaling; the weighted quantile is cross-checked with numpy (a third-party library) and the shared vectors for every language are written next to it. Run `python3 🐍️.py` to regenerate `../🧫️fixtures/🔣️.json`."""
import json,math,os,random,re,sys

MINIMUM_RAMP = 60000
POLICY = {"halfLifeMilliseconds": 259200000, "quantileBasisPoints": 9000, "marginBasisPoints": 2500, "minimumSamples": 3, "maximumSamples": 16, "maximumClasses": 256, "minimumRatioBasisPoints": 3000, "maximumRatioBasisPoints": 20000, "minimumMegabytes": 512, "quickMilliseconds": 120000, "maximumShareBasisPoints": 5000, "fallbackCeilingMegabytes": 8192, "defaultRampMilliseconds": 300000, "maximumRampMilliseconds": 1800000, "phaseReferenceMegabytes": 1024, "rebuildPriorMegabytes": 4096, "rebuildShareBasisPoints": 5000, "maximumRebuildSlots": 4, "quietReferenceMegabytes": 2048, "quietMaximumMegabytes": 4096, "quietFollowUpMilliseconds": 0}

def _round(x):
    return math.floor(x + 0.5)

def weights(policy, samples, now):
    half = policy["estimates"]["halfLifeMilliseconds"]
    return [_round(1e6 * 2.0 ** (-(max(0, now - s["atMs"])) / half)) for s in samples]

def value_of(sample, key):
    return sample.get("peakAtMilliseconds", sample["activeMilliseconds"]) if key == "peakAtMilliseconds" else sample[key]

def weighted_quantile(samples, weight, key, basis_points):
    rows = sorted(zip(samples, weight), key=lambda row: (value_of(row[0], key), row[0]["atMs"]))
    total = sum(w for _, w in rows)
    target = -(-total * basis_points // 10000)
    cumulative = 0
    for sample, w in rows:
        cumulative += w
        if cumulative >= target:
            return value_of(sample, key)
    return value_of(rows[-1][0], key)

def numpy_quantile(samples, weight, key, basis_points):
    import numpy
    order = numpy.lexsort(([s["atMs"] for s in samples], [value_of(s, key) for s in samples]))
    values = numpy.array([value_of(samples[i], key) for i in order])
    cumulative = numpy.cumsum(numpy.array([weight[i] for i in order], dtype=numpy.int64))
    target = -(-int(cumulative[-1]) * basis_points // 10000)
    return int(values[numpy.searchsorted(cumulative, target, side="left")])

def estimate(policy, store, lineage, now):
    e = policy["estimates"]
    ratio, ramp, learned, quick = 10000, e["defaultRampMilliseconds"], 0, False
    for key in lineage:
        samples = store["classes"].get(key, {"samples": []})["samples"]
        weight = weights(policy, samples, now)
        total = sum(weight)
        if not samples or total == 0:
            ratio = min(10000, ratio)
            ramp = min(e["defaultRampMilliseconds"], ramp)
            quick = False
            continue
        effective = total / 1e6
        peak = _round(weighted_quantile(samples, weight, "ratioBasisPoints", e["quantileBasisPoints"]) * (10000 + e["marginBasisPoints"]) / 10000)
        trust = min(1.0, effective / e["minimumSamples"])
        late = _round(weighted_quantile(samples, weight, "peakAtMilliseconds", e["quantileBasisPoints"]) * (10000 + e["marginBasisPoints"]) / 10000)
        ratio = _round(trust * peak + (1 - trust) * min(10000, ratio))
        ramp = _round(trust * late + (1 - trust) * min(e["defaultRampMilliseconds"], ramp))
        learned = _round(effective)
        quick = effective >= e["minimumSamples"] and e["quickMilliseconds"] > 0 and weighted_quantile(samples, weight, "activeMilliseconds", 5000) <= e["quickMilliseconds"]
    return {"ratioBasisPoints": max(e["minimumRatioBasisPoints"], min(e["maximumRatioBasisPoints"], ratio)), "rampMilliseconds": max(MINIMUM_RAMP, min(e["maximumRampMilliseconds"], ramp)), "samples": learned, "quick": quick}

def class_ceiling(policy, key):
    rows = [row["megabytes"] for row in policy.get("classCeilings", []) if re.search(row["pattern"], key)]
    return min(rows) if rows else None

def measured_peak(policy, store, lineage, now):
    e = policy["estimates"]
    samples = [s for s in store["classes"].get(lineage[-1], {"samples": []})["samples"] if "peakMegabytes" in s]
    if not samples:
        return None
    weight = weights(policy, samples, now)
    if sum(weight) / 1e6 < e["minimumSamples"]:
        return None
    return _round(weighted_quantile(samples, weight, "peakMegabytes", e["quantileBasisPoints"]) * (10000 + e["marginBasisPoints"]) / 10000)

def class_bound(policy, store, lineage, now):
    declared, measured = class_ceiling(policy, lineage[-1]), measured_peak(policy, store, lineage, now)
    if measured is not None:
        return (measured if declared is None else min(measured, declared)), "measured"
    if declared is not None:
        return declared, "declared"
    return policy["estimates"]["fallbackCeilingMegabytes"], "fallback"

def record(policy, store, lineage, outcome):
    e = policy["estimates"]
    ratio = max(1, _round(outcome["peakMegabytes"] * 10000 / outcome["defaultMegabytes"]))
    classes = {k: {"samples": list(v["samples"])} for k, v in store["classes"].items()}
    for key in lineage:
        rows = classes.setdefault(key, {"samples": []})["samples"]
        rows.append({"atMs": outcome["endedAtMs"], "ratioBasisPoints": ratio, "activeMilliseconds": outcome["activeMilliseconds"], "peakAtMilliseconds": min(outcome["peakAtMilliseconds"], outcome["activeMilliseconds"])})
        rows.sort(key=lambda s: (-s["atMs"], s["ratioBasisPoints"], s["activeMilliseconds"], value_of(s, "peakAtMilliseconds")))
        classes[key]["samples"] = rows[: e["maximumSamples"]]
    kept = sorted(classes, key=lambda k: (-max(s["atMs"] for s in classes[k]["samples"]), k))[: e["maximumClasses"]]
    return {"version": 1, "classes": {k: classes[k] for k in sorted(kept)}}

def scale(policy, request, ratio, capacity):
    per_job = _round(request["perJobMegabytes"] * ratio / 10000)
    fixed = _round(request["fixedMegabytes"] * ratio / 10000)
    fixed += max(0, policy["estimates"]["minimumMegabytes"] - (fixed + request["minimumJobs"] * per_job))
    ceiling = capacity * policy["estimates"]["maximumShareBasisPoints"] // 10000
    need = fixed + request["minimumJobs"] * per_job
    if need > ceiling:
        fixed = fixed * ceiling // need
        per_job = per_job * ceiling // need
    return {**request, "fixedMegabytes": fixed, "perJobMegabytes": per_job, "envelopePerJobMegabytes": request["perJobMegabytes"]}

def ramp_request(base, scaled, estimate, measured=False):
    shrunk = scaled["fixedMegabytes"] < base["fixedMegabytes"] or scaled["perJobMegabytes"] < base["perJobMegabytes"]
    return {**scaled, "rampMilliseconds": estimate["rampMilliseconds"], "rampSlot": shrunk and not measured}

def learnable(status, peak_ratio, estimate_ratio):
    return status == "ok" or peak_ratio > estimate_ratio

def priority(base, standing, quick):
    return "interactive" if base == "interactive" or standing == "interactive" or quick else "batch"

def generate():
    policy = {"estimates": POLICY}
    rng = random.Random(20261026)
    lineages = [["run"], ["run", "run:build"], ["run", "run:build", "run:@a/b:build"], ["cargo", "cargo:check"], ["show"]]
    cases = []
    for index in range(80):
        lineage = rng.choice(lineages)
        store = {"version": 1, "classes": {}}
        now = 1_000_000_000 + rng.randrange(0, 10 * 86_400_000)
        for step in range(rng.choice([0, 1, 2, 3, 5, 9, 20])):
            peak = rng.choice([150, 400, 900, 1700, 3500, 6000, 13000])
            outcome = {"operation": lineage[0], "jobs": rng.choice([1, 2, 4]), "peakMegabytes": peak, "defaultMegabytes": rng.choice([1536, 3584, 5120, 9728]), "activeMilliseconds": rng.choice([800, 9000, 60000, 300000, 900000]), "peakAtMilliseconds": rng.choice([0, 500, 8000, 40000, 250000, 900000]), "endedAtMs": now - rng.randrange(0, 12 * 86_400_000)}
            store = record(policy, store, rng.choice([lineage, lineage[: max(1, len(lineage) - 1)]]), outcome)
        for entry in store["classes"].values():
            for sample in entry["samples"]:
                if rng.random() < 0.3:
                    del sample["peakAtMilliseconds"]
        case = {"name": f"estimate-{index}", "lineage": lineage, "nowMs": now, "store": store, "expect": estimate(policy, store, lineage, now)}
        for key, entry in store["classes"].items():
            w = weights(policy, entry["samples"], now)
            if sum(w) > 0:
                for field, basis_points in (("ratioBasisPoints", 9000), ("ratioBasisPoints", 5000), ("activeMilliseconds", 5000), ("peakAtMilliseconds", 9000)):
                    assert weighted_quantile(entry["samples"], w, field, basis_points) == numpy_quantile(entry["samples"], w, field, basis_points), (key, field)
        cases.append(case)
    request = {"id": "aaaaaaaa1", "operation": "run", "priority": "batch", "minimumJobs": 1, "preferredJobs": 4, "fixedMegabytes": 1536, "perJobMegabytes": 2048, "cpuPerJob": 1}
    scales = [{"ratioBasisPoints": r, "capacityMegabytes": c, "request": q, "expect": scale(policy, q, r, c)} for c in (28000, 12000) for r in (3000, 4534, 10000, 25000, 40000) for q in (request, {**request, "fixedMegabytes": 5120, "perJobMegabytes": 0, "preferredJobs": 1}, {**request, "fixedMegabytes": 100, "perJobMegabytes": 100})]
    ramps = []
    for item in cases[:40]:
        estimate_case = item["expect"]
        base = request if rng.random() < 0.5 else {**request, "fixedMegabytes": 5120, "perJobMegabytes": 0, "preferredJobs": 1}
        scaled_case = scale(policy, base, estimate_case["ratioBasisPoints"], rng.choice([28000, 12000]))
        ramps.append({"base": base, "scaled": scaled_case, "estimate": estimate_case, "expect": ramp_request(base, scaled_case, estimate_case)})
    ramps += [{**item, "measured": True, "expect": ramp_request(item["base"], item["scaled"], item["estimate"], True)} for item in ramps[:16]]
    learns = [{"status": s, "peakRatioBasisPoints": r, "estimateRatioBasisPoints": e, "expect": learnable(s, r, e)} for s in ("ok", "failed", "cancelled") for r in (900, 3000, 3001, 10000, 25000) for e in (3000, 10000)]
    priorities = [{"base": b, "standing": c, "quick": q, "expect": priority(b, c, q)} for b in ("batch", "interactive") for c in (None, "agent", "background", "interactive") for q in (False, True)]
    store = {"version": 1, "classes": {}}
    for index in range(300):
        store = record(policy, store, ["op", f"op:k{index}"], {"operation": "op", "jobs": 1, "peakMegabytes": 100 + index, "defaultMegabytes": 1000, "activeMilliseconds": 10, "peakAtMilliseconds": 5, "endedAtMs": index})
    bounded = {"classes": len(store["classes"]), "oldestKept": min(max(s["atMs"] for s in v["samples"]) for v in store["classes"].values()), "rootSamples": len(store["classes"]["op"]["samples"])}
    return {"policy": POLICY, "cases": cases, "scales": scales, "ramps": ramps, "learns": learns, "priorities": priorities, "bounded": bounded}

if __name__ == "__main__":
    target = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧫️fixtures", "🔣️.json")
    value = generate()
    with open(target, "w", encoding="utf8") as handle:
        json.dump(value, handle, separators=(",", ":"), ensure_ascii=False)
        handle.write("\n")
    print("wrote", len(value["cases"]), "estimate cases,", len(value["scales"]), "scales")
