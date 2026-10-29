"""🧪 Resume the bounded numerical main contracts on an explicitly retained R30 model/provider."""
import hashlib
import json
import runpy
import time
import unittest
from pathlib import Path

CLIP=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
namespace=runpy.run_path(str(CLIP/'🧪️tests'/'🐍️.py'))
api=globals()['captured_api']
namespace['ModelCase'].setUpClass.__func__.__globals__['model_api']=lambda:api
before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
classes=[value for name,value in namespace.items() if name.startswith('Bim') and isinstance(value,type) and issubclass(value,namespace['ModelCase'])]
ordered=[test for cls in classes for test in unittest.defaultTestLoader.loadTestsFromTestCase(cls)]
selected=ordered[64:]+list(unittest.defaultTestLoader.loadTestsFromTestCase(namespace['BimPartitionOracleContract']))
progress=Path(__file__).parent/'🗑️generated'/'clips-main-tail-r30-progress.json'
phases=[]
class Progress(unittest.TestResult):
    """⏳ Retain start, stop, failures and elapsed time outside the active interpreter."""
    def startTest(self,test):
        super().startTest(test)
        self.started=time.monotonic()
        progress.write_text(json.dumps({'debug':'[DEBUG] numerical tail','test':str(test),'phase':'started','completed':len(phases),'failures':len(self.failures),'errors':len(self.errors)}))
    def addSubTest(self,test,subtest,err):
        super().addSubTest(test,subtest,err)
        if err is not None:
            (progress.parent/(progress.stem+"-failures.json")).write_text(json.dumps({"failures":[(str(t),m) for t,m in self.failures],"errors":[(str(t),m) for t,m in self.errors]}))
    def addFailure(self,test,err):
        super().addFailure(test,err)
        (progress.parent/(progress.stem+"-failures.json")).write_text(json.dumps({"failures":[(str(t),m) for t,m in self.failures],"errors":[(str(t),m) for t,m in self.errors]}))
    def addError(self,test,err):
        super().addError(test,err)
        (progress.parent/(progress.stem+"-failures.json")).write_text(json.dumps({"failures":[(str(t),m) for t,m in self.failures],"errors":[(str(t),m) for t,m in self.errors]}))
    def stopTest(self,test):
        phases.append((str(test),time.monotonic()-self.started))
        progress.write_text(json.dumps({'debug':'[DEBUG] numerical tail','test':str(test),'phase':'completed','completed':len(phases),'failures':len(self.failures),'errors':len(self.errors),'seconds':phases[-1][1]}))
        super().stopTest(test)
result=Progress()
unittest.TestSuite(selected).run(result)
after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
print('PROVIDER',before,after,before==after)
print('RUN',result.testsRun,'FAIL',len(result.failures),'ERROR',len(result.errors))
for test,message in result.failures+result.errors:
    print(str(test),message)
