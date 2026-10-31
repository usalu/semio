"""🧪 Execute six source fixture-fitting contracts in the verified existing authoring runtime."""
import hashlib,importlib,importlib.util,importlib.metadata,json,sys,time,unittest
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;model=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model';assert sys.version_info[:2]==(3,12);paths=[p for p in model.rglob('*.py')if '🧪️tests'not in str(p)]+[model/'🔣️.json',model/'🧬️schema/🔣️.schema.json',Path(__file__),root/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml',root/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/uv.lock']+[model/'🧪️tests'/name for name in ('🔧️fixture-fits.py','🔧️fixture-fits.json','🔧️fixture-fits.schema.json')]
paths += [ticket/'fixture-fits-author-r95.py',ticket/'fixture-observer-r95.py']
def hashes():
    """🔐 Bind actual runtime bodies to physical source, producers and neutral inputs."""
    return {str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest()for p in paths}
before=hashes();prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.';module=importlib.import_module(prefix+'🔧️fixture-fits');functions=[getattr(module,name)for name in sorted(dir(module))if name.startswith('test_')];assert len(functions)==6;original_load=module.api.load_model
if sys.argv[1]=='candidate':
    from copy import deepcopy
    spec=importlib.util.spec_from_file_location('fixture_fit_author_r95',ticket/'fixture-fits-author-r95.py');author=importlib.util.module_from_spec(spec);spec.loader.exec_module(author);candidate=author.apply(original_load());module.api.load_model=lambda *args,**kwargs:deepcopy(candidate)
    tags=[author.fits.case()['rain']['tag'],author.fits.case()['lamp']['tag']]+[r['tag']for r in author.fits.case()['taps']]+[author.fits.case()['storage']['tag']];(ticket/'🗑️generated/fixture-fits-stock-r95.json').write_text(json.dumps({'source':hashlib.sha256((model/'🔣️.json').read_bytes()).hexdigest(),'objects':[e for e in candidate['elements']if e['tag']in tags]},ensure_ascii=False,indent=2),encoding='utf8')
suite=unittest.TestSuite(unittest.FunctionTestCase(fn)for fn in functions);start=time.monotonic();observer_spec=importlib.util.spec_from_file_location('fixture_observer_r95',ticket/'fixture-observer-r95.py');observer_module=importlib.util.module_from_spec(observer_spec);observer_spec.loader.exec_module(observer_module);observer=observer_module.Observer(ticket,Path(__file__).stem+'-'+sys.argv[1])
try:
    result=unittest.TextTestRunner(verbosity=2).run(suite)
finally:
    module.api.load_model=original_load;observer.close()
after=hashes();out={'pass':result.wasSuccessful()and not result.skipped and before==after,'tests':result.testsRun,'failures':result.failures,'errors':result.errors,'skipped':result.skipped,'before':before,'after':after,'stable':before==after,'python':sys.version,'executable':sys.executable,'packages':{name:importlib.metadata.version(name)for name in ('numpy','scipy','shapely','jsonschema','manim')},'seconds':time.monotonic()-start,'resource_scope':'Focused six-contract source fixture-fitting suite using the locked authoring CPython3.12 native runtime normal6GB admission; no whole building_geometry or native scene/Cairo lifecycle.'};out['failures']=[(str(t),msg)for t,msg in result.failures];out['errors']=[(str(t),msg)for t,msg in result.errors];(ticket/'🗑️generated'/('fixture-fits-'+sys.argv[1]+'-r95.json')).write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:v for k,v in out.items()if k not in ('before','after','failures','errors')}));raise SystemExit(not out['pass'])
                                                                                       