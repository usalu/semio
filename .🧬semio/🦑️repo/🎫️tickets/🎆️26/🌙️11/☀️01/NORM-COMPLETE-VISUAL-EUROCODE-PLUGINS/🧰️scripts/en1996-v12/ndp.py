"""EN 1996 v12: registers the chase and recess tables of EN 1996-1-1 8.6.2/8.6.3 (Table 8.1 vertical chases and recesses, Table 8.2 horizontal and inclined chases) and their distance rules as unverified registry rows, idempotently, then regenerates `🗃️table.rs`.
Usage: /home/deck/semio/.venv/bin/python -I ndp.py"""
import json, subprocess, sys

NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"
NOTE = "Recalled from EN 1996-1-1 8.6.2/8.6.3, not checked against the print; the repo spec (en1995-en1996-target.md M-70) only gives 'vertical chases t >= 115?: depth <= 30 mm'. The National Annex DE reading is not recorded, so NA-DE equals the recommended value."
BANDS = ["b1", "b2", "b3", "b4", "b5"]
BAND_NAME = {"b1": "85-115 mm", "b2": "116-175 mm", "b3": "176-225 mm", "b4": "226-300 mm", "b5": "> 300 mm"}
rows = []

def row(suffix, symbol, clause, value, unit, what):
    rows.append({"id": "ndp.en1996.chase." + suffix, "code": "EN 1996", "part": "1-1", "clause": clause, "symbol": symbol, "recommended": value, "naDe": value, "unit": unit,
                 "source": {"document": "EN 1996-1-1 / DIN EN 1996-1-1/NA", "edition": "to be recorded at verification", "locator": clause},
                 "status": "unverified", "specRef": "en1995-en1996-target.md M-70: chases and recesses (8.6, Table 8.1) limit depth and width by wall thickness; the table values are not printed in the repo",
                 "asWritten": what + ". " + NOTE, "evidence": ""})

row("t-lower", "t_min (tables 8.1/8.2)", "8.6.2 Tables 8.1, 8.2", 0.085, "m", "Smallest wall thickness covered by the chase tables")
for band, upper in zip(BANDS, [0.115, 0.175, 0.225, 0.300]):
    row(f"band.{band}.t-upper", f"t upper ({BAND_NAME[band]})", "8.6.2 Tables 8.1, 8.2", upper, "m", f"Upper wall thickness of the band {BAND_NAME[band]}")
for band, depth, width in zip(BANDS, [0.030, 0.030, 0.030, 0.030, 0.030], [0.100, 0.125, 0.150, 0.175, 0.200]):
    row(f"v.{band}.depth", f"d_max vertical ({BAND_NAME[band]})", "8.6.2 Table 8.1", depth, "m", f"Maximum depth of a vertical chase or recess cut after construction, wall {BAND_NAME[band]}")
    row(f"v.{band}.width", f"w_max vertical ({BAND_NAME[band]})", "8.6.2 Table 8.1", width, "m", f"Maximum width of a vertical chase or recess cut after construction, wall {BAND_NAME[band]}")
for band, long, short in zip(BANDS, [0.0, 0.0, 0.0, 0.010, 0.015], [0.0, 0.015, 0.020, 0.025, 0.030]):
    row(f"h.{band}.depth-long", f"d_max horizontal unlimited length ({BAND_NAME[band]})", "8.6.2 Table 8.2", long, "m", f"Maximum depth of a horizontal or inclined chase of unlimited length, wall {BAND_NAME[band]}")
    row(f"h.{band}.depth-short", f"d_max horizontal short ({BAND_NAME[band]})", "8.6.2 Table 8.2", short, "m", f"Maximum depth of a horizontal or inclined chase not longer than the short length, wall {BAND_NAME[band]}")
row("h.short-length", "l_max short horizontal chase", "8.6.2 Table 8.2", 1.25, "m", "Length up to which the short column of Table 8.2 applies")
row("v.spacing", "a_min vertical chase", "8.6.2 Table 8.1 note", 0.225, "m", "Minimum horizontal distance between adjacent vertical chases and between a vertical chase and a recess or opening")
row("recess.spacing-factor", "factor recess spacing", "8.6.2 Table 8.1 note", 2.0, "1", "Minimum horizontal distance between recesses or a recess and an opening as a multiple of the width of the wider recess")
row("width-ratio", "sum w / l (vertical chases and recesses)", "8.6.2 Table 8.1 note", 0.13, "1", "Maximum cumulative width of vertical chases and recesses as a multiple of the wall length")
row("v.low-third.t-min", "t_min low-third chase", "8.6.2 Table 8.1 note", 0.225, "m", "Smallest wall thickness for the larger vertical chase in the lower third of the storey height")
row("v.low-third.depth", "d_max low-third chase", "8.6.2 Table 8.1 note", 0.080, "m", "Maximum depth of a vertical chase that does not rise above the lower third of the storey height")
row("v.low-third.width", "w_max low-third chase", "8.6.2 Table 8.1 note", 0.120, "m", "Maximum width of a vertical chase that does not rise above the lower third of the storey height")
row("v.low-third.height-ratio", "h_chase / h low-third", "8.6.2 Table 8.1 note", 1.0 / 3.0, "1", "Share of the storey height above the floor within which the larger vertical chase must stay")
row("h.opening-distance", "a_min horizontal chase to opening", "8.6.2 Table 8.2 note", 0.5, "m", "Minimum horizontal distance between the end of a horizontal chase and an opening")
row("h.spacing-factor", "factor horizontal chase spacing", "8.6.2 Table 8.2 note", 2.0, "1", "Minimum horizontal distance between adjacent horizontal chases as a multiple of the longer chase")

path = NDP + "🔣️.json"
text = open(path, encoding="utf-8").read()
data = json.loads(text)
assert json.dumps(data, indent=1, ensure_ascii=False) == text, "registry formatting changed"
have = {e["id"] for e in data["entries"]}
new = [r for r in rows if r["id"] not in have]
for e in data["entries"]:
    if e["id"] == "ndp.en1996.m70.execution":
        e["asWritten"] = e["asWritten"].replace("; chase depth 30 mm for t >= 115 mm | Überbindung, Mindestdicke, Pfeilerfläche, Schlitztiefe", " | Überbindung, Mindestdicke, Pfeilerfläche (chases: ndp.en1996.chase.*)")
if new:
    data["entries"] = sorted(data["entries"] + new, key=lambda e: e["id"])
open(path, "w", encoding="utf-8").write(json.dumps(data, indent=1, ensure_ascii=False))
subprocess.run([sys.executable, "-I", CODEGEN, path, NDP + "🗃️table.rs"], check=True)
print("added", len(new))
