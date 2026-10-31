"""🖼️ Record the actual source, fonts, interpreter and camera envelope of one capture worker."""
import argparse
from datetime import datetime,timezone
import hashlib
import importlib
import importlib.metadata
import importlib.util
import inspect
import json
from pathlib import Path
import platform
import runpy
import sys
import traceback

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
GENERATED=(TICKET/'🗑️generated').resolve()
parser=argparse.ArgumentParser()
parser.add_argument('--target',required=True)
parser.add_argument('--baseline',type=Path,required=True)
parser.add_argument('--record',type=Path,required=True)
args,selected=parser.parse_known_args()
if selected[:1]==['--']:
    selected=selected[1:]
plan=json.loads((TICKET/'capture-workers.json').read_text(encoding='utf-8'))
if args.target not in {worker['script'] for worker in plan['workers']}:
    raise ValueError('Capture target is outside the explicit worker plan')
if not any(worker['script']==args.target and worker['arguments']==selected for worker in plan['workers']):
    raise ValueError('Capture arguments are outside the explicit worker plan')
if not args.baseline.resolve().is_relative_to(GENERATED) or not args.record.resolve().is_relative_to(GENERATED):
    raise ValueError('Capture records must remain in the ticket generated directory')
target=TICKET/args.target
args.record.mkdir(parents=True,exist_ok=True)
raw_baseline=args.baseline.read_bytes()
baseline=json.loads(raw_baseline)
expected=baseline['sources']
expected_runtime=baseline['runtime']
if not expected or not expected_runtime:
    raise ValueError('The parent source and runtime baseline is required')
spec=importlib.util.spec_from_file_location('capture_provenance',TICKET/'scene-provenance.py')
PROVENANCE=importlib.util.module_from_spec(spec)
spec.loader.exec_module(PROVENANCE)
import manim
manim.config.pixel_width,manim.config.pixel_height=1920,1080
manim.config.frame_width,manim.config.frame_height=16,9
importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')

def dependencies():
    """🔗 Include the complete parent baseline and every actual worker-owned source manifest."""
    paths=[Path(key.removeprefix('@external:')) if key.startswith('@external:') else ROOT/key for key in expected]
    for loaded in tuple(sys.modules.values()):
        filename=getattr(loaded,'__file__',None)
        if not filename:
            continue
        path=Path(filename).resolve()
        if not path.is_file() or not path.is_relative_to(ROOT) or {'site-packages','dist-packages','__pycache__'}&set(path.parts):
            continue
        paths.append(path)
        source_files=getattr(loaded,'source_files',None)
        if callable(source_files) and not any(parameter.default is inspect.Parameter.empty and parameter.kind not in (inspect.Parameter.VAR_POSITIONAL,inspect.Parameter.VAR_KEYWORD) for parameter in inspect.signature(source_files).parameters.values()):
            paths.extend(source_files())
    return PROVENANCE.dependency_paths(ROOT,(target,),paths,(Path(__file__),TICKET/'verify.py',TICKET/'scene-provenance.py'))

def runtime():
    """🧰️ Observe the real child interpreter, rendering packages and allocated camera/cache configuration."""
    versions={name:importlib.metadata.version(name) for name in ('manim','numpy','Pillow','pycairo','rhino3dm','networkx','manimpango')}
    config={key:str(getattr(manim.config,key)) for key in ('renderer','pixel_width','pixel_height','frame_width','frame_height','frame_rate','media_dir','text_dir','tex_dir')}
    return {'python':sys.version,'executable':sys.executable,'platform':platform.platform(),'packages':versions,'config':config}

started=datetime.now(timezone.utc).isoformat()
before=PROVENANCE.fingerprints(ROOT,dependencies())
runtime_before=runtime()
record={'started_at':started,'target':args.target,'arguments':selected,'baseline_sha256':hashlib.sha256(raw_baseline).hexdigest(),'source_start':before,'runtime_start':runtime_before,'success':False}
(args.record/'start.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
error=None
try:
    if before!=expected:
        raise RuntimeError('Actual child sources do not match the complete parent baseline')
    if any(runtime_before[key]!=expected_runtime[key] for key in ('python','executable','platform','packages')):
        raise RuntimeError('Actual child interpreter or package versions differ from the parent')
    if any(runtime_before['config'][key]!=expected_runtime['config'][key] for key in ('renderer','pixel_width','pixel_height','frame_width','frame_height','frame_rate')):
        raise RuntimeError('Actual child camera configuration differs from the parent')
    if not all(Path(runtime_before['config'][key]).resolve().is_relative_to(GENERATED) for key in ('media_dir','text_dir')):
        raise RuntimeError('Actual child rendering cache must remain ticket-local')
    sys.argv=[str(target),*selected]
    try:
        runpy.run_path(str(target),run_name='__main__')
    except SystemExit as completed:
        if completed.code not in (None,0):
            raise
    record['success']=True
except BaseException:
    error=traceback.format_exc()
after=PROVENANCE.fingerprints(ROOT,dependencies())
runtime_after=runtime()
unchanged=before==after==expected and runtime_before==runtime_after and args.baseline.read_bytes()==raw_baseline
record.update(source_end=after,runtime_end=runtime_after,unchanged=unchanged,error=error)
(args.record/'result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
if error:
    print(error,file=sys.stderr,flush=True)
print('[DEBUG] Actual capture worker source/runtime endpoints unchanged: '+str(unchanged),flush=True)
raise SystemExit(0 if record['success'] and unchanged else 1)
