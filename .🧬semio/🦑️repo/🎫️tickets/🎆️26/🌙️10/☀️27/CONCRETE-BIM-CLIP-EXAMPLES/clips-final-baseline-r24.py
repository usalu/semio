"""🧪 Main pure BIM baseline and callback/bilingual proofs for one recorded source cohort."""
import hashlib
import json
import time
import runpy
import unittest
from pathlib import Path

CLIP=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
SOURCE=CLIP/'🧱️model'/'🔣️.json'
source_before=globals().get('captured_source_hash') or hashlib.sha256(SOURCE.read_bytes()).hexdigest()
namespace=runpy.run_path(str(CLIP/'🧪️tests'/'🐍️.py'))
api=globals().get('captured_api') or namespace['model_api']()
provider_before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
namespace['ModelCase'].setUpClass.__func__.__globals__['model_api']=lambda:api
classes=[value for name,value in namespace.items() if name.startswith('Bim') and isinstance(value,type) and issubclass(value,namespace['ModelCase'])]
suite=unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromTestCase(cls) for cls in classes)
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(namespace["BimPartitionOracleContract"]))
excluded={'test_each_chapter_has_unique_bilingual_narration_and_discoverable_scene','test_all_bilingual_clauses_fit_native_caption_layout_for_both_audiences'}
for name in unittest.defaultTestLoader.getTestCaseNames(namespace['BimClipContract']):
    if name not in excluded:
        suite.addTest(namespace['BimClipContract'](name))
chapter_namespace=runpy.run_path(str(CLIP/'🧪️tests'/'🧩️chapter-source'/'🐍️.py'))
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(chapter_namespace['BimChapterSourceContract']))
class ProgressResult(unittest.TestResult):
    """⏱️ Retain exact per-test phases and durations for the portable cohort."""
    def startTest(self,test):
        super().startTest(test)
        self.phase=str(test)
        self.started=time.monotonic()
        progress=Path(__file__).parent/"🗑️generated"/"clips-baseline-r24-progress.json"
        progress.parent.mkdir(exist_ok=True)
        progress.write_text(json.dumps({"debug":"[DEBUG] portable baseline progress","test":self.phase,"completed":len(phases)},ensure_ascii=False),encoding="utf-8")
    def stopTest(self,test):
        phases.append((str(test),time.monotonic()-self.started))
        progress=Path(__file__).parent/"🗑️generated"/"clips-baseline-r24-progress.json"
        progress.write_text(json.dumps({"debug":"[DEBUG] portable baseline progress","test":str(test),"phase":"completed","completed":len(phases),"failures":len(self.failures),"errors":len(self.errors),"seconds":phases[-1][1]},ensure_ascii=False),encoding="utf-8")
        super().stopTest(test)
phases=[]
result=ProgressResult()
suite.run(result)
source_after=hashlib.sha256(SOURCE.read_bytes()).hexdigest()
provider_after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
print('SOURCE',len(api.load_model()['elements']),source_before,source_after,'STABLE',source_before==source_after)
print('PROVIDER_STABLE',provider_before,provider_after,provider_before==provider_after)
print('RUN',result.testsRun,'FAIL',len(result.failures),'ERROR',len(result.errors))
for test,message in result.failures+result.errors:
    print(str(test),message)
print('NATIVE_EXCLUDED',sorted(excluded))
