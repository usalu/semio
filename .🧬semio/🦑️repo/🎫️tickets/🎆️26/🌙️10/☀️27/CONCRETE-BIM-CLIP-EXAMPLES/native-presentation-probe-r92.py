"""🪟 Verify real browser transitions and ffmpeg publication on an explicitly diagnostic short clip."""
import importlib
import json
import os
from pathlib import Path
import shutil
import sys
import time

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
TICKET=Path(__file__).parent
OUT=TICKET/'🗑️generated/native-r92/presentation-probe'
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT))
runtime=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
source=TICKET/'🗑️generated/native-r92/preview/en/speaker/s/114/p/Beat1_DataToInformation/uncached_00000.mp4'
content=OUT/'diagnostic-content.mp4'
shutil.copyfile(source,content)
content.with_suffix('.render.json').write_text(json.dumps({'scene':'BIM_01_Information','diagnostic':True,'source_partial':str(source)}),encoding='utf-8')
settings={'language':'en','audience':'speaker','quality':'-ql','resolution':'480,270','fps':4.}
session=runtime.ToolSession()
start=time.monotonic()
try:
 result=runtime.render_presentation('bim_information',content,OUT/'diagnostic-presentation.mp4',settings,session=session)
 (OUT/'result.json').write_text(json.dumps({'diagnostic':True,'success':True,'source':str(source),'output':str(result),'seconds':time.monotonic()-start,'bytes':result.stat().st_size}),encoding='utf-8')
 print('[DEBUG] Native diagnostic presentation PASS',flush=True)
finally:
 session.cancel()
     