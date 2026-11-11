import importlib
from manim import tempconfig
r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️')
with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_width':14.2222222222,'frame_height':8}):
 scene=r.Beat4_Energy();scene.language=lambda:'en';scene._prepare()
 win=scene._window(scene.T,(6,16),'B14:B15','A:B')
 tiles=scene._annual_tiles((scene.dx,.5));plan=scene._plan((scene.dx+2.2,2.0))
 for name,m in (('window',win),('tiles',tiles),('plan',plan)):
  print(name,'bounds',round(float(m.get_left()[0]),3),round(float(m.get_right()[0]),3),round(float(m.get_bottom()[1]),3),round(float(m.get_top()[1]),3),flush=True)
  assert m.get_left()[0]>=-scene.camera.frame_width/2 and m.get_right()[0]<=scene.camera.frame_width/2
  assert m.get_bottom()[1]>=-scene.camera.frame_height/2 and m.get_top()[1]<=scene.camera.frame_height/2
