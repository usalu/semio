"""⏱️ Verify actual detailed static capsule sprite reuse through Cairo."""
import importlib
import os
import sys
import time
from pathlib import Path
from manim import tempconfig
sys.path.insert(0,str(Path.cwd()))
os.environ["SEMIO_TEACHING_LANGUAGE"]="en"
m=importlib.import_module("🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.💊️capsule.🐍️")
with tempconfig({"dry_run":True,"pixel_width":854,"pixel_height":480,"frame_rate":15}):
    scene=m.Beat2_Capsule()
    for alpha in (1.0,0.5):
        start=time.perf_counter()
        sprite=scene._static_sprite("A",0.0,"Slash",0.13,(1.0,0.5,0.0),alpha)
        scene.camera.reset()
        scene.camera.capture_mobjects([sprite])
        print({"alpha":alpha,"seconds":round(time.perf_counter()-start,3),"cache_entries":len(scene._static_capsules)},flush=True)
        assert len(scene._static_capsules)==1
        if alpha==1:
            scene.camera.get_image().save(Path(__file__).parent/"🗑️generated"/"capsule-static-sprite.png")
