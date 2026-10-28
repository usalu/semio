"""🔌 Ticket script: print and self-check the exchange model (STEP excerpt, mesh, IDS check)."""

import math
import re
import sys
from pathlib import Path

sys.dont_write_bytecode = True
sys.stdout.reconfigure(encoding="utf-8")
ROOT = Path(__file__).resolve().parents[7]
CLIP = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"
sys.path.insert(0, str(ROOT))
import importlib.util

spec = importlib.util.spec_from_file_location("bim_model_selfcheck", CLIP / "🧱️model/🐍️.py")
api = importlib.util.module_from_spec(spec)
sys.modules["bim_model_selfcheck"] = api
spec.loader.exec_module(api)

ATTRS = {"IFCCOLUMN": 9, "IFCSLAB": 9, "IFCWALL": 9, "IFCLOCALPLACEMENT": 2, "IFCAXIS2PLACEMENT3D": 3, "IFCAXIS2PLACEMENT2D": 2, "IFCCARTESIANPOINT": 1,
         "IFCDIRECTION": 1, "IFCPRODUCTDEFINITIONSHAPE": 3, "IFCSHAPEREPRESENTATION": 4, "IFCEXTRUDEDAREASOLID": 4, "IFCCIRCLEPROFILEDEF": 4,
         "IFCRECTANGLEPROFILEDEF": 5, "IFCGEOMETRICREPRESENTATIONCONTEXT": 6, "IFCMATERIAL": 3, "IFCMATERIALLAYER": 7, "IFCMATERIALLAYERSET": 3,
         "IFCRELASSOCIATESMATERIAL": 6, "IFCPROPERTYSINGLEVALUE": 4, "IFCPROPERTYSET": 5, "IFCRELDEFINESBYPROPERTIES": 6, "IFCCOLUMNTYPE": 10,
         "IFCWALLTYPE": 10, "IFCRELDEFINESBYTYPE": 6, "IFCBUILDINGSTOREY": 10, "IFCRELCONTAINEDINSPATIALSTRUCTURE": 6}
ROLES = {"guid", "class", "name", "tag", "predefined", "placement", "representation", "profile", "depth", "pset", "property", "value", "material", "reference", "number"}
REAL = re.compile(r"^-?\d+\.\d*(E[+-]\d+)?$")
failures = []


def check(condition, message):
    if not condition:
        failures.append(message)
        print("[FAIL]", message)


def balanced(line):
    depth, quote, index = 0, False, 0
    while index < len(line):
        char = line[index]
        if char == "'":
            if quote and line[index + 1 : index + 2] == "'":
                index += 1
            else:
                quote = not quote
        elif not quote:
            depth += (char == "(") - (char == ")")
            check(depth >= 0, f"negative depth {line}")
        index += 1
    return depth == 0 and not quote


def split_top(text):
    parts, depth, quote, last, index = [], 0, False, 0, 0
    while index < len(text):
        char = text[index]
        if char == "'":
            if quote and text[index + 1 : index + 2] == "'":
                index += 1
            else:
                quote = not quote
        elif not quote:
            depth += (char == "(") - (char == ")")
            if char == "," and depth == 0:
                parts.append(text[last:index])
                last = index + 1
        index += 1
    return parts + [text[last:]]


def verify(ex, tag):
    lines = ex["lines"]
    check(lines[0] == "ISO-10303-21;" and lines[-1] == "END-ISO-10303-21;", f"{tag} wrapper")
    check("FILE_SCHEMA(('IFC4X3_ADD2'));" in lines, f"{tag} schema")
    check(lines.count("ENDSEC;") == 2 and lines.index("DATA;") > lines.index("HEADER;"), f"{tag} sections")
    defined, used = {}, set()
    for index, line in enumerate(lines):
        check(balanced(line), f"{tag} unbalanced {line}")
        match = re.match(r"^(#\d+)=([A-Z0-9]+)\((.*)\);$", line)
        if not match:
            continue
        number, name, body = match.groups()
        check(number not in defined, f"{tag} duplicate {number}")
        defined[number] = name
        attrs = split_top(body)
        check(len(attrs) == ATTRS[name], f"{tag} {name} has {len(attrs)} attributes, expects {ATTRS[name]}")
        used.update(re.findall(r"#\d+", re.sub(r"'(?:[^']|'')*'", "''", body)))
        for token in re.findall(r"(?<![\w'.])-?\d+\.\d*(?:E[+-]\d+)?(?![\w'])", re.sub(r"'(?:[^']|'')*'", "''", body)):
            check(REAL.match(token) is not None, f"{tag} real {token}")
        check(len(line) < 110, f"{tag} long line {len(line)}: {line}")
    check(used <= set(defined), f"{tag} undefined {sorted(used - set(defined))}")
    check(set(defined) == set(ex["ids"].values()), f"{tag} ids map {sorted(set(defined) ^ set(ex['ids'].values()))}")
    check(all(ex["lines"][ex["entities"][name]].startswith(number + "=") for name, number in ex["ids"].items()), f"{tag} entities map")
    for span in ex["spans"]:
        line = lines[span["line"]]
        check(0 <= span["start"] < span["end"] <= len(line) and span["role"] in ROLES, f"{tag} span {span}")
    roles = {s["role"] for s in ex["spans"]}
    check({"guid", "class", "name", "tag", "predefined", "placement", "representation", "profile", "depth", "pset", "property", "value", "material", "reference", "number"} <= roles, f"{tag} roles {roles}")
    first = next(s for s in ex["spans"] if s["role"] == "guid")
    check(lines[first["line"]][first["start"] : first["end"]] == ex["guid"] and len(ex["guid"]) == 22, f"{tag} guid span")
    check(ex["lines"][ex["entities"]["element"]].split("=")[1].startswith("IFC" + ex["class"][3:].upper()), f"{tag} class")
    data = lines[lines.index("DATA;") + 1 : -2]
    return len(data)


m = api.load_model()
print("== STEP excerpts ==")
for key, language in (("B3", "en"), ("A1", "en"), ("SL-1", "en"), ("W-S", "en"), ("W-E", "de"), ("SL-R", "en"), ("B3", "de")):
    ex = api.step_excerpt(key, m, language=language)
    count = verify(ex, f"{key}/{language}")
    print(f"-- {key} {language}: {count} DATA lines, {len(ex['spans'])} spans")
    if key in ("B3", "W-E", "SL-1") and language in ("en", "de"):
        print("\n".join(ex["lines"]))
        for span in ex["spans"][:0]:
            print(span)

a, b = api.step_excerpt("B3", m), api.step_excerpt("B3", m, first_id=2300)
check(len(a["lines"]) == len(b["lines"]) and a["ids"]["element"] == "#101" and b["ids"]["element"] == "#2300", "first_id")
check(a["guid"] == b["guid"] and a["guid"] in a["lines"][a["entities"]["element"]] and a["guid"] in b["lines"][b["entities"]["element"]], "same GlobalId in both exports")
check(re.sub(r"#\d+", "#", "\n".join(a["lines"])) == re.sub(r"#\d+", "#", "\n".join(b["lines"])), "renumbering changes only references")
check([(s["line"], s["role"]) for s in a["spans"]] == [(s["line"], s["role"]) for s in b["spans"]], "spans keep lines and roles")
compact = api.step_excerpt("B3", m, spatial=False)
print("compact lines", len(compact["lines"]) - 9)
verify(compact, "B3/compact")
check(api.step_excerpt("B3", m)["lines"] == api.step_excerpt("B3", m)["lines"], "deterministic")
check("5.0" not in "".join(api.step_excerpt("B3", m)["lines"]).split("DATA;")[1].replace("1.E-05", ""), "no stray")
e = api.step_excerpt("B3", m)
print("pset guid", re.findall(r"IFCPROPERTYSET\('([^']+)'", "\n".join(e["lines"])), api.global_id("B3.Pset_ColumnCommon"))
for bad in ("RW-S", "XX", "PS-1"):
    try:
        api.step_excerpt(bad, m)
        check(False, f"{bad} must fail")
    except ValueError:
        pass

print("== mesh ==")
for n in (3, 6, 8, 12, 16):
    t = api.mesh("B3", n, m)
    r, h = t["radius"], t["height"]
    check(t["vertex_count"] == len(t["vertices"]) == 2 * n + 2 and t["triangle_count"] == len(t["faces"]) == 4 * n, f"mesh counts {n}")
    edges = {}
    for face in t["faces"]:
        check(len(set(face)) == 3 and all(0 <= i < t["vertex_count"] for i in face), f"face {face}")
        for i in range(3):
            edge = (face[i], face[(i + 1) % 3])
            edges[edge] = edges.get(edge, 0) + 1
    check(all(c == 1 for c in edges.values()) and all((b2, a2) in edges for a2, b2 in edges), f"closed manifold {n}")
    volume = 0.0
    for i, j, k in t["faces"]:
        p, q, s = t["vertices"][i], t["vertices"][j], t["vertices"][k]
        volume += (p[0] * (q[1] * s[2] - q[2] * s[1]) - p[1] * (q[0] * s[2] - q[2] * s[0]) + p[2] * (q[0] * s[1] - q[1] * s[0])) / 6
    expected = 0.5 * n * r * r * math.sin(2 * math.pi / n) * h
    check(abs(volume - expected) < 1e-6, f"volume {volume} vs {expected}")
    check(all(math.isclose(math.dist(v[:2], t["centre"]), r, abs_tol=1e-6) for v in t["vertices"][: 2 * n]), "on circle")
    print(n, t["vertex_count"], t["triangle_count"], round(volume, 5), round(math.pi * r * r * h, 5))
t = api.mesh("B3", 16, m)
print([k["id"] for k in t["kept"]], [(k["id"], k["en"], k["de"]) for k in t["lost"]])
for bad in (2, True, 4.0):
    try:
        api.mesh("B3", bad, m)
        check(False, f"segments {bad} must fail")
    except ValueError:
        pass
try:
    api.mesh("SL-1", 8, m)
    check(False, "slab mesh must fail")
except ValueError:
    pass

print("== ids ==")
r = api.ids_check(m)
print(r["applicability"], r["requirement"], r["cardinality"], r["checked"], r["passed"], r["failed"], r["items"][7])
check(r["checked"] == 25 and r["passed"] == 25 and r["failed"] == [] and all(i["value"] is True for i in r["items"]), "default column check")
w = api.ids_check(m, "IfcWall IsExternal exists")
print(w["requirement"], w["checked"], w["passed"], w["failed"], [(i["tag"], i["value"]) for i in w["items"]])
check(w["checked"] == 5 and w["passed"] == 5, "wall check")
mesh_run = api.ids_check(m, source="mesh")
print(mesh_run["expected"], mesh_run["found"], mesh_run["checked"], mesh_run["passed"])
check(mesh_run["expected"] == 25 and mesh_run["found"] == 0 and mesh_run["checked"] == 0, "mesh source")
import copy

broken = copy.deepcopy(m)
del next(t for t in broken["types"] if t["tag"] == "TY-PILOTI")["psets"]["Pset_ColumnCommon"]["LoadBearing"]
bad = api.ids_check(broken)
check(bad["checked"] == 25 and bad["passed"] == 0 and len(bad["failed"]) == 25, "failing check")
nobool = copy.deepcopy(m)
next(t for t in nobool["types"] if t["tag"] == "TY-PILOTI")["psets"]["Pset_ColumnCommon"]["LoadBearing"]["value"] = "yes"
check(api.ids_check(nobool)["passed"] == 0, "datatype")
false = copy.deepcopy(m)
next(t for t in false["types"] if t["tag"] == "TY-PILOTI")["psets"]["Pset_ColumnCommon"]["LoadBearing"]["value"] = False
check(api.ids_check(false)["passed"] == 25, "not forced to TRUE")
check(api.ids_check(false, {"entity": "IfcColumn", "property": "LoadBearing", "value": True})["passed"] == 0, "value restriction")
check(api.ids_check(m, "IfcColumn Pset_ColumnCommon.Slope prohibited")["passed"] == 25, "prohibited")
check(api.ids_check(m, "IfcColumn Pset_ColumnCommon.Slope exists")["failed"] == [i["tag"] for i in api.ids_check(m)["items"]], "missing property")
for bad in ("IfcColumn", {"entity": "IfcColumn"}, {"entity": "IfcColumn", "property": "x", "cardinality": "never"}):
    try:
        api.ids_check(m, bad)
        check(False, f"spec {bad} must fail")
    except ValueError:
        pass
check(api.ids_check(m)["spec"]["id"] == "IDS-COLUMN-LOADBEARING", "default spec from the model rules")

spec2 = importlib.util.spec_from_file_location("bim_model_unregistered", CLIP / "🧱️model/🐍️.py")
other = importlib.util.module_from_spec(spec2)
spec2.loader.exec_module(other)
check(other.step_excerpt("W-S")["lines"] == api.step_excerpt("W-S")["lines"] and other.mesh("B3")["faces"] == api.mesh("B3")["faces"], "unregistered core")
check(other.ids_check()["passed"] == 25 and api.ids_check()["checked"] == 25, "default model argument")
longest = max(len(l) for l in api.step_excerpt("B3")["lines"])
print("longest B3 line", longest)

print("FAILURES" if failures else "ALL OK", len(failures))
sys.exit(1 if failures else 0)
