"""📇️ Registers the third-party oracle claims: PyNiteFEA and anastruct in the norm root registry, one `enXXXX-1-<library>` entry per artifact for every library its case files use (idempotent).

Usage: /home/deck/semio/.venv/bin/python -I oracle-third-party-register.py
"""

import json
from pathlib import Path

NORM = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
LEAF = "🏅️standards/🔖️1/🪆️subsets/✳️any"
PROFILES = ["eurocode-quantities-v1", "numerical-v1", "method-spread-v1"]
LIBS = {
    "structuralcodes": ("norm-eurocode-structuralcodes", "structuralcodes", "0.7.3", "MIT", "https://github.com/fib-international/structuralcodes", "fib structuralcodes EN 1992-1-1:2004 and fib Model Code implementation"),
    "concreteproperties": ("norm-eurocode-concreteproperties", "concreteproperties", "0.8.0", "MIT", "https://github.com/robbievanleeuwen/concrete-properties", "concreteproperties fibre and finite element reinforced concrete section analysis"),
    "sectionproperties": ("norm-eurocode-sectionproperties", "sectionproperties", "3.10.2", "MIT", "https://github.com/robbievanleeuwen/section-properties", "sectionproperties finite element cross-section analysis"),
    "blueprints": ("norm-eurocode-blue-prints", "blue-prints", "0.8.0", "MIT", "https://github.com/Blueprints-org/blueprints", "Blueprints Eurocode formula library"),
    "groundhog": ("norm-eurocode-groundhog", "groundhog", "0.15.0", "GPL-3.0-only", "https://github.com/snakesonabrain/groundhog", "groundhog geotechnical formulas"),
    "sympy": ("norm-eurocode-sympy", "sympy", "1.14.0", "BSD-3-Clause", "https://www.sympy.org", "SymPy symbolic algebra"),
    "pynite": ("norm-eurocode-pynite", "PyNiteFEA", "3.0.0", "MIT", "https://github.com/JWock82/Pynite", "PyNiteFEA 3D finite element frame solver"),
    "anastruct": ("norm-eurocode-anastruct", "anastruct", "1.7.0", "GPL-3.0-only", "https://anastruct.readthedocs.io", "anastruct 2D direct-stiffness frame solver"),
}
USES = {
    "en1990": {"sympy": "reliability index beta = -Phi^-1(P_f) with the exact inverse error function"},
    "en1992": {"structuralcodes": "Table 3.1 laws, V_Rd,c, V_Rd,max, V_Rd,s, crack spacing and width, section M_Rd and M-N points", "concreteproperties": "section M_Rd, M-N points and cracked-section properties and stresses by fibre integration", "blueprints": "clause formulas 3.15, 5.13N, 5.14, 5.34, 6.2, 6.6N, 6.30, 6.38, 7.8, 7.9, 7.11, 7.16, 8.3", "pynite": "linear elastic propped cantilever internal forces", "anastruct": "linear elastic propped cantilever internal forces"},
    "en1993": {"sectionproperties": "FE section constants A, I, J, I_w, W_el, W_pl and shear centre of I, RHS, CHS, channel and angle sections, and M_cr inputs", "blueprints": "clause formulas 5.2, 6.6, 6.13, 6.18 with shear areas, 6.30, 6.36, 6.55", "sympy": "symbolic Ayrton-Perry root chi and Euler load", "pynite": "portal sway, base reactions and beam deflection", "anastruct": "portal sway, base reactions and beam deflection"},
    "en1994": {"pynite": "two-span beam load arrangements: reactions, support and span moments", "anastruct": "two-span beam load arrangements: reactions, support and span moments"},
    "en1995": {"blueprints": "clause formula 7.5 natural frequency", "pynite": "two-span timber beam load arrangements and deflection", "anastruct": "two-span timber beam load arrangements and deflection"},
    "en1997": {"groundhog": "Prandtl N_q, N_c, Vesic-derived N_gamma, Rankine and Coulomb earth pressure, CPT friction angle, undrained strength and soil behaviour index"},
}


def entry(art, key, use):
    oid, package, version, spdx, home, impl = LIBS[key]
    return {"id": f"{art}-1-{key}", "kind": "third-party-library", "ecosystem": "python", "package": package, "version": version, "capabilities": [f"{art}-1-third-party-cases"], "comparisonProfiles": PROFILES, "license": spdx, "testOnly": True, "homepage": home, "rationale": f"Reference values of the {art.upper()} oracle case files (`🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party*.json`) are computed by {package} {version}: {use}. The harness adapter `✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness/🔌️adapters/{key}/🐍️.py` is the only importer; values are blessed from the library, never from the Rust. National Annex parameters are inputs of the cases.", "engine": {"family": key, "implementation": impl, "version": version}, "productionReachable": False, "networkDuringExecution": False}


def write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main():
    for art, libs in USES.items():
        folder = next((NORM / "🗿️artifacts").glob(f"*{art}")) / LEAF / "🔮️oracles" / "🔣️.json"
        data = json.loads(folder.read_text(encoding="utf-8"))
        have = {o["id"] for o in data["oracles"]}
        data["oracles"] += [entry(art, k, u) for k, u in libs.items() if f"{art}-1-{k}" not in have]
        write(folder, data)
    root = NORM / "🔮️oracles" / "🔣️.json"
    data = json.loads(root.read_text(encoding="utf-8"))
    have = {o["id"] for o in data["oracles"]}
    for key, capability, rationale, companions in (("pynite", "norm-eurocode-frame-analysis", "Independent 3D finite element frame solver: internal forces, reactions and deflections of beams and portals that validate the mechanics kernel and the load arrangement coefficients of EN 1990-1999.", [{"package": "numpy", "version": "locked", "license": "BSD-3-Clause", "role": "Linear algebra backend of PyNiteFEA."}]), ("anastruct", "norm-eurocode-frame-analysis", "Independent 2D direct-stiffness frame solver (GPL-3.0, test-only, never linked into shipped code) for the same beam and portal cases as PyNiteFEA.", [])):
        if LIBS[key][0] in have:
            continue
        oid, package, version, spdx, home, impl = LIBS[key]
        item = {"id": oid, "kind": "third-party-library", "ecosystem": "python", "package": package, "version": version, "capabilities": [capability], "comparisonProfiles": ["eurocode-quantities-v1", "numerical-v1"], "license": spdx, "testOnly": True, "homepage": home, "rationale": rationale, "hostPath": "✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️harness", "engine": {"family": key, "implementation": impl, "version": version}, "productionReachable": False, "networkDuringExecution": False}
        if companions:
            item["packages"] = companions
        data["oracles"].append(item)
    write(root, data)


main()
