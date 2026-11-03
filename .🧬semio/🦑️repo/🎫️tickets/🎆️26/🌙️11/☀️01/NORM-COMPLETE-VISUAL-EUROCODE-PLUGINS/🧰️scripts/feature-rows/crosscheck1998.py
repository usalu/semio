"""EN 1998 third-party reproduction with scipy: exact T1 of the shear building (scipy.linalg.eigh) against the oracle Rayleigh period, and the base-shear utilisation of the cracked fixtures recomputed with the exact period and the spectrum plateau. Run: nice -n 19 .venv/bin/python -I crosscheck1998.py"""
import importlib.util, json, math, pathlib
import numpy as np
from scipy.linalg import eigh

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any")
spec = importlib.util.spec_from_file_location("o", ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py")
o = importlib.util.module_from_spec(spec)
spec.loader.exec_module(o)
for name in ("seismic-rayleigh-gross", "seismic-rayleigh-cracked", "seismic-cracked-fail"):
    doc = json.load(open(ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome" / name / "🔣️.json"))
    b = doc["buildings"][0]
    n = len(b["storeys"])
    masses = [o.weight(s, i + 1 == n) / o.G for i, s in enumerate(b["storeys"])]
    k = [o.stiffness_factor(b) * s["stiffnessX"] for s in b["storeys"]]
    stiff = np.zeros((n, n))
    for i in range(n):
        stiff[i, i] += k[i] + (k[i + 1] if i + 1 < n else 0)
        if i + 1 < n:
            stiff[i, i + 1] = stiff[i + 1, i] = -k[i + 1]
    exact = 2 * math.pi / math.sqrt(eigh(stiff, np.diag(masses), eigvals_only=True)[0])
    ray = o.rayleigh(masses, k)
    a_gr, s, tb, tc, td = o.site_parameters(doc)
    a_g = o.GAMMA_I[doc["site"]["importanceClass"]] * a_gr
    sy = b["systems"][0]
    q = min(max(sy["q0"] * sy["alphaUOverAlpha1"], 1.5), o.q_limit(sy))
    lam = 0.85 if exact <= 2 * tc and n > 2 else 1.0
    fb_exact = o.design_spectrum(a_g, s, tb, tc, td, q, exact) * sum(masses) * lam
    rows = {r["id"]: r["utilization"] for r in o.evaluate(doc)}
    u = rows["en1998.1.bldg-office.sys-x.baseShear"]
    ue = fb_exact / sy["baseShearResistanceN"]
    print(name, "T_rayleigh/T_eigh", round(ray / exact, 5), "baseShear oracle", round(u, 5), "with eigh period", round(ue, 5), "rel", round(ue / u - 1, 5))
    assert 0.97 <= ray / exact <= 1.0 and abs(ue / u - 1) < 0.05
