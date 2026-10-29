"""🧪 Main pure BIM baseline and callback/bilingual proofs for one recorded source cohort."""
import hashlib
import runpy
import unittest
from pathlib import Path

CLIP=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip')
SOURCE=CLIP/'🧱️model'/'🔣️.json'
source_before=hashlib.sha256(SOURCE.read_bytes()).hexdigest()
namespace=runpy.run_path(str(CLIP/'🧪️tests'/'🐍️.py'))
api=namespace['model_api']()
namespace['ModelCase'].setUpClass.__func__.__globals__['model_api']=lambda:api
classes=[value for name,value in namespace.items() if name.startswith('Bim') and isinstance(value,type) and issubclass(value,namespace['ModelCase'])]
suite=unittest.TestSuite(unittest.defaultTestLoader.loadTestsFromTestCase(cls) for cls in classes)
excluded={'test_each_chapter_has_unique_bilingual_narration_and_discoverable_scene','test_all_bilingual_clauses_fit_native_caption_layout_for_both_audiences'}
for name in unittest.defaultTestLoader.getTestCaseNames(namespace['BimClipContract']):
    if name not in excluded:
        suite.addTest(namespace['BimClipContract'](name))
chapter_namespace=runpy.run_path(str(CLIP/'🧪️tests'/'🧩️chapter-source'/'🐍️.py'))
suite.addTests(unittest.defaultTestLoader.loadTestsFromTestCase(chapter_namespace['BimChapterSourceContract']))
result=unittest.TestResult()
suite.run(result)
source_after=hashlib.sha256(SOURCE.read_bytes()).hexdigest()
print('SOURCE',len(api.load_model()['elements']),source_before,source_after,'STABLE',source_before==source_after)
print('RUN',result.testsRun,'FAIL',len(result.failures),'ERROR',len(result.errors))
for test,message in result.failures+result.errors:
    print(str(test),message)
print('NATIVE_EXCLUDED',sorted(excluded))
