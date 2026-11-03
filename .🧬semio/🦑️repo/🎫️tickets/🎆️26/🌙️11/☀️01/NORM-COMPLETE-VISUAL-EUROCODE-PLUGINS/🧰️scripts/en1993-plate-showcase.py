"""Places the worked-example rows of `jointPlates` and `foundationBlocks` in the EN 1993 special-parts showcase (Rust `special_parts_showcase()`) and in the committed showcase fixture: a fin plate on joint-j1, a new splice joint joint-sp with flange and web cover plates, and the foundation block under joint-bp. Idempotent; no generator output is edited by hand. Usage: python3 -I en1993-plate-showcase.py"""
import json, pathlib, re

A = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any")
RS = A / "🧬️schema/📸️snapshot/🦀️.rs"
SHOW = A / "🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json"

PLATES = [
    dict(id="plate-fin", jointId="joint-j1", label="Fin plate", role="finPlate", plateHeight=0.24, plateWidth=0.1, plateThickness=0.01, plateCount=1, plateFy=355e6, plateFu=470e6, boltClass="8.8", boltDiameter=0.02, holeDiameter=0.022, boltRows=3, boltsPerRow=1, pitch=0.07, gauge=0.0, endDistance=0.04, edgeDistance=0.04, shearPlanes=1, eccentricity=0.06, weldThroat=0.005, weldFu=470e6, weldBetaW=0.9),
    dict(id="plate-flange", jointId="joint-sp", label="Flange cover plate", role="flangeCover", plateHeight=0.22, plateWidth=0.2, plateThickness=0.014, plateCount=1, plateFy=355e6, plateFu=470e6, boltClass="8.8", boltDiameter=0.02, holeDiameter=0.022, boltRows=3, boltsPerRow=2, pitch=0.07, gauge=0.1, endDistance=0.05, edgeDistance=0.05, shearPlanes=1, eccentricity=0.0, weldThroat=0.0, weldFu=0.0, weldBetaW=0.0),
    dict(id="plate-web", jointId="joint-sp", label="Web cover plates", role="webCover", plateHeight=0.18, plateWidth=0.1, plateThickness=0.008, plateCount=2, plateFy=355e6, plateFu=470e6, boltClass="8.8", boltDiameter=0.02, holeDiameter=0.022, boltRows=2, boltsPerRow=1, pitch=0.07, gauge=0.0, endDistance=0.04, edgeDistance=0.04, shearPlanes=2, eccentricity=0.05, weldThroat=0.0, weldFu=0.0, weldBetaW=0.0),
]
BLOCK = dict(id="block-bp", jointId="joint-bp", label="Foundation block", blockLength=0.9, blockWidth=0.9, blockDepth=0.8, concreteFck=30e6, groutFck=40e6, groutThickness=0.03, anchorCount=2, anchorClass="8.8", anchorDiameter=0.024, anchorStressArea=0.000353, anchorEmbedment=0.4, anchorEdgeDistance=0.2)
snake = lambda s: re.sub(r"([A-Z])", lambda m: "_" + m.group(1).lower(), s)


def f(v):
    return repr(float(v))


def plate_rs(p):
    parts = []
    for k, v in p.items():
        if isinstance(v, str):
            parts.append(f'{snake(k)}: {json.dumps(v)}.into()')
        elif isinstance(v, int):
            parts.append(f"{snake(k)}: {v}")
        else:
            parts.append(f"{snake(k)}: {f(v)}")
    return ", ".join(parts)


def rust():
    s = RS.read_text(encoding="utf8")
    if '"plate-fin"' in s:
        return
    s = re.sub(r"        doc\.joint_plates\.push\(JointPlate \{[^\n]*\n        doc\.foundation_blocks\.push\(FoundationBlock \{[^\n]*\n", "", s, count=1)
    rows = "".join(f"        doc.joint_plates.push(JointPlate {{ {plate_rs(p)} }});\n" for p in PLATES)
    rows += f"        doc.foundation_blocks.push(FoundationBlock {{ {plate_rs(BLOCK)} }});\n"
    splice = '''        let mut splice = template.clone();
        splice.id = "joint-sp".into();
        splice.moment_demand = 80_000.0;
        splice.actions = vec![JointForceAction { id: "jf-sp".into(), load_case_id: "g-permanent".into(), shear: 30_000.0, tension: 0.0 }];
        doc.joints.push(splice);
'''
    anchor = "        doc.joints.push(base_plate);\n"
    assert anchor in s
    s = s.replace(anchor, anchor + splice + rows, 1)
    RS.write_text(s, encoding="utf8")


def fixture():
    d = json.loads(SHOW.read_text(encoding="utf8"))
    if any(r["id"] == "plate-fin" for r in d["jointPlates"]):
        return
    j = json.loads(json.dumps(next(r for r in d["joints"] if r["id"] == "joint-j1")))
    j.update(id="joint-sp", momentDemand=80000.0, actions=[{"id": "jf-sp", "loadCaseId": "g-permanent", "shear": 30000.0, "tension": 0.0}])
    d["joints"].append(j)
    d["jointPlates"] = PLATES
    d["foundationBlocks"] = [BLOCK]
    SHOW.write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n", encoding="utf8")


rust()
fixture()
print("ok")
