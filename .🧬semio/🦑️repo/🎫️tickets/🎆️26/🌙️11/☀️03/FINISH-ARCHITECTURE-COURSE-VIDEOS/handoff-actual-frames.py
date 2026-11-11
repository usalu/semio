import os,sys,importlib
from pathlib import Path
from types import SimpleNamespace
sys.path.insert(0,str(Path.cwd()))
from manim import ValueTracker,FadeIn,tempconfig
a=importlib.import_module('🎓️teaching.🏛️architecture.📊️excel.🎬️clip.🦴️anatomy.🐍️')
out=Path(__file__).parent / "🗑️generated"
os.environ['SEMIO_TEACHING_LANGUAGE']='en'
with tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180,'frame_rate':2,'background_color':'#000000'}):
 scene=a.Beat1_Anatomy();scene.language=lambda:'en';scene._outer={'width':2.5,'length':4.0,'height':2.5};scene._base_il=3.8
 scene.s=SimpleNamespace(iw=ValueTracker(2.3),il=3.8,dims=(),floor=False,floor_color=a.GOOD,interior=True,err=True,fit=0,chips=False,perim=None,area=False,delta=False,fixchip='fix: type the length into B6',show=True,status=None)
 scene._fittings=[]
 book=a._xl.workbook(language='en')
 div=a._xl.apply(book,[{'type':'clear-cell','sheet':a.SHEET,'cell':'B6'}])
 name=a._xl.apply(book,[{'type':'set-cell','sheet':a.SHEET,'cell':'G16','formula':'=SUMM(G12:G15)'}])
 old=scene._window(div,selected='B15',rows=(6,15));new=scene._window(name,selected='G16',cols='C:G',rows=(10,16))
 capsule=scene._live_capsule();caption=scene.text('#DIV/0! example',(0,-3.5,0),size=18,color='#ffffff')
 scene.add(old,capsule,caption)
 frames=[];phase=['old'];orig=scene.renderer.update_frame
 def update(*args,**kwargs):
  result=orig(*args,**kwargs);frames.append((phase[0],scene.camera.get_image().copy()));return result
 scene.renderer.update_frame=update
 def change():
  phase[0]='gap';scene._state(err=False,floor=True,floor_color=a.MUTED,interior=False,fit=0,fixchip='fix: write SUM correctly')
 scene._handoff(old,capsule,change)
 caption.become(scene.text('#NAME? example',(0,-3.5,0),size=18,color='#ffffff'))
 phase[0]='new';scene.play(FadeIn(new),FadeIn(capsule),run_time=1)
 for i,(name,img) in enumerate(frames):img.save(out/f'handoff-actual-{i}-{name}.png')
 print('frames',len(frames),'phases',[x for x,_ in frames],flush=True)
