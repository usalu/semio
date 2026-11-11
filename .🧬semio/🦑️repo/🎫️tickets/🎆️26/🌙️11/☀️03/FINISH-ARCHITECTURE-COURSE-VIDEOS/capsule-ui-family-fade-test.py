"""🧪 Concurrent widget reveal and tracker motion against fresh Cairo."""
import importlib
import os
import sys
import unittest
from pathlib import Path
import numpy as np
from manim import Camera, FadeIn, Scene, UpdateFromAlphaFunc, ValueTracker, tempconfig
sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
from live_family_prototype import RefreshMoving
class FreshCapsule(RefreshMoving,m.Beat2_Capsule): pass

class FadeTest(unittest.TestCase):
    def compare(self,kind):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=FreshCapsule()
            tracker=ValueTracker(1)
            def fresh():
                if kind=="slider":
                    return scene.gh_slider("Width",tracker.get_value(),1,3,at=(0,0,0),width=3)
                return scene.gh_panel([str(int(round(tracker.get_value()*50)))],at=(0,0,0),width=2)
            widget=scene.gh_slider("Width",1,1,3,at=(0,0,0),tracker=tracker,width=3) if kind=="slider" else scene.live(fresh,key=lambda:int(round(tracker.get_value()*50)))
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            differences=[]
            original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                expected_widget=fresh()
                fade=FadeIn(expected_widget,run_time=1,rate_func=lambda value:value)
                fade.begin()
                fade.interpolate((tracker.get_value()-1)/2)
                oracle.reset()
                oracle.capture_mobjects([expected_widget])
                expected=np.asarray(oracle.get_image(),dtype=int)
                differences.append(int(np.abs(actual-expected).max()))
                return result
            scene.renderer.update_frame=update
            scene.play(FadeIn(widget,run_time=1),UpdateFromAlphaFunc(tracker,lambda obj,alpha:obj.set_value(1+2*alpha)),run_time=1)
            self.assertLessEqual(max(differences),2,repr(differences))

    def test_slider(self): self.compare("slider")
    def test_panel(self): self.compare("panel")

if __name__=="__main__": unittest.main()
