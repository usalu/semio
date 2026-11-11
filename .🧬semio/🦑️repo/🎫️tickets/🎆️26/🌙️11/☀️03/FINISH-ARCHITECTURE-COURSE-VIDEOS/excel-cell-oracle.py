import importlib,sys
from pathlib import Path
from types import SimpleNamespace
import numpy as np
from PIL import Image
from manim import Camera,ValueTracker,UpdateFromAlphaFunc,tempconfig,VGroup
sys.path.insert(0,str(Path.cwd()))
a=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🦴️anatomy.🐍️')
with tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180,'frame_rate':6,'background_color':'#000000','disable_caching':True}):
 scene=a.Beat1_Anatomy(); scene.language=lambda:'en'; tracker=ValueTracker(2.3); scene.s=SimpleNamespace(iw=tracker,status=None)
 book=a._xl.workbook(language='en'); window=scene._window(book,selected='B5'); fn=lambda:format(a._interior(round(tracker.get_value(),3))['B5'],'.2f')+' m'
 window.bind('B5',fn); holder=window.cell('B5'); scene.add(holder)
 oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color='#000000',background_opacity=1)
 diffs=[]; original=scene.renderer.update_frame; out=Path(__file__).parent/'🗑️generated'
 def update(*args,**kwargs):
  result=original(*args,**kwargs); actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
  oracle.reset(); oracle.capture_mobjects([VGroup(window._cell_label('B5',fn(),'r'))]); expected=np.asarray(oracle.get_image(),dtype=int)
  diff=int(np.abs(actual-expected).max()); diffs.append(diff)
  if diff>2 and not (out/'excel125-cell-actual.png').exists():
   Image.fromarray(actual.astype('uint8')).save(out/'excel125-cell-actual.png'); Image.fromarray(expected.astype('uint8')).save(out/'excel125-cell-fresh.png')
  return result
 scene.renderer.update_frame=update; scene.play(UpdateFromAlphaFunc(tracker,lambda m,x:m.set_value(2.3+.2*x)),run_time=1)
 print('B5 bound cell frames',len(diffs),'max',max(diffs),'diffs',diffs,flush=True)
