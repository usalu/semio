"""🔎 Reproduce the exact caption trial independently of running render media."""
from importlib import import_module
import json
from pathlib import Path
import sys
import xml.etree.ElementTree as ET
TICKET=Path(__file__).resolve().parent
ROOT=Path('C:/git/semio')
sys.path.insert(0,str(ROOT))
from manim import Text,tempconfig
fonts=import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️')
fonts.apply_body_font()
output=TICKET/'🗑️generated/glyph-failure-probe'
results=[]
with tempconfig({'media_dir':str(output),'text_dir':str(output/'t')}):
    for text in ('volle Lüftungsverlust von 1500 W bleibt, η','volle Lüftungsverlust von 1500 W bleibt,','η','Der volle Lüftungsverlust von 1500 W bleibt, η'):
        try:
            obj=fonts.body_text(text,font_size=24,color="#FFFFFF")
            results.append({'text':text,'font':fonts.BODY_FONT,'characters':len(text.replace(' ','')),'submobjects':len(obj.submobjects),'svg':str(obj.file_name),'fallbacks':obj.t2f,'ok':True})
        except Exception as error:
            results.append({'text':text,'font':fonts.BODY_FONT,'ok':False,'error':repr(error)})
output.mkdir(parents=True,exist_ok=True)
(output/'result.json').write_text(json.dumps(results,ensure_ascii=False,indent=2),encoding='utf-8')
print('[DEBUG] '+json.dumps(results,ensure_ascii=True))
