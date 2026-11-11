import importlib
from pathlib import Path
from manim import Camera,tempconfig
r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️')
with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_width':14.2222222222,'frame_height':8}):
 for lang in ('en','de'):
  scene=r.Beat4_Energy();scene.language=lambda:lang;scene._prepare()
  for rows in ((7,16),(6,16)):
   win=scene._window(scene.T,rows,'B14:B15','A:B');win.update(0)
   bar=win.formula_bar[1];bottom=bar.get_bottom()[1]
   print(lang,rows,'bar bottom',round(float(bottom),3),'row9 top',round(win.rect('B9')[1],3),'row10 top',round(win.rect('B10')[1],3),flush=True)
   camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
   camera.capture_mobjects([win]);camera.get_image().save(Path(__file__).parent/'🗑️generated'/f'128-rows-{lang}-{rows[0]}.png')
