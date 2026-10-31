"""📑 Inspect actual native class-table typography and page transitions without constructing model geometry."""
import importlib,json,os,sys,hashlib
from pathlib import Path
import manim as engine
import numpy as np
from PIL import Image
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'package.json').is_file())
sys.path.insert(0,str(ROOT))
sys.dont_write_bytecode=True
OUT=Path(__file__).parent/'🗑️generated/native-r92/pages'
OUT.mkdir(parents=True,exist_ok=True)
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_AUDIENCE='speaker',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT))
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧩️models.🐍️')
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
Base=module.Beat4_PartialModels
paths=[Path(module.__file__),Path(api.__file__),Path(api.__file__).with_name('🔣️.json'),Path(__file__)]
def hashes():
    """🔐 Retain actual loaded native layout and source cohort provenance."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes()
records=[]
def apply(scene,duration,animations):
    """🎞️ Use the actual native scene playback and cleanup lifecycle for each inspected page."""
    scene.play(*animations,run_time=duration)
    scene.update_mobjects(0)

def snapshot(scene,name):
    """📷 Capture the actual native1080p canvas for manual layout inspection."""
    scene.camera.reset()
    scene.camera.capture_mobjects(scene.mobjects)
    cached_pixels=scene.camera.pixel_array.copy()
    reference=engine.Camera(pixel_width=scene.camera.pixel_width,pixel_height=scene.camera.pixel_height,frame_width=scene.camera.frame_width,frame_height=scene.camera.frame_height,frame_center=scene.camera.frame_center.copy(),background_color=scene.camera.background_color,background_opacity=scene.camera.background_opacity)
    reference.background=scene.camera.background.copy()
    reference.capture_mobjects(scene.mobjects)
    equal=bool(np.array_equal(reference.pixel_array,cached_pixels))
    if not equal:
        Image.fromarray(reference.pixel_array).save(OUT/(name+'-native.png'))
    path=OUT/(name+'.png')
    Image.fromarray(cached_pixels).save(path)
    records.append({'name':name,'path':str(path),'mobjects':len(scene.get_mobject_family_members()),'native_pixel_equal':equal})
    print('[DEBUG] Native class page '+name+' pixel_equal='+str(equal),flush=True)
    assert equal,'Native/cache pixel mismatch on '+name
with engine.tempconfig({'media_dir':str(OUT/'m'),'text_dir':str(OUT/'t'),'pixel_width':1920,'pixel_height':1080,'frame_rate':4,'renderer':'cairo','dry_run':True,'disable_caching':True}):
    for language in (sys.argv[1],) if len(sys.argv)>1 else ('en','de'):
        os.environ['SEMIO_TEACHING_LANGUAGE']=language
        for discipline in ('architecture','structure','services'):
            scene=Base()
            module._visuals._fonts.apply_scene_style(scene)
            scene.model=api.load_model()
            scene.info={name:api.partial_model(name,scene.model) for name in module.DISCIPLINES}
            scene._lit=lambda drawing,tags:engine.Animation(engine.VGroup())
            rows=scene._discipline_rows(discipline)
            _,steps=scene._rows(discipline,rows,1.3,None,pitch=.28)
            scene.add(scene._card(discipline,2.1))
            for index,(duration,animations) in enumerate(steps):
                apply(scene,duration,animations)
                if (index+1)%6==0 or index==len(steps)-1:
                    snapshot(scene,f'{language}-{discipline}-{index//6+1}')
        scene=Base()
        module._visuals._fonts.apply_scene_style(scene)
        scene.model=api.load_model()
        scene.info={name:api.partial_model(name,scene.model) for name in module.DISCIPLINES}
        scene._arch=lambda point,scale:engine.VGroup(engine.Rectangle(width=1,height=.5,color=module.ARCH).move_to(point))
        scene.villa_axon=lambda point,scale,**kwargs:engine.VGroup(engine.Rectangle(width=1,height=.5,color=module.SERV).move_to(point))
        scene._leave=lambda *args:engine.FadeOut(engine.VGroup())
        def show(topic,*phases):
            """📋 Inspect real matrix cells after every complete three-column page."""
            for index,(duration,animations) in enumerate(phases):
                apply(scene,duration,animations)
                if index and index%7==0:
                    snapshot(scene,f'{language}-matrix-{index//7}')
        scene._show=show
        scene._schema()
after=hashes()
(OUT/'result.json').write_text(json.dumps({'diagnostic':True,'geometry_placeholders_only_in_matrix':True,'native_typography':True,'inputs_before':before,'inputs_after':after,'stable':before==after,'snapshots':records},indent=2),encoding='utf-8')
