#!/usr/bin/env python3
"""🧪️ W2-W norm-1: one committed specification vector per EN 1991 / EN 1990 mutation leaf (design §11 fixture-quintet witness).

The payload of every vector is authored here, by hand, against the leaf schema; the before/after/diff/outcome JSON is written
once by the crate's temporary `[DEBUG]` generator test from production dispatch and then held by the permanent Rust law, the
independent Python reference (the `mutate-<std>-1` feature) and the `schema mutation-payloads` lint.

    python3 🧪️w2-w-norm-1-vectors.py plan <en1991|en1990>      # 🗑️generated/w2w-norm-1/plan-<std>.json for the generator
    python3 🧪️w2-w-norm-1-vectors.py sources <en1991|en1990>   # leaf tests, fixture module, catalog, feature Examples
    python3 🧪️w2-w-norm-1-vectors.py stubs <en1991|en1990>     # the superseded leaf test directories (printed, not deleted)
"""
import json
import re
import sys
import unicodedata
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated" / "w2w-norm-1"

FLOOR = {"id": "archive-l2", "category": "E1", "area": 120.0, "assumedQk": 6000.0, "assumedQkConcentrated": 7000.0, "assumedPartitions": 0.0}
SCREED = {"id": "screed", "material": "cement_screed", "thickness": 0.05, "assumedGk": 1100.0}
ANNEX_ROOF = {"id": "roof-annex", "roofType": "monopitch", "pitchDeg": 10.0, "cE": 1.0, "cT": 1.0, "hasParapet": True, "parapetHeight": 0.8, "driftObstructionHeight": 0.0, "multiSpan": False, "assumedSk": 1200.0}
LEEWARD = {"id": "facade-e", "zone": "E", "z": 8.0, "cPe10": -0.5, "cPe1": -0.7, "cPi": -0.3, "cS": 1.0, "cD": 1.0, "loadedArea": 10.0, "assumedWp": 400.0}
EXPLOSION = {"id": "explosion-1", "impact": [], "explosion": [{"explosionMass": 5.0, "standoff": 10.0, "assumedPressure": 20000.0}]}

EN1991 = [
    ("change-annex", "🌍", "en", {"newAnnex": "En"}, "switches the national annex from the German NA to the recommended EN values"),
    ("change-snow-zone", "⛄", "zone-3", {"newSnowZone": "3"}, "moves the site from snow zone 2 to snow zone 3"),
    ("change-altitude", "🗻", "480-m", {"newAltitude": 480.0}, "raises the site altitude above sea level from 150 m to 480 m"),
    ("change-en-sk", "⛄", "1250-pa", {"newEnSk": 1250.0}, "raises the EN characteristic ground snow load s_k from 850 Pa to 1250 Pa"),
    ("change-exceptional-snow-north-german-lowlands", "⛄", "on", {"newExceptionalSnowNorthGermanLowlands": True}, "flags the site as exposed to exceptional snow in the North German lowlands"),
    ("change-wind-zone", "🪁", "zone-3", {"newWindZone": 3}, "moves the site from wind zone 2 to wind zone 3"),
    ("change-en-vb", "💨", "27-5-m-s", {"newEnVb": 27.5}, "raises the EN basic wind velocity v_b from 25 m/s to 27.5 m/s"),
    ("change-terrain-category", "🌳", "class-3", {"newTerrainCategory": 3}, "roughens the terrain from category II to category III"),
    ("change-mixed-terrain-upwind", "🧭", "class-1", {"newMixedTerrainUpwind": 1}, "smooths the upwind terrain of a mixed profile from category II to category I"),
    ("change-mixed-terrain-distance", "📏", "1500-m", {"newMixedTerrainDistance": 1500.0}, "moves the upwind terrain change from 0 m to 1500 m"),
    ("change-orography-factor", "📐", "1-15", {"newOrographyFactor": 1.15}, "raises the orography factor c_o from 1.0 to 1.15"),
    ("change-coast-or-island", "🌊", "coast", {"newCoastOrIsland": True}, "places the site on the coast"),
    ("change-air-density", "💨", "1-225", {"newAirDensity": 1.225}, "adopts the standard air density of 1.225 kg/m³"),
    ("change-height", "🧱", "24-m", {"newHeight": 24.0}, "raises the building height h from 20 m to 24 m"),
    ("change-width", "🏠", "18-5-m", {"newWidth": 18.5}, "widens the building b from 15 m to 18.5 m"),
    ("change-depth", "🏢", "30-m", {"newDepth": 30.0}, "deepens the building d from 25 m to 30 m"),
    ("change-assumed-delta-t", "📈", "15-k", {"newAssumedDeltaT": 15.0}, "raises the assumed uniform temperature difference from 10 K to 15 K"),
    ("change-construction-activity", "🧰", "formwork", {"newConstructionActivity": "formwork"}, "switches the execution-stage activity from scaffolding to formwork"),
    ("change-assumed-construction-qk", "👷", "2-kpa", {"newAssumedConstructionQk": 2000.0}, "raises the assumed construction load q_ca from 1500 Pa to 2000 Pa"),
    ("change-structure-kind", "🌉", "bridge", {"newStructureKind": "bridge"}, "reclassifies the structure from a building to a bridge"),
    ("change-bridge-lane", "🚦", "3-lanes", {"newBridgeLane": 3}, "widens the carriageway from 1 to 3 notional lanes"),
    ("change-bridge-span", "🌉", "36-m", {"newBridgeSpan": 36.0}, "lengthens the bridge span from 20 m to 36 m"),
    ("change-bridge-lane-width", "📏", "3-5-m", {"newBridgeLaneWidth": 3.5}, "widens the notional lane from 3 m to 3.5 m"),
    ("change-assumed-bridge-tandem", "🚚", "600-kn", {"newAssumedBridgeTandem": 600000.0}, "assumes a 600 kN tandem system TS"),
    ("change-assumed-bridge-udl", "🚦", "9-kpa", {"newAssumedBridgeUdl": 9000.0}, "assumes a 9 kPa uniformly distributed load UDL"),
    ("change-assumed-bridge-lm2", "🚛", "400-kn", {"newAssumedBridgeLm2": 400000.0}, "assumes a 400 kN single axle load LM2"),
    ("change-assumed-bridge-footway", "🚶", "5-kpa", {"newAssumedBridgeFootway": 5000.0}, "assumes a 5 kPa footway load"),
    ("change-storey-count", "🏢", "5-storeys", {"newStoreyCount": 5}, "raises the storey count from 3 to 5"),
    ("change-t-max", "🌞", "39-c", {"newTMax": 39.0}, "raises the maximum shade air temperature T_max from 37 °C to 39 °C"),
    ("change-t-min", "🧊", "minus-28-c", {"newTMin": -28.0}, "lowers the minimum shade air temperature T_min from -24 °C to -28 °C"),
    ("change-initial-temperature", "⏰", "15-c", {"newT0": 15.0}, "raises the initial temperature T_0 from 10 °C to 15 °C"),
    ("change-thermal-element-type", "🌉", "bridge2", {"newThermalElementType": "bridge2"}, "switches the thermal element from a building element to bridge deck type 2"),
    ("change-thermal-bridge-type", "🌁", "type-2", {"newThermalBridgeType": 2}, "switches the thermal bridge deck type from 1 to 2"),
    ("change-linear-temperature-gradient", "📈", "5-k", {"newDeltaTM": 5.0}, "applies a 5 K linear temperature difference ΔT_M"),
    ("change-fire-mode", "🔥", "parametric", {"newFireMode": "parametric"}, "switches the fire design from none to a parametric fire"),
    ("change-fire-curve", "📉", "hydrocarbon", {"newFireCurve": "hydrocarbon"}, "switches the nominal fire curve from standard to hydrocarbon"),
    ("change-fire-duration", "⌛", "90-min", {"newFireDuration": 5400.0}, "extends the fire duration from 60 min to 90 min"),
    ("change-assumed-gas-temperature", "🔥", "1300-k", {"newAssumedGasTemperature": 1300.0}, "raises the assumed gas temperature from 1200 K to 1300 K"),
    ("change-assumed-h-net", "🔆", "35-kw-m2", {"newAssumedHNet": 35000.0}, "raises the assumed net heat flux from 25 kW/m² to 35 kW/m²"),
    ("change-fire-compartment-area", "📐", "150-m2", {"newFireCompartmentArea": 150.0}, "enlarges the fire compartment floor area from 100 m² to 150 m²"),
    ("change-fire-compartment-height", "📐", "3-5-m", {"newFireCompartmentHeight": 3.5}, "raises the fire compartment height from 3 m to 3.5 m"),
    ("change-fire-opening-factor", "🪟", "0-06", {"newFireOpeningFactor": 0.06}, "raises the opening factor O from 0.04 m^½ to 0.06 m^½"),
    ("change-fire-thermal-inertia", "🧱", "1500", {"newFireThermalInertia": 1500.0}, "raises the enclosure thermal inertia b from 1160 to 1500 J/(m²s^½K)"),
    ("change-fire-occupancy", "🏬", "shopping", {"newFireOccupancy": "shopping"}, "reclassifies the fire occupancy from office to shopping"),
    ("change-fire-load-density-qf", "⛽", "600-mj", {"newFireLoadDensityQf": 600000000.0}, "raises the characteristic fire load density q_f,k from 420 MJ/m² to 600 MJ/m²"),
    ("change-assumed-qf-d", "🔋", "511-mj", {"newAssumedQfD": 511000000.0}, "raises the assumed design fire load density q_f,d from 420 MJ/m² to 511 MJ/m²"),
    ("change-assumed-bridge-lm3", "🚛", "600-kn", {"newAssumedBridgeLm3": 600000.0}, "assumes a 600 kN special vehicle LM3"),
    ("change-assumed-bridge-lm4", "👥", "5-kpa", {"newAssumedBridgeLm4": 5000.0}, "assumes a 5 kPa crowd load LM4"),
    ("change-bridge-load-group", "📦", "gr1b", {"newBridgeLoadGroup": "gr1b"}, "switches the traffic load group from gr1a to gr1b"),
    ("change-crane-claimed", "🚫", "withdrawn", {"newCraneClaimed": False}, "withdraws the crane claim"),
    ("change-crane-class", "💥", "hc3", {"newCraneClass": "HC3"}, "upgrades the crane from class HC2 to HC3"),
    ("change-hoist-class", "🪝", "hc4", {"newHoistClass": "HC4"}, "upgrades the hoist from class HC2 to HC4"),
    ("change-hoisting-speed", "⏫", "1-25-m-s", {"newHoistingSpeed": 1.25}, "speeds hoisting from 0.5 m/s to 1.25 m/s"),
    ("change-assumed-crane-wheel", "🛞", "75-kn", {"newAssumedCraneWheel": 75000.0}, "assumes a 75 kN crane wheel load"),
    ("change-assumed-crane-horizontal", "🧲", "7-5-kn", {"newAssumedCraneHorizontal": 7500.0}, "assumes a 7.5 kN horizontal crane load"),
    ("change-silo-claimed", "🚫", "withdrawn", {"newSiloClaimed": False}, "withdraws the silo claim"),
    ("change-silo-kind", "💧", "tank", {"newSiloKind": "tank"}, "reclassifies the container from a silo to a tank"),
    ("change-silo-bulk-density", "🌾", "9-kn-m3", {"newSiloBulkDensity": 9000.0}, "raises the bulk unit weight γ from 8 kN/m³ to 9 kN/m³"),
    ("change-silo-height", "📏", "18-m", {"newSiloHeight": 18.0}, "raises the silo from 12 m to 18 m"),
    ("change-silo-hydraulic-radius", "⭕", "2-25-m", {"newSiloHydraulicRadius": 2.25}, "widens the hydraulic radius from 1.5 m to 2.25 m"),
    ("change-silo-mu", "🔎", "0-5", {"newSiloMu": 0.5}, "raises the wall friction coefficient μ from 0.4 to 0.5"),
    ("change-silo-k", "🔩", "0-55", {"newSiloK": 0.55}, "raises the lateral pressure ratio K from 0.4 to 0.55"),
    ("change-assumed-silo-pressure", "🌀", "8-kpa", {"newAssumedSiloPressure": 8000.0}, "raises the assumed horizontal wall pressure from 5 kPa to 8 kPa"),
    ("change-assumed-silo-patch", "📦", "1-5-kpa", {"newAssumedSiloPatch": 1500.0}, "raises the assumed patch load from 1 kPa to 1.5 kPa"),
    ("change-assumed-silo-wall-friction", "🧱", "2-kpa", {"newAssumedSiloWallFriction": 2000.0}, "raises the assumed wall friction traction from 1 kPa to 2 kPa"),
    ("change-floor-assumed-qk", "🏢", "3-kpa", {"index": 0, "newAssumedQk": 3000.0}, "raises the assumed imposed load q_k of floor 0 from 1 kPa to 3 kPa"),
    ("change-self-weight-assumed-gk", "🚧", "5-kpa", {"index": 0, "newAssumedGk": 5000.0}, "raises the assumed self-weight g_k of element 0 from 2 kPa to 5 kPa"),
    ("change-roof-assumed-sk", "⛄", "900-pa", {"index": 0, "newAssumedSk": 900.0}, "raises the assumed roof snow load of roof 0 from 200 Pa to 900 Pa"),
    ("change-wind-face-assumed-wp", "🪟", "750-pa", {"index": 0, "newAssumedWp": 750.0}, "raises the assumed wind pressure of face 0 from 100 Pa to 750 Pa"),
    ("change-accidental-assumed-force", "🚗", "150-kn", {"index": 0, "newAssumedForce": 150000.0}, "raises the assumed impact force of accidental case 0 from 50 kN to 150 kN"),
    ("insert-floors", "➕", "archive", {"index": 1, "item": FLOOR}, "adds an archive floor (category E1) behind the office floor"),
    ("remove-floors", "➖", "office", {"index": 0}, "removes the office floor"),
    ("insert-self-weight-elements", "➕", "screed", {"index": 1, "item": SCREED}, "adds a 50 mm cement screed layer behind the slab"),
    ("remove-self-weight-elements", "➖", "slab", {"index": 0}, "removes the reinforced-concrete slab layer"),
    ("insert-roofs", "➕", "annex-roof", {"index": 1, "item": ANNEX_ROOF}, "adds a monopitch annex roof with a parapet behind the main roof"),
    ("remove-roofs", "➖", "main-roof", {"index": 0}, "removes the main duopitch roof"),
    ("insert-wind-faces", "➕", "leeward", {"index": 1, "item": LEEWARD}, "adds the leeward façade zone E behind the windward zone D"),
    ("remove-wind-faces", "➖", "windward", {"index": 0}, "removes the windward façade zone D"),
    ("insert-accidental-cases", "➕", "explosion", {"index": 1, "item": EXPLOSION}, "adds an internal explosion case behind the vehicle impact case"),
    ("remove-accidental-cases", "➖", "impact", {"index": 0}, "removes the vehicle impact case"),
]

BEAM = {"id": "beam-B1", "labelEn": "Office beam B1", "labelDe": "Büroträger B1", "rdStr": 250000.0, "rdGeo": 250000.0, "rdEquStab": 200000.0, "rdEquDestab": 180000.0, "rdFat": 0.0, "span": 6.0, "deflectionW": 0.018, "deflectionLimitRatio": 250.0, "vibrationFrequency": 5.5, "vibrationFrequencyMin": 3.0}
EFFECTS = [{"memberId": "beam-B1", "actionId": action, "influence": 1.0} for action in ("G-sup", "G-inf", "Q-office", "Q-wind", "A-impact", "E-1")]
OFFICE, WIND, SNOW = {"id": "Q-office", "category": "office", "qk": 40000.0}, {"id": "Q-wind", "category": "wind", "qk": 20000.0}, {"id": "Q-snow", "category": "snow", "qk": 15000.0}

EN1990 = [
    ("change-annex", "🌍", "en", {"newAnnex": "En"}, "switches the national annex from the German NA to the recommended EN values"),
    ("change-project-id", "📛", "office-tower-b", {"newProjectId": "office-tower-b"}, "renames the project to office-tower-b"),
    ("change-altitude-m", "🗻", "950-m", {"newAltitudeM": 950.0}, "raises the site altitude from 120 m to 950 m"),
    ("change-consequence-class", "🚨", "cc3", {"newConsequenceClass": 3}, "escalates the building from consequence class CC2 to CC3"),
    ("change-reliability-class", "🎯", "rc3", {"newReliabilityClass": 3}, "raises the reliability class from RC2 to RC3"),
    ("change-design-working-life-category", "⏳", "cat-5", {"newDesignWorkingLifeCategory": 5}, "reclassifies the design working life from category 4 to category 5"),
    ("change-design-working-life-years", "📆", "100-y", {"newDesignWorkingLifeYears": 100.0}, "doubles the design working life from 50 to 100 years"),
    ("change-reference-period-years", "⌛", "1-y", {"newReferencePeriodYears": 1.0}, "shortens the reliability reference period from 50 years to 1 year"),
    ("change-supervision-level", "👀", "dsl3", {"newSupervisionLevel": "DSL3"}, "tightens design supervision from DSL2 to DSL3"),
    ("change-inspection-level", "🔍", "il3", {"newInspectionLevel": "IL3"}, "tightens execution inspection from IL2 to IL3"),
    ("change-beta-computed", "📐", "4-3", {"newBetaComputed": 4.3}, "raises the computed reliability index β from 3.8 to 4.3"),
    ("change-permanents", "⚓", "95-kn", {"newPermanents": [{"id": "G-sup", "kind": "g_sup", "gk": 95000.0}, {"id": "G-inf", "kind": "g_inf", "gk": 20000.0}]}, "raises the unfavourable permanent action G-sup from 80 kN to 95 kN"),
    ("change-variables", "⛄", "adds-snow", {"newVariables": [OFFICE, WIND, SNOW]}, "replaces the variable actions with a set that adds a 15 kN snow action"),
    ("change-accidentals", "💥", "75-kn", {"newAccidentals": [{"id": "A-impact", "ad": 75000.0}]}, "raises the design impact action A_d from 50 kN to 75 kN"),
    ("change-seismics", "🌋", "class-iii", {"newSeismics": [{"id": "E-1", "aEk": 60000.0, "importanceClass": "III"}]}, "raises the seismic importance class of E-1 from II to III"),
    ("change-members", "💪", "300-kn", {"newMembers": [{**BEAM, "rdStr": 300000.0}]}, "strengthens beam B1's STR resistance from 250 kN to 300 kN"),
    ("change-bridge-sls", "🌉", "deck-check", {"newBridgeSls": [{"id": "sls-deck", "memberId": "beam-B1", "deckAcceleration": 0.5, "deckAccelerationLimit": 0.7, "deckTwist": 0.001, "deckTwistLimit": 0.0015, "bridgeDeflection": 0.012, "bridgeDeflectionLimit": 0.024}]}, "adds one deck serviceability record (acceleration, twist, deflection) for beam B1"),
    ("change-effects", "🔗", "half-wind", {"newEffects": [{**effect, "influence": 0.5} if effect["actionId"] == "Q-wind" else effect for effect in EFFECTS]}, "halves the influence of the wind action on beam B1"),
    ("remove-effect", "🔌", "e-1", {"index": 5}, "detaches the seismic action E-1 from beam B1"),
    ("remove-member", "🪚", "beam-b1", {"index": 0}, "removes beam B1"),
    ("remove-seismic", "❌", "e-1", {"index": 0}, "removes the seismic action E-1"),
    ("remove-accidental", "🧯", "impact", {"index": 0}, "removes the impact action A-impact"),
    ("remove-variable", "📤", "wind", {"index": 1}, "removes the wind action Q-wind"),
    ("remove-permanent", "➖", "g-inf", {"index": 1}, "removes the favourable permanent action G-inf"),
    ("insert-effect", "📎", "office-half", {"index": 6, "item": {"memberId": "beam-B1", "actionId": "Q-office", "influence": 0.5}}, "adds a second office load path into beam B1 at half influence"),
    ("insert-member", "🔩", "beam-b2", {"index": 1, "item": {**BEAM, "id": "beam-B2", "labelEn": "Office beam B2", "labelDe": "Büroträger B2", "span": 7.5}}, "adds a 7.5 m office beam B2 behind beam B1"),
    ("insert-seismic", "🌋", "class-iv", {"index": 1, "item": {"id": "E-2", "aEk": 45000.0, "importanceClass": "IV"}}, "adds a 45 kN seismic action E-2 of importance class IV"),
    ("insert-accidental", "💣", "explosion", {"index": 1, "item": {"id": "A-explosion", "ad": 30000.0}}, "adds a 30 kN explosion action"),
    ("insert-variable", "📥", "snow", {"index": 2, "item": SNOW}, "adds a 15 kN snow action behind the wind action"),
    ("insert-permanent", "➕", "finishes", {"index": 2, "item": {"id": "G-finish", "kind": "g_sup", "gk": 12000.0}}, "adds a 12 kN unfavourable finishes action"),
]

STANDARDS = {
    "en1991": {
        "artifact": "🏋️en1991",
        "vectors": EN1991,
        "catalog": "en1991-1-any",
        "case": "🏋️mutate-en1991-1",
        "aggregate": "En1991Mutation",
        "snapshot": "En1991Snapshot",
        "diff": "En1991Diff",
        "imports": "use crate::{En1991Diff, En1991Mutation, En1991Snapshot};",
        "wire": lambda kind, payload: {pascal(kind): payload},
        "title": "EN 1991",
    },
    "en1990": {
        "artifact": "⚖️en1990",
        "vectors": EN1990,
        "catalog": "en1990-1-any",
        "case": "⚖️mutate-en1990-1",
        "aggregate": "En1990Mutation",
        "snapshot": "En1990Snapshot",
        "diff": "En1990Diff",
        "imports": "use crate::diff::En1990Diff;\nuse crate::{En1990Mutation, En1990Snapshot};",
        "wire": lambda kind, payload: {"mutation": camel(kind), **payload},
        "title": "EN 1990",
    },
}


def pascal(kind):
    """🐫️ `change-height` → `ChangeHeight`, the Rust variant spelling."""
    return "".join(part[:1].upper() + part[1:] for part in kind.split("-"))


def camel(kind):
    """🐪️ `change-height` → `changeHeight`, the camelCase tag spelling."""
    head = pascal(kind)
    return head[:1].lower() + head[1:]


def subset_root(standard):
    """📂️ The `✳️any` subset owner of one standard."""
    return ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts" / STANDARDS[standard]["artifact"] / "🏅️standards/🔖️1/🪆️subsets/✳️any"


def leaf_dirs(standard):
    """🍃️ semantic kind → leaf directory name, read off each leaf descriptor."""
    mutations = subset_root(standard) / "🧬️schema/🧬️mutations"
    found = {}
    for entry in sorted(mutations.iterdir()):
        descriptor = entry / "🔣️.json"
        declared = json.loads(descriptor.read_text(encoding="utf-8")) if descriptor.is_file() else {}
        if "semanticKind" in declared:
            found[declared["semanticKind"]] = entry.name
    return found


def rows(standard):
    """🧾️ Every vector of one standard in catalog (declaration) order, with its leaf and scenario directory names."""
    table = STANDARDS[standard]
    leaves = leaf_dirs(standard)
    kinds = [kind for kind, *_ in table["vectors"]]
    assert sorted(kinds) == sorted(leaves), (sorted(set(kinds) ^ set(leaves)))
    for kind, emoji, slug, payload, summary in table["vectors"]:
        yield {"kind": kind, "leafDir": leaves[kind], "scenarioDir": unicodedata.normalize("NFC", emoji + slug), "slug": slug, "emoji": emoji, "payload": payload, "summary": summary, "wire": table["wire"](kind, payload)}


def rust_kinds(standard):
    """🦀️ The aggregate's `KINDS` constant, in declaration order."""
    source = (subset_root(standard) / "🧬️schema/🧬️mutations/🦀️.rs").read_text(encoding="utf-8")
    block = source.split("pub const KINDS: &[&str] = &[", 1)[1].split("];", 1)[0]
    return re.findall(r'"([a-z0-9-]+)"', block)


def plan(standard):
    """🗺️ The generator plan: one entry per vector."""
    GENERATED.mkdir(parents=True, exist_ok=True)
    entries = [{key: row[key] for key in ("kind", "leafDir", "scenarioDir", "wire")} for row in rows(standard)]
    path = GENERATED / f"plan-{standard}.json"
    path.write_text(json.dumps(entries, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(path)


def leaf_test(standard, row):
    """🧪️ One leaf's canonical vector case."""
    bundle = f"../../../../../🧫️fixtures/🧬️mutations/{row['leafDir']}/{row['scenarioDir']}"
    name = f"{row['kind']}-{row['slug']}".replace("-", "_")
    return (
        f"//! {row['emoji']} `{row['kind']}` — {row['summary']}.\n"
        f"//!\n"
        f"//! @see {bundle} — the committed vector.\n"
        f"\n"
        f"/// {row['emoji']} The committed `{row['kind']}` vector holds the specification-vector law.\n"
        f"#[test]\n"
        f"fn {name}() {{\n"
        f"    super::assert_vector(super::Vector {{\n"
        f"        kind: \"{row['kind']}\",\n"
        f"        before: include_str!(\"{bundle}/📸️snapshot/⬅️before/🔣️.json\"),\n"
        f"        mutation: include_str!(\"{bundle}/🦠️mutation/🔣️.json\"),\n"
        f"        after: include_str!(\"{bundle}/📸️snapshot/➡️after/🔣️.json\"),\n"
        f"        diff: include_str!(\"{bundle}/🔺️diff/🔣️.json\"),\n"
        f"        outcome: include_str!(\"{bundle}/🎯️outcome/🔣️.json\"),\n"
        f"    }});\n"
        f"}}\n"
    )


def fixture_module(standard):
    """🧫️ The crate's committed-vector law and one `#[path]` module per leaf case."""
    table = STANDARDS[standard]
    aggregate, snapshot, diff = table["aggregate"], table["snapshot"], table["diff"]
    mods = "".join(f"#[path = \"../../{row['leafDir']}/🧪️tests/{row['scenarioDir']}/🦀️.rs\"]\nmod {row['kind'].replace('-', '_')};\n" for row in rows(standard))
    return f"""//! 🧫️ Every committed `{aggregate}` specification vector — one canonical case per mutation leaf — held to one law.
//!
//! @see ../../../../🧫️fixtures/🧬️mutations — the committed `(before, mutation, after, diff, outcome)` bundles.
//! @see ../../../../🔮️oracles/🔣️.json — the `{table['catalog']}` catalog that registers each bundle.
//! @see ../../../../🧪️tests/{table['case']}/🥒️.feature — the independent Python reference reading the same bundles.

{table['imports']}
use dsl::ToValue;
use protocol::{{Mutation, MutationDiff}};

//#region 🧾️Vector
/// 🧾️ One committed vector: the semantic kind it witnesses and its five committed files.
pub(crate) struct Vector {{
    pub(crate) kind: &'static str,
    pub(crate) before: &'static str,
    pub(crate) mutation: &'static str,
    pub(crate) after: &'static str,
    pub(crate) diff: &'static str,
    pub(crate) outcome: &'static str,
}}

/// 🔣️ A committed file as the independent `serde_json` oracle reads it.
fn committed(text: &str) -> serde_json::Value {{
    serde_json::from_str(text).expect("a committed vector file is JSON")
}}

/// 🪞️ A production value as the independent `serde_json` oracle reads its Rust wire.
fn wire<T: ToValue>(value: &T) -> serde_json::Value {{
    serde_json::Value::from(value.to_value())
}}

/// 🎯️ The same op addressing a position no collection holds, for an op whose payload carries an `index`.
fn out_of_range(op: &{aggregate}) -> Option<{aggregate}> {{
    let mut payload = serde_json::Value::from(op.payload_value());
    *payload.get_mut("index")? = serde_json::Value::from(u64::from(u32::MAX));
    op.with_payload_value(dsl::DslValue::from(&payload)).ok()
}}

/// ⚖️ The law every committed vector obeys: the mutation file is the canonical Rust wire of one `kind` op whose binary frame
/// round-trips; both snapshots and the diff are canonical; production dispatch turns BEFORE into exactly the committed diff
/// under the committed outcome and lands on AFTER; the committed diff alone carries BEFORE to AFTER; the op's own inverse
/// restores BEFORE; and the leaf descriptor declares exactly the outcome classes dispatch reaches from the vector — its own
/// status, `no-op` when re-applying the op to AFTER changes nothing, `rejected` when the op addressing an index no collection
/// holds is refused.
pub(crate) fn assert_vector(vector: Vector) {{
    let kind = vector.kind;
    let op: {aggregate} = store::os_store::test_support::assert_wire_witness(vector.mutation);
    assert_eq!(op.descriptor().semantic_kind, kind, "{{kind}}: the committed mutation is another kind's op");
    let framed = protocol::OpBinary::encode_op(&op).expect("the op encodes to its binary frame");
    assert_eq!(<{aggregate} as protocol::OpBinary>::decode_op(&framed).expect("its binary frame decodes"), op, "{{kind}}: the binary frame does not round-trip");
    let before: {snapshot} = pack::json::from_json_str(vector.before).expect("the committed before-snapshot decodes");
    let after: {snapshot} = pack::json::from_json_str(vector.after).expect("the committed after-snapshot decodes");
    let delta: {diff} = pack::json::from_json_str(vector.diff).expect("the committed diff decodes");
    assert_eq!(wire(&before), committed(vector.before), "{{kind}}: the committed before-snapshot is not the canonical wire");
    assert_eq!(wire(&after), committed(vector.after), "{{kind}}: the committed after-snapshot is not the canonical wire");
    assert_eq!(wire(&delta), committed(vector.diff), "{{kind}}: the committed diff is not the canonical wire");
    let outcome = op.diff(&before);
    assert_eq!(wire(outcome.diff()), committed(vector.diff), "{{kind}}: production dispatch produces another diff than the committed one");
    let status = committed(vector.outcome)["status"].as_str().expect("the committed outcome names its status").to_string();
    match status.as_str() {{
        "applied" => assert!(outcome.messages().is_empty(), "{{kind}}: an applied vector raises {{:?}}", outcome.messages()),
        "no-op" => assert_ne!(outcome.worst_level(), Some(protocol::Severity::Fatal), "{{kind}}: a no-op vector is refused"),
        "rejected" => assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal), "{{kind}}: a rejected vector is accepted"),
        other => panic!("{{kind}}: unknown committed outcome status {{other:?}}"),
    }}
    let applied = MutationDiff::apply(outcome.diff(), &before).expect("the produced diff applies to the committed before-snapshot");
    assert_eq!(applied, after, "{{kind}}: production dispatch does not land on the committed after-snapshot");
    assert_eq!(MutationDiff::apply(&delta, &before).expect("the committed diff applies to the committed before-snapshot"), after, "{{kind}}: the committed diff does not carry before to after");
    assert_eq!(status == "applied", applied != before, "{{kind}}: an applied vector must move the document and only an applied one may");
    let inverse = op.inverse(&before);
    assert_eq!(status == "applied", !inverse.is_empty(), "{{kind}}: an applied vector computes a non-empty inverse and only an applied one does");
    let restored = inverse.iter().fold(applied, |current, step| MutationDiff::apply(step.diff(&current).diff(), &current).expect("an inverse step applies"));
    assert_eq!(restored, before, "{{kind}}: replaying the inverse does not restore the committed before-snapshot");
    let no_op = op.diff(&after).messages().iter().any(|message| message.code.0 == "mutation.no-op");
    let rejected = out_of_range(&op).is_some_and(|stray| stray.diff(&before).worst_level() >= Some(protocol::Severity::Error));
    let reached: Vec<&str> = [(true, status.as_str()), (no_op, "no-op"), (rejected, "rejected")].into_iter().filter_map(|(reached, class)| reached.then_some(class)).collect();
    let declared: Vec<&str> = op.descriptor().outcome_classes.iter().map(|class| class.as_str()).collect();
    assert_eq!(declared, reached, "{{kind}}: the descriptor's outcome classes are not the ones production dispatch reaches");
}}
//#endregion 🧾️Vector

//#region 🧪️Cases
{mods}//#endregion 🧪️Cases
"""


def catalog(standard):
    """📇️ Rewrites the subset catalog's `kinds` and `vectors` from the vectors and the aggregate declaration order."""
    table = STANDARDS[standard]
    path = subset_root(standard) / "🔮️oracles/🔣️.json"
    manifest = json.loads(path.read_text(encoding="utf-8"))
    kinds = rust_kinds(standard)
    by_kind = {row["kind"]: row for row in rows(standard)}
    assert sorted(kinds) == sorted(by_kind)
    entry = next(entry for entry in manifest["mutationCatalogs"] if entry["id"] == table["catalog"])
    entry["vectors"] = [{"mutationId": kind, "sourceMutationDirectoryName": by_kind[kind]["leafDir"], "mutationDirectoryName": by_kind[kind]["leafDir"], "scenarios": [{"id": by_kind[kind]["slug"], "directoryName": by_kind[kind]["scenarioDir"]}]} for kind in kinds]
    entry["kinds"] = kinds
    for manifest_entry in manifest.get("mutationManifests", []):
        for mutation in manifest_entry["mutations"]:
            mutation["payloadSchema"] = "🧬️schema/🔣️.json"
    manifest.pop("semanticKinds", None)
    path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(path)


def examples(standard):
    """🥒️ The feature's two Examples tables, padded like the committed feature files."""
    table = [("id", "dir", "fixture")] + [(row["kind"], row["leafDir"], row["scenarioDir"]) for row in rows(standard)]
    widths = [max(len(cells[index]) for cells in table) for index in range(3)]
    return "".join("      | " + " | ".join(cell.ljust(width) for cell, width in zip(cells, widths)) + " |\n" for cells in table)


def sources(standard):
    """✍️ Writes every leaf case, the fixture module and the catalog; prints the Examples block for the feature."""
    root = subset_root(standard) / "🧬️schema/🧬️mutations"
    for row in rows(standard):
        case = root / row["leafDir"] / "🧪️tests" / row["scenarioDir"]
        case.mkdir(parents=True, exist_ok=True)
        (case / "🦀️.rs").write_text(leaf_test(standard, row), encoding="utf-8")
    (root / "🧪️tests/🔬️fixture/🦀️.rs").write_text(fixture_module(standard), encoding="utf-8")
    catalog(standard)
    GENERATED.mkdir(parents=True, exist_ok=True)
    (GENERATED / f"examples-{standard}.txt").write_text(examples(standard), encoding="utf-8")
    print(GENERATED / f"examples-{standard}.txt")


def stubs(standard):
    """🧹️ The leaf test directories the new canonical cases supersede."""
    root = subset_root(standard) / "🧬️schema/🧬️mutations"
    keep = {(row["leafDir"], row["scenarioDir"]) for row in rows(standard)}
    for leaf in sorted(root.iterdir()):
        tests = leaf / "🧪️tests"
        if leaf.name not in {row["leafDir"] for row in rows(standard)} or not tests.is_dir():
            continue
        for case in sorted(tests.iterdir()):
            if (leaf.name, case.name) not in keep:
                print(case)


if __name__ == "__main__":
    command, standard = sys.argv[1], sys.argv[2]
    {"plan": plan, "sources": sources, "stubs": stubs}[command](standard)
