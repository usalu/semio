import importlib,json
from pathlib import Path
from manim import Camera,tempconfig
r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.📍️references.🐍️')
case=json.loads(Path(__file__).with_name('126-roof-cases.json').read_text(encoding='utf-8'))
with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_width':14.2222222222,'frame_height':8}):
 for lang in case['languages']:
  scene=r.Beat2_References();scene.language=lambda:lang
  win=scene._window(r._xl.workbook(),r.SHEET,'A:D',(1,10),'D3',language=lang)
  labels=[scene._say(case['labels'][lang][0],(-2.85,3.1),size=r.SMALL,color=r.MUTED),scene._say(case['labels'][lang][1],(4,3.1),size=r.SMALL,color=r.MUTED)]
  camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
  camera.capture_mobjects([win,*labels])
  camera.get_image().save(Path(__file__).parent/'🗑️generated'/f'126-basis-{lang}.png')
  print(lang,[(round(float(x.get_left()[0]),2),round(float(x.get_right()[0]),2),round(float(x.get_bottom()[1]),2)) for x in labels],flush=True)
