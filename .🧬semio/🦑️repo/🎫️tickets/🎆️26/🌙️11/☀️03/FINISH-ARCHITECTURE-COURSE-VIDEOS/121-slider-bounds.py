import importlib,json,os
import jsonschema
from pathlib import Path
from types import SimpleNamespace
from manim import ValueTracker,tempconfig
from shapely.geometry import box
c=importlib.import_module('🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️')
fixture=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf-8'))
jsonschema.validate(fixture,json.loads(Path(__file__).with_name('121-slider-bounds.schema.json').read_text(encoding='utf-8')))
shift=float(os.environ.get('SEMIO_TEST_SLIDER_SHIFT','0'))
errors=[]
with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_width':fixture['frame_width'],'frame_height':fixture['frame_height']}):
 for lang in fixture['languages']:
  scene=c.Beat2_Capsule(); scene.language=lambda:lang
  scene.st=SimpleNamespace(vals={k:ValueTracker(1) for k in fixture['ids']},wv={},cv={})
  defn=SimpleNamespace(spec=lambda k:SimpleNamespace(kind='slider'),obj=lambda k:SimpleNamespace(settings={'value':1,'min':0,'max':4}))
  viewport=box(-scene.camera.frame_width/2+fixture['minimum_margin'],-scene.camera.frame_height/2+fixture['minimum_margin'],scene.camera.frame_width/2-fixture['minimum_margin'],scene.camera.frame_height/2-fixture['minimum_margin'])
  for key in fixture['ids']:
   at=c.SLIDER_AT[key] if key in c.SLIDER_AT else (-5.05,1.9-.8*c.BOLT_IDS.index(key))
   widget=scene._node(key,(*at,0),defn,lambda:None).view
   widget.shift([shift,0,0])
   bounds=(float(widget.get_left()[0]),float(widget.get_bottom()[1]),float(widget.get_right()[0]),float(widget.get_top()[1]))
   ok=viewport.covers(box(*bounds))
   print(lang,key,'bounds',tuple(round(x,3) for x in bounds),'contained',ok,flush=True)
   if not ok:errors.append((lang,key,bounds))
if errors:raise AssertionError('viewport overflow: '+repr(errors))
