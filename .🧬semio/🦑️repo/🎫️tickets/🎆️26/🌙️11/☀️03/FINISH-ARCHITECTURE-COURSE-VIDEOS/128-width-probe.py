import importlib
from pathlib import Path
from manim import Camera,tempconfig
r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️')
with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_width':14.2222222222,'frame_height':8}):
 for lang in ('en','de'):
  scene=r.Beat4_Energy();scene.language=lambda:lang;scene._prepare()
  for cols in ('A:B','A:B+'):
   for cell in ('B12','B13','B14:B15'):
    win=scene._window(scene.T,(8,16),cell,cols);win.update(0)
    bar=win.formula_bar[1]
    family=[m for m in bar.get_family() if hasattr(m,'lines')]
    print(lang,cols,cell,'lines', [x.lines for x in family], 'grid_top',round(win.grid_top,3), 'bar_bottom',round(float(bar.get_bottom()[1]),3),'dx',round(scene.dx,3),flush=True)
    camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
    camera.capture_mobjects([win]);camera.get_image().save(Path(__file__).parent/'🗑️generated'/f"128-width-{lang}-{cols.replace(':','').replace('+','plus')}-{cell[:3]}.png")
