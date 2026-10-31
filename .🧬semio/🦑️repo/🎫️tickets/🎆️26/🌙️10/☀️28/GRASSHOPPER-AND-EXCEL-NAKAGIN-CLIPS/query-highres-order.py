"""🔭 Attribute the finite high-resolution changed pixels using the exact lower-resolution camera map."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
import numpy as np
from PIL import Image

TICKET=Path(__file__).parent
parser=argparse.ArgumentParser()
parser.add_argument('--input-dir',type=Path,required=True)
parser.add_argument('--resolution',type=int,required=True)
parser.add_argument('--output',type=Path,required=True)
args=parser.parse_args()
D=args.input_dir
spec=importlib.util.spec_from_file_location('finite_highres_ray',TICKET/'pixel-nearest-surface.py')
M=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=M
spec.loader.exec_module(M)
files=[D/'coverage-items.json',D/'coverage-camera.json',D/(str(args.resolution)+'-geometric-full.png'),D/(str(args.resolution)+'-geometric-culled.png'),Path(__file__),TICKET/'pixel-nearest-surface.py']
def hashes():
    """🔐 Preserve both exact input endpoints for this finite changed-pixel query."""
    return {str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in files}
start=hashes()
camera=json.loads(files[1].read_text(encoding='utf8'))
scene=M.SurfaceScene(json.loads(files[0].read_text(encoding='utf8')),camera['towards'])
a,b=(np.asarray(Image.open(path).convert('RGB')) for path in files[2:4])
if a.shape!=b.shape or a.shape[:2]!=(args.resolution,args.resolution): raise ValueError('Control image sizes disagree')
ratio=camera['pixel_width']/args.resolution
if camera['pixel_height']/args.resolution!=ratio: raise ValueError('Isotropic camera control scaling required')
y,x=np.where(np.any(a!=b,axis=2))
rows=[]
counts={'full_wrong':0,'candidate_wrong':0,'new_wrong':0,'corrected':0,'shared_wrong':0,'point_order_unchanged':0}
for px,py in zip(x,y):
    samples=[]
    for point,hit in zip(M.sample_grid(int(px),int(py),5),scene.intersections(M.sample_grid(int(px),int(py),5)*ratio)):
        physical=scene.contributors(hit)['nearest_ids']
        full,new=scene.painter(hit,'full'),scene.painter(hit,'culled')
        bad=lambda painter:bool(physical)!=bool(painter) or bool(physical and painter and painter[0] not in physical)
        bf,bn=bad(full),bad(new)
        counts['full_wrong']+=int(bf);counts['candidate_wrong']+=int(bn)
        counts['new_wrong']+=int(bn and not bf);counts['corrected']+=int(bf and not bn);counts['shared_wrong']+=int(bf and bn)
        counts['point_order_unchanged']+=int(full==new)
        samples.append({'point':point.tolist(),'source_camera_point':(point*ratio).tolist(),'nearest':physical,'full':full,'candidate':new,'full_wrong':bf,'candidate_wrong':bn})
    rows.append({'pixel':[int(px),int(py)],'samples':samples})
end=hashes()
report={'resolution':args.resolution,'source_resolution':camera['pixel_width'],'source_coordinate_ratio':ratio,'pixels':len(rows),'samples':sum(len(row['samples']) for row in rows),'counts':counts,'source_before':start,'source_after':end,'unchanged':start==end,'rows':rows}
args.output.write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps({k:v for k,v in report.items() if k not in ('source_before','source_after','rows')}),flush=True)
raise SystemExit(0 if start==end else 1)
