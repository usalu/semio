"""🔎 Independently enumerate actual native test bodies and compare the published cohort."""
import ast
from collections import Counter
import hashlib
import importlib
import inspect
import json
import os
from pathlib import Path
import sys
import unittest
from jsonschema import Draft202012Validator

ROOT=Path.cwd()
sys.path.insert(0,str(ROOT))
os.environ.update(SEMIO_TEST_MANIM="1",SEMIO_TEACHING_LANGUAGE="en",SEMIO_TEACHING_OUTPUT_ROOT=str(Path(__file__).parent/"🗑️generated/native-r92"))
os.environ.pop("SEMIO_TEST_BIM_TOPICS",None)
CLIP=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"
PREFIX="🎓️teaching.🏛️architecture.🏗️bim.🎬️clip."
OUTPUT=Path(__file__).parent/"🗑️generated/native-r92"
def hashes():
    """🔐 Bind every authored native test input before and after collection."""
    return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in CLIP.rglob("*") if p.is_file() and p.suffix in (".py",".json") and "__pycache__" not in p.parts}
def identity(test):
    """🪪 Read the actual callable identity independently of unittest display labels."""
    fn=getattr(test,"_testFunc",None)
    if fn is not None:
        if fn.__name__=="<lambda>" and fn.__defaults__:
            source=fn.__defaults__[0]
            return Path(source.__file__).relative_to(ROOT).with_suffix("").as_posix().replace("/",".")+".run"
        return Path(fn.__code__.co_filename).relative_to(ROOT).with_suffix("").as_posix().replace("/",".")+"."+fn.__qualname__
    return Path(sys.modules[type(test).__module__].__file__).relative_to(ROOT).with_suffix("").as_posix().replace("/",".")+"."+type(test).__qualname__+"."+test._testMethodName

def flatten(suite):
    """📚 Retain every concrete body without executing it."""
    for test in suite:
        if isinstance(test,unittest.TestSuite):yield from flatten(test)
        else:yield test

before=hashes()
main=importlib.import_module(PREFIX+"🧪️tests.🐍️")
modules=[main,*[importlib.import_module(PREFIX+n) for n in main.COLOCATED_TEST_MODULES]]
modules.extend(importlib.import_module(PREFIX+"🎨️visuals.🏛️savoye.🧪️ortho-remodel."+p.stem) for p in sorted((CLIP/"🎨️visuals/🏛️savoye/🧪️ortho-remodel").glob("test_*.py")))
expected=[]
rows=[]
custom=[]
for source in modules:
    tree=ast.parse(Path(source.__file__).read_text(encoding="utf-8"))
    path=Path(source.__file__).relative_to(ROOT).with_suffix("").as_posix().replace("/",".")
    names=[]
    if source is not main and any(isinstance(n,ast.FunctionDef) and n.name=="load_tests" for n in tree.body):
        custom.append(source.__name__)
        names=[identity(t) for t in flatten(unittest.TestLoader().loadTestsFromModule(source))]
    else:
        for node in tree.body:
            if isinstance(node,ast.FunctionDef) and node.name.startswith("test_"):
                names.append(path+"."+node.name)
            if isinstance(node,ast.ClassDef):
                cls=getattr(source,node.name,None)
                if not inspect.isclass(cls) or not issubclass(cls,unittest.TestCase):continue
                methods={name for base in cls.__mro__ for name,value in vars(base).items() if name.startswith("test_") and callable(value)}
                names.extend(path+"."+cls.__qualname__+"."+name for name in sorted(methods))
    expected.extend(names)
    rows.append({"module":source.__name__,"file":str(Path(source.__file__).relative_to(ROOT)),"bodies":sorted(names)})
for name in main.ROOT_TEST_MODULES:
    source=importlib.import_module(PREFIX+name)
    expected.append(Path(source.__file__).relative_to(ROOT).with_suffix("").as_posix().replace("/",".")+".run")
collected=list(flatten(unittest.TestLoader().loadTestsFromModule(main)))
actual=[identity(t) for t in collected]
covered={str(Path(m.__file__).resolve()) for m in modules}
covered.update(str(Path(importlib.import_module(PREFIX+n).__file__).resolve()) for n in main.ROOT_TEST_MODULES)
for test in collected:
    fn=getattr(test,"_testFunc",None)
    filename=fn.__code__.co_filename if fn is not None else sys.modules[type(test).__module__].__file__
    covered.add(str(Path(filename).resolve()))
unregistered=[]
for p in CLIP.rglob("*.py"):
    if "__pycache__" in p.parts or str(p.resolve()) in covered:continue
    tree=ast.parse(p.read_text(encoding="utf-8"))
    bodies=[n.name for n in tree.body if isinstance(n,ast.FunctionDef) and n.name.startswith("test_")]
    bodies.extend(n.name+"."+m.name for n in tree.body if isinstance(n,ast.ClassDef) for m in n.body if isinstance(m,ast.FunctionDef) and m.name.startswith("test_"))
    if bodies:unregistered.append({"file":p.relative_to(ROOT).as_posix(),"bodies":bodies})
after=hashes()
report={"discovery_only":True,"test_bodies_executed":False,"source_stable":before==after,"before":before,"after":after,"declared_modules":len(modules),"expected_count":len(expected),"actual_count":len(actual),"missing":sorted(set(expected)-set(actual)),"extra":sorted(set(actual)-set(expected)),"duplicates":[k for k,v in Counter(actual).items() if v>1],"unregistered":unregistered,"custom_hooks":custom,"modules":rows,"actual":sorted(actual)}
report["success"]=before==after and not any(report[k] for k in ("missing","extra","duplicates","unregistered"))
(OUTPUT/"discovery-body-inventory.json").write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding="utf-8")
assert not list(Draft202012Validator({"type":"array","uniqueItems":True,"const":sorted(expected)}).iter_errors(sorted(actual))), "Independent body inventory mismatch; see retained report"
print("[DEBUG] "+json.dumps({k:report[k] for k in ("success","source_stable","declared_modules","expected_count","actual_count","missing","extra","duplicates","unregistered")} ),flush=True)
raise SystemExit(0 if report["success"] else 1)
