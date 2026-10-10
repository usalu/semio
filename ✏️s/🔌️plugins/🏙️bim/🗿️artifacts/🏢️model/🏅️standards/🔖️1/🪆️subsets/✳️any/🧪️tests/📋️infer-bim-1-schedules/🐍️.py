#!/usr/bin/env python3
"""📋️ Third-party ORACLE for the `s.bim.model@1` inference `📋️schedules`.

A schedule is authored (category, columns, sort, filter, grouping, storey and phase scope); its table is derived. The subject (Rust, `semio-s-artifact-bim-model`) reads the
quantities of the candidate elements from the model graph and builds the rows. This file reproduces every table of a committed snapshot with a library that has never seen
this repository:

* the quantities of walls, slabs, columns, beams and rooms come from the sibling oracle `../🧮️infer-bim-1-quantities` (`shapely` 2 / GEOS: layer areas as GEOS `intersection`s,
  lengths as GEOS lengths, rooms from `polygonize`), the finish areas of the rooms from `../🪣️infer-bim-1-finishes` (GEOS `Polygon.boundary.distance` for the opening faces);
  a door or a window measures what its type and its own size say, as the sibling does;
* the rows, filters, natural order, grouping, subtotals and totals are restated here from the documented rules alone: a number cell compares with a number as numbers (equality
  within 1e-9) and everything else as case-folded text; texts order naturally (digit runs as integers); empty cells order before numbers before texts; equal rows keep the element
  id order; a group closes with a subtotal row; a column flagged `total` sums per group and in a grand total row; sums are `math.fsum`, which is exactly rounded, not the subject's
  running f64 sum.

Audits beyond the table: the grand total of every summed column is the sum of the rows the table lists; every candidate that passes the filters is listed exactly once in an
itemized, ungrouped table; and the quantity law: a case directory with a `🦠️mutation/🔣️.json` is the model BEFORE one `setWallTop`; the oracle applies it itself, requires that
the schedules of the elements the edit does not touch are unchanged, that the edited wall's rows are the only ones that move, and that every summed total moves by exactly what its
rows moved. The committed table of such a case is the table of the model AFTER the edit.

The committed expectations under `🧫️fixtures/💡️inferences/📋️schedules/<case>/💡️inference/📋️schedules/🔣️.json` are WRITTEN by this file (`write`), never by hand, and the Rust
subject is compared against them.

    python 🐍️.py check <path to 🧫️fixtures/💡️inferences/📋️schedules>
    python 🐍️.py write <path to 🧫️fixtures/💡️inferences/📋️schedules>

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/BIM-PLUGIN/r9-audit-completeness.md — work package 13
"""

# region 🔖️Imports
import copy
import importlib.util
import json
import math
import re
import sys
from pathlib import Path

import shapely

# endregion 🔖️Imports


# region 🔖️Vocabulary
EQUAL = 1e-9
CATEGORIES = {"Wall": "walls", "CurtainWall": "curtain_walls", "Slab": "slabs", "Roof": "roofs", "Column": "columns", "Beam": "beams", "Stair": "stairs", "Railing": "railings", "Space": "spaces", "Finish": "spaces"}
OPENINGS = {"Window": "Window", "Door": "Door", "Void": "Void"}
MATERIAL_SOURCES = ["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces"]
STOREYED = ["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "stairs", "railings", "spaces"]
PHASED = STOREYED
TYPES = {"walls": ("wall_type", "wall_types"), "curtain_walls": ("curtain_wall_type", "curtain_wall_types"), "slabs": ("slab_type", "slab_types"), "roofs": ("roof_type", "roof_types"), "columns": ("column_type", "column_types"), "beams": ("beam_type", "beam_types")}


def load_sibling(case, name):
    """🧭️ Imports the oracle module of a sibling case by path (the cases are folders, not packages)."""
    key = "bim_oracle_" + name
    if key not in sys.modules:
        spec = importlib.util.spec_from_file_location(key, Path(__file__).resolve().parents[1] / case / "🐍️.py")
        module = importlib.util.module_from_spec(spec)
        sys.modules[key] = module
        spec.loader.exec_module(module)
    return sys.modules[key]


def quantities_oracle():
    return load_sibling("🧮️infer-bim-1-quantities", "quantities")


def finishes_oracle():
    return load_sibling("🪣️infer-bim-1-finishes", "finishes")


def variant(value):
    """🔀️ `(tag, body)` of an externally tagged enum value."""
    return next(iter(value.items()))


def snake(token):
    """🐍️ `GrossSideArea` → `gross_side_area`."""
    return re.sub(r"(?<!^)(?=[A-Z])", "_", token).lower()


# endregion 🔖️Vocabulary


# region 🔖️Facts
def storey_id(snapshot, element):
    """🪜️ The storey an element stands on; an opening stands on the storey of its host."""
    for collection in STOREYED:
        row = snapshot.get(collection, {}).get(element)
        if row is not None:
            return row["storey"]
    opening = snapshot["openings"].get(element)
    return storey_id(snapshot, opening["host"]) if opening else None


def phase_of(snapshot, element):
    """🕰️ The phase of an element: its own, an opening's is its host's, an element without a phase counts as new."""
    for collection in PHASED:
        row = snapshot.get(collection, {}).get(element)
        if row is not None:
            return row["phase"]
    opening = snapshot["openings"].get(element)
    return phase_of(snapshot, opening["host"]) if opening else "New"


def row_of(snapshot, element):
    for collection in MATERIAL_SOURCES:
        row = snapshot.get(collection, {}).get(element)
        if row is not None:
            return collection, row
    return None, None


def type_name(snapshot, element):
    collection, row = row_of(snapshot, element)
    if collection in TYPES:
        field, types = TYPES[collection]
        found = snapshot[types].get(row[field])
        return found["name"] if found else row[field]
    if collection == "openings":
        tag, body = variant(row["kind"])
        if tag in ("Window", "Door"):
            types = snapshot["window_types" if tag == "Window" else "door_types"]
            key = body["window_type" if tag == "Window" else "door_type"]
            return types[key]["name"] if key in types else key
    return ""


def host_name(snapshot, element):
    opening = snapshot["openings"].get(element)
    if not opening:
        return ""
    host = snapshot["walls"].get(opening["host"]) or snapshot["curtain_walls"].get(opening["host"])
    return host["name"] if host else opening["host"]


def hand(swing, flipped):
    return "left" if (swing == "Left") != flipped else "right"


def opening_size(snapshot, opening):
    width, height, _ = quantities_oracle().resolve(snapshot, opening)
    return width or 0.0, height or 0.0


# endregion 🔖️Facts


# region 🔖️Cells
def text(value):
    """🔤️ A text cell; the empty text is the empty cell."""
    return value if value else None


def display(cell):
    """🔤️ The text a cell shows: nothing for the empty cell, a number in shortest round-trip form."""
    if cell is None:
        return ""
    if isinstance(cell, str):
        return cell
    return str(int(cell)) if float(cell).is_integer() else repr(float(cell))


def property_cell(snapshot, element, key):
    """🗃️ The typed value of a property as a cell, empty where the element has none."""
    found = snapshot.get("properties", {}).get(element, {}).get(key["set"], {}).get(key["name"])
    if found is None:
        return None
    tag, body = variant(found)
    if tag == "Text":
        return text(body["value"])
    if tag == "Boolean":
        return "true" if body["value"] else "false"
    return float(body["value"])


def source_rows(snapshot, schedule, quantities, finishes):
    """🧱️ The source rows of a schedule: `(element, quantity row, layer, finish)` in element id order, one per element, per layer with a material (Material) or per finished surface (Finish)."""
    category = schedule["category"]
    if category in OPENINGS:
        ids = [element for element, opening in snapshot["openings"].items() if variant(opening["kind"])[0] == category]
    elif category == "Material":
        ids = sorted({element for collection in MATERIAL_SOURCES for element in snapshot.get(collection, {})})
    else:
        ids = list(snapshot[CATEGORIES[category]])
    ids = [element for element in ids if (not schedule["storeys"] or storey_id(snapshot, element) in schedule["storeys"]) and (not schedule["phases"] or phase_of(snapshot, element) in schedule["phases"])]
    rows = []
    for element in sorted(ids):
        quantity = quantity_of(snapshot, element, quantities)
        if quantity is None:
            continue
        if category == "Material":
            rows += [(element, quantity, layer, None) for layer in quantity["layers"] if layer["material"]]
        elif category == "Finish":
            rows += [(element, quantity, None, finish) for finish in finishes.get(element, [])]
        else:
            rows.append((element, quantity, None, None))
    return rows


def quantity_of(snapshot, element, quantities):
    """🧮️ The quantity row of an element: the sibling oracle's row for the closed-form kinds, the size of its type for an opening."""
    if element in quantities:
        return quantities[element]
    opening = snapshot["openings"].get(element)
    if opening is None:
        return None
    tag = variant(opening["kind"])[0]
    width, height = opening_size(snapshot, opening)
    row = quantities_oracle().empty(tag, storey_id(snapshot, element) or "", phase=phase_of(snapshot, element))
    row.update(width=width, height=height, perimeter=2.0 * (width + height), gross_area=width * height, net_area=width * height)
    row["kind"] = tag
    return row


def finish_rows(snapshot, quantities):
    """🎨️ `{space: [(surface, material, area)]}` of the resolved rooms: the material the space names, the area the finishes oracle measures with GEOS."""
    found = {}
    measured = finishes_oracle().tables(snapshot)
    for space_id, areas in measured.items():
        space = snapshot["spaces"][space_id]
        found[space_id] = [
            {"surface": "floor", "material": space.get("floor_finish") or "", "area": areas["floor_area"]},
            {"surface": "wall", "material": space.get("wall_finish") or "", "area": areas["wall_area"]},
            {"surface": "ceiling", "material": space.get("ceiling_finish") or "", "area": areas["ceiling_area"]},
        ]
    return {space_id: rows for space_id, rows in found.items() if space_id in quantities}


def kind_key(quantity):
    return quantities_oracle().KEYS.get(quantity["kind"], quantity["kind"].lower())


def field_cell(snapshot, source, field):
    """🧾️ The cell of a built-in field (snake case token) for one source row."""
    element, quantity, layer, finish = source
    storey = snapshot["storeys"].get(storey_id(snapshot, element) or "")
    collection, row = row_of(snapshot, element)
    opening_kind = variant(row["kind"]) if collection == "openings" else (None, None)
    space = row if collection == "spaces" else None
    names = lambda material: snapshot["materials"].get(material, {}).get("name", material)
    numbers = {name: quantity[name] for name in ("length", "width", "height", "perimeter", "gross_side_area", "opening_area", "net_side_area", "gross_area", "net_area", "surface_area", "gross_volume", "net_volume", "mass")}
    if field == "id":
        return element
    if field == "name":
        return text(row["name"]) if row else None
    if field == "kind":
        return kind_key(quantity)
    if field == "storey":
        return text(storey["name"]) if storey else None
    if field == "level":
        return float(storey["level"]) if storey else None
    if field == "type":
        return text(type_name(snapshot, element))
    if field == "phase":
        return phase_of(snapshot, element).lower()
    if field == "material":
        if layer is not None:
            return names(layer["material"])
        if finish is not None:
            return text(names(finish["material"]) if finish["material"] else "")
        joined = []
        for entry in quantity["layers"]:
            if entry["material"] and names(entry["material"]) not in joined:
                joined.append(names(entry["material"]))
        return text(", ".join(joined))
    if field == "host":
        return text(host_name(snapshot, element))
    if field == "number":
        return text(space["number"]) if space else None
    if field == "usage":
        return text(space["usage"]) if space else None
    if field == "surface":
        return finish["surface"] if finish is not None else None
    if field == "swing":
        if opening_kind[0] != "Door":
            return None
        door = snapshot["door_types"].get(opening_kind[1]["door_type"])
        return hand(door["swing"], row["flip_hand"]) if door else None
    if field == "leaves":
        if opening_kind[0] != "Door":
            return None
        door = snapshot["door_types"].get(opening_kind[1]["door_type"])
        return ("double" if door["leaves"] == "Double" else "single") if door else None
    if field == "panes":
        if opening_kind[0] != "Window":
            return None
        window = snapshot["window_types"].get(opening_kind[1]["window_type"])
        return float(window["panes"]) if window else None
    if field == "count":
        return float(quantity["count"])
    if field == "risers":
        return float(quantity["risers"])
    if field in numbers:
        return float(numbers[field])
    if field == "finish_area":
        return float(finish["area"]) if finish is not None else None
    layer_fields = {"thickness": "thickness", "layer_area": "area", "layer_volume": "volume", "layer_mass": "mass"}
    if field in layer_fields:
        return float(layer[layer_fields[field]]) if layer is not None else None
    raise AssertionError("unknown schedule field %s" % field)


def cell_of(snapshot, source, key):
    tag, body = variant(key)
    if tag == "Field":
        return field_cell(snapshot, source, snake(body["field"]))
    return property_cell(snapshot, source[0], body)


# endregion 🔖️Cells


# region 🔖️Order
def natural_key(value):
    """🔤️ Natural order of a text: case-folded, digit runs as integers, ties by the raw text."""
    folded = value.lower()
    runs = [(0, int(run)) if run[0] in "0123456789" else (1, run) for run in re.findall(r"[0-9]+|[^0-9]+", folded)]
    return (runs, value)


def cell_key(cell):
    """↕️ Empty cells first, then numbers by value, then texts in natural order."""
    if cell is None:
        return (0, 0, ([], ""))
    if isinstance(cell, str):
        return (2, 0, natural_key(cell))
    return (1, cell, ([], ""))


def natural_cmp(left, right):
    a, b = natural_key(left), natural_key(right)
    return (a > b) - (a < b)


def passes(cell, op, value):
    """🔎️ Whether a cell satisfies a filter: numbers against a numeric value as numbers, everything else as case-folded text."""
    empty = cell is None
    if op == "Empty":
        return empty
    if op == "NotEmpty":
        return not empty
    try:
        number = float(value.strip())
        number = number if math.isfinite(number) else None
    except ValueError:
        number = None
    if isinstance(cell, float) and number is not None and op != "Contains":
        equal = abs(cell - number) < EQUAL
        return {"Equals": equal, "NotEquals": not equal, "Greater": cell > number and not equal, "GreaterOrEqual": cell > number or equal, "Less": cell < number and not equal, "LessOrEqual": cell < number or equal}[op]
    shown, wanted = display(cell).lower(), value.strip().lower()
    order = natural_cmp(shown, wanted)
    return {"Equals": shown == wanted, "NotEquals": shown != wanted, "Contains": wanted in shown, "Greater": order > 0, "GreaterOrEqual": order >= 0, "Less": order < 0, "LessOrEqual": order <= 0}[op]


# endregion 🔖️Order


# region 🔖️Table
def total_of(lines, index):
    """➕️ The sum of the number cells of a column over the lines (empty when it has none), exactly rounded."""
    numbers = [line["cells"][index] for line in lines if isinstance(line["cells"][index], float)]
    return math.fsum(numbers) if numbers else None


def common_cell(lines, index):
    first = lines[0]["cells"][index]
    return first if all(line["cells"][index] == first for line in lines) else None


def partition(lines, keys):
    parts = []
    for line in lines:
        for part in parts:
            if all(part[0]["cells"][key] == line["cells"][key] for key in keys):
                part.append(line)
                break
        else:
            parts.append([line])
    return parts


def elements_of(lines):
    found = []
    for line in lines:
        if line["element"] not in found:
            found.append(line["element"])
    return found


def summary(part, shown, summed, level, fill):
    cells = []
    for index, total in enumerate(summed):
        if index in shown:
            cells.append(part[0]["cells"][index])
        elif total:
            cells.append(total_of(part, index))
        else:
            cells.append(common_cell(part, index) if fill else None)
    return {"kind": "Group", "level": level, "elements": elements_of(part), "cells": cells}


def itemized(lines, depth, groups, summed, out):
    if depth == len(groups):
        out += [{"kind": "Item", "level": depth, "elements": [line["element"]], "cells": line["cells"][: len(summed)]} for line in lines]
        return
    for part in partition(lines, [groups[depth]]):
        itemized(part, depth + 1, groups, summed, out)
        out.append(summary(part, set(groups[: depth + 1]), summed, depth, False))


def table_of(snapshot, schedule, quantities, finishes):
    """📋️ The table of one schedule: `{items, rows}` with rows `{kind, level, elements, cells}`."""
    keys = [column["key"] for column in schedule["columns"]]
    for key in [entry["key"] for entry in schedule["sort"] + schedule["filter"] + schedule["group"]]:
        if key not in keys:
            keys.append(key)
    lines = [{"element": source[0], "cells": [cell_of(snapshot, source, key) for key in keys]} for source in source_rows(snapshot, schedule, quantities, finishes)]
    for entry in schedule["filter"]:
        index = keys.index(entry["key"])
        lines = [line for line in lines if passes(line["cells"][index], entry["op"], entry["value"])]
    groups = [keys.index(entry["key"]) for entry in schedule["group"]]
    orders = [(index, False) for index in groups] + [(keys.index(entry["key"]), entry["descending"]) for entry in schedule["sort"]]
    for index, descending in reversed(orders):
        lines.sort(key=lambda line, index=index: cell_key(line["cells"][index]), reverse=descending)
    summed = [column["total"] for column in schedule["columns"]]
    rows = []
    if schedule["itemize"]:
        itemized(lines, 0, groups, summed, rows)
    elif lines:
        parts = groups if groups else [index for index, total in enumerate(summed) if not total]
        rows += [summary(part, set(parts), summed, 0, True) for part in partition(lines, parts)]
    if any(summed) and lines:
        rows.append({"kind": "Total", "level": 0, "elements": [], "cells": [total_of(lines, index) if total else None for index, total in enumerate(summed)]})
    return {"items": len(lines), "rows": rows}


def tables(snapshot):
    """📋️ The `📋️schedules` table of a snapshot: one table per schedule, in id order."""
    quantities = quantities_oracle().table(snapshot)["elements"]
    finishes = finish_rows(snapshot, quantities)
    return {schedule_id: table_of(snapshot, schedule, quantities, finishes) for schedule_id, schedule in sorted(snapshot["schedules"].items())}


# endregion 🔖️Table


# region 🔖️Audit
def listed(schedule, table):
    """🧾️ The rows whose cells a summed column adds up: the items when itemized, else the collapsed rows of the outermost level."""
    return [row for row in table["rows"] if (row["kind"] == "Item" if schedule["itemize"] else row["kind"] == "Group" and row["level"] == 0)]


def close(left, right):
    return abs(left - right) <= 1e-9 * max(1.0, abs(left), abs(right))


def problems_of(snapshot):
    """🧪️ The sum law of every table, and the coverage of the plain ones."""
    problems = []
    quantities = quantities_oracle().table(snapshot)["elements"]
    finishes = finish_rows(snapshot, quantities)
    for schedule_id, table in tables(snapshot).items():
        schedule = snapshot["schedules"][schedule_id]
        total = next((row for row in table["rows"] if row["kind"] == "Total"), None)
        if (total is not None) != (table["items"] > 0 and any(column["total"] for column in schedule["columns"])):
            problems.append("%s: a total row exists exactly when something is summed" % schedule_id)
        if total is not None:
            for index, column in enumerate(schedule["columns"]):
                if not column["total"]:
                    continue
                numbers = [row["cells"][index] for row in listed(schedule, table) if isinstance(row["cells"][index], float)]
                expected = math.fsum(numbers) if numbers else None
                if (expected is None) != (total["cells"][index] is None) or (expected is not None and not close(expected, total["cells"][index])):
                    problems.append("%s: the total of column %d is %r, its rows add up to %r" % (schedule_id, index, total["cells"][index], expected))
        if schedule["itemize"] and not schedule["filter"] and not schedule["group"] and schedule["category"] != "Material":
            covered = sorted(source[0] for source in source_rows(snapshot, schedule, quantities, finishes))
            if sorted(element for row in table["rows"] if row["kind"] == "Item" for element in row["elements"]) != covered:
                problems.append("%s: every candidate is listed exactly once" % schedule_id)
    return problems


def apply_mutation(snapshot, mutation):
    """🦠️ The model after a `setWallTop`: the one authored field of the one wall. An independent re-implementation of the rule (no refusal logic): the committed cases only carry mutations the subject accepts."""
    assert mutation["mutation"] == "setWallTop", mutation["mutation"]
    changed = copy.deepcopy(snapshot)
    changed["walls"][mutation["id"]]["top"] = mutation["top"]
    return changed


def mutation_problems(snapshot, mutation):
    """🧪️ The quantity law: the schedules of the elements the edit does not touch are unchanged, the edited wall's rows are the only ones that move, and every summed total moves by what those rows moved."""
    problems, edited = [], mutation["id"]
    one, two = tables(snapshot), tables(apply_mutation(snapshot, mutation))
    moved = False
    for schedule_id, before in one.items():
        schedule, after = snapshot["schedules"][schedule_id], two[schedule_id]
        if not schedule["itemize"]:
            continue
        key = lambda table: {tuple(row["elements"]): row["cells"] for row in table["rows"] if row["kind"] == "Item"}
        was, now = key(before), key(after)
        if set(was) != set(now):
            problems.append("%s: the edit of %s changed which elements are listed" % (schedule_id, edited))
            continue
        for element, cells in was.items():
            if element == (edited,):
                moved = moved or cells != now[element]
            elif cells != now[element]:
                problems.append("%s: the row of %s changed with the edit of %s" % (schedule_id, element[0], edited))
        totals = lambda table: next((row["cells"] for row in table["rows"] if row["kind"] == "Total"), None)
        if totals(before) is None:
            continue
        for index, column in enumerate(schedule["columns"]):
            if not column["total"]:
                continue
            delta = lambda cells: sum(c[index] for e, c in cells.items() if e == (edited,) and isinstance(c[index], float))
            moved_by = delta(now) - delta(was)
            if totals(before)[index] is not None and not close(totals(after)[index] - totals(before)[index], moved_by):
                problems.append("%s: the total of column %d moved by %r, the rows of %s by %r" % (schedule_id, index, totals(after)[index] - totals(before)[index], edited, moved_by))
    if not moved:
        problems.append("the edit of %s moved no row of any schedule" % edited)
    return problems


# endregion 🔖️Audit


# region 🔖️Handlers
def compare(expected, actual, path=""):
    """⚖️ Differences between a committed expectation and a computed table (the levels oracle's comparison)."""
    return load_sibling("🪜️infer-bim-1-levels-and-wall-heights", "levels").compare(expected, actual, path)


def case_snapshot(ctx):
    """📸️ The snapshot fixture URI a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "📸️snapshot" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def case_mutation(ctx):
    """🦠️ The mutation payload a scenario names, resolved through the host."""
    uri = next(candidate for candidate in ctx.step_input_uris() if "🦠️mutation" in candidate)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def answer(payload):
    from semio_repo_test import Outcome

    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def schedules_handler(ctx):
    """📋️ Oracle answer for `📋️schedules`, after the sum law and the coverage agreed with it."""
    snapshot = case_snapshot(ctx)
    problems = problems_of(snapshot)
    if problems:
        raise AssertionError("; ".join(problems))
    return answer(tables(snapshot))


def wall_edit_handler(ctx):
    """✂️ Oracle answer for the schedules of a model after `setWallTop`: the independent edit, the quantity law and the sum law agree."""
    snapshot, mutation = case_snapshot(ctx), case_mutation(ctx)
    edited = apply_mutation(snapshot, mutation)
    problems = mutation_problems(snapshot, mutation) + problems_of(edited)
    if problems:
        raise AssertionError("; ".join(problems))
    return answer(tables(edited))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("schedules-house", schedules_handler).oracle("schedules-wall-edit", wall_edit_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every case under a fixtures root with its committed expectation; `write` regenerates it."""
    command, root = arguments[0], Path(arguments[1])
    failures = []
    for snapshot_path in sorted(root.glob("*/📸️snapshot/🔣️.json")):
        case = snapshot_path.parents[1]
        snapshot = json.loads(snapshot_path.read_text(encoding="utf-8"))
        mutation_path = case / "🦠️mutation" / "🔣️.json"
        if mutation_path.exists():
            mutation = json.loads(mutation_path.read_text(encoding="utf-8"))
            failures += ["%s: %s" % (case.name, problem) for problem in mutation_problems(snapshot, mutation)]
            snapshot = apply_mutation(snapshot, mutation)
        failures += ["%s: %s" % (case.name, problem) for problem in problems_of(snapshot)]
        computed = tables(snapshot)
        target = case / "💡️inference" / "📋️schedules" / "🔣️.json"
        if command == "write":
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(json.dumps(computed, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        else:
            failures += ["%s: %s" % (case.name, problem) for problem in compare(json.loads(target.read_text(encoding="utf-8")), computed, "schedules")]
        print("%s: shapely %s, %d schedules, %d rows" % (case.name, shapely.__version__, len(computed), sum(len(table["rows"]) for table in computed.values())))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("%s: %s" % (command, "%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
