"""📇️ Registers the oracle claims of every Eurocode artifact from its case files: one `enXXXX-1-<library>` entry per adapter used (SciPy included), rationale listing the check ids, plus the SciPy root claim (idempotent, replaces earlier entries of the same id).

Usage: /home/deck/semio/.venv/bin/python -I oracle-wave2-register.py
"""

import json
from collections import defaultdict
from pathlib import Path

NORM = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
LEAF = "🏅️standards/🔖️1/🪆️subsets/✳️any"
PROFILES = ["eurocode-quantities-v1", "numerical-v1", "method-spread-v1", "table-exact-v1"]
LIBS = {
    "structuralcodes": ("structuralcodes", "0.7.3", "MIT", "https://github.com/fib-international/structuralcodes", "fib structuralcodes EN 1992-1-1:2004 and fib Model Code implementation"),
    "concreteproperties": ("concreteproperties", "0.8.0", "MIT", "https://github.com/robbievanleeuwen/concrete-properties", "concreteproperties fibre and finite element reinforced concrete section analysis"),
    "sectionproperties": ("sectionproperties", "3.10.2", "MIT", "https://github.com/robbievanleeuwen/section-properties", "sectionproperties finite element cross-section analysis"),
    "blueprints": ("blue-prints", "0.8.0", "MIT", "https://github.com/Blueprints-org/blueprints", "Blueprints Eurocode formula library"),
    "groundhog": ("groundhog", "0.15.0", "GPL-3.0-only", "https://github.com/snakesonabrain/groundhog", "groundhog geotechnical formulas"),
    "sympy": ("sympy", "1.14.0", "BSD-3-Clause", "https://www.sympy.org", "SymPy symbolic algebra"),
    "pynite": ("PyNiteFEA", "3.0.0", "MIT", "https://github.com/JWock82/Pynite", "PyNiteFEA 3D finite element frame solver"),
    "anastruct": ("anastruct", "1.7.0", "GPL-3.0-only", "https://anastruct.readthedocs.io", "anastruct 2D direct-stiffness frame solver"),
    "numerical": ("scipy", "1.18.0", "BSD-3-Clause", "https://scipy.org", "SciPy quadrature, root finding, constrained minimisation and generalized eigenproblems on NumPy"),
}
KEY = {"numerical": "scipy"}


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    for folder in sorted((NORM / "🗿️artifacts").glob("*")):
        art = folder.name[-6:]
        if not art.startswith("en19"):
            continue
        used = defaultdict(set)
        for file in sorted((folder / LEAF / "🧪️tests/🧫️fixtures/🔮️oracle-cases").glob("*.json")):
            for c in json.loads(file.read_text(encoding="utf-8"))["cases"]:
                for r in c["references"]:
                    used[r["adapter"]].add(c["check"])
        registry = folder / LEAF / "🔮️oracles" / "🔣️.json"
        data = json.loads(registry.read_text(encoding="utf-8"))
        ids = {f"{art}-1-{KEY.get(a, a)}" for a in used}
        data["oracles"] = [o for o in data["oracles"] if o["id"] not in ids]
        for adapter, checks in sorted(used.items()):
            if adapter not in LIBS:
                continue
            package, version, spdx, home, impl = LIBS[adapter]
            key = KEY.get(adapter, adapter)
            engine = "independent formulations (numerical integration, root finding, eigenproblems and closed-form derivations that never reuse the Rust expression)" if adapter == "numerical" else "reference values"
            data["oracles"].append({"id": f"{art}-1-{key}", "kind": "third-party-library", "ecosystem": "python", "package": package, "version": version, "capabilities": [f"{art}-1-third-party-cases"], "comparisonProfiles": PROFILES, "license": spdx, "testOnly": True, "homepage": home, "rationale": f"{engine.capitalize()} of the {art.upper()} oracle case files (`🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party*.json`) for the checks {', '.join(sorted(checks))} are computed by {package} {version}. The harness adapter `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🔌️adapters/{adapter}/🐍️.py` is the only importer; values are blessed from the library, never from the Rust. National Annex parameters are inputs of the cases.", "engine": {"family": key, "implementation": impl, "version": version}, "productionReachable": False, "networkDuringExecution": False})
        write(registry, data)
    root = NORM / "🔮️oracles" / "🔣️.json"
    data = json.loads(root.read_text(encoding="utf-8"))
    if not any(o["id"] == "norm-eurocode-scipy-numerics" for o in data["oracles"]):
        package, version, spdx, home, impl = LIBS["numerical"]
        data["oracles"].append({"id": "norm-eurocode-scipy-numerics", "kind": "third-party-library", "ecosystem": "python", "package": package, "version": version, "capabilities": ["norm-eurocode-numerical-formulations"], "comparisonProfiles": PROFILES, "license": spdx, "testOnly": True, "homepage": home, "rationale": "Independent formulations of Eurocode clauses no Eurocode library covers: strain-compatibility flexure by quadrature, Ayrton-Perry roots, Euler eigenproblems, FORM design points, EN 1990 combination enumeration, composite plastic neutral axes, shear-building eigenproblems, Janssen slice equilibrium, Boussinesq integration and influence functions. Reference quantities are computed with SciPy, never with the Rust expressions.", "hostPath": "✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness", "engine": {"family": "scipy", "implementation": impl, "version": version}, "productionReachable": False, "networkDuringExecution": False})
        write(root, data)


main()
