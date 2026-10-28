"""🛁️ Observe the unchanged official Boolean bath through a test-only geometry interface."""
from pathlib import Path
import hashlib,json,math
import ifcopenshell
import ifcopenshell.geom
import ifcopenshell.validate

class GeometryReference:
 """🔬️ External source/geometry details terminate at owned plain records."""
 def observe(self,path):
  model=ifcopenshell.open(str(path))
  logger=ifcopenshell.validate.json_logger()
  ifcopenshell.validate.validate(model,logger,express_rules=True)
  settings=ifcopenshell.geom.settings()
  settings.set(settings.USE_WORLD_COORDS,True)
  product=model.by_id(225)
  owner=ifcopenshell.geom.create_shape(settings,product)
  flat=list(owner.geometry.verts)
  points=[flat[i:i+3]for i in range(0,len(flat),3)]
  indices=list(owner.geometry.faces)
  origin=points[0]
  terms=[]
  for at in range(0,len(indices),3):
   a,b,c=[[points[index][axis]-origin[axis]for axis in range(3)]for index in indices[at:at+3]]
   terms.append((a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6)
  return {"library":ifcopenshell.version,"header":list(model.header.file_schema.schema_identifiers),"product":{"id":str(product.id()),"type":product.is_a()},"diagnostics":[{"level":row.get("level"),"attribute":row.get("attribute"),"message":str(row.get("message"))}for row in logger.statements],"mesh":{"positions":points,"indices":indices,"vertices":len(points),"triangles":len(indices)//3,"signedVolume":math.fsum(terms),"bounds":[[min(point[axis]for point in points)for axis in range(3)],[max(point[axis]for point in points)for axis in range(3)]]}}

ticket=Path(__file__).parent
source=ticket.parents[6]/"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/bath-csg-solid.ifc"
digest=hashlib.sha256(source.read_bytes()).hexdigest()
assert digest=="4592f3029faa12625494ada734fcfe46e4fc91a33d001198e9e3144721870b95"
value=GeometryReference().observe(source)
value.update(source=str(source),sourceSha256=digest,idealRoundedCutterArea=1.8*.6-(4-math.pi)*.2**2,idealRemainingVolume=1.28-.7*(1.8*.6-(4-math.pi)*.2**2),ourProducerExecuted=False,nativeExecuted=False)
value["externalMinusIdealVolume"]=value["mesh"]["signedVolume"]-value["idealRemainingVolume"]
(ticket/"🗑️generated/official-bath-geometry-reference.json").write_text(json.dumps(value,ensure_ascii=False,indent=2)+"\n",encoding="utf8")
(ticket/"official-bath-geometry-reference.md").write_text("# Original Official Boolean Bath Geometry Reference\n\nUnchanged source SHA-256 "+digest+" is observed through actual IfcOpenShell"+value["library"]+" with world coordinates enabled. Product"+value["product"]["id"]+"/"+value["product"]["type"]+" produces "+str(value["mesh"]["vertices"])+"vertices/"+str(value["mesh"]["triangles"])+"triangles, signed volume"+repr(value["mesh"]["signedVolume"])+" and bounds"+repr(value["mesh"]["bounds"])+". Actual full-EXP diagnostics:"+str(len(value["diagnostics"]))+"; raw messages/mesh/header are retained under generated, not suppressed.\n\nThe independent ideal rounded-cutter area is1.8*.6-(4-pi)*.2²; ideal remaining volume="+repr(value["idealRemainingVolume"])+". Observed external tessellation minus ideal="+repr(value["externalMinusIdealVolume"])+". This records an actual external reference; different finite chord policies need their own certified approximation bounds rather than exact equal-volume assumptions.\n\nOur actual producer remains unsupported for this genuine BooleanResult(Block, rounded-rectangle ExtrudedAreaSolid) original. The first authored Block-only matrix is not substituted for this official result. No runtime SDK dependency, actual IFC producer output, GLB validation or native proof follows from this test-only reference. Source bytes remain unchanged.\n",encoding="utf8")
assert hashlib.sha256(source.read_bytes()).hexdigest()==digest
print("[DEBUG] "+json.dumps({"diagnostics":len(value["diagnostics"]),"vertices":value["mesh"]["vertices"],"triangles":value["mesh"]["triangles"],"volume":value["mesh"]["signedVolume"],"ideal":value["idealRemainingVolume"]}))
