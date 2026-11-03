"""EN 1992 feature-rows fixtures: derived-load beams for 7.3 crack, 7.4 deflection, 7.4 l/d, 6.2 shear and 6.1 accidental flexure. Run: .venv/bin/python -I gen1992.py [--write]"""
import copy, json, pathlib, subprocess, sys

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any")
FIX = ROOT / "🧪️tests/🧫️fixtures/🚦️check-outcome"
ORACLE = ROOT / "🔮️oracles/🚦️check-outcome/🐍️.py"
base = json.load(open(FIX / "prestressed-wall-pass/🔣️.json"))
base["anchors"] = []


def action(i, kind, cat, g=0.0, q=0.0, v=0.0, m=0.0, source="Udl"):
    return {"id": i, "kind": kind, "category": cat, "source": source, "gKLine": g, "qKLine": q, "pointForce": 0.0, "mK": m, "nK": 0.0, "vK": v, "tK": 0.0, "vKPunch": 0.0}


def beam(title, span=6.0, g=20e3, q=15e3, count=4, dia=0.025, stirrup=(0.008, 0.15), extra=(), exposure="Xc1", support="SimplySupported"):
    d = copy.deepcopy(base)
    d["title"] = title
    m = d["members"][0]
    m.update(id="beam-B1", labelEn="Beam B1", labelDe="Balken B1", kind="Beam", prestressSteelId="", prestress=None, tendonPoints=[], exposure=exposure, width=0.3, height=0.55, effectiveDepth=0.5, cover=0.04, span=span, support=support, useFem=False, udl=0.0, relativeHumidity=0.5)
    m["longitudinal"] = [dict(m["longitudinal"][0], id="bot", count=count, diameter=dia)]
    m["stirrups"] = {"diameter": stirrup[0], "spacing": stirrup[1], "legs": 2}
    m["actions"] = [action("G", "Permanent", "self", g=g), action("Q", "Imposed", "office", q=q)] + list(extra)
    return d


FIXTURES = {
    "beam-sls-pass": beam("Simply supported beam within crack, deflection and l/d limits"),
    "beam-ld-fail": beam("Long beam above the span-to-depth limit", span=16.0, g=4e3, q=3e3),
    "beam-deflection-fail": beam("Slender loaded beam above the deflection limit", span=9.0, g=22e3, q=22e3, count=6, dia=0.025),
    "beam-shear-fail": beam("Short heavily loaded beam above the shear resistance", span=3.0, g=100e3, q=70e3, stirrup=(0.006, 0.3)),
    "beam-minas-fail": beam("Lightly loaded beam below the minimum reinforcement", g=3e3, q=2e3, count=2, dia=0.012),
    "beam-maxas-fail": beam("Beam above the maximum reinforcement", count=16, dia=0.025),
    "beam-accidental-pass": beam("Beam with an accidental design situation within resistance", extra=[action("A", "Accidental", "accidental", source="External", m=60e3)]),
    "beam-accidental-fail": beam("Beam with an accidental design situation above resistance", extra=[action("A", "Accidental", "accidental", source="External", m=600e3)]),
}

if __name__ == "__main__":
    for name, doc in FIXTURES.items():
        path = FIX / name / "🔣️.json"
        if "--write" in sys.argv:
            path.parent.mkdir(exist_ok=True)
            path.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
        out = subprocess.run([sys.executable, "-I", str(ORACLE), "--json"], input=json.dumps(doc), capture_output=True, text=True)
        print(name, out.stdout.strip()[:900], out.stderr[-300:])
