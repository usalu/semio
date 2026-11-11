"""🔬 Compare BIM 117 class-row counter frames with fresh Cairo."""
import importlib,os,sys
from pathlib import Path
import numpy as np
from PIL import Image
from manim import Camera,FadeIn,ValueTracker,tempconfig
sys.path.insert(0,str(Path.cwd()));os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️")
with tempconfig({"dry_run":True,"pixel_width":320,"pixel_height":180,"frame_rate":10,"background_color":"#000000"}):
    scene=m.Beat4_PartialModels();tracker=ValueTracker(0)
    row=scene._class_row(tracker,"IfcWall",("Wall","Wand"),1.3,m.ARCH)
    camera=Camera(pixel_width=320,pixel_height=180,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,background_color="#000000",background_opacity=1)
    frames=[];original=scene.renderer.update_frame
    def update(*args,**kwargs):
        result=original(*args,**kwargs);actual=np.asarray(scene.camera.get_image(),dtype=int).copy()
        camera.reset();camera.capture_mobjects([scene._class_row(tracker,"IfcWall",("Wall","Wand"),1.3,m.ARCH)])
        expected=np.asarray(camera.get_image(),dtype=int).copy()
        frames.append((float(tracker.get_value()),int(np.abs(actual-expected).max()),actual,expected));return result
    scene.renderer.update_frame=update
    if "--fixed" in sys.argv:
        actions=[m.Succession(FadeIn(row,run_time=.15),tracker.animate(rate_func=m.LINEAR).set_value(28).set_run_time(.4))]
    else:actions=[FadeIn(row,run_time=.15),tracker.animate(rate_func=m.LINEAR).set_value(28)]
    animation,duration=scene._timeline((.55,actions))
    scene.play(animation,run_time=duration)
    samples=[r for r in frames if 5<r[0]<25]
    worst=max(samples,key=lambda r:r[1]);folder=Path(__file__).parent/"🗑️generated"
    Image.fromarray(worst[2].astype("uint8")).save(folder/"bim117-counter-actual.png")
    Image.fromarray(worst[3].astype("uint8")).save(folder/"bim117-counter-fresh.png")
    print("[DEBUG] samples",[(round(v,3),d) for v,d,_,_ in samples],"worst",worst[1],flush=True)
    if worst[1]>2:raise AssertionError("Actual class row counter differs from fresh Cairo")
