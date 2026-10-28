import pathlib,json,ifcopenshell,ifcopenshell.validate
p=pathlib.Path(__file__).resolve().parent;r=p.parents[6]
fpath=r/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/triangulated-item.ifc'
data=json.loads((p/'tfs-admission-cases.json').read_text());results=[]
for case in data['cases']:
 s=case['source'];f=ifcopenshell.open(str(fpath));e=f.by_id(1021)
 try:
  if s['coordinateRole']!='IfcCartesianPointList3D':
   points=f.create_entity('IfcCartesianPointList2D',CoordList=[(0.,0.),(1.,0.),(0.,1.)])
  else:points=f.create_entity('IfcCartesianPointList3D',CoordList=[tuple(float(x) for x in a) for a in s['points']])
  e.Coordinates=points
  e.CoordIndex=[tuple(float(x) if s['indexKind']=='real' else x for x in a) for a in s['triangles']]
  e.PnIndex=None if s['pnIndex'] is None else [float(x) if s['pnKind']=='real' else x for x in s['pnIndex']]
  e.Closed='UNKNOWN' if s['closedKind']=='logical' else s['closed']
  e.Normals=None if s['normals'] is None else [tuple(float(x) for x in a) for a in s['normals']]
  log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True)
  results.append({'id':case['id'],'construction':'success','formalDiagnostics':[{'attribute':x.get('attribute'),'message':x.get('message'),'instance':str(x.get('instance'))} for x in log.statements]})
 except Exception as ex:results.append({'id':case['id'],'construction':'refused','exception':str(ex)})
(p/'🗑️generated/tfs-admission/neutral-formal.json').write_text(json.dumps({'version':ifcopenshell.version,'cases':results},indent=2),encoding='utf8')
print(json.dumps({'count':len(results),'constructed':sum(x['construction']=='success' for x in results),'withFormalDiagnostics':sum(bool(x.get('formalDiagnostics')) for x in results)}))
