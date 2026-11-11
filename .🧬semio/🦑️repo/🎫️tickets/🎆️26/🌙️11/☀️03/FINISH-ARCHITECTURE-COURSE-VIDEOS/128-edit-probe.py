import ast,importlib,json
from pathlib import Path
import numpy as np
from manim import Camera,tempconfig
r=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.⚡️energy.🐍️')
source=ast.parse(Path(r.__file__).read_text(encoding='utf-8'))
cls=next(n for n in source.body if isinstance(n,ast.ClassDef) and n.name=='Beat4_Energy')
method=next(n for n in cls.body if isinstance(n,ast.FunctionDef) and any(isinstance(x,ast.Call) and isinstance(x.func,ast.Attribute) and x.func.attr=='step' and x.args and isinstance(x.args[0],ast.Constant) and x.args[0].value=='volume' for x in ast.walk(n)))
out=Path(__file__).parent/'🗑️generated'
with tempconfig({'dry_run':True,'pixel_width':854,'pixel_height':480,'frame_rate':3}):
 for lang in ('en','de'):
  scene=r.Beat4_Energy();scene.language=lambda:lang;scene._prepare()
  for address in ('B12','B13','B14:B15'):
   assignment=next(n for n in method.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='new' for t in n.targets) and isinstance(n.value,ast.Call) and any(isinstance(a,ast.Constant) and a.value==address for a in n.value.args))
   edit=next(n for n in method.body if isinstance(n,ast.Expr) and n.lineno>assignment.lineno and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=='edit' and isinstance(n.value.func.value,ast.Name) and n.value.func.value.id=='new')
   env={'self':scene,'T':scene.T}
   exec(compile(ast.fix_missing_locations(ast.Module(body=[assignment,edit],type_ignores=[])),r.__file__,'exec'),env)
   win=env['new'];win.update(0)
   overlay=win._edit_overlay()
   camera=Camera(pixel_width=854,pixel_height=480,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
   camera.capture_mobjects([win]);a=np.asarray(camera.get_image()).copy()
   if lang=='en':camera.get_image().save(out/f'128-edit-{address[:3]}.png')
   win.edit(None);win.update(0);camera.reset();camera.capture_mobjects([win]);b=np.asarray(camera.get_image()).copy()
   if lang=='en':camera.get_image().save(out/f'128-committed-{address[:3]}.png')
   left,top,right,bottom=win.rect(address.split(':')[0]);fw=scene.camera.frame_width;fh=scene.camera.frame_height
   x0=int((left/fw+.5)*854)+4;x1=int((right/fw+.5)*854)-4;y0=int((.5-top/fh)*480)+4;y1=int((.5-bottom/fh)*480)-4
   print(lang,address,'overlay_parts',len(overlay.submobjects),'selected_delta',int(np.abs(a[y0:y1,x0:x1].astype(int)-b[y0:y1,x0:x1].astype(int)).max()),'value',scene._show(scene.T,address.split(':')[0]),flush=True)
