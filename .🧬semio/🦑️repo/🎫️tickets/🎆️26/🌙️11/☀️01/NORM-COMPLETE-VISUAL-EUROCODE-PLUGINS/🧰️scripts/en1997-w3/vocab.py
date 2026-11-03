"""W3 EN 1997: extends the committed input vocabulary json (the tsv is stale) with the keys of SLS-05, PILE-12 and SLOPE-06 and regenerates the rust twin with the formatting of catalogue-bridge-gen-inputs.py. Usage: python3 -I vocab.py <📚️inputs dir> [--check]"""
import json, pathlib, sys

OUT = pathlib.Path(sys.argv[1])
doc = json.loads((OUT / "🔣️.json").read_text(encoding="utf-8"))
rows = doc["inputs"]
SYM = {"Metre": "m", "Second": "s", "One": "1"}


WIDENED, ADDED = [], []


def widen(key, old, extra):
    row = next((r for r in rows if r["key"] == key and (r["scope"] == old or extra in r["scope"].split(","))), None)
    if row is None or extra in row["scope"].split(","):
        return
    new = ",".join(old.split(",") + [extra])
    row["scope"] = new
    WIDENED.append((key, old, new))


def add(key, scope, unit, en, de, conf="medium", note=""):
    if any(r["key"] == key and r["scope"] == scope for r in rows):
        return
    row = {"key": key, "scope": scope, "kind": "number", "en": en, "de": de, "confidence": conf}
    if note:
        row["note"] = note
    row["unit"], row["unitId"] = SYM[unit], unit
    rows.append(row)
    ADDED.append(row)


widen("t", "SLS-02", "SLS-05")
widen("m", "PILE-06", "PILE-12")
widen("n", "PILE-06", "PILE-12")
widen("s", "PILE-06", "PILE-12")
widen("h1", "G-01,SLS-01,PILE-05", "PILE-12")
widen("e1", "SLS-01", "PILE-12")
add("s_final", "SLS-05", "Metre", "Final primary consolidation settlement", "Endwert der Konsolidationssetzung", "high")
add("t90_lab", "SLS-05", "Second", "Laboratory time to 90 % consolidation t90 (Casagrande or Taylor)", "Laborzeit bis 90 % Konsolidation t90 (Casagrande oder Taylor)", "high")
add("h_dr_lab", "SLS-05", "Metre", "Drainage path of the oedometer specimen", "Dränweg der Ödometerprobe", "high")
add("s_res_lim", "SLS-05", "Metre", "Admissible residual settlement at time t", "Zulässige Restsetzung zur Zeit t", "high")
add("spread", "PILE-12", "One", "Load spread of the equivalent raft, horizontal per vertical (1:4 = 0.25)", "Lastausbreitung des Ersatzfundaments, horizontal je vertikal (1:4 = 0,25)", "low", "textbook equivalent-raft convention, not an EN 1997 value")
add("raft_depth_factor", "PILE-12", "One", "Depth of the equivalent raft as a fraction of the pile length (2/3)", "Tiefe des Ersatzfundaments als Anteil der Pfahllänge (2/3)", "low", "textbook equivalent-raft convention, not an EN 1997 value")
rows.sort(key=lambda r: r["key"])
(OUT / "🔣️.json").write_text(json.dumps(doc, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def q(s):
    return json.dumps(s, ensure_ascii=False)


lines = (OUT / "🦀️.rs").read_text(encoding="utf-8").split("\n")
for key, old, new in WIDENED:
    n = 0
    for i, l in enumerate(lines):
        if l.startswith(f"    InputSpec::") and f'({q(key)}, ' in l and l.endswith(f'.scope({q(old)}),'):
            lines[i] = l[: -len(f'.scope({q(old)}),')] + f'.scope({q(new)}),'
            n += 1
    assert n == 1 or old == new, (key, old, n)
end = lines.index("];")
have = {l for l in lines}
for r in ADDED:
    s = f'    InputSpec::number({q(r["key"])}, {q(r["en"])}, {q(r["de"])}, UnitId::{r["unitId"]}).scope({q(r["scope"])}),'
    if s not in have:
        lines.insert(end, s)
        end += 1
(OUT / "🦀️.rs").write_text("\n".join(lines), encoding="utf-8")
print(len(rows), "rows")
