"""🔎 Execute newly recovered native contracts before the final source-frozen cohort."""
import importlib,unittest,sys,os,json,hashlib,time,threading,_thread
from pathlib import Path
ROOT=Path.cwd();BASE=Path(__file__).parent;OUT=BASE/'🗑️generated/native-r92';sys.path.insert(0,str(ROOT))
os.environ.update(SEMIO_TEST_MANIM='1',SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT));os.environ.pop('SEMIO_TEST_BIM_TOPICS',None)
PREFIX='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
modules=('🔗️consistency.🧪️tests.🔄️lifecycle.🐍️','🧩️models.🧪️tests.📏️footer.🐍️','🧪️tests.🎭️presentation.🐍️','🧱️model.🧪️tests.🌀️screens','🧱️model.🔌️exchange.🧪️tests.🐍️','🧱️model.🧮️analysis.🧪️tests.📏️physical')
paths=[p for p in (ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip').rglob('*') if p.suffix in ('.py','.json','.md') and '__pycache__' not in p.parts]
def hashes():
    """🔐 Retain exact current producers and source inputs for this diagnostic."""
    return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes();start=time.monotonic();main=importlib.import_module(PREFIX+'🧪️tests.🐍️');loader=unittest.TestLoader();suite=unittest.TestSuite()
for module in modules:suite.addTests(main._colocated_tests(loader,importlib.import_module(PREFIX+module),None))
stop=threading.Event()
def observe():
    """⏳ Forward cancellation during expensive native bodies."""
    while not stop.wait(.5):
        if (OUT/'uncovered.cancel').exists():_thread.interrupt_main();return
threading.Thread(target=observe,daemon=True).start()
class Result(unittest.TextTestResult):
    """🚦 Persist exact individual outcomes as the native cohort advances."""
    def issue(self,test,error,kind):
        detail=self._exc_info_to_string(error,test)
        with (OUT/'uncovered-issues.jsonl').open('a',encoding='utf8') as stream:
            stream.write(json.dumps({'started':start,'time':time.time(),'id':test.id(),'kind':kind,'traceback':detail})+'\n');stream.flush()
        print('[DEBUG] '+kind+' '+detail,flush=True)
    def addError(self,test,error):self.issue(test,error,'error');super().addError(test,error)
    def addFailure(self,test,error):self.issue(test,error,'failure');super().addFailure(test,error)
    def addSubTest(self,test,subtest,error):
        if error is not None:self.issue(subtest,error,'subtest')
        super().addSubTest(test,subtest,error)
    def stopTest(self,test):
        super().stopTest(test)
        (OUT/'uncovered-progress.json').write_text(json.dumps({'test':test.id(),'tests_run':self.testsRun,'failures':len(self.failures),'errors':len(self.errors),'seconds':time.monotonic()-start}),encoding='utf8')
        print('[DEBUG] native recovered '+test.id(),flush=True)
try:result=unittest.TextTestRunner(verbosity=2,resultclass=Result).run(suite)
finally:stop.set()
after=hashes();report={'diagnostic':True,'source_stable':before==after,'before':before,'after':after,'tests':result.testsRun,'failures':[(t.id(),s) for t,s in result.failures],'errors':[(t.id(),s) for t,s in result.errors],'skipped':[(t.id(),s) for t,s in result.skipped],'seconds':time.monotonic()-start,'success':result.wasSuccessful() and not result.skipped}
(OUT/'uncovered-result.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');raise SystemExit(0 if report['success'] else 1)
