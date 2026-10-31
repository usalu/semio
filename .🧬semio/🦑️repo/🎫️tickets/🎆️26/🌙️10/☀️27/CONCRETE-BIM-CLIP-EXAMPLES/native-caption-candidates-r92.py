"""📏 Validate authored caption replacements with actual native font metrics."""
import importlib
import json
import os
from pathlib import Path
import sys
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
TICKET=Path(__file__).parent
OUT=TICKET/'🗑️generated/native-r92/candidates'
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker')
v=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️')
v._fonts.apply_body_font()
from manim import tempconfig
rows=json.loads((TICKET/'native-caption-repairs-r92.json').read_text(encoding='utf-8'))
result=[]
with tempconfig({'media_dir':str(OUT)}):
 for folder,old,new in rows:
  try:
   caption=v._art.caption_bar(new)
   result.append({'folder':folder,'text':new,'ok':True,'width':caption.width,'height':caption.height})
   print('[DEBUG] PASS '+new,flush=True)
  except Exception as error:
   result.append({'folder':folder,'text':new,'ok':False,'error':str(error)})
   print('[DEBUG] FAIL '+str(error),flush=True)
(OUT/'result.json').write_text(json.dumps(result,indent=2),encoding='utf-8')
raise SystemExit(0 if all(r['ok'] for r in result) else 1)
