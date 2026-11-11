"""Detect ghosted previous keyed drawings in an actual moving Scene."""
import importlib,sys,unittest
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
import numpy as np
from manim import Scene,Square,VGroup,Group,ValueTracker,UpdateFromAlphaFunc,tempconfig
a=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🦴️anatomy.🐍️")
class GhostTest(unittest.TestCase):
 def test_old_key_does_not_remain_visible(self):
  with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
   scene=Scene(); tracker=ValueTracker(0); frames=[]
   key=lambda: tracker.get_value()>.5
   def build():
    return VGroup(Square(side_length=1,fill_color="#ff0000",fill_opacity=1,stroke_width=0).shift([2 if key() else -2,0,0]))
   display=a._keyed_snapshot(build,key,scene.camera)
   caption=Square(side_length=.5,fill_color="#00ff00",fill_opacity=1,stroke_width=0).shift([0,2,0])
   scene.add(display,caption)
   original=scene.renderer.update_frame
   def update(*args,**kwargs):
    result=original(*args,**kwargs);frames.append(np.asarray(scene.camera.get_image()).copy());return result
   scene.renderer.update_frame=update
   scene.play(UpdateFromAlphaFunc(tracker,lambda m,alpha:m.set_value(alpha)),run_time=1)
   frame=frames[-1];h,w=frame.shape[:2]; left=frame[h//2,round(w/2-2*w/scene.camera.frame_width),:3];right=frame[h//2,round(w/2+2*w/scene.camera.frame_width),:3]
   self.assertLess(int(left.max()),10,repr((left,right)))
   self.assertGreater(int(right.max()),100,repr((left,right)))
   for frame in frames:
    left=frame[h//2,round(w/2-2*w/scene.camera.frame_width),:3]
    right=frame[h//2,round(w/2+2*w/scene.camera.frame_width),:3]
    self.assertFalse(int(left.max())>100 and int(right.max())>100,repr((left,right)))
    top=frame[round(h/2-2*h/scene.camera.frame_height),w//2,:3]
    self.assertGreater(int(top[1]),100)
if __name__=="__main__":unittest.main()
