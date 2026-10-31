"""⏱️ Profile one real native Cairo frame without altering authored geometry."""
import cProfile
import hashlib
import time
import numpy as np
import manim as engine
import importlib
import json
import os
from pathlib import Path
import pstats
import sys
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
OUT=Path(__file__).parent/'🗑️generated/native-r92/camera-benchmark'
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT))
from manim import tempconfig
from PIL import Image
def fingerprints():
    """🔐 Freeze actual source closure, fonts, environment declaration and owned benchmark input."""
    runtime=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️')
    theme=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️theme.🐍️')
    clip=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'
    paths={p for p in clip.rglob('*') if p.suffix in {'.py','.json','.mjs','.cjs','.md'} and '__pycache__' not in p.parts}
    paths.update(runtime.source_dependencies(clip/'📊️information/🐍️.py'))
    paths.update(theme.source_files())
    paths.update((Path(__file__),Path(runtime.__file__),ROOT/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml',ROOT/'🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/uv.lock'))
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths) if p.is_file()}

BEFORE=fingerprints()
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
            reference=engine.Camera(pixel_width=self.camera.pixel_width,pixel_height=self.camera.pixel_height,
                frame_width=self.camera.frame_width,frame_height=self.camera.frame_height,
                frame_center=self.camera.frame_center.copy(),background_color=self.camera.background_color,
                background_opacity=self.camera.background_opacity)
            reference.background=self.camera.background.copy()
            rows=[]
            for alpha in (0.,.5,1.):
                for animation in compiled:
                    animation.update_mobjects(.25)
                    animation.interpolate(alpha)
                self.update_mobjects(.25)
                times=[]
                arrays=[]
                before_hits=self.camera._path_cache_hits
                before_misses=self.camera._path_cache_misses
                for camera in (reference,self.camera,self.camera):
                    camera.reset()
                    begin=time.perf_counter()
                    camera.capture_mobjects(self.mobjects)
                    times.append(time.perf_counter()-begin)
                    arrays.append(camera.pixel_array.copy())
                equal=[bool(np.array_equal(arrays[0],a)) for a in arrays[1:]]
                warm_hits=self.camera._path_cache_hits-before_hits
                misses=self.camera._path_cache_misses-before_misses
                row={'alpha':alpha,'reference_seconds':times[0],'first_cached_seconds':times[1],
                     'warm_cached_seconds':times[2],'pixel_equal':equal,'native_pixel_sha256':hashlib.sha256(arrays[0].tobytes()).hexdigest(),'hit_delta':warm_hits,'miss_delta':misses,
                     'hits':self.camera._path_cache_hits,'misses':self.camera._path_cache_misses,
                     'entries':len(self.camera._path_cache),'accounted_bytes':self.camera._path_cache_size}
                rows.append(row)
                print('[DEBUG] '+json.dumps(row),flush=True)
                Image.fromarray(arrays[2]).save(OUT/('frame-'+str(alpha)+'.png'))
                if not all(equal) or warm_hits == 0 or times[2] >= times[0]:
                    (OUT/'result.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
                    raise AssertionError('Native full-model pixels, warm hits or measured capture improvement failed')
            after=fingerprints()
            import importlib.metadata
            versions={name:importlib.metadata.version(name) for name in ('manim','numpy','pycairo','manimpango','Pillow')}
            (OUT/'result.json').write_text(json.dumps({'inputs_before':BEFORE,'inputs_after':after,'stable':BEFORE==after,'python':sys.version,'packages':versions,'frames':rows,'width':self.camera.pixel_width,
                'height':self.camera.pixel_height,'family':len(self.get_mobject_family_members()),
                'points':sum(len(m.points) for m in self.get_mobject_family_members())},indent=2),encoding='utf-8')
            raise SystemExit(0 if BEFORE==after else 1)
        for animation in compiled:
            animation.finish()
            animation.clean_up_from_scene(self)
        self.update_mobjects(0)
        self.renderer.time+=max((a.run_time for a in compiled),default=0)

with tempconfig({'media_dir':str(OUT/'media'),'text_dir':str(OUT/'text'),'pixel_width':1920,'pixel_height':1080,'frame_rate':4,'renderer':'cairo','dry_run':True,'disable_caching':True}):
    Probe().construct()
