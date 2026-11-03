"""EN 1997 v2: appends the catalogue cases of GI-17, PILE-10, PILE-11, HYD-02 and SLOPE-05 to the shared fixture and to the feature table (idempotent by label)."""
import json
import pathlib
import re

ROOT = next(p for p in pathlib.Path(__file__).resolve().parents if (p / "AGENTS.md").is_file())
ANY = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any"
FIXTURE = ANY / "🧪️tests/🧫️fixtures/📇️catalogue/🔣️.json"
FEATURE = ANY / "🧪️tests/📇️catalogue-en1997-1/🥒️.feature"


def series(prefix, values):
    return {f"{prefix}{i + 1}": v for i, v in enumerate(values)}


def case(entry, label, annex="De", num=None, text=None):
    return {"entry": entry, "label": label, "case": {"annex": annex, "num": num or {}, "text": text or {}}}


BISHOP = {"h": 6, "beta_deg": 30, "xc": 6, "yc": 10, "r": 11, "gamma": 18000, "gamma_sat": 20000, "phi_k_deg": 30, "gwl_level": 1.5, "q": 10000}
CASES = [
    case("GI-17", "direct shear lower bound", num={**series("sigma", [50e3, 100e3, 150e3, 200e3, 250e3]), **series("tau", [46.5e3, 77e3, 107e3, 138.5e3, 168e3]), "phi_assumed_deg": 30.0, "c_assumed": 10e3}),
    case("GI-17", "triaxial lower bound", num={**series("p", [90e3, 160e3, 235e3, 310e3, 365e3, 440e3]), **series("q", [58e3, 99e3, 148e3, 192e3, 230e3, 270e3])}, text={"kind": "triaxial"}),
    case("PILE-10", "criterion reached, three tests", num={**series("ts", [0.002, 0.005, 0.01, 0.02, 0.04, 0.06]), **series("tq", [4.0e5, 8.0e5, 1.1e6, 1.35e6, 1.5e6, 1.58e6]), "d": 0.6, **series("rm", [1.5e6, 1.65e6]), "f_cd": 1.0e6}),
    case("PILE-10", "criterion not reached EN", annex="En", num={**series("ts", [0.002, 0.005, 0.01, 0.02, 0.04, 0.06]), **series("tq", [4.0e5, 8.0e5, 1.1e6, 1.35e6, 1.5e6, 1.58e6]), "d": 0.8, "f_cd": 1.0e6}),
    case("PILE-11", "two profiles", num={"d": 0.6, **series("p1_h", [3.0, 5.0]), **series("p1_qs", [40e3, 80e3]), "p1_qb": 3.0e6, **series("p2_h", [3.0, 5.0]), **series("p2_qs", [35e3, 70e3]), "p2_qb": 2.5e6, "f_cd": 1.2e6}),
    case("HYD-02", "Lane creep rule", num={"delta_h": 1.0, "gamma_prime": 9500.0, **series("lv", [2.0, 2.0]), **series("lh", [12.0]), "c_lane": 7.0, "c_bligh": 9.0, "k": 1.0e-4}),
    case("HYD-02", "sheet pile exit gradient", num={"delta_h": 2.5, "gamma_prime": 9500.0, "d_embed": 3.0}),
    case("HYD-02", "flow-net exit gradient EN", annex="En", num={"delta_h": 2.0, "gamma_prime": 9500.0, "i_exit": 0.4, "k": 2.0e-5}),
    case("SLOPE-05", "cohesive end planes", num={**BISHOP, "c_k": 5000, "width": 20}),
    case("SLOPE-05", "wide body, high cohesion", num={**BISHOP, "c_k": 15000, "width": 60}),
]

data = json.loads(FIXTURE.read_text(encoding="utf8"))
have = {(c["entry"], c["label"]) for c in data["cases"]}
added = [c for c in CASES if (c["entry"], c["label"]) not in have]
data["cases"].extend(added)
FIXTURE.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf8")
if added:
    text = FEATURE.read_text(encoding="utf8")
    start = len(data["cases"]) - len(added)
    rows = "".join(f"      | {c['entry'].lower()}-{start + i}".ljust(21) + f"| {start + i}".ljust(8) + f"| {c['entry']}".ljust(12) + f"| {c['label']}".ljust(35) + "|\n" for i, c in enumerate(added))
    text = text.rstrip("\n") + "\n" + rows
    FEATURE.write_text(text, encoding="utf8")
print(len(added), "cases added; total", len(data["cases"]))
