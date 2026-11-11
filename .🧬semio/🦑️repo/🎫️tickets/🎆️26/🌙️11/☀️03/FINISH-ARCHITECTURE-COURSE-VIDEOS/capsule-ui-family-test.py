"""👻 Compare actual slider and changing panel frames with fresh Cairo references."""
import importlib
import os
import sys
import unittest
from pathlib import Path
import numpy as np
from PIL import Image
from manim import Camera, UpdateFromAlphaFunc, ValueTracker, tempconfig
sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
from live_family_prototype import RefreshMoving
class FreshCapsule(RefreshMoving,m.Beat2_Capsule): pass

class UiGhost(unittest.TestCase):
    def compare(self,kind):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=FreshCapsule()
            tracker=ValueTracker(1)
            if kind=="slider":
                widget=scene.gh_slider("Width",1,1,3,at=(0,0,0),tracker=tracker,width=3)
                def fresh():
                    return scene.gh_slider("Width",tracker.get_value(),1,3,at=(0,0,0),width=3)
            else:
                text=lambda:[str(int(round(tracker.get_value()*50)))]
                widget=scene.live(lambda:scene.gh_panel(text(),at=(0,0,0),width=2),key=lambda:tuple(text()))
                def fresh(): return scene.gh_panel(text(),at=(0,0,0),width=2)
            scene.add(widget)
            camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            differences=[]
            frames=[]
            original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                camera.reset()
                camera.capture_mobjects([fresh()])
                expected=np.asarray(camera.get_image(),dtype=int)
                differences.append(int(np.abs(actual-expected).max()))
                frames.append((actual,expected))
                return result
            scene.renderer.update_frame=update
            scene.play(UpdateFromAlphaFunc(tracker,lambda obj,alpha:obj.set_value(1+2*alpha)),run_time=1)
            if max(differences)>2:
                index=int(np.argmax(differences))
                folder=Path(__file__).parent/"🗑️generated"
                Image.fromarray(frames[index][0].astype("uint8")).save(folder/("capsule-"+kind+"-actual.png"))
                Image.fromarray(frames[index][1].astype("uint8")).save(folder/("capsule-"+kind+"-fresh.png"))
            self.assertLessEqual(max(differences),2,repr(differences))

    def test_slider_frames(self):
        self.compare("slider")

    def test_changing_panel_frames(self):
        self.compare("panel")

if __name__=="__main__":
    unittest.main()
