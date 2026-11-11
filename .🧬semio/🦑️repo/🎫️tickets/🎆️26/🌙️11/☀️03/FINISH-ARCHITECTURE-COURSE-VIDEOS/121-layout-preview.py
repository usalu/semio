import importlib
from pathlib import Path
from types import SimpleNamespace
from manim import Camera,ValueTracker,tempconfig
m=importlib.import_module('🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️')
with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_width':14.222222222222221,'frame_height':8}):
 for lang in ('en','de'):
  scene=m.Beat2_Capsule();scene.language=lambda:lang
  scene.st=SimpleNamespace(vals={k:ValueTracker(1) for k in m.IDS},wv={},cv={})
  d=SimpleNamespace(spec=lambda k:SimpleNamespace(kind='slider'),obj=lambda k:SimpleNamespace(settings={'value':1,'min':0,'max':4}))
  rows=[(scene.tr(*pair),key) for key,pair in (('w',('Width','Breite')),('l',('Length','Länge')),('h',('Height','Höhe')),('r',('Corner radius','Eckenradius')),('d',('Window diameter','Fensterdurchmesser')),('wu',('Window offset','Fensterversatz')))]
  controls=[scene._node(k,(*m.SLIDER_AT[k],0),d,lambda:None).view for k in m.IDS]
  camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
  camera.capture_mobjects([*controls,scene._params_card(rows,m.PARAMS_AT)])
  camera.get_image().save(Path(__file__).parent/'🗑️generated'/f'121-layout-{lang}.png')
