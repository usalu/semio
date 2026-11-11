"""🔬 Compare BIM 117 trace footer and chart with fresh Cairo frames."""
import ast,importlib,os,sys
from pathlib import Path
import numpy as np
from PIL import Image
from manim import Camera,ValueTracker,tempconfig
sys.path.insert(0,str(Path.cwd()));os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️")
source=Path(m.__file__);tree=ast.parse(source.read_text(encoding="utf8"))
owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat4_PartialModels")
method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_trace")
show=next(n.value for n in method.body if isinstance(n,ast.Expr) and isinstance(n.value,ast.Call) and isinstance(n.value.func,ast.Attribute) and n.value.func.attr=="_show")
with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":6,"background_color":"#000000"}):
    scene=m.Beat4_PartialModels();records,kinds,fill=ValueTracker(0),ValueTracker(0),ValueTracker(0)
    groups=[{"id":key,"count":i+1} for i,key in enumerate(m.KIND_NAMES)]
    scene.analysis=analysis={"losses":list(range(28)),"loss_groups":groups}
    footer,squares=scene._trace_footer(records,kinds),scene._kind_chart(fill)
    env={**vars(m),"self":scene,"footer":footer,"squares":squares,"records":records,"kinds":kinds,"fill":fill,"analysis":analysis}
    stage=eval(compile(ast.Expression(body=show.args[-1]),str(source),"eval"),env)
    camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
    frames=[];original=scene.renderer.update_frame
    def update(*args,**kwargs):
        result=original(*args,**kwargs);actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
        camera.reset();camera.capture_mobjects([scene._trace_footer(records,kinds),scene._kind_chart(fill)])
        expected=np.asarray(camera.get_image(),dtype=int).copy()
        frames.append((float(fill.get_value()),int(np.abs(actual-expected).max()),actual,expected));return result
    scene.renderer.update_frame=update
    animation,duration=scene._timeline(stage);scene.play(animation,run_time=duration)
    samples=[r for r in frames if .2<r[0]<.9]
    worst=max(samples,key=lambda r:r[1]);folder=Path(__file__).parent/"🗑️generated"
    Image.fromarray(worst[2].astype("uint8")).save(folder/"bim117-trace-actual.png")
    Image.fromarray(worst[3].astype("uint8")).save(folder/"bim117-trace-fresh.png")
    print("[DEBUG] samples",[(round(v,3),d) for v,d,_,_ in samples],"worst",worst[1],flush=True)
    if worst[1]>2:raise AssertionError("Actual trace counters/chart differ from fresh Cairo")
