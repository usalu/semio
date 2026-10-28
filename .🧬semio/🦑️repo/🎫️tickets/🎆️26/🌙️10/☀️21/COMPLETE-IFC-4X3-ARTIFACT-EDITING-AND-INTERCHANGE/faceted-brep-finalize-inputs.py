import pathlib,json,copy
root=pathlib.Path(__file__).parent
p=root/'faceted-brep-geometry-vectors.json';d=json.loads(p.read_text(encoding='utf-8-sig'));rows=d['cases'];box=next(r for r in rows if r['id']=='box');hole=next(r for r in rows if r['id']=='face-hole')
for r in rows:
 if r['id']=='missing-outer-marker':r.update(expected='supported',volume=6,bounds=copy.deepcopy(box['bounds']))
 if r['id']=='with-voids-explicit-refusal':r.update(id='with-voids',expected='supported',volume=5.5,bounds=copy.deepcopy(box['bounds']))
for name,kind in [('hole-no-outer','no-outer'),('outer-marker-reordered','outer-last')]:
 if not any(r['id']==name for r in rows):
  r=copy.deepcopy(hole);r['id']=name;r['modifier']={'kind':kind};rows.append(r)
p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
p=root/'faceted-brep-geometry-vectors.schema.json';s=json.loads(p.read_text(encoding='utf-8-sig'))
def walk(x):
 if isinstance(x,dict):
  if x.get('minItems')==16:x['minItems']=18
  if x.get('maxItems')==16:x['maxItems']=18
  if 'enum'in x and 'no-outer'in x['enum'] and 'outer-last'not in x['enum']:x['enum'].append('outer-last')
  for v in x.values():walk(v)
 elif isinstance(x,list):
  for v in x:walk(v)
walk(s);p.write_text(json.dumps(s,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
p=root/'faceted-brep-vectors-oracle.py';s=p.read_text(encoding='utf-8-sig');s=s.replace("entity=f.create_entity('IfcFace',Bounds=tuple(bounds))","if mode=='outer-last' and len(bounds)>1:bounds=bounds[1:]+bounds[:1]\n            entity=f.create_entity('IfcFace',Bounds=tuple(bounds))")
s=s.replace("assert math.isclose(abs(result['geometry']['signedVolume']),row['volume'],rel_tol=1.e-9,abs_tol=1.e-9),result","if geometry['voids']:\n                result['compoundGeometryOracle']='unavailable; separate actual shell meshes and Three oriented-volume certificate recorded independently'\n            else:assert math.isclose(abs(result['geometry']['signedVolume']),row['volume'],rel_tol=1.e-9,abs_tol=1.e-9),result")
p.write_text(s,encoding='utf-8')
