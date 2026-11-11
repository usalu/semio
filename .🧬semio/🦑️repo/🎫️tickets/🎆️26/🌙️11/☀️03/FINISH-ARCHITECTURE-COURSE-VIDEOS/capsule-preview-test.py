"""🧪 Staged capsule dimensions, positions and redraw identity."""
import importlib
import json
import sys
import unittest
from pathlib import Path
import numpy as np
from shapely.geometry import box, Polygon
from unittest.mock import patch
from types import SimpleNamespace
from manim import ValueTracker
from PIL import Image
from manim import tempconfig

sys.path.insert(0,str(Path.cwd()))
module = importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
case = json.loads(Path(__file__).with_name("capsule-stage-cases.json").read_text(encoding="utf-8"))

class CapsulePreview(unittest.TestCase):
    def parameters(self):
        return {"width":case["width"],"length":case["length"],"height":case["height"],"back":case["back"],
                "front":case["back"]+case["length"],"corner_radius":case["radius"],"window_diameter":case["window_diameter"],
                "window_u":case["window_u"],"window_v":case["window_v"],"glazing_y":case["back"]+case["length"]}

    def test_geometry_modes_and_relevant_keys(self):
        p=self.parameters()
        for state in case["modes"]:
            self.assertEqual(module._preview_mode(state["round"],state["window"]),state["mode"])
        rv={"round":0.0,"window":0.0,"lift":1.0}
        first=module._preview_key(p,rv,(55.0,24.0),(4.0,-1.0,0.0))
        irrelevant={**p,"corner_radius":0.4,"window_diameter":1.8}
        self.assertEqual(first,module._preview_key(irrelevant,rv,(55.0,24.0),(4.0,-1.0,0.0)))
        self.assertNotEqual(first,module._preview_key({**p,"width":3.1},rv,(55.0,24.0),(4.0,-1.0,0.0)))
        self.assertNotEqual(first,module._preview_key(p,{**rv,"round":1.0},(55.0,24.0),(4.0,-1.0,0.0)))

    def test_cached_geometry_and_rendered_modes(self):
        p=self.parameters()
        with tempconfig({"dry_run":True,"pixel_width":480,"pixel_height":270,"frame_rate":15}):
            scene=module.Beat2_Capsule()
            origin=np.array([0.0,0.0,0.0])
            view=(55.0,24.0)
            v=module._nkd.make_view(view,origin,module.SCALE,(0,0,0))
            with patch.object(scene,"analytical_box_axon",wraps=scene.analytical_box_axon) as build:
                rv={"round":0.0,"window":0.0,"lift":1.0}
                first=scene._preview(p,rv,view,v,())
                second=scene._preview(p,{**rv,"body":0.5},view,v,("width",))
                self.assertEqual(build.call_count,1)
                self.assertNotEqual(id(first),id(second))
            images=[]
            for state in case["modes"]:
                drawing=scene._preview(p,{**state,"lift":1.0},view,v,())
                self.assertEqual(drawing.preview_mode,state["mode"])
                scene.camera.reset()
                scene.camera.capture_mobjects([drawing])
                images.append(scene.camera.get_image().convert("RGB"))
                if state["mode"] in ("rounded","window"):
                    profile=Polygon(drawing.preview_profile)
                    self.assertAlmostEqual(profile.bounds[2]-profile.bounds[0],p["width"])
                    self.assertAlmostEqual(profile.bounds[3]-profile.bounds[1],p["height"])
                    expected=p["width"]*p["height"]-(4-np.pi)*p["corner_radius"]**2
                    self.assertAlmostEqual(profile.area,expected,delta=0.01)
                if state["mode"]=="window":
                    self.assertTrue(hasattr(drawing,"preview_aperture"))
                    aperture=Polygon(drawing.preview_aperture)
                    self.assertAlmostEqual(aperture.area,np.pi*(p["window_diameter"]/2)**2,delta=0.01)
            contact=Image.new("RGB",(480*len(images),270),"white")
            for i,image in enumerate(images):
                contact.paste(image,(480*i,0))
            output=Path(__file__).parent/"🗑️generated"/"capsule-preview-modes.png"
            contact.save(output)

    def test_moving_core_projection_and_reuse(self):
        scene=module.Beat2_Capsule()
        core=module._nkd.NakaginDrawing3D(module._nkd.make_view((55.0,24.0),np.array([1.0,2.0,0.0]),0.25,(0,0,0)))
        capsule=module._nk.capsule_type("A")
        casing=module._nk.capsule_casing(capsule)
        position=np.array([3.0,4.0,5.0])
        with patch.object(module._nkd,"capsule_mesh",wraps=module._nkd.capsule_mesh) as mesh:
            first=scene._core_preview(core,position,30.0,capsule)
            second=scene._core_preview(core,position,30.0,capsule)
            self.assertEqual(mesh.call_count,1)
            self.assertNotEqual(id(first),id(second))
        np.testing.assert_allclose(first.at(0,(casing.back+casing.front)/2,casing.height/2),core.at(*position),atol=1e-9)
        self.assertEqual(first.preview_mode,"window")
        np.testing.assert_allclose(first.view.centre,(0,(casing.back+casing.front)/2,casing.height/2),atol=1e-9)
        outline=Polygon(first.preview_profile)
        self.assertAlmostEqual(outline.bounds[2]-outline.bounds[0],casing.width)
        self.assertAlmostEqual(outline.bounds[3]-outline.bounds[1],casing.height)
        shifted=scene._core_preview(core,position+np.array([0,0,1]),30.0,capsule)
        np.testing.assert_allclose(shifted.at(0,(casing.back+casing.front)/2,casing.height/2),core.at(*(position+np.array([0,0,1]))),atol=1e-9)

    def test_static_sprite_reuses_detailed_geometry_for_overlay_changes(self):
        scene=module.Beat2_Capsule()
        sprite=module._sprites.Sprite(np.full((4,4,4),255,dtype=np.uint8))
        with patch.object(scene,"capsule_axon",return_value=module._engine.VGroup()) as build:
            with patch.object(module._sprites,"capture",return_value=sprite) as capture:
                first=scene._static_sprite("A",0.0,"Slash",0.13,(1,2,0),1.0)
                second=scene._static_sprite("A",0.0,"Slash",0.13,(2,3,0),0.5)
                self.assertEqual(build.call_count,1)
                self.assertEqual(capture.call_count,1)
                np.testing.assert_allclose(second.get_center()-first.get_center(),(1,1,0),atol=1e-9)
                self.assertLess(second.get_pixel_array()[:,:,3].max(),first.get_pixel_array()[:,:,3].max())

    def test_glide_overlay_reuses_geometry_and_vector_morph_is_preserved(self):
        scene=module.Beat2_Capsule()
        core=module._nkd.NakaginDrawing3D(module._nkd.make_view((55.0,24.0),np.array([0,0,0]),0.25,(0,0,0)))
        scene._core=core
        scene._plane_a={"origin":(0,0,0)}
        scene._cap_centre=np.array([0,2,1.25])
        target=SimpleNamespace(x=SimpleNamespace(x=1.0,y=0.0),origin=SimpleNamespace(t=(2,3,4)))
        box=SimpleNamespace(center=SimpleNamespace(t=(2,3,4)))
        solution=SimpleNamespace(items=lambda name:[target if name=="rot" else SimpleNamespace(root_box=lambda:box)])
        scene._solve_place=lambda:solution
        cv={key:ValueTracker(0) for key in ("a","b","cap","glide","angle","arrow","arc")}
        cv["cap"].set_value(1)
        cv["glide"].set_value(0.5)
        scene.st=SimpleNamespace(cv=cv)
        with patch.object(module._nkd,"capsule_mesh",wraps=module._nkd.capsule_mesh) as mesh:
            scene._glide()
            cv["arrow"].set_value(1)
            scene._glide()
            self.assertEqual(mesh.call_count,1)
        with patch.object(scene,"capsule_axon",return_value=module._engine.VGroup()) as vector:
            scene._glide(vector=True)
            self.assertEqual(vector.call_count,1)

    def test_wrap_reuses_static_detailed_capsules_across_reveals(self):
        scene=module.Beat2_Capsule()
        scene._core_wrap=module._nkd.NakaginDrawing3D(module._nkd.make_view((55.0,24.0),np.array([0,0,0]),0.22,(0,0,0)))
        plane=lambda x:SimpleNamespace(origin=(x,0,0),x=(1,0,0))
        scene._wrap_slots=[SimpleNamespace(plane=plane(0)),SimpleNamespace(plane=plane(3))]
        pick=SimpleNamespace(tag={"type":"A","family":"Slash"})
        scene._solve_wrap=lambda:SimpleNamespace(tree=lambda name:SimpleNamespace(items=lambda:[pick,pick]))
        trackers={key:ValueTracker(0) for key in ("planes","caps","count","wrap","rev")}
        for key,value in (("caps",1),("count",2),("rev",2)):
            trackers[key].set_value(value)
        scene.st=SimpleNamespace(wv=trackers)
        with patch.object(scene,"capsule_axon",return_value=module._engine.Square(fill_opacity=1)) as build:
            with patch.object(module._sprites,"capture",wraps=module._sprites.capture) as capture:
                scene._wrapstage()
                self.assertEqual(build.call_count,2)
                trackers["caps"].set_value(0.5)
                scene._wrapstage()
                self.assertEqual(build.call_count,2)
                self.assertEqual(capture.call_count,2)
                live=scene._preview_live(scene._wrapstage,lambda:trackers["caps"].get_value())
                for transition in (module._engine.FadeIn,module._engine.FadeOut):
                    animation=transition(live)
                    animation.begin()
                    animation.interpolate(0.5)
                    scene.camera.reset()
                    scene.camera.capture_mobjects([live])
                    animation.finish()

    def test_centered_and_lifted_native_geometry(self):
        p=self.parameters()
        scene=module.Beat2_Capsule()
        origin=np.array([1.0,2.0,0.0])
        view=(55.0,24.0)
        v=module._nkd.make_view(view,origin,module.SCALE,(0,0,0))
        for state in case["states"]:
            rv={"round":0.0,"window":0.0,"lift":state["lift"]}
            drawing=scene._preview(p,rv,view,v,("width","length","height"))
            self.assertEqual(drawing.preview_mode,"box")
            center=np.array(state["expected_center"],float)
            np.testing.assert_allclose(drawing.at(0,p["back"]+p["length"]/2,p["height"]/2),v(*center),atol=1e-9)
            for name,key in (("width","width"),("length","length"),("height","height")):
                self.assertAlmostEqual(drawing.dims[name].measurement_value,p[key])
            self.assertAlmostEqual(box(-p["width"]/2,0,p["width"]/2,p["length"]).area,p["width"]*p["length"])

if __name__=="__main__":
    unittest.main()
