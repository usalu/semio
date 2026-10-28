"""🌳️ Mount genuine-frame Boolean publication and one private request session in the existing extractor."""
from pathlib import Path

ticket=Path(__file__).parent
path=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/geometry.ts"
source=path.read_text(encoding="utf8")
assert "ifcBooleanItem"not in source
anchor='import {ifcRoundedRectangleExtrusionSteps} from "./geometry/rounded.ts";'
assert anchor in source
source=source.replace(anchor,anchor+'\nimport {ifcBooleanItem,IfcBooleanWork} from "./geometry/boolean.ts";',1)
assert source.count('},frame);')==6
source=source.replace('},frame);',"},frame,'represented');")
source=source.replace('const publish=async(mesh:IfcTriangleMesh,frame:IfcMatrix)=>','const publish=async(mesh:IfcTriangleMesh,frame:IfcMatrix,winding:\'represented\'|\'outward\')=>',1)
source=source.replace('if(sign<0){function* reverse()',"if(sign<0&&winding==='represented'){function* reverse()",1)
source=source.replace('ifcRectangleExtrusionSteps(graph,item,frame,scale)','ifcRectangleExtrusionSteps(graph,item,frame,scale,remainingValues)',1)
source=source.replace("'IFCCSGSOLID','IFCFACEBASEDSURFACEMODEL'","'IFCCSGSOLID','IFCBOOLEANRESULT','IFCBOOLEANCLIPPINGRESULT','IFCFACEBASEDSURFACEMODEL'",1)
anchor='  const visitedItems=new Set<bigint>();'
assert anchor in source
source=source.replace(anchor,anchor+'\n  const booleanWork=new IfcBooleanWork(control,captured.booleanEvaluation,captured.curveSampling);\n  try{',1)
anchor="          if(item.name==='IFCCSGSOLID'){"
assert anchor in source
branch="          if(ifcBooleanItem(graph,item)){const result=await booleanWork.evaluate(graph,item,frame,scale,remainingValues,visitedItems);if(result.kind==='empty')diagnostics.push({code:'geometry.empty-boolean',severity:'warning',message:'The regularized Boolean expression is explicitly empty',entityIds:[product.id,item.id]});else await publish({...result.mesh,entityId:item.id,productId:product.id,name:ifcText(product.args[2])},frame,'outward');continue;}\n"
source=source.replace(anchor,branch+anchor,1)
anchor='  return {artifact:meshes,fidelity:diagnostics.length?"partial":"geometry",diagnostics};\n}'
assert anchor in source
source=source.replace(anchor,'  return {artifact:meshes,fidelity:diagnostics.length?"partial":"geometry",diagnostics};\n  }finally{booleanWork.close();}\n}',1)
path.write_text(source,encoding="utf8")
print("[DEBUG] mounted one request Boolean owner and actual outward provenance; rectangle grant supplied")
