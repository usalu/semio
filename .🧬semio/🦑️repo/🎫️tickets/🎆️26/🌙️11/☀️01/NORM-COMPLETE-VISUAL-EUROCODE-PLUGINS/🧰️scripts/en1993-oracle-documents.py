"""Committed oracle documents of the EN 1993 `check-outcome` case (class 4 bending, lateral-torsional buckling and the member interaction 6.3.3): python -I en1993-oracle-documents.py [--write]

Each document is the before-snapshot of `change-material-e-modulus` with a slender welded I section (class 4: web in compression, thin flanges), S355 and hand-chosen member data and permanent actions, so every row stays a plain committed snapshot. Without --write the independent oracle prints the utilisations of the interesting rows; with --write the documents land in `🧪️tests/🧫️fixtures/📄️oracle-documents/<name>/🔣️.json` of the artifact (`shared://📄️oracle-documents/<name>/🔣️.json`).
"""
import copy, importlib.util, json, pathlib, sys

SUBSET = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
BASE = next((SUBSET / "🧪️tests/🧫️fixtures/🧬️mutations").glob("*change-material-e-modulus/*apply/*snapshot/*before/*.json"))
OUT = SUBSET / "🧪️tests/🧫️fixtures/📄️oracle-documents"
spec = importlib.util.spec_from_file_location("oracle", SUBSET / "🔮️oracles/🚦️check-outcome/🐍️.py")
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)
base = json.loads(BASE.read_text(encoding="utf8"))


def welded_i(sid, designation, h, b, tw, tf):
    hw = h - 2 * tf
    iy = (b * h**3 - (b - tw) * hw**3) / 12
    iz = (2 * tf * b**3 + hw * tw**3) / 12
    area = 2 * b * tf + hw * tw
    return dict(id=sid, designation=designation, kind="weldedI", h=h, b=b, tw=tw, tf=tf, r=0.0, area=area, shearAreaY=2 * b * tf, shearAreaZ=hw * tw, iy=iy, iz=iz,
                it=(2 * b * tf**3 + hw * tw**3) / 3, iw=iz * (h - tf) ** 2 / 4, wElY=iy / (h / 2), wElZ=iz / (b / 2), wPlY=b * tf * (h - tf) + tw * hw**2 / 4, wPlZ=tf * b**2 / 2 + hw * tw**2 / 4, areaNet=area)


SLENDER = welded_i("sec-slender", "Welded I 600x200x6x8", 0.60, 0.20, 0.006, 0.008)
S355 = dict(id="mat-s355", grade="S355", fy=355.0e6, fu=490.0e6, eModulus=210.0e9, gModulus=81.0e9, subgrade="J2", kind="carbon", rambergOsgoodN=0.0)


def variant(section, member, action):
    doc = copy.deepcopy(base)
    doc["sections"].append(copy.deepcopy(section))
    doc["materials"].append(copy.deepcopy(S355))
    doc["members"][0].update(sectionId=section["id"], materialId="mat-s355", **member)
    doc["memberActions"][0]["action"].update(action)
    return doc


BEAM = dict(memberType="beam", length=6.0, bucklingLengthY=6.0, bucklingLengthZ=6.0, ltbLength=2.0, ltbRestraintSpacing=2.0, endMomentRatioPsi=0.0, momentDiagram="parabolic", loadApplication="shearCenter")
COLUMN = dict(memberType="beamColumn", length=4.0, bucklingLengthY=4.0, bucklingLengthZ=4.0, ltbLength=4.0, ltbRestraintSpacing=4.0, endMomentRatioPsi=0.0, momentDiagram="linear", loadApplication="shearCenter")
HEB = dict(COLUMN, bucklingLengthY=4.0, bucklingLengthZ=4.0)

DOCS = {
    "class4-beam": lambda: variant(SLENDER, BEAM, dict(n=0.0, vz=50e3, my=200e3, mz=0.0)),
    "class4-beam-fail": lambda: variant(SLENDER, BEAM, dict(n=0.0, vz=60e3, my=520e3, mz=0.0)),
    "class4-beam-column": lambda: variant(SLENDER, COLUMN, dict(n=250e3, vz=15e3, my=70e3, mz=0.0)),
    "class4-beam-column-fail": lambda: variant(SLENDER, COLUMN, dict(n=450e3, vz=25e3, my=150e3, mz=0.0)),
    "class1-beam-column": lambda: (lambda d: (d["members"][0].update(**HEB), d["memberActions"][0]["action"].update(n=700e3, vz=30e3, my=110e3, mz=0.0), d)[-1])(copy.deepcopy(base)),
    "class1-beam-column-fail": lambda: (lambda d: (d["members"][0].update(**HEB), d["memberActions"][0]["action"].update(n=1800e3, vz=70e3, my=330e3, mz=0.0), d)[-1])(copy.deepcopy(base)),
}
SHOWN = ("en1993.6.2.4.n", "en1993.6.2.5.my", "en1993.6.2.9.mn", "en1993.6.3.1.nb", "en1993.6.3.2.mb", "en1993.6.3.3.intb")

if __name__ == "__main__":
    for name, make in DOCS.items():
        doc = make()
        rows = {r["id"]: r["utilization"] for r in oracle.evaluate(doc)["checks"]}
        print(name, "class", oracle.section_class(next(s for s in doc["sections"] if s["id"] == doc["members"][0]["sectionId"]), next(m for m in doc["materials"] if m["id"] == doc["members"][0]["materialId"])["fy"]))
        for rid in SHOWN:
            if rid in rows:
                print("   %-28s %.6f" % (rid, rows[rid]))
        if "--write" in sys.argv:
            target = OUT / name / "🔣️.json"
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
