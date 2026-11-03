"""EN 1996 v12: registers the normative literals of the check code (slenderness limits, applicability limits of the simplified method, reinforcement ratios, overlap, eccentricity, concentrated-load area ratio) as unverified registry rows `ndp.en1996.limit.*` with the values the artifact used so far, idempotently, then regenerates `🗃️table.rs`.
Usage: /home/deck/semio/.venv/bin/python -I ndp_limits.py"""
import json, subprocess, sys

NDP = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/⚖️compliance/📜️annex-and-tables/🔖️ndp/"
CODEGEN = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🧰️ndp/codegen.py"
NOTE = "Value as used by the artifact so far, moved out of the check code; not checked against the print."
ROWS = [
    ("slenderness", "5.5.1.4", "h_ef/t_ef max", 27.0, "1", "Upper limit of the slenderness ratio of a wall"),
    ("simplified.slenderness", "EN 1996-3 4.1", "h_ef/t max (simplified)", 21.0, "1", "Upper limit of the slenderness ratio for the simplified method"),
    ("simplified.building-height", "EN 1996-3 4.1", "H max (simplified)", 20.0, "m", "Upper limit of the building height for the simplified method"),
    ("simplified.imposed-load", "EN 1996-3 4.1", "q_k max (simplified)", 5000.0, "Pa", "Upper limit of the imposed floor load for the simplified method"),
    ("simplified.floor-span", "EN 1996-3 4.1", "l_f max (simplified)", 6.0, "m", "Upper limit of the floor span for the simplified method"),
    ("simplified.storey-threshold", "EN 1996-3 4.1", "storeys for t_min", 2.0, "1", "Number of storeys above which the larger minimum thickness applies for the simplified method"),
    ("simplified.min-thickness", "EN 1996-3 4.1", "t_min (simplified)", 0.115, "m", "Minimum wall thickness for the simplified method up to the storey threshold"),
    ("simplified.min-thickness-tall", "EN 1996-3 4.1", "t_min (simplified, tall)", 0.150, "m", "Minimum wall thickness for the simplified method above the storey threshold"),
    ("simplified.thick-wall", "EN 1996-3 4.1", "t thick wall", 0.24, "m", "Wall thickness from which the clear height limit is a multiple of the thickness"),
    ("simplified.height-ratio", "EN 1996-3 4.1", "h/t max (thick wall)", 12.0, "1", "Clear height limit of a thick wall as a multiple of its thickness"),
    ("simplified.height-thin", "EN 1996-3 4.1", "h max (thin wall)", 2.75, "m", "Clear height limit of a thinner wall"),
    ("simplified.min-length", "EN 1996-3 4.1", "l min (simplified)", 0.5, "m", "Minimum wall length for the simplified method"),
    ("simplified.min-area", "EN 1996-3 4.1", "A min (simplified)", 0.1, "m2", "Minimum wall area for the simplified method"),
    ("reinforcement.min-ratio", "EN 1996-1-1 reinforced masonry", "As min / A", 0.0005, "1", "Minimum reinforcement ratio of reinforced masonry"),
    ("reinforcement.max-ratio", "EN 1996-1-1 reinforced masonry", "As max / A", 0.04, "1", "Maximum reinforcement ratio of reinforced masonry"),
    ("overlap.ratio", "EN 1996-2 / 1996-1-1 bond", "l_ol / h_u min", 0.4, "1", "Minimum overlap of units as a multiple of the unit height"),
    ("overlap.min", "EN 1996-2 / 1996-1-1 bond", "l_ol min", 0.045, "m", "Minimum overlap of units"),
    ("eccentricity.initial-divisor", "5.5.1.1", "e_init = h_ef / x", 450.0, "1", "Divisor of the effective height for the initial eccentricity"),
    ("eccentricity.min-ratio", "5.5.1.1", "e_mk min / t", 0.05, "1", "Minimum design eccentricity as a multiple of the wall thickness"),
    ("concentrated.area-ratio", "6.1.3", "A_b / A_ef limit", 0.45, "1", "Bearing area ratio above which the enhancement of a concentrated load is dropped"),
]
SPLIT = [
    ("min-thickness", "EN 1996-2 3 / 8", "t_min (load-bearing wall)", 0.100, 0.115, "m", "Minimum thickness of a load-bearing wall (recommended value and National Annex DE)"),
    ("pier-area", "EN 1996-1-1 / NA-DE", "A_min (pier)", 0.04, 0.10, "m2", "Minimum area of a pier (recommended value and National Annex DE)"),
]
entries = [{"id": "ndp.en1996.limit." + key, "code": "EN 1996", "part": "1-1", "clause": clause, "symbol": symbol, "recommended": value, "naDe": value, "unit": unit,
            "source": {"document": "EN 1996-1-1 / DIN EN 1996-1-1/NA", "edition": "to be recorded at verification", "locator": clause},
            "status": "unverified", "specRef": "wave2 ndp: value as used by the artifact", "asWritten": what + ". " + NOTE, "evidence": ""} for key, clause, symbol, value, unit, what in ROWS]
entries += [{"id": "ndp.en1996.limit." + key, "code": "EN 1996", "part": "1-1", "clause": clause, "symbol": symbol, "recommended": en, "naDe": de, "unit": unit,
             "source": {"document": "EN 1996-1-1 / DIN EN 1996-1-1/NA", "edition": "to be recorded at verification", "locator": clause},
             "status": "unverified", "specRef": "wave2 ndp: value as used by the artifact", "asWritten": what + ". " + NOTE, "evidence": ""} for key, clause, symbol, en, de, unit, what in SPLIT]
path = NDP + "🔣️.json"
text = open(path, encoding="utf-8").read()
data = json.loads(text)
assert json.dumps(data, indent=1, ensure_ascii=False) == text, "registry formatting changed"
have = {e["id"] for e in data["entries"]}
new = [e for e in entries if e["id"] not in have]
data["entries"] = sorted(data["entries"] + new, key=lambda e: e["id"])
open(path, "w", encoding="utf-8").write(json.dumps(data, indent=1, ensure_ascii=False))
subprocess.run([sys.executable, "-I", CODEGEN, path, NDP + "🗃️table.rs"], check=True)
print("added", len(new))
