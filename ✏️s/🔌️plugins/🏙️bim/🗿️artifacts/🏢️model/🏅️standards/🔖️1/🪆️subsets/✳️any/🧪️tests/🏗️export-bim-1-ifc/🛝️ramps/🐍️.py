"""🛝️ The ramp audit: the ramp flights and landings, their base quantities and railings against the ramp-runs table."""

from __future__ import annotations

# region 🔖️Imports
import json
import ifcopenshell.util.element

from export_bim_ifc_kernel import TOLERANCE, identity, settings, kernel_volume, authored, quantities_of, close
# endregion 🔖️Imports


# region 🔖️Measurement
def exported_ramps(snapshot, runs):
    """🛝️ The ramps the export writes: those the committed `ramp-runs` table resolves (their storey exists) with a path of positive length."""
    return {ramp_id: run for ramp_id, run in (runs or {}).items() if ramp_id in snapshot.get("ramps", {}) and run["length"] > TOLERANCE}


def ramp_type(ramp, run):
    """🛝️ `IfcRampTypeEnum` of a ramp: a curved path is a spiral, up to two landings make one straight run, three landings two straight runs, more is user defined."""
    if any(vertex["bulge"] != 0 for vertex in ramp["path"][:-1]):
        return "SPIRAL_RAMP"
    landings = len(run["landings"])
    return "STRAIGHT_RUN_RAMP" if landings <= 2 else "TWO_STRAIGHT_RUN_RAMP" if landings == 3 else "USERDEFINED"


def ramp_problems(model, snapshot, runs):
    """🛝️ Every `IfcRamp` of the file against the committed `ramp-runs` table (shapely-adjudicated): one ramp per resolved ramp in its storey, one `IfcRampFlight` per sloped flight and one `LANDING` slab per
    landing, all aggregated by the ramp with `IfcRelAggregates`; the ramp's base quantities (length, width, rise, plan area) and every flight and landing quantity equal the run; one `IfcRailing` per ramp with a side
    railing; the authored record travels back unchanged; and the kernel volumes of the aggregated parts add up to the written gross volume."""
    problems = []
    expected = exported_ramps(snapshot, runs)
    ramps = {element.Tag: element for element in model.by_type("IfcRamp", include_subtypes=False)}
    if ramps.keys() != expected.keys():
        problems.append("IfcRamp: %s in the file, %s resolved by the ramp-runs table" % (sorted(ramps), sorted(expected)))
    kernel = settings()
    flights_total = landings_total = 0
    for ramp_id, run in expected.items():
        authored, element = snapshot["ramps"][ramp_id], ramps.get(ramp_id)
        if element is None:
            continue
        if element.ShapeType != ramp_type(authored, run):
            problems.append("%s: ramp type %s, expected %s" % (ramp_id, element.ShapeType, ramp_type(authored, run)))
        container = ifcopenshell.util.element.get_container(element)
        if container is None or identity(container) != authored["storey"]:
            problems.append("%s is not contained in its storey %s" % (ramp_id, authored["storey"]))
        parts = ifcopenshell.util.element.get_parts(element)
        flights = sorted((part for part in parts if part.is_a("IfcRampFlight")), key=lambda part: part.Tag)
        landings = sorted((part for part in parts if part.is_a("IfcSlab") and part.PredefinedType == "LANDING"), key=lambda part: part.Tag)
        rails = [part for part in parts if part.is_a("IfcRailing")]
        flights_total, landings_total = flights_total + len(flights), landings_total + len(landings)
        if len(flights) != len(run["flights"]) or len(landings) != len(run["landings"]):
            problems.append("%s: %d flights and %d landings in the file, %d and %d in the run" % (ramp_id, len(flights), len(landings), len(run["flights"]), len(run["landings"])))
        if len(parts) != len(flights) + len(landings) + len(rails):
            problems.append("%s: %d aggregated parts are neither flights, landings nor railings" % (ramp_id, len(parts) - len(flights) - len(landings) - len(rails)))
        if len(rails) != (1 if authored["railing_left"] or authored["railing_right"] else 0):
            problems.append("%s: %d railings in the file for the sides asked for" % (ramp_id, len(rails)))
        for part, row in zip(flights, run["flights"]):
            values = quantities_of(part, "Qto_RampFlightBaseQuantities")
            if not (close(values.get("Length", -1), row["length"]) and close(values.get("Width", -1), run["width"])):
                problems.append("%s: flight %s quantities %s, run %.12g x %.12g" % (ramp_id, part.Tag, values, row["length"], run["width"]))
        for part, row in zip(landings, run["landings"]):
            values = quantities_of(part, "Qto_SlabBaseQuantities")
            if not (close(values.get("Length", -1), row["length"]) and close(values.get("Width", -1), run["width"])):
                problems.append("%s: landing %s quantities %s, run %.12g x %.12g" % (ramp_id, part.Tag, values, row["length"], run["width"]))
        values = quantities_of(element, "Qto_RampBaseQuantities")
        for name, want in (("Length", run["length"]), ("Width", run["width"]), ("Height", run["rise"]), ("GrossArea", run["width"] * run["length"])):
            if name not in values or not close(values[name], want, 1e-9 if name != "GrossArea" else 1e-6):
                problems.append("%s: Qto_RampBaseQuantities.%s is %s, the run says %.12g" % (ramp_id, name, values.get(name), want))
        volumes = [kernel_volume(kernel, part) for part in flights + landings]
        if any(volume is None for volume in volumes):
            problems.append("%s: the kernel has no geometry for a flight or a landing" % ramp_id)
        elif "GrossVolume" in values and not close(sum(volumes), values["GrossVolume"], 1e-6):
            problems.append("%s: the kernel volume of the parts is %.12g, the written GrossVolume %.12g" % (ramp_id, sum(volumes), values["GrossVolume"]))
        record = ifcopenshell.util.element.get_psets(element).get("Semio_Authoring", {}).get("Ramp")
        if record is None or json.loads(record) != authored:
            problems.append("%s: the authored ramp record in Semio_Authoring differs from the snapshot" % ramp_id)
    landing_slabs = sum(1 for slab in model.by_type("IfcSlab", include_subtypes=False) if slab.PredefinedType == "LANDING")
    if (flights_total, landings_total) != (len(model.by_type("IfcRampFlight", include_subtypes=False)), landing_slabs):
        problems.append("the file holds flights or landing slabs that no ramp aggregates")
    return problems


# endregion 🔖️Measurement
