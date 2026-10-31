"""🔬 Isolate native allocation, typography and object construction with finite cooperative probes."""
import os,sys,time,json,hashlib,faulthandler,platform,ctypes
from pathlib import Path
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));MODE,DIRECTORY=sys.argv[1:3];OUT=Path(DIRECTORY);OUT.mkdir(parents=True,exist_ok=True);START=time.monotonic()
os.environ.update(SEMIO_TEACHING_LANGUAGE='en',SEMIO_TEACHING_OUTPUT_ROOT=str(OUT),SEMIO_TEST_MANIM='1',PYTHONUTF8='1')
FATAL=(OUT/'fatal.log').open('w');TIMEOUT=(OUT/'timeout.log').open('w');faulthandler.enable(file=FATAL,all_threads=True)
def memory():
    """🌡️ Observe only this process with explicitly typed portable counter boundaries."""
    if os.name!='nt':
        import resource
        return {'peak_rss_bytes':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*(1 if sys.platform=='darwin' else 1024)}
    from ctypes import wintypes as W
    class Counters(ctypes.Structure):_fields_=[('cb',W.DWORD),('faults',W.DWORD)]+[(n,ctypes.c_size_t) for n in ('peak','rss','peak_paged','paged','peak_nonpaged','nonpaged','pagefile','peak_pagefile','private')]
    k=ctypes.WinDLL('kernel32',use_last_error=True);k.GetCurrentProcess.argtypes=[];k.GetCurrentProcess.restype=W.HANDLE;p=ctypes.WinDLL('psapi',use_last_error=True);p.GetProcessMemoryInfo.argtypes=[W.HANDLE,ctypes.POINTER(Counters),W.DWORD];p.GetProcessMemoryInfo.restype=W.BOOL;c=Counters();c.cb=ctypes.sizeof(c)
    if not p.GetProcessMemoryInfo(k.GetCurrentProcess(),ctypes.byref(c),c.cb):raise ctypes.WinError(ctypes.get_last_error())
    return {'rss_bytes':c.rss,'peak_rss_bytes':c.peak,'private_bytes':c.private}
def event(name,**values):
    """🧾 Flush finite-operation provenance before any subsequent native allocation."""
    row={'event':name,'mode':MODE,'pid':os.getpid(),'elapsed':time.monotonic()-START,**memory(),**values}
    with (OUT/'events.jsonl').open('a',encoding='utf8') as f:f.write(json.dumps(row)+'\n');f.flush()
    print('[DEBUG] '+json.dumps(row),flush=True)
    if (OUT/'cancel').exists() or row['elapsed']>(600 if MODE=='factory' else 110) or row.get('rss_bytes',0)>5905580032:raise InterruptedError('Own finite probe cooperatively cancelled or reached110s bound')
def manifest(np,manim=None):
    """🧬 Retain loaded Python/native binary identities without exposing environment secrets."""
    modules={'numpy':np,'numpy_core':np._core._multiarray_umath}
    if manim is not None:modules['manim']=manim
    rows={name:{'version':getattr(mod,'__version__',None),'file':str(mod.__file__),'sha256':hashlib.sha256(Path(mod.__file__).read_bytes()).hexdigest()} for name,mod in modules.items()};(OUT/'manifest.json').write_text(json.dumps({'python':sys.version,'platform':platform.platform(),'modules':rows},indent=2),encoding='utf8')
event('starting')
try:
    import numpy as np
    if MODE=='array':
        manifest(np)
        for i in range(1000000):
            value=np.asarray((.1,.2,.3),dtype=np.float64).copy();value.resize(4,refcheck=False);value[3]=1.;assert value.tolist()==[.1,.2,.3,1.]
            if i%50000==0:event('arrays',count=i)
    else:
        import manim as m
        manifest(np,m);event('manim-imported')
        if MODE=='color':
            for i in range(300000):
                color=m.ManimColor.from_rgb((.1,.2,.3));t=m.interpolate_color(color,m.WHITE,(i%101)/100);assert len(t.to_rgba())==4
                if i%10000==0:event('colors',count=i)
        elif MODE in ('text','native-text'):
            if MODE=='text':
                import importlib
                fonts=importlib.import_module('🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️');factory=lambda text:fonts.body_text(text,font_size=17,color='#324455')
            else:factory=lambda text:m.Text(text,font_size=60,warn_missing_font=False)
            for i in range(300):
                event('text-start',index=i);obj=factory('Native allocation probe '+str(i));assert obj.width>0;event('text-end',index=i,children=len(obj.submobjects))
        elif MODE=='vector':
            for batch in range(8):
                group=m.VGroup()
                for i in range(1200):
                    shape=m.VMobject();shape.set_points_as_corners(np.asarray([[0.,0.,0.],[1.,0.,0.],[.8,.7,0.],[0.,0.,0.]]));shape.set_fill(m.interpolate_color(m.BLUE,m.WHITE,(i%101)/100),opacity=.8);shape.set_stroke(m.BLACK,width=.5);group.add(shape)
                event('vector-group',batch=batch,children=len(group.submobjects));copy=group.copy().set_color(m.RED);assert len(copy.submobjects)==1200;event('vector-copy',batch=batch)
        elif MODE=='factory':
            import importlib,types
            from unittest.mock import patch
            obj=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧬️objects.🐍️');axon=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️');counts={'primitives':0};original_render=axon._Painter.render
            def render(self,prim,layer,dz):
                counts['primitives']+=1;row={'tag':prim.tag,'faces':len(prim.faces),'layer':layer,'index':counts['primitives']};(OUT/'last-primitive.json').write_text(json.dumps(row),encoding='utf8')
                if counts['primitives']%250==1:event('axon-primitive',**row)
                return original_render(self,prim,layer,dz)
            def wrap(name,method):
                def run(self,*args,**kwargs):
                    event('factory-enter',method=name);result=method(self,*args,**kwargs);event('factory-exit',method=name);return result
                return run
            with m.tempconfig({'media_dir':str(OUT/'media'),'text_dir':str(OUT/'text'),'pixel_width':480,'pixel_height':270,'frame_rate':4,'renderer':'cairo','dry_run':True,'disable_caching':True}),patch.object(axon._Painter,'render',render):
                scene=obj.Beat2_ObjectOrientation();obj._fonts.apply_scene_style(scene);scene._caption=None
                for name in ('_facts','_axon','_chip','text','class_box'):
                    setattr(scene,name,types.MethodType(wrap(name,getattr(type(scene),name)),scene))
                event('scene-ready');facts=scene._facts();event('facts-ready')
                for i in range(1000):m.ManimColor.from_rgb((.1,.2,.3))
                event('post-facts-colors');tag=scene.text('IFC4.3 · ISO16739-1:2024',(6.8,3.7,0),size=obj.S_SMALL,color=obj.MUTED,anchor='ru');scene.add(tag)
                scene.step=types.MethodType(lambda self,key,*args,**kwargs:event('first-beat-ready',key=key,animations=len(args)),scene)
                scene._schema(facts);event('schema-factory-complete',**counts)
                scene.camera.reset();scene.camera.capture_mobjects([facts.card,facts.badges,facts.axon,facts.count_chip,facts.guid_lines]);event('schema-cairo-complete')
                from PIL import Image
                Image.fromarray(scene.camera.pixel_array).save(OUT/'schema.png')
        else:raise ValueError(MODE)
    event('complete');(OUT/'result.json').write_text(json.dumps({'mode':MODE,'success':True}),encoding='utf8')
except BaseException:
    import traceback
    (OUT/'exception.log').write_text(traceback.format_exc(),encoding='utf8');raise
finally:TIMEOUT.close()
