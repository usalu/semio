"""🧱️ Factor an actual primitive entry without fabricating source wrapper entities."""
from pathlib import Path

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry/csg.ts"
source=path.read_text(encoding="utf8")
old=" yield;exact(solid,['IFCCSGSOLID']);const field=(entity:IfcEntity,owner:string,name:string)=>ifcGeometryFieldSteps(graph,entity,owner,name),primitive=graph.ref(yield* field(solid,'IFCCSGSOLID','TreeRootExpression'));exact(primitive,['IFCBLOCK','IFCRIGHTCIRCULARCYLINDER','IFCRIGHTCIRCULARCONE']);"
new=""" yield;exact(solid,['IFCCSGSOLID']);const primitive=graph.ref(yield* ifcGeometryFieldSteps(graph,solid,'IFCCSGSOLID','TreeRootExpression'));
 return yield* ifcCsgPrimitiveSteps(graph,primitive,world,scale,policy,maxValues);
}
/** 🧱️ Borrow the actual declared primitive and current graph for both represented roots and Boolean operands. */
export function* ifcCsgPrimitiveSteps(graph:IfcGeometryGraph,primitive:IfcEntity,world:IfcMatrix,scale:number,policy:IfcCurveSampling,maxValues:number):IfcIoSteps<Pick<IfcTriangleMesh,'positions'|'indices'>>{
 exact(primitive,['IFCBLOCK','IFCRIGHTCIRCULARCYLINDER','IFCRIGHTCIRCULARCONE']);const field=(entity:IfcEntity,owner:string,name:string)=>ifcGeometryFieldSteps(graph,entity,owner,name);"""
assert source.count(old)==1
path.write_text(source.replace(old,new),encoding="utf8")
print("[DEBUG] actual primitive factored; existing root yields/source owner preserved")
