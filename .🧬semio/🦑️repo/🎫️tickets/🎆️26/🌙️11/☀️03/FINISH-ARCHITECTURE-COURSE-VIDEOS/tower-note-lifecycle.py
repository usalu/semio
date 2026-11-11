import sys,os,importlib
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
import numpy as np
from manim import Camera,VGroup,Square,FadeIn,FadeOut,tempconfig
t=importlib.import_module('🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🗼️tower.🐍️')
os.environ['SEMIO_TEACHING_LANGUAGE']='en'
with tempconfig({'dry_run':True,'pixel_width':320,'pixel_height':180,'frame_rate':3,'background_color':'#000000'}):
 s=t.Beat3_Tower();s.language=lambda:'en'
 def label(x):
  result=s._say(x,(-2.35,-2.3),22,t.TXT);result.plain=True;return result
 old=label('A: 76 capsules · B: 64 capsules')
 a=t._morph.token('76',kind='number',color=t.ARCH,size=1.3).move_to([.7,-2.3,0])
 b=t._morph.token('64',kind='number',color=t.WARN,size=1.3).move_to([1.7,-2.3,0])
 group=VGroup(old,a,b)
 sources_a=[Square(side_length=.1).shift([i*.2,1,0]) for i in range(2)]
 sources_b=[Square(side_length=.1).shift([i*.2,2,0]) for i in range(2)]
 s.add(*sources_a,*sources_b)
 s.play(FadeIn(old),t._morph.gather(sources_a,a,run_time=.4),t._morph.gather(sources_b,b,run_time=.4),run_time=.4)
 s.remove(*group.get_family()[1:]);s.add(group)
 total=label('76 + 64 = 140')
 targets_a=[Square(side_length=.1).shift([i*.2,1,0]) for i in range(2)]
 targets_b=[Square(side_length=.1).shift([i*.2,2,0]) for i in range(2)]
 s.play(FadeOut(group),FadeIn(total),t._morph.split(a,targets_a,run_time=.4),t._morph.split(b,targets_b,run_time=.4),run_time=.4)
 s.play(FadeOut(VGroup(*targets_a,*targets_b)),run_time=.3);s.remove(*targets_a,*targets_b)
 if os.environ.get('RETIRE')=='1':s.remove(group,*group.get_family())
 print('old family survives',any(x is old for x in s.get_mobject_family_members()),'top-level',[(type(x).__name__,x is group,x is old,x is total) for x in s.mobjects],flush=True)
 oracle=Camera(pixel_width=320,pixel_height=180,frame_width=s.camera.frame_width,frame_height=s.camera.frame_height,background_color='#000000',background_opacity=1)
 oracle.capture_mobjects([total]);expected=np.asarray(oracle.get_image(),dtype=int)
 diffs=[];original=s.renderer.update_frame
 def update(*args,**kw):
  result=original(*args,**kw)
  diffs.append(int(np.abs(np.asarray(s.camera.get_image(),dtype=int)-expected).max()))
  return result
 s.renderer.update_frame=update
 next_note=label('Program B → C; porthole stays')
 s.play(t._morph.retext(total,next_note,run_time=1),run_time=1)
 print('output differences',diffs,flush=True)
