"""🚦️ Independent EN 1997 document oracle for the check-outcome case: a committed snapshot (JSON on stdin or a path) to the utilisation of every geotechnical check the compliance oracle reproduces (`--json`, default: `{"checks":[{"id","utilization"}]}`; `--self-check`: the bearing-capacity factors against groundhog, the Boussinesq stress under a rectangle corner against groundhog, the pair-settlement plane fit against numpy). The document evaluation is `../⚖️compliance/🐍️.py` `evaluate` (bearing 6.5, sliding, eccentricity, settlement and pair rows 6.6, piles 7.6, anchors, retaining walls and overturning 9, uplift and hydraulic failure 10, slope stability 11, investigation 2) with its stdlib library `📚️geo.py` of EN 1997-1 Annex D, DIN 4017 and DIN 4085, written from the clauses and not from the Rust. Rows with a status but no utilisation in the Rust (`9.earthPressure`, `6.5.bearing` below geotechnical category 2) have no row here."""

import importlib.util
import json
import math
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
BASE_PATH = HERE.parent / "⚖️compliance" / "🐍️.py"


def base():
    spec = importlib.util.spec_from_file_location("en1997_compliance", BASE_PATH)
    module = importlib.util.module_from_spec(spec)
    sys.modules["en1997_compliance"] = module
    spec.loader.exec_module(module)
    return module


def derivation():
    """🎯 The ground-derivation oracle (per-sounding samples, exact Student t of scipy.stats), loaded by path."""
    spec = importlib.util.spec_from_file_location("en1997_ground_derivation", HERE.parent / "🎯️ground-derivation" / "🐍️.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def resolved(doc):
    """🧱 The document the checks run on: a layer parameter with a derived basis carries the characteristic value of its soundings (0 when nothing can be derived); returns it with the `en1997.2.characteristic.<cu|phi>.<layer>` rows (minimum of 3 independent samples, so 3 / n; none without a sample)."""
    bases = [b for b in doc.get("characteristicBases", []) if b["basis"] == "derived" and any(l["id"] == b["layerId"] for l in doc["layers"])]
    if not bases:
        return doc, []
    ground, out, rows = derivation(), json.loads(json.dumps(doc)), []
    for b in bases:
        result = ground.derive(doc, b["layerId"], b["parameter"])
        layer = next(l for l in out["layers"] if l["id"] == b["layerId"])
        layer["cohesionUndrained" if b["parameter"] == "undrainedStrength" else "phiPrimeDeg"] = 0.0 if result is None else float(result["characteristic"])
        if result is not None:
            rows.append({"id": f"en1997.2.characteristic.{'cu' if b['parameter'] == 'undrainedStrength' else 'phi'}.{b['layerId']}", "utilization": max(3.0 / result["n"], 1.0 if result["n"] < 3 else 0.0)})
    return out, rows


def evaluate(doc):
    ground, rows = resolved(doc)
    return [{"id": row["id"], "utilization": row["utilization"]} for row in base().evaluate(ground)["checks"]] + rows


def self_check():
    """🧪 Nq and the Vesic N_gamma of groundhog against EN 1997-1 Annex D N_gamma = 2 (N_q - 1) tan(phi), and the corner stress of a rectangle against groundhog's Newmark solution superposed to the centre (the plotting package groundhog imports is stubbed, nothing is drawn)."""
    from unittest import mock

    for _ in range(40):
        try:
            from groundhog.shallowfoundations.capacity import nq_frictionangle_sand, ngamma_frictionangle_vesic
            from groundhog.shallowfoundations.stressdistribution import stresses_rectangle

            break
        except ModuleNotFoundError as error:
            sys.modules[error.name] = mock.MagicMock()
            for known in [m for m in sys.modules if m.startswith("groundhog")]:
                del sys.modules[known]

    module = base()
    geo = module.G
    out = {}
    for phi in (22.0, 30.0, 35.0, 40.0):
        nq_ref = nq_frictionangle_sand(phi)["Nq [-]"]
        vesic = ngamma_frictionangle_vesic(phi)["Ngamma [-]"]
        assert abs(geo.nq(phi) / nq_ref - 1) < 1e-9, (phi, geo.nq(phi), nq_ref)
        assert abs((vesic - geo.ng(phi)) - 4 * math.tan(math.radians(phi))) < 1e-6, (phi, vesic, geo.ng(phi))
        out[f"nq_{int(phi)}"] = geo.nq(phi)
    b, l, z, q = 2.0, 3.0, 2.5, 100.0
    centre = 4 * stresses_rectangle(q, l / 2, b / 2, z)["delta sigma z [kPa]"]
    ours = geo.sigma_z_rect_centre(q, b, l, z)
    assert abs(ours / centre - 1) < 1e-3, (ours, centre)
    out["sigma_z_centre"] = ours
    return out


if __name__ == "__main__":
    if "--self-check" in sys.argv:
        print(json.dumps(self_check()))
    else:
        paths = [a for a in sys.argv[1:] if not a.startswith("--")]
        text = open(paths[0], encoding="utf8").read() if paths else sys.stdin.read()
        print(json.dumps({"checks": evaluate(json.loads(text))}, ensure_ascii=False))
