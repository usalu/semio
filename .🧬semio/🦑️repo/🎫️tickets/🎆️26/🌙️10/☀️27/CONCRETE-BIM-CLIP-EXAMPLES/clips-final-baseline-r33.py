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
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest()==source_before,'Source changed before the recorded core cohort'
def production_hashes():
    """📦 Record current implementation identities without copying source files."""
    return {str(p.relative_to(CLIP)):hashlib.sha256(p.read_bytes()).hexdigest() for p in CLIP.rglob('*.py') if not any('🧪' in part for part in p.relative_to(CLIP).parts)}
def oracle_hashes():
    """🧪 Record portable test and fixture identities for the executing core cohort."""
    return {str(p.relative_to(CLIP)):hashlib.sha256(p.read_bytes()).hexdigest() for p in CLIP.rglob('*') if p.is_file() and p.suffix in ('.py','.json') and any('🧪' in part for part in p.relative_to(CLIP).parts)}
code_before=production_hashes()
assert globals().get('captured_code_hashes',code_before)==code_before,'Production changed after the recorded preparation cohort'
oracles_before=oracle_hashes()
namespace=runpy.run_path(str(CLIP/'🧪️tests'/'🐍️.py'))
api=globals().get('captured_api') or namespace['model_api']()
provider_before=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
manifest_directory=Path(__file__).parent/'🗑️generated'
manifest_directory.mkdir(exist_ok=True)
(manifest_directory/'clips-baseline-r33-before-manifest.json').write_text(json.dumps({'source':source_before,'provider':provider_before,'production':code_before,'oracles':oracles_before},ensure_ascii=False))
namespace['ModelCase'].setUpClass.__func__.__globals__['model_api']=lambda:api
classes=[value for name,value in namespace.items() if name.startswith('Bim') and isinstance(value,type) and issubclass(value,namespace['ModelCase'])]
suite=unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromTestCase(cls) for cls in classes)
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(namespace["BimPartitionOracleContract"]))
excluded={'test_each_chapter_has_unique_bilingual_narration_and_discoverable_scene','test_all_bilingual_clauses_fit_native_caption_layout_for_both_audiences'}
for name in unittest.defaultTestLoader.getTestCaseNames(namespace['BimClipContract']):
    if name not in excluded:
        suite.addTest(namespace['BimClipContract'](name))
chapter_namespace=runpy.run_path(str(CLIP/'🧪️tests'/'🧩️chapter-source'/'🐍️.py'),init_globals={'CAPTURED_MODEL':api.load_model()})
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(chapter_namespace['BimChapterSourceContract']))
progress=Path(__file__).parent/"🗑️generated"/"clips-baseline-r33-progress.json"
planned_tests=suite.countTestCases()
(progress.parent/(progress.stem+"-failures.json")).write_text(json.dumps({"failures":[],"errors":[]}))
class ProgressResult(unittest.TestResult):
    """⏱️ Retain exact per-test phases and durations for the portable cohort."""
    def startTest(self,test):
        super().startTest(test)
        self.phase=str(test)
        self.started=time.monotonic()
        progress=Path(__file__).parent/"🗑️generated"/"clips-baseline-r33-progress.json"
        progress.parent.mkdir(exist_ok=True)
        progress.write_text(json.dumps({"debug":"[DEBUG] portable baseline progress","test":self.phase,"phase":"started","completed":len(phases),"planned":planned_tests,"failures":len(self.failures),"errors":len(self.errors)},ensure_ascii=False),encoding="utf-8")
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
        progress=Path(__file__).parent/"🗑️generated"/"clips-baseline-r33-progress.json"
        progress.write_text(json.dumps({"debug":"[DEBUG] portable baseline progress","test":str(test),"phase":"completed","completed":len(phases),"planned":planned_tests,"failures":len(self.failures),"errors":len(self.errors),"seconds":phases[-1][1]},ensure_ascii=False),encoding="utf-8")
        super().stopTest(test)
phases=[]
result=ProgressResult()
suite.run(result)
source_after=hashlib.sha256(SOURCE.read_bytes()).hexdigest()
provider_after=hashlib.sha256(json.dumps(api.load_model(),sort_keys=True,separators=(',',':')).encode()).hexdigest()
print('SOURCE',len(api.load_model()['elements']),source_before,source_after,'STABLE',source_before==source_after)
print('PROVIDER_STABLE',provider_before,provider_after,provider_before==provider_after)
print('PLANNED',planned_tests)
print('RUN',result.testsRun,'FAIL',len(result.failures),'ERROR',len(result.errors))
for test,message in result.failures+result.errors:
    print(str(test),message)
code_after,oracles_after=production_hashes(),oracle_hashes()
(manifest_directory/'clips-baseline-r33-after-manifest.json').write_text(json.dumps({'source':source_after,'provider':provider_after,'production':code_after,'oracles':oracles_after},ensure_ascii=False))
summary={'scope':'core ModelCase, portable helper, four pure Clip and chapter contracts','planned':planned_tests,'tests':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),'seconds':sum(seconds for _,seconds in phases),'source_before':source_before,'source_after':source_after,'provider_before':provider_before,'provider_after':provider_after,'code_stable':code_before==code_after,'oracles_stable':oracles_before==oracles_after,'changed_code':[p for p in sorted(set(code_before)|set(code_after)) if code_before.get(p)!=code_after.get(p)],'changed_oracles':[p for p in sorted(set(oracles_before)|set(oracles_after)) if oracles_before.get(p)!=oracles_after.get(p)],'native_excluded':sorted(excluded),'native_frames':'unverified','phases':phases}
(progress.parent/'clips-baseline-r33-summary.json').write_text(json.dumps(summary,ensure_ascii=False))
print('CORE_STABILITY',summary['code_stable'],summary['oracles_stable'],summary['changed_code'],summary['changed_oracles'])
print('NATIVE_EXCLUDED',sorted(excluded))
