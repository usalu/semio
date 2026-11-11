"""🧪 Validate scene-family removal with later translucent objects."""
import sys
import unittest
from pathlib import Path
import numpy as np
from manim import Camera,Rectangle,Scene,Square,UpdateFromAlphaFunc,ValueTracker,tempconfig
sys.path.insert(0,str(Path.cwd()))
from live_family_prototype import RefreshMoving
class FreshScene(RefreshMoving,Scene): pass
class BoundaryTest(unittest.TestCase):
    def test_removed_earliest_moving_root(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=FreshScene()
            tracker=ValueTracker(0)
            background=Rectangle(width=6,height=3,fill_color="#345678",fill_opacity=1,stroke_width=0)
            moving=Square(side_length=1,fill_color="#ff0000",fill_opacity=1,stroke_width=0).shift([-2,0,0])
            moving.add_updater(lambda mob:scene.remove(mob) if tracker.get_value()>.5 else None)
            later=Square(side_length=1,fill_color="#00ff00",fill_opacity=.5,stroke_width=0).shift([2,0,0])
            opaque=Square(side_length=.5,fill_color="#0000ff",fill_opacity=1,stroke_width=0).shift([0,1,0])
            scene.add(background,moving,later,opaque)
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            samples=[]
            original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                oracle.reset()
                reference=[background]+([moving] if moving in scene.mobjects else [])+[later,opaque]
                oracle.capture_mobjects(reference)
                samples.append(int(np.abs(actual-np.asarray(oracle.get_image(),dtype=int)).max()))
                return result
            scene.renderer.update_frame=update
            scene.play(UpdateFromAlphaFunc(tracker,lambda obj,alpha:obj.set_value(alpha)),run_time=1)
            self.assertGreater(len(samples),1)
            self.assertLessEqual(max(samples[1:]),2,repr(samples))
if __name__=="__main__": unittest.main()
