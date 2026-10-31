"""⏱️ Profile one real native Cairo frame without altering authored geometry."""
import cProfile
import importlib
import json
import os
from pathlib import Path
import pstats
import sys
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
OUT=Path(__file__).parent/'🗑️generated/native-r92/profile'
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT))
from manim import tempconfig
from PIL import Image
base=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.📊️information.🐍️').Beat1_DataToInformation

class Probe(base):
    """🔬 Retain native animation ownership and profile the first detailed model frame."""
    calls=0
    def wait(self,duration=1,**kwargs):
        self.update_mobjects(.25)
        self.renderer.time+=duration
    def play(self,*animations,**kwargs):
        self.calls+=1
        compiled=self.compile_animations(*animations,**kwargs)
        self.add_mobjects_from_animations(compiled)
        self.animations=compiled
        self.begin_animations()
        for animation in compiled:
            animation.update_mobjects(.25)
            animation.interpolate(.5)
        self.update_mobjects(.25)
        if self.calls==10:
            profile=cProfile.Profile()
            profile.enable()
            self.camera.capture_mobjects(self.mobjects)
            profile.disable()
            profile.dump_stats(str(OUT/'frame.prof'))
            with (OUT/'frame-stats.txt').open('w',encoding='utf-8') as stream:
                pstats.Stats(profile,stream=stream).sort_stats('cumulative').print_stats(60)
            Image.fromarray(self.camera.pixel_array).save(OUT/'frame.png')
            (OUT/'frame.json').write_text(json.dumps({'calls':self.calls,'mobjects':len(self.mobjects),'family':len(self.get_mobject_family_members()),'points':sum(len(m.points) for m in self.get_mobject_family_members())}),encoding='utf-8')
            raise SystemExit(0)
        for animation in compiled:
            animation.finish()
            animation.clean_up_from_scene(self)
        self.update_mobjects(0)
        self.renderer.time+=max((a.run_time for a in compiled),default=0)

with tempconfig({'media_dir':str(OUT/'media'),'text_dir':str(OUT/'text'),'pixel_width':480,'pixel_height':270,'frame_rate':4,'renderer':'cairo','dry_run':True,'disable_caching':True}):
    Probe().construct()
