import importlib
from pathlib import Path
from manim import Camera,tempconfig
r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️')
with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_width':14.2222222222,'frame_height':8}):
 for lang in ('en','de'):
  scene=r.Beat4_Energy();scene.language=lambda:lang;scene._prepare()
  win=scene._window(scene.T,(6,16),'B14','A:B');win.update(0)
  refs=scene._formula_refs(win,'B14');per_area=scene.xl_ref_frame(win,'B15',color=r.GOOD)
  camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
  camera.capture_mobjects([win,refs,per_area]);camera.get_image().save(Path(__file__).parent/'🗑️generated'/f'128-final-window-{lang}.png')
