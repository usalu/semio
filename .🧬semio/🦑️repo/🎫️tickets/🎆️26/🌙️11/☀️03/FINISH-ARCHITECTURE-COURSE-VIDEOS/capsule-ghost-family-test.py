"""👻 Compare actual Manim frames of capsule live groups with fresh Cairo oracles."""
import importlib
import json
import os
import sys
import unittest
from pathlib import Path
import numpy as np
from PIL import Image
from manim import Camera, Group, Scene, Square, UpdateFromAlphaFunc, VGroup, ValueTracker, tempconfig
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
from live_family_prototype import RefreshMoving
class FreshScene(RefreshMoving,Scene): pass
case=json.loads(Path(__file__).with_name("capsule-stage-cases.json").read_text(encoding="utf-8"))["ghost"]

class MixedOld(Group):
    def __init__(self,builder,key):
        self.builder,self.key,self.seen=builder,key,key()
        super().__init__(*builder().submobjects)
        self.add_updater(lambda mob:mob.refresh())
    def refresh(self):
        value=self.key()
        if value!=self.seen:
            self.seen=value
            self.submobjects=self.builder().submobjects
        return self
def mixed_old(scene,shape,key):
    return MixedOld(lambda:Group(m._sprites.capture(shape(),scene.camera)),key)

class GhostTest(unittest.TestCase):
    def compare(self,mixed):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=FreshScene()
            tracker=ValueTracker(0)
            key=lambda:tracker.get_value()>case["threshold"]
            shape=lambda:Square(side_length=1,fill_color="#ff0000",fill_opacity=1,stroke_width=0).shift([case["positions"][1] if key() else case["positions"][0],0,0])
            caption=Square(side_length=.5,fill_color="#00ff00",fill_opacity=1,stroke_width=0).shift([0,2,0])
            if mixed:
                display=m._draft.Live(lambda:VGroup(shape()),key) if False else mixed_old(scene,shape,key)
            else:
                display=m._draft.Live(lambda:VGroup(shape()),key)
            scene.add(display,caption)
            oracle=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
            differences=[]
            actuals=[]
            original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
                oracle.reset()
                oracle.capture_mobjects([shape(),caption])
                expected=np.asarray(oracle.get_image(),dtype=int)
                differences.append(int(np.abs(actual-expected).max()))
                actuals.append((actual,expected))
                return result
            scene.renderer.update_frame=update
            scene.play(UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(alpha)),run_time=1)
            self.assertTrue(differences)
            if max(differences)>2:
                i=int(np.argmax(differences))
                out=Path(__file__).parent/"🗑️generated"
                Image.fromarray(actuals[i][0].astype("uint8")).save(out/("mixed-ghost-actual.png" if mixed else "vector-ghost-actual.png"))
                Image.fromarray(actuals[i][1].astype("uint8")).save(out/("mixed-ghost-fresh.png" if mixed else "vector-ghost-fresh.png"))
            self.assertLessEqual(max(differences),case["maxPixelDifference"],repr(differences))

    def test_mixed_wrap_frames(self):
        self.compare(True)

    def test_vector_stage_frames(self):
        self.compare(False)

if __name__=="__main__":
    unittest.main()
