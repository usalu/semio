"""🧪 Verify Excel static gates with ticket-local output and exact dependency endpoint records."""
import argparse
from datetime import datetime,timezone
import importlib
import importlib.metadata
import importlib.util
import inspect
import json
import os
from pathlib import Path
import platform
import sys
import tempfile
import unittest

sys.dont_write_bytecode=True
sys.stdout.reconfigure(encoding='utf8',errors='backslashreplace',line_buffering=True)
sys.stderr.reconfigure(encoding='utf8',errors='backslashreplace',line_buffering=True)
parser=argparse.ArgumentParser()
parser.add_argument('--gate',choices=('main','registry'),required=True)
args=parser.parse_args()
TICKET=Path(__file__).parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
CLIP=ROOT/'🎓️teaching/🏛️architecture/📊️excel/🎬️clip'
MODEL=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
OUT=TICKET/'🗑️generated/excel-static'/args.gate
OUT.mkdir(parents=True,exist_ok=True)
for name in ('TMPDIR','TEMP','TMP','SEMIO_TEACHING_OUTPUT_ROOT'): os.environ[name]=str(OUT)
os.environ['SEMIO_TEACHING_CACHE_DIR']=str(OUT/'cache')
tempfile.tempdir=str(OUT)
sys.path.insert(0,str(ROOT))

class Tee:
    """📝 Retain exact test progress while preserving the live tool output."""
    def __init__(self,stream,log): self.stream,self.log=stream,log
    def write(self,text):
        self.log.write(text)
        self.log.flush()
        return self.stream.write(text)
    def flush(self): self.log.flush();self.stream.flush()


def load(name,path):
    """📦 Load an owned validation input without running its command-line entry point."""
    spec=importlib.util.spec_from_file_location(name,path)
    module=importlib.util.module_from_spec(spec)
    sys.modules[name]=module
    spec.loader.exec_module(module)
    return module

P=load('excel_static_provenance',TICKET/'scene-provenance.py')
target=CLIP/'🧪️tests'/('🐍️.py' if args.gate=='main' else '📇️registry/🐍️.py')
seeds={target,Path(__file__),TICKET/'scene-provenance.py',MODEL/'🐍️.py',MODEL/'🔣️.json',MODEL/'🧬️schema/🔣️.schema.json'}
if args.gate=='main':
    seeds.update((CLIP/'🧱️model/🐍️.py',CLIP/'🧱️model/🔣️.json',CLIP/'🧱️model/🧬️schema/🔣️.schema.json',CLIP/'🧪️cases/🔣️.json'))
else:
    from manim import config
    config.media_dir,config.text_dir=str(OUT/'media'),str(OUT/'text')
    cases=json.loads((target.parent/'🔣️.cases.json').read_text(encoding='utf8'))
    authoring=ROOT/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip'
    seeds.update((target.parent/'🔣️.cases.json',ROOT/'🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json',authoring/'🪟️presentation/🔣️.json',authoring/'🪟️presentation/📚️collection/🔣️.json',CLIP/'🐍️.py'))
    seeds.update(CLIP.glob('🗣️transcript.*.md'))
    for chapter in cases['chapters']:
        seeds.update((ROOT/'🎓️teaching'/chapter['entry'],CLIP/chapter['folder']/'🐍️.py'))
bootstrap=P.fingerprints(ROOT,P.dependency_paths(ROOT,tuple(seeds),(),()))
module=load('excel_static_'+args.gate,target)
if args.gate=='registry':
    for chapter in cases['chapters']:
        importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.'+chapter['folder']+'.🐍️')

def dependencies():
    """🔗 Include explicit readers and every actual imported project implementation or declared source asset."""
    manifest=set(seeds)
    for loaded in tuple(sys.modules.values()):
        name=getattr(loaded,'__file__',None)
        if not name: continue
        path=Path(name).resolve()
        if not path.is_file() or not path.is_relative_to(ROOT) or {'site-packages','dist-packages','__pycache__'}&set(path.parts): continue
        manifest.add(path)
        source_files=getattr(loaded,'source_files',None)
        if callable(source_files) and not any(p.default is inspect.Parameter.empty and p.kind not in (inspect.Parameter.VAR_POSITIONAL,inspect.Parameter.VAR_KEYWORD) for p in inspect.signature(source_files).parameters.values()):
            manifest.update(source_files())
    if args.gate=='registry':
        from manim._config.utils import config_file_paths
        manifest.update(path for path in config_file_paths() if path.is_file())
    return P.dependency_paths(ROOT,tuple(manifest),(),())

def runtime():
    """⚙️ Bind installed independent oracles and the exact Python runtime to the result."""
    versions={}
    for name in ('numpy','openpyxl','formulas','rhino3dm','manim','Pillow'):
        try: versions[name]=importlib.metadata.version(name)
        except importlib.metadata.PackageNotFoundError: versions[name]=None
    return {'python':sys.version,'executable':sys.executable,'platform':platform.platform(),'packages':versions}

before=P.fingerprints(ROOT,dependencies())
if any(before.get(key)!=value for key,value in bootstrap.items()): raise RuntimeError('Static inputs changed during import bootstrap')
start=datetime.now(timezone.utc).isoformat()
identity=runtime()
(OUT/'start.json').write_text(json.dumps({'started_at':start,'source_fingerprints':before,'runtime_identity':identity},ensure_ascii=False,indent=2),encoding='utf8')
with (OUT/'tests.log').open('w',encoding='utf8') as log:
    print('[DEBUG] Excel static '+args.gate+' starting '+str(len(before))+' exact dependencies',flush=True)
    result=unittest.TextTestRunner(stream=Tee(sys.stdout,log),verbosity=2).run(unittest.defaultTestLoader.loadTestsFromModule(module))
after=P.fingerprints(ROOT,dependencies())
expected={'main':85,'registry':5}[args.gate]
record={'gate':args.gate,'started_at':start,'ended_at':datetime.now(timezone.utc).isoformat(),'tests':result.testsRun,'expected_tests':expected,'failures':len(result.failures),'errors':len(result.errors),'skipped':len(result.skipped),'assertions_passed':result.wasSuccessful(),'source_fingerprints':before,'source_fingerprints_end':after,'source_unchanged':before==after,'runtime_identity':identity,'runtime_identity_end':runtime()}
record['success']=result.wasSuccessful() and result.testsRun==expected and not result.skipped and before==after and record['runtime_identity']==record['runtime_identity_end']
(OUT/'result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps({key:record[key] for key in ('gate','tests','failures','errors','source_unchanged','success')}),flush=True)
raise SystemExit(0 if record['success'] else 1)
