#!/usr/bin/env python3
"""🏗️ Prepared patch (rule 20, apply after W2's `--packages all`): the five fem3d mutation cases (load, material,
boundary, analysis, mesh) gained `@id-hall-vector` and `@id-reject` Scenario Outlines with Rust subjects only, so their
Python references answered `adapter has no oracle registration` for 51+ rows. Each reference gains two outline-base
handlers built from its own laws: `hall-vector` = its spec-vector law on the glulam-hall vector of the row's kind, and
`reject` = the feature's own statement (refuse, or declare a no-op, and leave the committed before-model exactly where
it was; the committed after-model IS the before-model).
The `reject` rows then showed the references applied 29 committed refusals: they only knew duplicate ids and missing
selected targets. Each reference therefore also gains `refuse`, run first in `apply_mutation`: the refusals the fem3d
vocabulary states (`mutation.id-mismatch`, `mutation.target-missing` on every foreign key, `mutation.target-referenced`
on every guarded delete, `mutation.invariant` on the solver's value bounds), written from the statement, not the Rust
guards' code. The load reference's own `resolve_load` folds into it.
`add-load` of a load id the case already carries is the committed no-op it is stated as (`⏸️dup-load-id-4f4a0a`),
not a refusal. The mesh and any-mesh features' `create-solid`/`replace-solid` rows predate the solid's `axis` member, which both the
snapshot's records and `FemSolid` (no value default) require, so neither implementation could decode them: they gain
`"axis":"z"` (the extrusion axis every committed solid carries) and their Examples tables are re-aligned.
Usage: patch.py --dry-run | --write [--root <dir>]  (--root points at a scratch copy for check.py)"""
import sys
from pathlib import Path

root = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
SUBSETS = "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"
CASES = ["🏋️load/🧪️tests/🏋️mutate-fem3d-1-load", "🧱️material/🧪️tests/🧱️mutate-fem3d-1-material", "🛡️boundary/🧪️tests/🛡️mutate-fem3d-1-boundary", "📈️analysis/🧪️tests/📈️mutate-fem3d-1-analysis", "🕸️mesh/🧪️tests/🕸️mutate-fem3d-1-mesh"]
HANDLERS = '''def hall_vector(ctx):
    """🏗️ The committed glulam-hall vector for the kind the row names — this reference's spec-vector law on the
    hall model (`@id-hall-vector`)."""
    return spec_vector_handler(ctx.row())(ctx)


def reject(ctx):
    """🚨️ A committed vector both implementations must refuse or declare a no-op (`@id-reject`): the reference either
    raises on it or applies it without moving the model, and the committed after-model is the before-model."""
    before = document_of(json_fixture(ctx, "⬅️before"))
    mutation = json_fixture(ctx, "🦠️mutation")
    if document_of(json_fixture(ctx, "➡️after")) != before:
        raise AssertionError("reject-%s: the committed after-model is not the committed before-model" % ctx.row())
    try:
        applied = apply_mutation(before, mutation)
    except AssertionError:
        return outcome_of(before)
    if applied != before:
        raise AssertionError("reject-%s: the reference applied a mutation the committed vector refuses" % ctx.row())
    return outcome_of(applied)


'''
GUARDS = '''REFERENCES = {"element": (("start", "nodes"), ("end", "nodes"), ("materialId", "materials"), ("sectionId", "sections")), "solid": (("materialId", "materials"),), "support": (("nodeId", "nodes"),)}
"""🪢️ The foreign keys a record of each noun names, in the order they resolve, and the collection each resolves in."""
CARRIERS = {"nodal": ("nodeId", "nodes", ("value",)), "memberUdl": ("elementId", "elements", ("wx", "wy", "wz")), "area": ("solidId", "solids", ("pressure",))}
"""🪝️ The one carrier each load variant hangs on, and the magnitudes it carries."""


def finite(*values):
    """♾️ Every value is a finite JSON number — an infinite or undefined one poisons every solve it enters."""
    return all(isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value) for value in values)


def ring_area(ring):
    """⭕️ The signed shoelace area of a closed ring, negative when it winds clockwise."""
    return sum(ring[at][0] * ring[(at + 1) % len(ring)][1] - ring[(at + 1) % len(ring)][0] * ring[at][1] for at in range(len(ring))) / 2.0


def ring_contains(ring, point):
    """🎈️ Crossing-count containment; a point exactly on an edge stays undefined, as the classical ray cast leaves it."""
    inside = False
    for at in range(len(ring)):
        corner, other = ring[at], ring[(at + 1) % len(ring)]
        if (corner[1] > point[1]) != (other[1] > point[1]) and point[0] < corner[0] + (point[1] - corner[1]) / (other[1] - corner[1]) * (other[0] - corner[0]):
            inside = not inside
    return inside


def solid_holds(solid):
    """🪨️ The footprint an extrusion mesher can tet-split: a closed ring of non-zero area, a positive height through
    at least one layer, a positive mesh size, and every hole a non-degenerate ring strictly inside the outline."""
    outline = solid["outline"]
    if len(outline) < 3 or not all(finite(*point) for point in outline) or ring_area(outline) == 0:
        return False
    if not finite(solid["baseZ"], solid["height"], solid["meshSize"]) or solid["height"] <= 0 or solid["layers"] < 1 or solid["meshSize"] <= 0:
        return False
    return all(len(hole) >= 3 and ring_area(hole) != 0 and all(finite(*point) and ring_contains(outline, point) for point in hole) for hole in solid["holes"])


def holds(noun, record):
    """🩺️ The value bounds a record must keep for the model to stay solvable — a finite node position; a linear-elastic
    isotropic material (positive e, g and rho, a Poisson ratio in the open interval (-1, 0.5)); a cross-section with a
    positive area and second moments; a meshable solid; finite combination factors."""
    if noun == "node":
        return finite(record["x"], record["y"], record["z"])
    if noun == "material":
        return finite(record["e"], record["g"], record["nu"], record["rho"]) and min(record["e"], record["g"], record["rho"]) > 0 and -1 < record["nu"] < 0.5
    if noun == "section":
        return finite(record["area"], record["iy"], record["iz"], record["j"]) and min(record["area"], record["iy"], record["iz"], record["j"]) > 0
    if noun == "solid":
        return solid_holds(record)
    if noun == "combination":
        return finite(*record["terms"].values())
    return True


def referrers(document, noun, identifier):
    """🧷️ Every record that would dangle if `identifier` left its collection. A node, a support and a combination keep
    none by statement: `delete-node` is cascade-free (its committed vector keeps a frame naming the deleted node), and
    supports and combinations are leaves nothing points back at."""
    loads = [load for case in document["loadCases"] for load in case["loads"]]
    if noun == "element":
        return [load["id"] for load in loads if load["kind"] == "memberUdl" and load["elementId"] == identifier]
    if noun == "solid":
        return [load["id"] for load in loads if load["kind"] == "area" and load["solidId"] == identifier]
    if noun == "section":
        return [element["id"] for element in document["elements"] if element["sectionId"] == identifier]
    if noun == "material":
        return [record["id"] for name in ("elements", "solids") for record in document[name] if record["materialId"] == identifier]
    if noun == "load-case":
        return [combination["id"] for combination in document["combinations"] if identifier in combination["terms"]]
    return []


def land(document, kind, loads):
    """⚓️ Every load names exactly one carrier — a node, an element or a solid — and it must be in the model."""
    for load in loads:
        field, collection, _ = CARRIERS[load["kind"]]
        if find(document[collection], load[field]) is None:
            raise AssertionError("%s: load %r names %r, which is not in %s" % (kind, load["id"], load[field], collection))


def refuse(document, kind, mutation):
    """🛡️ The refusals owed before a mutation may move the model: `mutation.id-mismatch` for a replace that renames its
    target, `mutation.target-missing` for a reference that does not resolve, `mutation.target-referenced` for a delete
    that would leave a referrer dangling, `mutation.invariant` for a value the solver cannot take. Raises on the first;
    an identical replace or settings update is a no-op and passes."""
    if kind == "update-analysis-settings":
        settings = mutation["settings"]
        if settings != document["analysis"] and not (settings["modalCount"] >= 1 and settings["bucklingCount"] >= 1 and finite(settings["deformationScale"]) and settings["deformationScale"] > 0):
            raise AssertionError("%s: %r asks for no mode, no buckling factor or no positive deformation scale" % (kind, settings))
        return
    if kind in ("add-load", "replace-load"):
        load = mutation["load"] if kind == "add-load" else mutation["newLoad"]
        case = find(document["loadCases"], mutation["caseId"])
        if kind == "add-load" and case is not None and find(document["loadCases"][case]["loads"], load["id"]) is not None:
            return
        land(document, kind, [load])
        if kind == "replace-load" and not finite(*(load[name] for name in CARRIERS[load["kind"]][2])):
            raise AssertionError("%s: load %r carries a magnitude that is not finite" % (kind, load["id"]))
        return
    verb, noun = kind.split("-", 1)
    if verb not in ("create", "delete", "replace") or noun not in COLLECTIONS:
        return
    collection, create_argument, replace_argument = COLLECTIONS[noun]
    if verb == "delete":
        held = referrers(document, noun, mutation["id"])
        if held:
            raise AssertionError("%s: %r is still referenced by %r" % (kind, mutation["id"], held))
        return
    record = mutation[create_argument if verb == "create" else replace_argument]
    if verb == "replace":
        if record["id"] != mutation["id"]:
            raise AssertionError("%s: a replace selects %r and may not rename it to %r" % (kind, mutation["id"], record["id"]))
        if record in document[collection]:
            return
    for field, target in REFERENCES.get(noun, ()):
        if find(document[target], record[field]) is None:
            raise AssertionError("%s: %r names %s %r, which is not in %s" % (kind, record["id"], field, record[field], target))
    if noun == "load-case":
        land(document, kind, record["loads"])
    if noun == "combination":
        missing = [case for case in record["terms"] if find(document["loadCases"], case) is None]
        if missing:
            raise AssertionError("%s: %r weights %r, which is not in loadCases" % (kind, record["id"], missing))
    if not holds(noun, record):
        raise AssertionError("%s: %r breaks the %s bounds the solver needs" % (kind, record["id"], noun))


'''
RESOLVE_LOAD = '''def resolve_load(document, load, kind):
    """🎯️ A load names exactly one carrier — a node, an element or a solid — and it must be there."""
    if load["kind"] == "nodal":
        target, collection = load["nodeId"], "nodes"
    elif load["kind"] == "memberUdl":
        target, collection = load["elementId"], "elements"
    else:
        target, collection = load["solidId"], "solids"
    if find(document[collection], target) is None:
        raise AssertionError("%s: load %r names %r, which is not in %s" % (kind, load["id"], target, collection))


'''
RESOLVE_CALL = '''        if case["loads"][at] != load:
            resolve_load(result, load, kind)
'''
IMPORTS = "import copy\nimport json\n"
ADD_LOAD = '''        if find(case["loads"], load["id"]) is not None:
            raise AssertionError("%s: case %r already carries a load %r" % (kind, case["id"], load["id"]))
        case["loads"].append(load)
'''
ADD_LOAD_NO_OP = '''        if find(case["loads"], load["id"]) is None:
            case["loads"].append(load)
'''
FEATURES = ["🕸️mesh/🧪️tests/🕸️mutate-fem3d-1-mesh/🥒️.feature", "🌐️any/🧪️tests/🕸️mutate-fem3d-1-any-mesh/🥒️.feature"]
SOLID_ROWS = ('"materialId":"concrete"}}', '"materialId":"concrete","axis":"z"}}')


def realign(lines):
    """📏️ Re-pads every Examples table so each column is as wide as its widest cell."""
    at = 0
    while at < len(lines):
        if not lines[at].strip().startswith("|"):
            at += 1
            continue
        end = at
        while end < len(lines) and lines[end].strip().startswith("|"):
            end += 1
        indent = lines[at][: len(lines[at]) - len(lines[at].lstrip())]
        rows = [[cell.strip() for cell in line.strip()[1:-1].split(" | ")] for line in lines[at:end]]
        widths = [max(len(row[column]) for row in rows) for column in range(len(rows[0]))]
        lines[at:end] = [indent + "| " + " | ".join(cell.ljust(width) for cell, width in zip(row, widths)) + " |" for row in rows]
        at = end
    return lines
APPLY = '''def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting model."""
    kind = kind_of(mutation)
'''
write = "--write" in sys.argv
problems, edits = [], {}
for case in CASES:
    path = root / SUBSETS / case / "🐍️.py"
    source = path.read_text(encoding="utf-8")
    anchor = "# region 🔖️Registration\ndef adapter():\n"
    registered = "    return built\n\n\n# endregion 🔖️Registration"
    if source.count(anchor) != 1 or source.count(registered) != 1 or "def hall_vector(" in source:
        problems.append(f"{case}: anchors {source.count(anchor)}/{source.count(registered)}, hall_vector present {'def hall_vector(' in source}")
        continue
    if source.count(IMPORTS) != 1 or source.count(APPLY) != 1 or "def refuse(" in source:
        problems.append(f"{case}: imports {source.count(IMPORTS)}, apply_mutation {source.count(APPLY)}, refuse present {'def refuse(' in source}")
        continue
    if "🏋️load" in case:
        if source.count(RESOLVE_LOAD) != 1 or source.count(RESOLVE_CALL) != 1 or source.count("resolve_load(") != 2:
            problems.append(f"{case}: resolve_load {source.count(RESOLVE_LOAD)}/{source.count(RESOLVE_CALL)}/{source.count('resolve_load(')}")
            continue
        source = source.replace(RESOLVE_LOAD, "").replace(RESOLVE_CALL, "")
    source = source.replace(IMPORTS, IMPORTS + "import math\n")
    source = source.replace(APPLY, GUARDS + APPLY + "    refuse(document, kind, mutation)\n")
    if source.count(ADD_LOAD) != 1:
        problems.append(f"{case}: add-load branch {source.count(ADD_LOAD)}")
        continue
    source = source.replace(ADD_LOAD, ADD_LOAD_NO_OP)
    source = source.replace(anchor, "# region 🔖️Registration\n" + HANDLERS + "def adapter():\n")
    source = source.replace(registered, '    return built.oracle("hall-vector", hall_vector).oracle("reject", reject)\n\n\n# endregion 🔖️Registration')
    edits[path] = source
for name in FEATURES:
    feature = root / SUBSETS / name
    text = feature.read_text(encoding="utf-8")
    rows = [line for line in text.split("\n") if ('"mutation":"createSolid"' in line or '"mutation":"replaceSolid"' in line) and SOLID_ROWS[0] in line]
    if len(rows) != 4 or '"axis"' in "".join(rows):
        problems.append(f"{name}: {len(rows)} solid rows without axis")
        continue
    edits[feature] = "\n".join(realign([line.replace(SOLID_ROWS[0], SOLID_ROWS[1]) if line in rows else line for line in text.split("\n")]))
print(f"files={len(edits)} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, source in edits.items():
        path.write_text(source, encoding="utf-8")
