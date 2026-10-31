"""🧪 Retain dependency-scoped renderer contract outcomes without native scene replay."""
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
parser.add_argument('--target',action='append')
parser.add_argument('--input',action='append',default=[])
args=parser.parse_args()
if not args.output_name.replace('-','').isalnum():
    raise ValueError('Invalid output name')
OUTPUT=TICKET/'🗑️generated'/args.output_name
OUTPUT.mkdir(parents=True,exist_ok=True)
paths=[(ROOT/value).resolve() for value in args.target] if args.target else [BASE/'🧪️tests/👁️occlusion/🐍️.py',BASE/'🧪️tests/👁️occlusion/✂️fragments/🐍️.py',BASE/'🧪️tests/👁️occlusion/📦️grouping/🐍️.py']
if any(not path.is_file() or not path.is_relative_to(ROOT) for path in paths):
    raise ValueError('Test targets must be files inside the repository')
suite=unittest.TestSuite()
for path in paths:
    loaded=runpy.run_path(str(path))
    for value in loaded.values():
        if isinstance(value,type) and issubclass(value,unittest.TestCase) and value.__module__=='<run_path>':
            suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(value))
spec=importlib.util.spec_from_file_location('renderer_contract_provenance',TICKET/'scene-provenance.py')
provenance=importlib.util.module_from_spec(spec)
spec.loader.exec_module(provenance)
def dependencies():
    """🔗 Include actual loaded repository modules, declared assets, schemas, config and import closure."""
    manifest=[BASE/'🔣️.json',*((ROOT/value).resolve() for value in args.input)]
    for target in paths:
        parts=list(target.parent.parts)
        if '🧪️tests' in parts:
            parts[parts.index('🧪️tests')]='🧪️cases'
            manifest.extend(Path(*parts).rglob('*.json'))
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
    versions={name:importlib.metadata.version(name) for name in ('manim','numpy','Pillow','pycairo','rhino3dm','networkx')}
    config={key:str(getattr(manim.config,key)) for key in ('renderer','pixel_width','pixel_height','frame_width','frame_height','frame_rate','media_dir','text_dir','tex_dir')}
    return {'python':sys.version,'executable':sys.executable,'platform':platform.platform(),'packages':versions,'config':config}
started=datetime.now(timezone.utc).isoformat()
before=provenance.fingerprints(ROOT,dependencies())
runtime_before=runtime()
(OUTPUT/'start.json').write_text(json.dumps({'started_at':started,'expected_tests':suite.countTestCases(),'sources':before,'runtime':runtime_before},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
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
result=unittest.TextTestRunner(verbosity=2,resultclass=Result,failfast=True).run(suite)
after=provenance.fingerprints(ROOT,dependencies())
runtime_after=runtime()
record={'started_at':started,'tests':result.testsRun,'success':result.wasSuccessful(),'failures':len(result.failures),'errors':len(result.errors),'source_start':before,'source_end':after,'runtime_start':runtime_before,'runtime_end':runtime_after,'unchanged':provenance.unchanged(before,after) and runtime_before==runtime_after}
(OUTPUT/'result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] Renderer contract source/runtime endpoints unchanged:',record['unchanged'],flush=True)
raise SystemExit(0 if record['success'] and record['unchanged'] else 1)
