"""🖼️ Inventory every final gallery only after current-source capture acceptance."""
from datetime import datetime
import argparse
import hashlib
import importlib.metadata
import importlib.util
import json
from pathlib import Path
import platform
import struct
import sys
TICKET=Path(__file__).resolve().parent
ROOT=next(path for path in TICKET.parents if (path/'🧰️framework').is_dir())
OUTPUT=TICKET/'🗑️generated'
parser=argparse.ArgumentParser()
parser.add_argument('--capture-name',required=True)
args=parser.parse_args()
if not args.capture_name.replace('-','').isalnum():
    raise ValueError('Invalid capture record name')
record=json.loads((OUTPUT/args.capture_name/'result.json').read_text(encoding='utf-8'))
if not record['success'] or not record['unchanged'] or record['tests']!=1 or not record['source_start'] or record['source_start']!=record['source_end']:
    raise ValueError('Final capture did not pass against unchanged explicit inputs')
if record['runtime_start']!=record['runtime_end']:
    raise ValueError('Final capture runtime changed')
runtime=record['runtime_start']
if (runtime['python'],runtime['executable'],runtime['platform'])!=(sys.version,sys.executable,platform.platform()) or any(importlib.metadata.version(name)!=version for name,version in runtime['packages'].items()):
    raise ValueError('Inventory must use the accepted capture interpreter and packages')
spec=importlib.util.spec_from_file_location('inventory_provenance',TICKET/'scene-provenance.py')
provenance=importlib.util.module_from_spec(spec)
spec.loader.exec_module(provenance)
paths=[Path(key.removeprefix('@external:')) if key.startswith('@external:') else ROOT/key for key in record['source_start']]
if provenance.fingerprints(ROOT,paths)!=record['source_start']:
    raise ValueError('Accepted capture inputs are no longer current')
workers=json.loads((OUTPUT/args.capture_name/'workers.json').read_text(encoding='utf-8'))
if not workers['complete'] or len(workers['workers'])!=9 or workers['source_fingerprints']!=record['source_start'] or workers['runtime_identity']!=runtime or any(worker['status']!='passed' for worker in workers['workers']):
    raise ValueError('The complete accepted actual worker ledger is required')
bindings={image['name']:image['sha256'] for worker in workers['workers'] for image in worker['images']}
bindings[workers['gallery']['name']]=workers['gallery']['sha256']
for worker in workers['workers']:
    raw=(OUTPUT/args.capture_name/('child-'+worker['name'])/'result.json').read_bytes()
    if hashlib.sha256(raw).hexdigest()!=worker['child_result']['sha256'] or json.loads(raw)!=worker['child_result']['record']:
        raise ValueError('Actual worker proof changed after capture: '+worker['name'])
started=datetime.fromisoformat(record['started_at']).timestamp()
names=('nakagin-detail-gallery.png','nakagin-orthographic-gallery.png','nakagin-four-exterior-directions.png','nakagin-representative-sections.png','nakagin-six-families-closed.png','nakagin-six-families-cutaway.png','nakagin-window-mechanism-states.png','roof-frame-occlusion.png')
images=[]
for name in names:
    path=OUTPUT/name
    data=path.read_bytes()
    if data[:8]!=b'\x89PNG\r\n\x1a\n' or data[12:16]!=b'IHDR':
        raise ValueError('Invalid PNG: '+name)
    dimensions=struct.unpack('>II',data[16:24])
    if dimensions!=(1920,1080) or path.stat().st_mtime<started:
        raise ValueError('Stale or incorrectly sized final gallery: '+name)
    digest=hashlib.sha256(data).hexdigest()
    if bindings.get(name)!=digest:
        raise ValueError('Final gallery differs from its accepted actual worker: '+name)
    images.append({'name':name,'dimensions':dimensions,'bytes':len(data),'sha256':digest})
result={'capture_started_at':record['started_at'],'images':images,'image_count':len(images)}
(OUTPUT/'final-image-inventory.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Eight fresh final gallery PNGs inventoried with exact hashes',flush=True)
