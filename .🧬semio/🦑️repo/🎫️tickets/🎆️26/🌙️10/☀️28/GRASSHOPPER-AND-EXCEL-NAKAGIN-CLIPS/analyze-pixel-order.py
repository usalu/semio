"""🔍 Independently measure retained painter errors and the complete frame/glass overlap depth order."""
import argparse
from collections import Counter
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import numpy as np

TICKET=Path(__file__).parent
parser=argparse.ArgumentParser()
parser.add_argument('--input-dir',type=Path,required=True)
parser.add_argument('--ray-report',type=Path,required=True)
parser.add_argument('--output',type=Path,required=True)
args=parser.parse_args()
spec=importlib.util.spec_from_file_location('independent_pixel_geometry',TICKET/'pixel-nearest-surface.py')
M=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=M
spec.loader.exec_module(M)
ray=json.loads(args.ray_report.read_text(encoding='utf8'))
items=json.loads((args.input_dir/'coverage-items.json').read_text(encoding='utf8'))
camera=json.loads((args.input_dir/'coverage-camera.json').read_text(encoding='utf8'))
scene=M.SurfaceScene(items,camera['towards'])


def cross(a,b):
    """✂️ The oriented two-dimensional line determinant."""
    return float(a[0]*b[1]-a[1]*b[0])


def intersection(subject,clip):
    """📐 Independently clip one actual triangle against another convex physical triangle."""
    clip=np.asarray(clip,float)
    sign=1 if sum(cross(a,b) for a,b in zip(clip,np.roll(clip,-1,axis=0)))>=0 else -1
    result=list(np.asarray(subject,float))
    for a,b in zip(clip,np.roll(clip,-1,axis=0)):
        incoming,result=result,[]
        if not incoming: break
        for p,q in zip(incoming,incoming[1:]+incoming[:1]):
            inside_p=sign*cross(b-a,p-a)>=-1e-10
            inside_q=sign*cross(b-a,q-a)>=-1e-10
            if inside_p: result.append(p)
            if inside_p!=inside_q:
                denominator=cross(q-p,b-a)
                if abs(denominator)>1e-14: result.append(p+cross(a-p,b-a)/denominator*(q-p))
    return np.asarray(result,float)


def depth(owner,points):
    """📏 Evaluate the exported physical source plane on the independently fitted parallel camera rays."""
    item=scene.items[owner]
    base=(np.asarray(points)-scene.offset)@scene.inverse.T
    normal=np.asarray(item['normal'])
    return (np.asarray(item['world_vertices'][0])-base)@normal/(normal@scene.towards)


def overlap(first,second):
    """🧮 Affine source-depth extrema occur at the exact triangle-intersection polygon vertices."""
    aa=scene.triangles[scene.owners==first]@scene.matrix.T+scene.offset
    bb=scene.triangles[scene.owners==second]@scene.matrix.T+scene.offset
    polygons=[]
    for a in aa:
        for b in bb:
            polygon=intersection(a,b)
            if len(polygon)<3: continue
            area=abs(sum(cross(p,q) for p,q in zip(polygon,np.roll(polygon,-1,axis=0))))/2
            if area<=1e-10: continue
            differences=depth(first,polygon)-depth(second,polygon)
            polygons.append({'area_pixels_squared':area,'pixel_vertices':polygon.tolist(),'first_minus_second_depth':differences.tolist()})
    values=[value for polygon in polygons for value in polygon['first_minus_second_depth']]
    return {'first':first,'second':second,'positive_area_intersections':len(polygons),'overlap_area_pixels_squared':sum(p['area_pixels_squared'] for p in polygons),'minimum_depth_difference':min(values) if values else None,'maximum_depth_difference':max(values) if values else None,'sign_reverses':bool(values and min(values)<-1e-8 and max(values)>1e-8),'polygons':polygons}

counts={label:{'wrong_samples':0,'wrong_pixels':0} for label in ('full','stable','culled')}
for row in ray['pixels']:
    wrong=Counter()
    for sample in row['samples']:
        for label in counts:
            nearest=sample['before' if label=='full' else 'after']['nearest_ids']
            painted=sample['painter_'+label]
            bad=bool(nearest)!=bool(painted) or bool(nearest and painted and painted[0] not in nearest)
            counts[label]['wrong_samples']+=int(bad)
            wrong[label]+=int(bad)
    for label in counts: counts[label]['wrong_pixels']+=int(wrong[label]>0)
actual=overlap(318,340)
control=intersection(np.array([[0,0],[2,0],[2,2]]),np.array([[1,0],[3,0],[1,2]]))
if not np.all(np.isfinite(control)): raise AssertionError('Independent clipping positive control is not finite')
report={'input_ray_sha256':hashlib.sha256(args.ray_report.read_bytes()).hexdigest(),'changed_pixels':ray['pixel_count'],'samples':sum(row['sample_count'] for row in ray['pixels']),'painter_errors_against_physical_nearest':counts,'frame_glazing_overlap':actual,'scope':'All actual overlap vertices, not only raster centres; depth is affine on each exported source plane.'}
args.output.write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps({**{k:v for k,v in report.items() if k!='frame_glazing_overlap'},'frame_glazing_overlap':{k:v for k,v in actual.items() if k!='polygons'}}),flush=True)
