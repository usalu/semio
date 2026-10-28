"""🎬 Render compact canonical windows with a real click-activated heater."""
import hashlib,importlib,json,os,shutil,sys,time
from pathlib import Path
ROOT=next(path for path in Path(__file__).resolve().parents if (path/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
from manim import FadeIn,Line,Rectangle,Scene,VGroup,tempconfig
import numpy as np
BASE='🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring'
p=importlib.import_module(BASE+'.🎬️clip.🪟️presentation.🐍️')
r=p._interaction_render
core=p._interaction
helper=importlib.import_module(BASE+'.🎬️clip.🪟️presentation.🖱️interaction.🎬️scene.🐍️')
runtime=importlib.import_module(BASE+'.🎬️clip.⚙️runtime.🐍️')
visuals=importlib.import_module(BASE+'.🎨️visuals.🐍️')
output=Path(os.environ['SEMIO_TEACHING_OUTPUT_ROOT']);output.mkdir(parents=True,exist_ok=True)
settings={'quality':'-ql','resolution':'1600,900','fps':8,'language':'de','audience':'spectator'}
os.environ['SEMIO_TEACHING_AUDIENCE']='spectator'
viewport=p.presentation_contract()['focus']['viewport']


def pane(identity,title,rect,*,control=False):
    """🪟 Reuse the exact production WindowChrome renderer inside authored content."""
    data=r._geometry({'id':identity,'title':{'de':title},'symbol':'physics'},rect,'de')
    styles={identity:core.resolve_visual('window',{}),identity+':tab':core.resolve_visual('tab',{}),identity+':fullscreen':core.resolve_visual('fullscreen',{})}
    group=r._window(data,styles,False,thumbnail=False)
    if not control:
        group.remove(group._cursor_control)
    return group,data


class ExactDevice(Scene):
    """🔌 Activate the heater through the same event-driven native control used in production."""
    def construct(self):
        self.camera.background_color=p._theme.color('dark')
        self.camera.cairo_line_width_multiple=p._UNIT
        system,_=pane('system','System',{'x':252.8,'y':211.2,'width':1094.4,'height':480},control=True)
        device,data=pane('device','Heizgerät',{'x':400,'y':300.8,'width':800,'height':320})
        switch=r._button(data['control'],{**core.resolve_visual('toggle',{}),'stroke_width':0},label='Ein')
        switch[0].set_stroke(width=0)
        radiator=VGroup(p._box({'x':540.8,'y':403.2,'width':160,'height':160},color=p._theme.color('gray'),opacity=0,stroke=1),
                       *[Line(p._point(x,416),p._point(x,550.4),color=p._theme.color('gray'),stroke_width=1) for x in (560,585.6,611.2,636.8,662.4,688)])
        reading=p._text('0 W',800,480,font_size=16,line_height=16)
        caption=p._text('Mit dem Cursor einschalten',540.8,640)
        waves=visuals.radiation_waves(p._point(620.8,400),height=32*p._UNIT,color=p._theme.color('primary'),stroke_width=1)
        self.add(system,device,switch,radiator,reading,caption)
        self.wait(.5)

        def activate():
            switch[1].become(p._text('An',data['control']['x']+3.2,data['control']['y']+11.2,color=p._theme.color('dark')))
            reading.become(p._text('1500 W',800,480,font_size=16,line_height=16,color=p._theme.color('primary')))
            self.play(radiator.animate.set_color(p._theme.color('primary')),FadeIn(waves),run_time=.8)
            self.play(waves.animate.shift(np.array([0,16*p._UNIT,0])),run_time=.8)

        helper.cursor_click(self,switch,activate,parents=(system,device),role='toggle',move_time=.6,hover_time=.35,press_time=.25,release_time=.25)
        self.wait(.6)


body=output/'body'/'v'/'device.mp4'
if '--reuse-body' not in sys.argv or not body.is_file():
    runtime.render_config(output/'body')
    with tempconfig({'media_dir':str(output/'body'),'video_dir':str(output/'body'/'v'),'partial_movie_dir':str(output/'body'/'p'),
                     'text_dir':str(output/'body'/'t'),'output_file':'device','pixel_width':1588,'pixel_height':808,'frame_rate':8,
                     'frame_width':viewport['width']*p._UNIT,'frame_height':viewport['height']*p._UNIT,
                     'write_to_movie':True,'save_last_frame':False,'disable_caching':True,'verbosity':'ERROR','progress_bar':'none'}):
        scene=ExactDevice()
        scene.camera.frame_center=p._point(viewport['x']+viewport['width']/2,viewport['y']+viewport['height']/2)
        scene.render()
        body=Path(scene.renderer.file_writer.movie_file_path)
helper._dump()
identity=hashlib.sha256(Path(__file__).parent.relative_to(ROOT).as_posix().encode()).hexdigest()[:10]
trace=json.loads((output/'trace'/identity/'ExactDevice'/'👆️.json').read_text(encoding='utf8'))
runtime.publish_cursor_intervals(body,trace['intervals'])
if '--body-only' in sys.argv:
    print('[DEBUG] Fresh native device source rendered',flush=True)
    sys.exit(0)
lesson=output/'lesson.mp4'
p.render_presentation('physical_fundamentals',body,lesson,settings)
frames=output/'tour';frames.mkdir(parents=True,exist_ok=True)
index=0
requests=[]
probe=p.render_interaction_frame('physical_fundamentals','focus',.2,output/'menu-probe.png',settings,point=[48,14.4],menu_open=True)['browser']
band={'energy':probe['chrome']['energy'],'range':probe['chrome']['range'],'popup':probe['chrome'].get('menu_rect') or r._chrome_geometry('de')['popup']}
park=core.interaction_contract()['timeline']['park']
nav=[band['energy']['x']+band['energy']['width']/2,14.4]
leaf=next((item['rect'] for item in probe['chrome']['treeRows'] if 'physical_fundamentals' in item['id']),probe['chrome']['treeRows'][1]['rect'])
row=[leaf['x']+leaf['width']/2,leaf['y']+leaf['height']/2]
footer=[band['range']['x']+band['range']['width']/2,885.6]
menu_active=False


def move(start,end,duration,menu=False):
    """🧭 Tour measured chrome hit regions without separate drawing coordinates."""
    global index,menu_active
    count=max(1,round(duration*8))
    for step in range(count):
        u=step/max(1,count-1);u=u*u*(3-2*u)
        point=[start[i]*(1-u)+end[i]*u for i in (0,1)]
        menu_active=menu or r._inside(point,band['energy']) or menu_active and r._inside(point,band['popup'])
        requests.append({'video_id':'physical_fundamentals','phase':'focus','t':.2,'output':frames/f'{index:04d}.png','point':point,'menu_open':menu_active})
        index+=1


move(park,park,.4);move(park,nav,.55);move(nav,nav,.3);move(nav,row,.45,True);move(row,row,.4,True)
move(row,footer,.6);move(footer,footer,.35);move(footer,park,.4);move(park,park,.3)
r.frame_batch(requests,settings)
tour=output/'tour.mp4'
runtime.run_command([*runtime.ffmpeg_command(),'-hide_banner','-loglevel','error','-y','-framerate','8','-i',str(frames/'%04d.png'),'-an','-c:v','libx264','-crf','0','-pix_fmt','yuv420p',str(tour)],log=output/f'tour-{time.time_ns()}.log')
joined=output/'exact-ui.mp4'
runtime.concat_videos([tour,lesson],joined,output/f'join-{time.time_ns()}.txt',keep_audio=False)
shutil.copy2(joined,Path(__file__).with_name('exact-ui.mp4'))
p.render_overview(output/'overview.png',settings)
shutil.copy2(output/'overview.png',Path(__file__).with_name('exact-ui.png'))
Path(__file__).with_name('exact-ui-events.md').write_text('# Exact UI Authored Cursor Events\n\n'+'\n'.join(f"- {e['time']:.3f}s: {e['type']}" for e in trace['events'])+'\n',encoding='utf8')
print('[DEBUG] Exact UI review film retained',flush=True)
