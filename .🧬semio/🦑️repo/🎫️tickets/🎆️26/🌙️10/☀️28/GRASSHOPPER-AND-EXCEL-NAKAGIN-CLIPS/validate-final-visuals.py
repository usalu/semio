"""🧪 Retain complete final visual-suite or gallery outcomes and source identities."""
import argparse
from datetime import datetime,timezone
import importlib.metadata
import importlib.util
import inspect
import json
from pathlib import Path
import platform
import runpy
import sys
import unittest
sys.dont_write_bytecode=True
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
parser=argparse.ArgumentParser()
parser.add_argument('--output-name',required=True)
parser.add_argument('--mode',choices=('contracts','capture'),required=True)
args=parser.parse_args()
if not args.output_name.replace('-','').isalnum():
    raise ValueError('Invalid output name')
OUTPUT=TICKET/'🗑️generated'/args.output_name
OUTPUT.mkdir(parents=True,exist_ok=True)
capture_names=('final-capture.py','capture-worker.py','capture-worker-process.py','stop-capture-worker.ts','capture-process.schema.json','capture-workers.json','capture-workers.schema.json','visual-smoke.py','visual-audit-gallery.py','capture-capsule-families.py','capture-window-states.py','capture-roof-frame.py')
paths=[BASE/'🧪️tests/🎨️visuals/🐍️.py'] if args.mode=='contracts' else [*(TICKET/name for name in capture_names),BASE/'🧪️tests/👁️occlusion/🐍️.py']
import manim
manim.config.pixel_width,manim.config.pixel_height=1920,1080
manim.config.frame_width,manim.config.frame_height=16,9
V=__import__('importlib').import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
suite=unittest.TestSuite()
for path in (paths if args.mode=='contracts' else ()):
    loaded=runpy.run_path(str(path))
    for value in loaded.values():
        if isinstance(value,type) and issubclass(value,unittest.TestCase) and value.__module__=='<run_path>':
            suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(value))
spec=importlib.util.spec_from_file_location('renderer_contract_provenance',TICKET/'scene-provenance.py')
provenance=importlib.util.module_from_spec(spec)
spec.loader.exec_module(provenance)
def dependencies():
    """🔗 Include actual loaded repository modules, declared assets, schemas, config and import closure."""
    manifest=[BASE/'🔣️.json',*BASE.glob('🧬️schema/*.json'),*BASE.glob('🧪️cases/👁️occlusion/**/*.json'),*BASE.glob('🧪️cases/🎨️visuals/**/*.json')]
    if args.mode=='capture':
        manifest.extend((ROOT/'🧰️framework/🔨️modules/🏃️process/🪓️termination/🟦️.ts',ROOT/'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🩺️doctor/🔌️ports/🟦️.ts'))
    for loaded in tuple(sys.modules.values()):
        filename=getattr(loaded,'__file__',None)
        if not filename:
            continue
        path=Path(filename).resolve()
        if not path.is_file() or not path.is_relative_to(ROOT) or {'site-packages','dist-packages','__pycache__'}&set(path.parts):
            continue
        manifest.append(path)
        source_files=getattr(loaded,'source_files',None)
        if callable(source_files) and not any(p.default is inspect.Parameter.empty and p.kind not in (inspect.Parameter.VAR_POSITIONAL,inspect.Parameter.VAR_KEYWORD) for p in inspect.signature(source_files).parameters.values()):
            manifest.extend(source_files())
    from manim._config.utils import config_file_paths
    manifest.extend(Path(p) for p in config_file_paths() if Path(p).is_file())
    return provenance.dependency_paths(ROOT,paths,manifest,(Path(__file__),TICKET/'verify.py',TICKET/'scene-provenance.py'))
def runtime():
    """🪪 Retain the actual interpreter, renderer package versions and configuration identity."""
    import manim
    versions={name:importlib.metadata.version(name) for name in ('manim','numpy','Pillow','pycairo','rhino3dm','networkx','manimpango')}
    config={key:str(getattr(manim.config,key)) for key in ('renderer','pixel_width','pixel_height','frame_width','frame_height','frame_rate','media_dir','text_dir','tex_dir')}
    return {'python':sys.version,'executable':sys.executable,'platform':platform.platform(),'packages':versions,'config':config}
started=datetime.now(timezone.utc).isoformat()
before=provenance.fingerprints(ROOT,dependencies())
runtime_before=runtime()
(OUTPUT/'start.json').write_text(json.dumps({'started_at':started,'sources':before,'runtime':runtime_before},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
class Result(unittest.TextTestResult):
    def record(self,test,outcome,error=None):
        row={'started_at':started,'test':test.id(),'outcome':outcome}
        if error is not None:
            row['error']=self._exc_info_to_string(error,test)
        with (OUTPUT/'checks.jsonl').open('a',encoding='utf-8') as stream:
            stream.write(json.dumps(row,ensure_ascii=False)+'\n')
            stream.flush()
    def addSuccess(self,test):
        super().addSuccess(test)
        self.record(test,'success')
    def addFailure(self,test,error):
        super().addFailure(test,error)
        self.record(test,'failure',error)
    def addError(self,test,error):
        super().addError(test,error)
        self.record(test,'error',error)
    def addSubTest(self,test,subtest,error):
        super().addSubTest(test,subtest,error)
        if error is not None:
            self.record(subtest,'error',error)
if args.mode=='contracts':
    result=unittest.TextTestRunner(verbosity=2,resultclass=Result,failfast=True).run(suite)
else:
    class Capture(unittest.TestCase):
        def runTest(self):
            def checkpoint():
                if provenance.fingerprints(ROOT,dependencies())!=before or runtime()!=runtime_before:
                    raise RuntimeError('Frozen capture source/runtime inputs changed')
            runpy.run_path(str(TICKET/'final-capture.py'),run_name='__main__',init_globals={'capture_checkpoint':checkpoint,'capture_generated':TICKET/'🗑️generated','capture_output':OUTPUT,'capture_started':started,'capture_source':before,'capture_runtime':runtime_before})
    result=unittest.TextTestRunner(verbosity=2,resultclass=Result,failfast=True).run(unittest.TestSuite([Capture()]))
after=provenance.fingerprints(ROOT,dependencies())
runtime_after=runtime()
record={'started_at':started,'tests':result.testsRun,'success':result.wasSuccessful(),'failures':len(result.failures),'errors':len(result.errors),'source_start':before,'source_end':after,'runtime_start':runtime_before,'runtime_end':runtime_after,'unchanged':provenance.unchanged(before,after) and runtime_before==runtime_after}
(OUTPUT/'result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Renderer contract source/runtime endpoints unchanged:',record['unchanged'],flush=True)
raise SystemExit(0 if record['success'] and record['unchanged'] else 1)
