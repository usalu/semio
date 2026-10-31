"""🔬 Attribute all final-source AA controls with one independent exact-source ray scene."""
from collections import Counter
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import numpy as np
from PIL import Image

sys.stdout.reconfigure(encoding='utf8',errors='backslashreplace')
TICKET=Path(__file__).parent
parser=argparse.ArgumentParser()
parser.add_argument('--input-dir',type=Path,required=True)
parser.add_argument('--output',type=Path,required=True)
parser.add_argument('--attestation',type=Path)
parser.add_argument('--modes',default='geometric,silhouette,ids,aa-fill,aa-strokes,material')
args=parser.parse_args()
D=args.input_dir.resolve()
OUTPUT=args.output.resolve()
resolution=int(json.loads((D/'coverage-camera.json').read_text(encoding='utf8'))['pixel_width'])
if not D.is_relative_to(TICKET/'🗑️generated') or not OUTPUT.is_relative_to(TICKET/'🗑️generated'): raise ValueError('Pixel audit inputs/outputs must stay in ticket generated artifacts')
spec=importlib.util.spec_from_file_location('pixel_nearest_surface',TICKET/'pixel-nearest-surface.py')
M=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=M
spec.loader.exec_module(M)
files=[D/'coverage-items.json',D/'coverage-camera.json',TICKET/'pixel-nearest-surface.py',Path(__file__),TICKET/'pixel-ray-cases.json',TICKET/'pixel-source-export.schema.json',TICKET/'test-pixel-nearest-surface.py',M.ORACLE]+list(D.glob(str(resolution)+'-*.png'))
def hashes():
    """🔐 Keep both immutable input endpoints for actual source/raster attribution."""
    return {str(path.relative_to(TICKET)) if path.is_relative_to(TICKET) else str(path):hashlib.sha256(path.read_bytes()).hexdigest() for path in files}
start=hashes()
attestation=json.loads(args.attestation.read_text(encoding='utf8')) if args.attestation else None
if attestation and start[str((D/'coverage-items.json').relative_to(TICKET))]!=attestation['source_sha256']: raise ValueError('Source export changed after owner attestation')
images={path.stem:np.asarray(Image.open(path).convert('RGBA')) for path in files if path.suffix=='.png'}
for name,array in images.items():
    path=D/(name+'.png')
    if attestation:
        if start[str(path.relative_to(TICKET))]!=attestation['image_sha256'][name+'.png']: raise ValueError('Raster changed after input attestation')
    elif path.stat().st_mtime+1e-3<(D/'coverage-items.json').stat().st_mtime:
        raise ValueError('Raster predates exported source without explicit owner attestation')
comparisons=[]
all_pixels=set()
for mode in args.modes.split(','):
    for stage,before,after in (('removal','full','stable'),('retained-order','stable','culled'),('total','full','culled')):
        first,last=images[str(resolution)+'-'+mode+'-'+before],images[str(resolution)+'-'+mode+'-'+after]
        ys,xs=np.where(np.any(first[:,:,:3]!=last[:,:,:3],axis=2))
        pixels=list(zip(map(int,xs),map(int,ys)))
        all_pixels.update(pixels)
        comparisons.append({'mode':mode,'stage':stage,'before':before,'after':after,'pixels':pixels,'changed_channels':int(np.count_nonzero(first!=last))})
scene=M.SurfaceScene(json.loads((D/'coverage-items.json').read_text(encoding='utf8')),json.loads((D/'coverage-camera.json').read_text(encoding='utf8'))['towards'])
print('[DEBUG] Independent scene '+str(len(scene.triangles))+' triangles; '+str(len(all_pixels))+' changed pixels',flush=True)
rows=M.analyze(scene,sorted(all_pixels),5,progress=lambda done,total:print('[DEBUG] Independent rays '+str(done)+'/'+str(total),flush=True))
lookup={tuple(row['pixel']):row for row in rows}
for row in rows:
    x,y=row['pixel']
    centre=next(sample for sample in row['samples'] if np.allclose(sample['point'],[x+.5,y+.5],atol=1e-12,rtol=0))
    for label,side in (('full','before'),('stable','after'),('culled','after')):
        rgb=images[str(resolution)+'-ids-'+label][y,x,:3].astype(int)
        item=int(rgb@[65536,256,1])-1
        row[label+'_cairo_id']=item
        row[label+'_id_matches_nearest']=item in centre[side]['nearest_ids'] if item>=0 else not centre[side]['nearest_ids']
boundary_discrepancies=[]
for row in rows:
    if all(row[label+'_id_matches_nearest'] for label in ('full','stable','culled')): continue
    x,y=row['pixel']
    centre=next(sample for sample in row['samples'] if np.allclose(sample['point'],[x+.5,y+.5],atol=1e-12,rtol=0))
    ids=sorted(set([row[label+'_cairo_id'] for label in ('full','stable','culled')]+centre['before']['nearest_ids']))
    witnesses=[M.boundary_witness(scene,owner,[x+.5,y+.5]) for owner in ids if owner>=0]
    for witness in witnesses:
        witness['depth_difference_from_nearest']=witness['extrapolated_plane_depth']-centre['before']['nearest_depth']
    boundary_discrepancies.append({'pixel':[x,y],'cairo_full':row['full_cairo_id'],'cairo_stable':row['stable_cairo_id'],'cairo_culled':row['culled_cairo_id'],'physical_nearest':centre['before']['nearest_ids'],'witnesses':witnesses})
for comparison in comparisons:
    selected=[lookup[tuple(pixel)] for pixel in comparison['pixels']]
    comparison['classifications']=dict(Counter(row['classification'] for row in selected))
    comparison['visible_removed_pixels']=sum(row['visible_removed_samples']>0 for row in selected)
    comparison['coplanar_tie_pixels']=sum(row['coplanar_tie_samples']>0 for row in selected)
    comparison['physical_nearest_changed_pixels']=sum(not row['nearest_depth_unchanged'] for row in selected)
    comparison['painter_reorder_pixels']=sum(row['painter_reorder_samples']>0 for row in selected)
    comparison['centre_id_mismatches']={label:sum(not row[label+'_id_matches_nearest'] for row in selected) for label in ('full','stable','culled')}
    print('[DEBUG] '+json.dumps({key:value for key,value in comparison.items() if key!='pixels'}),flush=True)
end=hashes()
report={'resolution':resolution,'modes':args.modes.split(','),'source_before':start,'source_after':end,'unchanged':start==end,'owner_attestation':attestation,'projection_residual':scene.fit_residual,'world_to_pixel':scene.matrix.tolist(),'towards':scene.towards.tolist(),'triangles':len(scene.triangles),'source_items':len(scene.items),'camera_facing_items':len(scene.drawn),'grid':5,'pixel_count':len(rows),'comparisons':comparisons,'boundary_discrepancies':boundary_discrepancies,'pixels':rows,'limits':'Centre plus5x5 samples attribute source geometry only. No exhaustive AA equivalence or colour-tolerance waiver.'}
OUTPUT.write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] Exact input endpoints unchanged='+str(start==end),flush=True)
raise SystemExit(0 if start==end else 1)
