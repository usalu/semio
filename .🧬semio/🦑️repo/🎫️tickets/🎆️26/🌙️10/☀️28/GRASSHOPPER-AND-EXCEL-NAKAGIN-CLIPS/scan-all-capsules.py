from pathlib import Path
import json,math,hashlib
ROOT=Path(__file__).resolve().parent
exec((ROOT/'scan-reference.py').read_text(encoding='utf8').split("view=data['bufferViews'][data['images'][0]['bufferView']]")[0])
modelpath=Path('🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json')
model=json.loads(modelpath.read_text(encoding='utf8'))
pitch=.05965845070422535
rows=[]
for tower in model['towers']:
 for floor in tower['floors']:
  for group in floor['groups']:
   for item in group['items']:
    x,y,z=[q['value'] for q in item['center']]
    ang=math.radians(item['rotation']['value']);c,s=math.cos(ang),math.sin(ang)
    mx=-46.25*(centres[:,0]+.12);my=46.25*centres[:,2]
    lx=(mx-x)*c+(my-y)*s;ly=-(mx-x)*s+(my-y)*c
    inside=(abs(lx)<1.10)&(abs(ly)<1.84)
    base=.2186 if tower['key']=='A' else .2182
    levels=[]
    for phase in range(3):
     roof=base+(floor['floor']-3+phase/3)*pitch
     up=inside&(unit[:,1]>.90)&(abs(centres[:,1]-roof)<.005)
     down=inside&(unit[:,1]<-.90)&(abs(centres[:,1]-(roof-.058))<.005)
     levels.append({'phase':phase,'roof_raw':roof,'roof_area':float(areas[up].sum()),'underside_area':float(areas[down].sum())})
    phase=item['sublevel'];own=levels[phase];best=max(levels,key=lambda q:q['roof_area']+q['underside_area'])
    score=own['roof_area']+own['underside_area']
    conflict=score<.00012 and best['phase']!=phase and best['roof_area']+best['underside_area']>.0007
    fam=next(f for f in model['families'] if f['key']==item['family'])
    roof=own['roof_raw']
    vertical=(abs(unit[:,1])<.2)&(centres[:,1]>roof-.048)&(centres[:,1]<roof-.009)
    # Model local coordinates; scanner reflection is included in outward normal.
    if fam['window_wall']=='front':
     face=vertical&(abs(ly-2)<.24)&(abs(lx)<1.10)
     normal=unit[:,0]*s+unit[:,2]*c
    else:
     face=vertical&(abs(lx-1.25)<.24)&(abs(ly)<1.84)
     normal=-unit[:,0]*c+unit[:,2]*s
    face &= normal>.65
    rows.append({'tower':tower['key'],'floor':floor['floor'],'facade':group['facade'],'item':item['item'],'family':item['family'],'phase':phase,'center':[x,y,z],'rotation':item['rotation']['value'],'positive_surface_area':score,'status':'phase_conflict_candidate' if conflict else ('positive_roof_or_underside' if score>.00025 else 'occluded_or_sparse'),'phase_scores':levels,'window_wall':fam['window_wall'],'expected_window_face_material_area':float(areas[face].sum()),'aperture_orientation_status':'facade_material_present_not_aperture_fit' if areas[face].sum()>.00015 else 'occluded_or_sparse'})
out={'model_sha256':hashlib.sha256(modelpath.read_bytes()).hexdigest(),'registration':'xscan=-xmodel/46.25-.12,zscan=ymodel/46.25; horizontal scale inferred, phase pitch independently measured; signed areas raw squared','limitations':'Centroid ROI shrunk from nominal rectangle. Scores are positive material evidence, not absence proof or independent metric survey. Family door handing and circular aperture not certified. Phase alternative scores can include adjacent shells; candidates require manual review.','rows':rows}
(OUT/'gluon-all-capsule-coverage.json').write_text(json.dumps(out,indent=2))
for t in ('A','B'):
 for f in sorted(set(r['floor'] for r in rows if r['tower']==t)):
  rr=[r for r in rows if r['tower']==t and r['floor']==f]
  print(t,f,len(rr),{k:sum(r['status']==k for r in rr) for k in sorted(set(r['status']for r in rr))})
for r in rows:
 if r['status']=='phase_conflict_candidate':print('CONFLICT',r)
