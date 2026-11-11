"""Check four eagerly built tree swaps for premature visibility."""
import importlib,sys,unittest,json
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
import numpy as np
from types import SimpleNamespace
from manim import Scene,Square,Succession,LaggedStart,FadeIn,FadeOut,tempconfig
trees=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🌳️trees.🐍️")
case=json.loads(Path(__file__).with_name("path-timing-fixture.json").read_text(encoding="utf-8"))
class PathTiming(unittest.TestCase):
 def test_future_stages_are_not_visible_before_their_turn(self):
  with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":4,"background_color":"#000000"}):
   scene=Scene(); frames=[]
   shapes=[Square(side_length=1,fill_color=c,fill_opacity=1,stroke_width=0).shift([x,0,0]) for x,c in zip(case["positions"],case["colors"])]
   holder=SimpleNamespace(_slots={"diagram":shapes[0]})
   scene.add(shapes[0])
   stages=[lambda item=item:trees.Beat5_DataTrees._swap(holder,"diagram",item) for item in shapes[1:]]
   original=scene.renderer.update_frame
   def update(*args,**kwargs):
    result=original(*args,**kwargs)
    frames.append(np.asarray(scene.camera.get_image()).copy())
    return result
   scene.renderer.update_frame=update
   trees._sequence(scene,stages,3)
   self.assertTrue(frames)
   frame=frames[0]; h,w=frame.shape[:2]
   samples=[frame[h//2,round(w/2+x*w/scene.camera.frame_width),:3] for x in case["positions"]]
   self.assertGreater(int(samples[0].max()),100)
   self.assertTrue(all(int(sample.max())<10 for sample in samples[1:]),repr(samples))
   self.assertIs(holder._slots["diagram"],shapes[-1])
   self.assertAlmostEqual(scene.time,3)
   for stage,index in enumerate((0,4,8)):
    frame=frames[index]
    remaining=case["positions"][stage+2:]
    for x in remaining:
     pixel=frame[h//2,round(w/2+x*w/scene.camera.frame_width),:3]
     self.assertLess(int(pixel.max()),10)
 def test_staggered_recap_rows_do_not_preappear(self):
  with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":4,"background_color":"#000000"}):
   scene=Scene();frames=[]
   rows=[Square(side_length=1,fill_color=color,fill_opacity=1,stroke_width=0).shift([x,0,0]) for x,color in zip(case["positions"],case["colors"])]
   flights=[Square(side_length=.1,fill_opacity=1).shift([x,-2,0]) for x in case["positions"]]
   original=scene.renderer.update_frame
   def update(*args,**kwargs):
    result=original(*args,**kwargs);frames.append(np.asarray(scene.camera.get_image()).copy());return result
   scene.renderer.update_frame=update
   scene.play(LaggedStart(*[Succession(FadeOut(flight),FadeIn(row)) for flight,row in zip(flights,rows)],lag_ratio=.18),run_time=3)
   frame=frames[0];h,w=frame.shape[:2]
   samples=[frame[h//2,round(w/2+x*w/scene.camera.frame_width),:3] for x in case["positions"]]
   self.assertTrue(all(int(sample.max())<10 for sample in samples),repr(samples))
 def test_failure_does_not_build_later_stages(self):
  calls=[]
  class Fake:
   def play(self,*args,**kwargs): pass
  def first():
   calls.append(1)
   raise RuntimeError("cancel")
  def later():
   calls.append(2)
  with self.assertRaisesRegex(RuntimeError,"cancel"):
   trees._sequence(Fake(),[first,later],2)
  self.assertEqual(calls,[1])
if __name__=="__main__":unittest.main()
