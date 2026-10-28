"""🧪 Preserve final UI, media and publication verification results across tool reconnections."""
import json,os,sys,time,unittest
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT));sys.dont_write_bytecode=True
output=Path(__file__).with_name('🗑️generated')/'final-tests';output.mkdir(parents=True,exist_ok=True)
os.environ['SEMIO_TEACHING_OUTPUT_ROOT']=str(output)
os.environ['SEMIO_TEACHING_CACHE_DIR']=str(output/'fonts')
package='🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation'
names=[package+'.🥞️hierarchy.🧪️tests.🐍️',package+'.🌐️browser.🧪️tests.🐍️',package+'.🧪️tests.🐍️.PresentationContractTest',package+'.🧪️tests.🐍️.PresentationNativeTest']
started=time.monotonic()
with (output/'run.log').open('w',encoding='utf8') as stream:
    suite=unittest.defaultTestLoader.loadTestsFromNames(names)
    result=unittest.TextTestRunner(stream=stream,verbosity=2).run(suite)
summary={'passed':result.wasSuccessful(),'tests':result.testsRun,'seconds':time.monotonic()-started,'failures':[(str(t),e) for t,e in result.failures],'errors':[(str(t),e) for t,e in result.errors]}
(output/'result.json').write_text(json.dumps(summary,ensure_ascii=False),encoding='utf8')
print('[DEBUG] Final scoped tests '+json.dumps(summary,ensure_ascii=False),flush=True)
raise SystemExit(not result.wasSuccessful())
