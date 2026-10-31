"""🔠 Exercise native class-column glyph bounds in the ticket output tree."""
import os,sys,unittest,json,time
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'package.json').is_file())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
OUT=Path(__file__).parent/'🗑️generated/native-r92/labels'
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT))
suite=unittest.defaultTestLoader.loadTestsFromName('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️.BimClipContract.test_model_class_labels_keep_separate_native_text_columns')
start=time.perf_counter()
result=unittest.TextTestRunner(verbosity=2).run(suite)
(OUT/'result.json').write_text(json.dumps({'success':result.wasSuccessful(),'run':result.testsRun,'seconds':time.perf_counter()-start,'failures':[(str(t),e) for t,e in result.failures],'errors':[(str(t),e) for t,e in result.errors]},indent=2),encoding='utf-8')
raise SystemExit(0 if result.wasSuccessful() else 1)
