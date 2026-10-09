#!/usr/bin/env python3
"""🎭️ Third-party ORACLE (IfcOpenShell) for the `s.bim.model@1` inference `🎭️phase-visibility`.

The subject (Rust, `semio-s-artifact-bim-model`) derives, from the authored phase and storey of every element, which element ids a view of each storey shows under each view
phase. This file reproduces the table WITHOUT the subject's inference and without the snapshot's phases: it opens the committed IFC 2x3 export of the house with IfcOpenShell
0.8.4.post1, a library that has never seen this repository's writer, and reads what the file says.

* the construction phase of a product is the `Phase` row of its `Semio_Authoring` property set (`ifcopenshell.util.element.get_psets`); no row means new work;
* an `IfcOpeningElement` takes the phase of the wall or curtain wall it voids (`VoidsElements`), the rule the subject states as "an opening follows its host";
* the storey of a product is its spatial container (`get_container`) or, for a space, the storey that aggregates it (`get_aggregate`), mapped to the snapshot's storey id by name;
* the id of a product is the `Tag` the export writes (`ObjectType` for a space); parts such as stair flights, mullions and layers carry composite tags and are no elements.

The elements are grouped by storey and view phase: `all` lists every element of the storey, each other view phase exactly the elements of that phase. Audits beyond the table:
the phases partition the elements of a storey (every element is in exactly one non-`all` phase) and `all` is their union.

    python 🐍️.py check <path to 🧫️fixtures/🏗️ifc>
    python 🐍️.py table <path to 🧫️fixtures/🏗️ifc>      # prints the table read from the file

@see ../../🔮️oracles/🔣️.json — the registration of the oracle this file answers for
@see ../../🚪️io/📤️export/🏗️ifc/🦀️.rs — the writer that puts the phase into the file
"""

# region 🔖️Imports
import json
import sys
from pathlib import Path

import ifcopenshell
import ifcopenshell.util.element

# endregion 🔖️Imports


# region 🔖️Vocabulary
VIEW_PHASES = ["all", "existing", "new", "demolished", "temporary"]
"""🎭️ The view phase keys, `all` first, then the phases in the order of a project's life."""

ELEMENT_COLLECTIONS = ["walls", "curtain_walls", "columns", "beams", "slabs", "ceilings", "roofs", "stairs", "railings", "ramps", "spaces", "openings"]
"""🗂️ The snapshot collections whose ids are elements of a storey."""

# endregion 🔖️Vocabulary


# region 🔖️Reading
def identity(element):
    """🔖️ The model id the export carries in `Tag` (elements) or `ObjectType` (spatial elements)."""
    return element.Tag if element.is_a("IfcElement") else element.ObjectType


def phase_of_row(element):
    """🕰️ The phase row of one product, `New` when the file carries none."""
    return ifcopenshell.util.element.get_psets(element).get("Semio_Authoring", {}).get("Phase", "New")


def phase_of(element):
    """🕰️ The phase of a product: an opening element takes the phase of the wall it voids."""
    if element.is_a("IfcOpeningElement"):
        return phase_of_row(element.VoidsElements[0].RelatingBuildingElement)
    return phase_of_row(element)


def storey_of(element):
    """🪜️ The building storey that contains or aggregates a product."""
    holder = ifcopenshell.util.element.get_container(element) or ifcopenshell.util.element.get_aggregate(element)
    return holder if holder is not None and holder.is_a("IfcBuildingStorey") else None


def table(model, snapshot):
    """🎭️ `{storey id: {view phase: [element ids]}}` read from the file alone, restricted to the ids of the snapshot's elements."""
    storeys = {row["name"]: storey_id for storey_id, row in snapshot["storeys"].items()}
    known = {element_id for key in ELEMENT_COLLECTIONS for element_id in snapshot.get(key, {})}
    rows = {storey_id: {phase: [] for phase in VIEW_PHASES} for storey_id in snapshot["storeys"]}
    for product in model.by_type("IfcProduct"):
        element_id = identity(product)
        storey = storey_of(product)
        if element_id not in known or storey is None:
            continue
        phase = phase_of(product).lower()
        rows[storeys[storey.Name]]["all"].append(element_id)
        rows[storeys[storey.Name]][phase].append(element_id)
    return {storey_id: {phase: sorted(set(ids)) for phase, ids in phases.items()} for storey_id, phases in rows.items()}


# endregion 🔖️Reading


# region 🔖️Audit
def problems_of(result):
    """🩺️ Disagreements inside the table: the phases partition a storey and `all` is their union."""
    problems = []
    for storey, phases in result.items():
        seen = []
        for phase in VIEW_PHASES[1:]:
            seen += phases[phase]
        if len(seen) != len(set(seen)):
            problems.append("%s: an element is in two phases" % storey)
        if sorted(seen) != phases["all"]:
            problems.append("%s: the phases do not add up to all" % storey)
    return problems


# endregion 🔖️Audit


# region 🔖️Handlers
def open_inputs(ctx):
    """📸️ The snapshot and the opened IFC model the scenario names, resolved through the host."""
    uris = ctx.step_input_uris()
    snapshot_uri = next(uri for uri in uris if "📸️snapshot" in uri)
    ifc_uri = next(uri for uri in uris if uri.endswith(".ifc"))
    snapshot = json.loads(ctx.input_bytes(snapshot_uri).decode("utf-8"))
    model = ifcopenshell.file.from_string(ctx.input_bytes(ifc_uri).decode("utf-8"))
    return model, snapshot


def phases_handler(ctx):
    """🎭️ Oracle answer for `🎭️phase-visibility`, after the audit agreed with it."""
    from semio_repo_test import Outcome

    model, snapshot = open_inputs(ctx)
    payload = table(model, snapshot)
    problems = problems_of(payload)
    if problems:
        raise AssertionError("; ".join(problems))
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


def adapter():
    """🧭️ Registration in the ORACLE role only, by the feature's scenario ids."""
    from semio_repo_test import Adapter

    return Adapter("python").oracle("phases-house", phases_handler)


# endregion 🔖️Handlers


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` audits the committed house; `table` prints what the file says."""
    command, root = arguments[0], Path(arguments[1])
    house = root / "🏠️house"
    snapshot = json.loads((house / "📸️snapshot" / "🔣️.json").read_text(encoding="utf-8"))
    model = ifcopenshell.open(str(house / "🏠️house.ifc"))
    result = table(model, snapshot)
    if command == "table":
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0
    problems = problems_of(result)
    if len({phase for phases in result.values() for phase in VIEW_PHASES[1:] if phases[phase]}) < 3:
        problems.append("the committed house exercises fewer than 3 phases")
    for problem in problems:
        print(problem)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
# endregion 🔖️Standalone
