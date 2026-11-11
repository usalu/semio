"""Observe Boolean toggle and window pixels through simultaneous reveal."""
import importlib,sys,unittest,json
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
import numpy as np
from manim import Scene,Square,VGroup,ValueTracker,UpdateFromAlphaFunc,FadeIn,tempconfig
casting=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🫗️casting.🐍️")
draft=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.🐍️")
case=json.loads(Path(__file__).with_name("timing-fixture.json").read_text(encoding="utf-8"))
class BooleanTimingTest(unittest.TestCase):
    def test_widget_and_window_track_same_state_through_reveal(self):
        with tempconfig({"dry_run":True,"pixel_width":160,"pixel_height":90,"frame_rate":10}):
            scene=Scene(); tracker=ValueTracker(1); values=[]
            on=lambda:tracker.get_value()>.5
            def build():
                return VGroup(Square(fill_color="#ff0000" if on() else "#0000ff",fill_opacity=1,stroke_width=0))
            widget=draft.Live(build,key=on)
            canvas=VGroup(widget)
            image=casting._window_display(build,on,scene.camera)
            observer=Square(side_length=.01,fill_opacity=0,stroke_width=0)
            def observe(_):
                rgb=np.asarray(widget.current[0].get_fill_color().to_rgb())
                pixels=image[0].pixel_array
                pixel=pixels[pixels.shape[0]//2,pixels.shape[1]//2,:3]
                if pixel.max()>15:
                    values.append((bool(rgb[0]>rgb[2]),bool(pixel[0]>pixel[2])))
            observer.add_updater(observe); scene.add(observer)
            flip=UpdateFromAlphaFunc(tracker,lambda mob,alpha:mob.set_value(0 if case["off_start"]<alpha<case["off_end"] else 1),run_time=case["duration"])
            scene.play(FadeIn(canvas),FadeIn(image),run_time=1)
            scene.play(flip,run_time=case["duration"])
            mismatches=[pair for pair in values if pair[0]!=pair[1]]
            self.assertGreater(len(values),0)
            self.assertEqual(mismatches,[])
if __name__=="__main__":unittest.main()
