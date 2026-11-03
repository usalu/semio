"""EN 1993 Chs and Angle section kinds (v19): fixtures derived from the committed showcase (a tower leg member of kind Angle or Chs, section values from the third-party `sectionproperties` finite-element analysis, sharp corners) and their tower rows from the independent oracle `check-outcome/🐍️.py`. `--write` writes the fixtures and the feature rows."""
import copy, json, pathlib, re, subprocess, sys

from sectionproperties.analysis import Section
from sectionproperties.pre.library import angle_section, circular_hollow_section

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures"
DOCS = FIX / "📄️oracle-documents"
FEATURE = ROOT / "🧪️tests/🚦️check-outcome-en1993-1/🥒️.feature"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
showcase = json.load(open(FIX / "🎪️showcase/🔣️.json"))
S235 = {"id": "mat-s235", "grade": "S235", "fy": 235e6, "fu": 360e6, "eModulus": 210e9, "gModulus": 81e9, "subgrade": "J2", "kind": "Carbon", "rambergOsgoodN": 0.0}


def solve(geometry, mesh):
    geometry.create_mesh(mesh_sizes=[mesh])
    s = Section(geometry)
    s.calculate_geometric_properties()
    s.calculate_warping_properties()
    s.calculate_plastic_properties()
    return s


def angle_row(h, b, t):
    s = solve(angle_section(d=h * 1e3, b=b * 1e3, t=t * 1e3, r_r=0, r_t=0, n_r=1), t * 1e3 / 8)
    i11, i22 = s.get_ip()
    zp, sp = s.get_zp(), s.get_sp()
    return {"id": "sec-new", "designation": f"L {h * 1e3:g}x{b * 1e3:g}x{t * 1e3:g}", "kind": "Angle", "h": h, "b": b, "tw": t, "tf": t, "r": 0.0, "area": s.get_area() * 1e-6, "shearAreaY": b * t, "shearAreaZ": h * t, "iy": i11 * 1e-12, "iz": i22 * 1e-12, "it": s.get_j() * 1e-12, "iw": 0.0, "wElY": min(abs(zp[0]), abs(zp[1])) * 1e-9, "wElZ": min(abs(zp[2]), abs(zp[3])) * 1e-9, "wPlY": sp[0] * 1e-9, "wPlZ": sp[1] * 1e-9, "areaNet": 0.9 * s.get_area() * 1e-6}


def chs_row(d, t):
    s = solve(circular_hollow_section(d=d * 1e3, t=t * 1e3, n=128), t * 1e3 / 3)
    ix, _, _ = s.get_ic()
    wel, wpl = s.get_z()[0], s.get_s()[0]
    return {"id": "sec-new", "designation": f"CHS {d * 1e3:g}x{t * 1e3:g}", "kind": "Chs", "h": d, "b": d, "tw": t, "tf": t, "r": 0.0, "area": s.get_area() * 1e-6, "shearAreaY": 2 * s.get_area() * 1e-6 / 3.141592653589793, "shearAreaZ": 2 * s.get_area() * 1e-6 / 3.141592653589793, "iy": ix * 1e-12, "iz": ix * 1e-12, "it": s.get_j() * 1e-12, "iw": 0.0, "wElY": wel * 1e-9, "wElZ": wel * 1e-9, "wPlY": wpl * 1e-9, "wPlZ": wpl * 1e-9, "areaNet": 0.9 * s.get_area() * 1e-6}


def tower(section, material, length, n_tower, my, mz, shape, params):
    d = copy.deepcopy(showcase)
    if material["id"] not in {m["id"] for m in d["materials"]}:
        d["materials"].append(material)
    d["sections"].append(section)
    member = copy.deepcopy(next(m for m in d["members"] if m["id"] == "member-b1"))
    member.update({"id": "member-new", "label": "Tower leg", "memberType": "Column", "sectionId": "sec-new", "materialId": material["id"], "length": length, "bucklingLengthY": length, "bucklingLengthZ": length, "ltbLength": length, "ltbRestraintSpacing": 0.0})
    d["members"].append(member)
    d["memberActions"].append({"id": "act-new-g", "memberId": "member-new", "loadCaseId": "g-permanent", "action": {"n": n_tower, "vy": 0.0, "vz": 0.0, "my": my, "mz": mz, "t": 0.0}})
    leg = copy.deepcopy(d["towerLegs"][0])
    leg.update({"id": "tw-new", "memberId": "member-new", "forceCoefficient": 1.0, "dynamicFactor": 1.0, "actions": [{"id": "tw-new-g", "loadCaseId": "g-permanent", "force": n_tower}]})
    d["towerLegs"].append(leg)
    d["parametricSections"].append({"id": "param-new", "sectionId": "sec-new", "label": "Parametric " + section["designation"], "shape": shape, "height": params[0], "width": params[1], "webThickness": params[2], "flangeThickness": 0.0, "tolerance": 0.05})
    return d


FIXTURES = {
    "angle-tower-leg": tower(angle_row(0.1, 0.1, 0.01), S235, 3.0, 80e3, 1000.0, 500.0, "Angle", (0.1, 0.1, 0.01)),
    "angle-tower-leg-unequal": tower(angle_row(0.15, 0.09, 0.012), S235, 2.5, 100e3, 1500.0, 700.0, "Angle", (0.15, 0.09, 0.012)),
    "angle-tower-leg-class4": tower(angle_row(0.1, 0.1, 0.01), copy.deepcopy(showcase["materials"][0]), 3.0, 80e3, 1000.0, 500.0, "Angle", (0.1, 0.1, 0.01)),
    "angle-tower-leg-fail": tower(angle_row(0.1, 0.1, 0.01), S235, 3.0, 300e3, 5000.0, 2500.0, "Angle", (0.1, 0.1, 0.01)),
    "chs-tower-leg": tower(chs_row(0.1683, 0.0063), copy.deepcopy(showcase["materials"][0]), 5.0, 250e3, 8000.0, 3000.0, "Chs", (0.1683, 0.1683, 0.0063)),
    "chs-tower-leg-class3": tower(chs_row(0.1683, 0.0029), copy.deepcopy(showcase["materials"][0]), 5.0, 100e3, 3000.0, 1000.0, "Chs", (0.1683, 0.1683, 0.0029)),
    "chs-tower-leg-fail": tower(chs_row(0.1683, 0.0063), copy.deepcopy(showcase["materials"][0]), 5.0, 450e3, 30000.0, 12000.0, "Chs", (0.1683, 0.1683, 0.0063)),
}
text = FEATURE.read_text(encoding="utf8")
n = max(int(x) for x in re.findall(r"\|\s*(?:pass|fail)-(?:v\d+-)?(\d+)\s*\|", text))
rows = []
for name, doc in FIXTURES.items():
    if "--write" in sys.argv:
        (DOCS / name).mkdir(exist_ok=True)
        (DOCS / name / "🔣️.json").write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
    out = json.loads(subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True, check=True).stdout)
    for c in (c for c in out["checks"] if c["id"].startswith("en1993.3-1.tower") and c["id"].endswith("tw-new")):
        n += 1
        status = c["status"]
        print(name, c["id"], status, round(c["utilization"], 5))
        if f"| shared://📄️oracle-documents/{name}/🔣️.json | {c['id']} |" not in text:
            rows.append(f"      | {'fail' if status == 'fail' else 'pass'}-v19-{n:02d} | shared://📄️oracle-documents/{name}/🔣️.json | {c['id']} | {status} | {c['utilization']:.9g} | 1e-3 |")
print(len(rows), "new rows")
if "--write" in sys.argv and rows:
    FEATURE.write_text(text.rstrip("\n") + "\n" + "\n".join(rows) + "\n", encoding="utf8")
