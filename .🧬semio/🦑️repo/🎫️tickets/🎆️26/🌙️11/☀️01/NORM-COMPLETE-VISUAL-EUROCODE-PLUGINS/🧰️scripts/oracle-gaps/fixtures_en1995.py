"""🧫 Writes the EN 1995 check-outcome fixtures `tcc-beam-pass|fail` (design-cases-pass plus a T-90 timber-concrete composite beam: 1000 x 80 concrete flange screwed to a 200 x 240 C24 web, spacing 150 mm, K 15 kN/mm, span 5 m) and `entities-pass|fail` (the showcase diaphragms, truss and bracing of T-94, T-95, T-96 with raised or cut resistances)."""
import json, pathlib
root = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🚦️check-outcome")
for name, moment, shear, load in (("tcc-beam-pass", 30e6, 25e3, 6.0), ("tcc-beam-fail", 90e6, 100e3, 22.0)):
    d = json.load(open(root / "design-cases-pass/🔣️.json", encoding="utf8"))
    numbers = dict(a1=80000.0, i1=1000 * 80**3 / 12, e1=30000.0, h1=80.0, s1=150.0, a2=48000.0, i2=200 * 240**3 / 12, e2=11000.0, h2=240.0, k=15000.0, l=5000.0, m_d=moment, v_d=shear, q_d=load, fd_a1=14.0, fd_m1=14.0, fd_a2=9.0, fd_m2=14.0, fv_d_2=1.6, r_d1=20000.0)
    d["cases"].append({"id": "tcc", "labelEn": "TCC", "labelDe": "HBV", "entryId": "T-90", "numbers": [{"id": k, "value": v} for k, v in numbers.items()], "texts": [{"id": "class", "value": "C24"}]})
    out = root / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as handle:
        json.dump(d, handle, ensure_ascii=False, indent=2)
        handle.write("\n")

for name, fail in (("entities-pass", False), ("entities-fail", True)):
    d = json.load(open(root.parent / "🎪️showcase/🔣️.json", encoding="utf8"))
    d["bracings"][0]["resistanceN"] = 8000.0
    if fail:
        d["diaphragms"][0].update(shearEdN=60000.0, holdDownN=15000.0)
        d["diaphragms"][1]["diaphragmForceN"] *= 4.0
        d["trusses"][0]["bracingCapacityNPerM"] = 60.0
        d["bracings"][0].update(resistanceN=2000.0, stiffnessNPerM=100000.0)
    out = root / name
    out.mkdir(exist_ok=True)
    with open(out / "🔣️.json", "w", encoding="utf8") as handle:
        json.dump(d, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
