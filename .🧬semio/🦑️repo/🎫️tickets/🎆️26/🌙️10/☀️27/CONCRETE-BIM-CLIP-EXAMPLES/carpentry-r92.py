"""🪚 Run current native source-backed carpentry regressions with frozen input provenance."""
import hashlib,importlib,inspect,json,sys,time,unittest
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
clip=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip';stage=sys.argv[1] if len(sys.argv)>1 else 'green';out=Path(__file__).parent/'🗑️generated';out.mkdir(exist_ok=True)
def snapshot():
    """🔐 Hash current source, providers, schemas and all feature inputs."""
    return {str(p.relative_to(clip)):hashlib.sha256(p.read_bytes()).hexdigest() for p in clip.rglob('*') if p.suffix in ('.py','.json') and '__pycache__' not in p.parts}
before=snapshot();started=time.monotonic();prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.'
suite=unittest.TestSuite();selected=[]
for name in (['🪚️joinery'] if stage=='red' else ['🪚️joinery','🪟️fillings','🪟️curtains']):
    module=importlib.import_module(prefix+name)
    for key,fn in inspect.getmembers(module,inspect.isfunction):
        if key.startswith('test_') and (stage!='red' or key=='test_registered_original_carpentry_dimensions_and_physical_materials'):
            suite.addTest(unittest.FunctionTestCase(fn));selected.append(module.__name__+'.'+key)
result=unittest.TextTestRunner(verbosity=2).run(suite);after=snapshot();report={'stage':stage,'selected':selected,'run':result.testsRun,'failures':[(str(t),e) for t,e in result.failures],'errors':[(str(t),e) for t,e in result.errors],'before':before,'after':after,'stable':before==after,'seconds':time.monotonic()-started,'pass':result.wasSuccessful()}
(out/('carpentry-'+stage+'-r92.json')).write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');raise SystemExit(0 if report['pass'] and report['stable'] else 1)
