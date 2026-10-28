"""🔤 Inspect native SVG baseline and em sizing for CSS line-box parity."""
import importlib,json,os,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(root))
from manim import config
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']);output.mkdir(parents=True,exist_ok=True)
config.text_dir=str(output/'t')
f=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️')
f.register_fonts();f.apply_body_font()
for value in ['Physikalische Grundlagen','Lüftung','Heizen','Ágy','semio']:
 t=f.body_text(value,font_size=11.2,color='#f7f3e3')
 print(json.dumps({'value':value,'width':t.width,'height':t.height,'size':t.font_size,'file':str(t.file_name),'svg':Path(t.file_name).read_text(encoding='utf8')[:2200]},ensure_ascii=False))

import inspect
from manim import Camera,Text
print(inspect.getsource(Camera.apply_stroke))
print(inspect.getsource(Text.__init__))
print(inspect.getsource(Text._text2svg))

p=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️')
r=p._interaction_render
logo=r._brand()
print('[DEBUG] Logo',[(str(m.get_fill_color()),str(m.get_stroke_color()),m.get_stroke_width()) for m in logo.family_members_with_points()])
