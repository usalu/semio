"""💬 Diagnose the actual native first caption failure."""
import ast
import importlib
import os
from pathlib import Path
import sys
ROOT = next(p for p in Path(__file__).resolve().parents if (p / '🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
os.environ.update(SEMIO_TEACHING_LANGUAGE='en', SEMIO_TEACHING_AUDIENCE='speaker')
v=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️')
v._fonts.apply_body_font()
from manim import tempconfig
p=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py'
t=ast.parse(p.read_text(encoding='utf-8'))
c=next(n for n in t.body if isinstance(n,ast.ClassDef))
a=next(ast.literal_eval(n.value) for n in c.body if isinstance(n,ast.Assign) and any(isinstance(x,ast.Name) and x.id=='NARRATION' for x in n.targets))
with tempconfig({'media_dir':str(Path(__file__).parent/'🗑️generated/native-r92/caption')}):
 v._art.caption_bar(next(x[1] for x in a if x[0]=='pinch'))
