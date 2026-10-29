"""🔬 Record selected method, helper, fixture and production identities without executing tests."""
import ast,hashlib,inspect,json,runpy,unittest
from pathlib import Path
root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path(__file__).parent;generated=ticket/'🗑️generated'
def hash72(value):return hashlib.sha256(value).hexdigest()
prior=json.loads((generated/'features-r72a-inputs.json').read_text());ns=runpy.run_path(str(root/'🧪️tests/🐍️.py'));api=ns['model_api']()
def canon72(test):
    fn=getattr(test,'_testFunc',None)
    if fn:
        bound=next((v for v in fn.__defaults__ or () if inspect.ismodule(v)),None)
        return bound.__name__+'.run' if bound and fn.__name__=='<lambda>' else fn.__module__+'.'+fn.__qualname__
    return test.id()
def flat72(suite):
    for item in suite:
        if isinstance(item,unittest.TestSuite):yield from flat72(item)
        else:yield item
all72=list(flat72(ns['load_tests'](unittest.defaultTestLoader,unittest.TestSuite(),None)));lookup72={canon72(t):t for t in all72};methods72={};modulepaths72={}
for key72 in prior['selected_ids']:
    test72=lookup72[key72];fn72=getattr(test72,'_testFunc',None)
    if fn72:
        bound72=next((v for v in fn72.__defaults__ or () if inspect.ismodule(v)),None);fn72=bound72.run if bound72 and fn72.__name__=='<lambda>' else fn72
    else:fn72=getattr(test72,test72._testMethodName)
    path72=Path(inspect.getsourcefile(fn72));modulepaths72[str(path72.relative_to(root))]=path72
    methods72[key72]=hash72(ast.dump(ast.parse(__import__('textwrap').dedent(inspect.getsource(fn72))),include_attributes=False).encode())
helpers72={}
for rel72,path72 in modulepaths72.items():
    tree72=ast.parse(path72.read_text())
    for node72 in tree72.body:
        if isinstance(node72,ast.ClassDef):node72.body=[n for n in node72.body if not isinstance(n,ast.FunctionDef) or not n.name.startswith('test_')]
    tree72.body=[n for n in tree72.body if not isinstance(n,ast.FunctionDef) or not n.name.startswith('test_')]
    helpers72[rel72]=hash72(ast.dump(tree72,include_attributes=False).encode())
fixtures72={p:hash72((root/p).read_bytes()) for p in json.loads((generated/'features-r68-inputs.json').read_text())['files_before'] if p.endswith('.json')}
record72={'source':hash72((root/'🧱️model/🔣️.json').read_bytes()),'provider':hash72(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()),'production':{p:hash72((root/p).read_bytes()) for p in prior['before']['production']},'method_ast':methods72,'helper_ast':helpers72,'fixtures':fixtures72}
output72=globals().get('continuity_output','features-r72a-continuity-before.json');(generated/output72).write_text(json.dumps(record72,ensure_ascii=False));print({'methods':len(methods72),'helpers':len(helpers72),'source':record72['source'],'provider':record72['provider']})
