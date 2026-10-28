import pathlib,json,hashlib,urllib.request,re,sys
root=pathlib.Path(__file__).resolve().parents[7]
ticket=pathlib.Path(__file__).resolve().parent
out=ticket/'🗑️generated'/'tfs-admission';out.mkdir(exist_ok=True)
import ifcopenshell,ifcopenshell.validate
r=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any'
result={'ifcopenshell':ifcopenshell.version,'primary':[],'originals':[]}
for name in ['IfcTriangulatedFaceSet','IfcCartesianPointList3D']:
 url=f'https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/lexical/{name}.htm'
 try:
  data=urllib.request.urlopen(url).read();(out/(name+'.htm')).write_bytes(data)
  result['primary'].append({'name':name,'url':url,'sha256':hashlib.sha256(data).hexdigest()})
 except Exception as e:result['primary'].append({'name':name,'error':str(e)})
for name in ['triangulated-item.ifc','slab-tessellated-unique-vertices.ifc','beam-straight-i-shape-tessellated.ifc','beam-curved-i-shape-tessellated.ifc']:
 p=r/'🧪️fixtures/official'/name;data=p.read_bytes();f=ifcopenshell.open(str(p));rows=[]
 for e in f.by_type('IfcTriangulatedFaceSet'):
  rows.append({'id':e.id(),'type':e.is_a(),'coordinates':e.Coordinates.id(),'vertices':len(e.Coordinates.CoordList),'triangles':len(e.CoordIndex),'closed':e.Closed,'normals':None if e.Normals is None else len(e.Normals),'pnIndex':None if e.PnIndex is None else list(e.PnIndex),'coordIndexMax':max(max(x) for x in e.CoordIndex),'firstCoord':list(e.Coordinates.CoordList[0])})
 log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True)
 result['originals'].append({'name':name,'sha256':hashlib.sha256(data).hexdigest(),'header':list(f.header.file_schema.schema_identifiers),'entities':len(list(f)),'tfs':rows,'expressDiagnostics':[{'attribute':x.get('attribute'),'message':x.get('message'),'instance':str(x.get('instance'))} for x in log.statements]})
(out/'originals.json').write_text(json.dumps(result,indent=2),encoding='utf8')
print(json.dumps({'version':result['ifcopenshell'],'primary':len(result['primary']),'originals':[(x['name'],len(x['tfs']),len(x['expressDiagnostics'])) for x in result['originals']]}))

