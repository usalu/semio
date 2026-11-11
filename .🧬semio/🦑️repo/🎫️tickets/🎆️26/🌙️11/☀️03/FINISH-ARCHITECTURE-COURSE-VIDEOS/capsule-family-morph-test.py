"""🧪 Preserve static backgrounds and real vector morphs with refreshed live families."""
import importlib
import sys
import unittest
from pathlib import Path
import numpy as np
from manim import Scene,Square,UpdateFromAlphaFunc,VGroup,ValueTracker,tempconfig
sys.path.insert(0,str(Path.cwd()))
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
from live_family_prototype import RefreshMoving
class FreshScene(RefreshMoving,Scene): pass

class MorphTest(unittest.TestCase):
    def test_real_morph_live_and_static(self):
        with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
            scene=FreshScene()
            tracker=ValueTracker(0)
            key=lambda:tracker.get_value()>.5
            display=m._draft.Live(lambda:VGroup(Square(side_length=1,fill_color="#ff0000",fill_opacity=1,stroke_width=0).shift([2 if key() else -2,0,0])),key=key)
            background=Square(side_length=.5,fill_color="#00ff00",fill_opacity=1,stroke_width=0).shift([0,2,0])
            source=Square(side_length=.35,fill_color="#0000ff",fill_opacity=1,stroke_width=0).shift([-4,-2,0])
            target=Square(side_length=.35,fill_color="#0000ff",fill_opacity=1,stroke_width=0).shift([4,-2,0])
            scene.add(background,display,source)
            frames=[]
            original=scene.renderer.update_frame
            def update(*args,**kwargs):
                result=original(*args,**kwargs)
                frames.append((np.asarray(scene.camera.get_image()).copy(),key()))
                return result
            scene.renderer.update_frame=update
            scene.play(UpdateFromAlphaFunc(tracker,lambda obj,alpha:obj.set_value(alpha)),m._morph.morph(source,target,run_time=1),run_time=1)
            from PIL import Image
            Image.fromarray(frames[0][0]).save(Path(__file__).parent/"🗑️generated"/"capsule-family-morph-first.png")
            self.assertTrue(frames)
            self.assertGreater(len(frames),1)
            for frame,current in frames[1:]:
                h,w=frame.shape[:2]
                left=frame[h//2,round(w/2-2*w/scene.camera.frame_width),:3]
                right=frame[h//2,round(w/2+2*w/scene.camera.frame_width),:3]
                top=frame[round(h/2-2*h/scene.camera.frame_height),w//2,:3]
                self.assertGreater(int(top[1]),100)
                self.assertGreater(int((right if current else left)[0]),100,repr((current,left,right,top)))
                self.assertLess(int((left if current else right).max()),10)
            self.assertTrue(np.isfinite(target.points).all())

if __name__=="__main__": unittest.main()
