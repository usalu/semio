"""⏱️ Measure source-frozen construction with durable stages and separately owned stack diagnostics."""
import argparse
import hashlib
import importlib
import json
import os
from pathlib import Path
import sys
import time
import traceback

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
parser=argparse.ArgumentParser()
parser.add_argument('--output-name',required=True)
args=parser.parse_args()
if not args.output_name.replace('-','').isalnum():
    raise ValueError('Invalid output name')
output=Path(__file__).parent/'🗑️generated'
output.mkdir(parents=True,exist_ok=True)
target=output/(args.output_name+'.json')
sources=list(dict.fromkeys([*map(Path,V.source_files()),Path(V._model.__file__),Path(V._model.__file__).with_name('🔣️.json'),Path(__file__)]))
start_sources={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
started=time.perf_counter()
stages={}
measurements={'state':'running','source_start':start_sources,'runtime':{'pid':os.getpid(),'executable':sys.executable,'version':sys.version}}

def snapshot(phase,count=None):
    """💾 Preserve completed stage evidence even if the process is interrupted."""
    measurements.update(phase=phase,phase_items=count,wall_seconds=time.perf_counter()-started,stages={k:dict(v) for k,v in stages.items()})
    payload=json.dumps(measurements,indent=2)+'\n'
    with target.with_suffix('.jsonl').open('a',encoding='utf-8') as journal:
        journal.write(json.dumps(measurements,separators=(',',':'))+'\n')
        journal.flush()
    with target.open('r+' if target.exists() else 'w',encoding='utf-8') as current:
        current.seek(0)
        current.write(payload)
        current.truncate()
        current.flush()

originals={name:getattr(V,name) for name in ('_assembly_items','_opaque_visible','_section_order','_paint_ordered')}
for name,original in originals.items():
    def measured(*arguments,_name=name,_original=original,**keywords):
        count=len(arguments[0])
        notable=(_name=='_section_order' and count>10000) or (_name in ('_opaque_visible','_assembly_items') and count>1000)
        if notable:
            print('[DEBUG] Begin '+_name+' '+str(count)+' items',flush=True)
            snapshot('begin '+_name,count)
        if notable and _name=='_section_order' and keywords.get('progress') is None:
            last=[time.perf_counter()]
            def report(event):
                now=time.perf_counter()
                if now-last[0]>=5:
                    last[0]=now
                    measurements['ordering_progress']=dict(event)
                    print('[DEBUG] Ordering '+json.dumps(event),flush=True)
                    snapshot('ordering '+str(event.get('phase','unknown')),count)
            keywords['progress']=report
        start=time.perf_counter()
        result=_original(*arguments,**keywords)
        elapsed=time.perf_counter()-start
        row=stages.setdefault(_name,{'calls':0,'seconds':0,'total_inputs':0,'maximum_inputs':0,'maximum_outputs':0})
        row['calls']+=1
        row['seconds']+=elapsed
        row['total_inputs']+=count
        row['maximum_inputs']=max(row['maximum_inputs'],count)
        row['maximum_outputs']=max(row['maximum_outputs'],len(result))
        if notable:
            print('[DEBUG] End '+_name+' '+str(count)+' items in '+str(elapsed)+' s',flush=True)
            snapshot('end '+_name,count)
        return result
    setattr(V,name,measured)

with (output/(args.output_name+'-stacks.log')).open('w',encoding='utf-8') as stack:
    stack.write('[DEBUG] Native stack dumping disabled after reproducible process termination; durable phase snapshots remain\n')
    stack.flush()
    try:
        scene=V.NakaginDrawing()
        for label,kwargs in (('cold',{}),('warm',{'at':(1,2,0),'scale':.13})):
            print('[DEBUG] Building '+label+' tower',flush=True)
            snapshot(label)
            start=time.perf_counter()
            drawing=scene.tower_axon(**kwargs)
            measurements[label]={'seconds':time.perf_counter()-start,'faces':len(drawing),'capsules':len(drawing.capsules),'order_cache_entries':len(V._OCCLUSION_ORDERS),'opaque_visibility_entries':len(V._OPAQUE_VISIBILITY),'mobjects':len(drawing.get_family()),'source_capsule_parts':sum(len(c.semantic_parts) for c in drawing.capsules.values())}
            assert len(drawing.capsules)==140
            print('[DEBUG] '+label+' '+json.dumps(measurements[label]),flush=True)
            snapshot(label+' complete')
        measurements['state']='complete'
    except BaseException:
        measurements.update(state='failed',exception=traceback.format_exc())
        raise
    finally:
        measurements['source_end']={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sources}
        measurements['unchanged']=start_sources==measurements['source_end']
        snapshot(measurements['state'])
        for name,original in originals.items():
            setattr(V,name,original)
if not measurements['unchanged']:
    raise AssertionError('Profile sources changed during execution')
