"""Spec of the EN 1993 `Storey` entity (frame/storey, V-020..V-027). Needs `HERE` (the scripts dir) in scope. Imported by the en1993-storey-*-gen.py scripts (python3 -I)."""
import re
exec(compile(open(HERE / "en1993-special-spec.py", encoding="utf8").read().split("CH = {")[0], "units", "exec"))

ENT = "Storey"
COLL = "storeys"
SNAKE = "storey"
PATCH = "En1993StoreyPatch"
DELTA = "En1993StoreyDelta"
f = lambda name, ty, default, show, unit, en, de: dict(name=name, ty=ty, default=default, show=show, unit=unit, en=en, de=de, choices=None)
FIELDS = [
    dict(name="id", ty="String", ui=False),
    dict(name="member_id", ty="String", ui=False),
    dict(name="label", ty="String", ui=False),
    f("height", "f64", 4.0, 4.0, "m", "Storey height h", "Geschosshöhe h"),
    f("vertical_load", "f64", 4.0e5, 4.0e5, "N", "Total vertical load V_Ed", "Gesamte Vertikallast V_Ed"),
    f("horizontal_load", "f64", 2.0e4, 2.0e4, "N", "Horizontal load H_Ed (fictitious)", "Horizontallast H_Ed (Ersatzlast)"),
    f("drift", "f64", 0.002, 0.002, "m", "Horizontal drift δ_H,Ed", "Horizontale Kopfverschiebung δ_H,Ed"),
    f("columns", "u32", 2, 2, "", "Columns m in the row", "Stützen m in der Reihe"),
    f("plastic_analysis", "u32", 0, 0, "", "Plastic analysis (0/1)", "Plastische Berechnung (0/1)"),
    f("building_height", "f64", 4.0, 4.0, "m", "Structure height h", "Bauwerkshöhe h"),
    f("bracing_load", "f64", 0.0, 0.0, "N", "Sum of axial forces ΣN_Ed (bracing)", "Summe der Normalkräfte ΣN_Ed (Aussteifung)"),
    f("bracing_length", "f64", 0.0, 0.0, "m", "Bracing length L", "Aussteifungslänge L"),
    f("bracing_deflection", "f64", 0.0, 0.0, "m", "Bracing in-plane deflection δ_q", "Auslenkung der Aussteifung δ_q"),
]
for fd in FIELDS:
    fd.setdefault("default", "")
    fd.setdefault("show", "")
    fd.setdefault("unit", "")
    fd.setdefault("choices", None)
camel = lambda s: re.sub(r"_([a-z0-9])", lambda m: m.group(1).upper(), s)
PATCHF = [fd for fd in FIELDS if fd["name"] != "id"]
SHOW = {fd["name"]: fd["show"] for fd in FIELDS}
SHOW.update(id="storey-1", member_id="member-b1", label="Storey 1")
