"""🔬 Compare the actual BIM 117 clearance ring against fresh Cairo frames."""
import ast,importlib,os,sys
from pathlib import Path
import numpy as np
from PIL import Image
from manim import Camera,ValueTracker,tempconfig
sys.path.insert(0,str(Path.cwd()));os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️")
source=Path(m.__file__);tree=ast.parse(source.read_text(encoding="utf8"))
owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_check")
assignment=next(n for n in method.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=="ring" for t in n.targets))
show=next(n.value for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=="_show")
with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
    scene=m.Beat4_PartialModels();ring_value=ValueTracker(0);ring_radius=.6;zoom=2.;pipe_at=np.array([1.2,1.25,0.])
    env={**vars(m),"self":scene,"ring_value":ring_value,"ring_radius":ring_radius,"zoom":zoom,"pipe_at":pipe_at,"dimension":m.VGroup()}
    exec(compile(ast.Module(body=[assignment],type_ignores=[]),str(source),"exec"),env)
    ring=env["ring"]
    stage=eval(compile(ast.Expression(body=show.args[3]),str(source),"eval"),env)
    camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
    frames=[];original=scene.renderer.update_frame
    def update(*args,**kwargs):
        result=original(*args,**kwargs);actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
        camera.reset();camera.capture_mobjects([ring._builder()]);expected=np.asarray(camera.get_image(),dtype=int).copy()
        frames.append((float(ring_value.get_value()),int(np.abs(actual-expected).max()),actual,expected));return result
    scene.renderer.update_frame=update
    animation,duration=scene._timeline(stage);scene.play(animation,run_time=duration)
    samples=[row for row in frames if .1*ring_radius<row[0]<.9*ring_radius]
    worst=max(samples,key=lambda row:row[1]);folder=Path(__file__).parent/"🗑️generated"
    Image.fromarray(worst[2].astype("uint8")).save(folder/"bim117-ring-actual.png")
    Image.fromarray(worst[3].astype("uint8")).save(folder/"bim117-ring-fresh.png")
    print("[DEBUG] samples",[(round(v,3),diff) for v,diff,_,_ in samples],"worst",worst[1],flush=True)
    if worst[1]>2:raise AssertionError("Actual source ring differs from fresh Cairo during reveal/ramp")
