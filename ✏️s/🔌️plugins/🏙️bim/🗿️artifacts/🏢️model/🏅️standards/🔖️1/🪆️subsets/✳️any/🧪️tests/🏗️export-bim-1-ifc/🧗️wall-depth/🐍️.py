"""🧗️ The wall-depth audits: the attached walls, the envelope volumes, the sweeps and the opening reveals against the shapely-adjudicated tables."""

from __future__ import annotations

# region 🔖️Imports
import json
import ifcopenshell.util.element

from export_bim_ifc_kernel import settings, kernel_volume, kernel_bounds, quantities_of, close
# endregion 🔖️Imports


# region 🔖️Measurement
# region 🔖️WallDepth
ATTACH_SET = "Semio_WallAttach"
REVEAL_SET = "Semio_OpeningReveal"
SWEEP_SET = "Semio_WallSweep"
ATTACHED_TOPS = {"Roof": "roof", "Slab": "slab", "Ceiling": "ceiling"}
"""🧗️ The top constraints that follow a roof, a slab or a ceiling, with the field that names the target."""

EMPTY_COLLECTIONS = [
    "walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces", "openings", "ceilings", "ramps", "wall_sweeps", "properties", "classifications",
    "wall_types", "slab_types", "ceiling_types", "roof_types", "column_types", "beam_types", "window_types", "door_types",
    "components", "component_overrides", "mep_elements", "families", "family_parameters", "family_solids",
]
"""📚️ The collections a case may leave out of its snapshot: the audit reads each of them as empty."""


def rows_of(element, name):
    """🏷️ The rows of one property set of an element (empty when it has none)."""
    return ifcopenshell.util.element.get_psets(element).get(name, {})


def top_of(wall):
    """🔝️ `(kind, body)` of the authored top of a snapshot wall: `("Roof", {"roof": "r-1", "offset": 0})`."""
    return next(iter(wall["top"].items()))


def attach_problems(model, snapshot, table):
    """🧗️ The attach of every wall: the `Semio_WallAttach` set carries the authored top, target, offsets and base slab, a free wall carries none, and the `IfcRelConnectsElements` of the
    file are exactly the top and base attaches of the snapshot, from the wall to its roof, slab or ceiling (read back with `RelatingElement`, `RelatedElement` and `Description`)."""
    problems = []
    walls = {element.Tag: element for element in model.by_type("IfcWall")}
    expected = []
    for tag, wall in snapshot["walls"].items():
        kind, body = top_of(wall)
        slab = wall.get("base_slab") or ""
        rows = rows_of(walls[tag], ATTACH_SET) if tag in walls else {}
        if kind not in ATTACHED_TOPS and not slab:
            if rows:
                problems.append("%s: a free wall carries the set %s" % (tag, ATTACH_SET))
            continue
        target = body.get(ATTACHED_TOPS.get(kind, "storey"), "")
        if kind in ATTACHED_TOPS:
            expected.append(("TopAttach", tag, target))
        if slab:
            expected.append(("BaseAttach", tag, slab))
        want = {"TopKind": kind, "TopTarget": target, "BaseSlab": slab}
        problems += ["%s: %s is %r, the snapshot says %r" % (tag, key, rows.get(key), value) for key, value in want.items() if rows.get(key) != value]
        for key, value in (("TopOffset", body.get("offset", 0.0)), ("BaseOffset", wall["base_offset"])):
            if not close(rows.get(key, float("nan")), value):
                problems.append("%s: %s is %r, the snapshot says %r" % (tag, key, rows.get(key), value))
    found = sorted((relation.Description, relation.RelatingElement.Tag, relation.RelatedElement.Tag) for relation in model.by_type("IfcRelConnectsElements"))
    if found != sorted(expected):
        problems.append("IfcRelConnectsElements: %s in the file, %s attached by the snapshot" % (found, sorted(expected)))
    if table["counts"]["IfcRelConnectsElements"] != len(expected):
        problems.append("IfcRelConnectsElements: %d counted, %d expected" % (table["counts"]["IfcRelConnectsElements"], len(expected)))
    return problems


def envelope_problems(model, snapshot, committed):
    """🧊️ The tessellated body of an attached wall against the shapely-adjudicated `wall-layout` table: the kernel volume equals the table volume within 1e-6 (relative) and the vertical extent of the
    kernel mesh equals the lowest base and the highest top of the table, on top of the elevation of the building."""
    problems = []
    kernel = settings()
    walls = {element.Tag: element for element in model.by_type("IfcWall")}
    for tag, row in (committed or {}).get("walls", {}).items():
        element = walls.get(tag)
        if element is None or "base_z" not in row:
            continue
        storey = snapshot["storeys"][snapshot["walls"][tag]["storey"]]
        datum = snapshot["buildings"][storey["building"]]["elevation"]
        bounds = kernel_bounds(kernel, element)
        if bounds is None:
            problems.append("%s: the kernel has no geometry" % tag)
            continue
        if not (close(float(bounds[0][2]), row["base_z"] + datum, 1e-6) and close(float(bounds[1][2]), row["top_z"] + datum, 1e-6)):
            problems.append("%s: kernel z %.9f..%.9f, table %.9f..%.9f" % (tag, bounds[0][2], bounds[1][2], row["base_z"] + datum, row["top_z"] + datum))
        volume = kernel_volume(kernel, element)
        if "volume" in row and not close(volume, row["volume"], 1e-6):
            problems.append("%s: kernel volume %.9f, table %.9f" % (tag, volume, row["volume"]))
    return problems


def sweep_problems(model, snapshot, committed):
    """🧷️ The runs of every sweep: one `IfcMember` per run, all with the authored record in `Semio_WallSweep` (host, side, height, inset, material, profile JSON), `Run` counting from zero; the lengths, the
    cross-section and the gross volume of the runs add up to the shapely-adjudicated `sweeps` table, and the kernel volumes of the runs add up to its gross volume within 1e-6 (relative)."""
    problems = []
    kernel = settings()
    members = {}
    for element in model.by_type("IfcMember"):
        if element.ObjectType == "WallSweep":
            members.setdefault(element.Tag, []).append(element)
    if members.keys() != snapshot["wall_sweeps"].keys():
        problems.append("IfcMember sweeps %s in the file, %s in the snapshot" % (sorted(members), sorted(snapshot["wall_sweeps"])))
    for tag, sweep in snapshot["wall_sweeps"].items():
        runs = members.get(tag, [])
        written = sorted(int(rows_of(member, SWEEP_SET).get("Run", -1)) for member in runs)
        if written != list(range(len(runs))):
            problems.append("%s: runs %s are not numbered from zero" % (tag, written))
        for member in runs:
            rows = rows_of(member, SWEEP_SET)
            want = {"SweepId": tag, "Host": sweep["host"], "Side": sweep["side"], "Material": sweep["material"]}
            problems += ["%s: %s is %r, the snapshot says %r" % (tag, key, rows.get(key), value) for key, value in want.items() if rows.get(key) != value]
            problems += ["%s: %s is %r, the snapshot says %r" % (tag, key, rows.get(key), sweep[field]) for key, field in (("Height", "height"), ("Inset", "inset")) if not close(rows.get(key, float("nan")), sweep[field])]
            if json.loads(rows.get("Profile", "null")) != sweep["profile"]:
                problems.append("%s: the profile in the set differs from the snapshot" % tag)
        row = (committed or {}).get("sweeps", {}).get(tag)
        if row is None or not runs:
            continue
        quantities = [quantities_of(member, "Qto_MemberBaseQuantities") for member in runs]
        length, volume = sum(q.get("Length", 0.0) for q in quantities), sum(q.get("GrossVolume", 0.0) for q in quantities)
        if not (close(length, row["length"]) and close(volume, row["gross_volume"])):
            problems.append("%s: runs add up to length %.12g and gross volume %.12g, the table says %.12g and %.12g" % (tag, length, volume, row["length"], row["gross_volume"]))
        if any(not close(q.get("CrossSectionArea", float("nan")), row["section_area"]) for q in quantities):
            problems.append("%s: a cross-section differs from the table value %.12g" % (tag, row["section_area"]))
        kernel_total = sum(kernel_volume(kernel, member) or 0.0 for member in runs)
        if not close(kernel_total, row["gross_volume"], 1e-6):
            problems.append("%s: the kernel volume of the runs is %.12g, the table says %.12g" % (tag, kernel_total, row["gross_volume"]))
    return problems


def reveal_problems(model, snapshot, committed):
    """🪟️ The reveal of every opening: `Semio_OpeningReveal` on the opening element carries exactly the authored depth and material (an opening without a reveal carries none), and the kernel mesh of the
    window of an authored reveal spans the lateral range the shapely-adjudicated table gives its frame (`frame_back_y` .. `frame_front_y`, for a host axis along x)."""
    problems = []
    kernel = settings()
    voids = {element.Tag: element for element in model.by_type("IfcOpeningElement")}
    windows = {element.Tag: element for element in model.by_type("IfcWindow")}
    for tag, opening in snapshot["openings"].items():
        rows = rows_of(voids[tag], REVEAL_SET)
        depth, material = opening.get("reveal_depth"), opening.get("reveal_material")
        if depth is None and material is None:
            if rows:
                problems.append("%s: an opening without a reveal carries %s" % (tag, REVEAL_SET))
            continue
        if (depth is None) != ("RevealDepth" not in rows) or (depth is not None and not close(rows["RevealDepth"], depth)):
            problems.append("%s: RevealDepth %r, the snapshot says %r" % (tag, rows.get("RevealDepth"), depth))
        if rows.get("RevealMaterial") != material:
            problems.append("%s: RevealMaterial %r, the snapshot says %r" % (tag, rows.get("RevealMaterial"), material))
    for tag, row in (committed or {}).get("reveals", {}).items():
        bounds = kernel_bounds(kernel, windows[tag]) if tag in windows else None
        host = snapshot["walls"][snapshot["openings"][tag]["host"]]["axis"]["Line"]
        if bounds is None or host["start"]["y"] != host["end"]["y"]:
            problems.append("%s: no window geometry or a host axis that does not run along x" % tag)
            continue
        if not (close(float(bounds[0][1]), row["frame_back_y"] + host["start"]["y"], 1e-6) and close(float(bounds[1][1]), row["frame_front_y"] + host["start"]["y"], 1e-6)):
            problems.append("%s: the window spans y %.9f..%.9f, the table says %.9f..%.9f" % (tag, bounds[0][1], bounds[1][1], row["frame_back_y"], row["frame_front_y"]))
    return problems


def wall_depth_problems(model, snapshot, table, committed):
    """🧗️ Every place where the attached walls, sweeps and reveals of the file leave the snapshot or the committed wall-depth table."""
    return attach_problems(model, snapshot, table) + envelope_problems(model, snapshot, committed) + sweep_problems(model, snapshot, committed) + reveal_problems(model, snapshot, committed)


# endregion 🔖️WallDepth
# endregion 🔖️Measurement
