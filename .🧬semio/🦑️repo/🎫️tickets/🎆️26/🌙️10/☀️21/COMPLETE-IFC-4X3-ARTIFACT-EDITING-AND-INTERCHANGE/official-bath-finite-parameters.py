"""🛁 Extract actual official parameter records into an owned test-only contract."""
from pathlib import Path
import json,hashlib
import ifcopenshell,ifcopenshell.util.placement as placement,ifcopenshell.util.unit as unit
p=Path(__file__).parent;repo=next(a for a in p.parents if (a/'AGENTS.md').exists());source=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/bath-csg-solid.ifc';raw=source.read_bytes();digest=hashlib.sha256(raw).hexdigest();assert digest=='4592f3029faa12625494ada734fcfe46e4fc91a33d001198e9e3144721870b95';f=ifcopenshell.open(str(source));block=f.by_id(200);profile=f.by_id(202);solid=f.by_id(206);mapped=f.by_id(222);product=f.by_id(225)
def matrix(entity):return placement.get_axis2placement(entity).tolist() if entity else [[1.,0.,0.,0.],[0.,1.,0.,0.],[0.,0.,1.,0.],[0.,0.,0.,1.]]
data={'schema':'IFC4X3_ADD2','scope':'official-bath-independent-finite-policy','producerExecuted':False,'sourceSha256':digest,'productId':str(product.id()),'operator':f.by_id(207).Operator,'unitScaleMetres':unit.calculate_unit_scale(f),'block':{'id':str(block.id()),'dimensions':[block.XLength,block.YLength,block.ZLength],'position':matrix(block.Position)},'cutter':{'id':str(solid.id()),'profileId':str(profile.id()),'dimensions':[profile.XDim,profile.YDim],'radius':profile.RoundingRadius,'depth':solid.Depth,'direction':list(solid.ExtrudedDirection.DirectionRatios),'profilePosition':matrix(profile.Position),'solidPosition':matrix(solid.Position)},'mapping':{'id':str(mapped.id()),'origin':matrix(mapped.MappingSource.MappingOrigin),'target':placement.get_cartesiantransformationoperator3d(mapped.MappingTarget).tolist(),'composed':placement.get_mappeditem_transformation(mapped).tolist()},'productPlacement':placement.get_local_placement(product.ObjectPlacement).tolist(),'policy':{'curvedChords':72,'totalBoundary':76,'relativeChordError':.001,'absoluteChordErrorMetres':0,'maxSegments':4096},'references':{'idealVolume':.5480354056994858,'sdkVolume':.5482863988281255,'sdkVertices':112,'sdkTriangles':220}}
(p/'official-bath-finite-contract.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf8')
def schema(value):
 if isinstance(value,dict):return {'type':'object','additionalProperties':False,'required':list(value),'properties':{k:schema(v)for k,v in value.items()}}
 if isinstance(value,list):return {'type':'array','minItems':len(value),'maxItems':len(value),'items':schema(value[0])}
 if isinstance(value,bool):return {'const':value}
 if isinstance(value,str):return {'const':value}
 return {'type':'number'}
s={'$schema':'https://json-schema.org/draft/2020-12/schema',**schema(data)};(p/'official-bath-finite-contract.schema.json').write_text(json.dumps(s,indent=2)+'\n',encoding='utf8');assert hashlib.sha256(source.read_bytes()).hexdigest()==digest;print('[DEBUG]',digest,data['unitScaleMetres'],data['mapping']['composed'])
