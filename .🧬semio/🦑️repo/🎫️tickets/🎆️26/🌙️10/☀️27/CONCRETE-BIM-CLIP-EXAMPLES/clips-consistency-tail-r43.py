"""🧪 Complete the retained447 consistency tail with a bounded independent roof domain."""
import ast,hashlib,json,runpy,time,unittest
from pathlib import Path

TICKET=Path(__file__).parent
namespace=historical_namespace
api=captured_api
before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
source=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🐍️.py')
tree=ast.parse(source.read_text())
cls=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=='ModelCase')
method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name=='roof_load')
space={}
exec(compile(ast.fix_missing_locations(ast.Module(body=[method],type_ignores=[])),str(source),'exec'),space)
namespace['ModelCase'].roof_load=space['roof_load']
namespace['ModelCase'].setUpClass.__func__.__globals__['model_api']=lambda:api
suite=unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromTestCase(namespace['BimConsistencyContract']))
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(namespace['BimPartitionOracleContract']))
excluded={'test_each_chapter_has_unique_bilingual_narration_and_discoverable_scene','test_all_bilingual_clauses_fit_native_caption_layout_for_both_audiences'}
for name in unittest.defaultTestLoader.getTestCaseNames(namespace['BimClipContract']):
    if name not in excluded:
        suite.addTest(namespace['BimClipContract'](name))
chapters=runpy.run_path(str(source.parent/'🧩️chapter-source'/'🐍️.py'),init_globals={'CAPTURED_MODEL':api.load_model()})
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(chapters['BimChapterSourceContract']))
progress=TICKET/'🗑️generated'/'clips-consistency-tail-r43.json'
phases=[]
class Progress(unittest.TestResult):
    """⏳ Retain immediate per-test progress and every failure outside the interpreter."""
    def write(self,phase,test):
        progress.write_text(json.dumps({'debug':'[DEBUG] retained447tail','phase':phase,'test':str(test),'completed':len(phases),'planned':planned,'failures':[(str(t),m) for t,m in self.failures],'errors':[(str(t),m) for t,m in self.errors]}))
    def startTest(self,test):
        super().startTest(test)
        self.started=time.monotonic()
        self.write('started',test)
    def stopTest(self,test):
        phases.append((str(test),time.monotonic()-self.started))
        self.write('completed',test)
        super().stopTest(test)
    def addFailure(self,test,error):
        super().addFailure(test,error)
        self.write('failure',test)
    def addError(self,test,error):
        super().addError(test,error)
        self.write('error',test)
    def addSubTest(self,test,subtest,error):
        super().addSubTest(test,subtest,error)
        if error is not None:
            self.write('subtest failure',subtest)
planned=suite.countTestCases()
result=Progress()
suite.run(result)
after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
print('RETAINED447TAIL',planned,result.testsRun,len(result.failures),len(result.errors),sum(seconds for _,seconds in phases),before,after)
for test,message in result.failures+result.errors:
    print(str(test),message)
