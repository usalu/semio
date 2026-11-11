"""Validate source122 display-only tower snapshots and semantic originals."""
import importlib
import sys
import unittest
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
import numpy as np
from manim import Camera, FadeIn, FadeOut, Group, ImageMobject, Scene, Square, VGroup, tempconfig

casting=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🫗️casting.🐍️")
tower=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🗼️tower.🐍️")

class TowerPresentationTest(unittest.TestCase):
    def test_scene_display_keeps_vector_and_overlay(self):
        camera=Camera(pixel_width=320,pixel_height=180,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
        capsule=Square(side_length=1,fill_color="#3999cc",fill_opacity=1,stroke_width=0)
        vector=VGroup(capsule); vector.capsules={(0,0,0,0,0):capsule}
        axes=Square(side_length=.4,stroke_color="#ffcc00",stroke_width=3).shift([1,1,0])
        original=capsule.points.copy()
        camera.capture_mobjects([vector,axes]); baseline=np.asarray(camera.get_image(),dtype=int).copy()
        display=tower._tower_display(vector,camera,axes)
        self.assertIsInstance(display,Group)
        self.assertIsInstance(display[0],ImageMobject)
        self.assertIs(display[1],axes)
        self.assertIs(vector.capsules[(0,0,0,0,0)],capsule)
        self.assertTrue(np.array_equal(capsule.points,original))
        camera.reset(); camera.capture_mobjects([display])
        self.assertLessEqual(int(np.abs(baseline-np.asarray(camera.get_image(),dtype=int)).max()),2)

    def test_scene_fade_cleanup(self):
        with tempconfig({"dry_run":True,"pixel_width":160,"pixel_height":90,"frame_rate":4}):
            scene=Scene()
            vector=VGroup(Square(fill_opacity=1))
            axes=Square(side_length=.3).shift([1,1,0])
            display=tower._tower_display(vector,scene.camera,axes)
            scene.play(FadeIn(display),run_time=.5)
            self.assertIn(display,scene.mobjects)
            scene.play(FadeOut(display),run_time=.5)
            self.assertNotIn(display,scene.mobjects)
            self.assertEqual(len(vector),1)


class CastingPresentationTest(unittest.TestCase):
    def test_window_states_cache_and_copy(self):
        state={"window":False}
        calls=[]
        camera=Camera(pixel_width=160,pixel_height=90,frame_width=14.222222222222221,frame_height=8,background_opacity=1)
        def build():
            calls.append(state["window"])
            return VGroup(Square(fill_color="#ff0000" if state["window"] else "#0000ff",fill_opacity=1,stroke_width=0))
        display=casting._window_display(build,lambda:state["window"],camera)
        self.assertEqual(calls,[False])
        display.update(0)
        state["window"]=True; display.update(0)
        self.assertEqual(calls,[False,True])
        state["window"]=False; display.update(0)
        self.assertEqual(calls,[False,True])
        clone=display.copy()
        clone[0].set_opacity(.2)
        self.assertNotEqual(int(clone[0].pixel_array[:,:,3].max()),int(display[0].pixel_array[:,:,3].max()))
        camera.capture_mobjects([display])

if __name__=="__main__":
    unittest.main()
