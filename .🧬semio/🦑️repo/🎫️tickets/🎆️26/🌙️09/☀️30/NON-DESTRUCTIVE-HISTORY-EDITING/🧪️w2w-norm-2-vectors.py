"""🧪️ W2-W-norm-2: stage, cross-check and install the committed mutation vectors of EN 1996, DIN EN 16798,
DIN V 18599 and DIN 4108.

  python3 🧪️w2w-norm-2-vectors.py stage <artifact>     staged mutation (+ before) per kind for the Rust producer
  python3 🧪️w2w-norm-2-vectors.py check <artifact>     independent Python engine against the Rust-produced bundles
  python3 🧪️w2w-norm-2-vectors.py install <artifact>   pretty-print the agreed bundles into 🧫️fixtures/🧬️mutations

Rust produces every before/after/diff/outcome (temporary generator mounted into the crate); a bundle is
installed only after the independent Python engine reaches the same after-snapshot and restores the
before-snapshot through its own inverse.
"""

import copy
import importlib.util
import json
import os
import re
import shutil
import sys

ROOT = "/Users/ueli/Documents/semio"
TICKET = f"{ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING"
GEN = f"{TICKET}/🗑️generated/w2w-norm-2"
ARTIFACTS = {"en1996": "🪨️en1996", "din16798": "🌬️din16798", "din18599": "⚡️din18599", "din4108": "🧱️din4108"}


def subset(artifact):
    return f"{ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/{ARTIFACTS[artifact]}/🏅️standards/🔖️1/🪆️subsets/✳️any"


def load(path):
    with open(path) as handle:
        return json.load(handle)


# region Leaves
def leaves(artifact):
    """kind -> {dir, props, required, wire(payload) -> aggregate wire}"""
    root = f"{subset(artifact)}/🧬️schema/🧬️mutations"
    aggregate = load(f"{root}/🔣️.json")
    by_id = {}
    for name in sorted(os.listdir(root)):
        path = f"{root}/{name}/🧬️schema/🔣️.json"
        if os.path.exists(path):
            schema = load(path)
            by_id[schema["$id"]] = (name, schema)
    out = {}
    for branch in aggregate["oneOf"]:
        if "$ref" in branch:
            name, schema = by_id[branch["$ref"]]
            tag = schema["properties"]["mutation"]["const"]
            wire = (lambda tag: lambda payload: {"mutation": tag, **payload})(tag)
        else:
            variant = branch["required"][0]
            name, schema = by_id[branch["properties"][variant]["$ref"]]
            wire = (lambda variant: lambda payload: {variant: payload})(variant)
        kind = schema["$id"].split("/")[-2]
        props = [key for key in schema["properties"] if key != "mutation"]
        out[kind] = {"dir": name, "props": props, "wire": wire, "schema": schema}
    return out


def payload(leaf, values):
    """Maps the spec's short keys onto the leaf's own wire property names, in schema order."""
    props = leaf["props"]
    news = [key for key in props if key.startswith("new")]
    element = [key for key in props if not key.startswith("new") and not key.endswith("Index") and key not in ("index", "from", "to") and not key.endswith("Id")]
    out = {}
    for key in props:
        if key in values:
            out[key] = values[key]
        elif key in news and "new" in values and len(news) == 1:
            out[key] = values["new"]
        elif key in element and "element" in values and len(element) == 1:
            out[key] = values["element"]
    missing = [key for key in leaf["schema"].get("required", []) if key != "mutation" and key not in out]
    if missing:
        raise SystemExit(f"{leaf['dir']}: spec misses {missing} (props {props}, values {list(values)})")
    return out
# endregion Leaves


# region Specs
def dumped(artifact, name):
    return load(f"{GEN}/dump/{artifact}/{name}.json")


def spec(artifact):
    """kind -> (scenario directory, base constructor or snapshot, values)"""
    return SPECS[artifact](artifact)


def spec_en1996(artifact):
    clay = "compliant_clay_wall"
    wall = dumped(artifact, clay)["walls"][0]
    loads = wall["loadCases"][0]
    return {
        "change-annex": ("🌍️switches-annex-to-en", clay, {"new": "En"}),
        "change-masonry-class": ("🏭️applies-change-masonry-class", clay, {"new": "Class2"}),
        "change-design-situation": ("🌋️switches-the-design-situation-to-seismic", clay, {"new": "Seismic"}),
        "change-storeys": ("🏢️applies-change-storeys", clay, {"new": 3}),
        "insert-wall": ("➕️inserts-a-wall", clay, {"index": 1, "element": dict(copy.deepcopy(wall), id="wall-south", labelEn="South load-bearing wall", labelDe="Tragende Südwand")}),
        "remove-wall": ("➖️removes-first-wall", clay, {"index": 0}),
        "change-wall-thickness": ("↕️thickens-first-wall", clay, {"index": 0, "new": 0.49}),
        "change-wall-height": ("↕️shortens-first-wall", clay, {"index": 0, "new": 2.5}),
        "change-wall-length": ("↔️applies-change-wall-length", clay, {"index": 0, "new": 6.25}),
        "change-wall-type": ("🧱applies-change-wall-type", clay, {"index": 0, "new": "Shear"}),
        "change-support-sides": ("🧱sets-four-sided-support", "noncompliant_multi_fail", {"index": 0, "new": 4}),
        "change-slab-bearing-depth": ("📐️applies-change-slab-bearing-depth", clay, {"index": 0, "new": 0.15}),
        "change-eccentricity-top": ("↘️applies-change-eccentricity-top", clay, {"index": 0, "new": 0.025}),
        "change-eccentricity-bottom": ("↗️applies-change-eccentricity-bottom", clay, {"index": 0, "new": 0.01}),
        "change-unit-group": ("🧱applies-change-unit-group", clay, {"index": 0, "new": "Group2"}),
        "change-unit-material": ("🧱applies-change-unit-material", clay, {"index": 0, "new": "CalciumSilicate"}),
        "change-unit-fb": ("🧱raises-unit-strength", clay, {"index": 0, "new": 25000000.0}),
        "change-unit-length": ("🧱applies-change-unit-length", clay, {"index": 0, "new": 0.248}),
        "change-unit-width": ("🧱applies-change-unit-width", clay, {"index": 0, "new": 0.3}),
        "change-unit-height": ("🧱applies-change-unit-height", clay, {"index": 0, "new": 0.238}),
        "change-mortar-type": ("🧈applies-change-mortar-type", clay, {"index": 0, "new": "ThinLayer"}),
        "change-mortar-class": ("🧈upgrades-mortar-to-m20", clay, {"index": 0, "new": "M20"}),
        "change-fm": ("🧈applies-change-fm", clay, {"index": 0, "new": 12500000.0}),
        "change-bed-joint-thickness": ("🥪️applies-change-bed-joint-thickness", clay, {"index": 0, "new": 0.01}),
        "change-reinforced": ("🔩applies-change-reinforced", clay, {"index": 0, "new": True}),
        "change-as-vertical": ("🔩applies-change-as-vertical", "reinforced_wall_example", {"index": 0, "new": 0.0008}),
        "change-as-horizontal": ("🔩applies-change-as-horizontal", "reinforced_wall_example", {"index": 0, "new": 0.0005}),
        "change-f-yd": ("🔩applies-change-f-yd", "reinforced_wall_example", {"index": 0, "new": 400000000.0}),
        "change-fire-rei": ("🔥️applies-change-fire-rei", clay, {"index": 0, "new": 120}),
        "change-exposure": ("💧️applies-change-exposure", clay, {"index": 0, "new": "Mx3"}),
        "change-mu": ("🧲️raises-the-bed-joint-friction-coefficient-to-0-625", clay, {"index": 0, "new": 0.625}),
        "change-density": ("🧱applies-change-density", clay, {"index": 0, "new": 1600.0}),
        "change-phi-infinity": ("♾️applies-change-phi-infinity", clay, {"index": 0, "new": 2.0}),
        "change-is-basement": ("🏗️applies-change-is-basement", clay, {"index": 0, "new": True}),
        "change-wall-label-en": ("🏷️applies-change-wall-label-en", clay, {"index": 0, "new": "North exterior load-bearing wall"}),
        "change-wall-label-de": ("🏷️applies-change-wall-label-de", clay, {"index": 0, "new": "Tragende Außenwand Nord"}),
        "insert-opening": ("➕️applies-insert-opening", clay, {"wallIndex": 0, "index": 0, "element": {"id": "win-2", "widthM": 1.0, "heightM": 1.25, "sillHeightM": 0.9}}),
        "remove-opening": ("➖️applies-remove-opening", "opening_wall_example", {"wallIndex": 0, "index": 1}),
        "change-opening-width": ("🪟applies-change-opening-width", "opening_wall_example", {"wallIndex": 0, "index": 0, "new": 1.5}),
        "change-opening-height": ("🪟applies-change-opening-height", "opening_wall_example", {"wallIndex": 0, "index": 0, "new": 1.5}),
        "change-opening-sill": ("🪟applies-change-opening-sill", "opening_wall_example", {"wallIndex": 0, "index": 0, "new": 0.8}),
        "insert-load-case": ("➕️applies-insert-load-case", clay, {"wallIndex": 0, "index": 1, "element": dict(copy.deepcopy(loads), id="uls-snow", qKSnowPa=750.0)}),
        "remove-load-case": ("➖️applies-remove-load-case", {"from": clay, "patch": "second-load-case"}, {"wallIndex": 0, "index": 1}),
        "change-load-case-situation": ("🎭️applies-change-load-case-situation", clay, {"wallIndex": 0, "loadCaseIndex": 0, "new": "accidental"}),
        "change-imposed-category": ("🏷️applies-change-imposed-category", clay, {"wallIndex": 0, "index": 0, "new": "B"}),
        "change-gk-slab": ("🏋️applies-change-gk-slab", clay, {"wallIndex": 0, "index": 0, "new": 135000.0}),
        "change-qk-imposed": ("🏋️applies-change-qk-imposed", clay, {"wallIndex": 0, "index": 0, "new": 2500.0}),
        "change-tributary-area": ("📐️applies-change-tributary-area", clay, {"wallIndex": 0, "index": 0, "new": 15.0}),
        "change-slab-span": ("↔️applies-change-slab-span", clay, {"wallIndex": 0, "index": 0, "new": 5.0}),
        "change-qk-snow": ("❄️applies-change-qk-snow", clay, {"wallIndex": 0, "index": 0, "new": 750.0}),
        "change-qp-wind": ("🌬️applies-change-qp-wind", clay, {"wallIndex": 0, "index": 0, "new": 950.0}),
        "change-c-pe": ("🧮applies-change-c-pe", clay, {"wallIndex": 0, "index": 0, "new": 1.0}),
        "change-hk-earth": ("🪨applies-change-hk-earth", "basement_wall_example", {"wallIndex": 0, "index": 0, "new": 55000.0}),
        "insert-concentrated": ("➕️applies-insert-concentrated", clay, {"wallIndex": 0, "loadCaseIndex": 0, "index": 0, "element": {"id": "beam-C", "forceN": 20000.0, "bearingAreaM2": 0.02, "bearingLengthM": 0.15}}),
        "remove-concentrated": ("➖️applies-remove-concentrated", "concentrated_load_example", {"wallIndex": 0, "loadCaseIndex": 0, "index": 0}),
        "change-concentrated-force": ("🏋️applies-change-concentrated-force", "concentrated_load_example", {"wallIndex": 0, "loadCaseIndex": 0, "index": 0, "new": 42000.0}),
        "change-concentrated-bearing-area": ("📐️applies-change-concentrated-bearing-area", "concentrated_load_example", {"wallIndex": 0, "loadCaseIndex": 0, "index": 0, "new": 0.05}),
        "change-concentrated-bearing-length": ("↔️applies-change-concentrated-bearing-length", "concentrated_load_example", {"wallIndex": 0, "loadCaseIndex": 0, "index": 0, "new": 0.25}),
    }


def spec_din16798(artifact):
    office = "compliant_office"
    snapshot = dumped(artifact, office)
    zone, vent = snapshot["zones"][0], snapshot["ventSystems"][0]
    z, v = {"zoneId": zone["id"]}, {"ventId": vent["id"]}
    same = lambda leaf: leaf
    return {
        "change-annex": ("🌍️change-annex", office, {"new": "En"}),
        "change-theta-rm": ("🔄️change-theta-rm", office, {"new": 18.5}),
        "change-outdoor-co2": ("🌫️change-outdoor-co2", office, {"new": 420.0}),
        "change-envelope-n50": ("🏠️change-envelope-n50", office, {"new": 2.5}),
        "change-envelope-volume": ("📦️change-envelope-volume", office, {"new": 640.0}),
        "change-cellar-area": ("🏚️change-cellar-area", office, {"new": 62.5}),
        "change-cellar-ventilation": ("🌀change-cellar-ventilation", office, {"new": 22.5}),
        "change-night-setback": ("🌙️change-night-setback", office, {"new": 5.0}),
        "insert-zone": ("➕️insert-zone", office, {"index": 1, "element": dict(copy.deepcopy(zone), id="zone-meeting", name="Meeting room", floorAreaM2=40.0, occupants=12)}),
        "remove-zone": ("➖️remove-zone", {"from": office, "patch": "second-zone"}, dict(zoneId="zone-meeting")),
        "change-zone-usage-type": ("🏢️change-zone-usage-type", office, dict(z, new="meeting")),
        "change-zone-floor-area": ("📐️change-zone-floor-area", office, dict(z, new=240.0)),
        "change-zone-occupants": ("👥️change-zone-occupants", office, dict(z, new=24)),
        "change-zone-comfort-category": ("🛋️change-zone-comfort-category", office, dict(z, new="I")),
        "change-zone-pollution-class": ("🏭️change-zone-pollution-class", office, dict(z, new="very_low")),
        "change-zone-comfort-model": ("🧭️change-zone-comfort-model", office, dict(z, new="adaptive")),
        "change-zone-t-op-winter": ("❄️change-zone-t-op-winter", office, dict(z, new=21.0)),
        "change-zone-t-op-summer": ("☀️change-zone-t-op-summer", office, dict(z, new=25.5)),
        "change-zone-air-speed": ("💨change-zone-air-speed", office, dict(z, new=0.15)),
        "change-zone-clothing": ("👔change-zone-clothing", office, dict(z, new=0.7)),
        "change-zone-metabolic-rate": ("🏃️change-zone-metabolic-rate", office, dict(z, new=1.4)),
        "change-zone-rh": ("💧️change-zone-rh", office, dict(z, new=50.0)),
        "change-zone-outdoor-air": ("💨️change-zone-outdoor-air", office, dict(z, new=1200.0)),
        "change-zone-co2": ("🫧change-zone-co2", office, dict(z, new=800.0)),
        "change-zone-illuminance": ("💡change-zone-illuminance", office, dict(z, new=750.0)),
        "change-zone-noise": ("🔊️change-zone-noise", office, dict(z, new=30.0)),
        "change-zone-turbulence": ("💨change-zone-turbulence", office, dict(z, new=35.0)),
        "change-zone-vent-method": ("📐️change-zone-vent-method", office, dict(z, new="method_2_limit_values")),
        "change-zone-vent-system-id": ("🔗change-zone-vent-system-id", {"from": office, "patch": "second-vent"}, dict(z, new="vent-meeting")),
        "insert-vent-system": ("🆕️insert-vent-system", office, {"index": 1, "element": dict(copy.deepcopy(vent), id="vent-meeting", name="Meeting room unit", designAirflowM3H=480.0)}),
        "remove-vent-system": ("🗑️remove-vent-system", {"from": office, "patch": "second-vent"}, dict(ventId="vent-meeting")),
        "change-vent-system-type": ("⚙️change-vent-system-type", office, dict(v, new="decentral_mech")),
        "change-vent-sfp": ("🌀️change-vent-sfp", office, dict(v, new=1250.0)),
        "change-vent-sfp-class": ("🎓️change-vent-sfp-class", office, dict(v, new=2)),
        "change-vent-heat-recovery": ("♻️change-vent-heat-recovery", office, dict(v, new=0.875)),
        "change-vent-oda-class": ("🏞️change-vent-oda-class", office, dict(v, new="ODA3")),
        "change-vent-filter-sup": ("🧽change-vent-filter-sup", office, dict(v, new="ePM1_80")),
        "change-vent-inspection": ("📅️change-vent-inspection", office, dict(v, new=6)),
        "change-vent-duct-class": ("🧱change-vent-duct-class", office, dict(v, new="D")),
        "change-vent-duct-leakage": ("🕳️change-vent-duct-leakage", office, dict(v, new=0.0625)),
        "change-vent-design-airflow": ("🌬️change-vent-design-airflow", office, dict(v, new=1200.0)),
    }


def spec_din18599(artifact):
    house = "compliant_detached_house"
    snapshot = dumped(artifact, house)
    return {
        "change-building-category": ("🎯️applies-building-category", house, {"new": "NonResidential"}),
        "change-attachment": ("🎯️applies-attachment", house, {"new": "SemiDetached"}),
        "change-use-class": ("🏢️reclassifies-the-building-as-an-office", house, {"new": "Office"}),
        "change-method": ("🎯️applies-method", house, {"new": "Tabular"}),
        "change-net-floor-area-m2": ("📏️extends-net-floor-area-to-160-m2", house, {"new": 160.0}),
        "change-heated-volume-m3": ("🎯️applies-heated-volume-m3", house, {"new": 420.0}),
        "change-geg-qp-factor": ("🎯️applies-geg-qp-factor", house, {"new": 0.45}),
        "change-delta-u-wb": ("🎯️applies-delta-u-wb", house, {"new": 0.05}),
        "change-automation-class": ("🎯️applies-automation-class", house, {"new": "A"}),
        "specify-heating-system": ("🎯️applies-specify-heating-system", house, {"new": "heating"}),
        "specify-dhw-system": ("🎯️applies-specify-dhw-system", house, {"new": "dhw"}),
        "update-ventilation": ("🎯️applies-update-ventilation", house, {"new": "ventilation"}),
        "update-cooling": ("🎯️applies-update-cooling", house, {"new": {"plant": {"eer": 3.5, "energyCarrier": "electricity"}}}),
        "update-lighting": ("🎯️applies-update-lighting", house, {"new": "lighting"}),
        "update-renewables": ("🎯️applies-update-renewables", house, {"new": "renewables"}),
        "replace-zones": ("🎯️applies-replace-zones", house, {"new": "zones"}),
        "replace-elements": ("🎯️applies-replace-elements", house, {"new": "elements"}),
        "change-element-u": ("🎯️applies-change-element-u", house, {"elementId": snapshot["elements"][0]["id"], "new": 0.18}),
    }


def spec_din4108(artifact):
    dwelling = "compliant_etics_dwelling"
    return {
        "change-climate-zone": ("🗺️moves-to-zone-3", dwelling, {"new": "Zone3"}),
        "change-usage": ("🏢️sets-nonresidential", dwelling, {"new": "nonResidential"}),
        "change-t-int-c": ("🌡️sets-t-int-to-21-point-5", dwelling, {"new": 21.5}),
        "change-rh-int": ("💧️raises-rh-to-0-point-55", dwelling, {"new": 0.55}),
        "change-airtightness-n50": ("💨️tightens-n50-to-1-point-0", dwelling, {"new": 1.0}),
        "change-has-mechanical-ventilation": ("🌬️disables-mechanical-ventilation", dwelling, {"new": False}),
        "change-bb2-details-conform": ("❌️declares-bb2-non-conforming", dwelling, {"new": False}),
        "insert-zone": ("➕️appends-extra-zone", dwelling, {"index": 2, "element": {"id": "zone-extra", "floorAreaM2": 40.0, "heaviness": "medium", "nightVentilation": "none", "windows": []}}),
        "remove-zone": ("🚫️removes-first-zone", dwelling, {"index": 0}),
        "change-zone-floor-area": ("📐️sets-floor-area-to-90", dwelling, {"zoneId": "zone-living", "new": 90.0}),
        "change-zone-heaviness": ("🧱sets-heaviness-light", dwelling, {"zoneId": "zone-living", "new": "light"}),
        "change-zone-night-ventilation": ("🌙sets-night-ventilation-high", dwelling, {"zoneId": "zone-living", "new": "high"}),
        "insert-zone-window": ("🪟appends-extra-window", dwelling, {"zoneId": "zone-living", "index": 2, "element": {"id": "win-extra", "orientation": "W", "inclinationDeg": 90.0, "areaM2": 2.0, "gValue": 0.5, "shadingFc": 0.5}}),
        "remove-zone-window": ("🚫️removes-east-window", dwelling, {"zoneId": "zone-living", "index": 1}),
        "change-zone-window-area": ("📏grows-south-window", dwelling, {"zoneId": "zone-living", "windowId": "win-south", "new": 10.0}),
        "change-zone-window-g-value": ("☀️sets-g-value-0-point-6", dwelling, {"zoneId": "zone-living", "windowId": "win-south", "new": 0.6}),
        "change-zone-window-shading-fc": ("⛱️tightens-shading-fc", dwelling, {"zoneId": "zone-living", "windowId": "win-south", "new": 0.25}),
        "change-zone-window-orientation": ("🧭turns-south-window-west", dwelling, {"zoneId": "zone-living", "windowId": "win-south", "new": "W"}),
        "change-zone-window-inclination-deg": ("📐tilts-south-window-to-60-degrees", dwelling, {"zoneId": "zone-living", "windowId": "win-south", "new": 60.0}),
        "insert-element": ("🏠️appends-extra-wall", dwelling, {"index": 3, "element": "extra-wall"}),
        "remove-element": ("🚫️removes-first-element", dwelling, {"index": 0}),
        "change-element-area": ("📐️grows-wall-area", dwelling, {"elementId": "wall-north", "new": 45.0}),
        "change-element-adjacent": ("↔️sets-adjacent-unheated", dwelling, {"elementId": "wall-north", "new": "unheated"}),
        "change-element-kind": ("🏷️retags-as-opaque-frame", dwelling, {"elementId": "wall-north", "new": "frameOpaque"}),
        "change-element-orientation-deg": ("🧭turns-north-wall-south", dwelling, {"elementId": "wall-north", "new": 180.0}),
        "change-element-inclination-deg": ("📐tilts-north-wall-to-45-degrees", dwelling, {"elementId": "wall-north", "new": 45.0}),
        "change-element-delta-ug": ("📈️raises-glazing-delta-ug", dwelling, {"elementId": "wall-north", "new": 0.02}),
        "change-element-delta-uf": ("📈️raises-frame-delta-uf", dwelling, {"elementId": "wall-north", "new": 0.01}),
        "change-element-delta-ur": ("📈️raises-roof-delta-ur", dwelling, {"elementId": "wall-north", "new": 0.005}),
        "insert-layer": ("➕️inserts-layer-into-wall", dwelling, {"elementId": "wall-north", "index": 1, "element": "extra-layer"}),
        "remove-layer": ("➖️removes-eps-layer", dwelling, {"elementId": "wall-north", "index": 2}),
        "reorder-layers": ("🧭️swaps-first-two-layers", dwelling, {"elementId": "wall-north", "from": 0, "to": 1}),
        "change-layer-thickness": ("📏️thickens-eps-to-0-point-2", dwelling, {"elementId": "wall-north", "index": 2, "new": 0.2}),
        "change-layer-lambda": ("🌡️sets-eps-lambda", dwelling, {"elementId": "wall-north", "index": 2, "new": 0.032}),
        "change-layer-mu": ("💧raises-eps-mu", dwelling, {"elementId": "wall-north", "index": 2, "new": 60.0}),
        "change-layer-material-id": ("🧽️retags-eps-material", dwelling, {"elementId": "wall-north", "index": 2, "new": "eps_grey"}),
        "change-layer-application-type": ("🏷️reclassifies-eps-as-wab", dwelling, {"elementId": "wall-north", "index": 2, "new": "WAB"}),
        "change-layer-compressive-class": ("🏷️raises-eps-compressive-class", dwelling, {"elementId": "wall-north", "index": 2, "new": "dk"}),
        "insert-thermal-bridge": ("🌉️appends-extra-bridge", dwelling, {"index": 2, "element": {"id": "tb-extra", "psi": 0.08, "lengthM": 5.0, "bb2Type": "categoryB"}}),
        "remove-thermal-bridge": ("🧊removes-first-bridge", dwelling, {"index": 0}),
        "change-thermal-bridge-psi": ("🔘lowers-psi", dwelling, {"bridgeId": "tb-window-reveal", "new": 0.03}),
        "change-thermal-bridge-length": ("↔️shortens-bridge", dwelling, {"bridgeId": "tb-window-reveal", "new": 20.0}),
        "change-thermal-bridge-bb2-type": ("🏷️reclassifies-reveal-bridge", dwelling, {"bridgeId": "tb-window-reveal", "new": "categoryB"}),
    }


SPECS = {"en1996": spec_en1996, "din16798": spec_din16798, "din18599": spec_din18599, "din4108": spec_din4108}
# endregion Specs


# region Stage
def resolve_base(artifact, base):
    if isinstance(base, str):
        return dumped(artifact, base)
    snapshot = copy.deepcopy(dumped(artifact, base["from"]))
    PATCHES[base["patch"]](snapshot)
    return snapshot


def second_load_case(snapshot):
    case = copy.deepcopy(snapshot["walls"][0]["loadCases"][0])
    case.update(id="uls-snow", qKSnowPa=750.0)
    snapshot["walls"][0]["loadCases"].append(case)


def second_zone(snapshot):
    zone = copy.deepcopy(snapshot["zones"][0])
    zone.update(id="zone-meeting", name="Meeting room", floorAreaM2=40.0, occupants=12)
    snapshot["zones"].append(zone)


def second_vent(snapshot):
    vent = copy.deepcopy(snapshot["ventSystems"][0])
    vent.update(id="vent-meeting", name="Meeting room unit", designAirflowM3H=480.0)
    snapshot["ventSystems"].append(vent)


PATCHES = {"second-load-case": second_load_case, "second-zone": second_zone, "second-vent": second_vent}


def facet_value(artifact, kind, base, values):
    """din18599 facet setters and din4108 composite inserts take a whole record: derived from the base."""
    new = values.get("new")
    if artifact == "din18599" and kind in FACETS:
        return dict(values, new=FACETS[kind](copy.deepcopy(base[new])))
    element = values.get("element")
    if artifact == "din4108" and element == "extra-wall":
        wall = copy.deepcopy(base["elements"][0])
        wall.update(id="wall-extra", orientationDeg=90.0, areaM2=10.0)
        return dict(values, element=wall)
    if artifact == "din4108" and element == "extra-layer":
        layer = copy.deepcopy(base["elements"][0]["layers"][1])
        layer.update(id="layer-extra", materialId="eps", thicknessM=0.05, lambda_=None)
        layer.pop("lambda_")
        layer["lambda"], layer["mu"], layer["density"] = 0.035, 40.0, 20.0
        return dict(values, element=layer)
    return values


def tweak_numbers(record, factor):
    """The first numeric field of a record, moved by `factor` and rounded to a readable value."""
    for key, value in record.items():
        if isinstance(value, float) and value > 0:
            record[key] = round(value * factor, 4)
            return record
    raise SystemExit(f"no positive float in {record}")


FACETS = {
    "specify-heating-system": lambda record: tweak_numbers(record, 1.1),
    "specify-dhw-system": lambda record: tweak_numbers(record, 1.1),
    "update-ventilation": lambda record: tweak_numbers(record, 1.1),
    "update-lighting": lambda record: tweak_numbers(record, 1.1),
    "update-renewables": lambda record: tweak_numbers(record, 1.1),
    "replace-zones": lambda zones: [tweak_numbers(zone, 1.1) if index == 0 else zone for index, zone in enumerate(zones)],
    "replace-elements": lambda elements: [tweak_numbers(element, 1.1) if index == 0 else element for index, element in enumerate(elements)],
}


def stage(artifact):
    known = leaves(artifact)
    specs = spec(artifact)
    missing = sorted(set(known) - set(specs) - {"update-climate"})
    if missing:
        raise SystemExit(f"{artifact}: no spec for {missing}")
    root = f"{GEN}/stage/{artifact}"
    shutil.rmtree(root, ignore_errors=True)
    for kind, (scenario, base_spec, values) in specs.items():
        leaf = known[kind]
        base = resolve_base(artifact, base_spec)
        wire = leaf["wire"](payload(leaf, facet_value(artifact, kind, base, values)))
        directory = f"{root}/{leaf['dir']}/{scenario}"
        os.makedirs(f"{directory}/🦠️mutation", exist_ok=True)
        os.makedirs(f"{directory}/📸️snapshot/⬅️before", exist_ok=True)
        with open(f"{directory}/🦠️mutation/🔣️.json", "w") as handle:
            json.dump(wire, handle, ensure_ascii=False)
        with open(f"{directory}/📸️snapshot/⬅️before/🔣️.json", "w") as handle:
            json.dump(base, handle, ensure_ascii=False)
    print(f"staged {len(specs)} {artifact} vectors under {root}")
# endregion Stage


# region Check
def engine():
    host = importlib.util.spec_from_file_location("semio_repo_test", f"{ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🐍️.py")
    module = importlib.util.module_from_spec(host)
    sys.modules["semio_repo_test"] = module
    host.loader.exec_module(module)
    spec_ = importlib.util.spec_from_file_location("norm_engine", f"{ROOT}/✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py")
    vocabulary = importlib.util.module_from_spec(spec_)
    spec_.loader.exec_module(vocabulary)
    return vocabulary


def bundles(root):
    for leaf in sorted(os.listdir(root)):
        if not os.path.isdir(f"{root}/{leaf}"):
            continue
        for scenario in sorted(os.listdir(f"{root}/{leaf}")):
            directory = f"{root}/{leaf}/{scenario}"
            if os.path.exists(f"{directory}/🦠️mutation/🔣️.json"):
                yield leaf, scenario, directory


def check(artifact, root=None):
    vocabulary = engine()
    known = leaves(artifact)
    kinds = list(known)
    kind_of = {leaf["dir"]: kind for kind, leaf in known.items()}
    root = root or f"{GEN}/stage/{artifact}"
    failures = 0
    for leaf, scenario, directory in bundles(root):
        kind = kind_of[leaf]
        before = load(f"{directory}/📸️snapshot/⬅️before/🔣️.json")
        after = load(f"{directory}/📸️snapshot/➡️after/🔣️.json")
        outcome = load(f"{directory}/🎯️outcome/🔣️.json")
        _tag, arguments = vocabulary.unwrap(load(f"{directory}/🦠️mutation/🔣️.json"))
        verdict = []
        try:
            mutated = vocabulary.apply_mutation(before, kind, arguments)
            refused = None
        except vocabulary.Refused as reason:
            mutated, refused = copy.deepcopy(before), str(reason)
        if outcome["status"] == "applied" and refused:
            verdict.append(f"python refused: {refused}")
        if outcome["status"] == "rejected" and not refused:
            verdict.append("python applied a rejected vector")
        if mutated != after:
            verdict.append("python after differs: " + diff_paths(mutated, after))
        if outcome["status"] == "applied" and after == before:
            verdict.append("rust vector does not move the document")
        if outcome["status"] == "applied" and not refused:
            restored = mutated
            try:
                steps = vocabulary.inverse_mutation(kinds, before, kind, arguments)
                if not steps:
                    verdict.append("python inverse is empty")
                view = vocabulary.derived_view(before)
                for step, step_arguments in steps:
                    restored = vocabulary.apply_mutation(restored, step, step_arguments, view)
                if restored != before:
                    verdict.append("python inverse does not restore: " + diff_paths(restored, before))
            except (vocabulary.Refused, AssertionError) as reason:
                verdict.append(f"python inverse refused: {reason}")
        if outcome.get("messages"):
            verdict.append(f"rust messages {outcome['messages']}")
        failures += bool(verdict)
        print(("✗ " if verdict else "✓ ") + f"{kind:40} {scenario}" + ("" if not verdict else "\n      " + "\n      ".join(verdict)))
    print(f"{artifact}: {failures} disagreement(s)")
    return failures


def diff_paths(got, expected, path=""):
    if type(got) is not type(expected):
        return f"{path or '/'}: {json.dumps(got)[:80]} != {json.dumps(expected)[:80]}"
    if isinstance(got, dict):
        for key in sorted(set(got) | set(expected)):
            if got.get(key, "<absent>") != expected.get(key, "<absent>"):
                return diff_paths(got.get(key, "<absent>"), expected.get(key, "<absent>"), f"{path}/{key}")
    if isinstance(got, list):
        if len(got) != len(expected):
            return f"{path}: length {len(got)} != {len(expected)}"
        for index, (left, right) in enumerate(zip(got, expected)):
            if left != right:
                return diff_paths(left, right, f"{path}/{index}")
    return f"{path or '/'}: {json.dumps(got)[:80]} != {json.dumps(expected)[:80]}"
# endregion Check


# region Install
class Raw(str):
    """A JSON number lexeme kept exactly as the Rust writer spelled it."""


def pretty(text):
    value = json.loads(text, parse_float=Raw, parse_int=Raw)

    def emit(node, indent):
        pad, inner = "  " * indent, "  " * (indent + 1)
        if isinstance(node, Raw):
            return str(node)
        if isinstance(node, dict):
            if not node:
                return "{}"
            return "{\n" + ",\n".join(f"{inner}{json.dumps(key, ensure_ascii=False)}: {emit(item, indent + 1)}" for key, item in node.items()) + f"\n{pad}}}"
        if isinstance(node, list):
            if not node:
                return "[]"
            return "[\n" + ",\n".join(f"{inner}{emit(item, indent + 1)}" for item in node) + f"\n{pad}]"
        return json.dumps(node, ensure_ascii=False)

    return emit(value, 0) + "\n"


FILES = ["🦠️mutation/🔣️.json", "📸️snapshot/⬅️before/🔣️.json", "📸️snapshot/➡️after/🔣️.json", "🎯️outcome/🔣️.json"]


def install(artifact):
    if check(artifact):
        raise SystemExit("refusing to install: the two implementations disagree")
    target = f"{subset(artifact)}/🧫️fixtures/🧬️mutations"
    for leaf, scenario, directory in bundles(f"{GEN}/stage/{artifact}"):
        destination = f"{target}/{leaf}/{scenario}"
        shutil.rmtree(destination, ignore_errors=True)
        for name in FILES + ["🔺️diff/🔣️.json"]:
            if os.path.exists(f"{directory}/{name}"):
                os.makedirs(os.path.dirname(f"{destination}/{name}"), exist_ok=True)
                with open(f"{directory}/{name}") as handle, open(f"{destination}/{name}", "w") as out:
                    out.write(pretty(handle.read()))
        if os.path.exists(f"{directory}/🔺️diff/🚫️.absent"):
            os.makedirs(f"{destination}/🔺️diff", exist_ok=True)
            open(f"{destination}/🔺️diff/🚫️.absent", "w").close()
    print(f"installed {artifact} vectors into {target}")
# endregion Install


if __name__ == "__main__":
    command, artifact = sys.argv[1], sys.argv[2]
    {"stage": stage, "check": check, "install": install}[command](artifact)
