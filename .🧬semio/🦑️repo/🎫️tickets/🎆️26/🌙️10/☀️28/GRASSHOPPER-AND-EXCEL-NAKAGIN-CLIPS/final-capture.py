"""🖼️ Bound drawing caches to one fresh gallery worker and bind every produced image."""
from datetime import datetime,timezone
import hashlib
import importlib.util
import json
from pathlib import Path
import sys
from PIL import Image

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
PLAN=json.loads((TICKET/'capture-workers.json').read_text(encoding='utf-8'))
CHECKPOINT=globals()['capture_checkpoint']
GENERATED=Path(globals()['capture_generated']).resolve()
OUTPUT=Path(globals()['capture_output']).resolve()
assert GENERATED.is_relative_to((TICKET/'🗑️generated').resolve())
assert OUTPUT.is_relative_to((TICKET/'🗑️generated').resolve())
OUTPUT.mkdir(parents=True,exist_ok=True)
spec=importlib.util.spec_from_file_location('capture_process',TICKET/'capture-worker-process.py')
PROCESS=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=PROCESS
spec.loader.exec_module(PROCESS)
assert {worker['name'] for worker in PLAN['workers']}=={'details','direction0','direction1','direction2','direction3','sections','families','windows','roofs'}
assert len(PLAN['workers'])==9
report={'started_at':globals()['capture_started'],'source_fingerprints':globals()['capture_source'],'runtime_identity':globals()['capture_runtime'],'workers':[],'complete':False}

def save_record():
    """🧾 Retain each worker state before advancing to the next expensive drawing."""
    (OUTPUT/'workers.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')

def repo_child(worker,label):
    """▶️ Require the actual child source/runtime envelope before accepting its pictures."""
    baseline=OUTPUT/'start.json'
    destination=OUTPUT/('child-'+worker['name'])
    arguments=['--target',worker['script'],'--baseline',str(baseline),'--record',str(destination),'--',*worker['arguments']]
    PROCESS.run_worker(ROOT,TICKET,OUTPUT,TICKET/'capture-worker.py',label,lambda event:CHECKPOINT(),arguments=arguments)
    raw=(destination/'result.json').read_bytes()
    record=json.loads(raw)
    if not record['success'] or not record['unchanged'] or record['target']!=worker['script'] or record['arguments']!=worker['arguments'] or record['baseline_sha256']!=hashlib.sha256(baseline.read_bytes()).hexdigest():
        raise RuntimeError('Unaccepted actual capture worker: '+label)
    if record['source_start']!=record['source_end'] or record['source_start']!=report['source_fingerprints'] or record['runtime_start']!=record['runtime_end']:
        raise RuntimeError('Capture child escaped the frozen source/runtime baseline')
    observed=record['runtime_start']
    expected=report['runtime_identity']
    if any(observed[key]!=expected[key] for key in ('python','executable','platform','packages')) or any(observed['config'][key]!=expected['config'][key] for key in ('renderer','pixel_width','pixel_height','frame_width','frame_height','frame_rate')):
        raise RuntimeError('Capture child interpreter or camera identity differs')
    for key,folder in (('media_dir','media'),('text_dir','text')):
        if Path(observed['config'][key]).resolve()!=(TICKET/'🗑️generated'/label/folder).resolve():
            raise RuntimeError('Capture child used an unexpected cache directory')
    return {'sha256':hashlib.sha256(raw).hexdigest(),'record':record}

for worker in PLAN['workers']:
    CHECKPOINT()
    started=datetime.now(timezone.utc)
    row={'name':worker['name'],'script':worker['script'],'arguments':worker['arguments'],'started_at':started.isoformat(),'status':'running'}
    report['workers'].append(row)
    save_record()
    print('[DEBUG] Capture worker '+worker['name'],flush=True)
    try:
        child=repo_child(worker,'fc-'+worker['name'])
        images=[]
        for name in worker['outputs']:
            path=GENERATED/name
            if path.parent!=GENERATED or path.stat().st_mtime<started.timestamp():
                raise RuntimeError('Stale or invalid capture output: '+name)
            with Image.open(path) as image:
                if image.format!='PNG' or image.size!=(1920,1080):
                    raise RuntimeError('Incorrect capture image: '+name)
            images.append({'name':name,'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
        CHECKPOINT()
        row.update(status='passed',images=images,child_result=child)
    except BaseException:
        row['status']='failed'
        save_record()
        raise
    save_record()

CHECKPOINT()
gallery=Image.new('RGB',(1920,1080))
for index,name in enumerate(PLAN['gallery']['panels']):
    path=GENERATED/name
    expected=next(image['sha256'] for row in report['workers'] for image in row['images'] if image['name']==name)
    if hashlib.sha256(path.read_bytes()).hexdigest()!=expected:
        raise RuntimeError('Capture panel changed before composition: '+name)
    left,right=PLAN['gallery']['boundaries'][index:index+2]
    with Image.open(path) as image:
        gallery.paste(image.convert('RGB').crop((left,0,right,1080)),(left,0))
destination=GENERATED/PLAN['gallery']['output']
gallery.save(destination)
CHECKPOINT()
report.update(complete=True,gallery={'name':destination.name,'sha256':hashlib.sha256(destination.read_bytes()).hexdigest()})
save_record()
print('[DEBUG] All nine bounded capture workers and eight final galleries completed',flush=True)
