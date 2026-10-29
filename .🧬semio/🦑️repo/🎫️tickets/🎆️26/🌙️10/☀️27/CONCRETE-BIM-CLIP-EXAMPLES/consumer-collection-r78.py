import ast,hashlib,inspect,json,runpy,textwrap,unittest
from pathlib import Path
clip=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path("/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES");gen=ticket/'🗑️generated'
ns=runpy.run_path(str(clip/'🧪️tests/🐍️.py'))
def flat(s):
 for t in s:
  if isinstance(t,unittest.TestSuite):yield from flat(t)
  else:yield t
def canonical(t):
 fn=getattr(t,'_testFunc',None)
 if fn:
  bound=next((v for v in fn.__defaults__ or () if inspect.ismodule(v)),None)
  return bound.__name__+'.run' if bound and fn.__name__=='<lambda>' else fn.__module__+'.'+fn.__qualname__
 return t.id()
items=list(flat(ns['load_tests'](unittest.defaultTestLoader,unittest.TestSuite(),None)));ids=[canonical(t) for t in items]
assert len(ids)==len(set(ids))==352
before=json.loads((gen/'features-r72b-selected-scope-before.json').read_text());methods={}
for t in items:
 fn=getattr(t,'_testFunc',None) or getattr(t,t._testMethodName)
 try:methods[canonical(t)]=hashlib.sha256(ast.dump(ast.parse(textwrap.dedent(inspect.getsource(fn))),include_attributes=False).encode()).hexdigest()
 except (TypeError,OSError):pass
changed=[key for key,value in before['selected_method_ast'].items() if methods.get(key)!=value]
helpers={};helper_changed=[]
for relative,old in before['selected_module_helper_ast'].items():
 nodes=ast.parse((clip/relative).read_text()).body
 current={n.name:hashlib.sha256(ast.dump(n,include_attributes=False).encode()).hexdigest() for n in nodes if isinstance(n,(ast.FunctionDef,ast.AsyncFunctionDef)) and not n.name.startswith('test_')}
 helpers[relative]=current
 helper_changed.extend(relative+':'+name for name,value in old.items() if current.get(name)!=value)
record={'registered':len(ids),'all_ids':ids,'selected_method_ast':methods,'changed_selected_methods':changed,'changed_existing_helpers':helper_changed,'current_helpers':helpers,'source':hashlib.sha256((clip/'🧱️model/🔣️.json').read_bytes()).hexdigest(),'scope':'Collection and AST only; no test execution or geometry setup'}
(gen/'consumer-collection-r78.json').write_text(json.dumps(record));print({'registered':len(ids),'changed_selected_methods':changed,'changed_existing_helpers':helper_changed})
