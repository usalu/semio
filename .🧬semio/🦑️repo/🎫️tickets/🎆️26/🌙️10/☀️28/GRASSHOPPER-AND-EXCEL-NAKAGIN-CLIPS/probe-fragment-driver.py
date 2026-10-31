"""🔬 Exercise the actual audit driver with a finite analytic crossing and original-source aliases."""
import hashlib
import json
from pathlib import Path
import runpy
import sys
from PIL import Image
TICKET=Path(__file__).resolve().parent
stamp=hashlib.sha256((TICKET/'pixel-ray-cases.json').read_bytes()+(TICKET/'run-fragment-surface-audit.py').read_bytes()).hexdigest()[:12]
FOLDER=TICKET/'🗑️generated'/('fragment-driver-probe-'+stamp)
RESULT=TICKET/'🗑️generated'/('fragment-driver-probe-'+stamp+'.json')
FOLDER.mkdir(parents=True,exist_ok=True)
case=next(row for row in json.loads((TICKET/'pixel-ray-cases.json').read_text(encoding='utf-8'))['fragment_cases'] if row['name']=='explicit-stage-orders-never-use-original-rank')
(FOLDER/'coverage-items.json').write_text(json.dumps(case['items']),encoding='utf-8')
(FOLDER/'coverage-paint-fragments.json').write_text(json.dumps(case['fragments']),encoding='utf-8')
(FOLDER/'coverage-camera.json').write_text(json.dumps({'towards':[0,0,1],'pixel_width':4,'pixel_height':4}),encoding='utf-8')
identity={'portable_cases':hashlib.sha256((TICKET/'pixel-ray-cases.json').read_bytes()).hexdigest()}
(FOLDER/'coverage-provenance.json').write_text(json.dumps({'start_sources':identity,'end_sources':identity,'unchanged':True}),encoding='utf-8')
for stage in ('full','stable','culled'):
    image=Image.new('RGB',(4,4),(0,0,0))
    for y in range(3):
        for x in range(3):
            source=1 if stage!='culled' or x==0 else 0
            image.putpixel((x,y),(0,0,source+1))
    image.save(FOLDER/('4-ids-'+stage+'.png'))
roi=FOLDER/'roi.json'
roi.write_text(json.dumps({'pixels':[[0,1],[2,1]]}),encoding='utf-8')
sys.argv=[str(TICKET/'run-fragment-surface-audit.py'),'--input-dir',str(FOLDER),'--output',str(RESULT),'--roi',str(roi)]
try:
    runpy.run_path(str(TICKET/'run-fragment-surface-audit.py'),run_name='__main__')
except SystemExit as result:
    if result.code not in (0,None):
        raise
record=json.loads(RESULT.read_text(encoding='utf-8'))
assert record['unchanged']
assert record['summary']['culled_wrong_nearest_samples']==0
assert record['summary']['full_wrong_nearest_samples']>0
assert record['summary']['culled_cairo_physical_mismatch_centres']==0
assert record['summary']['full_cairo_physical_mismatch_centres']>0
print('[DEBUG] End-to-end independent fragment driver analytic crossing proof passed.')
