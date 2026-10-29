"""🧪 Validate the complete native curtain cohort through portable and independent geometry contracts."""
import importlib,unittest,io,time,json,ast
from pathlib import Path
ct=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️curtains")
pf=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪟️fillings")
ns=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🪟️fillings")
root=Path(ct.api.__file__).parents[1]
for relative in ("🎨️visuals/🏛️savoye/🧱️core/🐍️.py","🎨️visuals/🏛️savoye/📋️plan/🐍️.py","🎨️visuals/🏛️savoye/🧊️axon/🐍️.py","🎨️visuals/🏛️savoye/✂️section/🐍️.py"):
    ast.parse((root/relative).read_text())
stream=io.StringIO()
suite=unittest.defaultTestLoader.loadTestsFromModule(ct)
suite.addTests(unittest.FunctionTestCase(fn) for name,fn in vars(pf).items() if name.startswith("test_"))
suite.addTests(unittest.defaultTestLoader.loadTestsFromModule(ns))
class ObservedResult(unittest.TextTestResult):
    def startTest(self,test):
        (Path(__file__).parent/"🗑️generated/curtain-validation-stage-r31.json").write_text(json.dumps({"stage":"[DEBUG] "+str(test),"time":time.time()}))
        super().startTest(test)
start=time.perf_counter()
result=unittest.TextTestRunner(stream=stream,verbosity=2,resultclass=ObservedResult).run(suite)
print(stream.getvalue())
print("seconds",time.perf_counter()-start)
print("four drawing modules parse PASS")
                                                             