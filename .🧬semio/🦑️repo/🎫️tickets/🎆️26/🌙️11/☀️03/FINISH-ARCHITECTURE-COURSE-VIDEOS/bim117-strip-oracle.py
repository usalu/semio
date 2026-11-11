"""🔬 Compare actual BIM 117 buildup strip frames with fresh Cairo reference frames."""
import ast,importlib,os,sys
from pathlib import Path
import numpy as np
from PIL import Image
from manim import Camera,FadeIn,ValueTracker,tempconfig
sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
module=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️")
source=Path(module.__file__);tree=ast.parse(source.read_text(encoding="utf8"))
owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_buildup")
builder=next(n for n in method.body if isinstance(n,ast.FunctionDef) and n.name=="strip")
with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
    if "--mixin" in sys.argv:
        moving=importlib.import_module("🎓️teaching.🧰️interop.🎬️scene.🐍️")
        class Probe(moving.RefreshMoving,module.Beat4_PartialModels):pass
        scene=Probe()
    else:scene=module.Beat4_PartialModels()
    wall=module._model.element("W-S")["material"]
    layers,total=wall["layers"],wall["total_thickness"]
    zoom,x0,y0,height,grow=8.0,1.2,1.25,1.5,ValueTracker(0)
    tones=[(module.mix(module.BACKGROUND,module.INK,.6),module.INK,45,.07),(module.mix(module.BACKGROUND,module.WARN,.5),module.WARN,45,.12),(module.mix(module.BACKGROUND,module.INK,.3),module.INK,135,.07)]
    env={**vars(module),"self":scene,"layers":layers,"total":total,"zoom":zoom,"x0":x0,"y0":y0,"height":height,"grow":grow,"tones":tones}
    exec(compile(ast.Module(body=[builder],type_ignores=[]),str(source),"exec"),env)
    strip=env["strip"]
    live=scene.live(strip,key=lambda:round(grow.get_value(),3))
    camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
    frames=[];original=scene.renderer.update_frame
    def update(*args,**kwargs):
        result=original(*args,**kwargs)
        actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
        camera.reset();camera.capture_mobjects([strip()])
        expected=np.asarray(camera.get_image(),dtype=int).copy()
        frames.append((float(grow.get_value()),int(np.abs(actual-expected).max()),actual,expected))
        return result
    scene.renderer.update_frame=update
    if "--timeline" in sys.argv:
        if "--sequence" in sys.argv:
            animation,total=scene._timeline((.1,[FadeIn(live)]),(1.7,[grow.animate(rate_func=module.SMOOTH).set_value(1.0)]))
        else:
            animation,total=scene._timeline((1.8,[FadeIn(live,run_time=.1),grow.animate(rate_func=module.SMOOTH).set_value(1.0)]))
        scene.play(animation,run_time=total)
    elif "--sequence" in sys.argv:
        scene.play(FadeIn(live),run_time=.1)
        scene.play(grow.animate(rate_func=module.SMOOTH).set_value(1.0),run_time=1.7)
    else:
        scene.play(FadeIn(live,run_time=.1),grow.animate(rate_func=module.SMOOTH).set_value(1.0),run_time=1.8)
    samples=[row for row in frames if .1<row[0]<.9]
    worst=max(samples,key=lambda row:row[1])
    folder=Path(__file__).parent/"🗑️generated"
    Image.fromarray(worst[2].astype("uint8")).save(folder/"bim117-strip-actual.png")
    Image.fromarray(worst[3].astype("uint8")).save(folder/"bim117-strip-fresh.png")
    print("[DEBUG] samples",[(round(v,3),diff) for v,diff,_,_ in samples],"worst",worst[1],flush=True)
    if worst[1]>2:raise AssertionError("Actual source strip differs from fresh Cairo during reveal/ramp")
