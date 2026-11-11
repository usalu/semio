"""⏳ Verify normal Manim waits refresh changing families without forced updates."""
import importlib
import sys
import unittest
from pathlib import Path
import numpy as np
from manim import Scene,Square,VGroup,ValueTracker,tempconfig
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
from live_family_prototype import RefreshMoving
class FreshScene(RefreshMoving,Scene): pass
class WaitTest(unittest.TestCase):
    def test_unchanging_wait_retains_static_behavior(self):
        with tempconfig({"dry_run":True,"pixel_width":160,"pixel_height":90,"frame_rate":6}):
            scene=FreshScene()
            scene.add(Square())
            calls=[]
            original=scene.renderer.update_frame
            def update(*args,**kwargs):
                calls.append(True)
                return original(*args,**kwargs)
            scene.renderer.update_frame=update
            scene.wait(1)
            self.assertLess(len(calls),6)

    def test_natural_wait_updates(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=FreshScene()
            tracker=ValueTracker(0)
            tracker.add_updater(lambda mob,dt:mob.increment_value(dt))
            key=lambda:tracker.get_value()>.5
            display=m._draft.Live(lambda:VGroup(Square(side_length=1,fill_color="#ff0000",fill_opacity=1,stroke_width=0).shift([2 if key() else -2,0,0])),key=key)
            background=Square(side_length=.5,fill_color="#00ff00",fill_opacity=1,stroke_width=0).shift([0,2,0])
            scene.add(tracker,background,display)
            frames=[]
            original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                frames.append((np.asarray(scene.camera.get_image()).copy(),key()))
                return result
            scene.renderer.update_frame=update
            scene.wait(1)
            self.assertGreater(len(frames),1)
            seen=set()
            for frame,current in frames[1:]:
                h,w=frame.shape[:2]
                left=frame[h//2,round(w/2-2*w/scene.camera.frame_width),:3]
                right=frame[h//2,round(w/2+2*w/scene.camera.frame_width),:3]
                top=frame[round(h/2-2*h/scene.camera.frame_height),w//2,:3]
                self.assertGreater(int(top[1]),100)
                self.assertGreater(int((right if current else left)[0]),100)
                self.assertLess(int((left if current else right).max()),10)
                seen.add(current)
            self.assertEqual(seen,{False,True})
if __name__=="__main__": unittest.main()
