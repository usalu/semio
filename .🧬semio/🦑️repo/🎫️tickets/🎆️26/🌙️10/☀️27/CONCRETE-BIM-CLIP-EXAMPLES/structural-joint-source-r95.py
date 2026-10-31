"""🔗 Declare only independently confirmed native same-material construction ownership."""
from pathlib import Path
import json


def apply(model,ramps=False):
    """📝 Preserve all raw geometry, sections, identities and prior declared interface relationships."""
    case=json.loads(Path(__file__).with_name("structural-joint-source-cases-r95.json").read_text(encoding="utf-8"))
    registry={o["tag"]:o for o in model["elements"]}
    for row in case["relationships"]:
        joints=registry[row["donor"]]["geometry"].setdefault("structural_joints",[])
        value={"owner":row["owner"],"relationship":case["relationship"]}
        if value not in joints:joints.append(value)
    for row in case.get("column_reservations",()):
        owners=registry[row["donor"]]["geometry"].setdefault("column_reserves",[])
        for owner in row["owners"]:
            if owner not in owners:owners.append(owner)
    for row in case.get("receiving_interfaces",()):
        owners=registry[row["donor"]]["geometry"].setdefault("interface_owners",[])
        for owner in row["owners"]:
            if owner not in owners:owners.append(owner)
    evidence={}
    for row in case["relationships"]+case.get("column_reservations",[])+case.get("receiving_interfaces",[]):evidence.setdefault(row["donor"],[]).append(row["reason"])
    for tag,reasons in evidence.items():
        registry[tag].setdefault("psets",{}).setdefault("Pset_ConstructionInterface",{})["ExclusiveOwnership"]={"value":"Inferred construction ownership; original raw dimensions, parts and GUIDs retained. "+" ".join(reasons),"ifc_type":"IfcText","basis":"inferred","source":"CONCRETE-BIM-CLIP-EXAMPLES structural-joint source contracts"}
    if ramps:
        for row in case["ramp_interfaces"]:
            owners=registry[row["donor"]]["geometry"].setdefault("interface_owners",[])
            if row["owner"] not in owners:owners.append(row["owner"])
            registry[row["donor"]].setdefault("psets",{}).setdefault("Pset_ConstructionInterface",{})["ExclusiveOwnership"]={"value":"Inferred exact receiving-member ownership; retain original ramp section and datums as reference. "+row.get("reason","The primary structural beam and its existing true physical slab/column owners retain the shared RC bearing material."),"ifc_type":"IfcText","basis":"inferred","source":"CONCRETE-BIM-CLIP-EXAMPLES structural-joint source contracts"}
    return case
