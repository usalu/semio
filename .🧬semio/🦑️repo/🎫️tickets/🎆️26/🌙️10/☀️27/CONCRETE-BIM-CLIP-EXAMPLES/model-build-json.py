"""🏗️ Ticket script: handcraft the Villa Savoye teaching model JSON (input script, kept with the ticket)."""

import importlib.util
import json
import sys
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[7]
CLIP = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"
sys.path.insert(0, str(ROOT))
api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
gid = api.global_id

ARCH, STRUCT, SERV = "Atelier (architecture)", "Structural engineer", "Heating engineer"
VS = "research-villa-savoye"


def q(value, unit="m", basis="assumed", source=None):
    out = {"value": value, "unit": unit, "basis": basis}
    if source:
        out["source"] = source
    return out


def n(en, de):
    return {"en": en, "de": de}


def prop(value, ifc_type, basis="assumed", source=None):
    out = {"value": value, "ifc_type": ifc_type, "basis": basis}
    if source:
        out["source"] = source
    return out


def meta(author, domains, revision=1):
    return {"author": author, "revision": revision, "domains": domains}


def base(tag, cls, predefined, name, storey, author, domains, **rest):
    return {"guid": gid(tag), "tag": tag, "class": cls, "predefined_type": predefined, "name": name,
            "storey": storey, **rest, "metadata": meta(author, domains)}


GX = [1.25, 6.0, 10.75, 15.5, 20.25]
GY = [0.0, 4.75, 9.5, 14.25, 19.0]
ROWS, COLS = "ABCDE", "12345"
BOX = [[0, 0], [21.5, 0], [21.5, 19], [0, 19]]


def rect(x0, y0, x1, y1):
    return [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]


RAMP_VOID = rect(7.6, 2.8, 10.0, 14.5)
TERRACE = rect(10.5, 4.2, 21.2, 14.5)
SOLARIUM = rect(0.5, 4.5, 7.5, 14.5)

elements = []
for j, row in enumerate(ROWS):
    for i, col in enumerate(COLS):
        tag = row + col
        elements.append(base(tag, "IfcColumn", "COLUMN", n(f"Piloti {tag}", f"Piloti-Stütze {tag}"), "ground", STRUCT, ["structure"],
                             type="TY-PILOTI", geometry={"axis": [GX[i], GY[j]], "base_level": "ground", "top_level": "soffit:SL-1"},
                             attributes={}, psets={}))

elements += [
    base("SL-1", "IfcSlab", "FLOOR", n("First-floor slab", "Geschossdecke über dem Erdgeschoss"), "first", STRUCT, ["architecture", "structure"],
         material="MAT-RC", geometry={"outline": BOX, "voids": [{"tag": "V-RAMP", "outline": RAMP_VOID}], "level": "first"},
         attributes={"thickness": q(0.25)}, psets={"Pset_SlabCommon": {"LoadBearing": prop(True, "IfcBoolean", "sourced", "VS-3.1"),
                                                                       "IsExternal": prop(False, "IfcBoolean")}}),
    base("SL-R", "IfcSlab", "ROOF", n("Roof slab over the enclosed rooms", "Dachdecke über den Wohnräumen"), "roof", STRUCT, ["architecture", "structure"],
         material="LS-ROOF", geometry={"outline": BOX, "voids": [{"tag": "V-TERRACE", "outline": TERRACE}, {"tag": "V-RAMP-R", "outline": RAMP_VOID}],
                                       "level": "roof", "zones": {"solarium": "SP-SOLARIUM"}},
         attributes={}, psets={"Pset_SlabCommon": {"LoadBearing": prop(True, "IfcBoolean", "sourced", "VS-3.1"),
                                                   "IsExternal": prop(True, "IfcBoolean")}}),
]

walls = [("S", "South", "Süd", [[0, 0.15], [21.5, 0.15]], 1.5, 20.0),
         ("E", "East", "Ost", [[21.35, 0.3], [21.35, 18.7]], 1.5, 16.9),
         ("N", "North", "Nord", [[0, 18.85], [21.5, 18.85]], 1.5, 20.0),
         ("W", "West", "West", [[0.15, 0.3], [0.15, 18.7]], 1.5, 16.9)]
for s, en, de, path, start, end in walls:
    elements.append(base(f"W-{s}", "IfcWall", "STANDARD", n(f"{en} wall", f"{de}wand"), "first", ARCH, ["architecture"],
                         type="TY-WALL", geometry={"path": path, "base_level": "first", "top_level": "soffit:SL-R"}, attributes={}, psets={}))
for s, en, de, path, start, end in walls:
    elements.append(base(f"OP-{s}", "IfcOpeningElement", "OPENING", n(f"Ribbon opening {en.lower()}", f"Bandfensteröffnung {de}"), "first", ARCH, ["architecture"],
                         geometry={"wall": f"W-{s}"}, attributes={"start": q(start), "end": q(end), "sill": q(0.9), "head": q(2.0)}, psets={}))
for s, en, de, path, start, end in walls:
    elements.append(base(f"RW-{s}", "IfcWindow", "WINDOW", n(f"Ribbon window {en.lower()}", f"Bandfenster {de}"), "first", ARCH, ["architecture"],
                         type="TY-RIBBON", geometry={"opening": f"OP-{s}"}, attributes={}, psets={}))

elements.append(base("CW-1", "IfcCurtainWall", "NOTDEFINED", n("Curved glass wall", "Gebogene Glaswand"), "ground", ARCH, ["architecture"],
                     material="MAT-GLASS", geometry={"centre": [8.5, 12.0], "south_y": 4.5, "radius_from": "car", "base_level": "ground", "top_level": "soffit:SL-1"},
                     attributes={}, psets={"Pset_CurtainWallCommon": {"IsExternal": prop(True, "IfcBoolean")}}))

ramps = [("RM-1", "ground", "first", "Ramp, ground floor to first floor", "Rampe Erdgeschoss bis Obergeschoss", "ground"),
         ("RM-2", "first", "roof", "Ramp, first floor to roof", "Rampe Obergeschoss bis Dach", "first")]
for tag, lo, hi, en, de, storey in ramps:
    south = {"outline": rect(7.6, 2.8, 10.0, 4.0), "portion": 0.5}
    north = {"outline": rect(7.6, 14.5, 10.0, 15.1), "portion": 0.0}
    elements.append(base(tag, "IfcRamp", "HALF_TURN_RAMP", n(en, de), storey, ARCH, ["architecture", "structure"], material="MAT-RC",
                         geometry={"from_level": lo, "to_level": hi, "landings": [south, north]}, attributes={"width": q(1.0)},
                         psets={"Pset_RampCommon": {"IsExternal": prop(False, "IfcBoolean"), "LoadBearing": prop(True, "IfcBoolean")}}))
flights = [("RF-1A", "RM-1", [[8.2, 14.5], [8.2, 4.0]], 0.0, 0.5, "Ramp flight 1", "Rampenlauf 1", "ground"),
           ("RF-1B", "RM-1", [[9.4, 4.0], [9.4, 14.5]], 0.5, 1.0, "Ramp flight 2", "Rampenlauf 2", "ground"),
           ("RF-2A", "RM-2", [[9.4, 14.5], [9.4, 4.0]], 0.0, 0.5, "Ramp flight 3", "Rampenlauf 3", "first"),
           ("RF-2B", "RM-2", [[8.2, 4.0], [8.2, 14.5]], 0.5, 1.0, "Ramp flight 4", "Rampenlauf 4", "first")]
for tag, ramp, path, p0, p1, en, de, storey in flights:
    lo, hi = ("ground", "first") if ramp == "RM-1" else ("first", "roof")
    elements.append(base(tag, "IfcRampFlight", "STRAIGHT", n(en, de), storey, ARCH, ["architecture", "structure"], material="MAT-RC",
                         geometry={"ramp": ramp, "path": path, "from_level": lo, "to_level": hi, "portion": [p0, p1]}, attributes={"width": q(1.0)}, psets={}))

elements.append(base("SC-1", "IfcWall", "SOLIDWALL", n("Solarium screen", "Solarium-Windschutzwand"), "roof", ARCH, ["architecture"], material="MAT-RENDERED",
                     geometry={"path": [[7.5, 4.5], [2.0, 4.5], {"arc": {"centre": [2.0, 6.0], "radius": 1.5, "start": 270, "end": 180}},
                                        [0.5, 13.0], {"arc": {"centre": [2.0, 13.0], "radius": 1.5, "start": 180, "end": 90}}, [7.5, 14.5]],
                               "base_level": "roof", "top_level": "overall"},
                     attributes={"thickness": q(0.15)}, psets={"Pset_WallCommon": {"IsExternal": prop(True, "IfcBoolean"), "LoadBearing": prop(False, "IfcBoolean")}}))

elements.append(base("BL-1", "IfcBoiler", "WATER", n("Heating boiler", "Heizkessel"), "ground", SERV, ["services"], system="heating",
                     geometry={"points": [[6.0, 2.75, 0.0], [6.0, 2.75, 1.2]], "size": [0.6, 0.6, 1.2]}, attributes={}, psets={}))
elements.append(base("RAD-1", "IfcSpaceHeater", "RADIATOR", n("Radiator under the salon window", "Heizkörper unter dem Salonfenster"), "first", SERV, ["services"], system="heating",
                     geometry={"points": [[2.0, 0.45, 3.6], [4.0, 0.45, 3.6]], "size": [2.0, 0.1, 0.6]}, attributes={}, psets={}))
for k, (x, y) in enumerate([(1.40, 0.35), (20.0, 0.6), (1.5, 18.4), (20.0, 18.4)], 1):
    elements.append(base(f"DR-{k}", "IfcWasteTerminal", "ROOFDRAIN", n(f"Roof drain {k}", f"Dachablauf {k}"), "roof", SERV, ["services"], system="rainwater",
                         geometry={"points": [[x, y, "roof"]], "size": [0.3, 0.3, 0.1]}, attributes={}, psets={}))
for k, (x, y) in enumerate([(1.40, 0.35), (20.0, 0.6), (1.5, 18.4), (20.0, 18.4)], 1):
    elements.append(base(f"DP-{k}", "IfcPipeSegment", "RIGIDSEGMENT", n(f"Downpipe {k}", f"Fallrohr {k}"), "ground", SERV, ["services"], system="rainwater",
                         geometry={"points": [[x, y, "roof"], [x, y, "ground"]]}, attributes={"diameter": q(0.1)}, psets={}))
for k, (en, de, pts) in enumerate([("Heating riser", "Heizungssteigleitung", [[6.0, 2.75, 1.2], [6.0, 2.75, 3.6]]),
                                    ("Heating flow, salon", "Heizungsvorlauf Salon", [[6.0, 2.75, 3.6], [6.0, 0.45, 3.6]]),
                                    ("Heating branch, radiator", "Heizungsanbindung Heizkörper", [[6.0, 0.45, 3.6], [4.0, 0.45, 3.6]])], 1):
    elements.append(base(f"PS-{k}", "IfcPipeSegment", "RIGIDSEGMENT", n(en, de), "ground" if k == 1 else "first", SERV, ["services"], system="heating",
                         geometry={"points": pts}, attributes={"diameter": q(0.05)}, psets={}))

hall = [[5.0, 4.5], [12.0, 4.5], [12.0, 12.0], {"arc_of": "CW-1", "reverse": True}, [5.0, 12.0]]
spaces = [
    {"guid": gid("SP-HALL"), "tag": "SP-HALL", "class": "IfcSpace", "predefined_type": "INTERNAL", "name": n("Entrance hall with ramp foot", "Eingangshalle mit Rampenfuß"),
     "storey": "ground", "outline": hall, "metadata": meta(ARCH, ["architecture"])},
    {"guid": gid("SP-GARAGE"), "tag": "SP-GARAGE", "class": "IfcSpace", "predefined_type": "PARKING", "name": n("Garage", "Garage"),
     "storey": "ground", "outline": rect(7.0, 1.0, 12.0, 4.5), "metadata": meta(ARCH, ["architecture"])},
    {"guid": gid("SP-BOILER"), "tag": "SP-BOILER", "class": "IfcSpace", "predefined_type": "INTERNAL", "name": n("Boiler room", "Heizraum"),
     "storey": "ground", "outline": rect(5.0, 1.0, 7.0, 4.5), "metadata": meta(ARCH, ["architecture"])},
    {"guid": gid("SP-SALON"), "tag": "SP-SALON", "class": "IfcSpace", "predefined_type": "INTERNAL", "name": n("Salon", "Salon"),
     "storey": "first", "outline": rect(0.3, 0.3, 7.6, 9.0), "metadata": meta(ARCH, ["architecture"])},
    {"guid": gid("SP-TERRACE"), "tag": "SP-TERRACE", "class": "IfcSpace", "predefined_type": "EXTERNAL", "name": n("Roof garden (hanging garden)", "Dachgarten (hängender Garten)"),
     "storey": "first", "outline": TERRACE, "reference_area": q(110.0, "m²", "sourced", "VS-2.7"), "metadata": meta(ARCH, ["architecture"])},
    {"guid": gid("SP-SOLARIUM"), "tag": "SP-SOLARIUM", "class": "IfcSpace", "predefined_type": "EXTERNAL", "name": n("Solarium", "Solarium"),
     "storey": "roof", "outline": SOLARIUM, "reference_area": q(70.0, "m²", "sourced", "VS-2.8"), "metadata": meta(ARCH, ["architecture"])},
]

storeys = [
    {"guid": gid("ground"), "tag": "ground", "id": "ground", "class": "IfcBuildingStorey", "name": n("Ground floor", "Erdgeschoss"), "elevation": q(0.0)},
    {"guid": gid("first"), "tag": "first", "id": "first", "class": "IfcBuildingStorey", "name": n("First floor", "Obergeschoss"), "elevation": q(3.5)},
    {"guid": gid("roof"), "tag": "roof", "id": "roof", "class": "IfcBuildingStorey", "name": n("Roof", "Dach"), "elevation": q(6.7)},
]

types = [
    {"guid": gid("TY-PILOTI"), "tag": "TY-PILOTI", "class": "IfcColumnType", "predefined_type": "COLUMN", "name": n("Piloti", "Piloti-Stütze"),
     "material": "MAT-RC", "profile": {"shape": "circle", "diameter": q(0.3)}, "attributes": {},
     "psets": {"Pset_ColumnCommon": {"LoadBearing": prop(True, "IfcBoolean", "sourced", "VS-3.1"), "IsExternal": prop(True, "IfcBoolean"),
                                     "FireRating": prop("R 60", "IfcLabel")}}, "metadata": meta(STRUCT, ["structure"])},
    {"guid": gid("TY-RIBBON"), "tag": "TY-RIBBON", "class": "IfcWindowType", "predefined_type": "WINDOW", "name": n("Ribbon window", "Bandfenster"),
     "material": "MAT-GLASS", "attributes": {}, "psets": {"Pset_WindowCommon": {"IsExternal": prop(True, "IfcBoolean"), "FireRating": prop("none", "IfcLabel")}},
     "metadata": meta(ARCH, ["architecture"])},
    {"guid": gid("TY-WALL"), "tag": "TY-WALL", "class": "IfcWallType", "predefined_type": "STANDARD", "name": n("Infill wall", "Ausfachungswand"),
     "material": "LS-WALL", "attributes": {},
     "psets": {"Pset_WallCommon": {"LoadBearing": prop(False, "IfcBoolean", "sourced", "VS-3.1"), "IsExternal": prop(True, "IfcBoolean"),
                                   "FireRating": prop("EI 30", "IfcLabel")}}, "metadata": meta(ARCH, ["architecture"])},
]


def mat(tag, en, de, density):
    return {"guid": gid(tag), "tag": tag, "class": "IfcMaterial", "name": n(en, de), "density": q(density, "kN/m³")}


def layer(lid, material, en, de, thickness, zones=None):
    out = {"id": lid, "material": material, "name": n(en, de), "thickness": q(thickness)}
    if zones:
        out["zones"] = zones
    return out


materials = [
    mat("MAT-RC", "Reinforced concrete", "Stahlbeton", 25.0),
    mat("MAT-WATERPROOF", "Waterproofing", "Abdichtung", 10.0),
    mat("MAT-SCREED", "Screed", "Estrich", 22.0),
    mat("MAT-SOIL", "Planting soil", "Pflanzsubstrat", 18.0),
    mat("MAT-RENDER", "Render", "Außenputz", 20.0),
    mat("MAT-BRICK", "Brick infill", "Ziegelausfachung", 18.0),
    mat("MAT-PLASTER", "Plaster", "Innenputz", 18.0),
    mat("MAT-GLASS", "Glass", "Glas", 25.0),
    mat("MAT-RENDERED", "Rendered concrete", "Verputzter Beton", 24.0),
    {"guid": gid("LS-ROOF"), "tag": "LS-ROOF", "class": "IfcMaterialLayerSet", "name": n("Roof build-up", "Dachaufbau"),
     "layers": [layer("concrete", "MAT-RC", "Reinforced concrete slab", "Stahlbetondecke", 0.20, ["planted", "solarium"]),
                layer("waterproofing", "MAT-WATERPROOF", "Waterproofing", "Abdichtung", 0.02, ["planted", "solarium"]),
                layer("screed", "MAT-SCREED", "Screed to falls", "Gefälleestrich", 0.06, ["planted", "solarium"]),
                layer("soil", "MAT-SOIL", "Planting soil", "Pflanzsubstrat", 0.10, ["planted"])]},
    {"guid": gid("LS-WALL"), "tag": "LS-WALL", "class": "IfcMaterialLayerSet", "name": n("Infill wall build-up", "Wandaufbau Ausfachung"),
     "layers": [layer("render", "MAT-RENDER", "Render (white)", "Außenputz (weiß)", 0.03),
                layer("brick", "MAT-BRICK", "Brick infill", "Ziegelausfachung", 0.24),
                layer("plaster", "MAT-PLASTER", "Plaster", "Innenputz", 0.03)]},
]

rel_list = []


def rel(cls, relating, related, en, de):
    tag = f"REL-{len(rel_list) + 1:03d}"
    rel_list.append({"guid": gid(tag), "tag": tag, "class": cls, "relating": relating, "related": related, "name": n(en, de)})


rel("IfcRelAggregates", "PROJECT", ["SITE"], "Project contains the site", "Projekt enthält das Grundstück")
rel("IfcRelAggregates", "SITE", ["BUILDING"], "Site contains the building", "Grundstück enthält das Gebäude")
rel("IfcRelAggregates", "BUILDING", ["ground", "first", "roof"], "Building is divided into storeys", "Gebäude gliedert sich in Geschosse")
for st in ("ground", "first", "roof"):
    sp = [s["tag"] for s in spaces if s["storey"] == st]
    rel("IfcRelAggregates", st, sp, "Storey is divided into spaces", "Geschoss gliedert sich in Räume")
rel("IfcRelAggregates", "RM-1", ["RF-1A", "RF-1B"], "Ramp is made of flights", "Rampe besteht aus Läufen")
rel("IfcRelAggregates", "RM-2", ["RF-2A", "RF-2B"], "Ramp is made of flights", "Rampe besteht aus Läufen")
for st in ("ground", "first", "roof"):
    tags = [e["tag"] for e in elements if e["storey"] == st]
    rel("IfcRelContainedInSpatialStructure", st, tags, "Storey contains the elements", "Geschoss enthält die Bauteile")
rel("IfcRelDefinesByType", "TY-PILOTI", [e["tag"] for e in elements if e["class"] == "IfcColumn"], "Type defines all pilotis", "Typ definiert alle Pilotis")
rel("IfcRelDefinesByType", "TY-RIBBON", [f"RW-{w[0]}" for w in walls], "Type defines the ribbon windows", "Typ definiert die Bandfenster")
rel("IfcRelDefinesByType", "TY-WALL", [f"W-{w[0]}" for w in walls], "Type defines the infill walls", "Typ definiert die Ausfachungswände")
for w in walls:
    rel("IfcRelVoidsElement", f"W-{w[0]}", [f"OP-{w[0]}"], "Wall is voided by the opening", "Wand wird durch die Öffnung durchbrochen")
for w in walls:
    rel("IfcRelFillsElement", f"OP-{w[0]}", [f"RW-{w[0]}"], "Window fills the opening", "Fenster füllt die Öffnung")
for e in elements:
    if e["class"] == "IfcColumn":
        rel("IfcRelConnectsElements", e["tag"], ["SL-1"], "Piloti carries the slab", "Piloti trägt die Decke")
rel("IfcRelAssociatesMaterial", "MAT-RC", ["TY-PILOTI", "SL-1", "RM-1", "RM-2", "RF-1A", "RF-1B", "RF-2A", "RF-2B"], "Reinforced concrete is assigned", "Stahlbeton ist zugewiesen")
rel("IfcRelAssociatesMaterial", "LS-ROOF", ["SL-R"], "Roof build-up is assigned", "Dachaufbau ist zugewiesen")
rel("IfcRelAssociatesMaterial", "LS-WALL", ["TY-WALL"], "Wall build-up is assigned", "Wandaufbau ist zugewiesen")
rel("IfcRelAssociatesMaterial", "MAT-GLASS", ["TY-RIBBON", "CW-1"], "Glass is assigned", "Glas ist zugewiesen")
rel("IfcRelAssociatesMaterial", "MAT-RENDERED", ["SC-1"], "Rendered concrete is assigned", "Verputzter Beton ist zugewiesen")

arch_tags = [e["tag"] for e in elements if e["class"] in ("IfcWall", "IfcOpeningElement", "IfcWindow", "IfcCurtainWall", "IfcRamp", "IfcRampFlight")] + ["SL-1", "SL-R"] + [s["tag"] for s in spaces]
struct_tags = [e["tag"] for e in elements if e["class"] in ("IfcColumn", "IfcRamp", "IfcRampFlight")] + ["SL-1", "SL-R"]
serv_tags = [e["tag"] for e in elements if "services" in e["metadata"]["domains"]]
models = [
    {"id": "architecture", "schema_id": "IFC4X3_ADD2", "author": ARCH, "name": n("Architecture model", "Architekturmodell"), "revision": 1, "element_tags": arch_tags},
    {"id": "structure", "schema_id": "IFC4X3_ADD2", "author": STRUCT, "name": n("Structure model", "Tragwerksmodell"), "revision": 1, "element_tags": struct_tags},
    {"id": "services", "schema_id": "IFC4X3_ADD2", "author": SERV, "name": n("Services model", "Haustechnikmodell"), "revision": 1, "element_tags": serv_tags},
]

notes = [
    ("scope", "Simplified teaching model of the Villa Savoye (Poissy, 1928-1931): rounded values, one orientation, selected elements only.",
     "Vereinfachtes Lehrmodell der Villa Savoye (Poissy, 1928-1931): gerundete Werte, eine Ausrichtung, ausgewählte Bauteile."),
    ("piloti-count", "25 pilotis on a 4.75 m grid are a teaching assumption; the sources disagree on the real count (25 versus 15).",
     "25 Pilotis auf dem 4,75-m-Raster sind eine Lehrannahme; die Quellen widersprechen sich bei der echten Anzahl (25 gegenüber 15)."),
    ("piloti-diameter", "Piloti diameter 0.30 m, storey elevations, slab thickness and wall build-up are assumed; no source gives them.",
     "Pilotidurchmesser 0,30 m, Geschosshöhen, Deckendicke und Wandaufbau sind angenommen; keine Quelle nennt sie."),
    ("ribbon-windows", "Ribbon windows are modelled on all four first-floor façades; sources conflict (all sides versus only the south façade).",
     "Bandfenster sind an allen vier Fassaden des Obergeschosses modelliert; die Quellen widersprechen sich (alle Seiten gegenüber nur Südseite)."),
    ("infill-walls", "Walls are infill on the concrete frame (LoadBearing FALSE); the pilotis carry the slab (LoadBearing TRUE).",
     "Wände sind Ausfachung des Betonskeletts (LoadBearing FALSE); die Pilotis tragen die Decke (LoadBearing TRUE)."),
    ("ground-enclosure", "The ground-floor enclosure is set back from the façades; its curved glass end follows the turning path of an assumed 1929 car.",
     "Der Erdgeschossraum ist gegenüber den Fassaden zurückgesetzt; sein gebogenes Glasende folgt der Wendekurve eines angenommenen Autos von 1929."),
    ("enclosed-area", "The model encloses about 96 m² on the ground floor; the 190 m² reported by one source is not reproduced (service rooms are not modelled).",
     "Das Modell schließt im Erdgeschoss rund 96 m² ein; die 190 m² einer Quelle werden nicht erreicht (Nebenräume sind nicht modelliert)."),
    ("ramp", "The ramp is two half-turn ramps with four straight flights at an assumed slope of about 1:6; the real slope and length are not sourced.",
     "Die Rampe besteht aus zwei Halbwendelrampen mit vier geraden Läufen bei angenommener Steigung von etwa 1:6; echte Steigung und Länge sind nicht belegt."),
    ("garden-fit", "Roof garden and solarium areas are fitted to the sourced 110 m² and 70 m² (+0.2 % and +0.0 %).",
     "Dachgarten- und Solariumfläche sind an die belegten 110 m² und 70 m² angepasst (+0,2 % und +0,0 %)."),
    ("hanging-garden", "Planting beds on the hanging garden are not loaded separately; only the roof slab carries a soil layer.",
     "Pflanzbeete im Dachgarten sind nicht gesondert belastet; nur die Dachdecke trägt eine Substratschicht."),
    ("ifc-names", "IFC class chains and attributes follow IFC 4.3.2.0 as summarised in the research; items marked as memory there still need a check against the official schema.",
     "IFC-Klassenketten und Attribute folgen IFC 4.3.2.0 wie in der Recherche zusammengefasst; dort als Erinnerung markierte Punkte brauchen noch eine Prüfung am offiziellen Schema."),
    ("tributary", "Piloti reactions are tributary-area estimates (area load times area), not finite element results.",
     "Pilotilasten sind Einflussflächen-Näherungen (Flächenlast mal Fläche), keine Finite-Elemente-Ergebnisse."),
]

classes = {
    "IfcRoot": (None, ["GlobalId", "OwnerHistory", "Name", "Description"]),
    "IfcObjectDefinition": ("IfcRoot", []),
    "IfcObject": ("IfcObjectDefinition", ["ObjectType"]),
    "IfcProduct": ("IfcObject", ["ObjectPlacement", "Representation"]),
    "IfcElement": ("IfcProduct", ["Tag"]),
    "IfcBuiltElement": ("IfcElement", []),
    "IfcColumn": ("IfcBuiltElement", ["PredefinedType"]),
    "IfcSlab": ("IfcBuiltElement", ["PredefinedType"]),
    "IfcWall": ("IfcBuiltElement", ["PredefinedType"]),
    "IfcWindow": ("IfcBuiltElement", ["OverallHeight", "OverallWidth", "PredefinedType", "PartitioningType", "UserDefinedPartitioningType"]),
    "IfcCurtainWall": ("IfcBuiltElement", ["PredefinedType"]),
    "IfcRamp": ("IfcBuiltElement", ["PredefinedType"]),
    "IfcRampFlight": ("IfcBuiltElement", ["PredefinedType"]),
    "IfcFeatureElement": ("IfcElement", []),
    "IfcFeatureElementSubtraction": ("IfcFeatureElement", []),
    "IfcOpeningElement": ("IfcFeatureElementSubtraction", ["PredefinedType"]),
    "IfcDistributionElement": ("IfcElement", []),
    "IfcDistributionFlowElement": ("IfcDistributionElement", []),
    "IfcEnergyConversionDevice": ("IfcDistributionFlowElement", []),
    "IfcBoiler": ("IfcEnergyConversionDevice", ["PredefinedType"]),
    "IfcFlowTerminal": ("IfcDistributionFlowElement", []),
    "IfcSpaceHeater": ("IfcFlowTerminal", ["PredefinedType"]),
    "IfcWasteTerminal": ("IfcFlowTerminal", ["PredefinedType"]),
    "IfcFlowSegment": ("IfcDistributionFlowElement", []),
    "IfcPipeSegment": ("IfcFlowSegment", ["PredefinedType"]),
    "IfcSpatialElement": ("IfcProduct", ["LongName"]),
    "IfcSpatialStructureElement": ("IfcSpatialElement", ["CompositionType"]),
    "IfcSpace": ("IfcSpatialStructureElement", ["PredefinedType", "ElevationWithFlooring"]),
    "IfcBuildingStorey": ("IfcSpatialStructureElement", ["Elevation"]),
    "IfcSite": ("IfcSpatialStructureElement", ["RefLatitude", "RefLongitude", "RefElevation", "LandTitleNumber", "SiteAddress"]),
    "IfcBuilding": ("IfcSpatialStructureElement", ["ElevationOfRefHeight", "ElevationOfTerrain", "BuildingAddress"]),
    "IfcProject": ("IfcObjectDefinition", ["ObjectType", "LongName", "Phase", "RepresentationContexts", "UnitsInContext"]),
    "IfcStructuralItem": ("IfcProduct", []),
    "IfcStructuralMember": ("IfcStructuralItem", []),
    "IfcStructuralCurveMember": ("IfcStructuralMember", ["PredefinedType", "Axis"]),
    "IfcStructuralSurfaceMember": ("IfcStructuralMember", ["PredefinedType", "Thickness"]),
    "IfcStructuralConnection": ("IfcStructuralItem", ["AppliedCondition"]),
    "IfcStructuralPointConnection": ("IfcStructuralConnection", ["ConditionCoordinateSystem"]),
}
classes_json = {k: {"supertype": v[0], "adds": v[1]} for k, v in classes.items()}
types_ifc = ["IfcColumnType", "IfcWindowType", "IfcWallType", "IfcMaterial", "IfcMaterialLayerSet"]

property_names = {
    "diameter": (n("Diameter", "Durchmesser"), "m"),
    "height": (n("Height", "Höhe"), "m"),
    "length": (n("Length", "Länge"), "m"),
    "width": (n("Width", "Breite"), "m"),
    "thickness": (n("Thickness", "Dicke"), "m"),
    "volume": (n("Volume", "Volumen"), "m³"),
    "area": (n("Area", "Fläche"), "m²"),
    "cross_section_area": (n("Cross-section area", "Querschnittsfläche"), "m²"),
    "spacing": (n("Grid spacing", "Achsabstand"), "m"),
    "initial_spacing": (n("Initial grid spacing", "Ursprünglicher Achsabstand"), "m"),
    "clear_width": (n("Clear width between pilotis", "Lichte Weite zwischen den Pilotis"), "m"),
    "car_width": (n("Car width", "Fahrzeugbreite"), "m"),
    "car_length": (n("Car length", "Fahrzeuglänge"), "m"),
    "turning_radius": (n("Turning radius", "Wenderadius"), "m"),
    "material": (n("Material", "Material"), ""),
    "LoadBearing": (n("Load-bearing", "Tragend"), ""),
    "IsExternal": (n("External", "Außenbauteil"), ""),
    "FireRating": (n("Fire rating", "Feuerwiderstand"), ""),
    "GlobalId": (n("GlobalId", "GlobalId"), ""),
    "PredefinedType": (n("Predefined type", "Vordefinierter Typ"), ""),
    "sill": (n("Sill height", "Brüstungshöhe"), "m"),
    "head": (n("Head height", "Sturzhöhe"), "m"),
    "start": (n("Opening start", "Öffnungsbeginn"), "m"),
    "end": (n("Opening end", "Öffnungsende"), "m"),
}
meanings = {
    "spacing": n("Distance between neighbouring piloti axes", "Abstand benachbarter Pilotiachsen"),
    "initial_spacing": n("First planned distance between piloti axes, reduced to save cost", "Zuerst geplanter Abstand der Pilotiachsen, aus Kostengründen verkleinert"),
    "LoadBearing": n("Whether the object carries loads", "Ob das Bauteil Lasten trägt"),
    "IsExternal": n("Whether the object faces the outside", "Ob das Bauteil an die Außenluft grenzt"),
    "FireRating": n("Fire resistance class of the object", "Feuerwiderstandsklasse des Bauteils"),
    "material": n("Material the object is made of", "Material, aus dem das Bauteil besteht"),
    "GlobalId": n("Stable identity of the object across exchanges", "Stabile Identität des Objekts über Austausche hinweg"),
    "clear_width": n("Free width between two piloti surfaces", "Freie Breite zwischen zwei Piloti-Oberflächen"),
    "car_width": n("Width of the assumed car of 1929", "Breite des angenommenen Autos von 1929"),
    "car_length": n("Length of the assumed car of 1929", "Länge des angenommenen Autos von 1929"),
    "turning_radius": n("Turning radius of the assumed car of 1929", "Wenderadius des angenommenen Autos von 1929"),
}

translation = {
    "id": "TR-STRUCT", "source_schema": "IFC4X3_ADD2", "target_schema": "IFC4X3_ADD2 Structural Analysis Domain",
    "target_domain": "IfcStructuralAnalysisDomain",
    "mesh": {"x_lines": [0.0, 1.25, 6.0, 10.75, 15.5, 20.25, 21.5], "y_lines": "grid", "surface_source": "SL-1", "node_level": "mid-surface"},
    "boundary": {"id": "fixed", "class": "IfcBoundaryNodeCondition", "en": "fixed (all degrees of freedom restrained)", "de": "eingespannt (alle Freiheitsgrade gebunden)", "basis": "assumed"},
    "mapping": [
        {"source_class": "IfcColumn", "target_class": "IfcStructuralCurveMember", "kind": "curve", "en": "Piloti becomes a vertical line member from the base to the slab mid-surface", "de": "Piloti wird zum vertikalen Stabelement vom Fuß bis zur Plattenmittelfläche"},
        {"source_class": "IfcSlab", "source_predefined": "FLOOR", "target_class": "IfcStructuralSurfaceMember", "kind": "surface", "en": "First-floor slab becomes a surface member meshed into shells on the grid lines", "de": "Geschossdecke wird zum Flächenelement, auf den Rasterlinien in Schalen unterteilt"},
        {"source_class": "IfcColumn", "target_class": "IfcStructuralPointConnection", "kind": "support", "en": "Piloti foot becomes a support node with a boundary condition", "de": "Pilotifuß wird zum Lagerknoten mit Randbedingung"},
    ],
    "assumptions": [
        {"id": "support-fixed", "en": "Every piloti foot is fixed.", "de": "Jeder Pilotifuß ist eingespannt.", "basis": "assumed"},
        {"id": "mid-surface", "en": "The slab is idealised at its mid-surface; the thickness is an attribute of the surface member.", "de": "Die Decke ist an ihrer Mittelfläche idealisiert; die Dicke ist ein Attribut des Flächenelements.", "basis": "assumed"},
        {"id": "mesh-grid", "en": "Shells follow the grid lines and the façade edges: {shells} shells, {slab_nodes} slab nodes and {base_nodes} base nodes.", "de": "Die Schalen folgen den Rasterlinien und Fassadenkanten: {shells} Schalen, {slab_nodes} Deckenknoten und {base_nodes} Fußknoten.", "basis": "assumed"},
        {"id": "load-path", "en": "Roof loads reach the pilotis straight down through an assumed continuous frame and act as point loads at the slab nodes.", "de": "Dachlasten erreichen die Pilotis über einen angenommenen durchgehenden Rahmen und wirken als Punktlasten an den Deckenknoten.", "basis": "assumed"},
        {"id": "material", "en": "All members are reinforced concrete of 25 kN/m³.", "de": "Alle Bauteile sind Stahlbeton mit 25 kN/m³.", "basis": "assumed"},
        {"id": "ramp-void", "en": "The ramp void of the slab is not meshed; the analysis slab is larger than the built one.", "de": "Die Rampenöffnung der Decke ist nicht vernetzt; die Analysedecke ist größer als die gebaute.", "basis": "assumed"},
    ],
    "losses": [
        {"applies_to": {"class": "IfcWindow"}, "property": "element", "en": "Ribbon window: opening and glazing are not modelled", "de": "Bandfenster: Öffnung und Verglasung sind nicht modelliert"},
        {"applies_to": {"class": "IfcOpeningElement"}, "property": "element", "en": "Ribbon opening: void in the wall is not modelled", "de": "Bandfensteröffnung: Aussparung in der Wand ist nicht modelliert"},
        {"applies_to": {"class": "IfcWall", "predefined_type": "STANDARD"}, "property": "element", "en": "Infill wall: carries no load, not part of the analysis", "de": "Ausfachungswand: trägt keine Last, nicht Teil der Berechnung"},
        {"applies_to": {"class": "IfcWall", "predefined_type": "STANDARD"}, "property": "material.layer.render", "en": "Render colour (white) is not kept", "de": "Putzfarbe (weiß) bleibt nicht erhalten"},
        {"applies_to": {"class": "IfcCurtainWall"}, "property": "element", "en": "Curved glass wall: geometry and glazing are not modelled", "de": "Gebogene Glaswand: Geometrie und Verglasung sind nicht modelliert"},
        {"applies_to": {"class": "IfcWall", "predefined_type": "SOLIDWALL"}, "property": "element", "en": "Solarium screen: not part of the analysis", "de": "Solarium-Windschutzwand: nicht Teil der Berechnung"},
        {"applies_to": {"class": "IfcRamp"}, "property": "element", "en": "Ramp: not modelled as a member", "de": "Rampe: nicht als Bauteil modelliert"},
        {"applies_to": {"class": "IfcRampFlight"}, "property": "element", "en": "Ramp flight: not modelled as a member", "de": "Rampenlauf: nicht als Bauteil modelliert"},
        {"applies_to": {"class": "IfcSlab", "predefined_type": "ROOF"}, "property": "element", "en": "Roof slab: replaced by point loads at the pilotis", "de": "Dachdecke: durch Punktlasten an den Pilotis ersetzt"},
        {"applies_to": {"class": "IfcSlab", "predefined_type": "FLOOR"}, "property": "voids", "en": "Ramp void in the slab is not kept", "de": "Rampenöffnung in der Decke bleibt nicht erhalten"},
        {"applies_to": {"class": "IfcColumn"}, "property": "Pset_ColumnCommon.FireRating", "en": "Fire rating is not an analysis property", "de": "Feuerwiderstand ist keine Berechnungseigenschaft"},
        {"applies_to": {"class": "IfcColumn"}, "property": "Pset_ColumnCommon.IsExternal", "en": "External flag is not an analysis property", "de": "Außenbauteil-Kennzeichen ist keine Berechnungseigenschaft"},
    ],
}

doc = {
    "version": 2,
    "languages": ["en", "de"],
    "revision": 1,
    "schema_id": "IFC4X3_ADD2",
    "orientation_note": n("Teaching simplification: the box is aligned with the compass; the entrance with the curved glass wall faces north, the salon faces south.",
                          "Lehrvereinfachung: der Baukörper ist nach den Himmelsrichtungen ausgerichtet; der Eingang mit der gebogenen Glaswand liegt im Norden, der Salon im Süden."),
    "notes": [{"id": i, "text": n(en, de)} for i, en, de in notes],
    "classes": classes_json,
    "property_names": {k: {"name": v[0], "unit": v[1], **({"meaning": meanings[k]} if k in meanings else {})} for k, v in property_names.items()},
    "project": {"guid": gid("PROJECT"), "tag": "PROJECT", "class": "IfcProject", "name": n("Villa Savoye teaching project", "Lehrprojekt Villa Savoye")},
    "site": {"guid": gid("SITE"), "tag": "SITE", "class": "IfcSite", "name": n("Poissy", "Poissy"), "address": "82 rue de Villiers, 78300 Poissy",
             "latitude": q(48.9244, "°", "sourced", "VS-10.7"), "longitude": q(2.0283, "°", "sourced", "VS-10.7")},
    "building": {"guid": gid("BUILDING"), "tag": "BUILDING", "class": "IfcBuilding", "name": n("Villa Savoye", "Villa Savoye"),
                 "architects": ["Le Corbusier", "Pierre Jeanneret"], "clients": ["Pierre Savoye", "Eugénie Savoye"],
                 "years": {"design_start": 1928, "completed": 1931, "listed": 1965, "restoration_start": 1985, "restoration_end": 1997, "unesco": 2016, "basis": "sourced"},
                 "box": {"length": q(21.5, "m", "sourced", "VS-2.1"), "width": q(19.0, "m", "sourced", "VS-2.1"), "height": q(9.4, "m", "sourced", "VS-2.2")}},
    "storeys": storeys,
    "grid": {"x": GX, "y": GY, "labels_x": list(COLS), "labels_y": list(ROWS), "spacing": q(4.75, "m", "sourced", "VS-2.10"),
             "initial_spacing": q(5.0, "m", "sourced", "VS-2.10")},
    "types": types,
    "materials": materials,
    "elements": elements,
    "spaces": spaces,
    "relations": rel_list,
    "models": models,
    "loads": {"finishes": {"first": q(1.5, "kN/m²")},
              "imposed": {"first": q(2.0, "kN/m²"), "roof_planted": q(1.0, "kN/m²"), "solarium": q(2.0, "kN/m²")}},
    "car": {"name": n("Assumed car of 1929", "Angenommenes Auto von 1929"), "length": q(4.6), "width": q(1.75), "turning_radius": q(5.5),
            "wall_clearance": q(0.25), "min_side_margin": q(0.5)},
    "rules": {"clearance": [{"id": "RULE-RAINWATER-PILOTI", "name": n("Rainwater pipe keeps away from a piloti", "Regenfallrohr hält Abstand zur Stütze"),
                             "a_class": "IfcPipeSegment", "a_system": "rainwater", "b_class": "IfcColumn", "required": q(0.25)}],
              "ids": [{"id": "IDS-COLUMN-LOADBEARING", "name": n("Every column states whether it is load-bearing", "Jede Stütze gibt an, ob sie tragend ist"),
                       "entity": "IfcColumn", "pset": "Pset_ColumnCommon", "property": "LoadBearing", "cardinality": "required"}]},
    "demo_events": {
        "sill": {"type": "OpeningResized", "object_guid": gid("RW-S"), "sill": 0.7, "head": 2.0},
        "roof_soil": {"type": "LayerThicknessChanged", "object_guid": gid("SL-R"), "layer": "soil", "value": 0.3, "unit": "m"},
        "roof_soil_alt": {"type": "LayerThicknessChanged", "object_guid": gid("SL-R"), "layer": "soil", "value": 0.2, "unit": "m"},
    },
    "translation": translation,
    "links": [],
}


def dump(value, indent=0):
    pad = "  " * indent
    if isinstance(value, dict):
        if not value:
            return "{}"
        flat = json.dumps(value, ensure_ascii=False)
        if len(flat) <= 100 and not any(isinstance(v, (dict, list)) and len(json.dumps(v, ensure_ascii=False)) > 60 for v in value.values()):
            return flat
        body = ",\n".join(f'{pad}  {json.dumps(k, ensure_ascii=False)}: {dump(v, indent + 1)}' for k, v in value.items())
        return "{\n" + body + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        flat = json.dumps(value, ensure_ascii=False)
        if len(flat) <= 110 or all(not isinstance(v, (dict, list)) for v in value) and len(flat) <= 160:
            return flat
        body = ",\n".join(f"{pad}  {dump(v, indent + 1)}" for v in value)
        return "[\n" + body + "\n" + pad + "]"
    return json.dumps(value, ensure_ascii=False)


target = CLIP / "🧱️model/🔣️.json"
target.write_text(dump(doc) + "\n", encoding="utf-8")
api._source.cache_clear()
model = api.load_model()
analysis_ready = hasattr(api, "link_values")
kinds = [("LK-SLAB", "slab-load", "SL-1"), ("LK-ROOF", "roof-load", "SL-R"), ("LK-PILOTI", "piloti-section", "TY-PILOTI")]
doc["links"] = [{"id": i, "model": "analysis", "kind": k, "object_guid": gid(t), "source_revision": 1,
                 "used": api.link_values(model, k) if analysis_ready else {}} for i, k, t in kinds]
print("[DEBUG] links filled from analysis:", analysis_ready)
target.write_text(dump(doc) + "\n", encoding="utf-8")
print("[DEBUG] written", target.stat().st_size, "bytes;", len(elements), "elements;", len(rel_list), "relations")
