"""🎯️ Third-party oracle of the ground investigation derivation: characteristic c_u (CPT) and phi' (SPT) per layer with numpy stress integration and the exact Student t quantile of scipy.stats (EN 1997-1 2.4.5.2, EN 1997-2 Annex D, GI-03/GI-06/GI-07/GI-13), independent of the Rust formulation (`⚖️checks/🎯️ground-derivation`).
`python -I 🐍️.py <cases.json>` prints the derived values, `--self-check` asserts the blessed values of the fixture, `--bless` rewrites them from this oracle. The cone factor default is read from the shared registry row `ndp.en1997.2.cpt-nkt` (unverified), never from the fixture. The statistics run over the independent samples per investigation point (the mean of its readings inside the layer, registry row `ndp.en1997.2.characteristic-samples-per-sounding`, unverified), not over the autocorrelated readings; below 3 samples the characteristic value is the lowest reading."""
import json
import pathlib
import sys

import numpy as np
from scipy import stats

HERE = pathlib.Path(__file__).resolve().parent
NDP = next(p / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json" for p in HERE.parents if (p / "⚖️compliance" / "📜️annex-and-tables" / "🔖️ndp" / "🔣️.json").is_file())
FIXTURE = HERE.parents[1] / "🧪️tests" / "🧫️fixtures" / "🎯️ground-derivation" / "🔣️.json"
MIN_SAMPLES = 3


def registry_row(row_id, annex):
    row = next(e for e in json.loads(NDP.read_text(encoding="utf-8"))["entries"] if e["id"] == row_id)
    return row["naDe"] if annex.lower() == "de" else row["recommended"]


def registry_nkt(annex):
    return registry_row("ndp.en1997.2.cpt-nkt", annex)


def stresses(doc, depth):
    """Total and effective vertical stress [Pa] at `depth` by numeric integration of the layer unit weights (gamma above, gamma' below the water table, plus pore pressure)."""
    gw = 10000.0 if doc["annex"].lower() == "de" else 9810.0
    zs = np.linspace(0.0, depth, int(round(depth * 2000)) + 1)
    mid = 0.5 * (zs[1:] + zs[:-1])
    dz = np.diff(zs)
    eff = 0.0
    for z, h in zip(mid, dz):
        layer = next(l for l in doc["layers"] if l["depthTop"] <= z < l["depthBottom"])
        eff += (layer["gamma"] if z < doc["groundwaterLevel"] else layer["gammaPrime"]) * h
    return eff + gw * max(depth - doc["groundwaterLevel"], 0.0), eff


def layer_at(doc, depth):
    return next((l for l in doc["layers"] if l["depthTop"] <= depth < l["depthBottom"]), None)


def sounding_samples(readings, per_sounding):
    """Independent samples of one investigation point from its (depth, value) readings: `per_sounding` depth segments with equal reading count, each averaged (one segment = the spatial average over the layer thickness)."""
    ordered = sorted(readings)
    n, k = len(ordered), max(1, min(int(per_sounding), len(ordered)))
    return [float(np.mean([v for _, v in ordered[i * n // k:(i + 1) * n // k]])) for i in range(k)]


def characteristic(values, lowest, readings):
    v = np.asarray(values, dtype=float)
    n = len(v)
    if n >= MIN_SAMPLES:
        mean, s = v.mean(), v.std(ddof=1)
        variation = s / mean
        kn = stats.t.ppf(0.95, n - 1) * np.sqrt(1.0 / n)
        return dict(n=n, readings=readings, method="statistical", mean=mean, variation=variation, kn=kn, characteristic=min(mean * (1.0 - kn * variation), mean))
    return dict(n=n, readings=readings, method="cautious", mean=float(v.mean()), variation=0.0, kn=0.0, characteristic=float(lowest))


def derive(doc, layer_id, parameter):
    layer = next(l for l in doc["layers"] if l["id"] == layer_id)
    per_sounding = registry_row("ndp.en1997.2.characteristic-samples-per-sounding", doc["annex"])
    samples, every = [], []
    for inv in doc["investigations"]:
        values = []
        for r in inv["readings"]:
            if not (layer["depthTop"] <= r["depth"] < layer["depthBottom"]):
                continue
            total, eff = stresses(doc, r["depth"])
            if parameter == "undrainedStrength" and inv["kind"] == "cpt":
                nkt = layer["cptNkt"] if layer["cptNkt"] > 0 else registry_nkt(doc["annex"])
                if r["measurement"] - total > 0:
                    values.append((r["depth"], (r["measurement"] - total) / nkt))
            if parameter == "frictionAngle" and inv["kind"] == "spt":
                n1 = r["measurement"] * min(np.sqrt(100000.0 / eff), 1.7)
                values.append((r["depth"], np.sqrt(20.0 * n1) + 20.0))
            if parameter == "frictionAngle" and inv["kind"] == "cpt":
                values.append((r["depth"], 17.6 + 11.0 * np.log10((r["measurement"] / 100000.0) / np.sqrt(eff / 100000.0))))
        if values:
            samples.extend(sounding_samples(values, per_sounding))
            every.extend(v for _, v in values)
    return characteristic(samples, min(every), len(every)) if samples else None


def evaluate(case):
    doc = case["document"]
    return {f"{b['layerId']}.{b['parameter']}": derive(doc, b["layerId"], b["parameter"]) for b in doc["characteristicBases"]}


def blessed(result):
    return None if result is None else {k: (round(float(v), 9) if isinstance(v, (float, np.floating)) else v) for k, v in result.items()}


def main(argv):
    path = pathlib.Path(argv[1]) if len(argv) > 1 and not argv[1].startswith("--") else FIXTURE
    data = json.loads(path.read_text(encoding="utf-8"))
    computed = {c["id"]: {k: blessed(v) for k, v in evaluate(c).items()} for c in data["cases"]}
    if "--bless" in argv:
        for c in data["cases"]:
            c["expected"] = computed[c["id"]]
        path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print("blessed", len(data["cases"]), "cases")
    elif "--self-check" in argv:
        for c in data["cases"]:
            for key, want in c["expected"].items():
                got = computed[c["id"]][key]
                assert (want is None) == (got is None), (c["id"], key)
                if want is not None:
                    assert got["method"] == want["method"] and got["n"] == want["n"], (c["id"], key, got, want)
                    for field in ("mean", "variation", "kn", "characteristic"):
                        assert abs(got[field] - want[field]) <= 1e-6 * max(1.0, abs(want[field])), (c["id"], key, field, got[field], want[field])
        print("self-check ok", len(data["cases"]), "cases")
    else:
        print(json.dumps(computed, indent=2))


if __name__ == "__main__":
    main(sys.argv)
