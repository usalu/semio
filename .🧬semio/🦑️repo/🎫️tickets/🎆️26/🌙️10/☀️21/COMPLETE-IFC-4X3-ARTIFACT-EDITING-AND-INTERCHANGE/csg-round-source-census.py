"""📌️ Freeze actual circular CSG producer, consumers and selected reviewed dependencies without copying source."""
from pathlib import Path
import hashlib,json
ticket=Path(__file__).parent
root=ticket.parents[6]
artifact=root/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc"
io=artifact/"🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io"
owned=[io/name for name in ["geometry.ts","geometry/csg.ts","geometry/circle.ts","geometry/circular-solid.ts"]]
owned += [io/"🧪️tests"/name for name in ["csg-round-fixtures.ts","csg-round.test.ts","csg-round-cases.json","csg-round.schema.json","csg-round.feature","csg-round-supplemental-cases.json","csg-round-supplemental.schema.json","circular-solid-orientation-cases.json","circular-solid-orientation.schema.json","circle-extrusion-fixtures.ts","csg-block.test.ts","mapped-circle.test.ts"]]
refs=[io/name for name in ["control.ts","geometry/source.ts","geometry/control.ts","orientation.ts","step.ts","step-values.ts","gltf.ts","gltf-mesh.ts","exchange-source.ts","geometry/mapping.ts"]]
refs += [io/"🧪️tests"/name for name in ["csg-block-fixtures.ts","csg-block-cases.json","csg-block.schema.json","csg-block.feature","circle-extrusion.test.ts","circle-extrusion-cases.json","circle-extrusion.schema.json","mapped-circle-cases.json","mapped-circle.schema.json","rectangle-extrusion-fixtures.ts","mapping-fixtures.ts","mapping-cases.json","geometry-progress.test.ts","mapping.test.ts","rectangle-control.test.ts","gltf-placement.test.ts","gltf-normals.test.ts"]]
refs += [io.parent/"🧪️fixtures/official"/name for name in ["csg-primitive.ifc","slab-openings.ifc"]]
refs += [artifact/name for name in ["📐️geometry/⭕️circle/🟦️.ts","📐️geometry/⭕️circle/🦀️.rs","📐️geometry/🧊️boolean/🟦️.ts"]]
def row(path):
 return {"path":str(path),"sha256":hashlib.sha256(path.read_bytes()).hexdigest(),"utf16PathUnits":len(str(path).encode("utf-16-le"))//2}
result={"owned":list(map(row,owned)),"references":list(map(row,refs)),"nativeExecuted":False,"scope":"Exact current CSG Cylinder/Cone source roles and shared finite Circle geometry, central reflection, canonical diffs/inverses, STEP/GLB, original controls and existing Circle/Block observations. Selected dependencies only; not complete IFC/MVD/native/semantic fidelity."}
assert all(x["utf16PathUnits"]<=256 for x in result["owned"]+result["references"])
(ticket/"🗑️generated/csg-round-source.json").write_text(json.dumps(result,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
print("[DEBUG]",json.dumps({"owned":len(owned),"references":len(refs),"maxPath":max(x["utf16PathUnits"]for x in result["owned"]+result["references"])}))
