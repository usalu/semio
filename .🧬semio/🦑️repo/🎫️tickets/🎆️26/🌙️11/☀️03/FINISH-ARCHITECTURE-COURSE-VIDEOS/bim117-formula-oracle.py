"""🔬 Compare BIM 117 slab formula frames with fresh Cairo."""
import ast,importlib,os,sys
from pathlib import Path
import numpy as np
from manim import Camera,ValueTracker,tempconfig
sys.path.insert(0,str(Path.cwd()));os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️")
source=Path(m.__file__);tree=ast.parse(source.read_text(encoding="utf8"))
owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_slab")
assignment=next(n for n in method.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=="formula" for t in n.targets))
show=next(n.value for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=="_show")
with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
    scene=m.Beat4_PartialModels();gross,cut,net=ValueTracker(0),ValueTracker(0),ValueTracker(0)
    slab={"gross_area":20.,"void_area":3.,"net_area":17.}
    env={**vars(m),"self":scene,"gross":gross,"cut":cut,"net":net,"slab":slab,"edge":m.VGroup()}
    exec(compile(ast.Module(body=[assignment],type_ignores=[]),str(source),"exec"),env)
    formula=env["formula"];stage=eval(compile(ast.Expression(body=show.args[2]),str(source),"eval"),env)
    camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
    samples=[];original=scene.renderer.update_frame
    def update(*args,**kwargs):
        result=original(*args,**kwargs)
        if 4<gross.get_value()<18:
            actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
            fresh=eval(compile(ast.Expression(body=assignment.value),str(source),"eval"),env)
            camera.reset();camera.capture_mobjects([fresh]);expected=np.asarray(camera.get_image(),dtype=int)
            samples.append((float(gross.get_value()),int(np.abs(actual-expected).max())))
        return result
    scene.renderer.update_frame=update
    animation,duration=scene._timeline(stage);scene.play(animation,run_time=duration)
    print("[DEBUG] slab formula",samples,flush=True)
    if max(v for _,v in samples)>2:raise AssertionError("Slab formula counter differs from fresh Cairo")
