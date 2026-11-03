#!/usr/bin/env python3
"""EN 1993 v9: derives the check-outcome oracle documents for the feature-row gaps (6.2.4.n fail, 6.2.6.vy pass and fail, 1-8.3.9.slip pass and fail, 1-8.4.5.simplified pass) from `shear-member-fail` and prints the oracle rows. Usage: python3 -I en1993-v9-fixtures.py [--write]"""
import copy, importlib.util, json, pathlib, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
DOCS = ROOT / "🧪️tests/🧫️fixtures/📄️oracle-documents"
spec = importlib.util.spec_from_file_location("o", ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
base = json.loads((DOCS / "shear-member-fail/🔣️.json").read_text(encoding="utf-8"))


def derive(member_action, joints=()):
    doc = copy.deepcopy(base)
    doc["memberActions"][0]["action"] = {"n": 0, "vy": 0, "vz": 0, "my": 0, "mz": 0, "t": 0, **member_action}
    template = {j["kind"]: j for j in doc["joints"]}
    out = []
    for jid, kind, fields, shear in joints:
        j = copy.deepcopy(template[kind])
        j.update(fields)
        j["id"] = jid
        j["actions"] = [{"id": "jf-1", "loadCaseId": "g-permanent", "shear": shear, "tension": 0}]
        out.append(j)
    doc["joints"] = out
    return doc


DOCUMENTS = {
    "axial-member-fail": derive({"n": 4.8e6}),
    "shear-y-member": derive({"vy": 5.0e5}),
    "shear-y-member-fail": derive({"vy": 1.4e6}),
    "joint-slip-weld": derive({}, [
        ("joint-s1", "bolted", {"category": "B", "slipFactorKs": 1.0, "frictionSurfaces": 1, "frictionMu": 0.5, "preloadForce": 0}, 200000),
        ("joint-s2", "bolted", {"category": "B", "slipFactorKs": 1.0, "frictionSurfaces": 1, "frictionMu": 0.5, "preloadForce": 0}, 380000),
        ("joint-w2", "welded", {"weldThroat": 0.005, "weldLength": 0.2}, 100000),
    ]),
}

if __name__ == "__main__":
    for name, doc in DOCUMENTS.items():
        rows = {r["id"]: round(r["utilization"], 9) for r in oracle.evaluate(doc)["checks"]}
        print(name, json.dumps({k: v for k, v in rows.items() if any(t in k for t in ("6.2.4.n", "6.2.6.v", "3.9.slip", "4.5.simplified"))}))
        if "--write" in sys.argv:
            target = DOCS / name / "🔣️.json"
            target.parent.mkdir(exist_ok=True)
            target.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
