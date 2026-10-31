"""🔬 Compare original physical nearest surfaces with explicit actual paint-fragment compositing."""
import argparse
import atexit
import time
from collections import Counter
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import numpy as np
from PIL import Image
sys.stdout.reconfigure(encoding="utf-8",errors="backslashreplace")
TICKET=Path(__file__).resolve().parent
parser=argparse.ArgumentParser()
parser.add_argument('--input-dir',type=Path,required=True)
parser.add_argument('--output',type=Path,required=True)
parser.add_argument('--roi',type=Path)
parser.add_argument('--include-steel',action='store_true')
args=parser.parse_args()
D=args.input_dir.resolve()
OUTPUT=args.output.resolve()
if not D.is_relative_to(TICKET/'🗑️generated') or not OUTPUT.is_relative_to(TICKET/'🗑️generated'):
    raise ValueError('All actual source/raster/query artifacts belong to ticket generated output')
spec=importlib.util.spec_from_file_location('fragment_pixel_oracle',TICKET/'pixel-nearest-surface.py')
M=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=M
spec.loader.exec_module(M)
FILES=[path for path in D.iterdir() if path.is_file()]+[Path(__file__),TICKET/'pixel-nearest-surface.py',TICKET/'pixel-ray-cases.json',TICKET/'pixel-plane-lineage-cases.json',TICKET/'pixel-plane-lineage.schema.json',TICKET/'pixel-paint-fragments.schema.json',TICKET/'pixel-source-export.schema.json',TICKET/'test-pixel-nearest-surface.py',TICKET/'pixel-warped-mesh-cases.json',TICKET/'pixel-warped-mesh.schema.json',TICKET/'pixel-convex-fan-cases.json',TICKET/'pixel-convex-fan.schema.json',M.ORACLE,Path(M.O.MODEL.__file__).resolve(),Path(M.O.MODEL.__file__).with_name('🔣️.json'),Path(M.O.MODEL.__file__).parent/'🧬️schema/🔣️.schema.json',M.O.ROOT/'🧪️cases/🚶️circulation/🔣️.json']
if args.roi:
    FILES.append(args.roi.resolve())
def hashes():
    """🔐 Bind every original/paint/camera/raster/ROI/oracle input to unchanged exact endpoints."""
    return {str(path.relative_to(TICKET)) if path.is_relative_to(TICKET) else str(path):hashlib.sha256(path.read_bytes()).hexdigest() for path in FILES}
start=hashes()
provenance=json.loads((D/'coverage-provenance.json').read_text(encoding='utf-8'))
if not provenance.get('unchanged') or provenance['start_sources']!=provenance['end_sources']:
    raise ValueError('Producer sources changed during actual fragment export/capture')
camera=json.loads((D/'coverage-camera.json').read_text(encoding='utf-8'))
resolution=camera['pixel_width']
items=json.loads((D/'coverage-items.json').read_text(encoding='utf-8'))
fragments=json.loads((D/'coverage-paint-fragments.json').read_text(encoding='utf-8'))
if {row['stage'] for row in fragments}!={'full','stable','culled'}:
    raise ValueError('All three actual paint stages must be explicit')
scene=M.SurfaceScene(items,camera['towards'])
paint=M.FragmentPaint(scene,fragments)
images={stage:np.asarray(Image.open(D/(str(resolution)+'-ids-'+stage+'.png')).convert('RGB')).astype(np.int32)@np.array([65536,256,1])-1 for stage in ('full','stable','culled')}
pixels=set()
if args.roi:
    roi=json.loads(args.roi.read_text(encoding='utf-8'))
    pixels.update(tuple(point) for point in roi['pixels'])
changed={}
for first,last in (('full','stable'),('stable','culled'),('full','culled')):
    ys,xs=np.where(images[first]!=images[last])
    values=set(zip(map(int,xs),map(int,ys)))
    changed[first+'-'+last]=len(values)
    pixels.update(values)
steel_pixels=set()
if args.include_steel:
    steel=[row['index'] for row in items if row['role']=='steel_truss']
    for image in images.values():
        ys,xs=np.where(np.isin(image,steel))
        steel_pixels.update(zip(map(int,xs),map(int,ys)))
    pixels.update(steel_pixels)
if any(not (0<=x<resolution and 0<=y<camera['pixel_height']) for x,y in pixels):
    raise ValueError('ROI coordinates must belong to the exported camera')
summary=Counter()
rows=[]
started=time.perf_counter()
journal_path=OUTPUT.with_suffix('.progress.jsonl')
journal=journal_path.open('x',encoding='utf-8')
atexit.register(journal.close)
journal.write(json.dumps({'event':'start','source_before':start,'pixels':len(pixels),'resolution':resolution},ensure_ascii=False)+'\n')
journal.flush()
print('[DEBUG] Fragment-aware physical sources '+str(len(items))+'; actual paint pieces '+str(len(fragments))+'; ROI '+str(len(pixels)),flush=True)
for index,(x,y) in enumerate(sorted(pixels)):
    if index%25==0:
        print('[DEBUG] Fragment rays '+str(index)+'/'+str(len(pixels)),flush=True)
    points=M.sample_grid(x,y,5)
    physical_hits=scene.intersections(points)
    paint_hits=paint.intersections(points)
    samples=[]
    for point,original,clipped in zip(points,physical_hits,paint_hits):
        physical=scene.contributors(original)
        retained=scene.contributors(original,True)
        depths={}
        for triangle in np.flatnonzero(np.isfinite(original)):
            owner=int(scene.owners[triangle])
            depths[owner]=max(depths.get(owner,-np.inf),float(original[triangle]))
        stages={}
        for stage in ('full','stable','culled'):
            actual=paint.contributors(clipped,stage)
            owners=actual['layers'][0]['sources'] if actual['layers'] else []
            wrong=bool(owners)!=bool(physical['nearest_ids']) or any(owner not in physical['nearest_ids'] for owner in owners)
            unsupported=sorted(set(actual['visible_ids'])-set(depths))
            ranges=[(min(depths[source] for source in layer['sources'] if source in depths),max(depths[source] for source in layer['sources'] if source in depths)) for layer in actual['layers'] if any(source in depths for source in layer['sources'])]
            inversion=any(following[1]>before[0]+1e-8 for before,following in zip(ranges,ranges[1:]))
            actual.update(wrong_nearest=wrong,unsupported_sources=unsupported,depth_inversion=inversion)
            stages[stage]=actual
            summary[stage+'_wrong_nearest_samples']+=wrong
            summary[stage+'_unsupported_samples']+=bool(unsupported)
            summary[stage+'_layer_inversion_samples']+=inversion
        summary['physical_visible_steel_samples']+=any(scene.items[owner]['role']=='steel_truss' for owner in physical['nearest_ids'])
        summary['physical_culling_nearest_changed_samples']+=set(physical['nearest_ids'])!=set(retained['nearest_ids'])
        samples.append({'point':point.tolist(),'physical':physical,'retained_physical':retained,'paint':stages})
    centre=next(row for row in samples if np.allclose(row['point'],[x+.5,y+.5],atol=1e-12,rtol=0))
    cairo={stage:int(image[y,x]) for stage,image in images.items()}
    centres={}
    for stage,owner in cairo.items():
        sources=centre['paint'][stage]['layers'][0]['sources'] if centre['paint'][stage]['layers'] else []
        matches_physical=owner in centre['physical']['nearest_ids'] if owner>=0 else not centre['physical']['nearest_ids']
        matches_paint=owner in sources if owner>=0 else not sources
        centres[stage]={'physical':matches_physical,'paint':matches_paint}
        summary[stage+'_cairo_physical_mismatch_centres']+=not matches_physical
        summary[stage+'_cairo_paint_mismatch_centres']+=not matches_paint
    witnesses=[]
    if any(not row['physical'] or not row['paint'] for row in centres.values()):
        owners=set(cairo.values())|set(centre['physical']['nearest_ids'])
        witnesses=[M.boundary_witness(scene,owner,[x+.5,y+.5]) for owner in sorted(owners) if owner>=0]
    rows.append({'pixel':[x,y],'cairo':cairo,'centre_matches':centres,'boundary_witnesses':witnesses,'samples':samples})
    journal.write(json.dumps({'event':'pixel','record':rows[-1]},ensure_ascii=False)+'\n')
    if len(rows)%25==0 or len(rows)==len(pixels):
        journal.write(json.dumps({'event':'progress','completed':len(rows),'total':len(pixels),'elapsed_seconds':time.perf_counter()-started})+'\n')
        journal.flush()
journal.close()
end=hashes()
report={'progress_journal':str(journal_path.relative_to(TICKET)),'progress_sha256':hashlib.sha256(journal_path.read_bytes()).hexdigest(),'elapsed_seconds':time.perf_counter()-started,'source_before':start,'source_after':end,'unchanged':start==end,'producer_provenance':provenance,'resolution':resolution,'source_items':len(items),'paint_piece_rows':len(fragments),'paint_unique_polygons':len(paint.geometry_rows),'physical_triangles':len(scene.triangles),'warped_original_polygons':scene.warped_polygons,'paint_triangles':len(paint.triangles),'projection_residual':scene.fit_residual,'pixels':len(rows),'rays':25*len(rows),'changed_id_pixels':changed,'visible_steel_id_pixels':len(steel_pixels),'summary':dict(summary),'rows':rows,'limits':'Finite centre plus 5x5 rays over explicit prior ROI, every new source-ID difference and optional steel pixels. Physical nearest uses only original source polygons; actual point compositing uses clipped paint rows unioned once per rank. No Cairo rerender, colour tolerance or original scalar-rank fallback.'}
OUTPUT.write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps({key:value for key,value in report.items() if key not in ('source_before','source_after','producer_provenance','rows')}),flush=True)
raise SystemExit(0 if report['unchanged'] else 1)
