"""🔬 Classify capital frame pixels and changed source IDs with independent physical triangle rays."""
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
from collections import Counter
import numpy as np
from PIL import Image
TICKET=Path(__file__).resolve().parent
OLD=TICKET/'🗑️generated/capital-order-source'
NEW=TICKET/'🗑️generated/capital-after-dependency-order'
OUTPUT=TICKET/'🗑️generated/capital-ray-classification.json'
SPEC=importlib.util.spec_from_file_location('capital_ray_oracle',TICKET/'pixel-nearest-surface.py')
M=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=M
SPEC.loader.exec_module(M)
FILES=[path for folder in (OLD,NEW) for path in folder.iterdir() if path.is_file()]+[Path(__file__),TICKET/'pixel-nearest-surface.py',M.ORACLE]
def hashes():
    """🔐 Preserve both exact source, camera, raster and independent oracle endpoints."""
    return {str(path.relative_to(TICKET)) if path.is_relative_to(TICKET) else str(path):hashlib.sha256(path.read_bytes()).hexdigest() for path in FILES}
before=hashes()
items={name:json.loads((folder/'coverage-items.json').read_text(encoding='utf-8')) for name,folder in (('old',OLD),('current',NEW))}
cameras={name:json.loads((folder/'coverage-camera.json').read_text(encoding='utf-8')) for name,folder in (('old',OLD),('current',NEW))}
assert cameras['old']==cameras['current']
for one,two in zip(items['old'],items['current'],strict=True):
    assert all(one[key]==two[key] for key in ('index','key','role','world_vertices','pixel_polygons','normal','opacity','removed'))
resolution=cameras['current']['pixel_width']
images={}
pixels=set()
steel_pixels={}
changed={}
for name,folder in (('old',OLD),('current',NEW)):
    steel={row['index'] for row in items[name] if row['role']=='steel_truss'}
    images[name]={stage:np.asarray(Image.open(folder/(str(resolution)+'-ids-'+stage+'.png')).convert('RGB')).astype(np.int32)@np.array([65536,256,1])-1 for stage in ('full','stable','culled')}
    selected=set()
    for image in images[name].values():
        ys,xs=np.where(np.isin(image,list(steel)))
        selected.update(zip(map(int,xs),map(int,ys)))
    steel_pixels[name]=selected
    pixels.update(selected)
    ys,xs=np.where(images[name]['full']!=images[name]['culled'])
    changed[name]=set(zip(map(int,xs),map(int,ys)))
    pixels.update(changed[name])
scene=M.SurfaceScene(items['current'],cameras['current']['towards'])
old_ranks={row['index']:(row['full_rank'],row['new_rank']) for row in items['old']}
rows=[]
counts=Counter()
print('[DEBUG] Capital independent source '+str(len(scene.items))+'; ROI '+str(len(pixels))+' pixels',flush=True)
for index,(x,y) in enumerate(sorted(pixels)):
    if index%25==0:
        print('[DEBUG] Capital rays '+str(index)+'/'+str(len(pixels)),flush=True)
    points=M.sample_grid(x,y,5)
    intersections=scene.intersections(points)
    samples=[]
    for point,hits in zip(points,intersections):
        physical=scene.contributors(hits)
        depth_by_owner={}
        for triangle in np.flatnonzero(np.isfinite(hits)):
            owner=int(scene.owners[triangle])
            depth_by_owner[owner]=max(depth_by_owner.get(owner,-np.inf),float(hits[triangle]))
        current=scene.painter(hits,'culled')
        retained=[owner for owner in depth_by_owner if not scene.items[owner]['removed']]
        old=sorted(retained,key=lambda owner:old_ranks[owner][1] if old_ranks[owner][1] is not None else -1,reverse=True)
        nearest=physical['nearest_ids']
        painters={name:values[0] if values else None for name,values in (('old',old),('current',current))}
        bad={name:owner is not None and owner not in nearest for name,owner in painters.items()}
        counts.update({name+'_wrong_samples':int(value) for name,value in bad.items()})
        steel_hits=[owner for owner in depth_by_owner if scene.items[owner]['role']=='steel_truss']
        front=[owner for owner in nearest if scene.items[owner]['role']=='steel_truss']
        counts['nearest_steel_samples']+=bool(front)
        blockers={str(owner):[other for other in depth_by_owner if scene.items[other]['opacity']>=1 and depth_by_owner[other]>depth_by_owner[owner]+1e-8] for owner in steel_hits}
        samples.append({'point':point.tolist(),'nearest_ids':nearest,'nearest_roles':[scene.items[owner]['role'] for owner in nearest],'nearest_depth':physical['nearest_depth'],'painter':painters,'painter_wrong':bad,'painter_depth_deficit':{name:None if owner is None or physical['nearest_depth'] is None else physical['nearest_depth']-depth_by_owner[owner] for name,owner in painters.items()},'steel_hits':steel_hits,'opaque_in_front_of_steel':blockers})
    centre=next(row for row in samples if np.allclose(row['point'],[x+.5,y+.5],atol=1e-12,rtol=0))
    cairo={name:{stage:int(image[y,x]) for stage,image in stages.items()} for name,stages in images.items()}
    centre_errors={name:{stage:owner not in centre['nearest_ids'] if owner>=0 else bool(centre['nearest_ids']) for stage,owner in stages.items()} for name,stages in cairo.items()}
    for name,stages in centre_errors.items():
        counts[name+'_cairo_wrong_centres']+=stages['culled']
    rows.append({'pixel':[x,y],'old_visible_steel':(x,y) in steel_pixels['old'],'current_visible_steel':(x,y) in steel_pixels['current'],'old_changed_id':(x,y) in changed['old'],'current_changed_id':(x,y) in changed['current'],'cairo':cairo,'cairo_wrong_centres':centre_errors,'samples':samples})
after=hashes()
report={'source_before':before,'source_after':after,'unchanged':before==after,'source_items':len(scene.items),'retained_items':sum(not row['removed'] for row in items['current']),'triangles':len(scene.triangles),'projection_residual':scene.fit_residual,'pixels':len(rows),'rays':len(rows)*25,'old_steel_pixels':len(steel_pixels['old']),'current_steel_pixels':len(steel_pixels['current']),'old_changed_pixels':len(changed['old']),'current_changed_pixels':len(changed['current']),'counts':dict(counts),'rows':rows,'limits':'Finite centre plus 5x5 source rays in the union of actual steel source-ID and changed-ID pixel ROIs. No global raster, AA or universal aperture claim.'}
OUTPUT.write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps({key:value for key,value in report.items() if key not in ('source_before','source_after','rows')}),flush=True)
raise SystemExit(0 if report['unchanged'] else 1)
